use crate::metrics::snapshot::MetricSnapshot;
use ratatui::{
    Frame,
    prelude::*,
    widgets::{Block, Borders, Gauge},
};

pub fn render(f: &mut Frame, snapshot: &MetricSnapshot, area: Rect) {
    let gauge = Gauge::default()
        .block(Block::default().title("CPU Usage").borders(Borders::ALL))
        .gauge_style(Style::default().fg(Color::Green))
        .ratio(snapshot.cpu_usage as f64 / 100.0);
    f.render_widget(gauge, area);
}
