use ratatui::{
    Frame,
    layout::{Alignment, Constraint, Flex, Layout, Margin, Rect},
    style::{Modifier, Style},
    widgets::{Block, BorderType, Paragraph},
};

use crate::states::ui_state::UiState;

pub const FILE_TRANSFER_OPTIONS_MAX_INDEX: usize = 2;

pub fn draw_file_transfer_menu(frame: &mut Frame, area: Rect, ui_state: &UiState) {
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

    let [send_row, receive_row, back_row] = Layout::vertical([
        Constraint::Length(3),
        Constraint::Length(3),
        Constraint::Length(3),
    ])
    .spacing(1)
    .areas(options);

    let [send] = Layout::horizontal([Constraint::Length(22)])
        .flex(Flex::Center)
        .areas(send_row);

    let [receive] = Layout::horizontal([Constraint::Length(22)])
        .flex(Flex::Center)
        .areas(receive_row);

    let [back] = Layout::horizontal([Constraint::Length(22)])
        .flex(Flex::Center)
        .areas(back_row);

    let text_style = Style::default()
        .fg(colors.text)
        .add_modifier(Modifier::BOLD);

    let border_style = Style::default().fg(colors.accent);

    // Send a file
    let block = Block::bordered()
        .border_type(BorderType::Double)
        .border_style(border_style);

    frame.render_widget(block.clone(), send);

    let inner = block.inner(send);

    frame.render_widget(
        Paragraph::new("Send a file")
            .style(text_style)
            .alignment(Alignment::Center),
        inner,
    );

    if ui_state.submenu_hovered == Some(0) {
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

    frame.render_widget(block.clone(), receive);

    let inner = block.inner(receive);

    frame.render_widget(
        Paragraph::new("Receive file")
            .style(text_style)
            .alignment(Alignment::Center),
        inner,
    );

    if ui_state.submenu_hovered == Some(1) {
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

    frame.render_widget(block.clone(), back);

    let inner = block.inner(back);

    frame.render_widget(
        Paragraph::new("Back")
            .style(text_style)
            .alignment(Alignment::Center),
        inner,
    );

    if ui_state.submenu_hovered == Some(2) {
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
    let banner = include_str!("../../assets/file_transfer_banner.txt");
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
