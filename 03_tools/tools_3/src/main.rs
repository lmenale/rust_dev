// Rust Application: Rustfmt/Clippy Showcase
//
// This example demonstrate code formatting with Rustfmt and linting with Clippy involves using these tools on your codebase.
// In this example:
//
// Create a Rust code file main.rs with some code snippets that may have varying styles and potential issues.
// Use the rustfmt command to format the code according to Rustfmt's defined style, ensuring consistent formatting.
// Use the cargo clippy command to run Clippy on the code, identifying potential problems or non-idiomatic patterns.

// Please note that this example is simplified for demonstration purposes and doesn't cover all aspects of code formatting
// and linting. Real-world applications may involve more complex code and detailed linting results.

mod lib_clippy;

fn main() {
    let x=5;
    let y = 10;

    if x<y {
        println!("x is less than y");
    } else {
        println!("x is greater than or equal to y");
    }

let mut sum = 0;
for i in 1..=10 {
    sum += i;
}
    println!("Sum of numbers from 1 to 10: {}", sum);

    let greeting = "Hello, world!";
    println!("{}", greeting);

    lib_clippy::clippy_test();
}
