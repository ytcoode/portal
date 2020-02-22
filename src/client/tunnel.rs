use tokio::io::AsyncReadExt;
use tokio::io::AsyncWriteExt;
use tokio::io::BufReader;
use tokio::net::TcpStream;

pub async fn open(host: String, port: u16, dest: String) -> Result<(), Box<dyn std::error::Error>> {
    let host = host.as_ref();

    let peer = TcpStream::connect((host, port)).await?;
    let mut peer = BufReader::with_capacity(512, peer);

    let lp1 = peer.read_u16().await?; // outer
    let lp2 = peer.read_u16().await?; // inner
    let key = peer.read_u64().await?;

    println!("forwarding: {}:{} -> {}", host, lp1, dest);

    loop {
        let id = peer.read_u64().await?;
        match TcpStream::connect(&dest).await {
            Ok(s2) => {
                let mut s1 = TcpStream::connect((host, lp2)).await?;
                s1.write_u64(key).await?;
                s1.write_u64(id).await?;
                tokio::spawn(crate::util::proxy(s1, s2));
            }

            Err(_) => {
                // failed to connect to dest
                peer.write_u64(key).await?;
                peer.write_u64(id).await?;
            }
        }
    }
}
