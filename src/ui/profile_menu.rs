use ratatui::{
    Frame,
    layout::{Alignment, Constraint, Flex, Layout, Margin, Rect},
    style::{Modifier, Style},
    widgets::{Block, BorderType, Paragraph},
};

use crate::states::ui_state::{InputMode, UiState};

pub const PROFILE_OPTIONS_MAX_INDEX: usize = 1;

pub fn draw_profile_menu(frame: &mut Frame, area: Rect, ui_state: &UiState) {
    let colors = ui_state.theme.colors();

    let [_, title, _, options, _] = Layout::vertical([
        Constraint::Percentage(10),
        Constraint::Percentage(20),
        Constraint::Percentage(3),
        Constraint::Percentage(50),
        Constraint::Percentage(10),
    ])
    .areas(area);

    draw_banner(frame, title, ui_state);

    let [info_area, change_area, back_area] = Layout::vertical([
        Constraint::Length(5),
        Constraint::Length(4),
        Constraint::Length(4),
    ])
    .spacing(1)
    .areas(options);

    let [info] = Layout::horizontal([Constraint::Length(40)])
        .flex(Flex::Center)
        .areas(info_area);

    let [change_username, change_description] =
        Layout::vertical([Constraint::Length(3), Constraint::Length(1)]).areas(change_area);

    let [back, back_description] =
        Layout::vertical([Constraint::Length(3), Constraint::Length(1)]).areas(back_area);

    let [change_username] = Layout::horizontal([Constraint::Length(22)])
        .flex(Flex::Center)
        .areas(change_username);

    let [back] = Layout::horizontal([Constraint::Length(22)])
        .flex(Flex::Center)
        .areas(back);

    let text_style = Style::default()
        .fg(colors.text)
        .add_modifier(Modifier::BOLD);

    let description_style = Style::default().fg(colors.text);

    let border_style = Style::default().fg(colors.accent);

    // Profile
    let info_block = Block::bordered()
        .border_type(BorderType::Double)
        .border_style(border_style);

    frame.render_widget(info_block.clone(), info);

    let info_inner = info_block.inner(info);

    if ui_state.input_mode == Some(InputMode::ChangeUsername) {
        frame.render_widget(
            Paragraph::new(ui_state.username.as_str())
                .style(text_style)
                .alignment(Alignment::Center),
            info_inner,
        );
    } else {
        frame.render_widget(
            Paragraph::new(format!(
                "Username: {}\n\nLocal IP: {}",
                ui_state.username, ui_state.local_ip
            ))
            .style(text_style)
            .alignment(Alignment::Center),
            info_inner,
        );
    }

    // Change username
    let change_block = Block::bordered()
        .border_type(BorderType::Double)
        .border_style(if ui_state.submenu_hovered == Some(0) {
            Style::default()
                .fg(colors.selected)
                .add_modifier(Modifier::BOLD)
        } else {
            border_style
        });

    frame.render_widget(change_block.clone(), change_username);

    let change_inner = change_block.inner(change_username);

    frame.render_widget(
        Paragraph::new("Change username")
            .style(text_style)
            .alignment(Alignment::Center),
        change_inner,
    );

    if ui_state.submenu_hovered == Some(0) {
        frame.render_widget(
            Paragraph::new("»")
                .style(text_style)
                .alignment(Alignment::Left),
            change_inner,
        );

        frame.render_widget(
            Paragraph::new("«")
                .style(text_style)
                .alignment(Alignment::Right),
            change_inner,
        );
    }

    frame.render_widget(
        Paragraph::new(if ui_state.input_mode == Some(InputMode::ChangeUsername) {
            "Enter to confirm and Esc to cancel"
        } else {
            "Requires atleast 3 characters"
        })
        .style(description_style)
        .alignment(Alignment::Center),
        change_description,
    );

    // Back
    let back_block = Block::bordered()
        .border_type(BorderType::Double)
        .border_style(if ui_state.submenu_hovered == Some(1) {
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

    if ui_state.submenu_hovered == Some(1) {
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
    let banner = include_str!("../../assets/profile_banner.txt");
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
