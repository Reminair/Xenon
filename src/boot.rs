use colored::*;
use std::{thread, time};

pub fn boot() {
    // Ramsay Boot screen
    println!("{}", "Welcome to xenonOS v1.1".purple().bold());
    println!("{}", "Run 'help' to get started".purple().bold());

    let duration = time::Duration::from_secs(3);

    let start_time = std::time::Instant::now();
    while start_time.elapsed() < duration {
            thread::sleep(time::Duration::from_millis(100));
    }

    println!();
}
