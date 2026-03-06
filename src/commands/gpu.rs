use wgpu::Instance;

use ratatui::{
    Frame, layout::{Alignment, Constraint, Layout, Rect}, 
    style::{Color, Modifier, Style, Stylize}, 
    text::Span, 
    widgets::{Axis, Block, Borders, Chart, Dataset, GraphType, Paragraph, Wrap}
};
use super::Command;

pub struct GpuView {
    gpu_name: String,
    gpu_vendor: String,
    gpu_driver: String,
}

impl GpuView {
    #[tokio::main]
    pub async fn new() -> Self {
        let instance = Instance::default();
        let adapters = instance.enumerate_adapters(wgpu::Backends::all()).await;
        let adapter = adapters.first().expect("No GPU adapters found");
        let info = adapter.get_info();

        let vendor = match info.vendor {
            0x10DE => "NVIDIA",
            0x1002 | 0x1022 => "AMD",
            0x8086 => "Intel",
            _ => "Unknown"

        };

        Self {
            gpu_name: info.name,
            gpu_vendor: vendor.to_string(),
            gpu_driver: info.driver,
        }
    }
}

impl Command for GpuView {
    fn tick(&mut self) {}

    fn draw(&self, frame: &mut Frame, area: Rect) {
        let (title_area, layout) = calculate_layout(frame.area());
        render_title(frame, title_area);

        let name = string_to_paragraph(self.gpu_name.clone());
        let vendor = string_to_paragraph(self.gpu_vendor.clone());
        let driver = string_to_paragraph(self.gpu_driver.clone());

        render_borders("GPU", &name, Borders::ALL, frame, layout[0][0]);
        render_borders("Vendor", &vendor, Borders::ALL, frame, layout[0][1]);
        render_borders("Driver", &driver, Borders::ALL, frame, layout[1][0]);
 
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
        Paragraph::new("GPU Info. Press 'q' to quit")
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