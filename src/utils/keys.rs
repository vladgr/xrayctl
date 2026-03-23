use anyhow::{Context, Result};

use super::{constants::KEYS_PATH, entities::Keys};

/// Parse the .keys file written during install.
/// Handles both "PrivateKey:" / "Password:" (xray x25519 style) and
/// "Private key:" / "Public key:" variants.
pub fn read_keys() -> Result<Keys> {
    let content = std::fs::read_to_string(KEYS_PATH)
        .context("Failed to read .keys file — is Xray installed?")?;

    let mut uuid = None;
    let mut private_key = None;
    let mut public_key = None;
    let mut short_id = None;

    for line in content.lines() {
        let line = line.trim();
        if let Some(v) = line.strip_prefix("uuid: ") {
            uuid = Some(v.to_string());
        } else if let Some(v) = line
            .strip_prefix("PrivateKey: ")
            .or_else(|| line.strip_prefix("Private key: "))
        {
            private_key = Some(v.to_string());
        } else if let Some(v) = line
            .strip_prefix("Password: ")
            .or_else(|| line.strip_prefix("Public key: "))
        {
            public_key = Some(v.to_string());
        } else if let Some(v) = line.strip_prefix("shortsid: ") {
            short_id = Some(v.to_string());
        }
    }

    Ok(Keys {
        uuid: uuid.context("uuid not found in .keys")?,
        private_key: private_key.context("PrivateKey not found in .keys")?,
        public_key: public_key.context("Password/PublicKey not found in .keys")?,
        short_id: short_id.context("shortsid not found in .keys")?,
    })
}
