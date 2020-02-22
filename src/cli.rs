use clap::crate_authors;
use clap::crate_description;
use clap::crate_name;
use clap::crate_version;
use clap::App;
use clap::AppSettings;
use clap::Arg;

mod cmd;

pub fn start() {
    let cmds = cmd::all();

    let app = App::new(crate_name!())
        .version(crate_version!())
        .author(crate_authors!())
        .about(crate_description!())
        .global_setting(AppSettings::ArgRequiredElseHelp)
        .global_setting(AppSettings::HelpRequired)
        .arg(
            Arg::with_name("debug")
                .short('d')
                .long("debug")
                .help("Prints debugging messages"),
        );

    let app = cmds.iter().fold(app, |app, cmd| {
        app.subcommand(App::new(cmd.name()).about(cmd.about()).args(cmd.args()))
    });

    let matches = app.get_matches();

    if let (name, Some(m)) = matches.subcommand() {
        cmds.into_iter()
            .find(|cmd| cmd.name() == name)
            .unwrap()
            .run(m, matches.is_present("debug"))
            .unwrap();
    } else {
        panic!("command not found");
    }
}
