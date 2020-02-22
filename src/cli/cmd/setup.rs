use super::Cmd;
use crate::server;
use clap::Arg;
use clap::ArgMatches;
use std::net::SocketAddr;

// portal setup [[ip:]port]
pub struct Setup;

impl Cmd for Setup {
    fn name(&self) -> &str {
        "setup"
    }

    fn about(&self) -> &str {
        "Sets up a portal server"
    }

    fn args(&self) -> Vec<Arg<'_>> {
        vec![Arg::with_name("addr")
            .value_name("[host:]port")
            .default_value("65535")
            .help("Specifies the address to bind to")
            .validator(|s| {
                if s.contains(":") {
                    s.parse::<SocketAddr>()
                        .map_or_else(|e| Err(e.to_string()), |_| Ok(()))
                } else {
                    s.parse::<u16>()
                        .map_or_else(|e| Err(e.to_string()), |_| Ok(()))
                }
            })]
    }

    fn run(&self, matches: &ArgMatches, debug: bool) -> Result<(), Box<dyn std::error::Error>> {
        let addr = matches.value_of("addr").unwrap();
        if addr.contains(":") {
            server::start(addr, debug)
        } else {
            server::start(("0.0.0.0", addr.parse::<u16>().unwrap()), debug)
        }
    }
}
