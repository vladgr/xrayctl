use anyhow::{bail, Result};

use crate::utils::config;

pub fn run() -> Result<()> {
    let cfg = config::read_config()?;
    let clients = config::get_clients(&cfg)?;

    if clients.is_empty() {
        bail!("No clients configured");
    }

    log::info!("Client list:");
    for (i, (_, email, _)) in clients.iter().enumerate() {
        log::info!("{}. {email}", i + 1);
    }

    Ok(())
}
