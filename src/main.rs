use std::{collections::HashMap, net::IpAddr, sync::mpsc, thread, time::Duration};

use ratatui::{
    crossterm::event::{self, Event, KeyCode, KeyModifiers},
    style::Style,
    widgets::Block,
};

use crate::{
    app::{
        server_state::{Message, ServerState, User},
        ui_state::{HomeItems, Mode, UiState},
    },
    services::{
        get_username::get_username,
        network::{
            GetClientConnection, accept_connections, broadcast_message, broadcast_users,
            connect_to_server, get_network_interface_and_user_ip, send_udp_packets_to_broadcast,
        },
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

    let mut ui_state = UiState::new();
    let mut server_state = ServerState::new();

    ui_state.username = get_username();

    let (accept_conn_tx, accept_conn_rx) = mpsc::channel::<GetClientConnection>();
    let (accept_user_list_tx, accept_user_list_rx) = mpsc::channel::<HashMap<IpAddr, User>>();
    let (send_message_tx, send_message_rx) = mpsc::channel::<String>();

    loop {
        if ui_state.in_chat && !ui_state.mode_activated {
            match &ui_state.mode {
                Some(Mode::Host) => {
                    thread::spawn(send_udp_packets_to_broadcast);

                    if let Some((_, ip)) = get_network_interface_and_user_ip() {
                        let host_ip: IpAddr = ip.parse().map_err(std::io::Error::other)?;

                        let username = format!("{} [host]", ui_state.username.clone());
                        server_state.users.insert(
                            host_ip,
                            User {
                                username,
                                online: true,
                            },
                        );
                    }
                }
                Some(Mode::Client) | Some(Mode::Error(_)) | None => {}
            }

            ui_state.mode_activated = true;
        }

        if let Some(mode) = &ui_state.mode {
            match mode {
                Mode::Host => {
                    if let Ok(network_event) = accept_conn_rx.try_recv() {
                        match network_event {
                            GetClientConnection::ClientConnected(ip, stream, username) => {
                                server_state.users.insert(
                                    ip,
                                    User {
                                        username,
                                        online: true,
                                    },
                                );

                                server_state.connections.insert(ip, stream);

                                broadcast_users(&mut server_state);
                            }

                            GetClientConnection::ClientDisconnected(ip) => {
                                if let Some(user) = server_state.users.get_mut(&ip) {
                                    user.online = false;
                                }

                                server_state.connections.remove(&ip);

                                broadcast_users(&mut server_state);
                            }

                            GetClientConnection::Message(ip, message) => {
                                server_state.messages.push(Message {
                                    sender: ip,
                                    message: message.clone(),
                                });

                                broadcast_message(&mut server_state, message);
                            }

                            GetClientConnection::Error(err) => {
                                if ui_state.in_chat {
                                    ui_state.mode = Some(Mode::Error(err));
                                    ui_state.error_occured = true;
                                }
                            }
                        }
                    }
                }

                Mode::Client => {
                    if let Ok(users) = accept_user_list_rx.try_recv() {
                        server_state.users = users;
                    }

                    // You'll eventually have another receiver here for chat messages,
                    // just like accept_user_list_rx.
                }

                Mode::Error(_) => {}
            }
        }
        terminal.draw(|frame| {
            frame.render_widget(
                Block::new().style(Style::default().bg(ui_state.theme.colors().background)),
                frame.area(),
            );

            let inner = border::draw_border(frame, &ui_state);

            if ui_state.in_home {
                ui::home::draw_home(frame, inner, &ui_state);
            } else if ui_state.in_submenu
                && let Some(n) = ui_state.home_hovered
            {
                match n {
                    0 => modes_menu::draw_modes_menu(frame, inner, &ui_state),
                    1 => theme_menu::draw_theme_menu(frame, inner, &ui_state),
                    2 => installation_menu::draw_installation_menu(frame, inner, &ui_state),
                    _ => {}
                }
            } else if ui_state.in_chat
                && let Some(mode) = &ui_state.mode
            {
                match mode {
                    Mode::Client => client::draw_client(frame, inner, &ui_state, &server_state),
                    Mode::Host => host::draw_host(frame, inner, &ui_state, &server_state),
                    Mode::Error(err) => error_page::draw_error_page(frame, inner, &ui_state, *err),
                }
            }
        })?;

        // In home menu
        if ui_state.in_home
            && event::poll(Duration::from_millis(16))?
            && let Event::Key(key_event) = event::read()?
        {
            match key_event.code {
                KeyCode::Enter | KeyCode::Right => {
                    ui_state.home_selected = ui_state.home_hovered;
                }
                KeyCode::Up => {
                    if let Some(n) = ui_state.home_hovered
                        && n > 0
                    {
                        ui_state.home_hovered = Some(n - 1);
                    }
                }
                KeyCode::Down => {
                    if let Some(n) = ui_state.home_hovered
                        && n < home::HOME_OPTIONS_MAX_INDEX
                    {
                        ui_state.home_hovered = Some(n + 1);
                    }
                }

                KeyCode::Char('q') | KeyCode::Esc => break,
                KeyCode::Char('c') if key_event.modifiers == KeyModifiers::CONTROL => break,
                _ => {}
            }

            if let Some(n) = ui_state.home_selected.take() {
                match n {
                    0 => {
                        ui_state.in_home = false;
                        ui_state.in_submenu = true;
                        ui_state.home_state = HomeItems::Modes;
                    }
                    1 => {
                        ui_state.in_home = false;
                        ui_state.in_submenu = true;
                        ui_state.home_state = HomeItems::Themes;
                    }
                    2 => {
                        ui_state.in_home = false;
                        ui_state.in_submenu = true;
                        ui_state.home_state = HomeItems::Project;
                    }
                    3 => break,
                    _ => {}
                }
            }
        } else if ui_state.in_submenu
            && event::poll(Duration::from_millis(16))?
            && let Event::Key(key_event) = event::read()?
        {
            // In submenu
            match key_event.code {
                KeyCode::Enter => {
                    if ui_state.home_state == HomeItems::Modes {
                        ui_state.in_chat = true;
                        ui_state.submenu_selected = ui_state.submenu_hovered;
                    } else {
                        ui_state.submenu_selected = ui_state.submenu_hovered;
                    }
                }
                KeyCode::Right => match ui_state.home_state {
                    HomeItems::Modes => {
                        if let Some(n) = ui_state.submenu_hovered
                            && n < modes_menu::MODE_OPTIONS_MAX_INDEX
                        {
                            ui_state.submenu_hovered = Some(n + 1);
                        }
                    }
                    HomeItems::Themes | HomeItems::Project => {
                        ui_state.submenu_selected = ui_state.submenu_hovered;
                    }
                },
                KeyCode::Up => match ui_state.home_state {
                    HomeItems::Modes => {
                        if let Some(n) = ui_state.submenu_hovered
                            && n > 0
                        {
                            ui_state.submenu_hovered = Some(n - 1);
                        }
                    }
                    HomeItems::Themes => {
                        if let Some(n) = ui_state.submenu_hovered
                            && n > 0
                        {
                            ui_state.submenu_hovered = Some(n - 1);
                        }
                    }
                    HomeItems::Project => {}
                },
                KeyCode::Down => match ui_state.home_state {
                    HomeItems::Modes => {
                        if let Some(n) = ui_state.submenu_hovered
                            && n < modes_menu::MODE_OPTIONS_MAX_INDEX
                        {
                            ui_state.submenu_hovered = Some(n + 1);
                        }
                    }
                    HomeItems::Themes => {
                        if let Some(n) = ui_state.submenu_hovered
                            && n < theme_menu::THEME_OPTIONS_MAX_INDEX
                        {
                            ui_state.submenu_hovered = Some(n + 1);
                        }
                    }
                    HomeItems::Project => {}
                },
                KeyCode::Char('q') | KeyCode::Esc | KeyCode::Backspace => {
                    ui_state.in_home = true;
                    ui_state.in_submenu = false;
                    ui_state.submenu_hovered = Some(0);
                    ui_state.submenu_selected = None;
                }
                KeyCode::Left => match ui_state.home_state {
                    HomeItems::Modes => {
                        if let Some(n) = ui_state.submenu_hovered
                            && n > 0
                        {
                            ui_state.submenu_hovered = Some(n - 1);
                        }
                    }
                    HomeItems::Themes | HomeItems::Project => {
                        ui_state.in_home = true;
                        ui_state.in_submenu = false;
                        ui_state.submenu_hovered = Some(0);
                        ui_state.submenu_selected = None;
                    }
                },
                KeyCode::Char('c') if key_event.modifiers == KeyModifiers::CONTROL => break,
                _ => {}
            }

            match ui_state.home_state {
                HomeItems::Modes => {
                    if let Some(n) = ui_state.submenu_selected.take() {
                        match n {
                            0 => {
                                ui_state.in_submenu = false;
                                ui_state.mode = Some(Mode::Client);

                                let username = ui_state.username.clone();
                                let accept_user_list_tx_clone = accept_user_list_tx.clone();
                                thread::spawn(move || {
                                    connect_to_server(&username, accept_user_list_tx_clone)
                                });
                            }
                            1 => {
                                ui_state.in_submenu = false;
                                ui_state.mode = Some(Mode::Host);

                                let accept_conn_tx_clone = accept_conn_tx.clone();
                                thread::spawn(move || {
                                    if let Err(err) =
                                        accept_connections(accept_conn_tx_clone.clone())
                                    {
                                        let _ = accept_conn_tx_clone
                                            .send(GetClientConnection::Error(err.kind()));
                                    }
                                });
                            }

                            _ => {}
                        }
                    }
                }
                HomeItems::Themes => {
                    if let Some(n) = ui_state.submenu_selected.take() {
                        match n {
                            0 => ui_state.theme = Theme::Dark,
                            1 => ui_state.theme = Theme::Light,
                            _ => {}
                        }
                    }
                }
                HomeItems::Project => {}
            }
        } else if ui_state.in_chat
            && event::poll(Duration::from_millis(16))?
            && let Event::Key(key_event) = event::read()?
        {
            // In chat
            if ui_state.error_occured {
                break;
            } else {
                // match key_event.code {
                //     KeyCode::Char('q') | KeyCode::Esc => break,
                //     KeyCode::Char('c') if key_event.modifiers == KeyModifiers::CONTROL => {
                //         break;
                //     }
                //     _ => {}
                // }

                match key_event.code {
                    KeyCode::Char('c') if key_event.modifiers == KeyModifiers::CONTROL => {
                        break;
                    }

                    KeyCode::Esc => break,

                    KeyCode::Char(c) => {
                        ui_state.input.push(c);
                    }

                    KeyCode::Backspace => {
                        ui_state.input.pop();
                    }

                    KeyCode::Enter => {
                        if !ui_state.input.trim().is_empty() {
                            ui_state.last_message = ui_state.input.clone();
                            let _ = send_message_tx.send(ui_state.input.clone());
                            ui_state.input.clear();
                        }
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
