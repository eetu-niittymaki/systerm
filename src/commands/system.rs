use sysinfo::System;

use ratatui::{
    layout::{Alignment, Constraint, Layout, Rect},
    style::Stylize,
    widgets::{Block, Borders, Paragraph, Wrap},
    Frame,
};

use super::Command;

pub struct SystemView {
    name: String,
    kernel: String,
    os_version: String,
    host: String
}

impl SystemView {
    pub fn new() -> Self {
        let sys_name = System::name();
        let sys_kernel = System::kernel_version();
        let sys_os_version = System::os_version();
        let sys_host = System::host_name();

        Self {
            name: sys_name.unwrap(),
            kernel: sys_kernel.unwrap(),
            os_version: sys_os_version.unwrap(),
            host: sys_host.unwrap(),  
        }
    }

}

impl Command for SystemView {
    fn tick(&mut self) {}

    fn draw(&self, frame: &mut Frame, _area: Rect) {
        let (title_area, layout) = calculate_layout(frame.area());

        render_title(frame, title_area);

        let name = string_to_paragraph(self.name.clone());
        let kernel = string_to_paragraph(self.kernel.clone());
        let os_version = string_to_paragraph(self.os_version.clone());
        let host = string_to_paragraph(self.host.clone());

        render_borders("OS Name", &name, Borders::ALL, frame, layout[0][0]);
        render_borders("Kernel", &kernel, Borders::ALL, frame, layout[0][1]);
        render_borders("OS Version", &os_version, Borders::ALL, frame, layout[1][0]);
        render_borders("Host", &host, Borders::ALL, frame, layout[1][1]);
    }
}

fn calculate_layout(area: Rect) -> (Rect, Vec<Vec<Rect>>) {
    let main_layout = Layout::vertical([Constraint::Length(1), Constraint::Min(0)]);
    let block_layout = Layout::vertical([Constraint::Max(4); 9]);
    let [title_area, main_area] = main_layout.areas(area);
    let main_areas = block_layout
        .split(main_area)
        .iter()
        .map(|&area| {
            Layout::horizontal([Constraint::Percentage(50), Constraint::Percentage(50)])
                .split(area)
                .to_vec()
        })
        .collect();
    (title_area, main_areas)
}

fn render_title(frame: &mut Frame, area: Rect) {
    frame.render_widget(
        Paragraph::new("System Info. Press 'q' to quit")
            .dark_gray()
            .alignment(Alignment::Center),
        area,
    );
}

fn string_to_paragraph(text: String) -> Paragraph<'static> {
    Paragraph::new(text.dark_gray()).wrap(Wrap { trim: true })
}

fn render_borders(title: &str, paragraph: &Paragraph, border: Borders, frame: &mut Frame, area: Rect) {
    let block = Block::new()
        .borders(border)
        .title(format!("{title}"));
    frame.render_widget(paragraph.clone().block(block), area);
}