// Rust Application: Box, Rc, and Arc Showcase
// 
// This example demonstrates the use of smart pointers.
// In this example:
// 
// - Demonstrate the use of Box to allocate data on the heap.
// - Show how Rc enables shared ownership within a single thread.
// - Highlight Arc for shared ownership across multiple threads.
// 
// Compile and run this Rust code to observe how smart pointers (Box, Rc, and Arc) provide various ownership semantics and
// memory management strategies to ensure memory safety and efficient data sharing in different scenarios.

use std::rc::Rc;
use std::sync::Arc;

#[derive(Debug)]struct Data {
    value: i32,
}

fn main() {
    // Box: Allocate data on the heap
    let boxed_data = Box::new(Data { value: 42 });
    println!("Boxed Data: {:?}", boxed_data);

    // Rc: Shared ownership with reference counting (Single-threaded)
    let rc_data = Rc::new(Data { value: 69 });
    let rc_clone1 = Rc::clone(&rc_data);
    let rc_clone2 = Rc::clone(&rc_data);
    println!("Rc Data: {:?}", rc_data);
    println!("Rc Clone 1: {:?}", rc_clone1);
    println!("Rc Clone 2: {:?}", rc_clone2);

    // Arc: Shared ownership across threads (Multi-threaded)
    let arc_data = Arc::new(Data { value: 123 });
    let arc_clone1 = Arc::clone(&arc_data);
    let arc_clone2 = Arc::clone(&arc_data);
    println!("Arc Data: {:?}", arc_data);
    println!("Arc Clone 1: {:?}", arc_clone1);
    println!("Arc Clone 2: {:?}", arc_clone2);
}
