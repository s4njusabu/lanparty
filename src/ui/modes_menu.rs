use ratatui::{
    Frame,
    layout::{Alignment, Constraint, Flex, Layout, Margin, Rect},
    style::{Modifier, Style},
    widgets::{Block, BorderType, Paragraph},
};

use crate::app::ui_state::UiState;

pub const MODE_OPTIONS_MAX_INDEX: usize = 1;

pub fn draw_modes_menu(frame: &mut Frame, area: Rect, ui_state: &UiState) {
    let colors = ui_state.theme.colors();

    let [_, title, _, content] = Layout::vertical([
        Constraint::Length(1),
        Constraint::Length(8),
        Constraint::Length(1),
        Constraint::Min(0),
    ])
    .areas(area);

    draw_banner(frame, title, ui_state);

    let content = content.inner(Margin {
        horizontal: 3,
        vertical: 1,
    });

    let [client, host] =
        Layout::horizontal([Constraint::Percentage(50), Constraint::Percentage(50)])
            .spacing(3)
            .areas(content);

    let text_style = Style::default()
        .fg(colors.text)
        .add_modifier(Modifier::BOLD);

    let selected_title_style = Style::default()
        .fg(colors.selected)
        .add_modifier(Modifier::BOLD);

    let normal_title_style = Style::default()
        .fg(colors.text)
        .add_modifier(Modifier::BOLD);

    // client
    let client_border = if ui_state.submenu_hovered == Some(0) {
        Style::default()
            .fg(colors.selected)
            .add_modifier(Modifier::BOLD)
    } else {
        Style::default().fg(colors.accent)
    };

    frame.render_widget(
        Block::bordered()
            .title(" Client ")
            .title_style(if ui_state.submenu_hovered == Some(0) {
                selected_title_style
            } else {
                normal_title_style
            })
            .border_type(BorderType::Double)
            .border_style(client_border),
        client,
    );

    let inner = client.inner(Margin {
        horizontal: 2,
        vertical: 1,
    });

    let [_, body, _, footer] = Layout::vertical([
        Constraint::Percentage(18),
        Constraint::Length(8),
        Constraint::Min(0),
        Constraint::Length(1),
    ])
    .areas(inner);

    frame.render_widget(
        Paragraph::new(format!(
            "Join a session\n\n\nUsername: {}\n\n\nNote: An active host is required.",
            ui_state.username
        ))
        .alignment(Alignment::Center)
        .style(text_style),
        body,
    );

    frame.render_widget(
        Paragraph::new(if ui_state.submenu_hovered == Some(0) {
            "● Selected ●"
        } else {
            "Press ← / →"
        })
        .alignment(Alignment::Center)
        .style(if ui_state.submenu_hovered == Some(0) {
            Style::default()
                .fg(colors.selected)
                .add_modifier(Modifier::BOLD)
        } else {
            Style::default().fg(colors.text)
        }),
        footer,
    );

    // host
    let host_border = if ui_state.submenu_hovered == Some(1) {
        Style::default()
            .fg(colors.selected)
            .add_modifier(Modifier::BOLD)
    } else {
        Style::default().fg(colors.accent)
    };

    frame.render_widget(
        Block::bordered()
            .title(" Host ")
            .title_style(if ui_state.submenu_hovered == Some(1) {
                selected_title_style
            } else {
                normal_title_style
            })
            .border_type(BorderType::Double)
            .border_style(host_border),
        host,
    );

    let inner = host.inner(Margin {
        horizontal: 2,
        vertical: 1,
    });

    let [_, body, _, footer] = Layout::vertical([
        Constraint::Percentage(18),
        Constraint::Length(8),
        Constraint::Min(0),
        Constraint::Length(1),
    ])
    .areas(inner);

    frame.render_widget(
        Paragraph::new(format!(
            "Host a session\n\n\nUsername: {}\n\n\nNote: This device will host the session.",
            ui_state.username
        ))
        .alignment(Alignment::Center)
        .style(text_style),
        body,
    );

    frame.render_widget(
        Paragraph::new(if ui_state.submenu_hovered == Some(1) {
            "● Selected ●"
        } else {
            "Press ← / →"
        })
        .alignment(Alignment::Center)
        .style(if ui_state.submenu_hovered == Some(1) {
            Style::default()
                .fg(colors.selected)
                .add_modifier(Modifier::BOLD)
        } else {
            Style::default().fg(colors.text)
        }),
        footer,
    );
}

fn draw_banner(frame: &mut Frame, area: Rect, ui_state: &UiState) {
    let banner = include_str!("../../assets/modes_banner.txt");
    let colors = ui_state.theme.colors();

    let banner_width = banner
        .lines()
        .map(|line| line.chars().count())
        .max()
        .unwrap_or(0) as u16;

    let inner = area.inner(Margin {
        horizontal: 1,
        vertical: 1,
    });

    let [banner_area] = Layout::horizontal([Constraint::Length(banner_width)])
        .flex(Flex::Center)
        .areas(inner);

    frame.render_widget(
        Paragraph::new(banner)
            .style(Style::default().fg(colors.banner))
            .alignment(Alignment::Center),
        banner_area,
    );
}
