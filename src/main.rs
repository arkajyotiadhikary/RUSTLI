#[allow(unused_imports)]
use std::io::{self, Write};

fn main() {
    // Uncomment this block to pass the first stage
    print!("$ ");
    io::stdout().flush().unwrap();

    // Wait for user input
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();

    // print invalid command message 
    println!("{}: command not found",input.trim());
}
