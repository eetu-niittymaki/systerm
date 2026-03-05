use std::{thread, time::Duration};
use sysinfo::Networks;

use ratatui::{
    Frame,
    layout::{Constraint, Layout, Rect},
    style::{Color, Style, Stylize},
    text::Line,
    widgets::{Block, Borders, Gauge, Paragraph},
};


use super::Command;

pub struct NetworkView {
    networks: Vec<NetworkInfo>,
}

pub struct NetworkInfo {
    pub interface_name: String,
    pub received: u64,
    pub transmitted: u64,
    pub max_download: u64,
    pub max_upload: u64,
}

impl NetworkView {
    pub fn new() -> Self {
        Self {
            networks: Vec::new(),
        }
    }
}

impl Command for NetworkView {
    fn tick(&mut self) {
        let mut networks_list = Networks::new_with_refreshed_list();
        thread::sleep(Duration::from_secs(1));
        networks_list.refresh(true);

        self.networks = networks_list
            .iter()
            .map(|(interface, data)| {
                let interface_name = interface.to_string();

                let prev = self
                    .networks
                    .iter()
                    .find(|n| n.interface_name == interface_name);

                let received = data.received();
                let transmitted = data.transmitted();

                // current download speed in bytes
                let current_speed_download = if let Some(prev) = prev {
                    received.saturating_sub(prev.received)
                } else {
                    0
                };

                // current upload speed in bytes
                let current_speed_upload = if let Some(prev) = prev {
                    transmitted.saturating_sub(prev.transmitted)
                } else {
                    0
                };

                // max download in bytes
                let max_download = if let Some(prev) = prev {
                    let prev_max_down_bytes = (prev.max_download) as u64;
                    prev_max_down_bytes.max(current_speed_download)
                } else {
                    current_speed_download
                };

                // max upload in bytes
                let max_upload = if let Some(prev) = prev {
                    let prev_max_up_bytes = (prev.max_upload) as u64;
                    prev_max_up_bytes.max(current_speed_upload)
                } else {
                    current_speed_upload
                };

                NetworkInfo {
                    interface_name,
                    received,
                    transmitted,
                    max_download,
                    max_upload,
                }
            })
            .collect();
    }

    fn draw(&self, frame: &mut Frame, area: Rect) {
        // Create vertical rows (3 per interface: name, rx, tx)
        let mut constraints = Vec::new();
        for _ in &self.networks {
            constraints.push(Constraint::Length(1)); // title
            constraints.push(Constraint::Length(1)); // spacing
            constraints.push(Constraint::Length(1)); // name
            constraints.push(Constraint::Length(1)); // max download
            constraints.push(Constraint::Length(1)); // max upload
            constraints.push(Constraint::Length(1)); // rx
            constraints.push(Constraint::Length(1)); // tx
            constraints.push(Constraint::Length(1)); // spacing
        }

        let rows = Layout::vertical(constraints).split(area);

        let mut i = 0;

        for net in &self.networks {
            // Convertt bytes to mb and round to three trailing digits
            let max_download_mb = ((net.max_download as f64 / 1024.0 / 1024.0) * 1000.0).ceil() / 1000.0;
            let max_upload_mb = ((net.max_upload as f64 / 1024.0 / 1024.0) * 1000.0).ceil() / 1000.0;

            let title = Paragraph::new(Line::from("Press 'q' to quit"));
            frame.render_widget(title.bold(), rows[i]);
            i += 2;

            let name = Paragraph::new(Line::from(net.interface_name.clone()));
            frame.render_widget(name, rows[i]);
            i += 1;

            let max_dl = Paragraph::new(Line::from(format!("Max Download: {} MB/s", max_download_mb)));
            frame.render_widget(max_dl, rows[i]);
            i += 1;

            let max_ul = Paragraph::new(Line::from(format!("Max Upload: {} MB/s", max_upload_mb)));
            frame.render_widget(max_ul, rows[i]);
            i += 1;

            // RX gauge
            let rx_ratio = if net.max_download == 0 {
                0.0
            } else {
                net.received as f64 / net.max_download as f64
            };

            let rx = Gauge::default()
                .block(Block::default().borders(Borders::NONE))
                .gauge_style(Style::default().fg(Color::Green))
                .ratio(rx_ratio.min(1.0))
                .label(format!("Download {} B/s", net.received));

            frame.render_widget(rx, rows[i]);
            i += 1;

            // TX gauge
            let tx_ratio = if net.max_upload == 0 {
                0.0
            } else {
                net.transmitted as f64 / net.max_upload as f64
            };

            let tx = Gauge::default()
                .block(Block::default().borders(Borders::NONE))
                .gauge_style(Style::default().fg(Color::Yellow))
                .ratio(tx_ratio.min(1.0))
                .label(format!("Upload {} B/s", net.transmitted));

            frame.render_widget(tx, rows[i]);
            i += 1;

            i += 1;
        }
    }
}
