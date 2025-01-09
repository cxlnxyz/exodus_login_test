use std::process::Command;
use std::io::{self, Write};

fn main() {
    let mut username = String::new();
    let mut password = String::new();

    print!("Enter username: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut username).unwrap();
    let username = username.trim();

    print!("Enter password: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut password).unwrap();
    let password = password.trim();

    let output = Command::new("powershell")
        .arg("-File")
        .arg("check_ad_login.ps1")
        .arg(username)
        .arg(password)
        .output()
        .expect("Failed to execute PowerShell script");

    if output.status.success() {
        println!("Login successful");
    } else {
        println!("Login failed");
    }
}