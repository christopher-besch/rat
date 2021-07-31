use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader, Result, Write};
use std::path::Path;

const BUFFER_CAP: usize = 1024 * 128;

fn print_files(files: &[&String]) -> bool {
    let mut ok = true;
    for file in files {
        if *file == &String::from("-") {
            ok = print_stdin().is_ok();
            continue;
        }

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

        if let Ok(file) = File::open(file) {
            ok &= print_file(&file).is_ok();
        } else {
            false;
        }
    }

    ok
}

fn print_file(file: &File) -> Result<()> {
    let stdout = std::io::stdout();

    let mut stdout_lock = stdout.lock();
    let mut reader = BufReader::with_capacity(BUFFER_CAP, file);
    loop {
        let length = {
            let buffer = reader.fill_buf()?;
            stdout_lock.write_all(buffer)?;
            buffer.len()
        };

        if length == 0 {
            break;
        }

        reader.consume(length);
    }

    Ok(())
}

fn print_stdin() -> Result<()> {
    let stdin = std::io::stdin();
    let stdout = std::io::stdout();

    let mut stdout_lock = stdout.lock();
    let mut stdin_lock = stdin.lock();
    let mut line = String::new();

    loop {
        let bytes = stdin_lock.read_line(&mut line)?;
        if bytes == 0 {
            break;
        }

        stdout_lock.write_all(line.as_bytes())?;
        line = String::from("");
    }

    Ok(())
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
    let options_to_remove = vec![String::from("-u"), String::from("--help")];
    let args: Vec<String> = env::args().skip(1).collect();

    if args.contains(&String::from("--help")) {
        print_help();
        std::process::exit(0)
    }

    let files: Vec<&String> = args
        .iter()
        .filter(|arg| !options_to_remove.contains(*arg))
        .collect();

    let mut errors = false;
    if files.is_empty() {
        errors = !print_files(&vec![&String::from("-")]);
    } else {
        errors = !print_files(&files);
    }

    if errors {
        std::process::exit(1)
    }
}
