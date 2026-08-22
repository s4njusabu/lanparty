use std::{
    net::{IpAddr, UdpSocket},
    thread,
    time::Duration,
};

use crate::services::system;

const DISCOVERY_PACKET: &str = "LANPARTY";
const DISCOVERY_PORT: u16 = 55555;

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
