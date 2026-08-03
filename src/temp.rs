use std::process::Command;

fn main() {
    if ip_command_exists() {
        if let Some((interface, user_ip)) = get_user_ip_and_network_interface()
            && let Some(broadcast) = get_broadcast_addr(&interface)
        {
            println!(
                "interface: {}\nuser ip: {}\nbroadcast addr: {}",
                interface, user_ip, broadcast
            );
        }
    }
}

// Keep
pub fn ip_command_exists() -> bool {
    Command::new("ip")
        .arg("-V")
        .output()
        .is_ok_and(|output| output.status.success())
}

// The logic should be
// If the above function returns true, run the function below
// else just return a string that says "dependencies not installed" or something like that

// Keep
pub fn get_user_ip_and_network_interface() -> Option<(String, String)> {
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

// Keep
pub fn get_broadcast_addr(interface: &str) -> Option<String> {
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
