#[allow(unused_imports)]
use std::io::{self, Write};

struct Command {
    name: String,
    args: Vec<String>,
}

// a class for commands
impl Command {
    // a static array of commands
    const COMMANDS: &'static [&'static str] = &["exit", "echo", "type"];

    // construct new commadn with name and args
    fn new(name: String, args: Vec<String>) -> Self {
        Self { name, args }
    }

    // exicute the command
    fn execute(&self) {
        match self.name.as_str() {
            "exit" => {
                std::process::exit(0);
            }
            "echo" => {
                println!("{}", self.args.join(" "));
            }
            "type" => {
                if Self::COMMANDS.contains(&self.args[0].as_str()) {
                    if let Some(details) = Self::get_command_details(&self.args[0]) {
                        println!("{}", details);
                    }
                } else {
                    println!("{}: not found", self.args[0]);
                }
            }
            _ => {
                println!("{}: command not found", self.name);
            }
        }
    }

    // get details about a specific command
    fn get_command_details(command: &str) -> Option<&str> {
        match command {
            "exit" => Some("exit is a shell builtin"),
            "echo" => Some("echo is a shell builtin"),
            "type" => Some("type is a shell builtin"),
            _ => None,
        }
    }
}

fn print_prompt() -> io::Result<()> {
    print!("$ ");
    io::stdout().flush()
}

fn main() {
    // defining a  infinite loop
    loop {
        if let Err(e) = print_prompt() {
            eprintln!("Error displaying prompt: {}", e);
            continue;
        }

        let mut input = String::new();
        if let Err(e) = io::stdin().read_line(&mut input) {
            eprintln!("Error reading input: {}", e);
            continue;
        }

        let input = input.trim();

        if input.is_empty() {
            continue;
        }

        // spliting the txt into args
        // if user inputs exit 0 exit will the the command and 0 will be the arg
        let args: Vec<String> = input.split_whitespace().map(String::from).collect();

        if args.is_empty() {
            continue;
        }

        let command = args[0].clone();
        let args = args[1..].to_vec();

        let cmd = Command::new(command, args);
        cmd.execute();
    }
}
