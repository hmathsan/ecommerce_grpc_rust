use std::net::SocketAddr;

use tonic::transport::Server;
use crate::services::{MyGreeter};

pub struct EcommerceServer {
    addr: SocketAddr
}

impl EcommerceServer {
    pub fn new(addr: String) -> Self {
        let parsed_addr = addr.parse::<SocketAddr>().unwrap();
        EcommerceServer { addr: parsed_addr }
    }

    pub async fn run_server(self) -> Result<(), Box<dyn std::error::Error>> {
        println!("Listening to address: {}\r\n", self.addr);
    
        Server::builder()
            .add_service(MyGreeter::rtn_service())
            .serve(self.addr)
            .await?;
    
        Ok(())
    }
}