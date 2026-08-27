use std::{
    io::{Read, Write},
    net::{IpAddr, TcpListener, TcpStream, UdpSocket},
    sync::mpsc::{Receiver, Sender},
    thread,
    time::Duration,
};

use crate::{
    services::system,
    states::group_chat_state::{Message, Packet},
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
pub fn create_connection(host_ip: IpAddr, from_client_tx: Sender<Packet>) -> std::io::Result<()> {
    let listener = TcpListener::bind(format!("{}:{}", host_ip, DISCOVERY_PORT))?;

    loop {
        let (mut stream, addr) = listener.accept()?;
        let tx = from_client_tx.clone();

        thread::spawn(move || {
            let mut buf = [0u8; 1024];
            let username = match stream.read(&mut buf) {
                Ok(0) => return,
                Ok(n) => String::from_utf8_lossy(&buf[..n]).to_string(),
                Err(_) => return,
            };

            if tx
                .send(Packet::User {
                    ip: addr.ip(),
                    username: username,
                })
                .is_err()
            {
                return;
            }

            loop {
                match stream.read(&mut buf) {
                    Ok(0) => break,
                    Ok(n) => {
                        if tx
                            .send(Packet::Message(Message {
                                sender: addr.ip(),
                                message: String::from_utf8_lossy(&buf[..n]).to_string(),
                            }))
                            .is_err()
                        {
                            return;
                        }
                    }
                    Err(_) => break,
                }
            }
        });
    }
}

// Accept
pub fn accept_connections(
    ip: IpAddr,
    username: String,
    from_clients_tx: Sender<Packet>,
    to_server_rx: Receiver<String>,
) -> std::io::Result<()> {
    let destination = format!("{}:{}", ip, DISCOVERY_PORT);
    let mut stream = TcpStream::connect(destination)?;
    stream.write_all(username.as_bytes())?;

    let mut read_stream = stream.try_clone()?;
    let mut write_stream = stream;

    thread::spawn(move || {
        let mut buf = [0u8; 1024];
        loop {
            match read_stream.read(&mut buf) {
                Ok(0) => break,
                Ok(n) => {
                    if from_clients_tx
                        .send(Packet::Message(Message {
                            sender: ip,
                            message: String::from_utf8_lossy(&buf[..n]).to_string(),
                        }))
                        .is_err()
                    {
                        break;
                    }
                }
                Err(_) => break,
            }
        }
    });

    thread::spawn(move || {
        while let Ok(text) = to_server_rx.recv() {
            if write_stream.write_all(text.as_bytes()).is_err() {
                break;
            }
        }
    });

    Ok(())
}
