use std::{
    collections::HashMap,
    env, fs,
    io::{self, BufRead, Write},
    path::PathBuf,
    process::{self, Stdio},
};

use base64::prelude::*;
use rand::{thread_rng, Rng};
use reqwest::header;
use serde::Deserialize;

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct AuthResponse {
    access_token: String,
    selected_profile: Profile,
}

#[derive(Deserialize)]
struct Profile {
    id: String,
    name: String,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();

    if args.len() < 4 {
        eprintln!("Usage: {} <username> <password> <api url>", args[0]);
        process::exit(1);
    }

    if args.len() == 4 {
        eprintln!("Looks like you have entered a valid command, but you can't run mmcai_rs directly! Put your command in \"Wrapper command\" in Prism Launcher.");
        process::exit(1);
    }

    // find authlib injector
    let current_exe = env::current_exe()?;
    let exe_dir = current_exe.parent().unwrap();
    let mut authlib_injector_path: PathBuf = PathBuf::new();

    if let Ok(entries) = fs::read_dir(exe_dir) {
        for entry in entries {
            if let Ok(entry) = entry {
                let file_name = entry.file_name();
                if let Some(file_name) = file_name.to_str() {
                    if file_name.starts_with("authlib-injector") && file_name.ends_with(".jar") {
                        authlib_injector_path = entry.path();
                        break;
                    }
                }
            }
        }
    }

    if authlib_injector_path == PathBuf::new() {
        eprintln!("[mmcai_rs] authlib-injector not found in the same directory as mmcai_rs!");
        process::exit(1);
    }

    println!(
        "[mmcai_rs] authlib-injector found at {:?}, logging in...",
        authlib_injector_path
    );

    // yggdrasil part
    let username = &args[1];
    let password = &args[2];
    let api_url = &args[3];

    let mut rng = thread_rng();
    let mut buffer = [0u8; 128];
    rng.fill(&mut buffer);
    let base64_encoded = BASE64_STANDARD.encode(&buffer);
    let client_token = &base64_encoded[..128];

    let mut headers = header::HeaderMap::new();
    headers.insert("Content-Type", "application/json".parse().unwrap());

    let mut body: HashMap<&str, &str> = HashMap::new();
    body.insert("username", username);
    body.insert("password", password);
    body.insert("requestUser", "true");
    body.insert("clientToken", client_token);

    let client = reqwest::blocking::Client::builder()
        .redirect(reqwest::redirect::Policy::none())
        .build()
        .unwrap();

    let hello_response_text = client.get(api_url).send()?.text()?;

    let auth_response = client
        .post(format!("{}/authserver/authenticate", api_url))
        .headers(headers)
        .json(&body)
        .send()?
        .json::<AuthResponse>()?;

    println!(
        "[mmcai_rs] Successfully authenticated as {}",
        auth_response.selected_profile.name
    );

    // minecraft params
    let mut minecraft_params: Vec<String> = Vec::new();

    let stdin = io::stdin();
    for line in stdin.lock().lines() {
        let line = &line.expect("Failed to read Minecraft params");

        minecraft_params.push(line.clone());

        if line.trim() == "launch" {
            break;
        }
    }

    let access_token = auth_response.access_token;
    let uuid = auth_response.selected_profile.id;
    let playername = auth_response.selected_profile.name;

    for index in 0..minecraft_params.len() {
        if minecraft_params[index].contains("param --username") {
            if let Some(next_line) = minecraft_params.get_mut(index + 1) {
                *next_line = format!("param {}", playername).to_string();
            }
        }

        if minecraft_params[index].contains("param --uuid") {
            if let Some(next_line) = minecraft_params.get_mut(index + 1) {
                *next_line = format!("param {}", uuid).to_string();
            }
        }

        if minecraft_params[index].contains("param --accessToken") {
            if let Some(next_line) = minecraft_params.get_mut(index + 1) {
                *next_line = format!("param {}", access_token).to_string();
            }
        }

        if minecraft_params[index].contains("userName ") {
            if let Some(this_line) = minecraft_params.get_mut(index) {
                *this_line = format!("userName {}", playername).to_string();
            }
        }

        if minecraft_params[index].contains("sessionId ") {
            if let Some(this_line) = minecraft_params.get_mut(index) {
                *this_line = format!("sessionId token:{}", access_token).to_string();
            }
        }
    }

    // ready to launch
    let prefetched_data = BASE64_STANDARD.encode(hello_response_text);
    let java_executable = env::var("INST_JAVA").unwrap();
    let mut jvm_args = Vec::from(&args[5..]);
    jvm_args.insert(
        0,
        format!(
            "-javaagent:{}={}",
            authlib_injector_path.to_str().unwrap(),
            api_url
        ),
    );
    jvm_args.insert(
        1,
        format!("-Dauthlibinjector.yggdrasil.prefetched={}", prefetched_data),
    );

    let mut command = process::Command::new(java_executable);
    command.args(jvm_args);

    if let Ok(mut child) = command
        .stdin(Stdio::piped())
        .stdout(Stdio::inherit())
        .spawn()
    {
        let child_stdin = &mut child.stdin;

        if let Some(stdin) = child_stdin {
            for line in minecraft_params {
                writeln!(stdin, "{}", line).expect("Failed to write Minecraft params");
            }
        }

        let _ = child.wait();
    }

    Ok(())
}
