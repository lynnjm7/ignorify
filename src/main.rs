extern crate ignorify;
extern crate clap;

use clap::{App, Arg};
use ignorify::{init, process};

// Get the package configuration details from the Cargo.toml file
const NAME: &'static str = env!("CARGO_PKG_NAME");
const VERSION: &'static str = env!("CARGO_PKG_VERSION");
const AUTHORS: &'static str = env!("CARGO_PKG_AUTHORS");

fn main() {
    // Setup the command line arguments
    let cmd_args = App::new(NAME)
        .version(VERSION)
        .author(AUTHORS)
        .about("Command line utility to create gitignore files.")
        .arg(Arg::with_name("init").short("i").long("init").help(
            "Runs the initial setup",
        ))
        .arg(
            Arg::with_name("options")
                .short("o")
                .long("option")
                .help(
                    "Add a platform or language to be included in the gitignore file",
                )
                .multiple(true)
                .takes_value(true)
                .index(1),
        )
        .arg(Arg::with_name("list").short("l").long("list").help(
            "List the available platforms and langauge snippets that are available",
        ))
        .get_matches();

    // If the init command is specified, run the init operation
    if cmd_args.is_present("init") {
        init::setup_ignorify();
        return;
    }

    // If the list command is specicied, list all the available snippets
    if cmd_args.is_present("list") {
        process::list_options(true);
        return;
    }

    // Gather the languages and platforms to generate the ignore file for
    let files = match cmd_args.values_of("options") {
        Some(x) => x.collect(),
        None => {
            println!("For usage information please use --help");
            std::process::exit(0);
        }
    };
    process::generate_ignore(files);
}
