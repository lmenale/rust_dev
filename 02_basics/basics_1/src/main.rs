// Rust Application: Immutability Showcase
// 
// This example demonstrates variable declaration, data types, and immutability.
// In this example:
// 
// Declare and initialize variables of various data types, including integers, floats, Booleans, characters, and strings.
// Define a constant MAX_SCORE and use it for basic type conversions.
// Demonstrate the concept of immutability by attempting to reassign an immutable variable, showcasing Rust's safety features.
// 
// Please note that this is a simplified example for illustrative purposes, and real-world applications may involve
// more complexity and features.

fn main() {
    // Declare and initialize variables
    let age: u32 = 25;
    let height: f64 = 1.75;
    let is_student: bool = true;
    let first_initial: char = 'J';
    let city: String = String::from("New York");

    // Constants
    const MAX_SCORE: u32 = 100;

    // Basic type conversions
    let score: u32 = 85;
    let average: f64 = (score as f64) / (MAX_SCORE as f64);

    println!("Age: {}", age);
    println!("Height: {} meters", height);
    println!("Is Student: {}", is_student);
    println!("First Initial: {}", first_initial);
    println!("City: {}", city);
    println!("Maximum Score: {}", MAX_SCORE);
    println!("Average Score: {:.2}", average);

    // Immutable variables by default
    let language = "Rust";  // Immutable by default
    // language = "Python";  // Uncommenting this line will result in an error

    println!("Preferred Language: {}", language);
}
