use tokio::io::AsyncReadExt;
use tokio::io::AsyncWriteExt;
use tokio::io::BufReader;
use tokio::net::TcpStream;

pub async fn open(host: String, port: u16, dest: String) -> Result<(), Box<dyn std::error::Error>> {
    let host = host.as_ref();
    let (rp, dest) = parse_dest(dest);
    let dest = &dest;

    let peer = TcpStream::connect((host, port)).await?;
    let mut peer = BufReader::with_capacity(512, peer);

    peer.write_u16(rp).await?; // remote port

    let lp1 = peer.read_u16().await?; // outer
    let lp2 = peer.read_u16().await?; // inner
    let key = peer.read_u64().await?;

    println!("Forwarding: {}:{} -> {}", host, lp1, dest);

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

// [remote_port:][local_host:]local_port
fn parse_dest(s: String) -> (u16, String) {
    let v = s.split(":").collect::<Vec<&str>>();
    match v.len() {
        1 => (0, "127.0.0.1:".to_string() + &s),
        2 => match v[0].parse::<u16>() {
            Ok(p) => (p, "127.0.0.1:".to_string() + v[1]),
            Err(_) => (0, s),
        },
        3 => (v[0].parse::<u16>().unwrap(), v[1].to_string() + v[2]),
        _ => unreachable!(),
    }
}
