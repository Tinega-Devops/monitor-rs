use ratatui::style::{Color, Style};
#[warn(dead_code)]
pub fn default_style() -> Style {
    Style::default().fg(Color::White).bg(Color::Black)
}
