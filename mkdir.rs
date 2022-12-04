use std::env;
use std::fs;
use std::path::Path;

fn main() {
let mut args = env::args();
args.next(); // skip the program name


let mut options = vec![];
let mut directories = vec![];
let mut parents = false;
let mut verbose = false;

while let Some(arg) = args.next() {
    if arg == "-p" || arg == "--parents" {
        parents = true;
    } else if arg == "-v" || arg == "--verbose" {
        verbose = true;
    } else if arg.starts_with("-m") || arg.starts_with("--mode=") {
        let mode = arg.split("=").last().unwrap();
        options.push(format!("-m {}", mode));
    } else if arg == "--help" {
        println!("Usage: mkdir [options] directory...");
        return;
    } else if arg == "--version" {
        println!("mkdir version 1.0");
        return;
    } else if arg == "--" {
        break;
    } else {
        directories.push(arg);
    }
}

for dir in directories {
    let path = Path::new(&dir);
    if parents {
        fs::create_dir_all(&path).expect("Failed to create directory");
    } else {
        fs::create_dir(&path).expect("Failed to create directory");
    }

    if verbose {
        println!("Created directory {}", dir);
    }
}
}