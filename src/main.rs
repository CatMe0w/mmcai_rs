use std::{
    env, fs,
    io::{self, BufRead, Write},
    path::PathBuf,
    process::{self, Stdio},
};
use std::collections::HashMap;

use base64::prelude::*;
use rand::{Rng, thread_rng};
use reqwest::Error as ReqwestError;
use reqwest::header;
use serde::{Deserialize, Serialize};
use crate::errors::MmcaiError;

mod errors;

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct AuthRequest<'a> {
    username: &'a str,
    password: &'a str,
    request_user: bool,
    client_token: &'a str,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct AuthResponse {
    access_token: String,
    selected_profile: Profile,
}

struct LoginResult {
    prefetched: String,
    access_token: String,
    selected_profile: Profile,
}

#[derive(Deserialize)]
struct Profile {
    id: String,
    name: String,
}

fn check_args(args: &Vec<String>) -> Result<(), MmcaiError> {
    match args.len() {
        len if len < 4 => Err(MmcaiError::WrongUsage(args[0].to_owned())),
        4 => Err(MmcaiError::RunDirectly),
        _ => Ok(()),
    }
}

fn find_authlib_injector() -> Option<PathBuf> {
    let current_exe = env::current_exe().ok()?;
    let exe_dir = current_exe.parent()?;
    let is_filename_valid = |filename: &str| {
        filename.starts_with("authlib-injector") && filename.ends_with(".jar")
    };

    fs::read_dir(exe_dir).ok().and_then(|entries| {
        entries
            .filter_map(Result::ok)
            .find(|entry| {
                let file_name = entry.file_name();
                file_name.to_str().map_or(false, is_filename_valid)
            })
            .map(|entry| entry.path())
    })
}

fn get_rnd_client_token() -> String {
    let mut rng = thread_rng();
    let mut buffer = [0u8; 128];
    rng.fill(&mut buffer);
    let base64_encoded = BASE64_STANDARD.encode(&buffer);
    base64_encoded[..128].to_string()
}

fn login_yggdrasil(
    username: &str,
    password: &str,
    client_token: &str,
    api_url: &str,
) -> Result<LoginResult, MmcaiError> {
    let client = reqwest::blocking::Client::builder()
        .redirect(reqwest::redirect::Policy::none())
        .build().map_err(MmcaiError::ReqwestClientBuildFailed)?;

    let get_prefetched = || -> Result<String, ReqwestError> {
        let prefetched = client.get(api_url).send()?.text()?;
        Ok(prefetched)
    };

    let get_authenticate = || -> Result<AuthResponse, ReqwestError> {
        let mut headers = header::HeaderMap::new();
        headers.insert("Content-Type", "application/json".parse().unwrap());

        let body = AuthRequest {
            username,
            password,
            request_user: true,
            client_token,
        };

        Ok(client
            .post(format!("{}/authserver/authenticate", api_url))
            .headers(headers)
            .json(&body)
            .send()?
            .json::<AuthResponse>()?)
    };

    let prefetched = get_prefetched().map_err(MmcaiError::PrefetchFailed)?;
    let auth_response = get_authenticate().map_err(MmcaiError::AuthFailed)?;

    Ok(LoginResult {
        prefetched,
        access_token: auth_response.access_token,
        selected_profile: auth_response.selected_profile,
    })
}

