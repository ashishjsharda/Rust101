use std::fs::File;
use std::io::{Write, Read};

fn main() -> std::io::Result<()> {
    // Create and write to file
    let mut file = File::create("hello.txt")?;
    file.write_all(b"Hello Rust!")?;  // Added ? for error handling

    // Open and read file - using a different variable name
    let mut file = File::open("hello.txt")?;  // Changed to ? instead of unwrap()
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;  // Added ? for error handling

    println!("File Contents are {}", contents);
    Ok(())
}
