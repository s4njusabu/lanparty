use ratatui::{
    Frame,
    layout::{Alignment, Constraint, Layout, Margin, Rect},
    style::{Modifier, Style},
    text::{Line, Span},
    widgets::{Block, BorderType, Paragraph, Wrap},
};

use crate::states::{group_chat_state::GroupChatHostState, ui_state::UiState};
pub fn draw_host(
    frame: &mut Frame,
    inner: Rect,
    ui_state: &mut UiState,
    gc_host_state: &GroupChatHostState,
) {
    let colors = ui_state.theme.colors();

    let area = inner.inner(Margin {
        horizontal: 1,
        vertical: 0,
    });

    let [chat_area, input_area] =
        Layout::vertical([Constraint::Min(1), Constraint::Length(5)]).areas(area);

    let [messages_area, info_area] =
        Layout::horizontal([Constraint::Min(1), Constraint::Length(30)]).areas(chat_area);

    // Messages
    let messages_block = Block::bordered()
        .title(
            Line::from(" Messages ").style(
                Style::default()
                    .fg(colors.text)
                    .add_modifier(Modifier::BOLD),
            ),
        )
        .title_alignment(Alignment::Center)
        .border_type(BorderType::Thick)
        .border_style(Style::default().fg(colors.accent));

    let messages_inner = messages_block.inner(messages_area);

    frame.render_widget(messages_block, messages_area);

    let mut messages = Vec::new();

    for message in &gc_host_state.messages {
        let username = gc_host_state
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

    let messages_rect = messages_inner.inner(Margin {
        horizontal: 3,
        vertical: 1,
    });

    let rect_width = messages_rect.width.max(1) as usize;

    let wrapped_lines: u16 = messages
        .iter()
        .map(|line| line.width().max(1).div_ceil(rect_width) as u16)
        .sum();

    let max_scroll = wrapped_lines.saturating_sub(messages_rect.height);

    ui_state.chat_max_scroll = max_scroll;

    let scroll = if ui_state.chat_at_bottom {
        max_scroll
    } else {
        ui_state.chat_scroll.min(max_scroll)
    };

    frame.render_widget(
        Paragraph::new(messages)
            .wrap(Wrap { trim: false })
            .scroll((scroll, 0)),
        messages_rect,
    );

    // Users
    let users_block = Block::bordered()
        .title(
            Line::from(" Users ").style(
                Style::default()
                    .fg(colors.text)
                    .add_modifier(Modifier::BOLD),
            ),
        )
        .border_type(BorderType::Thick)
        .border_style(Style::default().fg(colors.accent));

    let users_inner = users_block.inner(info_area);

    frame.render_widget(users_block, info_area);

    let mut users = Vec::new();

    for (ip, user) in &gc_host_state.users {
        let status_color = if user.online {
            ratatui::style::Color::LightGreen
        } else {
            ratatui::style::Color::LightRed
        };

        users.push(Line::from(vec![
            Span::styled("● ", Style::default().fg(status_color)),
            Span::styled(
                &user.username,
                Style::default()
                    .fg(colors.text)
                    .add_modifier(Modifier::BOLD),
            ),
        ]));

        users.push(Line::styled(
            format!("  {ip}"),
            Style::default().fg(colors.text),
        ));

        users.push(Line::default());
    }

    frame.render_widget(
        Paragraph::new(users),
        users_inner.inner(Margin {
            horizontal: 2,
            vertical: 1,
        }),
    );

    // Input
    let input_block = Block::bordered()
        .title(
            Line::from(format!(" Input - {} ", ui_state.username)).style(
                Style::default()
                    .fg(colors.text)
                    .add_modifier(Modifier::BOLD),
            ),
        )
        .border_type(BorderType::Double)
        .border_style(Style::default().fg(colors.accent));

    let input_inner = input_block.inner(input_area);

    frame.render_widget(input_block, input_area);

    let millis = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_millis();

    let cursor_visible = (millis / 500).is_multiple_of(2);

    let input = if cursor_visible {
        format!("{}█", ui_state.input)
    } else {
        ui_state.input.clone()
    };

    frame.render_widget(
        Paragraph::new(input)
            .style(Style::default().fg(colors.text))
            .wrap(Wrap { trim: false }),
        input_inner.inner(Margin {
            horizontal: 1,
            vertical: 1,
        }),
    );
}
