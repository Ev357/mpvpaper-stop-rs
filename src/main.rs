mod args;
mod atomic_pause_state;
mod calculate_screen_coverage;
mod calculate_window_coverage;
mod check_and_update;
mod constants;
mod pause_state;
mod send_mpv_pause;

use std::thread;

use anyhow::{Context, Result};
use clap::Parser;
use daemonize::Daemonize;
use hyprland::{event_listener::EventListener, shared::WorkspaceType};

use crate::{
    args::Args,
    check_and_update::check_and_update,
    constants::{config, init_config},
};

fn main() -> Result<()> {
    let mut args = Args::parse();

    if args.socket_path.is_relative()
        && let Ok(current_dir) = std::env::current_dir()
    {
        args.socket_path = current_dir.join(&args.socket_path);
    }

    if args.daemonize {
        Daemonize::new()
            .start()
            .context("failed to daemonize process")?;
    }

    init_config(args);

    let config = config();
    let fallback_interval = config.fallback_check_interval();
    let retry_interval = config.listener_retry_interval();

    check_and_update();

    thread::spawn(move || {
        loop {
            thread::sleep(fallback_interval);
            check_and_update();
        }
    });

    let mut listener = EventListener::new();

    listener.add_window_opened_handler(|_| check_and_update());
    listener.add_window_closed_handler(|_| check_and_update());
    listener.add_active_window_changed_handler(|_| check_and_update());
    listener.add_workspace_changed_handler(|workspace_event| {
        let _workspace: &WorkspaceType = &workspace_event.name;
        check_and_update();
    });
    listener.add_fullscreen_state_changed_handler(|_| check_and_update());
    listener.add_active_monitor_changed_handler(|_| check_and_update());

    loop {
        if let Err(error) = listener.start_listener() {
            eprintln!(
                "Hyprland event listener stopped: {error:#}. Retrying in {} seconds...",
                retry_interval.as_secs()
            );
            thread::sleep(retry_interval);
        }
    }
}
