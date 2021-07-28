use std::env;
use std::path::Path;
use std::fs;
use std::io::{self, Write};


fn print_file_contents(filename: &String) {
    if let Ok(data) = fs::read(filename) {
        io::stdout().write_all(&data);
    }
}

// TODO: handle -u option
fn main() {
    let no_buffering_option: String = String::from("-u");
    let args: Vec<String> = env::args().collect::<Vec<String>>()[1..].to_vec();
    let files: Vec<&String> = args.iter().filter(|arg| *arg != &no_buffering_option).collect();
    let _ = args.contains(&no_buffering_option);
    
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
