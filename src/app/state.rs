use crate::ui::themes::Theme;

pub struct State {
    pub theme: Theme,

    pub in_home: bool,
    pub home_state: HomeItems,

    pub in_submenu: bool,

    pub in_chat: bool,

    pub mode: Option<Mode>,

    pub home_hovered: Option<usize>,
    pub home_selected: Option<usize>,

    pub submenu_hovered: Option<usize>,
    pub submenu_selected: Option<usize>,
}

pub enum HomeItems {
    Modes,
    Themes,
    Project,
}

pub enum Mode {
    Client,
    Server,
}

impl State {
    pub fn new() -> Self {
        State {
            theme: Theme::Dark,

            in_home: true,
            home_state: HomeItems::Modes,

            in_submenu: false,

            in_chat: false,

            mode: None,

            home_hovered: Some(0),
            home_selected: None,

            submenu_hovered: Some(0),
            submenu_selected: None,
        }
    }
}
