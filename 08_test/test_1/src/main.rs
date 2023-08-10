// Rust Application: Unit Tests and Built-in Testing Framework Showcase
// 
// This example demonstrates the use of Rust's built-in testing framework to verify code meets specifications and
// catch regression bugs early.
// In this example:
// 
// - Define a simple add function that takes two integers and returns their sum.
// - Use Rust's built-in testing framework to write unit tests for the add function. Each test is marked with the #[test] attribute.
// - The assert_eq! macro is used to compare the expected result with the actual result of the add function.
// - The tests module contains multiple test functions to validate different scenarios.
// 
// To run the tests, you can use the cargo test command in your terminal. It will compile and execute the tests,
// providing feedback on whether the code meets specifications and whether any regression bugs are detected.
// 
// Remember, systematic testing is essential for verifying code behavior, improving code quality, and ensuring
// the reliability of your software.

// A simple function to add two numbers
fn add(a: i32, b: i32) -> i32 {
    a + b
}

// Unit test for the add function
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_positive_numbers() {
        assert_eq!(add(2, 3), 5);
    }

    #[test]
    fn test_add_negative_numbers() {
        assert_eq!(add(-5, -3), -8);
    }

    #[test]
    fn test_add_mixed_numbers() {
        assert_eq!(add(-10, 15), 5);
    }
}

fn main() {
    println!("Running tests...");
    // This line is not necessary in a real application.
    // Cargo automatically compiles and runs tests.
}
