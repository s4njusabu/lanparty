use std::{
    collections::HashMap,
    io::{ErrorKind, Read, Write},
    net::{IpAddr, TcpListener, TcpStream, UdpSocket},
    process::Command,
    sync::mpsc::{Receiver, Sender},
    thread,
    time::Duration,
};

use serde::{Deserialize, Serialize};

use crate::app::server_state::{ServerState, User};

const DISCOVERY_PACKET: &[u8] = b"LANPARTY";
const DISCOVERY_PORT: u16 = 55555;

#[derive(Clone, Serialize, Deserialize)]
pub struct ChatMessage {
    pub sender: IpAddr,
    pub message: String,
}
pub enum GetClientConnection {
    ClientConnected(IpAddr, TcpStream, String),
    ClientDisconnected(IpAddr),
    Message(ChatMessage),
    Error(ErrorKind),
}

#[derive(Serialize, Deserialize)]
pub enum Packet {
    UserList(HashMap<IpAddr, User>),
    Message(ChatMessage),
}

fn ip_command_exists() -> bool {
    Command::new("ip")
        .arg("-V")
        .output()
        .is_ok_and(|output| output.status.success())
}

pub fn get_network_interface_and_user_ip() -> Option<(String, String)> {
    if let Ok(output) = Command::new("ip")
        .args(["route", "get", "8.8.8.8"])
        .output()
    {
        let mut interface = String::new();
        let mut user_ip = String::new();

        let text = String::from_utf8_lossy(&output.stdout);
        let mut words = text.split_whitespace();
        while let Some(word) = words.next() {
            if word == "dev"
                && let Some(v1) = words.next()
            {
                interface = v1.to_string();
            } else if word == "src"
                && let Some(v2) = words.next()
            {
                user_ip = v2.to_string();
            }
        }

        return Some((interface, user_ip));
    }

    None
}

fn get_broadcast_addr(interface: &str) -> Option<String> {
    if let Ok(output) = Command::new("ip")
        .args(["address", "show", interface])
        .output()
    {
        let text = String::from_utf8_lossy(&output.stdout);
        for line in text.lines() {
            let mut words = line.split_whitespace();
            while let Some(w1) = words.next() {
                if w1 == "inet" {
                    while let Some(w2) = words.next() {
                        if w2 == "brd" {
                            return words.next().map(str::to_string);
                        }
                    }
                }
            }
        }
    }

    None
}

pub fn send_udp_packets_to_broadcast() -> std::io::Result<()> {
    if !ip_command_exists() {
        return Err(std::io::Error::other("`ip` command not found"));
    }

    let (interface, user_ip) = get_network_interface_and_user_ip()
        .ok_or(std::io::Error::other("Failed to get network interface"))?;

    let broadcast = get_broadcast_addr(&interface)
        .ok_or(std::io::Error::other("Failed to get broadcast address"))?;

    let socket = UdpSocket::bind(format!("{user_ip}:0"))?;

    socket.set_broadcast(true)?;

    let destination = format!("{broadcast}:{DISCOVERY_PORT}");

    loop {
        thread::sleep(Duration::from_millis(250));

        socket.send_to(DISCOVERY_PACKET, &destination)?;
    }
}

pub fn receive_udp_packets_from_broadcast() -> std::io::Result<IpAddr> {
    if !ip_command_exists() {
        return Err(std::io::Error::other("`ip` command not found"));
    }

    let socket = UdpSocket::bind(format!("0.0.0.0:{DISCOVERY_PORT}"))?;
    let mut buf = [0u8; 1024];

    loop {
        let (n, sender) = socket.recv_from(&mut buf)?;
        if &buf[..n] != DISCOVERY_PACKET {
            continue;
        }

        return Ok(sender.ip());
    }
}

/*

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

pub enum GetClientConnection {
    ClientConnected(IpAddr, TcpStream, String),
    ClientDisconnected(IpAddr),
    Error(ErrorKind),
}

pub enum MessageFlow {
    SendToServer(String),
    GetFromServer(Vec<Message>),
}

pub enum Packet {
UserList(HashMap<IpAddr, User>),
Message(String),
}

*/

