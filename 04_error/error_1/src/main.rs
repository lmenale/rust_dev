// Rust Application: Result/Option Showcase
// 
// This example uses Cargo to manage dependencies and build the project.
// In this example:
// 
// - The read_file_contents function reads the contents of a file and returns a Result<String, Error>, where Ok variant contains
// the file contents and Err variant contains an error.
// - The find_element_in_array function searches for an element in an array and returns an Option<usize>, where Some variant
// contains the index of the found element and None variant indicates absence.
// - The main function demonstrates how to use Result and Option to handle errors and absence of values in a file reading
// scenario and an array searching scenario.

use std::fs::File;
use std::io::{Read, Error};

fn read_file_contents(filename: &str) -> Result<String, Error> {
    let mut file = File::open(filename)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    Ok(contents)
}

fn find_element_in_array(arr: &[i32], target: i32) -> Option<usize> {
    for (index, &value) in arr.iter().enumerate() {
        if value == target {
            return Some(index);
        }
    }
    None
}

fn main() {
    // Example of reading a file and handling errors with Result
    match read_file_contents("example.txt") {
        Ok(contents) => println!("File contents: {}", contents),
        Err(error) => eprintln!("Error reading file: {}", error),
    }

    // Example of finding an element in an array and handling absence with Option
    let numbers = [1, 2, 3, 4, 5];
    let target = 3;

    match find_element_in_array(&numbers, target) {
        Some(index) => println!("Found {} at index {}", target, index),
        None => println!("{} not found in the array", target),
    }
}
