use futures::future;
use futures::future::TryFutureExt;
use std::time::Duration;
use tokio::runtime::Runtime;
use tokio::time::delay_for;

mod tunnel;

pub fn start<I, T>(
    host: &str,
    port: u16,
    dests: T,
    debug: bool,
) -> Result<(), Box<dyn std::error::Error>>
where
    I: ToString,
    T: IntoIterator<Item = I>,
{
    Runtime::new()?.block_on(async move {
        dests.into_iter().for_each(|dest| {
            open_tunnel(host.to_string(), port, dest.to_string(), debug);
        });
        future::pending().await
    })
}

fn open_tunnel(host: String, port: u16, dest: String, debug: bool) {
    tokio::spawn(
        tunnel::open(host.clone(), port, dest.clone())
            .inspect_err(move |e| {
                if debug {
                    println!("client tunnel: {}", e);
                }
            })
            .or_else(move |_| async move {
                delay_for(Duration::from_secs(1)).await;
                println!("Reconnecting: {}:{}", host, port);
                open_tunnel(host, port, dest, debug);
                Ok::<(), ()>(())
            }),
    );
}
