use std::error::Error as StdError;

use hyper::{body::Body, client::conn::http1::SendRequest, Uri};
use hyper_util::rt::TokioIo;
use tokio::net::TcpStream;

use hyper::client::conn::http1::handshake;

pub async fn handshake_url<T>(uri: &Uri) -> SendRequest<T>
where
    T: Body + 'static + std::marker::Send,
    T::Data: Send,
    T::Error: Into<Box<dyn StdError + Send + Sync>>,
{
    // Get the host and the port
    let host = uri.host().expect("uri has no host");
    let port = uri.port_u16().unwrap_or(80);

    let address = format!("{}:{}", host, port);

    // Open a TCP connection to the remote host
    let stream = TcpStream::connect(address).await.unwrap();

    // Use an adapter to access something implementing `tokio::io` traits as if they implement
    // `hyper::rt` IO traits.
    let io = TokioIo::new(stream);

    // Perform a TCP handshake
    let (sender, connection) = handshake::<TokioIo<TcpStream>, T>(io).await.unwrap();

    tokio::task::spawn(async move {
        if let Err(err) = connection.await {
            println!("Connection failed: {:?}", err);
        }
    });

    return sender;
}
