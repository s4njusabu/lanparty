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
pub fn create_connection(
    host_ip: IpAddr,
    from_client_tx: Sender<Packet>,
    to_clients_rx: Receiver<Packet>,
) -> std::io::Result<()> {
    let listener = TcpListener::bind(format!("{}:{}", host_ip, DISCOVERY_PORT))?;
    listener.set_nonblocking(true)?;

    let mut clients: Vec<TcpStream> = Vec::new();

    loop {
        // Accept new clients
        match listener.accept() {
            Ok((mut stream, addr)) => {
                let tx = from_client_tx.clone();

                let write_stream = stream.try_clone()?;
                clients.push(write_stream);

                thread::spawn(move || {
                    let mut buf = [0u8; 1024];

                    let username = match stream.read(&mut buf) {
                        Ok(0) => return,
                        Ok(n) => String::from_utf8_lossy(&buf[..n]).to_string(),
                        Err(_) => return,
                    };

                    if tx
                        .send(Packet::UserConnected {
                            ip: addr.ip(),
                            username,
                        })
                        .is_err()
                    {
                        return;
                    }

                    loop {
                        match stream.read(&mut buf) {
                            Ok(0) => {
                                let _ = tx.send(Packet::UserDisconnected(addr.ip()));
                                break;
                            }

                            Ok(n) => {
                                let (packet, _) = match bincode::serde::decode_from_slice(
                                    &buf[..n],
                                    bincode::config::standard(),
                                ) {
                                    Ok(packet) => packet,
                                    Err(_) => continue,
                                };

                                if let Packet::Message(message) = packet
                                    && tx.send(Packet::Message(message)).is_err()
                                {
                                    break;
                                }
                            }

                            Err(_) => {
                                let _ = tx.send(Packet::UserDisconnected(addr.ip()));
                                break;
                            }
                        }
                    }
                });
            }

            Err(ref err) if err.kind() == std::io::ErrorKind::WouldBlock => {}

            Err(err) => return Err(err),
        }

        // Host to clients
        while let Ok(packet) = to_clients_rx.try_recv() {
            let bytes = bincode::serde::encode_to_vec(&packet, bincode::config::standard())
                .map_err(std::io::Error::other)?;

            for stream in &mut clients {
                let _ = stream.write_all(&bytes);
            }
        }

        thread::sleep(Duration::from_millis(20));
    }
}

// Accept
pub fn accept_connections(
    ip: IpAddr,
    username: String,
    from_clients_tx: Sender<Packet>,
    to_host_rx: Receiver<String>,
    error_tx: Sender<std::io::Error>,
) -> std::io::Result<()> {
    let destination = format!("{}:{}", ip, DISCOVERY_PORT);
    let mut stream = TcpStream::connect(destination)?;

    stream.set_nonblocking(true)?;

    // Username
    stream.write_all(username.as_bytes())?;

    let mut buf = [0u8; 4096];

    loop {
        // Client to Host
        if let Ok(message) = to_host_rx.try_recv() {
            let packet = Packet::Message(Message {
                sender: system::get_local_ip()
                    .ok_or(std::io::Error::other("Failed to get local IP"))?,
                message,
            });

            let bytes = bincode::serde::encode_to_vec(&packet, bincode::config::standard())
                .map_err(std::io::Error::other)?;

            stream.write_all(&bytes)?;
        }

        // Host to Client
        match stream.read(&mut buf) {
            Ok(0) => {
                let _ = error_tx.send(std::io::Error::from(std::io::ErrorKind::ConnectionReset));
                break;
            }

            Ok(n) => {
                let (packet, _): (Packet, usize) =
                    bincode::serde::decode_from_slice(&buf[..n], bincode::config::standard())
                        .map_err(std::io::Error::other)?;

                match packet {
                    Packet::UserList(users) => {
                        from_clients_tx
                            .send(Packet::UserList(users))
                            .map_err(|_| std::io::Error::other("channel closed"))?;
                    }

                    Packet::Message(message) => {
                        from_clients_tx
                            .send(Packet::Message(message))
                            .map_err(|_| std::io::Error::other("channel closed"))?;
                    }

                    Packet::UserConnected { .. } | Packet::UserDisconnected(_) => {}
                }
            }

            Err(ref err) if err.kind() == std::io::ErrorKind::WouldBlock => {}

            Err(err) => return Err(err),
        }

        thread::sleep(Duration::from_millis(5));
    }

    Ok(())
}
