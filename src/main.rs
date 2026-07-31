use ratatui::{
    crossterm::event::{self, Event, KeyCode},
    style::Style,
    widgets::Block,
};

use crate::{
    app::state::{HomeItems, State},
    ui::{border, home, theme_menu, themes::Theme},
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
            } else if state.in_submenu {
                if let Some(n) = state.home_hovered {
                    match n {
                        1 => theme_menu::draw_theme_menu(frame, inner, &state),
                        _ => {}
                    }
                }
            } else if state.in_chat {
            }
        })?;

        // In home menu
        if state.in_home
            && let Event::Key(key_event) = event::read()?
        {
            match key_event.code {
                KeyCode::Enter | KeyCode::Right => {
                    state.home_selected = state.home_hovered;
                }
                KeyCode::Up => {
                    if let Some(n) = state.home_hovered
                        && n > 0
                    {
                        state.home_hovered = Some(n - 1);
                    }
                }
                KeyCode::Down => {
                    if let Some(n) = state.home_hovered
                        && n < home::HOME_OPTIONS_MAX_INDEX
                    {
                        state.home_hovered = Some(n + 1);
                    }
                }

                KeyCode::Char('q') | KeyCode::Esc => break,
                _ => {}
            }

            if let Some(n) = state.home_selected.take() {
                match n {
                    0 => {
                        state.in_home = false;
                        state.in_submenu = true;
                        state.home_state = HomeItems::Modes;
                    }
                    1 => {
                        state.in_home = false;
                        state.in_submenu = true;
                        state.home_state = HomeItems::Themes;
                    }
                    2 => {
                        state.in_home = false;
                        state.in_submenu = true;
                        state.home_state = HomeItems::Project;
                    }
                    3 => break,
                    _ => {}
                }
            }
        } else if state.in_submenu
        // In submenu
            && let Event::Key(key_event) = event::read()?
        {
            match key_event.code {
                KeyCode::Enter | KeyCode::Right => {
                    state.submenu_selected = state.submenu_hovered;
                }
                KeyCode::Up => match state.home_state {
                    HomeItems::Modes => {}
                    HomeItems::Themes => {
                        if let Some(n) = state.submenu_hovered
                            && n > 0
                        {
                            state.submenu_hovered = Some(n - 1);
                        }
                    }
                    HomeItems::Project => {}
                    HomeItems::Exit => {}
                },
                KeyCode::Down => match state.home_state {
                    HomeItems::Modes => {}
                    HomeItems::Themes => {
                        if let Some(n) = state.submenu_hovered
                            && n < theme_menu::THEME_OPTIONS_MAX_INDEX
                        {
                            state.submenu_hovered = Some(n + 1);
                        }
                    }
                    HomeItems::Project => {}
                    HomeItems::Exit => {}
                },
                KeyCode::Char('q') | KeyCode::Left | KeyCode::Esc => {
                    state.in_home = true;
                    state.in_submenu = false;
                    state.submenu_hovered = Some(0);
                    state.submenu_selected = None;
                }
                _ => {}
            }

            match state.home_state {
                HomeItems::Modes => {}
                HomeItems::Themes => {
                    if let Some(n) = state.submenu_selected {
                        match n {
                            0 => state.theme = Theme::Dark,
                            1 => state.theme = Theme::Light,
                            _ => {}
                        }
                    }
                }
                HomeItems::Project => {}
                HomeItems::Exit => {}
            }
        } else if state.in_chat
        // In chat
            && let Event::Key(key_event) = event::read()?
        {
            match key_event.code {
                KeyCode::Char('q') | KeyCode::Esc => {
                    state.in_home = true;
                    state.in_chat = false;
                }
                _ => {}
            }
        }
    }

    ratatui::restore();
    println!("Bye from LAN Party!");
    Ok(())
}
