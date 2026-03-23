use anyhow::{Context, Result};
use serde_json::json;
use std::fs;
use std::io::Write as IoWrite;
use std::process::Command;

use crate::utils::config::write_config;
use crate::utils::constants::{CONFIG_PATH, KEYS_PATH};
use crate::utils::entities::Keys;
use crate::utils::keys::read_keys;
use crate::utils::system::restart_xray;

pub fn run() -> Result<()> {
    log::info!("Installing VLESS with TCP transport...");
    std::thread::sleep(std::time::Duration::from_secs(3));

    // Install system dependencies
    run_cmd("apt", &["update"])?;
    run_cmd("apt", &["install", "-y", "qrencode", "curl", "jq"])?;

    // Enable BBR congestion control
    enable_bbr()?;

    // Install Xray-core via the official script
    log::info!("Installing Xray-core...");
    run_cmd(
        "bash",
        &[
            "-c",
            r#"bash -c "$(curl -4 -L https://github.com/XTLS/Xray-install/raw/main/install-release.sh)" @ install"#,
        ],
    )?;

    // Generate and persist credentials
    generate_credentials()?;

    // Build config.json from the generated keys
    let keys = read_keys()?;
    write_initial_config(&keys)?;

    // Start / restart the service
    restart_xray()?;

    log::info!("Xray-core successfully installed!");
    log::info!("Tip: run `xrayctl mainuser` to show the main user link at any time.");

    // Print the main user link right away
    super::mainuser::run()
}

// ---------- private helpers ----------

fn run_cmd(prog: &str, args: &[&str]) -> Result<()> {
    let status = Command::new(prog)
        .args(args)
        .status()
        .with_context(|| format!("Failed to execute: {prog}"))?;
    if !status.success() {
        anyhow::bail!("{prog} exited with a non-zero status");
    }
    Ok(())
}

fn cmd_output(prog: &str, args: &[&str]) -> Result<String> {
    let out = Command::new(prog)
        .args(args)
        .output()
        .with_context(|| format!("Failed to execute: {prog}"))?;
    if !out.status.success() {
        anyhow::bail!("{prog} failed");
    }
    Ok(String::from_utf8_lossy(&out.stdout).trim().to_string())
}

fn enable_bbr() -> Result<()> {
    let out = Command::new("sysctl")
        .arg("-a")
        .output()
        .context("Failed to run sysctl -a")?;

    let output = String::from_utf8_lossy(&out.stdout);
    let already_bbr = output
        .lines()
        .any(|l| l.contains("net.ipv4.tcp_congestion_control") && l.contains("bbr"));

    if already_bbr {
        log::info!("BBR already enabled");
    } else {
        let mut f = fs::OpenOptions::new()
            .append(true)
            .open("/etc/sysctl.conf")
            .context("Failed to open /etc/sysctl.conf")?;
        writeln!(f, "net.core.default_qdisc=fq")?;
        writeln!(f, "net.ipv4.tcp_congestion_control=bbr")?;
        run_cmd("sysctl", &["-p"])?;
        log::info!("BBR enabled");
    }
    Ok(())
}

fn generate_credentials() -> Result<()> {
    // Clean up any existing keys file
    let _ = fs::remove_file(KEYS_PATH);
    fs::create_dir_all(
        std::path::Path::new(KEYS_PATH)
            .parent()
            .unwrap_or(std::path::Path::new("/")),
    )?;

    let short_id = cmd_output("openssl", &["rand", "-hex", "8"])?;
    let uuid = cmd_output("xray", &["uuid"])?;

    // xray x25519 prints "PrivateKey: ..." and "Password: ..." (the public key)
    let x25519_out = Command::new("xray")
        .arg("x25519")
        .output()
        .context("Failed to run xray x25519")?;
    let x25519 = String::from_utf8_lossy(&x25519_out.stdout);

    let mut file = fs::File::create(KEYS_PATH).context("Failed to create .keys file")?;
    writeln!(file, "shortsid: {short_id}")?;
    writeln!(file, "uuid: {uuid}")?;
    write!(file, "{x25519}")?;

    Ok(())
}

fn write_initial_config(keys: &Keys) -> Result<()> {
    let cfg = json!({
        "log": {
            "loglevel": "warning"
        },
        "routing": {
            "domainStrategy": "IPIfNonMatch",
            "rules": [
                {
                    "type": "field",
                    "domain": ["geosite:category-ads-all"],
                    "outboundTag": "block"
                }
            ]
        },
        "inbounds": [
            {
                "listen": "0.0.0.0",
                "port": 443,
                "protocol": "vless",
                "settings": {
                    "clients": [
                        {
                            "email": "main",
                            "id": keys.uuid,
                            "flow": "xtls-rprx-vision"
                        }
                    ],
                    "decryption": "none"
                },
                "streamSettings": {
                    "network": "tcp",
                    "security": "reality",
                    "realitySettings": {
                        "show": false,
                        "dest": "github.com:443",
                        "xver": 0,
                        "serverNames": ["github.com", "www.github.com"],
                        "privateKey": keys.private_key,
                        "minClientVer": "",
                        "maxClientVer": "",
                        "maxTimeDiff": 0,
                        "shortIds": [keys.short_id]
                    }
                },
                "sniffing": {
                    "enabled": true,
                    "destOverride": ["http", "tls"]
                }
            }
        ],
        "outbounds": [
            {
                "protocol": "freedom",
                "tag": "direct"
            },
            {
                "protocol": "blackhole",
                "tag": "block"
            }
        ],
        "policy": {
            "levels": {
                "0": {
                    "handshake": 3,
                    "connIdle": 180
                }
            }
        }
    });

    // Ensure the directory exists
    if let Some(parent) = std::path::Path::new(CONFIG_PATH).parent() {
        fs::create_dir_all(parent)?;
    }

    write_config(&cfg)
}
