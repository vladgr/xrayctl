use anyhow::{bail, Context, Result};
use std::io::{self, Write as IoWrite};
use std::process::Command;

use crate::utils::config::{get_clients, read_config, write_config};
use crate::utils::display::{build_vless_link, print_qr};
use crate::utils::keys::read_keys;
use crate::utils::system::{get_public_ip, restart_xray};

pub fn run() -> Result<()> {
    let email = prompt("Enter username (email): ")?;

    if email.is_empty() {
        bail!("Username cannot be empty");
    }
    if email.contains(' ') {
        bail!("Username cannot contain spaces");
    }

    let mut cfg = read_config()?;

    // Check for duplicate
    let clients = get_clients(&cfg)?;
    if clients.iter().any(|(_, e, _)| e == &email) {
        bail!("A user named '{email}' already exists");
    }

    // Generate a fresh UUID for the new user
    let out = Command::new("xray")
        .arg("uuid")
        .output()
        .context("Failed to run xray uuid")?;
    if !out.status.success() {
        bail!("xray uuid command failed");
    }
    let uuid = String::from_utf8_lossy(&out.stdout).trim().to_string();

    // Append the new client entry
    cfg["inbounds"][0]["settings"]["clients"]
        .as_array_mut()
        .context("No clients array in config")?
        .push(serde_json::json!({
            "email": email,
            "id": uuid,
            "flow": "xtls-rprx-vision"
        }));

    write_config(&cfg)?;
    restart_xray()?;

    let keys = read_keys()?;
    let ip = get_public_ip()?;
    let link = build_vless_link(&cfg, &keys, &email, &uuid, &ip)?;

    log::info!("Connection link:");
    log::info!("{link}");
    log::info!("QR code:");
    print_qr(&link);

    Ok(())
}

fn prompt(message: &str) -> Result<String> {
    log::info!("{message}");
    io::stdout().flush()?;
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    Ok(input.trim().to_string())
}
