use anyhow::{Context, Result};
use serde_json::Value;

use super::constants::CONFIG_PATH;

pub fn read_config() -> Result<Value> {
    let content = std::fs::read_to_string(CONFIG_PATH)
        .context("Failed to read config.json — is Xray installed?")?;
    serde_json::from_str(&content).context("Failed to parse config.json")
}

pub fn write_config(config: &Value) -> Result<()> {
    let content = serde_json::to_string_pretty(config)?;
    std::fs::write(CONFIG_PATH, content).context("Failed to write config.json")?;
    Ok(())
}

// Returns (index, email, uuid) for each client in inbounds[0]
pub fn get_clients(config: &Value) -> Result<Vec<(usize, String, String)>> {
    let clients = config["inbounds"][0]["settings"]["clients"]
        .as_array()
        .context("No clients array in config")?;
    Ok(clients
        .iter()
        .enumerate()
        .map(|(i, c)| {
            (
                i,
                c["email"].as_str().unwrap_or("").to_string(),
                c["id"].as_str().unwrap_or("").to_string(),
            )
        })
        .collect())
}
