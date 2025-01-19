use std::{
    fs::File,
    path::Path,
    io::{
        stdout,
        self,
        Write,
        BufRead
    }
};
use crossterm::{
    ExecutableCommand,
    cursor::{
        MoveToColumn,
        MoveToRow
    },
    terminal::{
        Clear,
        ClearType
    }
};
use colored::Colorize;
use crate::socha;
use crate::sfetch;

pub fn clear() {
    let mut stdout = stdout();
    let _ = stdout.execute(MoveToColumn(0));            // Move to left-most column
    let _ = stdout.execute(MoveToRow(0));               // Move to top-most row
    let _ = stdout.execute(Clear(ClearType::All));      // Clear screen
    let _ = stdout.execute(Clear(ClearType::Purge));    // Clear history
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

pub fn run() {
    loop {
        // Display a prompt for the user
        print!("{}{}", "Xenon".bold().purple(), ">");
        io::stdout().flush().unwrap();

        // Read the user's input
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        // Remove the newline character from the input
        let input = input.trim();

        // Parse arguments
        let mut input_pieces = input.split_whitespace();
        let command = input_pieces.next().unwrap_or("");

        // Execute the command
        match command {
            "help" => {
                if let Ok(lines) = read_lines("./help.txt") {
                    // Consumes the iterator, returns an (Optional) String
                    for line in lines.map_while(Result::ok) {
                        println!("{}", line);
                    }
                }
            }
            "clr" => {
                clear();
            }
            "say" => {
                let text = input_pieces.collect::<Vec<&str>>().join(" ");

                println!("{}", text);
                io::stdout().flush().unwrap();
            }
            "sfetch" => {
                sfetch::run();
            }
            "socha" => {
                let _ = socha::run();
            }
            "credits" => {
                if let Ok(lines) = read_lines("./credits.txt") {
                    // Consumes the iterator, returns an (Optional) String
                    for line in lines.map_while(Result::ok) {
                        println!("{}", line.bold().purple());
                    }
                }
            }
            "shutdwn" => {
                println!("Goodbye!...");
                std::process::exit(0);
            }
            _ => {
                println!("Command not recognized. Type 'help' for a list of commands.");
            }
        }
    }
}
