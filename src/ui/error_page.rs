use std::io::ErrorKind;

use ratatui::{
    Frame,
    layout::{Alignment, Constraint, Flex, Layout, Margin, Rect},
    style::{Modifier, Style},
    text::{Line, Text},
    widgets::{Block, BorderType, Paragraph},
};

use crate::states::ui_state::UiState;

pub fn draw_error_page(frame: &mut Frame, inner: Rect, ui_state: &UiState, err: &std::io::Error) {
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

    let error = match err.kind() {
        ErrorKind::AddrInUse => "Port 55555 is already in use".to_string(),
        ErrorKind::PermissionDenied => "Permission denied".to_string(),
        ErrorKind::AddrNotAvailable => "No valid network interface found".to_string(),
        ErrorKind::ConnectionRefused => "Couldnt connect to the host".to_string(),
        ErrorKind::ConnectionReset => "Connection was lost".to_string(),
        ErrorKind::ConnectionAborted => "Connection was aborted".to_string(),
        ErrorKind::NotConnected => "Not connected to a server".to_string(),
        ErrorKind::TimedOut => "Connection timed out".to_string(),
        ErrorKind::BrokenPipe => "Connection was closed".to_string(),
        ErrorKind::HostUnreachable => "Host is unreachable".to_string(),
        ErrorKind::NetworkUnreachable => "Network is unreachable".to_string(),
        ErrorKind::InvalidData => "Received invalid data".to_string(),
        ErrorKind::InvalidInput => "Invalid input".to_string(),
        ErrorKind::NotFound => "Required resource not found".to_string(),
        ErrorKind::Other => err.to_string(),
        _ => "Something went wrong".to_string(),
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
