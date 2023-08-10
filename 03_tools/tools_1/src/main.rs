// Rust Application: Cargo Showcase
// 
// This example uses Cargo to manage dependencies and build the project.
// In this example:
// 
// - Use Cargo to manage dependencies by adding the rand crate as a dependency in the Cargo.toml file.
// - Import the rand crate and use it to generate a random number between 1 and 10.
// - Build and run the application using the cargo run command, which handles the compilation, linking, and execution process for us.
// 
// Please note that this example is simplified for demonstration purposes and doesn't cover all aspects of using Cargo
// for managing dependencies and building projects. Real-world applications may involve more complexity and features.

use rand::Rng;

fn main() {
    let random_number = rand::thread_rng().gen_range(1..=10);

    println!("Random number between 1 and 10: {}", random_number);
}
