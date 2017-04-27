extern crate ignorify;
extern crate clap;

use clap::{App, Arg};
use ignorify::{init, process};

fn main() {
    let cmd_args = App::new("Ignorify")
        .version("1.0")
        .author("Josh Lynn <lynnjm7@gmail.com>")
        .about("Creates gitignore files.")
        .arg(Arg::with_name("init")
                 .short("i")
                 .long("init")
                 .help("Runs the initial setup"))
        .arg(Arg::with_name("options")
                 .short("o")
                 .long("option")
                 .help("Add a platform or language to be included in the gitignore file")
                 .multiple(true)
                 .takes_value(true)
                 .index(1))
        .arg(Arg::with_name("list")
                 .short("l")
                 .long("list")
                 .help("List the available platforms and langauge snippets that are available"))
        .get_matches();

    if cmd_args.is_present("init") {
        init::setup_ignorify();
        return;
    }

    if cmd_args.is_present("list") {
        process::list_options(true);
        return;
    }

    // Gather the languages and platforms to generate the ignore file for
    let files = cmd_args.values_of("options").unwrap().collect();
    process::generate_ignore(files);
}
