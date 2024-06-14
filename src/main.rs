// Description: Fixes a FASTA file

mod memory_monitor;
mod file_info;

use std::sync::{Arc, atomic::{AtomicBool, Ordering}};
use std::time::Duration;
use std::fmt::Write as FmtWrite;
use std::io::{BufReader, Read, Write as IoWrite};
use std::{env, fs::File, error::Error};
use regex::Regex;
use std::process;

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

    match file_info::get_file_info(filename) {
        Ok((file_size, num_chars)) => {
            let ratio = file_size as f64 / num_chars as f64;
            //println!("File size: {} bytes, Number of characters: {}, Ratio: {}", file_size, num_chars, ratio);
            if ratio >= 2.0 {
                eprintln!("Warning: Multibyte encoding detected");
                process::exit(1);
            }
        },
        Err(e) => {
            eprintln!("Error reading file: {}", e);
            process::exit(1);
        }
    }

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
    
    // Remove all "0A" pairs from the end of clean_string
    while clean_string.ends_with("0A") {
    clean_string.truncate(clean_string.len() - 2);
    }

    hex_string = clean_string;

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
    output.push('\n');
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
    // Check if the last character is not a newline, then add one
    if !unwrapped.ends_with('\n') {
    unwrapped.push('\n');
    }

    unwrapped
}

// Unit tests
#[cfg(test)]
mod tests {
    // Bring all functions under test into scope
    use super::*;

    // Unit tests
    #[test]
    fn test_rename_headers_in_output() {
        let input = ">header1\nATGC\n>header2\nATGC";
        let expected_output = ">A0B\nATGC\n>A1B\nATGC\n";
        let (output, mappings) = rename_headers_in_output(input).unwrap();

        assert_eq!(output, expected_output);
        assert_eq!(mappings.len(), 2);
        assert_eq!(mappings[0], (String::from(">header1"), String::from(">A0B")));
        assert_eq!(mappings[1], (String::from(">header2"), String::from(">A1B")));
    }

    #[test]
    fn test_convert_hex_to_text_without_unwrap() {
        let hex_str = "3E686561646572310A41544743";
        let expected_output = ">header1\nATGC\n";
        let output = convert_hex_to_text(hex_str, false).unwrap();

        assert_eq!(output, expected_output);
    }

    #[test]
    fn test_convert_hex_to_text_with_unwrap() {
        let hex_str = "3E686561646572310A415447430A3E686561646572320A43414754";
        let expected_output = ">header1\nATGC\n>header2\nCAGT\n";
        let output = convert_hex_to_text(hex_str, true).unwrap();

        assert_eq!(output, expected_output);
    }
    #[test]
    fn test_unwrap_fasta() {
        let wrapped_fasta = ">header1\nATGC\nATGC\n>header2\nGCTA\nGCTA";
        let expected = ">header1\nATGCATGC\n>header2\nGCTAGCTA\n";
        let result = unwrap_fasta(wrapped_fasta);
        assert_eq!(result, expected);
    }
}