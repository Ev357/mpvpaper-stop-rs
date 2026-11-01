use std::path::PathBuf;

use clap::Parser;

use crate::constants::{
    DEFAULT_COVERAGE_THRESHOLD, DEFAULT_FALLBACK_CHECK_INTERVAL_MILLIS, DEFAULT_MPV_SOCKET_PATH,
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

    #[arg(short = 'i', long = "ignore-classes")]
    pub ignore_classes: Vec<String>,
}
