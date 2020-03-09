use std::{io, process};

pub fn prompt_for_input(prompt: &str) -> String {
    // print!() doesn't work, newline is needed, and it looks ugly.
    // Will make it pretty, later.
    println!("{}", prompt);
    let mut user_input = String::new();
    match io::stdin().read_line(&mut user_input) {
        Ok(_num_bytes_read) => {},
        Err(e) => {
            eprintln!("Failed to read CLI input from user: {}", e);
            eprintln!("Literally crashing the entire program. Goodbye.");
            process::exit(1);
        },
    }
    user_input.trim().to_string()
}