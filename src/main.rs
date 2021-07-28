use std::env;
use std::path::Path;


// TODO: handle -u option
fn main() {
    let no_buffering_option: String = String::from("-u");
    let args: Vec<String> = env::args().collect::<Vec<String>>()[1..].to_vec();
    let files: Vec<&String> = args.iter().filter(|arg| *arg != &no_buffering_option).collect();
    let _ = args.contains(&no_buffering_option);
    for file in files {
        if !Path::new(file).exists() {
            println!("rat: {}: No such file or directory", file);
            continue;
        }
    }
}
