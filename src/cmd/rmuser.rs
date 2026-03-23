use anyhow::{bail, Context, Result};
use std::io::{self, Write as IoWrite};

use crate::utils::{
    config::{get_clients, read_config, write_config},
    system::restart_xray,
};

pub fn run() -> Result<()> {
    let mut cfg = read_config()?;
    let clients = get_clients(&cfg)?;

    if clients.is_empty() {
        bail!("No clients to remove");
    }

    log::info!("Client list:");
    for (i, (_, email, _)) in clients.iter().enumerate() {
        log::info!("{}. {email}", i + 1);
    }

    let choice = prompt_number("Enter client number to remove: ", clients.len())?;
    let selected_email = clients[choice - 1].1.clone();

    cfg["inbounds"][0]["settings"]["clients"]
        .as_array_mut()
        .context("No clients array in config")?
        .retain(|c| c["email"].as_str() != Some(&selected_email));

    write_config(&cfg)?;
    restart_xray()?;

    log::info!("Client '{selected_email}' removed.");
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
