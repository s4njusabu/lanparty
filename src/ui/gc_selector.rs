use ratatui::{
    Frame,
    layout::{Alignment, Constraint, Flex, Layout, Margin, Rect},
    style::{Modifier, Style},
    text::Line,
    widgets::{Block, BorderType, Paragraph},
};

use crate::states::{group_chat_state::GroupChatClientState, ui_state::UiState};

pub fn draw_group_chat_selector(
    frame: &mut Frame,
    inner: Rect,
    ui_state: &UiState,
    gc_client_state: &GroupChatClientState,
) {
    let colors = ui_state.theme.colors();

    let [_, title, _, hosts, _, input, description, _] = Layout::vertical([
        Constraint::Percentage(10),
        Constraint::Percentage(20),
        Constraint::Percentage(5),
        Constraint::Length(14),
        Constraint::Percentage(5),
        Constraint::Length(3),
        Constraint::Percentage(10),
        Constraint::Percentage(10),
    ])
    .areas(inner);

    draw_banner(frame, title, ui_state);

    let text_style = Style::default()
        .fg(colors.text)
        .add_modifier(Modifier::BOLD);

    let description_style = Style::default().fg(colors.text);

    let border_style = Style::default().fg(colors.accent);

    // Found hosts
    let [hosts] = Layout::horizontal([Constraint::Length(30)])
        .flex(Flex::Center)
        .areas(hosts);

    let hosts_block = Block::bordered()
        .title(Line::from(" Found Hosts ").style(text_style))
        .title_alignment(Alignment::Center)
        .border_type(BorderType::Thick)
        .border_style(border_style);

    let hosts_inner = hosts_block.inner(hosts);

    frame.render_widget(hosts_block, hosts);

    let mut host_lines = Vec::with_capacity(10);

    for (ip, username) in gc_client_state.discovered_hosts.iter().take(10) {
        host_lines.push(Line::from(format!("{username:<8} {ip}")));
    }

    while host_lines.len() < 10 {
        host_lines.push(Line::from("-"));
    }

    frame.render_widget(
        Paragraph::new(host_lines)
            .style(description_style)
            .alignment(Alignment::Center),
        hosts_inner.inner(Margin {
            horizontal: 0,
            vertical: 1,
        }),
    );

    // Connect
    let [input] = Layout::horizontal([Constraint::Length(30)])
        .flex(Flex::Center)
        .areas(input);

    let input_block = Block::bordered()
        .border_type(BorderType::Double)
        .border_style(border_style);

    let input_inner = input_block.inner(input);

    frame.render_widget(input_block, input);

    let connect_ip = if ui_state.input_mode.is_some() {
        ui_state.input.clone()
    } else {
        gc_client_state
            .discovered_hosts
            .keys()
            .next()
            .map(|ip| ip.to_string())
            .unwrap_or_else(|| "-".to_string())
    };

    frame.render_widget(
        Paragraph::new(connect_ip)
            .style(description_style)
            .alignment(Alignment::Center),
        input_inner,
    );

    frame.render_widget(
        Paragraph::new("Enter to connect    Esc to cancel")
            .style(description_style)
            .alignment(Alignment::Center),
        description,
    );
}

fn draw_banner(frame: &mut Frame, area: Rect, ui_state: &UiState) {
    let banner = include_str!("../../assets/host_selector_banner.txt");
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
