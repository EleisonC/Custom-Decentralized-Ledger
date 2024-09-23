use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "PrivateKeyCLI")]
#[command(about  =  "EleisonC CLI to generate and display a private key", long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    Generate,
    Display {
        #[arg(short, long)]
        file_path: String,
    }
}
