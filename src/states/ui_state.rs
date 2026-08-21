use std::io::ErrorKind;

use crate::{
    services::{interface, username},
    themes::Theme,
};

pub struct UiState {
    pub theme: Theme,
    pub username: String,
    pub local_ip: String,

    pub in_home: bool,
    pub in_submenu: bool,
    pub in_chat: Option<InChat>,

    pub home_state: HomeOptions,

    // group chat
    pub group_chat_mode: Option<GroupChatMode>,
    pub group_chat_selected: bool,

    // home state
    pub home_hovered: Option<usize>,
    pub home_selected: Option<usize>,

    // submenu state
    pub submenu_hovered: Option<usize>,
    pub submenu_selected: Option<usize>,

    pub error: Option<ErrorKind>,

    // input state
    pub input_mode: Option<InputMode>,
    pub input: String,

    pub previous_text: String,
}

#[derive(PartialEq)]
pub enum HomeOptions {
    PrivateChat,
    GroupChat,
    FileTransfer,
    Profile,
    Themes,
}

pub enum InChat {
    Private,
    Group,
}

#[derive(PartialEq, Clone, Copy)]
pub enum InputMode {
    ChangeUsername,
    PrivateChat,
    GroupChat,
}

// Enum that i should consider doing some changes
pub enum GroupChatMode {
    Client,
    Host,
    Error(std::io::ErrorKind),
}

impl UiState {
    pub fn new() -> Self {
        let local_ip = if let Some(ip) = interface::get_local_ip() {
            ip
        } else {
            String::from("Unknown")
        };
        Self {
            theme: Theme::Dark,
            username: username::default_username(),
            local_ip,

            in_home: true,
            in_submenu: false,
            in_chat: None,

            home_state: HomeOptions::PrivateChat,

            // group chat
            group_chat_mode: None,
            group_chat_selected: false,

            // home state
            home_hovered: Some(0),
            home_selected: None,

            // submenu state
            submenu_hovered: Some(0),
            submenu_selected: None,

            error: None,

            input: String::new(),
            input_mode: None,

            previous_text: String::new(),
        }
    }
}
