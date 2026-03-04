use sysinfo::{System, Disks};

use ratatui::{
    layout::Rect,
    style::{Color, Modifier, Style, Stylize},
    text::Span,
    widgets::{Axis, Block, Chart, Dataset, GraphType},
    Frame,
};

use super::Command;

pub struct DiskView {
    sys: System,
    window: [f64; 2],

}

impl DiskView {
    pub fn new() -> Self {
        let sys = System::new_all();

        let sys_name = System::name();
        let sys_kernel = System::kernel_version();
        let sys_os_version = System::os_version();
        let sys_host = System::host_name();

        Self {
            sys,
            window: [0.0, 120.0],

        }
    }

}

impl Command for DiskView {
    fn tick(&mut self) {}

    fn draw(&self, frame: &mut Frame, area: Rect) {

    }
}