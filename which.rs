use std::env;
use std::fs;
use std::path::Path;

fn main() {
let mut args = env::args();

if args.len() < 2 {
    eprintln!("Usage: which COMMAND [-a]");
    std::process::exit(1);
}

let command = args.nth(1).unwrap();
let show_all = args.any(|arg| arg == "-a");

let path_string = env::var("PATH").expect("PATH is not set");
let paths: Vec<&str> = path_string.split(":").collect();

let mut found = false;
for path in paths {
    let full_path = Path::new(path).join(&command);
    if full_path.exists() {
        found = true;
        println!("{}", full_path.to_str().unwrap());
        if !show_all {
            break;
        }
    }
}

if !found {
    eprintln!("{}: command not found", command);
    std::process::exit(1);
}
}




