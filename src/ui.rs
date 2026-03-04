use std::time::{Duration, Instant};

use color_eyre::Result;
use crossterm::event::{self, Event, KeyCode};
use ratatui::DefaultTerminal;

use crate::cli::Commands;
use crate::commands::{
    Command, 
    cpu::CpuView, 
    memory::MemoryView,
    disk::DiskView,
    system::SystemView,
};

pub struct Ui {
    active: Box<dyn Command>,
}

impl Ui {
    pub fn new(command: Commands) -> Self {
        let active: Box<dyn Command> = match command {
            Commands::CPU => Box::new(CpuView::new()),
            Commands::Memory => Box::new(MemoryView::new()),
            Commands::GPU => todo!(),
            Commands::Disk => Box::new(DiskView::new()),
            Commands::System => Box::new(SystemView::new())
        };

        Self { active }
    }

    pub fn run(mut self, mut terminal: DefaultTerminal) -> Result<()> {
        let tick_rate = Duration::from_millis(500);
        let mut last_tick = Instant::now();

        loop {
            terminal.draw(|frame| {
                let area = frame.area();
                self.active.draw(frame, area);
            })?;

            let timeout = tick_rate.saturating_sub(last_tick.elapsed());

            if event::poll(timeout)? {
                let ev = event::read()?;

                if let Event::Key(key) = ev {
                    if key.code == KeyCode::Char('q') {
                        return Ok(());
                    }
                }

            }

            if last_tick.elapsed() >= tick_rate {
                self.active.tick();
                last_tick = Instant::now();
            }
        }
    }
}
