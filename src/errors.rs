use reqwest::Error as ReqwestError;
use std::io::Error as IoError;
use thiserror::Error;

#[derive(Error)]
pub enum MmcaiError {
    #[error("Usage: {0} <username> <password> <api url>")]
    InvalidArgument(String),

    #[error("Looks like you have entered a valid command, but you can't run mmcai_rs directly! Put your command in \"Wrapper command\" in Prism Launcher.")]
    CannotRunDirectly,

    #[error("authlib-injector not found in the same directory as mmcai_rs.")]
    AuthlibInjectorNotFound,

    #[error("Cannot reach the authentication server.")]
    YggdrasilHelloFailed(#[source] ReqwestError),

    #[error("Wrong username or password.")]
    YggdrasilAuthFailed(#[source] ReqwestError),

    #[error("Cannot build reqwest client. This should not happen. Please report this issue to the developers.")]
    ReqwestClientBuildFailed(#[source] ReqwestError),

    #[error("Cannot read Minecraft params. This should not happen. Please report this issue to the developers.")]
    ReadMinecraftParamsFailed(#[source] IoError),

    #[error("Cannot write Minecraft params. This should not happen. Please report this issue to the developers.")]
    WriteMinecraftParamsFailed(#[source] IoError),

    #[error("Cannot start Minecraft. This should not happen. Please report this issue to the developers.")]
    SpawnProcessFailed(#[source] IoError),

    #[error("Cannot write Minecraft params. Stdin is unavailable. This should not happen. Please report this issue to the developers.")]
    StdinUnavailable,

    #[error("Cannot find Java executable. This should not happen. Please report this issue to the developers.")]
    JavaExecutableNotFound,

    #[error("Unknown error. This should not happen. Please report this issue to the developers.")]
    Other,
}

impl std::fmt::Debug for MmcaiError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self)
    }
}
