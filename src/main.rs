mod args;
use std::time::Duration;

use tokio::{
    io::{AsyncBufReadExt, BufReader},
    net::UnixStream,
    signal, time,
};
mod calculate_screen_coverage;
mod calculate_window_coverage;
mod check_and_update;
mod constants;
mod hyprland;
mod send_mpv_pause;

use anyhow::Result;
use clap::Parser;
use daemonize::Daemonize;

use crate::{
    args::Args, check_and_update::check_and_update, hyprland::get_hyprland_event_socket_path,
};

#[tokio::main]
async fn main() -> Result<()> {
    let mut args = Args::parse();

    if args.socket_path.is_relative()
        && let Ok(current_dir) = std::env::current_dir()
    {
        args.socket_path = current_dir.join(&args.socket_path);
    }

    if args.daemonize {
        Daemonize::new().start()?;
    }

    check_and_update(&args).await?;

    let socket_path = get_hyprland_event_socket_path()?;
    let stream = UnixStream::connect(socket_path).await?;
    let mut reader = BufReader::new(stream);
    let mut line = String::new();

    let mut interval = time::interval(Duration::from_millis(args.fallback_interval));

    loop {
        line.clear();

        tokio::select! {
            result = reader.read_line(&mut line) => {
                let bytes_read = result?;
                if bytes_read == 0 {
                    break;
                }

                let trimmed = line.trim_end();
                let _ = match trimmed.split_once(">>") {
                    Some((
                        event @ ("openwindow" | "closewindow" | "activewindow" | "activewindowv2" | "workspace"
                        | "workspacev2" | "fullscreen" | "focusedmon" | "focusedmonv2"),
                        data,
                    )) => (event, data),
                    _ => continue,
                };

                check_and_update(&args).await?;
            }

            _ = interval.tick() => {
                check_and_update(&args).await?;
            }

            _ = signal::ctrl_c() => {
                break;
            }
        }
    }

    Ok(())
}
