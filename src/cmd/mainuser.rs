use anyhow::{Context, Result};

use crate::utils::{
    config::read_config,
    display::{build_vless_link, print_qr},
    keys::read_keys,
    system::get_public_ip,
};

pub fn run() -> Result<()> {
    let cfg = read_config()?;
    let keys = read_keys()?;
    let ip = get_public_ip()?;

    let first_client = cfg["inbounds"][0]["settings"]["clients"]
        .as_array()
        .and_then(|a| a.first())
        .context("No clients found in config")?;

    let uuid = first_client["id"].as_str().unwrap_or(&keys.uuid);
    let email = first_client["email"].as_str().unwrap_or("main");

    let link = build_vless_link(&cfg, &keys, email, uuid, &ip)?;

    log::info!("Connection link:");
    log::info!("{link}");
    log::info!("QR code:");
    print_qr(&link);

    Ok(())
}
