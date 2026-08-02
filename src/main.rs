use std::process::Command;

fn main() {
    if ip_command_exists() {
        if let Some(mut v1) = get_user_ip_and_network_interface() {
            let userip = v1.pop().unwrap();
            if let Some(v2) = get_broadcast_addr(&v1.pop().unwrap()) {
                let broadcast = v2;
                println!("{} {}", userip, broadcast);
            }
        }
    }
}

// Keep
fn ip_command_exists() -> bool {
    Command::new("ip")
        .arg("-V")
        .output()
        .is_ok_and(|output| output.status.success())
}

// The logic should be
// If the above function returns true, run the function below
// else just return a string that says "dependencies not installed" or something like that

// Keep 
// Interface first
// User ip second
fn get_user_ip_and_network_interface() -> Option<Vec<String>> {
    if let Ok(output) = Command::new("ip")
        .args(["route", "get", "8.8.8.8"])
        .output()
    {
        let mut ip_and_interface: Vec<String> = Vec::new();

        let text = String::from_utf8_lossy(&output.stdout);
        let mut words = text.split_whitespace();
        while let Some(word) = words.next() {
            if word == "dev"
                && let Some(t1) = words.next()
            {
                ip_and_interface.push(t1.to_string());
            } else if word == "src"
                && let Some(t2) = words.next()
            {
                ip_and_interface.push(t2.to_string());
            }
        }

        return Some(ip_and_interface);
    }

    None
}

// Keep
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