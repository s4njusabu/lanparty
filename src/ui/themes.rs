use ratatui::style::Color;

pub struct ThemeStyle {
    pub background: Color,
    pub accent: Color,

    pub text: Color,
    pub banner: Color,

    pub selected: Color,
}

pub enum Theme {
    // Dark mode
    // background: rgb(26, 26, 26)
    // accent: rgb(223, 208, 184)
    // text: rgb(250, 240, 230)
    // banner: rgb(236, 223, 204)
    // selected: rgb(220, 190, 120)
    Dark,

    // Light mode
    // background: rgb(245, 241, 236)
    // accent: rgb(186, 170, 150)
    // text: rgb(50, 47, 43)
    // banner: rgb(220, 186, 128)
    // selected: rgb(165, 118, 62)
    Light,
}

impl Theme {
    pub fn colors(&self) -> ThemeStyle {
        match self {
            Theme::Dark => ThemeStyle {
                background: Color::Rgb(26, 26, 26),  // rgb(26, 26, 26)
                accent: Color::Rgb(223, 208, 184),   // rgb(223, 208, 184)
                text: Color::Rgb(250, 240, 230),     // rgb(250, 240, 230)
                banner: Color::Rgb(236, 223, 204),   // rgb(236, 223, 204)
                selected: Color::Rgb(220, 190, 120), // rgb(220, 190, 120)
            },

            Theme::Light => ThemeStyle {
                background: Color::Rgb(245, 241, 236), // rgb(245, 241, 236)
                accent: Color::Rgb(186, 170, 150),     // rgb(186, 170, 150)
                text: Color::Rgb(50, 47, 43),          // rgb(50, 47, 43)
                banner: Color::Rgb(220, 186, 128),     // rgb(220, 186, 128)
                selected: Color::Rgb(165, 118, 62),    // rgb(165, 118, 62)
            },
        }
    }
}
