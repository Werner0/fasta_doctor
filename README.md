# fasta_doctor  
### Description
A command-line tool to repair and clean [FASTA files](https://en.wikipedia.org/wiki/FASTA_format), equipped with a wide array of features. It ensures the presence of a single end-of-file newline character, regardless of its initial state, and adeptly handles the removal of both empty lines and various non-printable characters, such as carriage returns and horizontal tabs. Notably, `fasta_doctor` stands out with its capability to parse multibyte character set encoded files, like UTF-16, which is not a common feature among similar tools. It also efficiently removes duplicate fasta header markers and any content leading the first header mark, ensuring the integrity of the file structure. Additionally, `fasta_doctor` offers functionalities for renaming header lines and saving a persistent mapping of old to new headers, enhancing its utility for users requiring advanced FASTA file manipulation and management.

### Installation 
Option 1: A pre-built version for Linux is included in this repository.
```
git clone https://github.com/Werner0/fasta_doctor.git
cd fasta_doctor
chmod a+x fasta_doctor_x86_64
```
Option 2: Compile it yourself (requires [Rust and Cargo](https://doc.rust-lang.org/cargo/getting-started/installation.html)).
```
git clone https://github.com/Werner0/fasta_doctor.git
cd fasta_doctor
cargo test
cargo build
```
### Usage
+ Linux: `./fasta_doctor_x86_64 <input.fasta|.fna|.faa|.aa> [--rename] [--unwrap]`.  
+ Self compiled: `./target/debug/fasta_doctor <input.fasta|.fna|.faa|.aa> [--rename] [--unwrap]`.

### Output
+ A fixed FASTA file: `output.fasta`.
+ A file that maps old to new FASTA headers: `mapping.fasta_doctor` (only created with `--rename`).

### What fasta_doctor does by default
+ Ensures the input FASTA file starts with a greater-than sign (">") and ends with a newline character ("\n").
+ Removes duplicate greater-than signs (e.g. ">>" becomes ">").
+ Removes all non-printable control characters in the hexadecimal range 00-1F (except 0A).
+ Detects multibyte character encoding (e.g. when the input file uses UTF-16 or UTF-32). Output is saved using UTF-8.

### What fasta_doctor does optionally
+ Renames FASTA headers using the alphanumeric pattern A[n]B (e.g. A0001B). Requires `--rename`.
+ Creates a persistent mapping from old to new FASTA headers. Requires `--rename`.
+ Removes line wrapping. Requires `--unwrap`.

### Example
The following incorrectly formatted FASTA content contains a comment line, an empty line, duplicate header row markers (greater-than signs), inconsistent header names, and line wrapping.
```
#Fasta
>>Clone@1 | test 1
ACTG
GTCA

>>Clone2
ACTG
G
T
C
A
```
Running fasta_doctor with the `--rename` and `--unwrap` flags will return the following output:
```
>A0B
ACTGGTCA
>A1B
ACTGGTCA
```
Because the `--rename` flag is used, old headers are also mapped to new headers and saved in `mapping.fasta_doctor`.

### Comparison to other FASTA file parsers
| Method | [Test number](example/) |	fasta_doctor | [EMBOSS seqret](https://www.ebi.ac.uk/jdispatcher/sfc/emboss_seqret) | [SeqKit](https://bioinf.shenwei.me/seqkit/) | [FASTX Toolkit](http://hannonlab.cshl.edu/fastx_toolkit/) |
| --- | :---:| :---: | :---: | :---: | :---: |
Ensures single EOF newline character (multiple present) | 1 | :white_check_mark: | :white_check_mark: | :white_check_mark: | |	
Ensures single EOF newline character (none present) | 2 | :white_check_mark: | :white_check_mark: | :white_check_mark: | :white_check_mark: |
Parses and removes empty lines | 3 | :white_check_mark: | :white_check_mark: |:white_check_mark: | |	
Parses and removes non-printable carriage return | 5 | :white_check_mark: | :white_check_mark: | :white_check_mark: | :white_check_mark: |
Parses and removes non-printable horizontal tab | 9 | :white_check_mark: | :white_check_mark: | | |		
Parses multibyte character set encoded files (e.g. UTF-16) | 4 |:white_check_mark: | | | |			
Removes duplicate fasta header markers | 6 | :white_check_mark: | :white_check_mark: | :green_circle: | :white_check_mark: |
Removes file content leading the first header mark | 7 | :white_check_mark: | | :green_circle: | |	
Removes line wrapping | 8 | :white_check_mark: | :white_check_mark: | :white_check_mark: | |	
Renames header lines | NA | :white_check_mark: | | :white_check_mark: | :white_check_mark: |
Saves persistent mapping of old to new headers | NA | :white_check_mark: | | | |  

:green_circle: Requires external regex pattern
