use serde::{Deserialize, Serialize};
use std::fs;
use std::thread;
use std::time::Duration;
extern crate get_if_addrs;

use get_if_addrs::{get_if_addrs, IfAddr};
use reqwest::StatusCode; // 加上StatusCode来迎合服务端,但解决不了一点

#[derive(Debug, Serialize, Deserialize)]
struct Config {
    server_host: String,
    server_port: u16,
    program_simple_name: String,
    version: String,
    coredir: String,
    machine_id: Option<String>,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let define_version = "Beta 1.12.113";
    let file_content = fs::read_to_string("config.yml")?;

    let config: Config = serde_yaml::from_str(&file_content)?;

    println!("DoConnectTools {}", define_version);
    println!("Server Host: {}", config.server_host);
    println!("Server Port: {}", config.server_port);
    println!("Program Simple Name: {}", config.program_simple_name);
    println!("Version: {}", config.version);
    println!("DoConnect core directory: {}", config.coredir);
    if let Some(machine_id) = config.machine_id {
        println!("Machine ID: {}", machine_id);
    } else {
        println!("Machine ID is not specified.");
    }
    let interfaces = get_if_addrs()?;

    for interface in interfaces {
        println!("Interface: {}", interface.name);
        match interface.addr {
            IfAddr::V4(ip) => println!("IPV4: {}", ip.ip),
            IfAddr::V6(ip) => println!("IPV6: {}", ip.ip),
        }
    }
    let server_url = format!("http://{}:{}/online", config.server_host, config.server_port);
    let response = reqwest::get(&server_url).await?;
    if response.status() == StatusCode::NO_CONTENT {
        println!("This Client is online");
    } else {
        println!("Client is not online or returned a different status code: {},DoConnect will run in offline mode", response.status());
    }
    println!("");
    tokio::time::sleep(Duration::from_millis(5000)).await;

    Ok(())
}
