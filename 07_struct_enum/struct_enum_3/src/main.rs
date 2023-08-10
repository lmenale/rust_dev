// Rust Application: Pattern Matching with Enums Showcase
// 
// This example demonstrates the use of pattern matching to efficiently handle enum variants.
// In this example:
// 
// - Define an enum Shape with variants Circle, Rectangle, and Triangle.
// - Provide a method calculate_area to calculate the area of each shape based on its variant.
// - In the main function, we create instances of different shapes and call the calculate_area method on each.
//
// Compile and run this Rust code to see how pattern matching is used to efficiently handle different enum variants and
// execute tailored actions based on the shape's structure and form.

// Define an enum to represent different shapes
enum Shape {
    Circle(f64),
    Rectangle(f64, f64),
    Triangle(f64, f64, f64),
}

impl Shape {
    // Method to calculate area based on shape variant
    fn calculate_area(&self) -> f64 {
        match self {
            Shape::Circle(radius) => std::f64::consts::PI * radius * radius,
            Shape::Rectangle(width, height) => width * height,
            Shape::Triangle(a, b, c) => {
                let s = (a + b + c) / 2.0;
                (s * (s - a) * (s - b) * (s - c)).sqrt()
            }
        }
    }
}

fn main() {
    // Create instances of different shapes
    let circle = Shape::Circle(3.0);
    let rectangle = Shape::Rectangle(4.0, 5.0);
    let triangle = Shape::Triangle(3.0, 4.0, 5.0);

    // Calculate and print the area of each shape
    println!("Circle area: {}", circle.calculate_area());
    println!("Rectangle area: {}", rectangle.calculate_area());
    println!("Triangle area: {}", triangle.calculate_area());
}
