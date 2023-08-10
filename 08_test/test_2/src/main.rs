// Rust Application: UT Attributes and Assertions Showcase
// 
// This example demonstrates the use of test attributes, assertions, and how they help check if code produces the desired results.
// In this example:
// 
// - Define a factorial function that calculates the factorial of a non-negative integer.
// - Use test attributes (#[test]) to mark functions as test cases. These functions will be executed when running tests
// with cargo test.
// - The assert_eq! macro is used within the test functions to check if the actual output of the factorial function
// matches the expected output.
// - The tests cover different scenarios, including calculating the factorial of zero and positive integers.
// 
// To run the tests, use the cargo test command in your terminal. The test framework will execute the test functions and
// report whether the assertions pass or fail. This allows you to verify the code's behavior, detect bugs, and ensure that
// the desired outcomes are produced.

// Assertions play a crucial role in validating your code's correctness and ensuring that it behaves as intended.
// They provide a powerful tool for confidently validating your code during development.

// A function to calculate the factorial of a number
fn factorial(n: u32) -> u32 {
    if n == 0 {
        1
    } else {
        n * factorial(n - 1)
    }
}

// Unit tests for the factorial function
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_factorial_zero() {
        assert_eq!(factorial(0), 1);
    }

    #[test]
    fn test_factorial_positive() {
        assert_eq!(factorial(5), 120);
        assert_eq!(factorial(7), 5040);
    }
}

fn main() {
    println!("Running tests...");
    // This line is not necessary in a real application.
    // Cargo automatically compiles and runs tests.
}
