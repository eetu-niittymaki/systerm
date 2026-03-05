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
    /// Display CPU usage graph
    CPU,
    /// Display GPU usage graph
    GPU,
    /// Display memory usage graph
    Memory,
    /// Display disk usage information
    Disk,
    /// Display operating system information
    Os,
    /// Display network information
    Network
}