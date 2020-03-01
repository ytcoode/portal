use super::Cmd;
use crate::client;
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

                    v[1].parse::<u16>()
                        .map_or_else(|e| Err(e.to_string()), |_| Ok(()))
                }),
            Arg::with_name("dest")
                .value_name("DEST")
                .multiple(true)
                .required(true)
                .help("Specifies the destination to which the portal tunnel will forward connections [format: [host:]port]")
                .validator(|s| {
                    // [host:]port
                    if s.contains(":") {
                        let v = s.split(":").collect::<Vec<&str>>();
                        match v.len() {
                            2 if v.iter().all(|x| !x.is_empty()) => v[1],
                            _ => return Err("invalid format".to_string()),
                        }
                    } else {
                        &s
                    }
                    .parse::<u16>()
                    .map_or_else(|e| Err(e.to_string()), |_| Ok(()))
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

        let dests = matches
            .values_of("dest")
            .unwrap()
            .map(|s| {
                if s.contains(":") {
                    s.to_string()
                } else {
                    "127.0.0.1".to_string() + ":" + s
                }
            })
            .collect::<Vec<String>>();

        client::start(host, port, dests, debug)
    }
}
