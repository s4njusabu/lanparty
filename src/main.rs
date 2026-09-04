use std::{net::IpAddr, sync::mpsc, thread, time::Duration};

use ratatui::{
    crossterm::event::{self, Event, KeyCode, KeyModifiers},
    style::Style,
    widgets::Block,
};

use crate::{
    services::{
        network::{
            accept_connections, create_connection, receive_udp_packets_from_broadcast,
            send_udp_packets_to_broadcast,
        },
        system,
    },
    states::{
        group_chat_state::{self, Message, Packet, User},
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

    // Init changes for states
    if let Some(ip) = system::get_local_ip() {
        ui_state.local_ip = ip;
    } else {
        ui_state.error = Some(std::io::Error::other("Failed to get IP address"));
    }

    // Channels
    let (host_discovery_tx, host_discovery_rx) = mpsc::channel::<(IpAddr, String)>();
    let (from_client_tx, from_client_rx) = mpsc::channel::<Packet>();
    let (to_clients_tx, to_clients_rx) = mpsc::channel::<Packet>();
    let mut to_clients_rx = Some(to_clients_rx);
    let (to_host_tx, to_host_rx) = mpsc::channel::<String>();
    let mut to_host_rx = Some(to_host_rx);
    let (error_tx, error_rx) = mpsc::channel::<std::io::Error>();

    loop {
        // Render block
        if let Err(err) = terminal.draw(|frame| {
            frame.render_widget(
                Block::new().style(Style::default().bg(ui_state.theme.colors().background)),
                frame.area(),
            );
            let inner = border::draw_border(frame, &ui_state);

            if let Some(err) = &ui_state.error {
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
                                    if !gc_client_state.host_decided {
                                        gc_selector::draw_group_chat_selector(
                                            frame,
                                            inner,
                                            &ui_state,
                                            &gc_client_state,
                                        );
                                    } else {
                                        gc_client::draw_client(
                                            frame,
                                            inner,
                                            &mut ui_state,
                                            &gc_client_state,
                                        );
                                    }
                                }
                                GroupChatMode::Host => {
                                    gc_host::draw_host(frame, inner, &mut ui_state, &gc_host_state)
                                }
                            }
                        }
                    }
                }
            }
        }) {
            ui_state.error = Some(err);
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
                    ui_state.error = Some(err);
                    continue;
                }
            }
            && let Event::Key(key_event) = match event::read() {
                Ok(event) => event,
                Err(err) => {
                    ui_state.error = Some(err);
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
                    ui_state.error = Some(err);
                    continue;
                }
            }
            && let Event::Key(key_event) = match event::read() {
                Ok(event) => event,
                Err(err) => {
                    ui_state.error = Some(err);
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
                            if let Ok(ip) = ui_state.input.parse::<IpAddr>() {
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
                    InputMode::GroupChat => match key_event.code {
                        KeyCode::Char(c) => {
                            if (c.is_ascii_digit() || c == '.') && ui_state.input.len() < 15 {
                                ui_state.input.push(c);
                            }
                        }

                        KeyCode::Backspace => {
                            ui_state.input.pop();
                        }

                        KeyCode::Enter | KeyCode::Right => {
                            if let Ok(ip) = ui_state.input.parse::<IpAddr>() {
                                gc_client_state.host_decided = true;
                                gc_client_state.host_ip = Some(ip);

                                ui_state.input.clear();
                                ui_state.input_mode = None;
                            }
                        }

                        KeyCode::Esc => {
                            ui_state.input.clear();
                            ui_state.input_mode = None;
                        }

                        _ => {}
                    },
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
                            GroupChatMode::Client => {
                                // Things that needs to be ran once
                                if !gc_client_state.ran_once {
                                    gc_client_state.ran_once = true;

                                    let host_discovery_tx_clone = host_discovery_tx.clone();

                                    thread::spawn(move || {
                                        loop {
                                            if let Ok(host_info) =
                                                receive_udp_packets_from_broadcast()
                                                && host_discovery_tx_clone.send(host_info).is_err()
                                            {
                                                break;
                                            }
                                        }
                                    });
                                }

                                // If statement for gc_selector
                                if !gc_client_state.host_decided {
                                    while let Ok((host_ip, host_name)) =
                                        host_discovery_rx.try_recv()
                                    {
                                        gc_client_state.discovered_hosts.insert(host_ip, host_name);
                                    }

                                    if event::poll(Duration::from_millis(16)).unwrap_or(false)
                                        && let Event::Key(key_event) = match event::read() {
                                            Ok(event) => event,
                                            Err(err) => {
                                                ui_state.error = Some(err);
                                                continue;
                                            }
                                        }
                                    {
                                        match key_event.code {
                                            KeyCode::Enter if ui_state.input_mode.is_none() => {
                                                ui_state.input = gc_client_state
                                                    .discovered_hosts
                                                    .keys()
                                                    .next()
                                                    .map(|ip| ip.to_string())
                                                    .unwrap_or_default();

                                                ui_state.input_mode = Some(InputMode::GroupChat);
                                            }
                                            KeyCode::Esc if ui_state.input_mode.is_none() => break,

                                            KeyCode::Char(c)
                                                if ui_state.input_mode
                                                    == Some(InputMode::GroupChat) =>
                                            {
                                                if (c.is_ascii_digit() || c == '.')
                                                    && ui_state.input.len() < 15
                                                {
                                                    ui_state.input.push(c);
                                                }
                                            }

                                            KeyCode::Backspace
                                                if ui_state.input_mode
                                                    == Some(InputMode::GroupChat) =>
                                            {
                                                ui_state.input.pop();
                                            }

                                            KeyCode::Enter | KeyCode::Right
                                                if ui_state.input_mode
                                                    == Some(InputMode::GroupChat) =>
                                            {
                                                if let Ok(ip) = ui_state.input.parse::<IpAddr>() {
                                                    gc_client_state.host_ip = Some(ip);
                                                    gc_client_state.host_decided = true;

                                                    ui_state.input.clear();
                                                    ui_state.input_mode = None;
                                                }
                                            }

                                            KeyCode::Esc
                                                if ui_state.input_mode
                                                    == Some(InputMode::GroupChat) =>
                                            {
                                                ui_state.input.clear();
                                                ui_state.input_mode = None;
                                            }

                                            _ => {}
                                        }
                                    }
                                }
                                // After deciding the host connect to it
                                if gc_client_state.host_decided && !gc_client_state.connected {
                                    let host_ip = gc_client_state.host_ip.unwrap();
                                    let username = ui_state.username.clone();
                                    let from_clients_tx_clone = from_client_tx.clone();
                                    let error_tx_clone = error_tx.clone();

                                    if let Some(to_server_rx) = to_host_rx.take() {
                                        thread::spawn(move || {
                                            let connection_error_tx = error_tx_clone.clone();

                                            if let Err(err) = accept_connections(
                                                host_ip,
                                                username,
                                                from_clients_tx_clone,
                                                to_server_rx,
                                                connection_error_tx,
                                            ) {
                                                let _ = error_tx_clone.send(err);
                                            }
                                        });
                                    }
                                    gc_client_state.connected = true;
                                }

                                while let Ok(err) = error_rx.try_recv() {
                                    ui_state.error = Some(err);
                                }

                                while let Ok(packet) = from_client_rx.try_recv() {
                                    match packet {
                                        Packet::UserList(users) => {
                                            gc_client_state.users = users;
                                        }

                                        Packet::Message(msg) => {
                                            gc_client_state.messages.push(msg);
                                        }

                                        _ => {}
                                    }
                                }
                                // After deciding the host continue the chat (key logic of chat)
                                if gc_client_state.host_decided
                                    && event::poll(Duration::from_millis(16)).unwrap_or(false)
                                    && let Event::Key(key_event) = match event::read() {
                                        Ok(event) => event,
                                        Err(err) => {
                                            ui_state.error = Some(err);
                                            continue;
                                        }
                                    }
                                {
                                    match key_event.code {
                                        KeyCode::Char(c) => {
                                            ui_state.input.push(c);
                                        }

                                        KeyCode::Backspace => {
                                            ui_state.input.pop();
                                        }

                                        KeyCode::Enter => {
                                            if !ui_state.input.is_empty() {
                                                let message = ui_state.input.clone();

                                                let _ = to_host_tx.send(message.clone());

                                                ui_state.input.clear();

                                                ui_state.chat_at_bottom = true;
                                                ui_state.chat_scroll = ui_state.chat_max_scroll;
                                            }
                                        }

                                        KeyCode::Up => {
                                            if ui_state.chat_at_bottom {
                                                ui_state.chat_at_bottom = false;
                                            }

                                            ui_state.chat_scroll =
                                                ui_state.chat_scroll.saturating_sub(1);
                                        }

                                        KeyCode::Down => {
                                            ui_state.chat_scroll = ui_state
                                                .chat_scroll
                                                .saturating_add(1)
                                                .min(ui_state.chat_max_scroll);

                                            if ui_state.chat_scroll == ui_state.chat_max_scroll {
                                                ui_state.chat_at_bottom = true;
                                            }
                                        }

                                        KeyCode::Esc => break,
                                        _ => {}
                                    }
                                }
                            }
                            GroupChatMode::Host => {
                                // Host initialization
                                if !gc_host_state.ran_once {
                                    gc_host_state.ran_once = true;

                                    gc_host_state.users.insert(
                                        ui_state.local_ip,
                                        User {
                                            username: ui_state.username.clone(),
                                            online: true,
                                        },
                                    );

                                    let username = ui_state.username.clone();
                                    thread::spawn(move || send_udp_packets_to_broadcast(&username));

                                    let from_client_tx_clone = from_client_tx.clone();
                                    let error_tx_clone = error_tx.clone();
                                    // Thread
                                    if let Some(to_clients_rx) = to_clients_rx.take() {
                                        thread::spawn(move || {
                                            if let Err(err) = create_connection(
                                                ui_state.local_ip,
                                                from_client_tx_clone,
                                                to_clients_rx,
                                            ) {
                                                let _ = error_tx_clone.send(err);
                                            }
                                        });
                                    }
                                }

                                while let Ok(err) = error_rx.try_recv() {
                                    ui_state.error = Some(err);
                                }
                                while let Ok(packet) = from_client_rx.try_recv() {
                                    match packet {
                                        Packet::UserConnected { ip, username } => {
                                            gc_host_state.users.insert(
                                                ip,
                                                User {
                                                    username,
                                                    online: true,
                                                },
                                            );

                                            let _ = to_clients_tx.send(Packet::UserList(
                                                gc_host_state.users.clone(),
                                            ));
                                        }

                                        Packet::UserDisconnected(ip) => {
                                            if let Some(user) = gc_host_state.users.get_mut(&ip) {
                                                user.online = false;
                                            }

                                            let _ = to_clients_tx.send(Packet::UserList(
                                                gc_host_state.users.clone(),
                                            ));
                                        }

                                        Packet::Message(msg) => {
                                            gc_host_state.messages.push(msg.clone());

                                            let _ = to_clients_tx.send(Packet::Message(msg));
                                        }

                                        Packet::UserList(_) => {}
                                    }
                                }

                                // Host chat key logic
                                if event::poll(Duration::from_millis(16)).unwrap_or(false)
                                    && let Event::Key(key_event) = match event::read() {
                                        Ok(event) => event,
                                        Err(err) => {
                                            ui_state.error = Some(err);
                                            continue;
                                        }
                                    }
                                {
                                    match key_event.code {
                                        KeyCode::Char(c) => {
                                            ui_state.input.push(c);
                                        }

                                        KeyCode::Backspace => {
                                            ui_state.input.pop();
                                        }

                                        KeyCode::Enter => {
                                            if !ui_state.input.is_empty() {
                                                let message = Message {
                                                    sender: ui_state.local_ip,
                                                    message: ui_state.input.clone(),
                                                };

                                                gc_host_state.messages.push(message.clone());

                                                let _ =
                                                    to_clients_tx.send(Packet::Message(message));

                                                ui_state.input.clear();

                                                ui_state.chat_at_bottom = true;
                                                ui_state.chat_scroll = ui_state.chat_max_scroll;
                                            }
                                        }
                                        KeyCode::Up => {
                                            if ui_state.chat_at_bottom {
                                                ui_state.chat_at_bottom = false;
                                            }

                                            ui_state.chat_scroll =
                                                ui_state.chat_scroll.saturating_sub(1);
                                        }

                                        KeyCode::Down => {
                                            ui_state.chat_scroll = ui_state
                                                .chat_scroll
                                                .saturating_add(1)
                                                .min(ui_state.chat_max_scroll);

                                            if ui_state.chat_scroll == ui_state.chat_max_scroll {
                                                ui_state.chat_at_bottom = true;
                                            }
                                        }
                                        KeyCode::Esc => break,
                                        _ => {}
                                    }
                                }
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
