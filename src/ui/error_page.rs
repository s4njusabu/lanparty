use std::io::ErrorKind;

use ratatui::{
    Frame,
    layout::{Alignment, Constraint, Flex, Layout, Margin, Rect},
    style::{Modifier, Style},
    text::{Line, Text},
    widgets::{Block, BorderType, Paragraph},
};

use crate::states::ui_state::UiState;

pub fn draw_error_page(frame: &mut Frame, inner: Rect, ui_state: &UiState, err: ErrorKind) {
    let colors = ui_state.theme.colors();

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
        ErrorKind::AddrNotAvailable => "No valid network interface found",
        ErrorKind::ConnectionRefused => "Couldn't connect to the host",
        ErrorKind::ConnectionReset => "Connection was lost",
        ErrorKind::ConnectionAborted => "Connection was aborted",
        ErrorKind::NotConnected => "Not connected to a server",
        ErrorKind::TimedOut => "Connection timed out",
        ErrorKind::BrokenPipe => "Connection was closed",
        ErrorKind::HostUnreachable => "Host is unreachable",
        ErrorKind::NetworkUnreachable => "Network is unreachable",
        ErrorKind::InvalidData => "Received invalid data",
        ErrorKind::InvalidInput => "Invalid input",
        ErrorKind::NotFound => "Required resource not found",
        _ => "Something went wrong",
    };

    let text = Text::from(vec![
        Line::from("Unable to continue").style(
            Style::default()
                .fg(colors.text)
                .add_modifier(Modifier::BOLD),
        ),
        Line::from(""),
        Line::from(error).style(
            Style::default()
                .fg(colors.text)
                .add_modifier(Modifier::BOLD),
        ),
        Line::from(""),
        Line::from("Press any key to exit").style(
            Style::default()
                .fg(colors.text)
                .add_modifier(Modifier::BOLD),
        ),
    ]);

    let [text_area] = Layout::vertical([Constraint::Length(5)])
        .flex(Flex::Center)
        .areas(area);

    frame.render_widget(Paragraph::new(text).alignment(Alignment::Center), text_area);
}
