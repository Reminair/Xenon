use std::io::{  // For reading user input and printing output
    self,
    Write
};
use sysinfo::System;
use raw_cpuid::CpuId;
use colored::*;
use console::*;
use crate::socha;
use crate::socha::run_file_manager;

pub fn clear_screen() {
    let term = Term::stdout();
    term.clear_screen().unwrap();
}

pub fn run_cli() -> Result<(), std::io::Error> {
    let mut sys = System::new();
    sys.refresh_all();
    // Display a prompt for the user
    loop {
        print!("{}", "Xenon".purple().bold());
        print!("{}", ">".white());
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
                println!("Available commands:\n");
                println!("{}: {}",
                    format!("{: >11}",
                    "help"),
                    "Show this help message"
                );
                println!("{}: {}",
                    format!("{: >11}",
                    "shutdwn"),
                    "Shutdown Xenon"
                );
                println!("{}: {}",
                    format!("{: >11}",
                    "clr"),
                    "Clear the screen"
                );
                println!("{}: {}",
                    format!("{: >11}",
                    "say"),
                    "Repeats a message exactly"
                );
                println!("{}: {}",
                    format!("{: >11}",
                    "sfetch"),
                    "Show Xenon system info"
                );
                println!("{}: {}",
                    format!("{: >11}",
                    "socha"),
                    "The Xenon file manager"
                );
                println!("{}: {}",
                    format!("{: >11}",
                    "credits"),
                    "Credits to the people who made Xenon Possible"
                );
            }
            "shutdwn" => {
                println!("Goodbye!...");
                std::process::exit(0);
            }
            "clr" => {
                clear_screen();
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
                let _ = socha::run_file_manager();
            }
            "credits" => {
                println!("{}", "XXXXXXX       XXXXXXX          OOOOOOOOO        SSSSSSSSSSSSSSS ".bold().purple());
                println!("{}", "X:::::X       X:::::X        OO:::::::::OO    SS:::::::::::::::S".bold().purple());
                println!("{}", "X:::::X       X:::::X      OO:::::::::::::OO S:::::SSSSSS::::::S".bold().purple());
                println!("{}", "X::::::X     X::::::X     O:::::::OOO:::::::OS:::::S     SSSSSSS".bold().purple());
                println!("{}", "XXX:::::X   X:::::XXX     O::::::O   O::::::OS:::::S            ".bold().purple());
                println!("{}", "   X:::::X X:::::X        O:::::O     O:::::OS:::::S            ".bold().purple());
                println!("{}", "    X:::::X:::::X         O:::::O     O:::::O S::::SSSS         ".bold().purple());
                println!("{}", "     X:::::::::X          O:::::O     O:::::O  SS::::::SSSSS    ".bold().purple());
                println!("{}", "     X:::::::::X          O:::::O     O:::::O    SSS::::::::SS  ".bold().purple());
                println!("{}", "    X:::::X:::::X         O:::::O     O:::::O       SSSSSS::::S ".bold().purple());
                println!("{}", "   X:::::X X:::::X        O:::::O     O:::::O            S:::::S".bold().purple());
                println!("{}", "XXX:::::X   X:::::XXX     O::::::O   O::::::O            S:::::S".bold().purple());
                println!("{}", "X::::::X     X::::::X     O:::::::OOO:::::::OSSSSSSS     S:::::S".bold().purple());
                println!("{}", "X:::::X       X:::::X      OO:::::::::::::OO S::::::SSSSSS:::::S".bold().purple());
                println!("{}", "X:::::X       X:::::X        OO:::::::::OO   S:::::::::::::::SS ".bold().purple());
                println!("{}", "XXXXXXX       XXXXXXX          OOOOOOOOO      SSSSSSSSSSSSSSS   ".bold().purple());
                println!(" ");
                println!(" ");
                println!("{}", "Xenon - Bought to you by the Xenon Group".purple());
                println!("{}", "Built by Orion in his bedroom on 06/01/24".purple());
                println!("{}", "Credit for the name 'Xenon' goes to 'Reminair', Thank you ^-^".purple());
                println!("{}", "Credit to the IEEE for creating the POSIX standard.".purple());
                println!("{}", "Thank you to everyone using Xenon without people using it, Xenon wouldn't exist!".purple().bold());
            }
            _ => {
                println!("Command not recognized. Type 'help' for a list of commands.");
            }
        }
        fn show_sfetch(sys: &mut System) {
            let os_name = "xenonOS 1.0";
            let uptime = System::uptime();
            let cpuid = CpuId::new();

            // Get the vendor string (e.g., "GenuineIntel" or "AuthenticAMD")
            let vendor_info = cpuid.get_vendor_info().map_or("Unknown Vendor".to_string(), |v| v.as_str().to_string());

            // Get the CPU brand (e.g., "Intel Core i7")
            let cpu_brand = cpuid.get_processor_brand_string()
                .map_or("Unknown CPU".to_string(), |brand| brand.as_str().to_string());

            let total_ram = sys.total_memory() / u64::pow(1024, 2); // Convert to MB
        
            // Info header
            println!("{}", style("Xenon System Info").bold().magenta());

            // Info body
            let ylbld = console::Style::new().bold().yellow(); // Yellow bold style

            println!("{}: {}",
                format!("{: >11}",      // Pad "OS:" with spaces from the left to 11 characters
                ylbld.apply_to("OS")),  // Apply style
                os_name
            );
            println!("{}: {} seconds",
                format!("{: >11}",
                ylbld.apply_to("Uptime")),
                uptime
            );
            println!("{}: {}",
                format!("{: >11}",
                ylbld.apply_to("CPU Vendor")),
                vendor_info
            );
            println!("{}: {}",
                format!("{: >11}",
                ylbld.apply_to("CPU Brand")),
                cpu_brand
            );
            println!("{}: {} MB",
                format!("{: >11}",
                ylbld.apply_to("RAM")),
                total_ram
            );
        }
    }
}
