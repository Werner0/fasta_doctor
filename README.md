# fasta_doctor  
### Description
A command-line tool to repair and clean FASTA files.

### Usage
Linux: `fasta_doctor_x86_64 <input.fasta> [--rename] [--unwrap]`.

### Output
+ Fixed FASTA file: `output.fasta`.
+ FASTA header mapping file: `mapping.fasta_doctor` (only created with `--rename`).

### What fasta_doctor does by default
+ Ensures FASTA file starts with a ">" and ends with a newline.  
+ Removes all non-printable control characters in the hexadecimal range 00-1F (except 0A).

### What fasta_doctor does optionally
+ Renames FASTA headers using the alphanumeric pattern A[n]B (e.g. A0001B). Requires `--rename`.
+ Creates a persistent mapping from old to new FASTA headers. Requires `--rename`.
+ Removes line wrapping. Requires `--unwrap`.

### Example
The following FASTA file content includes a comment line, inconsistent header names, line wrapping and non-printable carriage returns.
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
Running `fasta_doctor_x86_64 windows.fasta --rename --unwrap` yields:
```
>A0B
ACTGGTCA
>A1B
ACTGGTCA
```
with an additional file mapping old to new header names.
