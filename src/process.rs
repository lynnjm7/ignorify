use std::env;
use std::fs::{self, File};
use std::io;
use std::io::prelude::*;
use std::path::{Path, PathBuf};
use std::collections::HashMap;

///
/// Traverse the snippets directory recursively; building a hash table of the file name
/// to the snippet's path. 
///
fn visit_dirs(dir: &Path, options: &mut HashMap<Box<String>, Box<PathBuf>>) -> io::Result<()> {
    if dir.is_dir() {
        for entry in fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();


            // Ignore the '.git' repository that was cloned for the snippets
            if path.ends_with(".git") {
                continue;
            }
            
            match path.extension() {
                Some(x) => if x != "gitignore" {continue;},
                None => continue
            }

            if path.is_dir() {
                visit_dirs(&path, options)?
            } else {
                let f = Box::new(entry.path().file_stem().unwrap().to_str().unwrap().to_owned().to_lowercase());
                let p = Box::new(entry.path());
                options.insert(f, p);
            }
        }
    }
    Ok(())
}

///
/// Generate a map of potential options from the ~/.ignorify/snippets directory.
///
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

pub fn generate_ignore(in_opts: Vec<&str>) {
    let avail_opts = list_options(false);

    println!("\n### Generated from ignorify ###\n");

    for op in in_opts {
        let s = op.to_string().to_lowercase();
        println!("### {} ###", s);
        let p = avail_opts.get(&s).unwrap().as_path();
        let mut file = File::open(p).unwrap();
        let mut contents = String::new();
        file.read_to_string(&mut contents).unwrap();

        println!("{}", contents);
    }
}
