use std::env;
use std::path::Path;
use std::fs;
use std::io::{self, Write};


fn print_file_contents(filename: &String) {
    if let Ok(data) = fs::read(filename) {
        io::stdout().write_all(&data);
    } else {

    }
}

fn main() {
    let no_buffering_option: String = String::from("-u");
    let args: Vec<String> = env::args().collect::<Vec<String>>()[1..].to_vec();
    // remove -u option
    let files: Vec<&String> = args.iter().filter(|arg| *arg != &no_buffering_option).collect();
    
    for file in files {
        let path = Path::new(file);

        if !path.exists() {
            eprintln!("rat: {}: No such file or directory", file);
            continue;
        }
        if path.is_dir() {
            eprintln!("rat: {}: Is a directory", file);
            continue;
        }

        print_file_contents(file);
    }
}
