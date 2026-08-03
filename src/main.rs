fn main() {
    send_udp_packets_to_broadcast();
}

use std::{
    net::{IpAddr, TcpListener, TcpStream, UdpSocket},
    process::Command,
    thread,
    time::Duration,
};

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
    let destination = format!("{broadcast}:55555");
    loop {
        thread::sleep(Duration::from_secs(1));

        socket.send_to(b"LANPARTY", &destination).ok()?;
    }
}

pub fn receive_udp_packets_from_broadcast() -> Option<IpAddr> {
    if !ip_command_exists() {
        return None;
    }

    let socket = UdpSocket::bind("0.0.0.0:55555").ok()?;
    let mut buf = [0u8; 1024];

    loop {
        let (n, sender) = socket.recv_from(&mut buf).ok()?;

        if &buf[..n] != b"LANPARTY" {
            continue;
        }

        return Some(sender.ip());
    }
}

// everything above goes to the stable src (except the main function of course)

struct Client {
    ip: IpAddr,
    stream: TcpStream,
}

fn create_server() {
    if let Some((_, user_ip)) = get_network_interface_and_user_ip()
        && let Ok(listener) = TcpListener::bind(format!("{user_ip}:55555"))
    {}
}

fn connect_to_server() {
    if let Some(server_ip) = receive_udp_packets_from_broadcast()
        && let Ok(stream) = TcpStream::connect((server_ip, 55555))
    {}
}
