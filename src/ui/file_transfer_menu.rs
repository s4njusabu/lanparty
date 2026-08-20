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

    let [send_area, receive_area, back_area] = Layout::vertical([
        Constraint::Length(5),
        Constraint::Length(5),
        Constraint::Length(5),
    ])
    .spacing(1)
    .areas(options);

    let [send, send_description] =
        Layout::vertical([Constraint::Length(3), Constraint::Length(2)]).areas(send_area);

    let [receive, receive_description] =
        Layout::vertical([Constraint::Length(3), Constraint::Length(2)]).areas(receive_area);

    let [back, back_description] =
        Layout::vertical([Constraint::Length(3), Constraint::Length(2)]).areas(back_area);

    let [send] = Layout::horizontal([Constraint::Length(22)])
        .flex(Flex::Center)
        .areas(send);

    let [receive] = Layout::horizontal([Constraint::Length(22)])
        .flex(Flex::Center)
        .areas(receive);

    let [back] = Layout::horizontal([Constraint::Length(22)])
        .flex(Flex::Center)
        .areas(back);

    let text_style = Style::default()
        .fg(colors.text)
        .add_modifier(Modifier::BOLD);

    let description_style = Style::default().fg(colors.text);

    let border_style = Style::default().fg(colors.accent);

    // Send
    let send_block = Block::bordered()
        .border_type(BorderType::Double)
        .border_style(if ui_state.submenu_hovered == Some(0) {
            Style::default()
                .fg(colors.selected)
                .add_modifier(Modifier::BOLD)
        } else {
            border_style
        });

    frame.render_widget(send_block.clone(), send);

    let send_inner = send_block.inner(send);

    frame.render_widget(
        Paragraph::new("Send a file")
            .style(text_style)
            .alignment(Alignment::Center),
        send_inner,
    );

    if ui_state.submenu_hovered == Some(0) {
        frame.render_widget(
            Paragraph::new("»")
                .style(text_style)
                .alignment(Alignment::Left),
            send_inner,
        );

        frame.render_widget(
            Paragraph::new("«")
                .style(text_style)
                .alignment(Alignment::Right),
            send_inner,
        );
    }

    frame.render_widget(
        Paragraph::new("Send a file to another device")
            .style(description_style)
            .alignment(Alignment::Center),
        send_description,
    );

    // Receive
    let receive_block = Block::bordered()
        .border_type(BorderType::Double)
        .border_style(if ui_state.submenu_hovered == Some(1) {
            Style::default()
                .fg(colors.selected)
                .add_modifier(Modifier::BOLD)
        } else {
            border_style
        });

    frame.render_widget(receive_block.clone(), receive);

    let receive_inner = receive_block.inner(receive);

    frame.render_widget(
        Paragraph::new("Receive file")
            .style(text_style)
            .alignment(Alignment::Center),
        receive_inner,
    );

    if ui_state.submenu_hovered == Some(1) {
        frame.render_widget(
            Paragraph::new("»")
                .style(text_style)
                .alignment(Alignment::Left),
            receive_inner,
        );

        frame.render_widget(
            Paragraph::new("«")
                .style(text_style)
                .alignment(Alignment::Right),
            receive_inner,
        );
    }

    frame.render_widget(
        Paragraph::new("Receive a file from another device")
            .style(description_style)
            .alignment(Alignment::Center),
        receive_description,
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
