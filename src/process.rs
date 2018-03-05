use std::env;
use std::fs::{self, File};
use std::io;
use std::io::prelude::*;
use std::path::{Path, PathBuf};
use std::collections::HashMap;

/// Traverse the snippets directory recursively; building a hash table of the file name
/// to the snippet's path.
fn visit_dirs(dir: &Path, options: &mut HashMap<Box<String>, Box<PathBuf>>) -> io::Result<()> {
    if dir.is_dir() {
        for entry in fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();

            // Ignore the '.git' repository that was cloned for the snippets
            if path.ends_with(".git") {
                continue;
            }

            if path.is_dir() {
                visit_dirs(&path, options)?
            } else {
                match path.extension() {
                    Some(x) => {
                        if x != "gitignore" {
                            continue;
                        }
                    }
                    None => continue,
                }

                // Because we need to return these values, they need to be boxed
                // on the free store/heap.
                let f = Box::new(
                    entry
                        .path()
                        .file_stem()
                        .unwrap()
                        .to_str()
                        .unwrap()
                        .to_owned()
                        .to_lowercase(),
                );
                let p = Box::new(entry.path());
                options.insert(f, p);
            }
        }
    }
    Ok(())
}

/// Generate a map of potential options from the ~/.ignorify/snippets directory.
pub fn list_options(display: bool) -> HashMap<Box<String>, Box<PathBuf>> {
    let mut dir = env::home_dir().unwrap();
    dir.push(".ignorify");
    dir.push("snippets");

    let mut options = HashMap::new();
    visit_dirs(dir.as_path(), &mut options).unwrap();

    if display {
        for (key, _) in &options {
            println!("{}", key);
        }
    }

    options
}

/// Generate the GitIgnore file from the list of options and write the file
/// to stdout. The output from stdout can be redirected into a file or
/// piped into another *nix command.
pub fn generate_ignore(in_opts: Vec<&str>) {
    let avail_opts = list_options(false);

    println!("\n### Generated from ignorify ###\n");

    for op in in_opts {
        let s = op.to_string().to_lowercase();
        println!("### {} ###", s);
        let p = match avail_opts.get(&s) {
            Some(x) => x.as_path(),
            None => {
                println!("# Invalid selection! Please use --list to see available options.\n");
                continue;
            }
        };
        let mut file = File::open(p).unwrap();
        let mut contents = String::new();
        file.read_to_string(&mut contents).unwrap();

        println!("{}", contents);
    }
}
