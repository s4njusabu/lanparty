use ratatui::{
    Frame,
    layout::{Constraint, Layout, Margin, Rect},
    style::{Modifier, Style},
    text::{Line, Span},
    widgets::{Block, BorderType, Paragraph, Wrap},
};

use crate::app::{server_state::ServerState, ui_state::UiState};

pub fn draw_client(frame: &mut Frame, inner: Rect, ui_state: &UiState, server_state: &ServerState) {
    let colors = ui_state.theme.colors();

    let area = inner.inner(Margin {
        horizontal: 1,
        vertical: 0,
    });

    let [chat_area, input_area] =
        Layout::vertical([Constraint::Min(1), Constraint::Length(5)]).areas(area);

    let [messages_area, info_area] =
        Layout::horizontal([Constraint::Min(1), Constraint::Length(30)]).areas(chat_area);

    let mut messages = Vec::new();

    for message in &server_state.messages {
        let username = server_state
            .users
            .get(&message.sender)
            .map_or("Unknown", |user| user.username.as_str());

        messages.push(Line::from(vec![
            Span::styled(
                username,
                Style::default()
                    .fg(colors.accent)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::styled(": ", Style::default().fg(colors.text)),
            Span::styled(&message.message, Style::default().fg(colors.text)),
        ]));
        messages.push(Line::default());
    }

    let messages_block = Block::bordered()
        .title(
            Line::from(" Messages ").style(
                Style::default()
                    .fg(colors.text)
                    .add_modifier(Modifier::BOLD),
            ),
        )
        .border_type(BorderType::Thick)
        .border_style(Style::default().fg(colors.accent));

    let messages_inner = messages_block.inner(messages_area);

    frame.render_widget(messages_block, messages_area);

    let messages_rect = messages_inner.inner(Margin {
        horizontal: 3,
        vertical: 1,
    });

    let total_lines = messages.len();
    let visible_lines = messages_rect.height as usize;
    let scroll = total_lines.saturating_sub(visible_lines) as u16;

    frame.render_widget(
        Paragraph::new(messages)
            .wrap(Wrap { trim: false })
            .scroll((scroll, 0)),
        messages_rect,
    );

    let mut online = Vec::new();

    for (ip, user) in &server_state.users {
        let status_color = if user.online {
            ratatui::style::Color::LightGreen
        } else {
            ratatui::style::Color::LightRed
        };

        online.push(Line::from(vec![
            Span::styled("● ", Style::default().fg(status_color)),
            Span::styled(
                &user.username,
                Style::default()
                    .fg(colors.text)
                    .add_modifier(Modifier::BOLD),
            ),
        ]));

        online.push(Line::from(Span::styled(
            format!("  {ip}"),
            Style::default().fg(colors.text),
        )));

        online.push(Line::default());
    }

    let online_block = Block::bordered()
        .title(
            Line::from(" Users ").style(
                Style::default()
                    .fg(colors.text)
                    .add_modifier(Modifier::BOLD),
            ),
        )
        .border_type(BorderType::Thick)
        .border_style(Style::default().fg(colors.accent));

    let online_inner = online_block.inner(info_area);

    frame.render_widget(online_block, info_area);

    frame.render_widget(
        Paragraph::new(online),
        online_inner.inner(Margin {
            horizontal: 2,
            vertical: 1,
        }),
    );

    let input_block = Block::bordered()
        .title(
            Line::from(" Input ").style(
                Style::default()
                    .fg(colors.text)
                    .add_modifier(Modifier::BOLD),
            ),
        )
        .border_type(BorderType::Double)
        .border_style(Style::default().fg(colors.accent));

    let input_inner = input_block.inner(input_area);

    frame.render_widget(input_block, input_area);

    frame.render_widget(
        Paragraph::new(ui_state.input.as_str())
            .style(Style::default().fg(colors.text))
            .wrap(Wrap { trim: false }),
        input_inner.inner(Margin {
            horizontal: 1,
            vertical: 1,
        }),
    );
}
