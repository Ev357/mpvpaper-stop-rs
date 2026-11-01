use anyhow::{Context, Result};
use tokio::try_join;

use crate::{
    args::Args,
    calculate_window_coverage::calculate_window_coverage,
    hyprland::{Client, Monitor, query_hyprland},
};

pub async fn calculate_screen_coverage(args: &Args) -> Result<u8> {
    let (monitors_res, clients_res) = try_join!(
        query_hyprland::<Vec<Monitor>>("j/monitors"),
        query_hyprland::<Vec<Client>>("j/clients")
    )?;

    let monitor = monitors_res
        .iter()
        .find(|monitor| monitor.focused)
        .or_else(|| monitors_res.first())
        .context("no monitors found")?;

    Ok(calculate_window_coverage(args, monitor, &clients_res))
}
