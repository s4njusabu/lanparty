// TO CHANGE

use std::{
    collections::HashMap,
    net::{IpAddr, Ipv4Addr, TcpStream},
};

use serde::{Deserialize, Serialize};

pub struct GroupChatState {
    pub users: HashMap<IpAddr, User>,
    pub connections: HashMap<IpAddr, TcpStream>,
    pub messages: Vec<Message>,
}

impl GroupChatState {
    pub fn new() -> Self {
        Self {
            users: HashMap::new(),
            connections: HashMap::new(),
            messages: Vec::new(),
        }
    }
}

// V2 rewrite

pub struct GroupChatHostState {
    pub users: HashMap<Ipv4Addr, User>,
    pub messages: Vec<Message>,
    pub added_host: bool,
}

impl GroupChatHostState {
    pub fn new() -> Self {
        Self {
            users: HashMap::new(),
            messages: Vec::new(),
            added_host: false,
        }
    }
}

pub struct GroupChatClientState {
    pub users: HashMap<Ipv4Addr, User>,
    pub messages: Vec<Message>,
}

impl GroupChatClientState {
    pub fn new() -> Self {
        Self {
            users: HashMap::new(),
            messages: Vec::new(),
        }
    }
}

pub struct Message {
    pub sender: Ipv4Addr,
    pub message: String,
}

#[derive(Clone)]
pub struct User {
    pub username: String,
    pub online: bool,
}
