use futures::future::FutureExt;
use tokio::net::TcpListener;
use tokio::net::ToSocketAddrs;
use tokio::runtime::Runtime;

//mod tunnel;

pub fn start<A: ToSocketAddrs, B: ToSocketAddrs>(
    addr: A,
    http_server_addr: B,
    debug: bool,
) -> Result<(), Box<dyn std::error::Error>> {
    Runtime::new()?.block_on(async move {
        let mut l = TcpListener::bind(addr).await?;
        println!("Listening on: {}", l.local_addr().unwrap());

        loop {
            let (s, _) = l.accept().await?;
            // tokio::spawn(tunnel::open(s).map(move |r| {
            //     if debug {
            //         if let Err(e) = r {
            //             println!("server tunnel: {}", e);
            //         }
            //     }
            // }));
        }
    })
}
