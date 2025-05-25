use crate::metrics::snapshot::MetricSnapshot;
use ratatui::{
    Frame,
    prelude::*,
    widgets::{Block, Borders, Paragraph},
};

pub fn render(f: &mut Frame, snapshot: &MetricSnapshot, area: Rect) {
    let text = format!(
        "Net Rx: {} B/s | Tx: {} B/s",
        snapshot.net_rx, snapshot.net_tx
    );
    let paragraph =
        Paragraph::new(text).block(Block::default().title("Network IO").borders(Borders::ALL));
    f.render_widget(paragraph, area);
}
