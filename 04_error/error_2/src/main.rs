// Rust Application: Custom error type Showcase
// 
// This example demonstrates the concepts of custom error types and error handling strategies using a simple file reading scenario.
// In this example:
// 
// - Define a custom error type FileReadError that implements the Error trait, allowing us to create more meaningful error structures.
// - From trait is implemented to convert IoError into our custom error type, FileReadError.
// - main function demonstrates how to use the custom error type FileReadError to handle different error scenarios, such as
// file not found or IO errors.
// 
// Compile and run this Rust code to observe how custom error types and error handling strategies can be used to create
// more informative and tailored error handling in your application.

use std::fs::File;
use std::io::{Read, Error as IoError, ErrorKind};

// Define a custom error type
#[derive(Debug)]
enum FileReadError {
    NotFound,
    Io(IoError),
}

impl From<IoError> for FileReadError {
    fn from(error: IoError) -> Self {
        FileReadError::Io(error)
    }
}

impl std::error::Error for FileReadError {}

impl std::fmt::Display for FileReadError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            FileReadError::NotFound => write!(f, "File not found"),
            FileReadError::Io(inner) => inner.fmt(f),
        }
    }
}

// Function that reads a file and returns a Result with custom error type
fn read_file_contents(filename: &str) -> Result<String, FileReadError> {
    let mut file = File::open(filename)?;

    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    Ok(contents)
}

fn main() {
    match read_file_contents("example.txt") {
        Ok(contents) => println!("File contents: {}", contents),
        Err(error) => match error {
            FileReadError::NotFound => eprintln!("Error: File not found"),
            FileReadError::Io(inner) => eprintln!("IO Error: {}", inner),
        },
    }
}
