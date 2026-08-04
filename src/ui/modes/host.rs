use ratatui::{
    Frame,
    layout::{Margin, Rect},
    style::Style,
    widgets::Block,
};

use crate::app::state::State;

pub fn draw_host(frame: &mut Frame, inner: Rect, state: &State) {
    let colors = state.theme.colors();
    let area = inner.inner(Margin {
        horizontal: 1,
        vertical: 0,
    });

    frame.render_widget(
        Block::bordered()
            .border_type(ratatui::widgets::BorderType::Double)
            .border_style(Style::default().fg(colors.accent)),
        area,
    );
}
