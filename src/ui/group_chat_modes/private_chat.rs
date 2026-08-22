use ratatui::{
    Frame,
    layout::Rect,
    style::{Color, Style},
    widgets::Block,
};

pub fn draw_host(frame: &mut Frame, inner: Rect) {
    frame.render_widget(Block::new().style(Style::default().bg(Color::Black)), inner);
}
