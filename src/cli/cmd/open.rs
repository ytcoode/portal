use super::Cmd;
use crate::client;
use crate::util::args;
use clap::Arg;
use clap::ArgMatches;

// portal open [-s host[:port]] <[host:]port>...
pub struct Open;

impl Cmd for Open {
    fn name(&self) -> &str {
        "open"
    }

    fn about(&self) -> &str {
        "Opens portal tunnels to forward connections"
    }

    fn args(&self) -> Vec<Arg<'_>> {
        vec![
            Arg::with_name("server")
                .short('s')
                .long("server")
                .value_name("host[:port]")
                .default_value("p.ytcode.io:65535")
                .help("Specifies the portal server to connect to")
                .validator(|s| {
                    // host[:port]
                    if !s.contains(":") {
                        return Ok(());
                    }

                    let v = s.split(":").collect::<Vec<&str>>();
                    if v.len() != 2 || v.iter().any(|x| x.is_empty()) {
                        return Err("invalid format".to_string());
                    }

                    args::validate_port(v[1])
                }),
            Arg::with_name("dest")
                .value_name("DEST")
                .multiple(true)
                .required(true)
                .help(
                    "Specifies the destination to which the portal tunnel will forward connections
Format: [remote_port:][local_host:]local_port",
                )
                .validator(|s| {
                    // [remote_port:][local_host:]local_port
                    if !s.contains(":") {
                        return args::validate_port(&s);
                    }

                    let v = s.split(":").collect::<Vec<&str>>();
                    if v.iter().any(|x| x.is_empty()) {
                        return Err("invalid format".to_string());
                    }

                    args::validate_port(v[v.len() - 1])?;

                    match v.len() {
                        2 => Ok(()),
                        3 => args::validate_port(v[0]),
                        _ => Err("invalid format".to_string()),
                    }
                }),
        ]
    }

    fn run(&self, matches: &ArgMatches, debug: bool) -> Result<(), Box<dyn std::error::Error>> {
        let server = matches
            .value_of("server")
            .unwrap()
            .split(":")
            .collect::<Vec<&str>>();

        let (host, port) = match server.len() {
            1 => (server[0], 65535),
            2 => (server[0], server[1].parse::<u16>().unwrap()),
            _ => unreachable!(),
        };

        let dests = matches.values_of("dest").unwrap();

        client::start(host, port, dests, debug)
    }
}
