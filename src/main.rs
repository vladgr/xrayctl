use clap::{Parser, Subcommand};

mod cmd;
mod utils;

#[derive(Parser)]
#[command(name = "xrayctl", about = "Xray-core management tool (VLESS+Reality)")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Install Xray-core with VLESS+Reality on port 443
    Install,
    /// Show main user connection link and QR code
    Mainuser,
    /// Add a new user and print their connection link
    Newuser,
    /// Remove an existing user
    Rmuser,
    /// Show connection link and QR code for any user
    Sharelink,
    /// List all configured users
    Listusers,
}

fn main() {
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info"))
        .format_timestamp(None)
        .init();

    let cli = Cli::parse();
    let result = match cli.command {
        Commands::Install => cmd::install::run(),
        Commands::Mainuser => cmd::mainuser::run(),
        Commands::Newuser => cmd::newuser::run(),
        Commands::Rmuser => cmd::rmuser::run(),
        Commands::Sharelink => cmd::sharelink::run(),
        Commands::Listusers => cmd::listusers::run(),
    };
    if let Err(e) = result {
        log::error!("{e}");
        std::process::exit(1);
    }
}
