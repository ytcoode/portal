use tokio::io;
use tokio::io::AsyncReadExt;
use tokio::io::AsyncWriteExt;
use tokio::net::TcpStream;
use tokio::try_join;

pub mod args;

pub async fn proxy(mut s1: TcpStream, mut s2: TcpStream) -> io::Result<(u64, u64)> {
    let (mut r1, mut w1) = s1.split();
    let (mut r2, mut w2) = s2.split();

    let c1 = copy(&mut r1, &mut w2);
    let c2 = copy(&mut r2, &mut w1);

    try_join!(c1, c2)
}

pub async fn copy<R, W>(reader: &mut R, writer: &mut W) -> io::Result<u64>
where
    R: AsyncReadExt + Unpin,
    W: AsyncWriteExt + Unpin,
{
    let mut a = 0;
    let mut b = [0 as u8; 2048];

    loop {
        let n = reader.read(&mut b).await?;
        if n == 0 {
            writer.shutdown().await?;
            return Ok(a);
        }
        writer.write_all(&b[0..n]).await?;
        a += n as u64;
    }
}
