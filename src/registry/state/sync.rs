use crate::state::Global;
use futures::future::FutureExt;
use std::sync::Arc;
use std::sync::Mutex;
use tokio::io;
use tokio::net::TcpListener;
use tokio::net::ToSocketAddrs;
use tokio::runtime::Runtime;
use tokio::task::JoinHandle;

mod tunnel;

pub fn sync(global: Arc<Mutex<Global>>) -> io::Result<JoinHandle<io::Result<()>>> {
    Ok(Runtime::new()?.spawn(async move {
        let mut l = TcpListener::bind("0.0.0.0:65534").await?;
        println!("Syncing on: {}", l.local_addr().unwrap());

        loop {
            println!("accept");
            let (s, _) = l.accept().await?;
            tokio::spawn(tunnel::sync(s).map(move |r| {
                if let Err(e) = r {
                    println!("server tunnel: {}", e);
                }
            }));
        }
    }))
}
