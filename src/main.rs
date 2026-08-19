#![allow(unused)]
use crate::services::username;

mod services;
mod states;
pub mod themes;
mod ui;

fn main() {
    let terminal = ratatui::init();
    let mut username = username::default_username();

    // loop {

    // }

    ratatui::restore();
    println!("{username}");
    username = "sanjusabu".to_string();
    println!("{username}");
    println!("Bye from LAN Party!");
}
