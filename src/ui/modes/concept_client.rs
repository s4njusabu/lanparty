use ratatui::{
    Frame,
    layout::{Constraint, Layout, Margin, Rect},
    style::{Modifier, Style},
    text::{Line, Span},
    widgets::{Block, BorderType, Paragraph, Wrap},
};

use crate::app::state::State;

pub fn draw_client(frame: &mut Frame, inner: Rect, state: &State) {
    let colors = state.theme.colors();

    let area = inner.inner(Margin {
        horizontal: 1,
        vertical: 0,
    });

    let [chat_area, input_area] =
        Layout::vertical([Constraint::Min(1), Constraint::Length(5)]).areas(area);

    let [messages_area, info_area] =
        Layout::horizontal([Constraint::Min(1), Constraint::Length(30)]).areas(chat_area);

    let messages = vec![
        Line::from(vec![
            Span::styled(
                "SilverFox",
                Style::default()
                    .fg(colors.text)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::styled(": Hey everyone.", Style::default().fg(colors.text)),
        ]),
        Line::default(),
        Line::from(vec![
            Span::styled(
                "BlueOtter",
                Style::default()
                    .fg(colors.text)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::styled(": Hi.", Style::default().fg(colors.text)),
        ]),
        Line::default(),
        Line::from(vec![
            Span::styled(
                "AmberWolf",
                Style::default()
                    .fg(colors.text)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::styled(
                ": This UI is looking really clean. Nice work!",
                Style::default().fg(colors.text),
            ),
        ]),
        Line::default(),
        Line::from(vec![
            Span::styled(
                "NightOwl",
                Style::default()
                    .fg(colors.text)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::styled(
                ": Can't wait to test this.",
                Style::default().fg(colors.text),
            ),
        ]),
        Line::default(),
        Line::from(vec![
            Span::styled(
                "CrimsonBear",
                Style::default()
                    .fg(colors.text)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::styled(
                ": Looks good from my machine.",
                Style::default().fg(colors.text),
            ),
        ]),
        Line::default(),
        Line::from(vec![
            Span::styled(
                "SwiftHawk",
                Style::default()
                    .fg(colors.text)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::styled(
                ": Anyone hosting tonight?",
                Style::default().fg(colors.text),
            ),
        ]),
        Line::default(),
        Line::from(vec![
            Span::styled(
                "IronPanda",
                Style::default()
                    .fg(colors.text)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::styled(
                ": I'll join in five minutes.",
                Style::default().fg(colors.text),
            ),
        ]),
        Line::default(),
        Line::from(vec![
            Span::styled(
                "FrostLynx",
                Style::default()
                    .fg(colors.text)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::styled(
                ": Connected successfully!",
                Style::default().fg(colors.text),
            ),
        ]),
    ];
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

    frame.render_widget(
        Paragraph::new(messages).wrap(Wrap { trim: false }),
        messages_inner.inner(Margin {
            horizontal: 3,
            vertical: 1,
        }),
    );

    let online = vec![
        Line::from(vec![
            Span::styled("● ", Style::default().fg(ratatui::style::Color::LightGreen)),
            Span::styled(
                "Sanju",
                Style::default()
                    .fg(colors.text)
                    .add_modifier(Modifier::BOLD),
            ),
        ]),
        Line::from(Span::styled(
            "  192.168.1.12",
            Style::default().fg(colors.text),
        )),
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
        Line::from(Span::styled(
            "  192.168.1.18",
            Style::default().fg(colors.text),
        )),
        Line::default(),
        Line::from(vec![
            Span::styled("● ", Style::default().fg(ratatui::style::Color::LightGreen)),
            Span::styled(
                "Printer",
                Style::default()
                    .fg(colors.text)
                    .add_modifier(Modifier::BOLD),
            ),
        ]),
        Line::from(Span::styled(
            "  192.168.1.25",
            Style::default().fg(colors.text),
        )),
        Line::default(),
        Line::from(vec![
            Span::styled("● ", Style::default().fg(ratatui::style::Color::LightRed)),
            Span::styled(
                "SilverFox",
                Style::default()
                    .fg(colors.text)
                    .add_modifier(Modifier::BOLD),
            ),
        ]),
        Line::from(Span::styled(
            "  192.168.1.31",
            Style::default().fg(colors.text),
        )),
        Line::default(),
        Line::from(vec![
            Span::styled("● ", Style::default().fg(ratatui::style::Color::LightRed)),
            Span::styled(
                "BlueOtter",
                Style::default()
                    .fg(colors.text)
                    .add_modifier(Modifier::BOLD),
            ),
        ]),
        Line::from(Span::styled(
            "  192.168.1.42",
            Style::default().fg(colors.text),
        )),
        Line::default(),
        Line::from(vec![
            Span::styled("● ", Style::default().fg(ratatui::style::Color::LightRed)),
            Span::styled(
                "NightOwl",
                Style::default()
                    .fg(colors.text)
                    .add_modifier(Modifier::BOLD),
            ),
        ]),
        Line::from(Span::styled(
            "  192.168.1.53",
            Style::default().fg(colors.text),
        )),
    ];

    let online_block = Block::bordered()
        .title(
            Line::from(" Online ").style(
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
        Paragraph::new("Lorem ipsum dolor sit amet, consectetur adipiscing elit.")
            .style(Style::default().fg(colors.text))
            .wrap(Wrap { trim: false }),
        input_inner.inner(Margin {
            horizontal: 1,
            vertical: 1,
        }),
    );
}
