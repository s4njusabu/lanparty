use std::{
    net::UdpSocket,
    thread,
    time::Duration,
};

use crate::temp::{get_broadcast_addr, get_user_ip_and_network_interface};

mod temp;

fn main() {
    if temp::ip_command_exists() {
        if let Some((interface, user_ip)) = get_user_ip_and_network_interface() {
            if let Some(broadcast) = get_broadcast_addr(&interface) {
                if let Ok(socket) = UdpSocket::bind(format!("{user_ip}:55555")) {
                    if let Ok(_) = socket.set_broadcast(true) {
                        loop {
                            thread::sleep(Duration::from_secs(1));
                            if let Ok(_) = socket.send_to(
                                format!("{user_ip}:55555").as_bytes(),
                                format!("{broadcast}:55555"),
                            ) {}
                        }
                    }
                }
            }
        }
    }
}