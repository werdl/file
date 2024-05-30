# file
> An alternative filesystem API for Rust, for simplity.

## Usage
```rust
use file::File; // all functions are implemented on File struct
use file::FileOptions; // options for file operations, a bitflag but can also be used like a builder


// read a file
let file: File = FileOptions::Read.open("README.md").unwrap();

let u8_vec: Vec<u8> = file.read().unwrap();
let string: String = file.read_to_string().unwrap();


// read to a file, then write it back prefixed with "Hello, "
let file: File = (FileOptions::Read | FileOptions::Write).open("README.md").unwrap();

let mut string: String = file.read_to_string().unwrap();

string = format!("Hello, {}", string);

let _ = file.seek(0);

file.write(string.as_bytes()).unwrap();


// create a file and write to it
let file: File = FileOptions::new()
                    .write(true)
                    .create(true)
                    .open("hello.txt");

let _ = file.write("Hello, world!");
```