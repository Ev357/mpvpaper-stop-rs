use std::{env, path::PathBuf};

use anyhow::Result;
use serde::Deserialize;
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::UnixStream,
};

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Monitor {
    pub width: i32,
    pub height: i32,
    pub x: i32,
    pub y: i32,
    pub scale: f32,
    pub focused: bool,
    pub active_workspace: ActiveWorkspace,
}

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ActiveWorkspace {
    pub id: i64,
}

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Client {
    pub at: (i32, i32),
    pub size: (i32, i32),
    pub workspace: ClientWorkspace,
    pub class: String,
    pub fullscreen: i32,
    pub fullscreen_client: i32,
    pub floating: bool,
}

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ClientWorkspace {
    pub id: i64,
}

pub async fn query_hyprland<T: serde::de::DeserializeOwned>(command: &str) -> Result<T> {
    let socket_path = get_command_socket_path()?;

    let mut stream = UnixStream::connect(&socket_path).await?;

    stream.write_all(command.as_bytes()).await?;

    stream.flush().await?;

    let mut response = String::new();

    stream.read_to_string(&mut response).await?;

    let data: T = serde_json::from_str(&response)?;

    Ok(data)
}

fn get_hyprland_socket_dir() -> Result<PathBuf> {
    let xdg_runtime = env::var("XDG_RUNTIME_DIR")?;
    let instance_sig = env::var("HYPRLAND_INSTANCE_SIGNATURE")?;

    let socket_dir = PathBuf::from(xdg_runtime).join("hypr").join(instance_sig);
    Ok(socket_dir)
}

pub fn get_hyprland_event_socket_path() -> Result<PathBuf> {
    Ok(get_hyprland_socket_dir()?.join(".socket2.sock"))
}

pub fn get_command_socket_path() -> Result<PathBuf> {
    Ok(get_hyprland_socket_dir()?.join(".socket.sock"))
}
