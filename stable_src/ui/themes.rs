use ratatui::style::Color;

pub struct ThemeStyle {
    pub background: Color,
    pub accent: Color,

    pub text: Color,
    pub banner: Color,
}

pub enum Theme {
    // Dark mode
    // background: rgb(26, 26, 26)
    // accent: rgb(223, 208, 184)
    // text: rgb(250, 240, 230)
    // banner: rgb(236, 223, 204)
    Dark,

    // Light mode
    // background: rgb(255, 250, 250)
    // accent: rgb(201, 181, 156)
    // text: rgb(18, 18, 18)
    // banner: rgb(253, 216, 161)
    Light,
}

impl Theme {
    pub fn colors(&self) -> ThemeStyle {
        match self {
            // Default
            Theme::Dark => ThemeStyle {
                background: Color::Rgb(26, 26, 26), // rgb(26, 26, 26)
                accent: Color::Rgb(223, 208, 184),  // rgb(223, 208, 184)
                text: Color::Rgb(250, 240, 230),    // rgb(250, 240, 230)
                banner: Color::Rgb(236, 223, 204),  // rgb(236, 223, 204)
            },
            Theme::Light => ThemeStyle {
                background: Color::Rgb(255, 250, 250), // rgb(255, 250, 250)
                accent: Color::Rgb(201, 181, 156),     // rgb(201, 181, 156)
                text: Color::Rgb(18, 18, 18),          // rgb(18, 18, 18)
                banner: Color::Rgb(253, 216, 161),     // rgb(253, 216, 161)
            },
        }
    }
}