fn main() -> Result<(), MmcaiError> {
    let args: Vec<String> = env::args().collect();

    check_args(&args)?;

    let authlib_injector_path = find_authlib_injector().ok_or(MmcaiError::AuthlibInjectorNotFound)?;

    println!(
        "[mmcai_rs] authlib-injector found at {:?}, logging in...",
        authlib_injector_path
    );

    // yggdrasil part
    let username = &args[1];
    let password = &args[2];
    let api_url = &args[3];

    let client_token = get_rnd_client_token();

    let login_result = login_yggdrasil(username, password, &client_token, api_url)?;

    println!(
        "[mmcai_rs] Successfully authenticated as {}",
        login_result.selected_profile.name
    );

    // minecraft params
    let stdin = io::stdin();

    let access_token = login_result.access_token;
    let profile_id = login_result.selected_profile.id;
    let profile_name = login_result.selected_profile.name;

    let convert_next_line = |line: &str| match line {
        line if line.contains("param --username") => Some(format!("param {}", profile_name)),
        line if line.contains("param --uuid") => Some(format!("param {}", profile_id)),
        line if line.contains("param --accessToken") => Some(format!("param {}", access_token)),
        line if line.contains("userName ") => Some(format!("userName {}", profile_name)),
        line if line.contains("sessionId ") => Some(format!("sessionId token:{}", access_token)),
        _ => None,
    };
    let mut modify_items: HashMap<usize, String> = HashMap::new();
    let minecraft_params = stdin.lock().lines()
        .take_while(|line| match line {
            Ok(line) => line.trim_end() != "launch",
            Err(_) => false,
        })
        .chain(std::iter::once(Ok("launch".to_owned())))
        .enumerate()
        .map::<Result<String, MmcaiError>, _>(|(index, line)| {
            let line = line.map_err(|_| MmcaiError::Other)?;
            match modify_items.remove(&(index + 1)) { 
                Some(modified) => Ok(modified),
                None => {
                    if let Some(converted) = convert_next_line(line.as_str()) {
                        modify_items.insert(index + 1, converted.clone());
                    };
                    Ok(line)
                }
            }
        });

    // ready to launch
    let prefetched_data = BASE64_STANDARD.encode(login_result.prefetched);
    let java_executable = env::var("INST_JAVA").map_err(|_| MmcaiError::EnvVarNotFound("INST_JAVA".to_owned()))?;
    let javaagent_arg = format!(
        "-javaagent:{}={}",
        authlib_injector_path.to_str().ok_or(MmcaiError::Other)?,
        api_url
    );
    let prefetched_arg = format!("-Dauthlibinjector.yggdrasil.prefetched={}", prefetched_data);
    let jvm_args_iter = args.iter().skip(5).chain(
        std::iter::once(&javaagent_arg)
    ).chain(
        std::iter::once(&prefetched_arg)
    );

    let mut command = process::Command::new(java_executable);
    command.args(jvm_args_iter);

    let mut child = command
        .stdin(Stdio::piped())
        .stdout(Stdio::inherit())
        .spawn()
        .map_err(MmcaiError::SpawnProcessFailed)?;

    let stdin = (&mut child.stdin).as_mut().ok_or(MmcaiError::StdinNotFoundFailed)?;

    minecraft_params.map(|line| {
        let line = line?;
        stdin.write_all(line.as_bytes()).map_err(MmcaiError::WriteMinecraftParamsFailed)
    }).take_while(Result::is_ok).collect::<Result<_, _>>()?;

    let status = child.wait().map_err(|_| MmcaiError::Other)?;
    if !status.success() {
        process::exit(status.code().unwrap_or(1));
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use fake::{Fake, Faker};
    use rand::rngs::StdRng;
    use rand::SeedableRng;
    use super::*;
    
    fn get_fake_args(length: usize) -> Vec<String> {
        let seed = [
            1, 0, 0, 0, 23, 0, 0, 0, 200, 1, 0, 0, 210, 30, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0,
        ];
        let ref mut r = StdRng::from_seed(seed);
        (0..length).map(|_| Faker.fake_with_rng::<String, _>(r)).collect()
    }
    
    #[test]
    fn test_check_args() {
        assert!(matches!(check_args(&get_fake_args(1)), Err(MmcaiError::WrongUsage(_))));
        assert!(matches!(check_args(&get_fake_args(2)), Err(MmcaiError::WrongUsage(_))));
        assert!(matches!(check_args(&get_fake_args(3)), Err(MmcaiError::WrongUsage(_))));
        assert!(matches!(check_args(&get_fake_args(4)), Err(MmcaiError::RunDirectly)));
        assert!(matches!(check_args(&get_fake_args(5)), Ok(())));
    }
    
    #[test]
    fn test_get_rnd_client_token() {
        for _ in 0..1000 {
            let token = get_rnd_client_token();
            assert_eq!(token.len(), 128);
        }
    }
}