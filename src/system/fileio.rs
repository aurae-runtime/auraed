use std::fs;
use walkdir::WalkDir;

fn print_flat_dir(paths: fs::ReadDir) {
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
        let paths = fs::read_dir(dir);

        match paths {
            Ok(paths) => print_flat_dir(paths),
            Err(e) => println!("Could not read {}. Error: {}", dir, e),
        }
    }
}

#[allow(dead_code)]
fn write_file(file_path: &str, content: &str) {
    unsafe {
        let fp: *mut libc::FILE = libc::fopen(
            String::from(file_path).as_bytes().as_ptr() as *const i8,
            String::from("w").as_bytes().as_ptr() as *const i8,
        );

        libc::fprintf(
            fp,
            String::from(content).as_bytes().as_ptr() as *const i8,
        );
        libc::fclose(fp);
    }
}
