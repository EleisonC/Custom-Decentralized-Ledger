mod models;
mod ecrypto;

use models::{Cli, Commands};
use ecrypto::generate_private_key;
use clap::Parser;

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Generate => {
            if let Err(e) = generate_private_key() {
                eprintln!("Error generating private key: {}", e);
            }
        }
    }
}
