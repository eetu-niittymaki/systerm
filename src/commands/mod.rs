use ratatui::{Frame, layout::Rect};

pub mod cpu;
pub mod gpu;
pub mod memory;
pub mod disk;
pub mod system;

pub trait Command {
    fn tick(&mut self);
    fn draw(&self, frame: &mut Frame, area: Rect);
}