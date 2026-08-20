#![allow(unused)]
use ratatui::{style::Style, widgets::Block};

use crate::{states::ui_state::UiState, ui::border};

mod services;
mod states;
pub mod themes;
mod ui;

fn main() {
    let mut terminal = ratatui::init();

    let mut ui_state = UiState::new();

    loop {
        terminal.draw(|frame| {
            frame.render_widget(
                Block::new().style(Style::default().bg(ui_state.theme.colors().background)),
                frame.area(),
            );
            let inner = border::draw_border(frame, &ui_state);

            if ui_state.in_home {
                ui::home::draw_home(frame, inner, &ui_state);
            }
        });
    }

    ratatui::restore();
    println!("Bye from LAN Party!");
}
