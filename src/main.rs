use std::io::{BufReader, Read, Write};
use std::fmt::Write as FmtWrite;
use std::{env, fs::File, error::Error};
use regex::Regex; // Import the Regex type

fn main() -> Result<(), Box<dyn Error>> {
    // Get the filename from the command line arguments
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: {} <file>", args[0]);
        std::process::exit(1);
    }
    let filename = &args[1];

    // Open the file to read from
    let file = File::open(filename)?;
    let mut buf_reader = BufReader::new(file);

    // String to accumulate the hex representation
    let mut hex_string = String::new();

    // Buffer to hold file data
    let mut buffer = [0; 16]; // Read 16 bytes at a time
    loop {
        let nbytes = buf_reader.read(&mut buffer)?;
        if nbytes == 0 {
            break;
        }
        for &byte in buffer.iter().take(nbytes) {
            // Convert each byte to a two-character uppercase hex string
            write!(&mut hex_string, "{:02X}", byte).expect("Unable to write to string");
        }
    }

    // Find the index of the first occurrence of "3e"
    if let Some(index) = hex_string.find("3E") {
        // Trim everything up to but NOT including the first "3e"
        // and update hex_string with the remaining part
        hex_string = hex_string[index..].to_string();
    }

    // DEBUG STEP: Print the original hex dump to the console as a single line
    // println!("{}", hex_string);

    // Regex operations
    let re_3e = Regex::new(r"(3E)+")?;
    hex_string = re_3e.replace_all(&hex_string, "3E").to_string();

    // Remove all characters in the hex range 00 to 1f, except 0a
    let re_control_chars = Regex::new(r"(00|01|02|03|04|05|06|07|08|09|0B|0C|0D|0E|0F|10|11|12|13|14|15|16|17|18|19|1A|1B|1C|1D|1E|1F)").unwrap();
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
    
    // Append "0A" at the end of the string
    hex_string.push_str("0A");
    let re_consecutive_0a_at_end = Regex::new(r"(0A)+\z")?;
    hex_string = re_consecutive_0a_at_end.replace(&hex_string, "0A").to_string();

    // DEBUG STEP: Print the amended hex dump to the console as a single line
    // println!("{}", hex_string);

    // Convert the hex string to text
    let output = convert_hex_to_text(&hex_string)?;

    // Write the output to a text file
    let mut file = File::create("output.fasta")?;
    file.write_all(output.as_bytes())?;

    Ok(())
}

// Function to convert hex string to text
fn convert_hex_to_text(hex_str: &str) -> Result<String, Box<dyn Error>> {
    let mut output = String::new();
    for i in (0..hex_str.len()).step_by(2) {
        let hex_slice = &hex_str[i..i + 2];
        let hex_value = u8::from_str_radix(hex_slice, 16)?;
        output.push(hex_value as char);
    }
    Ok(output)
}