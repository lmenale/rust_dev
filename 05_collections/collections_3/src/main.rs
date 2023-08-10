// Rust Application: Iterators and Collections Showcase
// 
// This example demonstrates the use of iterators and iterator adapters to traverse and manipulate collections.
// In this example:
// 
// - Use a for loop and a while let loop to iterate over the vector.
// - Demonstrate various iterator adapters like map, filter, take, skip, and zip.
// - Show how iterators enable functional programming patterns, allowing for concise and expressive code.
// 
// Compile and run this Rust code to see how iterators and iterator adapters can be used to traverse and manipulate collections
// with ease.

fn main() {
    // Create a vector of numbers
    let numbers = vec![1, 2, 3, 4, 5];

    // Using a for loop to iterate over the vector
    for num in &numbers {
        println!("Number: {}", num);
    }

    // Using a while let loop to iterate and print even numbers
    let mut iter = numbers.iter();
    while let Some(num) = iter.next() {
        if num % 2 == 0 {
            println!("Even Number: {}", num);
        }
    }

    // Using the `map` iterator adapter to double the numbers
    let doubled: Vec<i32> = numbers.iter().map(|&x| x * 2).collect();
    println!("Doubled: {:?}", doubled);

    // Using the `filter` iterator adapter to keep even numbers
    let even_numbers: Vec<i32> = numbers.iter().filter(|&&x| x % 2 == 0).cloned().collect();
    println!("Even Numbers: {:?}", even_numbers);

    // Using the `take` iterator adapter to get the first two elements
    let first_two: Vec<i32> = numbers.iter().take(2).cloned().collect();
    println!("First Two: {:?}", first_two);

    // Using the `skip` iterator adapter to skip the first three elements
    let skip_three: Vec<i32> = numbers.iter().skip(3).cloned().collect();
    println!("Skip Three: {:?}", skip_three);

    // Using the `zip` iterator adapter to combine two vectors
    let other_numbers = vec![10, 20, 30, 40, 50];
    let combined: Vec<_> = numbers.iter().zip(other_numbers.iter()).collect();
    println!("Combined: {:?}", combined);
}
