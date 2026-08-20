use ratatui::{
    Frame,
    layout::{Alignment, Constraint, Flex, Layout, Margin, Rect},
    style::{Modifier, Style},
    widgets::{Block, BorderType, Paragraph},
};

use crate::states::ui_state::UiState;

pub const THEME_OPTIONS_MAX_INDEX: usize = 2;

pub fn draw_themes_menu(frame: &mut Frame, inner: Rect, ui_state: &UiState) {
    let colors = ui_state.theme.colors();

    let [_, title, _, options, _] = Layout::vertical([
        Constraint::Percentage(10),
        Constraint::Percentage(20),
        Constraint::Percentage(5),
        Constraint::Percentage(50),
        Constraint::Percentage(10),
    ])
    .areas(inner);

    draw_banner(frame, title, ui_state);

    let [dark_area, light_area, back_area] = Layout::vertical([
        Constraint::Length(5),
        Constraint::Length(5),
        Constraint::Length(5),
    ])
    .spacing(1)
    .areas(options);

    let [dark, dark_description] =
        Layout::vertical([Constraint::Length(3), Constraint::Length(2)]).areas(dark_area);

    let [light, light_description] =
        Layout::vertical([Constraint::Length(3), Constraint::Length(2)]).areas(light_area);

    let [back, back_description] =
        Layout::vertical([Constraint::Length(3), Constraint::Length(2)]).areas(back_area);

    let [dark] = Layout::horizontal([Constraint::Length(22)])
        .flex(Flex::Center)
        .areas(dark);

    let [light] = Layout::horizontal([Constraint::Length(22)])
        .flex(Flex::Center)
        .areas(light);

    let [back] = Layout::horizontal([Constraint::Length(22)])
        .flex(Flex::Center)
        .areas(back);

    let text_style = Style::default()
        .fg(colors.text)
        .add_modifier(Modifier::BOLD);

    let description_style = Style::default().fg(colors.text);

    let border_style = Style::default().fg(colors.accent);

    // Dark
    let dark_block = Block::bordered()
        .border_type(BorderType::Double)
        .border_style(if ui_state.submenu_hovered == Some(0) {
            Style::default()
                .fg(colors.selected)
                .add_modifier(Modifier::BOLD)
        } else {
            border_style
        });

    frame.render_widget(dark_block.clone(), dark);

    let dark_inner = dark_block.inner(dark);

    frame.render_widget(
        Paragraph::new("Dark")
            .style(text_style)
            .alignment(Alignment::Center),
        dark_inner,
    );

    if ui_state.submenu_hovered == Some(0) {
        frame.render_widget(
            Paragraph::new("»")
                .style(text_style)
                .alignment(Alignment::Left),
            dark_inner,
        );

        frame.render_widget(
            Paragraph::new("«")
                .style(text_style)
                .alignment(Alignment::Right),
            dark_inner,
        );
    }

    frame.render_widget(
        Paragraph::new("Default theme")
            .style(description_style)
            .alignment(Alignment::Center),
        dark_description,
    );

    // Light
    let light_block = Block::bordered()
        .border_type(BorderType::Double)
        .border_style(if ui_state.submenu_hovered == Some(1) {
            Style::default()
                .fg(colors.selected)
                .add_modifier(Modifier::BOLD)
        } else {
            border_style
        });

    frame.render_widget(light_block.clone(), light);

    let light_inner = light_block.inner(light);

    frame.render_widget(
        Paragraph::new("Light")
            .style(text_style)
            .alignment(Alignment::Center),
        light_inner,
    );

    if ui_state.submenu_hovered == Some(1) {
        frame.render_widget(
            Paragraph::new("»")
                .style(text_style)
                .alignment(Alignment::Left),
            light_inner,
        );

        frame.render_widget(
            Paragraph::new("«")
                .style(text_style)
                .alignment(Alignment::Right),
            light_inner,
        );
    }

    frame.render_widget(
        Paragraph::new("I dont recommend this")
            .style(description_style)
            .alignment(Alignment::Center),
        light_description,
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
    let banner = include_str!("../../assets/themes_banner.txt");
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
