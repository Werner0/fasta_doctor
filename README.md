# fasta_doctor  
### Description
A command-line tool to repair and clean [FASTA files](https://en.wikipedia.org/wiki/FASTA_format).

### Installation 
Option 1: A pre-built version for Linux is included in this repository.
```
git clone https://github.com/Werner0/fasta_doctor.git
chmod a+x fasta_doctor_x86_64
```
Option 2: Compile it yourself (requires [Rust and Cargo](https://doc.rust-lang.org/cargo/getting-started/installation.html)).
```
git clone https://github.com/Werner0/fasta_doctor.git
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
+ Ensures the input FASTA file starts with a carat (">") and ends with a newline character ("\n"). 
+ Removes all non-printable control characters in the hexadecimal range 00-1F (except 0A).

### What fasta_doctor does optionally
+ Renames FASTA headers using the alphanumeric pattern A[n]B (e.g. A0001B). Requires `--rename`.
+ Creates a persistent mapping from old to new FASTA headers. Requires `--rename`.
+ Removes line wrapping. Requires `--unwrap`.

### Example
The following content from a FASTA file contains a comment line, an empty line, inconsistent header names and line wrapping. The [original FASTA file](/example/windows.fasta) also contains non-printable carriage returns ("\r") because it was created in a text editor in Windows.
```
#Fasta
>Clone@1 | test 1
ACTG
GTCA

>Clone2
ACTG
G
T
C
A
```
Running `fasta_doctor_x86_64 input.fasta --rename --unwrap` yields:
```
>A0B
ACTGGTCA
>A1B
ACTGGTCA
```
Old headers are mapped to new headers and saved in `mapping.fasta_doctor`.
