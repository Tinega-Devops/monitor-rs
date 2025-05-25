use crate::metrics::snapshot::MetricSnapshot;
use ratatui::{
    Frame,
    prelude::*,
    widgets::{Block, Borders, Gauge},
};

pub fn render(f: &mut Frame, snapshot: &MetricSnapshot, area: Rect) {
    let used = snapshot.used_memory;
    let total = snapshot.total_memory;
    let ratio = used as f64 / total as f64;

    let gauge = Gauge::default()
        .block(Block::default().title("Memory Usage").borders(Borders::ALL))
        .gauge_style(Style::default().fg(Color::Blue))
        .ratio(ratio);
    f.render_widget(gauge, area);
}
