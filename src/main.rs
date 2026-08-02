use std::process::Command;

fn main() {
    if ip_command_exists() {
        if let Some(ip) = get_user_ip_addr() {
            println!("{ip}");
        } else {
            println!("Failed to find IP address");
        }
    } else {
        println!("The \"ip\" command is not installed");
    }

    println!("{:?}", get_interface());
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
fn get_user_ip_addr() -> Option<String> {
    if let Ok(output) = Command::new("ip")
        .args(["route", "get", "8.8.8.8"])
        .output()
    {
        let text = String::from_utf8_lossy(&output.stdout);
        let mut words = text.split_whitespace();
        while let Some(word) = words.next() {
            if word == "src" {
                return words.next().map(|ip| ip.to_string());
            }
        }
    }

    None
}

fn get_interface() -> Option<String> {
    if let Ok(output) = Command::new("ip")
        .args(["route", "get", "8.8.8.8"])
        .output()
    {
        let text = String::from_utf8_lossy(&output.stdout);
        let mut words = text.split_whitespace();
        while let Some(word) = words.next() {
            if word == "dev" {
                return words.next().map(|interface| interface.to_string());
            }
        }
    }
    None
}
