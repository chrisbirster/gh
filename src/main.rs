use clap::{App, Arg, SubCommand};

fn main() {
    let matches = App::new("gh")
        .version("1.0")
        .author("Chris Birster <me@chris.website>")
        .about("The github api as a cli app")
        .arg(
            Arg::with_name("config")
                .short("c")
                .long("config")
                .value_name("FILE")
                .help("Sets a custom config file")
                .takes_value(true),
        )
        .subcommand(
            SubCommand::with_name("test")
                .about("controls testing features")
                .arg(
                    Arg::with_name("debug")
                        .short("d")
                        .help("print debug information verbosely"),
                ),
        )
        .get_matches();

    let config = matches.value_of("config").unwrap_or("default.conf");
    println!("Value for config: {}", config);
}
