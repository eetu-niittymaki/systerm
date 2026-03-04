use sysinfo::System;

use ratatui::{
    layout::Rect,
    style::{Color, Modifier, Style},
    text::{Span, Line},
    widgets::{Axis, Block, Chart, Dataset, GraphType},
    Frame,
};

use super::Command;

pub struct MemoryView {
    sys: System,
    data: Vec<(f64, f64)>,
    window: [f64; 2],
    x: f64,
}

impl MemoryView {
    pub fn new() -> Self {
        let mut sys = System::new_all();
        sys.refresh_all();

        Self {
            sys,
            data: Vec::with_capacity(120),
            window: [0.0, 120.0],
            x: 0.0,
        }
    }

    fn push_point(&mut self, value: f64) {
        self.data.push((self.x, value));

        if self.data.len() >= 120 {
            self.data.remove(0);
        }

        self.x += 1.0;
        self.window = [
            (self.x - 120.0).max(0.0),
            self.x,
        ];
    }
}

impl Command for MemoryView {
    fn tick(&mut self) {
        self.sys.refresh_memory();

        let total_mb = self.sys.total_memory() as f64 / 1024.0 / 1024.0;
        let used_mb = self.sys.used_memory() as f64 / 1024.0 / 1024.0;
        let percent_used = (used_mb / total_mb) * 100.0;

        self.push_point(percent_used);
    }

    fn draw(&self, frame: &mut Frame, area: Rect) {
        let total_mb = self.sys.total_memory()  / 1024 / 1024;
        let free_mb = total_mb - (self.sys.used_memory() / 1024 / 1024) ;
        let min_val = self.data.iter().map(|(_, y)| *y).fold(f64::INFINITY, f64::min);
        let max_val = self.data.iter().map(|(_, y)| *y).fold(f64::NEG_INFINITY, f64::max);
        let bounds = [min_val * 0.95, max_val * 1.05];


        let dataset = Dataset::default()
            .graph_type(GraphType::Line)
            .style(Style::default().fg(Color::Blue))
            .data(&self.data);
        
        let chart = Chart::new(vec![dataset])
            .block(Block::bordered().title("Memory Usage MB"))
            .x_axis(
                Axis::default()
                    .bounds(self.window)
                    .labels(vec![
                        Span::styled(
                            format!("Total Memory: {} MB", self.sys.total_memory() / 1024 / 1024),
                            Style::default()
                                .fg(Color::Green)
                                .add_modifier(Modifier::BOLD),
                        ),
                        Span::styled(
                            format!("{:.0}  MB Free", free_mb),
                            Style::default()
                                .fg(Color::Green)
                                .add_modifier(Modifier::BOLD),
                        ),
                        Span::styled("Press 'q' to quit", Style::default().fg(Color::Red))
                    ]),
            )
           .y_axis(
                Axis::default()
                    .bounds(bounds)
                    .labels(vec![
                        Line::from(format!("0 MB")),
                        Line::from(format!("{:.0} MB", total_mb / 2)),
                        Line::from(format!("{:.0} MB", total_mb)),
                    ]),
            );

        frame.render_widget(chart, area);
    }
}