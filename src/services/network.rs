use std::{
    io::Read,
    net::{IpAddr, TcpListener, TcpStream, UdpSocket},
    thread,
    time::Duration,
};

use crate::{
    services::system::{self, get_local_ip},
    states::{group_chat_state::Message, ui_state::UiState},
};

const DISCOVERY_PACKET: &str = "LANPARTY";
const DISCOVERY_PORT: u16 = 55555;

// Discovery 1 (UDP)
pub fn send_udp_packets_to_broadcast(username: &str) -> std::io::Result<()> {
    let (interface, user_ip) = system::get_network_interface_and_user_ip()?;
    let broadcast = system::get_broadcast_addr(&interface)?;

    let socket = UdpSocket::bind(format!("{user_ip}:0"))?;

    socket.set_broadcast(true)?;

    let destination = format!("{broadcast}:{DISCOVERY_PORT}");

    let packet = format!("{} {}", username, DISCOVERY_PACKET);

    loop {
        thread::sleep(Duration::from_millis(250));

        socket.send_to(packet.as_bytes(), &destination)?;
    }
}

// Discovery 2 (UDP)
pub fn receive_udp_packets_from_broadcast() -> std::io::Result<(IpAddr, String)> {
    let socket = UdpSocket::bind(format!("0.0.0.0:{DISCOVERY_PORT}"))?;
    let mut buf = [0u8; 1024];

    loop {
        let (n, sender) = socket.recv_from(&mut buf)?;

        let text = String::from_utf8_lossy(&buf[..n]);
        let mut words = text.split_whitespace();
        let username = words.next();
        let packet = words.next();

        if let Some(packet) = packet
            && packet == DISCOVERY_PACKET
            && let Some(username) = username
        {
            return Ok((sender.ip(), username.to_string()));
        }
    }
}

// Create connection
pub fn create_connection(ui_state: &UiState) -> std::io::Result<()> {
    let listener = TcpListener::bind(format!("{}:{}", ui_state.local_ip, DISCOVERY_PORT))?;

    loop {
        let (mut stream, addr) = listener.accept()?;
        thread::spawn(move || {
            let mut buf = [0u8; 1024];

            loop {
                match stream.read(&mut buf) {
                    Ok(0) => break,
                    Ok(_) => {}
                    Err(_) => break,
                }
            }
        });
    }
}

// Accept
pub fn accept_connections(ip: IpAddr) -> std::io::Result<()> {
    let destination = format!("{}:{}", ip, DISCOVERY_PORT);
    let listener = TcpStream::connect(destination)?;

    Ok(())
}
