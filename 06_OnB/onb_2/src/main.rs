// Rust Application: Borrowing Rules and References Showcase
// 
// This example demonstrates borrowing and references, highlighting Rust's strict borrowing rules to prevent data races.
// In this example:
// 
// - Demonstrate mutable and immutable borrowing using references.
// - Show how Rust's borrowing rules prevent multiple mutable references at the same time.
// - Use a simple example to highlight how Rust prevents data races using Arc and Mutex for thread-safe concurrency.
// 
// Compile and run this Rust code to observe how Rust's borrowing and reference system ensures safe data sharing and
// prevents data races.

fn main() {
    let mut data = vec![1, 2, 3, 4, 5];

    // Mutable borrowing: Modify borrowed data
    let mut_borrowed = &mut data;
    mut_borrowed.push(6);

    println!("Modified Borrowed Data: {:?}", mut_borrowed);

    // The following line would result in a compilation error, as we can't have multiple mutable references at once.
    // let another_mut_borrowed = &mut data;

    // Immutable borrowing: Read-only access
    let immut_borrowed = &data;

    // Mutable borrowing again after the immutable reference goes out of scope
    drop(immut_borrowed);
    let another_mut_borrowed = &mut data;

    println!("Another Modified Borrowed Data: {:?}", another_mut_borrowed);

    // Borrowing rules enforced at compile-time
    // The following lines would result in compilation errors due to borrowing rules violations.
    // let immut_borrowed = &data;
    // let mut_borrowed = &mut data;

    // Demonstrating the prevention of data races
    use std::sync::{Arc, Mutex};
    let counter = Arc::new(Mutex::new(0));

    let mut handles = vec![];

    for _ in 0..10 {
        let counter_clone = Arc::clone(&counter);
        let handle = std::thread::spawn(move || {
            let mut data = counter_clone.lock().unwrap();
            *data += 1;
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Counter value: {}", *counter.lock().unwrap());
}
