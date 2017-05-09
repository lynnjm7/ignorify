extern crate git2;

use self::git2::Repository;

use std::fs;
use std::env;

///
/// Create an empty ~/.ignorify directory.
/// This directory is used for storing config files and
/// the ignore snippets (in ~/.ignorify/snippets
///
fn create_ignorify_dir() {
    println!("Creating '~/.ignorify' directory...");
    let mut dir = env::home_dir().unwrap();
    dir.push(".ignorify");

    match fs::create_dir(dir) {
        Ok(_) => println!("    Ignorify directory created!"),
        Err(_) => println!("    Unable to create ignorify directory!"),
    };
}

///
/// Using libgit2, clone the remote gitignore snippets repository
/// from GitHub to the ~/.ignorify/snippets
///
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

///
/// Run the initial setup functions.
/// This function will create the ~/.ignorify directory and fetch
/// the snippets directory from the GitHub repository.
///
pub fn setup_ignorify() {
    create_ignorify_dir();
    clone_snippets();
}
