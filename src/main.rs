use std::env;
use std::fs;
use std::fs::File;
use std::io::{BufRead, Write};
use std::os::unix::io::FromRawFd;
use std::path::Path;

fn print_files(files: &[&String]) -> bool {
    let mut ok: bool = true;

    unsafe {
        let mut stdout = File::from_raw_fd(1);
        for file in files {
            let path = Path::new(file);

            if !path.exists() {
                eprintln!("rat: {}: No such file or directory", file);
                ok = false;
                continue;
            }

            if path.is_dir() {
                eprintln!("rat: {}: Is a directory", file);
                ok = false;
                continue;
            }

            if let Ok(data) = fs::read(file) {
                if stdout.write_all(&data).is_err() {
                    ok = false;
                }
            } else {
                ok = false;
            }
        }
    }

    ok
}

fn print_stdin() -> bool {
    let stdin = std::io::stdin();
    let mut stdin_lock = stdin.lock();
    let mut line = String::new();

    loop {
        if let Ok(bytes) = stdin_lock.read_line(&mut line) {
            if bytes == 0 {
                return true;
            }

            print!("{}", line);
            line = String::from("");
        } else {
            return false;
        }
    }
}

fn print_help() {
    println!("Usage: rat [OPTION]... [FILE]...");
    println!("Concatenate FILE(s) to standard output.");
    println!();
    println!("With no FILE, or when FILE is -, read standard input.");
    println!();
    println!("-u     ignored");
    println!("--help display this help and exit");
    println!("Examples:");
    println!();
    println!("cat f g  Output f's contents, then g's contents.");
    println!("cat      Copy standard input to standard output.");
}

#[allow(unused_assignments)]
fn main() {
    let options_to_remove = vec![String::from("-"), String::from("--help")];
    let args: Vec<String> = env::args().collect::<Vec<String>>()[1..].to_vec();

    if args.contains(&String::from("--help")) {
        print_help();
        std::process::exit(0)
    }

    let files: Vec<&String> = args
        .iter()
        .filter(|arg| !options_to_remove.contains(*arg))
        .collect(); // remove -u

    let mut errors = false;
    if files.is_empty() || files[0] == &String::from("-") {
        errors = !print_stdin();
    } else {
        errors = !print_files(&files);
    }

    if errors {
        std::process::exit(1)
    }
}
