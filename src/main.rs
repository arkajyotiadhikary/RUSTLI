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

        // spliting the txt into args
        // if user inputs exit 0 exit will the the command and 0 will be the arg
        let args: Vec<&str> = input.split_whitespace().collect();

        let command = args[0];
        let args = &args[1..];

        match command{
            "exit" => {
                break;
            }
            "echo" => {
                println!("{}", args.join(" "));
            }
            _ => {
                println!("{}: command not found", command);
            }
        }
    }
}
