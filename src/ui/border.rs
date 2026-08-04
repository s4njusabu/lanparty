use ratatui::{
    Frame,
    layout::Rect,
    style::{Modifier, Style, Stylize},
    text::{Line, Span},
    widgets::{Block, BorderType},
};

use crate::app::ui_state::UiState;

pub fn draw_border(frame: &mut Frame, ui_state: &UiState) -> Rect {
    let colors = ui_state.theme.colors();
    let block = Block::bordered()
        // .title(Line::from(vec![
        //     Span::styled(
        //         " LAN",
        //         Style::default()
        //             .fg(colors.text)
        //             .add_modifier(Modifier::BOLD),
        //     ),
        //     Span::styled(
        //         "Party ",
        //         Style::default()
        //             .fg(colors.text)
        //             .add_modifier(Modifier::BOLD),
        //     ),
        // ]))
        .title(Line::from(
            Span::styled(" LANParty ", colors.text).add_modifier(Modifier::BOLD),
        ))
        .border_type(BorderType::Thick)
        .border_style(Style::default().fg(colors.accent));

    let inner = block.inner(frame.area());

    frame.render_widget(block, frame.area());

    inner
}
