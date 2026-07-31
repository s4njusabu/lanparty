use ratatui::{
    crossterm::event::{self, Event, KeyCode},
    style::Style,
    widgets::Block,
};

use crate::{app::state::State, ui::border};

mod app;
mod services;
mod ui;

fn main() -> std::io::Result<()> {
    let mut terminal = ratatui::init();
    let mut state = State::new();

    loop {
        terminal.draw(|frame| {
            frame.render_widget(
                Block::new().style(Style::default().bg(state.theme.colors().background)),
                frame.area(),
            );

            let inner = border::draw_border(frame, &state);

            if state.in_home {
                ui::home::draw_home(frame, inner, &state);
            } else {
            }
        })?;

        if let Event::Key(key_event) = event::read()? {
            match key_event.code {
                KeyCode::Esc => break,
                _ => {}
            }
        }
    }

    ratatui::restore();
    println!("Bye from LAN Party!");
    Ok(())
}
