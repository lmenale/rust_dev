// Rust Application: Control flow Showcase
// 
// This example covers the key concepts of control flow, including if statements, loops, and match statements.
// In this example:
// 
// Use an if statement to make a decision based on the value of the age variable.
// Demonstrate while and for loops to repeatedly execute code while certain conditions hold.
// Use a match statement to efficiently handle different values of the fruit variable.
// 
// Please note that this example is simplified for demonstration purposes and doesn't cover all possible variations and
// use cases of control flow statements. Real-world applications may involve more complexity and features.

fn main() {
    // If Statements: Making decisions based on conditions
    let age = 18;
    if age >= 18 {
        println!("You are an adult.");
    } else {
        println!("You are a minor.");
    }

    // Loops: Repeatedly executing code while conditions hold
    let mut count = 0;
    while count < 5 {
        println!("Count: {}", count);
        count += 1;
    }

    for number in 1..=3 {
        println!("Number: {}", number);
    }

    // Match Statements: Efficiently handling multiple conditions
    let fruit = "apple";
    match fruit {
        "apple" => println!("It's an apple."),
        "banana" => println!("It's a banana."),
        _ => println!("It's something else."),
    }
}
