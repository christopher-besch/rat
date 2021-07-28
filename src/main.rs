use std::env;
use std::path::Path;
use std::fs;
use std::fs::File;
use std::io::Write;
use std::os::unix::io::FromRawFd;

#[allow(unused_must_use)]
fn print_file_contents(filename: &String, stdout: &mut File) {
    if let Ok(data) = fs::read(filename) {
        stdout.write_all(&data);
    }
}

fn main() {
    let no_buffering_option: String = String::from("-u");
    let args: Vec<String> = env::args().collect::<Vec<String>>()[1..].to_vec();
    // remove -u option
    let files: Vec<&String> = args.iter().filter(|arg| *arg != &no_buffering_option).collect();
    
    unsafe {
        let mut stdout = File::from_raw_fd(1);
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

            print_file_contents(file, &mut stdout);
        }
    }
}
