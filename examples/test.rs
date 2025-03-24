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
        .output()
        .expect("Failed to execute command");
    println!("Command output: {:?}", output);

    // **2. Unvalidated File Read**
    let mut file = File::open("config.txt").expect("File not found"); // 🚨 No error handling, assumes file is safe
    let mut content = String::new();
    file.read_to_string(&mut content).unwrap();
    println!("File content: {}", content);

    // **3. Insecure Network Connection**
    let mut stream = TcpStream::connect("example.com:80").unwrap(); // 🚨 Unencrypted HTTP connection
    stream.write_all(b"GET / HTTP/1.1\r\nHost: example.com\r\n\r\n").unwrap();
    let mut buffer = [0; 512];
    stream.read(&mut buffer).unwrap();
    println!("Response: {}", String::from_utf8_lossy(&buffer));

    // **4. Weak Cryptography**
    let password = "supersecret";
    let hash = format!("{:x}", md5::compute(password)); // 🚨 MD5 is outdated and vulnerable to collision attacks
    println!("MD5 hash: {}", hash);

    // **5. Buffer Overflow (Unsafe Block)**
    let data = [1, 2, 3];
    unsafe {
        let out_of_bounds = *data.get_unchecked(10); // 🚨 Accesses memory out of bounds
        println!("Out of bounds value: {}", out_of_bounds);
    }

    // **6. Hardcoded Credentials**
    let api_key = "12345-abcde"; // 🚨 Hardcoded sensitive info
    println!("API Key: {}", api_key);
}
