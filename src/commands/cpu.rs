use sysinfo::System;

use ratatui::{
    layout::Rect,
    style::{Color, Modifier, Style, Stylize},
    text::Span,
    widgets::{Axis, Block, Chart, Dataset, GraphType},
    Frame,
};

use super::Command;

pub struct CpuView {
    sys: System,
    data: Vec<(f64, f64)>,
    window: [f64; 2],
    x: f64,
}

impl CpuView {
    pub fn new() -> Self {
        let mut sys = System::new_all();
        sys.refresh_cpu_usage();
        std::thread::sleep(sysinfo::MINIMUM_CPU_UPDATE_INTERVAL);

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

impl Command for CpuView {
    fn tick(&mut self) {
        self.sys.refresh_cpu_usage();

        let avg = self
            .sys
            .cpus()
            .iter()
            .map(|c| c.cpu_usage())
            .sum::<f32>()
            / self.sys.cpus().len() as f32;

        self.push_point(avg as f64);
    }

    fn draw(&self, frame: &mut Frame, area: Rect) {
        let dataset = Dataset::default()
            .graph_type(GraphType::Line)
            .style(Style::default().fg(Color::Blue))
            .data(&self.data);
        
        
        let chart = Chart::new(vec![dataset])
            .block(Block::bordered().title("CPU Usage %"))
            .x_axis(
                Axis::default()
                    .bounds(self.window)
                    .labels(vec![
                        Span::raw(""),
                        Span::styled(
                            format!("{:.1}% Used", self.data.last().map(|(_, y)| *y).unwrap_or(0.0)),
                            Style::default()
                                .fg(Color::Green)
                                .add_modifier(Modifier::BOLD),
                        ),
                        Span::styled("Press 'q' to quit", Style::default().fg(Color::Red))
                    ]),
            )
            .y_axis(
                Axis::default()
                    .bounds([0.0, 100.0])
                    .labels(["0".bold(), "50".into(), "100".bold()]),
            );

        frame.render_widget(chart, area);
    }
}