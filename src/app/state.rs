use crate::ui::themes::Theme;

pub struct State {
    pub theme: Theme,
    pub in_home: bool,

    pub mode: Option<Mode>,
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

            mode: None,
        }
    }
}
