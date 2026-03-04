mod commands;
mod cli;
mod ui;

use clap::Parser;
use cli::Cli;
use color_eyre::Result;

fn main() -> Result <()> {
    color_eyre::install()?;
    let cli = Cli::parse();

    let terminal = ratatui::init();
    let ui = ui::Ui::new(cli.command);
    let result = ui.run(terminal);
    ratatui::restore();

    result
}
