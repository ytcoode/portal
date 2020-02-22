#![feature(option_unwrap_none)]

mod cli;
mod client;
mod server;
mod util;

fn main() {
    cli::start();
}
