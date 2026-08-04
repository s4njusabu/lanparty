use std::io::ErrorKind;

use ratatui::{
    Frame,
    layout::{Alignment, Constraint, Flex, Layout, Margin, Rect},
    style::{Modifier, Style},
    text::{Line, Text},
    widgets::{Block, BorderType, Paragraph},
};

use crate::app::state::State;

pub fn draw_error_page(frame: &mut Frame, inner: Rect, state: &State, err: ErrorKind) {
    let colors = state.theme.colors();

    let area = inner.inner(Margin {
        horizontal: 1,
        vertical: 0,
    });

    frame.render_widget(
        Block::bordered()
            .border_type(BorderType::Double)
            .border_style(Style::default().fg(colors.accent)),
        area,
    );

    let error = match err {
        ErrorKind::AddrInUse => "Port 55555 is already in use",
        ErrorKind::PermissionDenied => "Permission denied",
        _ => "Something went wrong",
    };

    let text = Text::from(vec![
        Line::from("Unable to start the server").style(
            Style::default()
                .fg(colors.accent)
                .add_modifier(Modifier::BOLD),
        ),
        Line::from(""),
        Line::from(error).style(
            Style::default()
                .fg(colors.accent)
                .add_modifier(Modifier::BOLD),
        ),
        Line::from(""),
        Line::from("Press any key to exit").style(
            Style::default()
                .fg(colors.accent)
                .add_modifier(Modifier::BOLD),
        ),
    ]);

    let [text_area] = Layout::vertical([Constraint::Length(5)])
        .flex(Flex::Center)
        .areas(area);

    frame.render_widget(Paragraph::new(text).alignment(Alignment::Center), text_area);
}
