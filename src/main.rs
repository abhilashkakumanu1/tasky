use std::fs::{File, OpenOptions};
use std::io;
use std::io::Write;
use std::path::Path;

fn main() {
    println!("Welcome to tasky - A simple todo application!");

    let mut file: File;

    // Check if the file exists, if not create it
    let file_exists = Path::new("db.txt").exists();

    if !file_exists {
        file = File::create("db.txt").expect("Error creating the file.");
    } else {
        // If file already exists, open it in append mode
        file = OpenOptions::new()
            .append(true)
            .open("db.txt")
            .expect("Error opening the file.");
    }

    // Infinite loop => REPL
    loop {
        // Read user input
        println!("Enter input: ");
        let mut input = String::new();

        io::stdin()
            .read_line(&mut input)
            .expect("failed to read line.");

        file.write_all(input.to_owned().as_bytes())
            .expect("Error writing to file.");
    }
}
