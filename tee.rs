use std::env;
use std::fs::OpenOptions;
use std::io;
use std::io::prelude::*;

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    let mut append = false;
    let mut ignore_interrupts = false;
    let mut files = Vec::new();

    // Parse arguments
    let mut i = 1;
    while i < args.len() {
        let arg = &args[i];
        match arg.as_ref() {
            "-a" | "--append" => {
                append = true;
                i += 1;
            }
            "-i" | "--ignore-interrupts" => {
                ignore_interrupts = true;
                i += 1;
            }
            "--help" => {
                print_help();
                return Ok(());
            }
            "--version" => {
                print_version();
                return Ok(());
            }
            _ => {
                files.push(arg);
                i += 1;
            }
        }
    }

    // Read from standard input and write to standard output and files
    let mut stdin = io::stdin();
    let mut stdout = io::stdout();
    let mut buffer = [0; 1024];
    loop {
        let bytes_read = stdin.read(&mut buffer)?;
        if bytes_read == 0 {
            break;
        }

        stdout.write_all(&buffer[..bytes_read])?;

        for file in &files {
            let mut file = OpenOptions::new()
                .write(true)
                .create(true)
                .append(append)
                .open(file)?;
            file.write_all(&buffer[..bytes_read])?;
        }
    }

    Ok(())
}

fn print_help() {
    println!("Usage: tee [-ai] [--append] [--ignore-interrupts] [ file... ]");
    println!("       tee [--help] [--version]");
}

fn print_version() {
    println!("tee 0.1.0");
}
