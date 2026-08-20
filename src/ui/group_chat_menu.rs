use ratatui::{
    Frame,
    layout::{Alignment, Constraint, Flex, Layout, Margin, Rect},
    style::{Modifier, Style},
    widgets::{Block, BorderType, Paragraph},
};

use crate::states::ui_state::UiState;

pub const GROUP_CHAT_MODES_MAX_INDEX: usize = 1;

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

    let [client_row, host_row] = Layout::vertical([Constraint::Length(3), Constraint::Length(3)])
        .spacing(1)
        .areas(options);

    let [client] = Layout::horizontal([Constraint::Length(22)])
        .flex(Flex::Center)
        .areas(client_row);

    let [host] = Layout::horizontal([Constraint::Length(22)])
        .flex(Flex::Center)
        .areas(host_row);

    let text_style = Style::default()
        .fg(colors.text)
        .add_modifier(Modifier::BOLD);

    let border_style = Style::default().fg(colors.accent);

    let block = Block::bordered()
        .border_type(BorderType::Double)
        .border_style(border_style);

    frame.render_widget(
        Paragraph::new("Client")
            .style(text_style)
            .alignment(Alignment::Center)
            .block(block.clone()),
        client,
    );

    if ui_state.submenu_hovered == Some(0) {
        let inner = block.inner(client);

        frame.render_widget(
            Paragraph::new("»")
                .style(text_style)
                .alignment(Alignment::Left),
            inner,
        );

        frame.render_widget(
            Paragraph::new("«")
                .style(text_style)
                .alignment(Alignment::Right),
            inner,
        );
    }

    let block = Block::bordered()
        .border_type(BorderType::Double)
        .border_style(border_style);

    frame.render_widget(
        Paragraph::new("Host")
            .style(text_style)
            .alignment(Alignment::Center)
            .block(block.clone()),
        host,
    );

    if ui_state.submenu_hovered == Some(1) {
        let inner = block.inner(host);

        frame.render_widget(
            Paragraph::new("»")
                .style(text_style)
                .alignment(Alignment::Left),
            inner,
        );

        frame.render_widget(
            Paragraph::new("«")
                .style(text_style)
                .alignment(Alignment::Right),
            inner,
        );
    }
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
