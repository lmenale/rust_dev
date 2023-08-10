// Rust Application: Ownership Model Showcase
// 
// This example demonstrates Rust's ownership model and how it prevents common memory-related bugs.
// In this example:
// 
// - Demonstrate ownership transfer and borrowing to showcase how Rust's ownership model works.
// - Highlight how Rust's ownership system prevents common memory-related bugs, such as use-after-free and double frees.
// - Illustrate the scope-based automatic memory deallocation.
// - Show how compile-time checks catch ownership and borrowing violations.
// 
// Compile and run this Rust code to observe how Rust's ownership system prevents memory-related bugs and ensures robust and
// secure code.

fn main() {
    // Ownership and memory deallocation
    let owner = String::from("Hello, Rust!");
    println!("Owner: {}", owner);

    // Transfer ownership
    let new_owner = owner;
    // The following line would result in a compilation error because `owner` no longer owns the string.
    // println!("Owner after transfer: {}", owner);

    // Borrowing and lifetime management
    let borrowed = &new_owner;
    println!("Borrowed: {}", borrowed);

    // This would also result in a compilation error, as `borrowed` is still in use when trying to modify `new_owner`.
    // new_owner.push_str(" Welcome!");

    // Scope and ownership drop
    {
        let local_owner = String::from("Scoped owner");
        println!("Local Owner: {}", local_owner);
    } // `local_owner` goes out of scope and its memory is automatically deallocated

    // Demonstrating the prevention of common bugs
    let s1 = String::from("hello");
    let s2 = s1; // Moves ownership from s1 to s2

    // The following line would result in a compilation error, as `s1` no longer owns the string.
    // println!("s1: {}", s1);

    println!("s2: {}", s2);

    // Ownership ensures that when `s2` goes out of scope, its memory is automatically deallocated.

    // Memory safety and compile-time checks
    let mut vec = Vec::new();
    vec.push(1);
    vec.push(2);
    vec.push(3);

    // This line would result in a compilation error, as the reference `&vec` prevents modifying the vector while it's borrowed.
    // vec.push(4);

    println!("Vector: {:?}", vec);
}
