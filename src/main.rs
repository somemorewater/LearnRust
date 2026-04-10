use std::char;

fn main() {
    // Output
    println!("Hello, world!");

    // Variables
    let name = "Water";
    println!("My name is {}", name);

    // Changeable variables
    let mut age = 100;
    println!("My age is {}", age);
    age = 101;
    println!("My age is now {}", age);

    // Data types
    let number: i32 = 5; // integer
    let pi: f64 = 3.14; // floating-point number
    let character: char = 'A'; // character
    let boolean: bool = true; // boolean
    let text: &str = "Hello World"; // String
    println!("Number: {}, Pi: {}, Character: {}, Boolean: {}, Text: {}", number, pi, character, boolean, text);

    // Numbers
    let height: i32 = 180;
    println!("Height is: {}cm", height);

    // Floating Point(f64)
    let g: f64 = 9.81;
    println!("Acceleration due to gravity is: {} m/s²", g);

    // Characters(char)
    let currency: char = '$';
    println!("I go soon chop k{}", currency);

    // Strings(&str)
    let info: &str = "Hello World";
    println!("{}", info);

    // Booleans(bool)
    let is_me: bool = true;
    println!("Is this me? {}", is_me);

    // Constants
    const PI: f64 = 3.142;
    println!("The value of PI is: {}", PI);

    // Arithmetic operations
    let sum = 5 + 3; // Addition
    let difference = 10 - 4; // Subtraction
    let product = 6 * 7; // Multiplication
    let division = 20 / 5; // Division
    let remainder = 10 % 3; // Modulus
    println!("Sum: {}, Difference: {}, Product: {}, Division: {}, Remainder: {}", sum, difference, product, division, remainder);

    //Assignment operators
    let mut x = 5;
    println!("Start: {}", x);
    
    x += 3;
    println!("After addition: {}", x);

    x -= 2;
    println!("After subtraction: {}", x);

    x *= 4;
    println!("After multiplication: {}", x);

    x /= 2;
    println!("After division: {}", x);

    x %= 3;
    println!("After modulus: {}", x);

    // Comparison operators
    let a = 10;
    let b = 20;

    println!("Is a equal to b? {}", a == b);
    println!("Is a not equal to b? {}", a != b);
    println!("Is a greater than b? {}", a > b);
    println!("Is a less than b? {}", a < b);
    println!("Is a greater than or equal to b? {}", a >= b);
    println!("Is a less than or equal to b? {}", a <= b);

    // Logical operators
    let tuff = true;
    let not_tuff = false;

    println!("Codes in Rust: {}", tuff && not_tuff); // Logical AND
    println!("Codes in Rust: {}", tuff || not_tuff); // Logical OR
    println!("Codes in Rust: {}", !tuff); // Logical NOT
}
