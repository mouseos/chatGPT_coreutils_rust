use std::env;
use std::io;
use std::path::PathBuf;

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();

    match args.len() {
        1 => {
            let pwd = env::current_dir()?;
            println!("{}", pwd.display());
        }
        2 => {
            match args[1].as_str() {
                "--help" => {
                    println!("Usage: pwd [OPTION]...");
                    println!("Print the current working directory.\n");
                    println!("  --help     display this help and exit");
                    println!("  --version  output version information and exit");
                }
                "--version" => println!("pwd version 0.1.0"),
                _ => println!("pwd: invalid option: {}", args[1]),
            }
        }
        _ => println!("pwd: too many arguments"),
    }

    Ok(())
}
