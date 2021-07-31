use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader, Error, Result, Write};
use std::os::unix::io::FromRawFd;
use std::path::Path;

const BUFFER_CAP: usize = 1024 * 128;

fn print_files(files: &[&String]) -> Result<()> {
    let mut ok = true;
    unsafe {
        let mut stdout = File::from_raw_fd(1);
        for file in files {
            if *file == &String::from("-") {
                ok = print_stdin(&mut stdout).is_ok();
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
                print_file(&file, &mut stdout)?;
            } else {
                false;
            }
        }
    }

    if ok {
        Ok(())
    } else {
        Err(Error::last_os_error())
    }
}

fn print_file(file: &File, stdout: &mut File) -> Result<()> {
    let mut reader = BufReader::with_capacity(BUFFER_CAP, file);
    loop {
        let length = {
            let buffer = reader.fill_buf()?;
            stdout.write_all(buffer)?;
            buffer.len()
        };

        if length == 0 {
            break;
        }

        reader.consume(length);
    }

    Ok(())
}

fn print_stdin(stdout: &mut File) -> Result<()> {
    let stdin = std::io::stdin();
    let mut stdin_lock = stdin.lock();
    let mut line = String::new();

    loop {
        let bytes = stdin_lock.read_line(&mut line)?;
        if bytes == 0 {
            break;
        }

        stdout.write_all(line.as_bytes())?;
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
        errors = print_files(&vec![&String::from("-")]).is_err();
    } else {
        errors = print_files(&files).is_err();
    }

    if errors {
        std::process::exit(1)
    }
}
