use sysinfo::System;
use raw_cpuid::CpuId;
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
use crate::boot::OS_NAME;
use crate::socha;

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

pub fn run_cli() {
    let mut sys = System::new();
    sys.refresh_all();

    loop {
        print!("{}{}", "Xenon".bold().purple(), ">");   // Display a prompt for the user
        io::stdout().flush().unwrap();

        // Read the user's input
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        // Remove the newline character from the input
        let input = input.trim();

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
            "shutdwn" => {
                println!("Goodbye!...");
                std::process::exit(0);
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
                sys.refresh_cpu(); // Ensure CPU info is updated before displaying
                show_sfetch(&mut sys);
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
            _ => {
                println!("Command not recognized. Type 'help' for a list of commands.");
            }
        }
        fn show_sfetch(sys: &mut System) {
            let uptime = System::uptime();
            let cpuid = CpuId::new();

            // Get the vendor string (e.g., "GenuineIntel" or "AuthenticAMD")
            let vendor_info = cpuid.get_vendor_info().map_or("Unknown Vendor".to_string(), |v| v.as_str().to_string());

            // Get the CPU brand (e.g., "Intel Core i7")
            let cpu_brand = cpuid.get_processor_brand_string()
                .map_or("Unknown CPU".to_string(), |brand| brand.as_str().to_string());

            let total_ram = sys.total_memory() / u64::pow(1024, 2); // Convert to MB
        
            // Info header
            println!("{}", ("Xenon System Info").bold().purple());

            // Info body
            let name = [
                "OS",
                "Uptime",
                "CPU Vendor",
                "CPU Brand",
                "RAM"
            ];
            let value = [
                OS_NAME,
                &(uptime.to_string() + " seconds"),
                &vendor_info,
                &cpu_brand,
                &(total_ram.to_string() + " MB")
            ];

            for i in 0..5 {
                println!("{}: {}",
                    format!("{: >11}",
                        (name[i]).bold().yellow()), // Pad %name with spaces from the left to column 11
                        value[i].to_string()        // Apply style to %value
                );
            }
        }
    }
}
