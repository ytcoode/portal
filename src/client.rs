use futures::future;
use futures::future::FutureExt;
use tokio::runtime::Runtime;

mod tunnel;

pub fn start(
    host: &str,
    port: u16,
    dests: Vec<String>,
    debug: bool,
) -> Result<(), Box<dyn std::error::Error>> {
    Runtime::new()?.block_on(async move {
        let a = dests.into_iter().map(|dest| {
            tokio::spawn(tunnel::open(host.to_string(), port, dest).map(move |r| {
                if debug {
                    if let Err(e) = r {
                        println!("client tunnel: {}", e);
                    }
                }
            }))
        });
        future::join_all(a).await;
        Ok(())
    })
}
