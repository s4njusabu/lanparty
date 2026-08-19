use crate::services::username;

mod states;
mod services;
mod ui;
pub mod themes;

fn main() {
    let terminal = ratatui::init();
    let mut username = username::default_username();

    // loop {

    // }

    ratatui::restore();
    println!("Bye from LAN Party!");
}