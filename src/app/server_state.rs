use std::{collections::HashMap, net::IpAddr};

pub struct ServerState {
    pub users: HashMap<IpAddr, User>,
    pub messages: Vec<Message>,
}

pub struct User {
    pub username: String,
    pub online: bool,
}

pub struct Message {
    pub sender: IpAddr,
    pub message: String,
}

impl ServerState {
    pub fn new() -> Self {
        Self {
            users: HashMap::new(),
            messages: Vec::new(),
        }
    }
}
