use anyhow::{Context, Result};
use hyprland::{
    data::{Clients, Monitor},
    prelude::*,
};

use crate::calculate_window_coverage::calculate_window_coverage;

pub fn calculate_screen_coverage() -> Result<u8> {
    let monitor = Monitor::get_active().context("failed to get active monitor")?;
    let clients = Clients::get().context("failed to get clients")?.to_vec();
    Ok(calculate_window_coverage(&monitor, &clients))
}
