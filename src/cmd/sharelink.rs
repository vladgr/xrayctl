use anyhow::{bail, Context, Result};
use std::io::{self, Write as IoWrite};

use crate::utils::{
    config::{get_clients, read_config},
    display::{build_vless_link, print_qr},
    keys::read_keys,
    system::get_public_ip,
};

pub fn run() -> Result<()> {
    let cfg = read_config()?;
    let clients = get_clients(&cfg)?;

    if clients.is_empty() {
        bail!("No clients found");
    }

    for (i, (_, email, _)) in clients.iter().enumerate() {
        log::info!("{}. {email}", i + 1);
    }

    let choice = prompt_number("Select client: ", clients.len())?;
    let (_, email, uuid) = &clients[choice - 1];

    let keys = read_keys()?;
    let ip = get_public_ip()?;
    let link = build_vless_link(&cfg, &keys, email, uuid, &ip)?;

    log::info!("Connection link:");
    log::info!("{link}");
    log::info!("QR code:");
    print_qr(&link);

    Ok(())
}

fn prompt_number(message: &str, max: usize) -> Result<usize> {
    log::info!("{message}");
    io::stdout().flush()?;
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    let n: usize = input
        .trim()
        .parse()
        .context("Please enter a valid number")?;
    if n < 1 || n > max {
        bail!("Number must be between 1 and {max}");
    }
    Ok(n)
}
