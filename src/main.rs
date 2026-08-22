use std::{
    net::{IpAddr, Ipv4Addr},
    thread,
    time::Duration,
};

use ratatui::{
    crossterm::event::{self, Event, KeyCode, KeyModifiers},
    style::Style,
    widgets::Block,
};

use crate::{
    services::network::send_udp_packets_to_broadcast,
    states::{
        group_chat_state::{self, User},
        private_chat_state::PrivateChatState,
        ui_state::{GroupChatMode, HomeOptions, InChat, InputMode, UiState},
    },
    ui::{
        border,
        chat::{gc_client, gc_host, private_chat},
        error_page, file_transfer_menu, gc_selector, group_chat_menu, home, private_chat_menu,
        profile_menu, themes_menu,
    },
};

mod services;
mod states;
pub mod themes;
mod ui;

fn main() {
    if !services::system::ip_command_exists() {
        println!("\"ip\" command not found");
        if let Some(command) = services::system::command_to_install_ip() {
            println!("Install it with: {command}");
        }

        return;
    }

    let mut terminal = ratatui::init();

    // States
    let mut ui_state = UiState::new();
    let mut private_chat_state = PrivateChatState::new();
    let mut gc_host_state = group_chat_state::GroupChatHostState::new();
    let mut gc_client_state = group_chat_state::GroupChatClientState::new();

    loop {
        // Render block
        if let Err(err) = terminal.draw(|frame| {
            frame.render_widget(
                Block::new().style(Style::default().bg(ui_state.theme.colors().background)),
                frame.area(),
            );
            let inner = border::draw_border(frame, &ui_state);

            if let Some(err) = ui_state.error.take() {
                error_page::draw_error_page(frame, inner, &ui_state, err);
            } else if ui_state.in_home {
                ui::home::draw_home(frame, inner, &ui_state);
            } else if ui_state.in_submenu
                && let Some(n) = ui_state.home_hovered
            {
                match n {
                    0 => private_chat_menu::draw_private_chat_modes_menu(frame, inner, &ui_state),
                    1 => group_chat_menu::draw_group_chat_modes_menu(frame, inner, &ui_state),
                    2 => file_transfer_menu::draw_file_transfer_menu(frame, inner, &ui_state),
                    3 => profile_menu::draw_profile_menu(frame, inner, &ui_state),
                    4 => themes_menu::draw_themes_menu(frame, inner, &ui_state),
                    _ => {}
                }
            } else if ui_state.in_chat.is_some()
                && let Some(mode) = ui_state.in_chat
            {
                match mode {
                    InChat::Private => private_chat::draw_host(frame, inner),
                    InChat::Group => {
                        if let Some(mode) = ui_state.gc_mode {
                            match mode {
                                GroupChatMode::Client => {
                                    gc_selector::draw_group_chat_selector(frame, inner, &ui_state, &gc_client_state);
                                }
                                GroupChatMode::Host => gc_host::draw_host(frame, inner, &ui_state),
                            }
                        }
                    }
                }
            }
        }) {
            ui_state.error = Some(err.kind());
            break;
        }

        // Handle error
        if ui_state.error.is_some() {
            match event::poll(Duration::from_millis(16)) {
                Ok(true) => {}
                Ok(false) => continue,
                Err(_) => continue,
            }

            match event::read() {
                Ok(Event::Key(_)) => break,
                Ok(_) => continue,
                Err(_) => continue,
            }
        }

        // In home menu
        if ui_state.in_home
            && match event::poll(Duration::from_millis(16)) {
                Ok(result) => result,
                Err(err) => {
                    ui_state.error = Some(err.kind());
                    continue;
                }
            }
            && let Event::Key(key_event) = match event::read() {
                Ok(event) => event,
                Err(err) => {
                    ui_state.error = Some(err.kind());
                    continue;
                }
            }
        {
            match key_event.code {
                KeyCode::Enter | KeyCode::Right => ui_state.home_selected = ui_state.home_hovered,
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
                        ui_state.home_state = HomeOptions::PrivateChat;
                    }
                    1 => {
                        ui_state.in_home = false;
                        ui_state.in_submenu = true;
                        ui_state.home_state = HomeOptions::GroupChat;
                    }
                    2 => {
                        ui_state.in_home = false;
                        ui_state.in_submenu = true;
                        ui_state.home_state = HomeOptions::FileTransfer;
                    }
                    3 => {
                        ui_state.in_home = false;
                        ui_state.in_submenu = true;
                        ui_state.home_state = HomeOptions::Profile;
                    }
                    4 => {
                        ui_state.in_home = false;
                        ui_state.in_submenu = true;
                        ui_state.home_state = HomeOptions::Themes;
                    }
                    5 => break,
                    _ => {}
                }
            }
        } else if ui_state.in_submenu
            && match event::poll(Duration::from_millis(16)) {
                Ok(result) => result,
                Err(err) => {
                    ui_state.error = Some(err.kind());
                    continue;
                }
            }
            && let Event::Key(key_event) = match event::read() {
                Ok(event) => event,
                Err(err) => {
                    ui_state.error = Some(err.kind());
                    continue;
                }
            }
        {
            // In submenu
            // Key logic layer 1
            match key_event.code {
                KeyCode::Enter if ui_state.input_mode.is_none() => {
                    ui_state.submenu_selected = ui_state.submenu_hovered;
                }
                KeyCode::Left if ui_state.input_mode.is_none() => {
                    ui_state.in_home = true;
                    ui_state.in_submenu = false;
                    ui_state.submenu_hovered = Some(0);
                    ui_state.submenu_selected = None;
                }
                KeyCode::Right if ui_state.input_mode.is_none() => {
                    ui_state.submenu_selected = ui_state.submenu_hovered
                }
                KeyCode::Up if ui_state.input_mode.is_none() => {
                    if let Some(n) = ui_state.submenu_hovered
                        && n > 0
                    {
                        ui_state.submenu_hovered = Some(n - 1)
                    }
                }
                KeyCode::Down if ui_state.input_mode.is_none() => match ui_state.home_state {
                    HomeOptions::PrivateChat => {
                        if let Some(n) = ui_state.submenu_hovered
                            && n < private_chat_menu::PRIVATE_CHAT_MODES_MAX_INDEX
                        {
                            ui_state.submenu_hovered = Some(n + 1);
                        }
                    }
                    HomeOptions::GroupChat => {
                        if let Some(n) = ui_state.submenu_hovered
                            && n < group_chat_menu::GROUP_CHAT_MODES_MAX_INDEX
                        {
                            ui_state.submenu_hovered = Some(n + 1);
                        }
                    }
                    HomeOptions::FileTransfer => {
                        if let Some(n) = ui_state.submenu_hovered
                            && n < file_transfer_menu::FILE_TRANSFER_OPTIONS_MAX_INDEX
                        {
                            ui_state.submenu_hovered = Some(n + 1);
                        }
                    }
                    HomeOptions::Profile => {
                        if let Some(n) = ui_state.submenu_hovered
                            && n < profile_menu::PROFILE_OPTIONS_MAX_INDEX
                        {
                            ui_state.submenu_hovered = Some(n + 1);
                        }
                    }
                    HomeOptions::Themes => {
                        if let Some(n) = ui_state.submenu_hovered
                            && n < themes_menu::THEME_OPTIONS_MAX_INDEX
                        {
                            ui_state.submenu_hovered = Some(n + 1);
                        }
                    }
                },
                KeyCode::Char('q') | KeyCode::Esc if ui_state.input_mode.is_none() => {
                    ui_state.in_home = true;
                    ui_state.in_submenu = false;
                    ui_state.submenu_hovered = Some(0);
                    ui_state.submenu_selected = None;
                }
                KeyCode::Char('c') if key_event.modifiers == KeyModifiers::CONTROL => break,
                _ => {}
            }

            if let Some(mode) = ui_state.input_mode {
                // Key logic layer 2
                match mode {
                    InputMode::PrivateChat => match key_event.code {
                        KeyCode::Char(c) => {
                            if (c.is_ascii_digit() || c == '.') && ui_state.input.len() < 15 {
                                ui_state.input.push(c);
                            }
                        }

                        KeyCode::Backspace => {
                            ui_state.input.pop();
                        }

                        KeyCode::Enter | KeyCode::Right => {
                            if let Ok(ip) = ui_state.input.parse::<Ipv4Addr>() {
                                private_chat_state.connected_user_ip = Some(ip);

                                ui_state.input.clear();
                                ui_state.input_mode = None;
                                ui_state.submenu_selected = None;

                                ui_state.in_submenu = false;
                                ui_state.in_chat = Some(InChat::Private);
                            }
                        }

                        KeyCode::Esc => {
                            ui_state.input.clear();
                            ui_state.input_mode = None;
                            ui_state.submenu_selected = None;
                        }

                        _ => {}
                    },
                    InputMode::GroupChat => {}
                    InputMode::ChangeUsername => match key_event.code {
                        KeyCode::Char(c) => {
                            if ui_state.username.len() < 15 {
                                ui_state.username.push(c);
                            }
                        }
                        KeyCode::Backspace => {
                            ui_state.username.pop();
                        }
                        KeyCode::Enter | KeyCode::Right => {
                            if ui_state.username.len() >= 3 {
                                ui_state.input_mode = None;
                                ui_state.previous_text.clear();
                                ui_state.submenu_selected = None;
                            }
                        }
                        KeyCode::Esc => {
                            ui_state.username = ui_state.previous_text.clone();
                            ui_state.previous_text.clear();
                            ui_state.input_mode = None;
                            ui_state.submenu_selected = None;
                        }

                        _ => {}
                    },
                }
            } else {
                match ui_state.home_state {
                    HomeOptions::PrivateChat => {
                        if let Some(n) = ui_state.submenu_selected.take() {
                            match n {
                                0 => {
                                    ui_state.input.clear();
                                    ui_state.input_mode = Some(InputMode::PrivateChat);
                                }
                                1 => {
                                    ui_state.in_home = true;
                                    ui_state.in_submenu = false;
                                    ui_state.submenu_hovered = Some(0);
                                }
                                _ => {}
                            }
                        }
                    }
                    HomeOptions::GroupChat => {
                        if let Some(n) = ui_state.submenu_selected.take() {
                            match n {
                                0 => {
                                    ui_state.in_chat = Some(InChat::Group);
                                    ui_state.gc_mode = Some(GroupChatMode::Client);
                                    ui_state.in_submenu = false;
                                }
                                1 => {
                                    ui_state.in_chat = Some(InChat::Group);
                                    ui_state.gc_mode = Some(GroupChatMode::Host);
                                    ui_state.in_submenu = false;
                                }
                                2 => {
                                    ui_state.in_home = true;
                                    ui_state.in_submenu = false;
                                    ui_state.submenu_hovered = Some(0);
                                }
                                _ => {}
                            }
                        }
                    }
                    HomeOptions::FileTransfer => {
                        if let Some(n) = ui_state.submenu_selected.take() {
                            match n {
                                0 => {}
                                1 => {}
                                2 => {
                                    ui_state.in_home = true;
                                    ui_state.in_submenu = false;
                                    ui_state.submenu_hovered = Some(0);
                                }
                                _ => {}
                            }
                        }
                    }
                    HomeOptions::Profile => {
                        if let Some(n) = ui_state.submenu_selected.take() {
                            match n {
                                0 => {
                                    ui_state.previous_text = ui_state.username.clone();
                                    ui_state.username.clear();
                                    ui_state.input_mode = Some(InputMode::ChangeUsername);
                                }
                                1 => {
                                    ui_state.in_home = true;
                                    ui_state.in_submenu = false;
                                    ui_state.submenu_hovered = Some(0);
                                }
                                _ => {}
                            }
                        }
                    }
                    HomeOptions::Themes => {
                        if let Some(n) = ui_state.submenu_selected.take() {
                            match n {
                                0 => ui_state.theme = themes::Theme::Dark,
                                1 => ui_state.theme = themes::Theme::Light,
                                2 => {
                                    ui_state.in_home = true;
                                    ui_state.in_submenu = false;
                                    ui_state.submenu_hovered = Some(0);
                                }
                                _ => {}
                            }
                        }
                    }
                }
            }
        } else if ui_state.in_chat.is_some()
            && let Some(chat_mode) = ui_state.in_chat
        {
            match chat_mode {
                InChat::Private => {}
                InChat::Group => {
                    if let Some(gc_mode) = ui_state.gc_mode {
                        match gc_mode {
                            GroupChatMode::Client => {}
                            GroupChatMode::Host => {
                                // Host initialization
                                if !gc_host_state.ran_once {
                                    gc_host_state.ran_once = true;

                                    gc_host_state.users.insert(
                                        ui_state.local_ip.parse::<IpAddr>().unwrap(),
                                        User {
                                            username: ui_state.username.clone(),
                                            online: true,
                                        },
                                    );

                                    thread::spawn(send_udp_packets_to_broadcast);
                                }

                                gc_client_state.users = gc_host_state.users.clone();
                            }
                        }
                    }
                }
            }
        }
    }

    ratatui::restore();

    if ui_state.error.is_some() {
        println!("An error occurred while rendering the terminal.");
    } else {
        println!("Bye from LAN Party!");
    }
}
