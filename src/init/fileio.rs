use std::fs::{ReadDir, read_dir};

use walkdir::WalkDir;

fn print_flat_dir(paths: ReadDir) {
    for path in paths {
        match path {
            Err(p) => println!("Error: {}", p),
            Ok(p) => println!("{}", p.path().display()),
        }
    }
}

#[allow(dead_code)]
pub fn show_dir(dir: &str, recurse: bool) {
    if recurse {
        for entry in WalkDir::new(dir) {
            let entry = entry;
            match entry {
                Ok(p) => println!("{}", p.path().display()),
                Err(e) => println!("Could not read {}. Error: {}", dir, e),
            }
        }
    } else {
        let paths = read_dir(dir);

        match paths {
            Ok(paths) => print_flat_dir(paths),
            Err(e) => println!("Could not read {}. Error: {}", dir, e),
        }
    }
}
