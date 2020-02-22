use rand;
use std::collections::HashMap;
use tokio::io::AsyncReadExt;
use tokio::io::AsyncWriteExt;
use tokio::io::BufReader;
use tokio::net::TcpListener;
use tokio::net::TcpStream;

pub async fn open(peer: TcpStream) -> Result<(), Box<dyn std::error::Error>> {
    let mut peer = BufReader::with_capacity(512, peer);
    let mut l1 = TcpListener::bind("0.0.0.0:0").await?; // outer
    let mut l2 = TcpListener::bind("0.0.0.0:0").await?; // inner

    peer.write_u16(l1.local_addr()?.port()).await?;
    peer.write_u16(l2.local_addr()?.port()).await?;

    let key = rand::random();
    peer.write_u64(key).await?;

    let mut map = HashMap::new();
    let mut id = rand::random();

    loop {
        tokio::select! {
            r = l1.accept() => { // outer
                let (s, _) = r?;
                map.insert(id, s).unwrap_none();
                peer.write_u64(id).await?;
                id += 1;
            }

            r = l2.accept() => { // inner
                let (mut s2, _) = r?;
                if s2.read_u64().await? == key { // unsafe, create a new task?
                    let id = s2.read_u64().await?;
                    if let Some(s1) = map.remove(&id) {
                        tokio::spawn(crate::util::proxy(s1, s2));
                    }
                }
            }

            r = peer.read_u64() => { // client failed to connect to dest
                if r? != key {
                    return Err("illegal key".into());
                }
                let id = peer.read_u64().await?;
                map.remove(&id);
            }

        }
    }
}
