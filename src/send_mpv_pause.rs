use std::{io::Write, os::unix::net::UnixStream};

use anyhow::{Context, Result};

use crate::constants::config;

pub fn send_mpv_pause(pause: bool) -> Result<()> {
    let mut stream =
        UnixStream::connect(&config().socket_path).context("failed to connect to mpv socket")?;
    let command = serde_json::json!({
        "command": ["set_property", "pause", pause]
    });
    let mut payload = serde_json::to_vec(&command)?;
    payload.push(b'\n');
    stream
        .write_all(&payload)
        .context("failed to write to mpv socket")?;
    Ok(())
}
