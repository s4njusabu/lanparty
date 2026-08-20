use ratatui::{
    Frame,
    layout::{Alignment, Constraint, Flex, Layout, Margin, Rect},
    style::{Modifier, Style},
    widgets::{Block, BorderType, Paragraph},
};

use crate::states::ui_state::UiState;

pub const GROUP_CHAT_MODES_MAX_INDEX: usize = 2;

pub fn draw_group_chat_modes_menu(frame: &mut Frame, area: Rect, ui_state: &UiState) {
    let colors = ui_state.theme.colors();

    let [_, title, _, options, _] = Layout::vertical([
        Constraint::Percentage(10),
        Constraint::Percentage(20),
        Constraint::Percentage(5),
        Constraint::Percentage(50),
        Constraint::Percentage(10),
    ])
    .areas(area);

    draw_banner(frame, title, ui_state);

    let [client_area, host_area, back_area] = Layout::vertical([
        Constraint::Length(5),
        Constraint::Length(5),
        Constraint::Length(5),
    ])
    .spacing(1)
    .areas(options);

    let [client, client_description] =
        Layout::vertical([Constraint::Length(3), Constraint::Length(2)]).areas(client_area);

    let [host, host_description] =
        Layout::vertical([Constraint::Length(3), Constraint::Length(2)]).areas(host_area);

    let [back, back_description] =
        Layout::vertical([Constraint::Length(3), Constraint::Length(2)]).areas(back_area);

    let [client] = Layout::horizontal([Constraint::Length(22)])
        .flex(Flex::Center)
        .areas(client);

    let [host] = Layout::horizontal([Constraint::Length(22)])
        .flex(Flex::Center)
        .areas(host);

    let [back] = Layout::horizontal([Constraint::Length(22)])
        .flex(Flex::Center)
        .areas(back);

    let text_style = Style::default()
        .fg(colors.text)
        .add_modifier(Modifier::BOLD);

    let description_style = Style::default().fg(colors.text);

    let border_style = Style::default().fg(colors.accent);

    // Client
    let client_block = Block::bordered()
        .border_type(BorderType::Double)
        .border_style(if ui_state.submenu_hovered == Some(0) {
            Style::default()
                .fg(colors.selected)
                .add_modifier(Modifier::BOLD)
        } else {
            border_style
        });

    frame.render_widget(client_block.clone(), client);

    let client_inner = client_block.inner(client);

    frame.render_widget(
        Paragraph::new("Client")
            .style(text_style)
            .alignment(Alignment::Center),
        client_inner,
    );

    if ui_state.submenu_hovered == Some(0) {
        frame.render_widget(
            Paragraph::new("»")
                .style(text_style)
                .alignment(Alignment::Left),
            client_inner,
        );

        frame.render_widget(
            Paragraph::new("«")
                .style(text_style)
                .alignment(Alignment::Right),
            client_inner,
        );
    }

    frame.render_widget(
        Paragraph::new("Join an existing group chat")
            .style(description_style)
            .alignment(Alignment::Center),
        client_description,
    );

    // Host
    let host_block = Block::bordered()
        .border_type(BorderType::Double)
        .border_style(if ui_state.submenu_hovered == Some(1) {
            Style::default()
                .fg(colors.selected)
                .add_modifier(Modifier::BOLD)
        } else {
            border_style
        });

    frame.render_widget(host_block.clone(), host);

    let host_inner = host_block.inner(host);

    frame.render_widget(
        Paragraph::new("Host")
            .style(text_style)
            .alignment(Alignment::Center),
        host_inner,
    );

    if ui_state.submenu_hovered == Some(1) {
        frame.render_widget(
            Paragraph::new("»")
                .style(text_style)
                .alignment(Alignment::Left),
            host_inner,
        );

        frame.render_widget(
            Paragraph::new("«")
                .style(text_style)
                .alignment(Alignment::Right),
            host_inner,
        );
    }

    frame.render_widget(
        Paragraph::new("Create a new group chat for others to join")
            .style(description_style)
            .alignment(Alignment::Center),
        host_description,
    );

    // Back
    let back_block = Block::bordered()
        .border_type(BorderType::Double)
        .border_style(if ui_state.submenu_hovered == Some(2) {
            Style::default()
                .fg(colors.selected)
                .add_modifier(Modifier::BOLD)
        } else {
            border_style
        });

    frame.render_widget(back_block.clone(), back);

    let back_inner = back_block.inner(back);

    frame.render_widget(
        Paragraph::new("Back")
            .style(text_style)
            .alignment(Alignment::Center),
        back_inner,
    );

    if ui_state.submenu_hovered == Some(2) {
        frame.render_widget(
            Paragraph::new("»")
                .style(text_style)
                .alignment(Alignment::Left),
            back_inner,
        );

        frame.render_widget(
            Paragraph::new("«")
                .style(text_style)
                .alignment(Alignment::Right),
            back_inner,
        );
    }

    frame.render_widget(
        Paragraph::new("Return to the previous menu")
            .style(description_style)
            .alignment(Alignment::Center),
        back_description,
    );
}

fn draw_banner(frame: &mut Frame, area: Rect, ui_state: &UiState) {
    let banner = include_str!("../../assets/group_chat_banner.txt");
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
