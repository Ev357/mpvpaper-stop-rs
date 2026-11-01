mod args;
use std::{env, time::Duration};

use tokio::{
    io::{AsyncBufReadExt, BufReader},
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

use crate::{args::Args, check_and_update::check_and_update, hyprland::connect_event_socket};

fn main() -> Result<()> {
    let mut args = Args::parse();

    if args.socket_path.is_relative()
        && let Ok(current_dir) = env::current_dir()
    {
        args.socket_path = current_dir.join(&args.socket_path);
    }

    if args.daemonize {
        Daemonize::new().start()?;
    }

    tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()?
        .block_on(run(args))
}

async fn run(args: Args) -> Result<()> {
    check_and_update(&args).await?;

    let stream = connect_event_socket(Duration::from_millis(args.retry_interval)).await?;
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

                if !matches!(
                    line.trim_end().split_once(">>"),
                    Some((
                        "openwindow"
                            | "closewindow"
                            | "activewindow"
                            | "activewindowv2"
                            | "workspace"
                            | "workspacev2"
                            | "fullscreen"
                            | "focusedmon"
                            | "focusedmonv2",
                        _,
                    ))
                ) {
                    continue;
                }

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
