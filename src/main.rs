use std::env;

use server::EcommerceServer;

mod server;
mod services;
mod repositories;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Starting server");

    let default_addr = "127.0.0.1";
    let default_port = "50051";

    println!("Checking address variable.");

    let server_ip = env::var("SERVER_IP_ADDR").unwrap_or(String::from(default_addr));
    let server_port = env::var("SERVER_PORT").unwrap_or(String::from(default_port));
    let addr = format!("{}:{}", server_ip, server_port);

    println!("Starting server on address {}", addr);

    let server = EcommerceServer::new(addr);
    server.run_server().await?;

    Ok(())
}
