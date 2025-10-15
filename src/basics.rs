// ============================================================================
// MODULE 1: RUST BASICS
// ============================================================================
// This module covers fundamental Rust concepts:
// - Variables and mutability
// - Data types (scalar and compound)
// - Functions and return values
// - Control flow (if, loops, match)
// - Comments and documentation
// ============================================================================

/// Demonstrates variable declarations and mutability in Rust
/// 
/// KEY CONCEPTS:
/// - Variables are immutable by default (cannot be changed)
/// - Use 'mut' keyword to make variables mutable
/// - Shadowing allows reusing variable names with different types
pub fn variables_and_mutability() {
    println!("\n--- Variables and Mutability ---");
    
    // Immutable variable (default in Rust)
    let x = 5;
    println!("Immutable x: {}", x);
    // x = 6; // ❌ This would cause a compile error!
    
    // Mutable variable (can be changed)
    let mut y = 10;
    println!("Mutable y before: {}", y);
    y = 15; // ✅ This works because y is mutable
    println!("Mutable y after: {}", y);
    
    // Shadowing: Reusing the same variable name
    // This creates a NEW variable, not modifying the old one
    let z = 20;
    println!("First z: {}", z);
    let z = z + 5; // Shadow the previous z
    println!("Shadowed z: {}", z);
    let z = "Now I'm a string!"; // Can even change type!
    println!("Shadowed z with different type: {}", z);
    
    // Constants: Always immutable, must have type annotation
    const MAX_POINTS: u32 = 100_000; // Underscores for readability
    println!("Constant MAX_POINTS: {}", MAX_POINTS);
}

/// Demonstrates Rust's scalar data types
/// 
/// SCALAR TYPES (single values):
/// - Integers: i8, i16, i32, i64, i128, isize (signed)
///            u8, u16, u32, u64, u128, usize (unsigned)
/// - Floating-point: f32, f64
/// - Boolean: bool
/// - Character: char (4 bytes, Unicode)
pub fn scalar_types() {
    println!("\n--- Scalar Data Types ---");
    
    // INTEGERS
    let decimal = 98_222; // i32 is the default
    let hex = 0xff; // Hexadecimal
    let octal = 0o77; // Octal
    let binary = 0b1111_0000; // Binary
    let byte = b'A'; // u8 only
    println!("Integers: dec={}, hex={}, oct={}, bin={}, byte={}", 
             decimal, hex, octal, binary, byte);
    
    // Explicit type annotations
    let small: i8 = -128; // Range: -128 to 127
    let big: u64 = 18_446_744_073_709_551_615u64; // Suffix for type
    println!("Small i8: {}, Big u64: {}", small, big);
    
    // FLOATING-POINT
    let float32: f32 = 3.14; // Single precision
    let float64 = 2.71828; // f64 is default (double precision)
    println!("Floats: f32={}, f64={}", float32, float64);
    
    // BOOLEAN
    let is_rust_awesome = true;
    let is_learning_fun: bool = true;
    println!("Booleans: {}, {}", is_rust_awesome, is_learning_fun);
    
    // CHARACTER (Unicode Scalar Value)
    let letter = 'A';
    let emoji = '😊';
    let chinese = '中';
    println!("Characters: {}, {}, {}", letter, emoji, chinese);
    
    // ARITHMETIC OPERATIONS
    let sum = 5 + 10;
    let difference = 95.5 - 4.3;
    let product = 4 * 30;
    let quotient = 56.7 / 32.2;
    let remainder = 43 % 5;
    println!("Math: sum={}, diff={}, prod={}, quot={:.2}, rem={}", 
             sum, difference, product, quotient, remainder);
}

/// Demonstrates compound data types (tuples and arrays)
/// 
/// COMPOUND TYPES (multiple values):
/// - Tuple: Fixed-size collection of different types
/// - Array: Fixed-size collection of same type
pub fn compound_types() {
    println!("\n--- Compound Data Types ---");
    
    // TUPLES: Group different types together
    let person: (&str, i32, f64) = ("Alice", 30, 5.6);
    println!("Tuple: {:?}", person);
    
    // Destructuring tuples
    let (name, age, height) = person;
    println!("Destructured: name={}, age={}, height={}", name, age, height);
    
    // Accessing tuple elements by index
    println!("First element: {}", person.0);
    println!("Second element: {}", person.1);
    
    // Unit type: empty tuple ()
    let unit = ();
    println!("Unit type (empty tuple): {:?}", unit);
    
    // ARRAYS: Fixed-size, same type
    let numbers = [1, 2, 3, 4, 5];
    println!("Array: {:?}", numbers);
    
    // Array with type annotation
    let months: [&str; 12] = [
        "Jan", "Feb", "Mar", "Apr", "May", "Jun",
        "Jul", "Aug", "Sep", "Oct", "Nov", "Dec"
    ];
    println!("First month: {}", months[0]);
    println!("Last month: {}", months[11]);
    
    // Array with repeated values
    let zeros = [0; 5]; // [0, 0, 0, 0, 0]
    println!("Zeros array: {:?}", zeros);
    
    // Array length
    println!("Array length: {}", numbers.len());
}

