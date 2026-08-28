use std::{
    io::Error,
    net::{IpAddr, Ipv4Addr},
};

use crate::{services::username, themes::Theme};

pub struct UiState {
    pub theme: Theme,
    pub username: String,

    pub in_home: bool,
    pub in_submenu: bool,
    pub in_chat: Option<InChat>,

    pub home_state: HomeOptions,

    // home state
    pub home_hovered: Option<usize>,
    pub home_selected: Option<usize>,

    // submenu state
    pub submenu_hovered: Option<usize>,
    pub submenu_selected: Option<usize>,

    pub error: Option<Error>,

    // IP address
    pub local_ip: IpAddr,

    // input state
    pub input_mode: Option<InputMode>,
    pub input: String,

    pub previous_text: String,

    // group chat
    pub gc_mode: Option<GroupChatMode>,

    // scroll
    pub chat_scroll: u16,
    pub chat_max_scroll: u16,
    pub chat_at_bottom: bool,
}

#[derive(PartialEq)]
pub enum HomeOptions {
    PrivateChat,
    GroupChat,
    FileTransfer,
    Profile,
    Themes,
}

#[derive(PartialEq, Clone, Copy)]
pub enum InputMode {
    ChangeUsername,
    PrivateChat,
    GroupChat,
}

#[derive(Clone, Copy)]
pub enum InChat {
    Private,
    Group,
}

#[derive(Clone, Copy)]
pub enum GroupChatMode {
    Client,
    Host,
}

impl UiState {
    pub fn new() -> Self {
        Self {
            theme: Theme::Dark,
            username: username::default_username(),

            in_home: true,
            in_submenu: false,
            in_chat: None,

            home_state: HomeOptions::PrivateChat,

            // home state
            home_hovered: Some(0),
            home_selected: None,

            // submenu state
            submenu_hovered: Some(0),
            submenu_selected: None,

            error: None,

            // IP address
            local_ip: IpAddr::V4(Ipv4Addr::UNSPECIFIED),

            input: String::new(),
            input_mode: None,

            previous_text: String::new(),

            // group chat
            gc_mode: None,

            // scroll
            chat_scroll: 0,
            chat_max_scroll: 0,
            chat_at_bottom: true,
        }
    }
}
