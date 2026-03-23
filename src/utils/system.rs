use anyhow::{Context, Result};
use std::process::Command;

pub fn get_public_ip() -> Result<String> {
    let out = Command::new("curl")
        .args(["-4", "-s", "--max-time", "5", "https://icanhazip.com"])
        .output()
        .context("Failed to run curl to detect public IP")?;
    Ok(String::from_utf8_lossy(&out.stdout).trim().to_string())
}

pub fn restart_xray() -> Result<()> {
    let status = Command::new("systemctl")
        .args(["restart", "xray"])
        .status()
        .context("Failed to run systemctl restart xray")?;
    if !status.success() {
        anyhow::bail!("systemctl restart xray failed");
    }
    Ok(())
}
