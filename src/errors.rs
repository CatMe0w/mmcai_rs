use thiserror::Error;
use reqwest::Error as ReqwestError;
use std::io::Error as IoError;

#[derive(Error, Debug)]
pub enum MmcaiError {
    #[error("Usage: {0} <username> <password> <api url>")]
    WrongUsage(String),
    #[error("Looks like you have entered a valid command, but you can't run mmcai_rs directly! Put your command in \"Wrapper command\" in Prism Launcher.")]
    RunDirectly,
    #[error("authlib-injector not found in the same directory as mmcai_rs.")]
    AuthlibInjectorNotFound,
    #[error("Error happened when trying to create reqwest client.")]
    ReqwestClientBuildFailed(#[source] ReqwestError),
    #[error("Error happened when trying to prefetch yggdrasil server.")]
    PrefetchFailed(#[source] ReqwestError),
    #[error("Error happened when trying to authenticate with yggdrasil server.")]
    AuthFailed(#[source] ReqwestError),
    #[error("IO error happened when trying to parse minecraft params.")]
    ParseMinecraftParamsFailed(#[source] IoError),
    #[error("IO error happened when trying to write minecraft params.")]
    WriteMinecraftParamsFailed(#[source] IoError),
    #[error("Error happened when trying to spawn MineCraft process.")]
    SpawnProcessFailed(#[source] IoError),
    #[error("Error happened when trying to get the pipe of the MineCraft process's stdin.")]
    StdinNotFoundFailed,
    #[error("Environment variable {0} not found.")]
    EnvVarNotFound(String),
    #[error("Unknown error happened.")]
    Other
}
