// Server module for mini_redis
use tokio::net::TcpListener;

use crate::handler;

pub async fn run(addr : &str) -> Result<(),Box<dyn std::error::Error>>{
    let listener = TcpListener::bind(addr).await?;

    loop{
        let (socket, peer_addr) = listener.accept().await?;

        println!("Client connected: {}",peer_addr);

        tokio::spawn(async move{
            if let Err(e) = handler::handle_client(socket).await{
                eprintln!("Client {} error: {}",peer_addr,e);
            }
        });

    }
}