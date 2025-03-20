use std::env;
use std::fs::File;
use std::io::{self, Read, Write};
use std::net::TcpStream;
use std::process::Command;

fn main() {
    // **1. Command Injection**
    let user_input = env::args().nth(1).unwrap_or("ls".to_string());
    let output = Command::new("sh")
        .arg("-c")
        .arg(user_input) // 🚨 Unsafe! Allows arbitrary command execution
    
