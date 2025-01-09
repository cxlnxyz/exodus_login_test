use std::process::Command;
use std::io::{self, Write};

fn main() {
    let mut username = String::new();
    let mut password = String::new();
    let mut list_users = String::new();

    print!("Enter username: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut username).unwrap();
    let username = username.trim();

    print!("Enter password: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut password).unwrap();
    let password = password.trim();

    print!("List all users? (yes/no): ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut list_users).unwrap();
    let list_users = list_users.trim().to_lowercase();

    let mut command = Command::new("powershell");
    command.arg("-File").arg("check_ad_login.ps1").arg(username).arg(password);

    if list_users == "yes" {
        command.arg("-listUsers");
    }

    let output = command.output().expect("Failed to execute PowerShell script");

    if output.status.success() {
        println!("{}", String::from_utf8_lossy(&output.stdout));
    } else {
        println!("Operation failed");
    }
}