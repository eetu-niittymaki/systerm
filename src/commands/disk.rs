use sysinfo::Disks;

use ratatui::{
    Frame,
    layout::{Alignment, Constraint, Layout, Rect},
    style::Stylize,
    widgets::{Block, Borders, Paragraph, Wrap},
};

use super::Command;

pub struct DiskView {
    disks: Vec<DiskInfo>,
}

#[derive(Clone, Debug)]
pub struct DiskInfo {
    pub name: String,
    pub disk_type: String,
    pub total_space_gb: u64,
    pub available_space_gb: u64,
    pub file_system: String,
}

impl DiskView {
    pub fn new() -> Self {
        Self { disks: Vec::new() }
    }
}

impl Command for DiskView {
    fn tick(&mut self) {
        let disks_list = Disks::new_with_refreshed_list();

        self.disks = disks_list
            .list()
            .iter()
            .map(|disk| DiskInfo {
                name: disk.name().to_string_lossy().into_owned(),
                disk_type: format!("{:?}", disk.kind()),
                total_space_gb: disk.total_space() / 1073741824,
                available_space_gb: disk.available_space() / 1073741824,
                file_system: disk.file_system().to_string_lossy().into_owned(),
            })
            .collect();
    }

    fn draw(&self, frame: &mut Frame, area: Rect) {
        let (title_area, disk_areas) = calculate_layout(area, self.disks.len());

        render_title(frame, title_area);

        for (disk, disk_area) in self.disks.iter().zip(disk_areas.iter()) {
            let used = disk.total_space_gb - disk.available_space_gb;
            let percent = if disk.total_space_gb > 0 {
                (used as f64 / disk.total_space_gb as f64) * 100.0
            } else {
                0.0
            };

            let content = format!(
                " ({})\n Total: {} GB\n Free: {} GB\n Used: {:.1}%\n File System: {}",
                disk.disk_type,
                disk.total_space_gb,
                disk.available_space_gb,
                percent,
                disk.file_system
            );

            let paragraph = string_to_paragraph(content);

            render_borders(&disk.name, &paragraph, Borders::ALL, frame, *disk_area);
        }
    }
}

fn calculate_layout(area: Rect, disk_count: usize) -> (Rect, Vec<Rect>) {
    let main_layout = Layout::vertical([
        Constraint::Length(1),
        Constraint::Min(0),
    ]);

    let [title_area, main_area] = main_layout.areas(area);

    // Calculate number of rows (2 disks per row)
    let rows = (disk_count + 1) / 2;

    // Create vertical row constraints
    let row_constraints = vec![Constraint::Length(4); rows];

    let row_areas = Layout::vertical(row_constraints).split(main_area);

    let mut disk_areas = Vec::new();

    for row in row_areas .iter() {
        let cols = Layout::horizontal([
            Constraint::Percentage(50),
            Constraint::Percentage(50),
        ])
        .split(*row);

        disk_areas.push(cols[0]);
        disk_areas.push(cols[1]);
    }

    // If disk_count is odd, remove extra last area
    disk_areas.truncate(disk_count);

    (title_area, disk_areas)
}

fn render_title(frame: &mut Frame, area: Rect) {
    frame.render_widget(
        Paragraph::new("Disk Drive Info. Press 'q' to quit")
            .dark_gray()
            .alignment(Alignment::Center),
        area,
    );
}

fn string_to_paragraph(text: String) -> Paragraph<'static> {
    Paragraph::new(text.dark_gray()).wrap(Wrap { trim: true })
}

fn render_borders(
    title: &str,
    paragraph: &Paragraph,
    border: Borders,
    frame: &mut Frame,
    area: Rect,
) {
    let block = Block::new().borders(border).title(format!("{title}"));
    frame.render_widget(paragraph.clone().block(block), area);
}
