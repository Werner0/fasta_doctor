//Get byte count and non-null character count

use std::fs::File;
use std::io::{self, BufReader, Read};
use std::path::Path;

pub fn get_file_info<P: AsRef<Path>>(path: P) -> io::Result<(u64, usize)> {
    let file = File::open(path)?;
    let metadata = file.metadata()?;
    let file_size = metadata.len(); // File size in bytes

    let mut buf_reader = BufReader::new(file);
    let mut contents = String::new();
    buf_reader.read_to_string(&mut contents)?;
    let chars_vector: Vec<char> = contents.chars().into_iter().filter(|&c| c != '\0').collect();
    let num_chars = chars_vector.len(); // Number of characters

    Ok((file_size, num_chars))
}