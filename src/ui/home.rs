use ratatui::{
    Frame,
    layout::{Alignment, Constraint, Flex, Layout, Margin, Rect},
    style::{Modifier, Style},
    widgets::{Block, BorderType, Paragraph},
};

use crate::states::ui_state::UiState;

pub const HOME_OPTIONS_MAX_INDEX: usize = 5;

pub fn draw_home(frame: &mut Frame, area: Rect, ui_state: &UiState) {
    let [banner_area, content_area] =
        Layout::vertical([Constraint::Length(12), Constraint::Min(0)]).areas(area);

    draw_banner(frame, banner_area, ui_state);
    draw_content(frame, content_area, ui_state);
}

fn draw_banner(frame: &mut Frame, area: Rect, ui_state: &UiState) {
    let banner = include_str!("../../assets/banner.txt");
    let colors = ui_state.theme.colors();
    let banner_width = banner
        .lines()
        .map(|line| line.chars().count())
        .max()
        .unwrap_or(0) as u16;

    let banner_area = area.inner(Margin {
        horizontal: 1,
        vertical: 0,
    });

    let banner_block = Block::bordered()
        .border_type(BorderType::Double)
        .border_style(Style::default().fg(colors.accent));

    frame.render_widget(banner_block.clone(), banner_area);

    let inner_banner = banner_block.inner(banner_area).inner(Margin {
        horizontal: 0,
        vertical: 1,
    });
    let [banner_text_area] = Layout::horizontal([Constraint::Length(banner_width)])
        .flex(Flex::Center)
        .areas(inner_banner);

    frame.render_widget(
        Paragraph::new(banner).style(Style::default().fg(colors.banner)),
        banner_text_area,
    );
}

fn draw_content(frame: &mut Frame, content_area: Rect, ui_state: &UiState) {
    let colors = ui_state.theme.colors();

    let content_area = content_area.inner(Margin {
        horizontal: 1,
        vertical: 0,
    });

    let block = Block::bordered()
        .border_type(BorderType::Double)
        .border_style(Style::default().fg(colors.accent));

    frame.render_widget(block.clone(), content_area);

    let area = block.inner(content_area);

    // horizontal (dividing the area so i can get like a perfecly centered buttons)
    let [_, center, _] = Layout::horizontal([
        Constraint::Percentage(40),
        Constraint::Percentage(20),
        Constraint::Percentage(40),
    ])
    .areas(area);

    // vertical
    let [_, menu, _] = Layout::vertical([
        Constraint::Percentage(10),
        Constraint::Percentage(80),
        Constraint::Percentage(10),
    ])
    .areas(center);

    // horizontal
    let [private, group, file, profile, themes, exit] = Layout::vertical([
        Constraint::Length(3),
        Constraint::Length(3),
        Constraint::Length(3),
        Constraint::Length(3),
        Constraint::Length(3),
        Constraint::Length(3),
    ])
    .areas(menu);

    let text_style = Style::default()
        .fg(colors.text)
        .add_modifier(Modifier::BOLD);

    let border_style = Style::default().fg(colors.accent);

    let button_block = Block::bordered()
        .border_type(BorderType::Double)
        .border_style(border_style);

    // Private chat
    frame.render_widget(
        Paragraph::new("Private chat")
            .style(text_style)
            .alignment(Alignment::Center)
            .block(button_block.clone()),
        private,
    );

    if ui_state.home_hovered == Some(0) {
        let inner = button_block.inner(private);

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

    // Group chat
    frame.render_widget(
        Paragraph::new("Group chat")
            .style(text_style)
            .alignment(Alignment::Center)
            .block(button_block.clone()),
        group,
    );

    if ui_state.home_hovered == Some(1) {
        let inner = button_block.inner(group);

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

    // File transfer
    frame.render_widget(
        Paragraph::new("File transfer")
            .style(text_style)
            .alignment(Alignment::Center)
            .block(button_block.clone()),
        file,
    );

    if ui_state.home_hovered == Some(2) {
        let inner = button_block.inner(file);

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

    // Profile
    frame.render_widget(
        Paragraph::new("Profile")
            .style(text_style)
            .alignment(Alignment::Center)
            .block(button_block.clone()),
        profile,
    );

    if ui_state.home_hovered == Some(3) {
        let inner = button_block.inner(profile);

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

    // Themes
    frame.render_widget(
        Paragraph::new("Themes")
            .style(text_style)
            .alignment(Alignment::Center)
            .block(button_block.clone()),
        themes,
    );

    if ui_state.home_hovered == Some(4) {
        let inner = button_block.inner(themes);

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

    // Exit
    frame.render_widget(
        Paragraph::new("Exit")
            .style(text_style)
            .alignment(Alignment::Center)
            .block(button_block.clone()),
        exit,
    );

    if ui_state.home_hovered == Some(5) {
        let inner = button_block.inner(exit);

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
