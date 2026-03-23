# xrayctl

A Rust CLI tool for managing Xray server configured with the **VLESS + TCP + Reality** protocol stack.

The following tools are installed automatically by `xrayctl install`:

- `xray` — Xray-core binary
- `curl` — used for IP detection
- `qrencode` — used for QR code output
- `jq` — available system-wide for manual config inspection


## Commands

`xrayctl install`

Installs and configures Xray-core from scratch:

1. Runs `apt update` and installs `qrencode`, `curl`, `jq`
2. Enables BBR congestion control (writes to `/etc/sysctl.conf` if not already active)
3. Downloads and runs the [official Xray install script](https://github.com/XTLS/Xray-install)
4. Generates credentials — UUID, x25519 key pair, short ID — and saves them to `/usr/local/etc/xray/.keys`
5. Writes `/usr/local/etc/xray/config.json` (VLESS inbound on port 443, Reality security, `github.com` as SNI destination)
6. Restarts the `xray` systemd service
7. Prints the main user connection link and QR code

`xrayctl mainuser`

Prints the connection link and QR code for the first (main) user.

`xrayctl newuser`

Interactively adds a new user:

- Prompts for a username (email label, no spaces allowed)
- Generates a new UUID via `xray uuid`
- Appends the new client to `config.json`
- Restarts the `xray` service
- Prints the new user's connection link and QR code

`xrayctl rmuser`

Interactively removes a user:

- Lists all current users numbered
- Prompts for the number of the user to remove
- Removes that client from `config.json`
- Restarts the `xray` service

`xrayctl sharelink`

Generates a connection link and QR code for any existing user:

- Lists all current users numbered
- Prompts for a selection
- Prints the selected user's `vless://` link and QR code

`xrayctl listusers`

Prints a numbered list of all configured usernames.

## File locations

| Path | Description |
|---|---|
| `/usr/local/etc/xray/config.json` | Xray configuration file |
| `/usr/local/etc/xray/.keys` | Generated credentials (UUID, private key, public key, short ID) |

