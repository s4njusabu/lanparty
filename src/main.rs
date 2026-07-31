use ratatui::{
    crossterm::event::{self, Event, KeyCode},
    style::Style,
    widgets::Block,
};

use crate::{
    app::state::State,
    ui::{border, home},
};

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

        if state.in_home
            && let Event::Key(key_event) = event::read()?
        {
            match key_event.code {
                KeyCode::Enter => {
                    state.home_selected = state.home_hovered;
                }
                KeyCode::Down => {
                    if let Some(n) = state.home_hovered
                        && n < home::HOME_OPTIONS_MAX_INDEX
                    {
                        state.home_hovered = Some(n + 1);
                    }
                }
                KeyCode::Up => {
                    if let Some(n) = state.home_hovered
                        && n > 0
                    {
                        state.home_hovered = Some(n - 1);
                    }
                }
                KeyCode::Char('q') | KeyCode::Esc => break,
                _ => {}
            }

            if let Some(n) = state.home_selected.take() {
                match n {
                    0 | 1 | 2 => {
                        state.in_home = false;
                        state.home_hovered = Some(n);
                    }
                    3 => break,
                    _ => {}
                }
            }
        } else if state.in_submenu
            && let Event::Key(key_event) = event::read()?
        {
            match key_event.code {
                _ => {}
            }
        } else if state.in_chat
            && let Event::Key(key_event) = event::read()?
        {
            match key_event.code {
                _ => {}
            }
        }
    }

    ratatui::restore();
    println!("Bye from LAN Party!");
    Ok(())
}
