use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "nom")]
#[command(about = "npm wrapper that tracks typos", long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    #[command(external_subcommand)]
    Npm(Vec<String>),
}
