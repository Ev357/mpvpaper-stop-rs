use std::{path::PathBuf, sync::OnceLock};

use crate::{args::Args, atomic_pause_state::AtomicPauseState};

pub const DEFAULT_MPV_SOCKET_PATH: &str = "/tmp/mpvsocket";
pub const DEFAULT_COVERAGE_THRESHOLD: u8 = 90;
pub const DEFAULT_FALLBACK_CHECK_INTERVAL_MILLIS: u64 = 1000;
pub const DEFAULT_LISTENER_RETRY_INTERVAL_MILLIS: u64 = 2000;

pub static LAST_PAUSE_STATE: AtomicPauseState = AtomicPauseState::new();

static CONFIG: OnceLock<Args> = OnceLock::new();

pub fn init_config(args: Args) {
    let _ = CONFIG.set(args);
}

pub fn config() -> &'static Args {
    CONFIG.get_or_init(|| Args {
        daemonize: false,
        socket_path: PathBuf::from(DEFAULT_MPV_SOCKET_PATH),
        coverage_threshold: DEFAULT_COVERAGE_THRESHOLD,
        fallback_interval: DEFAULT_FALLBACK_CHECK_INTERVAL_MILLIS,
        retry_interval: DEFAULT_LISTENER_RETRY_INTERVAL_MILLIS,
    })
}