/// Demonstrates functions, parameters, and return values
/// 
/// KEY CONCEPTS:
/// - Functions use snake_case naming
/// - Parameters must have type annotations
/// - Return type specified with ->
/// - Last expression is returned (no semicolon)
pub fn functions_demo() {
    println!("\n--- Functions ---");
    
    // Calling functions
    greet("Rustacean");
    
    let result = add(5, 3);
    println!("5 + 3 = {}", result);
    
    let (sum, product) = calculate(4, 7);
    println!("4 + 7 = {}, 4 * 7 = {}", sum, product);
    
    // Expression vs Statement
    let y = {
        let x = 3;
        x + 1 // No semicolon = expression (returns value)
    };
    println!("Block expression result: {}", y);
}

// Helper function: no return value (returns unit type ())
fn greet(name: &str) {
    println!("Hello, {}!", name);
}

// Helper function: returns a value
fn add(a: i32, b: i32) -> i32 {
    a + b // No semicolon = this is returned
}

// Helper function: returns multiple values (tuple)
fn calculate(a: i32, b: i32) -> (i32, i32) {
    (a + b, a * b)
}

/// Demonstrates control flow: if expressions, loops
/// 
/// CONTROL FLOW:
/// - if/else if/else
/// - loop (infinite loop)
/// - while (conditional loop)
/// - for (iterator loop)
pub fn control_flow() {
    println!("\n--- Control Flow ---");
    
    // IF EXPRESSIONS
    let number = 7;
    if number < 5 {
        println!("{} is less than 5", number);
    } else if number == 5 {
        println!("{} equals 5", number);
    } else {
        println!("{} is greater than 5", number);
    }
    
    // if is an expression (returns a value)
    let condition = true;
    let value = if condition { 5 } else { 6 };
    println!("Conditional value: {}", value);
    
    // LOOP: Infinite loop (must break manually)
    let mut counter = 0;
    let result = loop {
        counter += 1;
        if counter == 10 {
            break counter * 2; // Return value from loop
        }
    };
    println!("Loop result: {}", result);
    
    // WHILE: Conditional loop
    let mut countdown = 3;
    while countdown > 0 {
        println!("{}...", countdown);
        countdown -= 1;
    }
    println!("Liftoff! 🚀");
    
    // FOR: Iterator loop (most common)
    let array = [10, 20, 30, 40, 50];
    for element in array.iter() {
        println!("Array element: {}", element);
    }
    
    // Range iteration
    for number in 1..=5 { // 1 to 5 inclusive
        println!("Number: {}", number);
    }
    
    // Loop labels (for nested loops)
    'outer: for i in 0..3 {
        for j in 0..3 {
            if i == 1 && j == 1 {
                println!("Breaking outer loop at i={}, j={}", i, j);
                break 'outer;
            }
            println!("i={}, j={}", i, j);
        }
    }
}

/// Demonstrates different types of comments and documentation
pub fn comments_demo() {
    println!("\n--- Comments ---");
    
    // This is a single-line comment
    
    /* This is a 
       multi-line comment */
    
    /// This is a documentation comment (for items)
    /// It supports **Markdown** formatting!
    
    //! This is a module-level documentation comment
    
    println!("Check the source code to see different comment styles!");
}

/// Demonstrates string types in Rust
/// 
/// STRING TYPES:
/// - &str: String slice (immutable, fixed size)
/// - String: Owned, growable string
pub fn strings_basics() {
    println!("\n--- String Basics ---");
    
    // String literal: &str (stored in binary, immutable)
    let string_literal: &str = "Hello, world!";
    println!("String literal: {}", string_literal);
    
    // String: Heap-allocated, growable
    let mut owned_string = String::from("Hello");
    println!("String before: {}", owned_string);
    
    owned_string.push_str(", Rust!"); // Append to String
    println!("String after: {}", owned_string);
    
    // Converting between types
    let from_literal = "literal".to_string();
    let from_string: &str = &owned_string;
    println!("Converted: {} and {}", from_literal, from_string);
    
    // String formatting
    let name = "Alice";
    let age = 30;
    let formatted = format!("{} is {} years old", name, age);
    println!("Formatted: {}", formatted);
}

/// Demonstrates type conversion and casting
pub fn type_conversion() {
    println!("\n--- Type Conversion ---");
    
    // Explicit casting with 'as'
    let integer = 65;
    let character = integer as u8 as char;
    println!("Integer {} as char: {}", integer, character);
    
    // Parsing strings to numbers
    let number_str = "42";
    let number: i32 = number_str.parse().expect("Not a number!");
    println!("Parsed '{}' to: {}", number_str, number);
    
    // Turbofish syntax for type specification
    let parsed = "3.14".parse::<f64>().unwrap();
    println!("Parsed float: {}", parsed);
    
    // Converting to string
    let num = 123;
    let num_string = num.to_string();
    println!("Number {} as string: '{}'", num, num_string);
}

// ============================================================================
// PUBLIC INTERFACE
// ============================================================================

/// Runs all examples in the basics module
pub fn run_all_examples() {
    variables_and_mutability();
    scalar_types();
    compound_types();
    functions_demo();
    control_flow();
    comments_demo();
    strings_basics();
    type_conversion();
}

// ============================================================================
// UNIT TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_add_function() {
        assert_eq!(add(2, 3), 5);
        assert_eq!(add(-1, 1), 0);
    }
    
    #[test]
    fn test_calculate_function() {
        let (sum, product) = calculate(3, 4);
        assert_eq!(sum, 7);
        assert_eq!(product, 12);
    }
    
    #[test]
    fn test_arithmetic() {
        assert_eq!(5 + 5, 10);
        assert_eq!(10 - 5, 5);
        assert_eq!(4 * 3, 12);
        assert_eq!(10 / 2, 5);
        assert_eq!(10 % 3, 1);
    }
}
