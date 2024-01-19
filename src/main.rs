use std::error::Error;
use std::net::SocketAddr;

use fastingress::accept_connection;
use fastingress::constants::DEFAULT_LISTENING_PORT;
use tokio::net::TcpListener;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error + Send + Sync>> {
    let address = SocketAddr::from(([127, 0, 0, 1], DEFAULT_LISTENING_PORT));
    let listener = TcpListener::bind(address).await?;

    println!("Listening on http://{address}");
    loop {
        accept_connection(&listener).await
    }
}
