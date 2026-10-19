mod atomic_pause_state;
mod calculate_screen_coverage;
mod calculate_window_coverage;
mod check_and_update;
mod constants;
mod pause_state;
mod send_mpv_pause;

use std::thread;

use anyhow::Result;
use hyprland::{event_listener::EventListener, shared::WorkspaceType};

use crate::{
    check_and_update::check_and_update,
    constants::{FALLBACK_CHECK_INTERVAL, LISTENER_RETRY_INTERVAL},
};

fn main() -> Result<()> {
    check_and_update();

    thread::spawn(|| {
        loop {
            thread::sleep(FALLBACK_CHECK_INTERVAL);
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
                LISTENER_RETRY_INTERVAL.as_secs()
            );
            thread::sleep(LISTENER_RETRY_INTERVAL);
        }
    }
}
