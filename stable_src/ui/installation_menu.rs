use ratatui::{
    Frame,
    layout::{Alignment, Constraint, Flex, Layout, Margin, Rect},
    style::{Modifier, Style},
    widgets::Paragraph,
};

use crate::app::state::State;

pub fn draw_installation_menu(frame: &mut Frame, area: Rect, state: &State) {
    let colors = state.theme.colors();

    let [_, title, _, content_area, _] = Layout::vertical([
        Constraint::Percentage(10),
        Constraint::Percentage(20),
        Constraint::Percentage(10),
        Constraint::Percentage(50),
        Constraint::Percentage(10),
    ])
    .areas(area);

    draw_banner(frame, title, state);

    let text_style = Style::default()
        .fg(colors.text)
        .add_modifier(Modifier::BOLD);

    let content = format!(
        "{:<10}{}\n\n{:<10}{}",
        "GITHUB", "https://github.com/s4njusabu/lanparty", "CARGO", "cargo install lanparty",
    );

    frame.render_widget(
        Paragraph::new(content)
            .style(text_style)
            .alignment(Alignment::Center),
        content_area,
    );
}

fn draw_banner(frame: &mut Frame, area: Rect, state: &State) {
    let banner = include_str!("../../assets/installation_banner.txt");
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
