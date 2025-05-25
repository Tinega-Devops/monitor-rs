use ratatui::{
    Frame,
    layout::{Constraint, Direction, Layout},
};

use super::{cpu_widget, disk_widget, memory_widget, net_widget};
use crate::metrics::snapshot::MetricSnapshot;

pub fn render(f: &mut Frame, snapshot: &MetricSnapshot) {
    let layout = Layout::default()
        .direction(Direction::Vertical)
        .margin(1)
        .constraints([
            Constraint::Length(3),
            Constraint::Length(3),
            Constraint::Length(3),
            Constraint::Length(3),
            Constraint::Min(0),
        ])
        .split(f.area());

    cpu_widget::render(f, snapshot, layout[0]);
    memory_widget::render(f, snapshot, layout[1]);
    disk_widget::render(f, snapshot, layout[2]);
    net_widget::render(f, snapshot, layout[3]);
}
