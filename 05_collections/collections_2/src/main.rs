// Rust Application: Vectors and Strings Showcase
// 
// This example demonstrates the concepts of vectors and strings by performing various operations.
// In this example:
// 
// - Create a dynamic vector numbers and add elements using the push method.
// - Create a dynamic string hello and manipulate it using various string-related operations.
// - Search for a substring using the contains method.
// - Slice a string to extract a portion of it.
// - Join strings using the format macro.
// - Remove an element from the vector using the pop method.
// 
// Compile and run this Rust code to observe the behavior of vectors and strings, their dynamic nature, and their capabilities
// in handling variable-sized and textual data.

fn main() {
    // Creating a vector and adding elements
    let mut numbers: Vec<i32> = Vec::new();
    numbers.push(1);
    numbers.push(2);
    numbers.push(3);

    // Printing the vector
    println!("Vector: {:?}", numbers);

    // Creating a string
    let mut hello = String::from("Hello, ");
    hello.push_str("Rust!");

    // Printing the string
    println!("{}", hello);

    // Searching for a substring in a string
    if hello.contains("Rust") {
        println!("Found 'Rust' in the string.");
    }

    // Slicing a string
    let greeting = &hello[0..7];
    println!("Greeting: {}", greeting);

    // Joining strings
    let name = String::from("Alice");
    let message = format!("{} {}", greeting, name);
    println!("{}", message);

    // Removing an element from the vector
    numbers.pop();

    // Printing the updated vector
    println!("Updated Vector: {:?}", numbers);
}