// host
pub fn accept_connections(accept_conn_tx: Sender<GetClientConnection>) -> std::io::Result<()> {
    let (_, user_ip) = get_network_interface_and_user_ip()
        .ok_or(std::io::Error::other("Failed to get network interface"))?;

    let listener = TcpListener::bind(format!("{user_ip}:55555"))?;

    loop {
        let (mut stream, addr) = listener.accept()?;
        let accept_conn_tx_clone = accept_conn_tx.clone();

        thread::spawn(move || -> std::io::Result<()> {
            let mut buf = [0u8; 4096];

            let n = stream.read(&mut buf)?;
            if n > 0 {
                let stream_copy = stream.try_clone()?;
                let username = String::from_utf8_lossy(&buf[..n]).to_string();

                accept_conn_tx_clone
                    .send(GetClientConnection::ClientConnected(
                        addr.ip(),
                        stream_copy,
                        username,
                    ))
                    .map_err(|_| std::io::Error::other("channel closed"))?;
            }

            loop {
                let n = stream.read(&mut buf)?;

                if n == 0 {
                    accept_conn_tx_clone
                        .send(GetClientConnection::ClientDisconnected(addr.ip()))
                        .map_err(|_| std::io::Error::other("channel closed"))?;
                    break;
                }

                let (packet, _): (Packet, usize) =
                    bincode::serde::decode_from_slice(&buf[..n], bincode::config::standard())
                        .map_err(std::io::Error::other)?;

                match packet {
                    Packet::Message(chat) => {
                        accept_conn_tx_clone
                            .send(GetClientConnection::Message(chat))
                            .map_err(|_| std::io::Error::other("channel closed"))?;
                    }

                    Packet::UserList(_) => {}
                }
            }

            Ok(())
        });
    }
}

// client
pub fn connect_to_server(
    username: &str,
    accept_user_list_tx: Sender<HashMap<IpAddr, User>>,
    accept_message_tx: Sender<ChatMessage>,
    outgoing_message_rx: Receiver<ChatMessage>,
) -> std::io::Result<()> {
    let server_ip = receive_udp_packets_from_broadcast()?;
    let mut stream = TcpStream::connect((server_ip, 55555))?;

    stream.set_nonblocking(true)?;
    stream.write_all(username.as_bytes())?;

    let mut buf = [0u8; 4096];

    loop {
        if let Ok(chat_message) = outgoing_message_rx.try_recv() {
            let packet = Packet::Message(chat_message);

            let bytes = bincode::serde::encode_to_vec(&packet, bincode::config::standard())
                .map_err(std::io::Error::other)?;

            stream.write_all(&bytes)?;
        }

        match stream.read(&mut buf) {
            Ok(0) => break,

            Ok(n) => {
                let (packet, _): (Packet, usize) =
                    bincode::serde::decode_from_slice(&buf[..n], bincode::config::standard())
                        .map_err(std::io::Error::other)?;

                match packet {
                    Packet::UserList(users) => {
                        accept_user_list_tx
                            .send(users)
                            .map_err(|_| std::io::Error::other("channel closed"))?;
                    }

                    Packet::Message(chat_message) => {
                        accept_message_tx
                            .send(chat_message)
                            .map_err(|_| std::io::Error::other("channel closed"))?;
                    }
                }
            }

            Err(ref err) if err.kind() == ErrorKind::WouldBlock => {}

            Err(err) => return Err(err),
        }

        thread::sleep(Duration::from_millis(5));
    }

    Ok(())
}

pub fn broadcast_users(server_state: &mut ServerState) {
    let packet = Packet::UserList(server_state.users.clone());

    if let Ok(bytes) = bincode::serde::encode_to_vec(&packet, bincode::config::standard()) {
        for stream in server_state.connections.values_mut() {
            let _ = stream.write_all(&bytes);
        }
    }
}

pub fn broadcast_message(server_state: &mut ServerState, message: ChatMessage) {
    let packet = Packet::Message(message);

    if let Ok(bytes) = bincode::serde::encode_to_vec(&packet, bincode::config::standard()) {
        for stream in server_state.connections.values_mut() {
            let _ = stream.write_all(&bytes);
        }
    }
}
