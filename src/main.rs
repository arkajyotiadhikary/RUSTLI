#[allow(unused_imports)]
use std::io::{self, Write};

fn main() {
    // defining a  infinite loop
    loop {
        print!{"$ "};
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        let input = input.trim();

        if input == "exit 0" {
            break;
        }
        println!("{}: command not found", input);
    }
}
