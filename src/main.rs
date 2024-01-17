use std::net::SocketAddr;

use fastingress::eventloop;
use tokio::net::TcpListener;

const DEFAULT_LISTENING_PORT: u16 = 3000;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let addr = SocketAddr::from(([127, 0, 0, 1], DEFAULT_LISTENING_PORT));
    let listener = TcpListener::bind(addr).await?;

    println!("Listening on http://{addr}");

    // We start a loop to continuously accept incoming connections
    loop {
        eventloop(&listener).await?;
    }
}
