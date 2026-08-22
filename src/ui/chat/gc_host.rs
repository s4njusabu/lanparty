use ratatui::{
    Frame,
    layout::{Alignment, Constraint, Layout, Margin, Rect},
    style::{Modifier, Style},
    text::{Line, Span},
    widgets::{Block, BorderType, Paragraph, Wrap},
};

use crate::states::ui_state::UiState;

pub fn draw_host(frame: &mut Frame, inner: Rect, ui_state: &UiState) {
    let colors = ui_state.theme.colors();

    let area = inner.inner(Margin {
        horizontal: 1,
        vertical: 0,
    });

    let [chat_area, input_area] =
        Layout::vertical([Constraint::Min(1), Constraint::Length(5)]).areas(area);

    let [messages_area, info_area] =
        Layout::horizontal([Constraint::Min(1), Constraint::Length(30)]).areas(chat_area);

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

    frame.render_widget(
        Paragraph::new(vec![
            Line::from(vec![
                Span::styled(
                    "Sanju",
                    Style::default()
                        .fg(colors.accent)
                        .add_modifier(Modifier::BOLD),
                ),
                Span::styled(": Hey everyone", Style::default().fg(colors.text)),
            ]),
            Line::default(),
            Line::from(vec![
                Span::styled(
                    "Alex",
                    Style::default()
                        .fg(colors.accent)
                        .add_modifier(Modifier::BOLD),
                ),
                Span::styled(": What's up?", Style::default().fg(colors.text)),
            ]),
            Line::default(),
            Line::from(vec![
                Span::styled(
                    "You",
                    Style::default()
                        .fg(colors.accent)
                        .add_modifier(Modifier::BOLD),
                ),
                Span::styled(": Testing group chat", Style::default().fg(colors.text)),
            ]),
        ])
        .wrap(Wrap { trim: false }),
        messages_inner.inner(Margin {
            horizontal: 3,
            vertical: 1,
        }),
    );

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

    frame.render_widget(
        Paragraph::new(vec![
            Line::from(vec![
                Span::styled("● ", Style::default().fg(ratatui::style::Color::LightGreen)),
                Span::styled(
                    "Sanju",
                    Style::default()
                        .fg(colors.text)
                        .add_modifier(Modifier::BOLD),
                ),
            ]),
            Line::styled("  192.168.1.10", Style::default().fg(colors.text)),
            Line::default(),
            Line::from(vec![
                Span::styled("● ", Style::default().fg(ratatui::style::Color::LightGreen)),
                Span::styled(
                    "Alex",
                    Style::default()
                        .fg(colors.text)
                        .add_modifier(Modifier::BOLD),
                ),
            ]),
            Line::styled("  192.168.1.11", Style::default().fg(colors.text)),
            Line::default(),
            Line::from(vec![
                Span::styled("● ", Style::default().fg(ratatui::style::Color::LightRed)),
                Span::styled(
                    "Bob",
                    Style::default()
                        .fg(colors.text)
                        .add_modifier(Modifier::BOLD),
                ),
            ]),
            Line::styled("  192.168.1.12", Style::default().fg(colors.text)),
        ]),
        users_inner.inner(Margin {
            horizontal: 2,
            vertical: 1,
        }),
    );

    let input_block = Block::bordered()
        .title(
            Line::from(" Input - Sanju ").style(
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
        Paragraph::new("Type your message here...")
            .style(Style::default().fg(colors.text))
            .wrap(Wrap { trim: false }),
        input_inner.inner(Margin {
            horizontal: 1,
            vertical: 1,
        }),
    );
}
