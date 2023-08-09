// Rust Application: Memory safety Showcase
// 
// This example demonstrates defining functions, using parameters and return values, and organizing code into modules.
// In this example:
// 
// Define a module named math that contains the add and subtract functions.
// Use the pub keyword to make the add function public, allowing it to be accessed from outside the module.
// Demonstrate using the add function from the math module to compute the sum of two numbers.
// Attempt to use the subtract function from the math module, but it results in an error because
// the function is private within the module.
// 
// Please note that this example is simplified for demonstration purposes and doesn't cover all possible variations and
// use cases of functions and modules. Real-world applications may involve more complexity and features.

// Use a module named 'lib'
mod lib_math;

fn main() {
    let num1 = 10;
    let num2 = 5;

    // Using the add function from the lib module
    let sum = lib_math::add(num1, num2);
    println!("Sum: {}", sum);

    // Error: subtract function is private within the lib module
    // let diff = lib_math::subtract(num1, num2);
    // println!("Difference: {}", diff);
}
