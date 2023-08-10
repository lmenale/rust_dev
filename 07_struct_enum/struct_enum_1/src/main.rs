// Rust Application: Custom Data Types with Structs Showcase
// 
// This example demonstrates the use of structs to model complex data structures and related types.
// In this example:
// 
// - Define a struct Person with fields name, age, and is_student.
// - Provide a constructor new to create instances of Person.
// - Implement a method introduce to introduce the person based on their data.
// - In the main function, we create two Person instances and call the introduce method on each.
// 
// Compile and run this Rust code to observe how structs are used to model complex data structures, enabling code organization,
// abstraction, and better data clarity.

// Define a struct to represent a person
struct Person {
    name: String,
    age: u32,
    is_student: bool,
}

impl Person {
    // Constructor for creating a new Person instance
    fn new(name: &str, age: u32, is_student: bool) -> Self {
        Person {
            name: String::from(name),
            age,
            is_student,
        }
    }

    // Method to introduce the person
    fn introduce(&self) {
        println!(
            "Hi, I'm {} and I'm {} years old. I am{} a student.",
            self.name,
            self.age,
            if self.is_student { "" } else { " not" }
        );
    }
}

fn main() {
    // Create instances of the Person struct
    let person1 = Person::new("Alice", 25, true);
    let person2 = Person::new("Bob", 30, false);

    // Introduce the persons
    person1.introduce();
    person2.introduce();
}
