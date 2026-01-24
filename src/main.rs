use rand::Rng;
use std::fs::File;
use std::io::Write;
use uuid::Uuid;

fn main() {
    // Check internet connectivity
    if !check_internet_connectivity() {
        eprintln!("Error: No internet connection detected. Exiting without creating files.");
        std::process::exit(1);
    }

    println!("Internet connection verified.");

    // Generate random number of files to create (between 30 and 47)
    let mut rng = rand::thread_rng();
    let file_count = rng.gen_range(30..=47);
    println!("Creating {} files with UUID names...", file_count);

    // Create files with UUID names
    for i in 0..file_count {
        let uuid = Uuid::new_v4();
        let filename = format!("{}.txt", uuid);

        match File::create(&filename) {
            Ok(mut file) => {
                // Write some content to the file
                if let Err(e) = writeln!(file, "File created with UUID: {}", uuid) {
                    eprintln!("Warning: Could not write to file {}: {}", filename, e);
                }
                println!("Created file {}: {}", i + 1, filename);
            }
            Err(e) => {
                eprintln!("Error creating file {}: {}", filename, e);
            }
        }
    }

    println!("\nSuccessfully created {} files.", file_count);
}

fn check_internet_connectivity() -> bool {
    println!("Checking internet connectivity...");

    // Try multiple reliable endpoints for better reliability
    let test_urls = vec![
        "https://www.google.com",
        "https://www.cloudflare.com",
        "https://www.example.com",
    ];

    // Use reqwest blocking client to check connectivity
    for url in test_urls {
        match reqwest::blocking::get(url) {
            Ok(response) => {
                if response.status().is_success() {
                    return true;
                }
            }
            Err(_) => continue,
        }
    }

    eprintln!("Failed to connect to any test endpoint");
    false
}
