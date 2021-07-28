use std::env;
use std::path::Path;
use std::fs;
use std::fs::File;
use std::io::Write;
use std::os::unix::io::FromRawFd;


fn print_files(files: &Vec<&String>) -> bool {
    let mut errors: bool = false;

    unsafe {
        let mut stdout = File::from_raw_fd(1);
        for file in files {
            let path = Path::new(file);

            if !path.exists() {
                eprintln!("rat: {}: No such file or directory", file);
                errors = true;
                continue;
            }

            if path.is_dir() {
                eprintln!("rat: {}: Is a directory", file);
                errors = true;
                continue;
            }

            if let Ok(data) = fs::read(file) {
                if let Err(_) = stdout.write_all(&data) {
                    errors = true;
                }
            } else {
                errors = true;
            }
        }
    }

    errors
}

fn main() {
    let no_buffering_option: String = String::from("-u");
    let args: Vec<String> = env::args().collect::<Vec<String>>()[1..].to_vec();
    // remove -u option
    let files: Vec<&String> = args.iter().filter(|arg| *arg != &no_buffering_option).collect();
    let mut errors = false;
    
    // if files.is_empty() || files[0] == &String::from("-") {
    //     errors = read_stdin();
    // } else {
        errors = print_files(&files);
    // }
    
    if errors {
        std::process::exit(1)
    }
}
