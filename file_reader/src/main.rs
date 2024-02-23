use std::fs::File;
use std::io::{BufRead, BufReader};
use std::env;

fn read_file(file_path: &String){
    println!("Processing file {}", file_path);
    let file = File::open(file_path);

    let file = match file {
        Ok(file) => file,
        Err(error) => {
            match error.kind() {
                std::io::ErrorKind::NotFound => {
                    panic!("File not found {}", error)
                }
                _ => {
                    panic!("Error opening the file {}", error)
                }
            }
        }
    };

    let reader = BufReader::new(file);
    for line in reader.lines(){
        match line {
            Ok(line) => println!("{}", line),
            Err(error) => panic!("Error reading line {}", error)
        }

    }
}


fn main() {
    let args: Vec<String> = env::args().collect();
    read_file(&args[1])
}
