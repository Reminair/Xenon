use colored::Colorize;
use std::{
    thread,
    time
};

mod cli;
mod socha;
mod sfetch;

pub static OS_NAME: &str = "Xenon 1.2";

fn main() {
    boot();
    cli::run();
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