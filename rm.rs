use std::env;
use std::fs;
use std::io::{self, Write};

fn main() {
let args: Vec<String> = env::args().collect();
let mut recursive = false;
let mut force = false;
let mut interactive = false;
let mut files: Vec<String> = Vec::new();


let mut i = 1;
while i < args.len() {
    match args[i].as_ref() {
        "-r" | "-R" => recursive = true,
        "-f" => force = true,
        "-i" => interactive = true,
        _ => files.push(args[i].clone()),
    }
    i += 1;
}

if files.is_empty() {
    eprintln!("rm: missing operand");
    return;
}

for file in files.iter() {
    if file == "." || file == ".." {
        eprintln!("rm: cannot remove '.' or '..'");
        continue;
    }

    let metadata = match fs::metadata(file) {
        Ok(metadata) => metadata,
        Err(_) => {
            eprintln!("rm: cannot remove '{}': No such file or directory", file);
            continue;
        }
    };

    if metadata.is_dir() {
        if !recursive {
            eprintln!("rm: cannot remove '{}': Is a directory", file);
            continue;
        }

        remove_directory(file, force, interactive);
    } else {
        remove_file(file, force, interactive);
    }
}
}

fn remove_directory(dir: &str, force: bool, interactive: bool) {
if !force && interactive {
print!("rm: remove directory '{}'? ", dir);
io::stdout().flush().unwrap();


    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();

    if !input.starts_with('y') && !input.starts_with('Y') {
        return;
    }
}

match fs::remove_dir_all(dir) {
    Ok(_) => (),
    Err(err) => eprintln!("rm: cannot remove '{}': {}", dir, err),
}
}

fn remove_file(file: &str, force: bool, interactive: bool) {
if !force && interactive {
print!("rm: remove regular file '{}'? ", file);
io::stdout().flush().unwrap();

    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();

    if !input.starts_with('y') && !input.starts_with('Y') {
        return;
    }
}

match fs::remove_file(file) {
    Ok(_) => (),
    Err(err) => eprintln!("rm: cannot remove '{}': {}", file, err),
}
}