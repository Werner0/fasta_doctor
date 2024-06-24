use std::path::Path;
use std::ffi::OsStr;
use std::fs::File;
use std::io::{BufReader, prelude::*};

pub fn check_file_extension(filename: &str) -> Result<(), String> {
    let allowed_extensions = ["fasta", "fna", "faa", "aa"];
    let path = Path::new(filename);
    let extension = path.extension()
        .and_then(OsStr::to_str)
        .ok_or_else(|| format!("File '{}' does not have a valid extension.", filename))?;

    if !allowed_extensions.contains(&extension) {
        return Err(format!("File '{}' does not have a valid extension. Please use one of the following extensions: {:?}", filename, allowed_extensions));
    }

    Ok(())
}

pub fn check_file_content(filename: &str) -> Result<(), String> {
    let file = File::open(filename).map_err(|_| format!("Error: Could not open file '{}'.", filename))?;
    let reader = BufReader::new(file);

    if reader.lines().filter_map(Result::ok).any(|line| line.starts_with('>')) {
        Ok(())
    } else {
        Err(format!("File '{}' must contain at least one line starting with '>' character.", filename))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_extension() {
        let filename = "sequence.fasta";
        assert!(Path::new(filename).extension().is_some());
        assert!(check_file_extension(filename).is_ok());
    }

    #[test]
    fn test_invalid_extension() {
        let filename = "sequence.txt";
        assert!(check_file_extension(filename).is_err());
    }

    #[test]
    fn test_file_with_valid_content() {
        let filename = "valid_content.fasta";
        let content = ">sequence\nATCG";
        std::fs::write(filename, content).unwrap();
        assert!(check_file_content(filename).is_ok());
        std::fs::remove_file(filename).unwrap(); // Clean up the file.
    }

    #[test]
    fn test_file_with_invalid_content() {
        let filename = "invalid_content.fasta";
        let content = "ATCG";
        std::fs::write(filename, content).unwrap();
        assert!(check_file_content(filename).is_err());
        std::fs::remove_file(filename).unwrap(); // Clean up the file.
    }
}