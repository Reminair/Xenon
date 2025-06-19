use std::{
    thread,
    time,
    fs::File,
    io::{
        prelude::*,
        stdout,
        self,
        Write
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

// Import local modules
mod socha;
mod sfetch;

pub static OS_NAME: &str = "Xenon 1.2";

fn main() {
    boot();
    cli();
}

fn boot() {
    // Ramsay Boot screen
    println!("{} {}",
        "Welcome to".purple().bold(),
        OS_NAME.purple().bold()
    );
    println!("{}",
        "Run 'help' to get started".purple().bold()
    );

    let duration = time::Duration::from_secs(3);

    let start_time = std::time::Instant::now();
    while start_time.elapsed() < duration {
        thread::sleep(time::Duration::from_millis(100));
    }

    println!();
}

pub fn clear() {
    let mut stdout = stdout();
    let _ = stdout.execute(MoveToColumn(0));            // Move to left-most column
    let _ = stdout.execute(MoveToRow(0));               // Move to top-most row
    let _ = stdout.execute(Clear(ClearType::All));      // Clear screen
    let _ = stdout.execute(Clear(ClearType::Purge));    // Clear history
}

fn read_lines(filename: &str) -> std::io::Result<()> {  // Output contents of file given
    let mut file = File::open(filename)?;            // Set file to filename
    let mut contents = String::new();                   // Touch contents as String
    file.read_to_string(&mut contents)?;                // Read file to contents
    println!("{}", contents.bold().purple());           // Print contents formatted
    Ok(())
}

pub fn cli() {
    loop {
        // Display a prompt for the user
        print!("{}>", "Xenon".bold().purple());
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
                let _ = read_lines("help.txt");
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
                let _ = read_lines("credits.txt");
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
