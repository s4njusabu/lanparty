use ratatui::{
    Frame,
    layout::{Alignment, Constraint, Flex, Layout, Margin, Rect},
    style::{Modifier, Style},
    widgets::{Block, BorderType, Paragraph},
};

use crate::app::state::State;

pub const HOME_OPTIONS_MAX_INDEX: usize = 3;

pub fn draw_home(frame: &mut Frame, area: Rect, state: &State) {
    let [banner_area, content_area] =
        Layout::vertical([Constraint::Length(12), Constraint::Min(0)]).areas(area);

    draw_banner(frame, banner_area, state);
    draw_content(frame, content_area, state);
}

fn draw_banner(frame: &mut Frame, area: Rect, state: &State) {
    let banner = include_str!("../../assets/banner.txt");
    let colors = state.theme.colors();
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
fn draw_content(frame: &mut Frame, content_area: Rect, state: &State) {
    let colors = state.theme.colors();

    let content_area = content_area.inner(Margin {
        horizontal: 1,
        vertical: 0,
    });

    let block = Block::bordered()
        .border_type(BorderType::Double)
        .border_style(Style::default().fg(colors.accent));

    frame.render_widget(block.clone(), content_area);

    let area = block.inner(content_area);

    // horizontal
    let [_, left, _] = Layout::horizontal([
        Constraint::Percentage(40),
        Constraint::Percentage(20),
        Constraint::Percentage(40),
    ])
    .areas(area);

    // vertical
    let [_, menu, _] = Layout::vertical([
        Constraint::Length(2),
        Constraint::Length(20),
        Constraint::Fill(0),
    ])
    .areas(left);

    // horizontal
    let [modes, themes, install, exit] = Layout::vertical([
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

    if state.home_hovered == Some(0) {
        frame.render_widget(
            Paragraph::new("» Modes «")
                .style(text_style)
                .alignment(Alignment::Center)
                .block(
                    Block::bordered()
                        .border_type(BorderType::Double)
                        .border_style(border_style),
                ),
            modes,
        );
    } else {
        frame.render_widget(
            Paragraph::new("Modes")
                .style(text_style)
                .alignment(Alignment::Center)
                .block(
                    Block::bordered()
                        .border_type(BorderType::Double)
                        .border_style(border_style),
                ),
            modes,
        );
    }

    if state.home_hovered == Some(1) {
        frame.render_widget(
            Paragraph::new("» Themes «")
                .style(text_style)
                .alignment(Alignment::Center)
                .block(
                    Block::bordered()
                        .border_type(BorderType::Double)
                        .border_style(border_style),
                ),
            themes,
        );
    } else {
        frame.render_widget(
            Paragraph::new("Themes")
                .style(text_style)
                .alignment(Alignment::Center)
                .block(
                    Block::bordered()
                        .border_type(BorderType::Double)
                        .border_style(border_style),
                ),
            themes,
        );
    }

    if state.home_hovered == Some(2) {
        frame.render_widget(
            Paragraph::new("» Install «")
                .style(text_style)
                .alignment(Alignment::Center)
                .block(
                    Block::bordered()
                        .border_type(BorderType::Double)
                        .border_style(border_style),
                ),
            install,
        );
    } else {
        frame.render_widget(
            Paragraph::new("Install")
                .style(text_style)
                .alignment(Alignment::Center)
                .block(
                    Block::bordered()
                        .border_type(BorderType::Double)
                        .border_style(border_style),
                ),
            install,
        );
    }

    if state.home_hovered == Some(3) {
        frame.render_widget(
            Paragraph::new("» Exit «")
                .style(text_style)
                .alignment(Alignment::Center)
                .block(
                    Block::bordered()
                        .border_type(BorderType::Double)
                        .border_style(border_style),
                ),
            exit,
        );
    } else {
        frame.render_widget(
            Paragraph::new("Exit")
                .style(text_style)
                .alignment(Alignment::Center)
                .block(
                    Block::bordered()
                        .border_type(BorderType::Double)
                        .border_style(border_style),
                ),
            exit,
        );
    }
}
