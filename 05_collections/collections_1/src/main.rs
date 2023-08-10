// Rust Application: Arrays and Slices Showcase
// 
// This example demonstrates the concepts of arrays and slices by performing basic operations.
// In this example:
// 
// - Define an array numbers containing five integers.
// - Access elements of the array using index notation.
// - Create a slice slice from the array, demonstrating how slices provide a dynamic view into a portion of the data.
// - Iterate over the array using a for loop and print each element.
// - Modify an element of the array using a mutable reference, showcasing how arrays can be modified when mutable.
// 
// Compile and run this Rust code to observe the behavior of arrays and slices and their characteristics, such as
// efficient memory management, direct access, and dynamic views into data.

fn main() {
    // Creating an array of integers
    let numbers: [i32; 5] = [1, 2, 3, 4, 5];

    // Accessing elements of the array
    println!("First element: {}", numbers[0]);
    println!("Third element: {}", numbers[2]);

    // Creating a slice from the array
    let slice: &[i32] = &numbers[1..4];
    println!("Slice: {:?}", slice);

    // Iterating over the array using a for loop
    for number in &numbers {
        println!("Number: {}", number);
    }

    // Modifying elements of the array using a mutable reference
    let mut mutable_numbers = numbers;
    mutable_numbers[1] = 20;
    println!("Modified array: {:?}", mutable_numbers);
}
