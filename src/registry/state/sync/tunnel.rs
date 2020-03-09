use std::collections::HashMap;
use tokio::io::AsyncReadExt;
use tokio::io::AsyncWriteExt;
use tokio::io::BufReader;
use tokio::net::TcpListener;
use tokio::net::TcpStream;
use std::error::Error;

pub async fn sync(peer: TcpStream) -> Result<(), Box<dyn Error>> {
    Ok(())
}
