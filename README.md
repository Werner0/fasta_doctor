# fasta_doctor  

### Usage
Linux: `fasta_doctor_x86_64 <input.fasta>`

### Output
`output.fasta`

### What it does:
+ Ensures file starts with a ">" and ends with a newline.
+ Removes all non-printable control characters in the hexadecimal range 00-1F (except 0A).
