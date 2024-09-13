mod models;
mod ecrypto;
mod utils;

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
        Commands::Display { file_path } => {
            if let Err(e) = utils::display_file_content(file_path) {
                eprintln!("Error displaying file content: {}", e);
            }
        }

    }
}
