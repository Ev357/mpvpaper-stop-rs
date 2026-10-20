use std::{path::PathBuf, time::Duration};

use clap::Parser;

use crate::constants::{
    DEFAULT_COVERAGE_THRESHOLD, DEFAULT_FALLBACK_CHECK_INTERVAL_MILLIS,
    DEFAULT_LISTENER_RETRY_INTERVAL_MILLIS, DEFAULT_MPV_SOCKET_PATH,
};

#[derive(Parser, Debug, Clone)]
#[command(
    name = "mpvpaper-stop-rs",
    about = "Automatically pause/resume mpvpaper based on visible window coverage in Hyprland",
    version
)]
pub struct Args {
    #[arg(short = 'd', long = "daemonize")]
    pub daemonize: bool,

    #[arg(
        short = 's',
        long = "socket",
        default_value = DEFAULT_MPV_SOCKET_PATH
    )]
    pub socket_path: PathBuf,

    #[arg(
        short = 'c',
        long = "coverage",
        default_value_t = DEFAULT_COVERAGE_THRESHOLD
    )]
    pub coverage_threshold: u8,

    #[arg(
        short = 'f',
        long = "fallback-interval",
        default_value_t = DEFAULT_FALLBACK_CHECK_INTERVAL_MILLIS
    )]
    pub fallback_interval: u64,

    #[arg(
        short = 'r',
        long = "retry-interval",
        default_value_t = DEFAULT_LISTENER_RETRY_INTERVAL_MILLIS
    )]
    pub retry_interval: u64,
}

impl Args {
    pub fn fallback_check_interval(&self) -> Duration {
        Duration::from_millis(self.fallback_interval)
    }

    pub fn listener_retry_interval(&self) -> Duration {
        Duration::from_millis(self.retry_interval)
    }
}
