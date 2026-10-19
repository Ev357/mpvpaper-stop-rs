use std::io;

use crate::{
    calculate_screen_coverage::calculate_screen_coverage,
    constants::{COVERAGE_THRESHOLD, LAST_PAUSE_STATE, MPV_SOCKET_PATH},
    pause_state::PauseState,
    send_mpv_pause::send_mpv_pause,
};

pub fn check_and_update() {
    let coverage = match calculate_screen_coverage() {
        Ok(result) => result,
        Err(error) => {
            eprintln!("Error querying Hyprland: {error:#}");
            return;
        }
    };

    let should_pause = coverage > COVERAGE_THRESHOLD;
    let target_state = PauseState::from(should_pause);

    if LAST_PAUSE_STATE.load() == target_state {
        return;
    }

    if let Err(error) = send_mpv_pause(should_pause) {
        LAST_PAUSE_STATE.store(PauseState::Unknown);

        let is_socket_offline =
            error
                .root_cause()
                .downcast_ref::<io::Error>()
                .is_some_and(|io_error| {
                    matches!(
                        io_error.kind(),
                        io::ErrorKind::NotFound | io::ErrorKind::ConnectionRefused
                    )
                });

        if !is_socket_offline {
            eprintln!("Failed to send pause command to {MPV_SOCKET_PATH}: {error:#}");
        }
    } else {
        LAST_PAUSE_STATE.store(target_state);
    }
}
