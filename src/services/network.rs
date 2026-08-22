use std::{net::UdpSocket, thread, time::Duration};

use crate::services::system;

const DISCOVERY_PACKET: &[u8] = b"LANPARTY";
const DISCOVERY_PORT: u16 = 55555;

pub fn send_udp_packets_to_broadcast_prototype() -> std::io::Result<()> {
    let (interface, user_ip) = system::get_network_interface_and_user_ip()?;
    let broadcast = system::get_broadcast_addr(&interface)?;

    let socket = UdpSocket::bind(format!("{user_ip}:0"))?;

    socket.set_broadcast(true)?;

    let destination = format!("{broadcast}:{DISCOVERY_PORT}");

    loop {
        thread::sleep(Duration::from_millis(250));

        socket.send_to(DISCOVERY_PACKET, &destination)?;
    }
}

pub fn send_udp_packets_to_broadcast() -> std::io::Result<()> {
    let (interface, user_ip) = system::get_network_interface_and_user_ip()?;
    Ok(())
}
