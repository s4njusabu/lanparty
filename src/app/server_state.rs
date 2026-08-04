use std::{collections::HashMap, net::IpAddr};

pub struct ServerState {
    pub users: HashMap<IpAddr, String>,
    pub messages: Vec<Message>,
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
