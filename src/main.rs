use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        panic!("Please provide a file name");
    }

    let file = match File::open(&args[1]) {
        Ok(file) => file,
        Err(error) => 
            match error.kind() {
                std::io::ErrorKind::NotFound => {
                    panic!("File not found: {}", args[1]);
                },
                _ => {
                    panic!("Error opening file: {}", error);
                },
            }
        };
    let reader = BufReader::new(file);

    for line in reader.lines() {
        let line = line.expect("Could not read line");
        println!("{}", line);
    }
}