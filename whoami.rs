use std::env;

fn main() {
let args: Vec<String> = env::args().collect();
if args.len() == 1 {
println!("{}", env::var("USER").unwrap());
} else if args.len() == 2 {
match args[1].as_str() {
"--help" => print_help(),
"--version" => print_version(),
_ => println!("Invalid option: {}", args[1]),
}
} else {
println!("Too many arguments");
}
}

fn print_help() {
println!("Usage: whoami [OPTION]...");
println!("Displays the current user's username");
println!("");
println!("Options:");
println!(" --help\t\tDisplay this help message and exit");
println!(" --version\tDisplay version information and exit");
}

fn print_version() {
println!("whoami version 1.0.0");
}