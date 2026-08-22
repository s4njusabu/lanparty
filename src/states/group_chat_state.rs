// TO CHANGE

use std::{
    collections::HashMap,
    net::{IpAddr, TcpStream},
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
    pub users: HashMap<IpAddr, User>,
    pub messages: Vec<Message>,
    pub ran_once: bool,
}

impl GroupChatHostState {
    pub fn new() -> Self {
        Self {
            users: HashMap::new(),
            messages: Vec::new(),
            ran_once: false,
        }
    }
}

pub struct GroupChatClientState {
    pub users: HashMap<IpAddr, User>,
    pub messages: Vec<Message>,
    pub discovered_hosts: HashMap<IpAddr, String>,
}

impl GroupChatClientState {
    pub fn new() -> Self {
        Self {
            users: HashMap::new(),
            messages: Vec::new(),
            discovered_hosts: HashMap::new(),
        }
    }
}

pub struct Message {
    pub sender: IpAddr,
    pub message: String,
}

#[derive(Clone)]
pub struct User {
    pub username: String,
    pub online: bool,
}
