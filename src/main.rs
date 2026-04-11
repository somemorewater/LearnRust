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

    // Boolean
    let is_rust_fun: bool = true;
    println!("Is Rust fun? {}", is_rust_fun);

    // Conditional statements
    // if statement
    let number = 10;
    if number > 5 {
        println!("Number is greater than 5");
    }

    // if-else statement
    let age = 18;
    if age >= 18 {
        println!("You are an adult");
    } else {
        println!("You are a minor");
    }

    // if-else if-else statement
    let score = 85;
    if score >= 90 {
        println!("Grade: A");
    } else if score >= 80 {
        println!("Grade: B");
    } else if score >= 70 {
        println!("Grade: C");
    } else if score >= 60 {
        println!("Grade: D");
    } else {
        println!("Grade: F");
    }

    // is as an expression
    let value = 10;
    let result = if value > 5 { "Greater than 5" } else { "5 or less" };
    println!("Result: {}", result);

    // Match
    let day = 3;
    match day {
        1 => println!("Monday"),
        2 => println!("Tuesday"),
        3 => println!("Wednesday"),
        4 => println!("Thursday"),
        5 => println!("Friday"),
        6 => println!("Saturday"),
        7 => println!("Sunday"),
        _ => println!("Invalid day"),
    }

    // Multiple Matches
    let number = 2;
    match number {
        1 | 3 | 5 | 7 | 9 => println!("Odd number"),
        2 | 4 | 6 | 8 | 10 => println!("Even number"),
        _ => println!("Not between 1 and 10"),
    }

    //loop

    loop {
        println!("This will loop forever!");
        break; // To prevent infinite loop
    }

    // While loop
    let mut count = 0;
    while count < 5 {
        println!("Count: {}", count);
        count += 1;
    }

    // for loop
    for i in 0..5 {
        println!("Index: {}", i);
    }

    // Break and continue
    for i in 0..10 {
        if i == 3 {
            continue; // Skip the rest of the loop when i is 3
        }
        if i == 5 {
            break;
        }
        println!("Index: {}", i);
    }

    // Functions
    fn hello() {
        println!("Hello World");
    }

    hello();

    // Function with return value
    fn add(a: i32, b: i32) -> i32 { 
        return a + b
    }

    let sum = add(5, 3);
    println!("Sum: {}", sum);

    // Strings
    let greeting: &str = "Hello, Rust!";
    println!("{}", greeting);

    // Change a string
    let mut mutable_greeting = String::from("Hello, Rust!");
    mutable_greeting.push_str(" Welcome to programming.");
    println!("{}", mutable_greeting);

    // push() to add one char
    let mut word = String::from("Hi");
    word.push('!');
    println!("{}", word); 

    // Concatenate Strings
    let s1 = String::from("Hello");
    let s2 = String::from("World");
    let result = format!("{} {}", s1, s2);
    println!("{}", result);

    let s3 = String::from("!");
    let result = s1 + " " + &s2 + " " + &s3;
    println!("{}", result);

    // String length
    let name = String::from("Rust");
    println!("Length: {}", name.len());

    // Borrowing and references
    let a = String::from("Hello");
    let b = &a;

    println!("a = {}", a);
    println!("b = {}", b);

    // Mutable references
    let mut c = String::from("John");
    let d = &mut c;
    d.push_str(" Doe");

    println!("{}", d);

    // Data structure 

    // Arrays
    let numbers = [1, 2, 3, 4, 5];

    println!("The first number is: {}", numbers[0]);

    // Changing array values 
    let mut times = [1, 2, 3, 4, 5];

    times[0] = 10;
    println!("The first number is: {}", times[0]);

    // Array length 
    println!("The number of numbers in times are: {}", times.len());

    // Loop through array 
    let fruits = ["apple", "banana", "orange"];
    for fruit in fruits {
       println!("I like {}.", fruit);
    }

    // Print an entire array
    let numbers = [1, 2, 3, 4, 5];
    println!("{:?}", numbers);

    // Vectors 
    let fruits = vec!["apple", "banana", "orange"];
    println!("First fruit: {}", fruits[0]);

    // Add element to a vector 
    let mut fruits = vec!["apple", "banana"];
    fruits.push("cherry");
    println!("{:?}", fruits);

    // Remove the last element of a vector 
    let mut fruits = vec!["apple", "banana", "cherry"];
    fruits.pop();
    println!("{:?}", fruits);

    // Add or remove element at specific index from a vector 
    let mut fruits = vec!["banana", "orange"];
    fruits.insert(0, "apple");
    println!("{:?}", fruits);

    let mut fruits = vec!["banana", "orange"];
    fruits.remove(0);
    println!("{:?}", fruits);

    // Tuples
    let person = ("John", 30, true);
    println!("Name: {}", person.0);
    println!("Age: {}", person.1);
    println!("Is active: {}", person.2);

}
