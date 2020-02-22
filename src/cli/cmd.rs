use clap::Arg;
use clap::ArgMatches;

mod open;
mod setup;

pub trait Cmd {
    fn name(&self) -> &str;

    fn about(&self) -> &str;

    fn args(&self) -> Vec<Arg<'_>>;

    fn run(&self, matchs: &ArgMatches, debug: bool) -> Result<(), Box<dyn std::error::Error>>;
}

pub fn all() -> Vec<&'static dyn Cmd> {
    vec![&setup::Setup, &open::Open]
}
