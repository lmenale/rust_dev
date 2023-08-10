// Rust Application: RustDoc Showcase
// 
// This example demonstrate documentation generation with Rustdoc involves creating comments within your code and
// then generating HTML documentation using Rustdoc.
// In this example:
// 
// - Create a Rust module named rustdoc_demo with a function greet to demonstrate documentation generation.
// - Use Rustdoc-style comments (///) to provide inline documentation for the module and function.
// - Generate the HTML documentation using the rustdoc command, and the resulting documentation can be browsed in a web browser.

// Please note that this example is simplified for demonstration purposes and doesn't cover all aspects of documentation
// generation and Markdown usage. Real-world documentation may involve more detailed explanations and examples.

use tools_2::rustdoc_demo::greet;

fn main() {
    let message = greet("Alice");
    println!("{}", message);
}
