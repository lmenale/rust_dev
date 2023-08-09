// Rust Application: Concurrency Showcase
// 
// This example demonstrates a basic Rust application that showcases concurrency using threads and a channel for communication.
// It incorporates the principles of performance, memory safety, and concurrency emphasized in Rust:
// 
// 1. Concurrency:
// The application spawns multiple threads to perform tasks concurrently, simulating a time-consuming operation.
//
// 2. Memory Safety:
// Rust's ownership and borrowing rules are implicitly demonstrated as threads share ownership of the channel
// sender (tx_clone) and safely communicate results.
// 
// 3. Performance:
// The application uses multiple threads to execute tasks simultaneously, enhancing overall performance.
// 
// 4. Preventing Segfaults:
// Rust's ownership and borrowing system prevents data races and ensures thread safety.
// 
// Please note that this example is simplified for demonstration purposes and does not cover all aspects of
// Rust programming or error handling. A complete Rust application would typically involve more comprehensive code,
// error handling, and potentially more sophisticated concurrency patterns.

// Import the necessary modules
use std::thread;
use std::sync::mpsc;

// Define a function to simulate a time-consuming task
fn perform_task(task_id: i32) -> i32 {
    println!("Task {} started.", task_id);
    // Simulate a time-consuming task (e.g., processing data)
    thread::sleep(std::time::Duration::from_secs(2));
    println!("Task {} completed.", task_id);
    task_id * 10
}

fn main() {
    println!("Rust Concurrency Showcase");

    // Create a vector to store thread handles
    let mut handles = vec![];

    // Create a channel for communication between threads
    let (tx, rx) = mpsc::channel();

    // Spawn multiple threads to perform tasks concurrently
    for i in 0..5 {
        let tx_clone = tx.clone();
        let handle = thread::spawn(move || {
            let result = perform_task(i);
            tx_clone.send(result).unwrap();
        });
        handles.push(handle);
    }

    // Collect results from threads
    let mut results = vec![];
    for _ in 0..5 {
        let result = rx.recv().unwrap();
        results.push(result);
    }

    // Summarize results
    println!("Task results: {:?}", results);

    // Wait for all threads to complete
    for handle in handles {
        handle.join().unwrap();
    }

    println!("All tasks completed.");
}
