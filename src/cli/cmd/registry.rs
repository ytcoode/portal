use super::Cmd;
use crate::registry;
use crate::util::args;
use clap::Arg;
use clap::ArgMatches;

// portal registry --http-server [[ip:]port] [[ip:]port]
pub struct Registry;

impl Cmd for Registry {
    fn name(&self) -> &str {
        "registry"
    }

    fn about(&self) -> &str {
        "Sets up a portal registry for querying tunnel information"
    }

    fn args(&self) -> Vec<Arg<'_>> {
        vec![
            Arg::with_name("addr")
                .value_name("ADDR")
                .default_value("65534")
                .help("Specifies the registry address")
                .validator(|s| {
                    if s.contains(':') {
                        args::validate_socket_addr(&s)
                    } else {
                        args::validate_port(&s)
                    }
                }),
            Arg::with_name("http-server")
                .value_name("ADDR")
                .short('H')
                .long("http-server")
                .default_value("80")
                .help("Specifies the http server address")
                .validator(|s| {
                    if s.contains(':') {
                        args::validate_socket_addr(&s)
                    } else {
                        args::validate_port(&s)
                    }
                }),
        ]
    }

    fn run(&self, matches: &ArgMatches, debug: bool) -> Result<(), Box<dyn std::error::Error>> {
        let addr = matches.value_of("addr").unwrap();
        let addr = args::to_socket_addr(addr);

        let hs_addr = matches.value_of("http-server").unwrap();
        let hs_addr = args::to_socket_addr(hs_addr);

        registry::start(addr, hs_addr, debug)
    }
}
