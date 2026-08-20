use std::io::ErrorKind;

use crate::{services::username, themes::Theme};

pub struct UiState {
    pub theme: Theme,
    pub username: String,

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

    // things that may change
    pub input: String,
    pub last_message: String,
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

pub enum GroupChatMode {
    Client,
    Host,
    Error(std::io::ErrorKind),
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
            last_message: String::new(),
        }
    }
}
