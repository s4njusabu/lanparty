use std::{fs, process::Command};

pub fn ip_command_exists() -> bool {
    Command::new("ip")
        .arg("-V")
        .output()
        .is_ok_and(|output| output.status.success())
}

pub fn get_os_id() -> std::io::Result<String> {
    let output = fs::read_to_string("/etc/os-release")?;

    for line in output.lines() {
        if let Some(id) = line.strip_prefix("ID=") {
            return Ok(id.trim_matches('"').to_string());
        }
    }

    Ok(String::from("NOT FOUND"))
}

pub fn command_to_install_ip() -> Option<String> {
    if let Ok(os) = get_os_id() {
        match os.as_str() {
            "arch" => Some("pacman -S --needed iproute2".to_string()),
            "debian" | "ubuntu" => Some("apt install iproute2".to_string()),
            "fedora" => Some("dnf install iproute".to_string()),
            _ => None,
        }
    } else {
        None
    }
}

pub fn get_network_interface_and_user_ip() -> std::io::Result<(String, String)> {
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

        return Ok((interface, user_ip));
    }

    Err(std::io::Error::other(
        "Failed to get network interface and user IP",
    ))
}

pub fn get_broadcast_addr(interface: &str) -> std::io::Result<String> {
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
                        if w2 == "brd"
                            && let Some(broadcast) = words.next()
                        {
                            return Ok(broadcast.to_string());
                        }
                    }
                }
            }
        }
    }

    Err(std::io::Error::other("Failed to get broadcast address"))
}

pub fn get_local_ip() -> Option<String> {
    if let Ok(output) = Command::new("ip")
        .args(["route", "get", "8.8.8.8"])
        .output()
    {
        let text = String::from_utf8_lossy(&output.stdout);
        let mut words = text.split_whitespace();
        while let Some(word) = words.next() {
            if word == "src" {
                let ip = words.next();
                return ip.map(|ip| ip.to_string());
            }
        }
    }
    None
}
