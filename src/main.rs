use std::process::Command;

fn main() {
    if ip_command_exists() {
        println!("Success");
    } else {
        println!("Error");
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
