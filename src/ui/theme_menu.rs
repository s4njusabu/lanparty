use ratatui::{
    Frame,
    layout::{Alignment, Constraint, Flex, Layout, Margin, Rect},
    style::{Modifier, Style},
    widgets::{Block, BorderType, Paragraph},
};

use crate::app::state::State;

pub const THEME_OPTIONS_MAX_INDEX: usize = 1;

pub fn draw_theme_menu(frame: &mut Frame, area: Rect, state: &State) {
    let colors = state.theme.colors();

    let [title, _, options, _] = Layout::vertical([
        Constraint::Percentage(25),
        Constraint::Percentage(10),
        Constraint::Percentage(50),
        Constraint::Percentage(15),
    ])
    .areas(area);

    draw_banner(frame, title, state);

    let [light_row, dark_row] = Layout::vertical([Constraint::Length(3), Constraint::Length(3)])
        .spacing(1)
        .areas(options);

    let [light] = Layout::horizontal([Constraint::Length(22)])
        .flex(Flex::Center)
        .areas(light_row);

    let [dark] = Layout::horizontal([Constraint::Length(22)])
        .flex(Flex::Center)
        .areas(dark_row);

    let text_style = Style::default()
        .fg(colors.text)
        .add_modifier(Modifier::BOLD);

    let border_style = Style::default().fg(colors.accent);

    frame.render_widget(
        Paragraph::new(if state.submenu_hovered == Some(0) {
            "» Light «"
        } else {
            "Light"
        })
        .style(text_style)
        .alignment(Alignment::Center)
        .block(
            Block::bordered()
                .border_type(BorderType::Double)
                .border_style(border_style),
        ),
        light,
    );

    frame.render_widget(
        Paragraph::new(if state.submenu_hovered == Some(1) {
            "» Dark «"
        } else {
            "Dark"
        })
        .style(text_style)
        .alignment(Alignment::Center)
        .block(
            Block::bordered()
                .border_type(BorderType::Double)
                .border_style(border_style),
        ),
        dark,
    );
}

fn draw_banner(frame: &mut Frame, area: Rect, state: &State) {
    let banner = include_str!("../../assets/themes_banner.txt");
    let colors = state.theme.colors();

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
