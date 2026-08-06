use crate::ui::themes::Theme;

pub struct UiState {
    pub theme: Theme,

    pub in_home: bool,
    pub home_state: HomeItems,

    pub in_submenu: bool,

    pub mode: Option<Mode>,
    pub mode_activated: bool,
    pub in_chat: bool,

    pub username: String,

    pub home_hovered: Option<usize>,
    pub home_selected: Option<usize>,

    pub submenu_hovered: Option<usize>,
    pub submenu_selected: Option<usize>,
    pub error_occured: bool,

    pub input: String,

    pub last_message: String,
}

#[derive(PartialEq)]
pub enum HomeItems {
    Modes,
    Themes,
    Project,
}

pub enum Mode {
    Client,
    Host,
    Error(std::io::ErrorKind),
}

impl UiState {
    pub fn new() -> Self {
        Self {
            theme: Theme::Dark,

            in_home: true,
            home_state: HomeItems::Modes,

            in_submenu: false,

            in_chat: false,
            mode: None,
            mode_activated: false,

            username: String::new(),

            home_hovered: Some(0),
            home_selected: None,

            submenu_hovered: Some(0),
            submenu_selected: None,
            error_occured: false,

            input: String::new(),
            last_message: String::new(),
        }
    }
}
