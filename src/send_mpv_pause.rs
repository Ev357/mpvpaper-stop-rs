use std::path::Path;

use anyhow::Result;
use tokio::{io::AsyncWriteExt, net::UnixStream};

pub async fn send_mpv_pause<P: AsRef<Path>>(socket_path: P, pause: bool) -> Result<()> {
    let mut stream = UnixStream::connect(socket_path).await?;

    let command = serde_json::json!({
        "command": ["set_property", "pause", pause]
    });

    let mut payload = serde_json::to_vec(&command)?;
    payload.push(b'\n');

    stream.write_all(&payload).await?;

    stream.flush().await?;

    Ok(())
}
