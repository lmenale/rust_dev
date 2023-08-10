// Rust Application: Organizing Tests Showcase
// 
// This example demonstrates how to organize and run tests using Cargo.
// In this example:
// 
// - Define some functions (add, subtract, and multiply) and their corresponding test functions.
// - Organize the tests into separate modules (tests and more_tests) for clarity.
// - Running cargo test compiles and executes all the test functions, reporting the results.
// 
// By using Cargo's testing capabilities, you can easily organize and run tests for your Rust code, ensuring comprehensive
// code coverage and catching regressions early in the development process.

// mod more_tests;  // TODO: remove this comment

pub fn add(a: i32, b: i32) -> i32 {
    a + b
}

pub fn subtract(a: i32, b: i32) -> i32 {
    a - b
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add() {
        assert_eq!(add(2, 3), 5);
    }

    #[test]
    fn test_subtract() {
        assert_eq!(subtract(5, 3), 2);
    }
}
