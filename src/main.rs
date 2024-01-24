use std::env;
use std::error::Error;
use std::net::{Ipv4Addr, SocketAddr};

use fastingress::accept_connection;
use fastingress::constants::DEFAULT_LISTENING_PORT;
use tokio::net::TcpListener;

fn resolve_ip() -> Ipv4Addr {
    let hostout = env::var("HOST_OUT").ok();

    if hostout.is_some() && hostout.unwrap().eq("1") {
        println!("INFO: Detected HOST_OUT=1 in environment");

        return Ipv4Addr::UNSPECIFIED;
    }
    return Ipv4Addr::LOCALHOST;
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error + Send + Sync>> {
    let address = SocketAddr::from((resolve_ip(), DEFAULT_LISTENING_PORT));
    let listener = TcpListener::bind(address).await?;

    println!("INFO: Listening on http://{address}");
    loop {
        accept_connection(&listener).await
    }
}
