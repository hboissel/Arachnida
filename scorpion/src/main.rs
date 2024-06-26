use std::fs::File;
use std::io::BufReader;
use exif::Reader;
use chrono::offset::Utc;
use chrono::DateTime;
use std::time::SystemTime;
use std::io::Error;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let args: Vec<String> = args[1..].to_vec();

    if args.is_empty() {
        println!("🦂 Scorpion 🦂");
        println!("A simple command line tool to view metadata and EXIF data of files");
        println!("Usage: scorpion <file1> <file2> ...");
        return;
    }
    
    for arg in args {
        if !file_exists(&arg) {
            println!("🚫 File not found: {}", arg);
            continue;
        }
        println!("🖼️  {}", arg);
        print_metadata(&arg);
        let r = print_exit_data(&arg);
        if r.is_err() {
            println!("🚫 No EXIF data found");
        }
        println!("\n{}\n", "─".repeat(50));
    }
}

/// Fucntion to check if the file exists
/// It takes a path to a file as an argument
/// It returns a boolean
fn file_exists(file: &str) -> bool {
    std::fs::metadata(file).is_ok()
}

/// Function to print EXIF data from a file
/// It takes a path to a file as an argument
/// It returnes nothing
/// It prints the EXIF data of the file
fn print_exit_data(file: &str) -> Result<(), ()> {
    let file = File::open(file).expect("Failed to open file");
    let mut reader = BufReader::new(&file);

    let exif_reader = Reader::new();
    let exif_data = exif_reader.read_from_container(&mut reader);
    let exif_data = match exif_data {
        Ok(exif_data) => exif_data,
        Err(_) => return Err(()),
    };

    for field in exif_data.fields() {
        println!("{}: {}", field.tag, field.display_value().with_unit(&exif_data));
    }
    Ok(())
}

/// Function that prints all the metadata of a file
/// It takes a path to a file as an argument
/// It returns nothing
fn print_metadata(file: &str) {
    let metadata = std::fs::metadata(file).expect("Failed to read metadata");
    println!("Size: {:.2} MB", metadata.len() as f64 / 1_000_000.0);
    println!("Modified: {}", format_time(metadata.modified()));
    println!("Created: {:?}", format_time(metadata.created()));
    println!("Accessed: {:?}", format_time(metadata.accessed()));
    println!();
}

/// It takes a SystemTime as an argument
/// It returns a string
/// It formats the time into a readable format
/// It uses the chrono library
fn format_time(time: Result<SystemTime, Error>) -> String {
    let time = match time {
        Ok(time) => time,
        Err(_) => return "N/A".to_string(),
    };
    let datetime: DateTime<Utc> = time.into();
    let time_readable = format!("{}", datetime.format("%d/%m/%Y %T"));
    time_readable
}
