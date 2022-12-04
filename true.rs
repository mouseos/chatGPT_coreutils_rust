use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() == 2 {
        match args[1].as_ref() {
            "--help" => print_help(),
            "--version" => print_version(),
            _ => (),
        }
    }
}

fn print_help() {
    println!("Usage: true [--help] [--version]");
}

fn print_version() {
    println!("true version 1.0.0");
}
