use std::{sync::mpsc, thread, time::Duration};

use ratatui::{
    crossterm::event::{self, Event, KeyCode, KeyModifiers},
    style::Style,
    widgets::Block,
};

use crate::{
    app::state::{ConnectedUser, HomeItems, Mode, State},
    services::{
        get_username::get_username,
        network::{NetworkEvent, create_server, send_udp_packets_to_broadcast},
    },
    ui::{
        border, home, installation_menu,
        modes::{client, error_page, host},
        modes_menu, theme_menu,
        themes::Theme,
    },
};

mod app;
mod services;
mod ui;

fn main() -> std::io::Result<()> {
    let mut terminal = ratatui::init();
    let mut state = State::new();
    let (event_tx, event_rx) = mpsc::channel::<NetworkEvent>();
    state.username = get_username();

    thread::spawn(send_udp_packets_to_broadcast);
    loop {
        if let Ok(network_event) = event_rx.try_recv() {
            match network_event {
                NetworkEvent::ClientConnected(ip) => {
                    if !state.users_connected.iter().any(|user| user.ip == ip) {
                        state.users_connected.push(ConnectedUser {
                            ip,
                            username: "anonymous".to_string(),
                        });
                    }
                }
                NetworkEvent::ClientDisconnected(ip) => {}
                NetworkEvent::ChatMessage { ip, message } => {}
                NetworkEvent::Error(err) => {
                    if state.in_chat {
                        state.mode = Some(Mode::Error(err));
                        state.error_occured = true;
                    }
                }
            }
        }

        terminal.draw(|frame| {
            frame.render_widget(
                Block::new().style(Style::default().bg(state.theme.colors().background)),
                frame.area(),
            );

            let inner = border::draw_border(frame, &state);

            if state.in_home {
                ui::home::draw_home(frame, inner, &state);
            } else if state.in_submenu
                && let Some(n) = state.home_hovered
            {
                match n {
                    0 => modes_menu::draw_modes_menu(frame, inner, &state),
                    1 => theme_menu::draw_theme_menu(frame, inner, &state),
                    2 => installation_menu::draw_installation_menu(frame, inner, &state),
                    _ => {}
                }
            } else if state.in_chat
                && let Some(mode) = &state.mode
            {
                match mode {
                    Mode::Client => client::draw_client(frame, inner, &state),
                    Mode::Host => host::draw_host(frame, inner, &state),
                    Mode::Error(err) => {
                        error_page::draw_error_page(frame, inner, &state, *err);
                    }
                }
            }
        })?;

        // In home menu
        if state.in_home
            && event::poll(Duration::from_millis(16))?
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
                KeyCode::Char('c') if key_event.modifiers == KeyModifiers::CONTROL => break,
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
            && event::poll(Duration::from_millis(16))?
            && let Event::Key(key_event) = event::read()?
        {
            // In submenu
            match key_event.code {
                KeyCode::Enter => {
                    if state.home_state == HomeItems::Modes {
                        state.in_chat = true;
                        state.submenu_selected = state.submenu_hovered;
                    } else {
                        state.submenu_selected = state.submenu_hovered;
                    }
                }
                KeyCode::Right => match state.home_state {
                    HomeItems::Modes => {
                        if let Some(n) = state.submenu_hovered
                            && n < modes_menu::MODE_OPTIONS_MAX_INDEX
                        {
                            state.submenu_hovered = Some(n + 1);
                        }
                    }
                    HomeItems::Themes | HomeItems::Project => {
                        state.submenu_selected = state.submenu_hovered;
                    }
                },
                KeyCode::Up => match state.home_state {
                    HomeItems::Modes => {
                        if let Some(n) = state.submenu_hovered
                            && n > 0
                        {
                            state.submenu_hovered = Some(n - 1);
                        }
                    }
                    HomeItems::Themes => {
                        if let Some(n) = state.submenu_hovered
                            && n > 0
                        {
                            state.submenu_hovered = Some(n - 1);
                        }
                    }
                    HomeItems::Project => {}
                },
                KeyCode::Down => match state.home_state {
                    HomeItems::Modes => {
                        if let Some(n) = state.submenu_hovered
                            && n < modes_menu::MODE_OPTIONS_MAX_INDEX
                        {
                            state.submenu_hovered = Some(n + 1);
                        }
                    }
                    HomeItems::Themes => {
                        if let Some(n) = state.submenu_hovered
                            && n < theme_menu::THEME_OPTIONS_MAX_INDEX
                        {
                            state.submenu_hovered = Some(n + 1);
                        }
                    }
                    HomeItems::Project => {}
                },
                KeyCode::Char('q') | KeyCode::Esc => {
                    state.in_home = true;
                    state.in_submenu = false;
                    state.submenu_hovered = Some(0);
                    state.submenu_selected = None;
                }
                KeyCode::Left => match state.home_state {
                    HomeItems::Modes => {
                        if let Some(n) = state.submenu_hovered
                            && n > 0
                        {
                            state.submenu_hovered = Some(n - 1);
                        }
                    }
                    HomeItems::Themes | HomeItems::Project => {
                        state.in_home = true;
                        state.in_submenu = false;
                        state.submenu_hovered = Some(0);
                        state.submenu_selected = None;
                    }
                },
                KeyCode::Char('c') if key_event.modifiers == KeyModifiers::CONTROL => break,
                _ => {}
            }

            match state.home_state {
                HomeItems::Modes => {
                    if let Some(n) = state.submenu_selected.take() {
                        match n {
                            0 => {
                                state.in_submenu = false;
                                state.in_chat = true;
                                state.mode = Some(Mode::Client);
                            }
                            1 => {
                                state.in_submenu = false;
                                state.in_chat = true;
                                state.mode = Some(Mode::Host);

                                let event_tx_clone = event_tx.clone();
                                thread::spawn(move || {
                                    if let Err(err) = create_server(event_tx_clone.clone()) {
                                        let _ =
                                            event_tx_clone.send(NetworkEvent::Error(err.kind()));
                                    }
                                });
                            }

                            _ => {}
                        }
                    }
                }
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
            }
        } else if state.in_chat
            && event::poll(Duration::from_millis(16))?
            && let Event::Key(key_event) = event::read()?
        {
            // In chat
            if state.error_occured {
                match key_event.code {
                    _ => break,
                }
            } else {
                match key_event.code {
                    KeyCode::Char('q') | KeyCode::Esc => break,
                    KeyCode::Char('c') if key_event.modifiers == KeyModifiers::CONTROL => {
                        break;
                    }
                    _ => {}
                }
            }
        }
    }

    ratatui::restore();
    println!("Bye from LAN Party!");
    Ok(())
}
