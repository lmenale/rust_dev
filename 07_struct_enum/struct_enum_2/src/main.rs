// Rust Application: Data Types with Enums Showcase
// 
// This example demonstrates the use of enums to define a type by enumerating its possible values.
// In this example:
// 
// - Define an enum Vehicle with variants Car, Motorcycle, Bicycle, and Bus.
// - Provide a method describe to describe each vehicle based on its variant.
// - In the main function, we create instances of the Vehicle enum and call the describe method on each.
// 
// Compile and run this Rust code to see how enums are used to represent different possible values, fostering code flexibility and
// enabling the definition of diverse, related types.

// Define an enum to represent different types of vehicles
enum Vehicle {
    Car,
    Motorcycle,
    Bicycle,
    Bus,
}

impl Vehicle {
    // Method to describe the vehicle
    fn describe(&self) -> &'static str {
        match self {
            Vehicle::Car => "This is a car.",
            Vehicle::Motorcycle => "This is a motorcycle.",
            Vehicle::Bicycle => "This is a bicycle.",
            Vehicle::Bus => "This is a bus.",
        }
    }
}

fn main() {
    // Create instances of the Vehicle enum
    let car = Vehicle::Car;
    let motorcycle = Vehicle::Motorcycle;
    let bicycle = Vehicle::Bicycle;
    let bus = Vehicle::Bus;

    // Describe the vehicles
    println!("{}", car.describe());
    println!("{}", motorcycle.describe());
    println!("{}", bicycle.describe());
    println!("{}", bus.describe());
}
