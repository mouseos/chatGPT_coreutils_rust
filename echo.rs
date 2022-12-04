fn main() {
    let mut args: Vec<String> = std::env::args().collect();
    let mut newline = true;
    let mut escape = false;

    if args.len() == 1 {
        println!("echo: missing arguments");
        return;
    }

    if args[1] == "--help" {
        println!("Usage: echo [-ne] [string...]");
        return;
    } else if args[1] == "--version" {
        println!("echo 1.0");
        return;
    }

    if args[1] == "-n" {
        newline = false;
        args.remove(1);
    }

    if args[1] == "-e" {
        escape = true;
        args.remove(1);
    }

    for (i, arg) in args.iter().enumerate() {
        if i == 0 {
            continue;
        }

        if escape {
            let mut escaped_arg = String::new();
            let mut escaped = false;
            for c in arg.chars() {
                if escaped {
                    match c {
                        '\\' => escaped_arg.push_str("\\"),
                        'a' => escaped_arg.push_str("\u{0007}"),
                        'b' => escaped_arg.push_str("\u{0008}"),
                        'c' => newline = false,
                        'f' => escaped_arg.push_str("\u{000C}"),
                        'n' => escaped_arg.push_str("\n"),
                        'r' => escaped_arg.push_str("\r"),
                        't' => escaped_arg.push_str("\t"),
                        'v' => escaped_arg.push_str("\u{000B}"),
                        _ => escaped_arg.push(c),
                    }
                    escaped = false;
                } else if c == '\\' {
                    escaped = true;
                } else {
                    escaped_arg.push(c);
                }
            }
            print!("{}", escaped_arg);
        } else {
            print!("{}", arg);
        }

        if i != args.len() - 1 {
            print!(" ");
        } else if newline {
            println!("");
        }
    }
}
