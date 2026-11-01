use anyhow::Result;

use crate::{
    args::Args, calculate_screen_coverage::calculate_screen_coverage,
    send_mpv_pause::send_mpv_pause,
};

pub async fn check_and_update(args: &Args) -> Result<()> {
    let coverage = calculate_screen_coverage(args).await?;

    let should_pause = coverage > args.coverage_threshold;

    send_mpv_pause(&args.socket_path, should_pause).await?;

    Ok(())
}
