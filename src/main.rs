use std::io;

fn main() {
    println!("Welcome to tasky - A simple todo application!");

    // Read user input
    println!("Enter input: ");
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("failed to read line.");

    println!("You entered: {}", input);
}
