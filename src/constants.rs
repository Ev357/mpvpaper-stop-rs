use std::time::Duration;

use crate::atomic_pause_state::AtomicPauseState;

pub const MPV_SOCKET_PATH: &str = "/tmp/mpvsocket";
pub const COVERAGE_THRESHOLD: u8 = 90;
pub const FALLBACK_CHECK_INTERVAL: Duration = Duration::from_secs(1);
pub const LISTENER_RETRY_INTERVAL: Duration = Duration::from_secs(2);

pub static LAST_PAUSE_STATE: AtomicPauseState = AtomicPauseState::new();
