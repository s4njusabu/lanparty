use serde::{Deserialize, Serialize};
use std::{collections::HashMap, net::IpAddr};

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
    pub host_decided: bool,
    pub host_ip: Option<IpAddr>,
    pub ran_once: bool,
    pub connected: bool,
}

impl GroupChatClientState {
    pub fn new() -> Self {
        Self {
            users: HashMap::new(),
            messages: Vec::new(),
            discovered_hosts: HashMap::new(),
            host_decided: false,
            host_ip: None,
            ran_once: false,
            connected: false,
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct Message {
    pub sender: IpAddr,
    pub message: String,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct User {
    pub username: String,
    pub online: bool,
}

#[derive(Serialize, Deserialize)]
pub enum Packet {
    UserConnected { ip: IpAddr, username: String },
    UserDisconnected(IpAddr),
    Message(Message),
    UserList(HashMap<IpAddr, User>),
}
