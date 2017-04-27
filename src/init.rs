extern crate git2;

use self::git2::Repository;

use std::fs;
use std::env;

fn create_ignorify_dir() {
    println!("Creating '~/.ignorify' directory...");
    let mut dir = env::home_dir().unwrap();
    dir.push(".ignorify");

    match fs::create_dir(dir) {
        Ok(_) => println!("    Ignorify directory created!"),
        Err(_) => println!("    Unable to create ignorify directory!"),
    };
}

fn clone_snippets() {
    println!("Cloning gitignore snippets...");
    let url = "https://github.com/lynnjm7/gitignore.git";
    let mut dest = env::home_dir().unwrap();
    dest.push(".ignorify");
    dest.push("snippets");

    match Repository::clone(url, dest) {
        Ok(_) => println!("    Snippets repository cloned!"),
        Err(_) => println!("    Unable to clone snippets repository!"),
    }
}

pub fn setup_ignorify() {
    create_ignorify_dir();
    clone_snippets();
}
