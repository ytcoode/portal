#![feature(option_unwrap_none)]
#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;

mod cli;
mod client;
mod registry;
mod server;
mod util;

fn main() {
    cli::start();
}
