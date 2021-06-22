use std::env;
use std::fs;

fn main() {

    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("Not enough arguments");
    }

    let filename = &args[1];

    println!("Reading from the file {}", filename);

    let file_contents = fs::read_to_string(filename)
        .expect("Was unable to read the file");
    
    println!("{}", file_contents)
}