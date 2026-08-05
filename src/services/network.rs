use std::{
    collections::HashMap,
    io::{ErrorKind, Read, Write},
    net::{IpAddr, TcpListener, TcpStream, UdpSocket},
    process::Command,
    sync::mpsc::Sender,
    thread,
    time::Duration,
};

use crate::app::server_state::{Message, ServerState, User};

const DISCOVERY_PACKET: &[u8] = b"LANPARTY";
const DISCOVERY_PORT: u16 = 55555;

pub enum GetClientConnection {
    ClientConnected(IpAddr, TcpStream, String),
    ClientDisconnected(IpAddr),
    Error(ErrorKind),
}

pub enum MessageFlow {
    SendToServer(String),
    GetFromServer(Vec<Message>),
}

fn ip_command_exists() -> bool {
    Command::new("ip")
        .arg("-V")
        .output()
        .is_ok_and(|output| output.status.success())
}

fn get_network_interface_and_user_ip() -> Option<(String, String)> {
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
            while let Some(word) = words.next() {
                if word == "inet" {
                    words.next();
                    if words.next() == Some("brd") {
                        return words.next().map(str::to_string);
                    }
                }
            }
        }
    }

    None
}

pub fn send_udp_packets_to_broadcast() -> Option<()> {
    if !ip_command_exists() {
        return None;
    }

    let (interface, user_ip) = get_network_interface_and_user_ip()?;
    let broadcast = get_broadcast_addr(&interface)?;

    let socket = UdpSocket::bind(format!("{user_ip}:0")).ok()?;
    socket.set_broadcast(true).ok()?;

    let destination = format!("{broadcast}:{DISCOVERY_PORT}");

    loop {
        thread::sleep(Duration::from_secs(1));

        socket.send_to(DISCOVERY_PACKET, &destination).ok()?;
    }
}

pub fn receive_udp_packets_from_broadcast() -> Option<IpAddr> {
    if !ip_command_exists() {
        return None;
    }

    let socket = UdpSocket::bind(format!("0.0.0.0:{DISCOVERY_PORT}")).ok()?;
    let mut buf = [0u8; 1024];

    loop {
        let (n, sender) = socket.recv_from(&mut buf).ok()?;

        if &buf[..n] != DISCOVERY_PACKET {
            continue;
        }

        return Some(sender.ip());
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
            let mut buf = [0u8; 1024];

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
                    .map_err(|_| std::io::Error::other("Something went wrong"))?;
            }
            loop {
                let n = stream.read(&mut buf)?;
                if n == 0 {
                    accept_conn_tx_clone
                        .send(GetClientConnection::ClientDisconnected(addr.ip()))
                        .map_err(|_| std::io::Error::other("Something went wrong"))?;
                    break;
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
) -> std::io::Result<()> {
    if let Some(server_ip) = receive_udp_packets_from_broadcast()
        && let Ok(mut stream) = TcpStream::connect((server_ip, 55555))
    {
        stream.write_all(username.as_bytes())?;

        let mut buf = [0u8; 4096];

        loop {
            match stream.read(&mut buf) {
                Ok(0) => break,

                Ok(n) => {
                    let (users, _): (HashMap<IpAddr, User>, usize) =
                        bincode::serde::decode_from_slice(&buf[..n], bincode::config::standard())
                            .map_err(std::io::Error::other)?;

                    accept_user_list_tx
                        .send(users)
                        .map_err(|_| std::io::Error::other("channel closed"))?;
                }

                Err(err) => return Err(err),
            }
        }
    }

    Ok(())
}

// broadcast_messages(&mut server_state) {

// }

pub fn broadcast_users(server_state: &mut ServerState) {
    if let Ok(bytes) =
        bincode::serde::encode_to_vec(&server_state.users, bincode::config::standard())
    {
        for stream in server_state.connections.values_mut() {
            let _ = stream.write_all(&bytes);
        }
    }
}
