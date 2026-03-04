// Interface for CLI

use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "systerm")]
#[command(about = "CLI tool to visualize system info in the terminal")]
#[command(arg_required_else_help = true)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands
}

#[derive(Subcommand, Clone)]
pub enum Commands {
    CPU,
    GPU,
    Memory,
    Disk,
    Os
}