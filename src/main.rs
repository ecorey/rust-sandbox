use std::fs::File;
use std::io::{self, BufRead, BufReader, Write};

fn main() {
    // Writing to a file
    let write_result = write_to_file("output.txt", "Hello, Rust!");

    match write_result {
        Ok(_) => println!("Successfully wrote to the file."),
        Err(error) => {
            match error.kind() {
                std::io::ErrorKind::NotFound => {
                    println!("File not found: {}", error)
                }
                std::io::ErrorKind::PermissionDenied => {
                    println!("Permission denied: {}", error)
                }
                _ => {
                    println!("Error writing to file: {}", error)
                }
            }
        }
    }

    // Reading from a file (assuming the file exists)
    let file = File::open("output.txt");
    let file = match file {
        Ok(file) => file,
        Err(error) => {
            match error.kind() {
                std::io::ErrorKind::NotFound => {
                    panic!("File not found: {}", error)
                }
                std::io::ErrorKind::PermissionDenied => {
                    panic!("Permission denied: {}", error)
                }
                _ => {
                    panic!("Error opening file: {}", error)
                }
            }
        }
    };
    
    let reader = BufReader::new(file);
    for line in reader.lines() {
        match line {
            Ok(line) => println!("{}", line),
            Err(error) => {
                panic!("Error reading line: {}", error)
            }
        }
    }
}

// Function to write to a file
fn write_to_file(filename: &str, content: &str) -> io::Result<()> {
    let mut file = match File::create(filename) {
        Ok(file) => file,
        Err(error) => return Err(error),
    };

    match file.write_all(content.as_bytes()) {
        Ok(_) => Ok(()),
        Err(error) => Err(error),
    }
}
