// Description: Fixes a FASTA file

mod memory_monitor;

use std::sync::{Arc, atomic::{AtomicBool, Ordering}};
use std::time::Duration;
use std::fmt::Write as FmtWrite;
use std::io::{BufReader, Read, Write as IoWrite};
use std::{env, fs::File, error::Error};
use regex::Regex;

fn main() -> Result<(), Box<dyn Error>> {
    //initialize memory monitor
    let is_running = Arc::new(AtomicBool::new(true));
    let is_running_clone = Arc::clone(&is_running);
    memory_monitor::start_memory_monitoring(Duration::from_secs(5), is_running_clone);

    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: {} <file> [--rename] [--unwrap]", args[0]);
        std::process::exit(1);
    }
    let filename = &args[1];
    let rename_headers = args.iter().any(|arg| arg == "--rename");
    let unwrap_lines = args.iter().any(|arg| arg == "--unwrap");

    let file = File::open(filename)?;
    let mut buf_reader = BufReader::new(file);
    let mut hex_string = String::new();
    let mut buffer = [0; 16];
    while let Ok(nbytes) = buf_reader.read(&mut buffer) {
        if nbytes == 0 {
            break;
        }
        for &byte in buffer.iter().take(nbytes) {
            write!(&mut hex_string, "{:02X}", byte)?;
        }
    }

    if let Some(index) = hex_string.find("3E") {
        hex_string = hex_string[index..].to_string();
    }

    let re_3e = Regex::new(r"(3E)+")?;
    hex_string = re_3e.replace_all(&hex_string, "3E").to_string();

    let re_control_chars = Regex::new(r"00|01|02|03|04|05|06|07|08|09|0B|0C|0D|0E|0F|10|11|12|13|14|15|16|17|18|19|1A|1B|1C|1D|1E|1F")?;
    let mut clean_string = String::new();
    let chars: Vec<char> = hex_string.chars().collect();

    for i in (0..chars.len()).step_by(2) {
        if i + 1 < chars.len() {
            let pair = format!("{}{}", chars[i], chars[i + 1]);
            if !re_control_chars.is_match(&pair) {
                clean_string.push_str(&pair);
            }
        }
    }
    hex_string = clean_string;

    hex_string.push_str("0A");
    let re_consecutive_0a_at_end = Regex::new(r"(0A)+\z")?;
    hex_string = re_consecutive_0a_at_end.replace(&hex_string, "0A").to_string();

    let output = convert_hex_to_text(&hex_string, unwrap_lines)?;

    let (output, header_mappings) = if rename_headers {
        rename_headers_in_output(&output)?
    } else {
        (output, Vec::new())
    };

    let mut file = File::create("output.fasta")?;
    IoWrite::write_all(&mut file, output.as_bytes())?;

    if rename_headers {
        let mut names_file = File::create("mapping.fasta_doctor")?;
        for (old, new) in header_mappings {
            writeln!(names_file, "{}\t{}", old, new)?;
        }
    }
    //stop memory monitor
    is_running.store(false, Ordering::SeqCst);
    std::thread::sleep(Duration::from_secs(1));

    Ok(())
}

// Modify the convert_hex_to_text function to accept a new parameter for unwrapping
fn convert_hex_to_text(hex_str: &str, unwrap: bool) -> Result<String, Box<dyn Error>> {
    let mut output = String::with_capacity(hex_str.len() / 2);
    for i in (0..hex_str.len()).step_by(2) {
        let hex_slice = &hex_str[i..i + 2];
        let hex_value = u8::from_str_radix(hex_slice, 16)?;
        output.push(hex_value as char);
    }

    if unwrap {
        output = unwrap_fasta(&output);
    }

    Ok(output)
}

fn rename_headers_in_output(output: &str) -> Result<(String, Vec<(String, String)>), Box<dyn Error>> {
    let mut new_output = String::with_capacity(output.len());
    let mut header_mappings = Vec::new();
    let mut header_count = 0;

    for line in output.lines() {
        if line.starts_with('>') {
            header_count += 1;
        }
    }

    let header_width = header_count.to_string().len();
    let mut current_header_number = 0;

    for line in output.lines() {
        if line.starts_with('>') {
            let new_header = format!(">A{:0width$}B", current_header_number, width = header_width);
            header_mappings.push((line.to_string(), new_header.clone()));
            new_output.push_str(&new_header);
            current_header_number += 1;
        } else {
            new_output.push_str(line);
        }
        new_output.push('\n');
    }

    Ok((new_output, header_mappings))
}

// Function to handle unwrapping
fn unwrap_fasta(input: &str) -> String {
    let mut unwrapped = String::new();
    let mut is_header = true; // Assume the first line is a header

    for line in input.lines() {
        if line.starts_with('>') {
            if !is_header {
                unwrapped.push('\n'); // Separate previous sequence from this header
            }
            unwrapped.push_str(line);
            unwrapped.push('\n');
            is_header = true;
        } else if is_header {
            unwrapped.push_str(line);
            is_header = false;
        } else {
            unwrapped.push_str(line);
        }
    }

    unwrapped
}