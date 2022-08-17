use std::io;

fn main() {
    println!("Welcome to tasky - A simple todo application!");

    let mut tasks: Vec<String> = Vec::new();

    // Infinite loop => REPL
    loop {
        // Read user input
        println!("Enter input: ");
        let mut input = String::new();

        io::stdin()
            .read_line(&mut input)
            .expect("failed to read line.");

        tasks.push(input.trim().to_string());

        println!("Tasks: {:?}", tasks);
    }
}
