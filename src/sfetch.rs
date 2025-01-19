use sysinfo::System;
use raw_cpuid::CpuId;
use colored::Colorize;
use crate::OS_NAME;

pub fn run() {
    let mut sys = System::new();
    sys.refresh_all();

    let uptime = System::uptime();
    let cpuid = CpuId::new();

    // Get the vendor string (e.g., "GenuineIntel" or "AuthenticAMD")
    let vendor_info = cpuid.get_vendor_info()
        .map_or("Unknown Vendor".to_string(),
        |v| v.as_str().to_string());

    // Get the CPU brand (e.g., "Intel Core i7")
    let cpu_brand = cpuid.get_processor_brand_string()
        .map_or("Unknown CPU".to_string(),
        |brand| brand.as_str().to_string());

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
