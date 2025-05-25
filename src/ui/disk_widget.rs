use crate::metrics::snapshot::MetricSnapshot;
use ratatui::{
    Frame,
    prelude::*,
    widgets::{Block, Borders, Paragraph},
};

pub fn render(f: &mut Frame, snapshot: &MetricSnapshot, area: Rect) {
    let text = format!(
        "Disk Read: {} B/s | Write: {} B/s",
        snapshot.disk_read, snapshot.disk_write
    );
    let paragraph =
        Paragraph::new(text).block(Block::default().title("Disk IO").borders(Borders::ALL));
    f.render_widget(paragraph, area);
}
