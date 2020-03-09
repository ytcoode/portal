use super::Cmd;
use crate::server;
use crate::util::args;
use clap::Arg;
use clap::ArgMatches;

// portal server [[ip:]port]
pub struct Server;

impl Cmd for Server {
    fn name(&self) -> &str {
        "server"
    }

    fn about(&self) -> &str {
        "Sets up a portal server"
    }

    fn args(&self) -> Vec<Arg<'_>> {
        vec![
            Arg::with_name("addr")
                .value_name("[host:]port")
                .default_value("65535")
                .help("Specifies the address to bind to")
                .validator(|s| {
                    if s.contains(':') {
                        args::validate_socket_addr(&s)
                    } else {
                        args::validate_port(&s)
                    }
                }),
            Arg::with_name("registry")
                .value_name("ADDR")
                .short('r')
                .long("registry")
                .default_value("registry.portals.rs:65534")
                .help("Specifies the registry to synchronize tunnel information with"),
        ]
    }

    fn run(&self, matches: &ArgMatches, debug: bool) -> Result<(), Box<dyn std::error::Error>> {
        let addr = matches.value_of("addr").unwrap();
        if addr.contains(':') {
            server::start(addr, debug)
        } else {
            server::start(("0.0.0.0", addr.parse::<u16>().unwrap()), debug)
        }
    }
}
