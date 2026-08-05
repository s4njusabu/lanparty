use std::{
    collections::HashMap,
    net::{IpAddr, TcpStream},
};

use serde::{Deserialize, Serialize};

pub struct ServerState {
    pub users: HashMap<IpAddr, User>,
    pub connections: HashMap<IpAddr, TcpStream>,
    pub messages: Vec<Message>,
}

#[derive(Clone, Serialize, Deserialize)]
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
            connections: HashMap::new(),
            messages: Vec::new(),
        }
    }
}
