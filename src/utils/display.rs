use anyhow::Result;
use serde_json::Value;
use std::process::Command;

use super::entities::Keys;

pub fn build_vless_link(
    config: &Value,
    keys: &Keys,
    email: &str,
    uuid: &str,
    ip: &str,
) -> Result<String> {
    let protocol = config["inbounds"][0]["protocol"]
        .as_str()
        .unwrap_or("vless");
    let port = config["inbounds"][0]["port"].as_u64().unwrap_or(443);
    let sni = config["inbounds"][0]["streamSettings"]["realitySettings"]["serverNames"][0]
        .as_str()
        .unwrap_or("github.com");

    Ok(format!(
        "{protocol}://{uuid}@{ip}:{port}\
         ?security=reality&sni={sni}&fp=firefox\
         &pbk={pbk}&sid={sid}&spx=/\
         &type=tcp&flow=xtls-rprx-vision&encryption=none\
         #{email}",
        pbk = keys.public_key,
        sid = keys.short_id,
    ))
}

pub fn print_qr(link: &str) {
    match Command::new("qrencode")
        .args(["-t", "ansiutf8", link])
        .status()
    {
        Ok(_) => {}
        Err(_) => log::warn!("qrencode not available — skipping QR code"),
    }
}
