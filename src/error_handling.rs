// ============================================================================
// MODULE 6: ERROR HANDLING
// ============================================================================
// This module covers Rust's error handling mechanisms:
// - panic! for unrecoverable errors
// - Result<T, E> for recoverable errors
// - Option<T> for optional values
// - Error propagation with ?
// - Custom error types
// ============================================================================

use std::fs::File;
use std::io::{self, Read, ErrorKind};
use std::fmt;

/// Demonstrates panic! for unrecoverable errors
/// 
/// PANIC:
/// - Used for unrecoverable errors
/// - Unwinds the stack and cleans up
/// - Can set RUST_BACKTRACE=1 for backtrace
pub fn panic_basics() {
    println!("\n--- Panic Basics ---");
    
    // Explicit panic
    // panic!("crash and burn"); // Uncomment to see panic
    
    // Panic from out of bounds access
    let v = vec![1, 2, 3];
    // v[99]; // Uncomment to see panic
    
    println!("Panic examples are commented out to avoid crashing!");
    println!("Uncomment them in the source to see how panics work.");
}

/// Demonstrates Result<T, E> for recoverable errors
/// 
/// RESULT:
/// - Enum with Ok(T) and Err(E) variants
/// - Used for operations that might fail
/// - Forces explicit error handling
pub fn result_basics() {
    println!("\n--- Result Basics ---");
    
    // File operations return Result
    let file_result = File::open("hello.txt");
    
    match file_result {
        Ok(file) => println!("File opened successfully: {:?}", file),
        Err(error) => println!("Failed to open file: {:?}", error),
    }
    
    // Handling different error kinds
    let file_result = File::open("hello.txt");
    
    let _file = match file_result {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => {
                println!("File not found, would create it here");
                return; // Skip file creation for this example
            }
            other_error => {
                println!("Problem opening file: {:?}", other_error);
                return;
            }
        },
    };
}

/// Demonstrates unwrap and expect
/// 
/// SHORTCUTS:
/// - unwrap(): Returns value or panics
/// - expect(): Like unwrap but with custom message
pub fn unwrap_and_expect() {
    println!("\n--- Unwrap and Expect ---");
    
    // unwrap: panics on error
    // let file = File::open("hello.txt").unwrap(); // Panics if file doesn't exist
    
    // expect: panics with custom message
    // let file = File::open("hello.txt")
    //     .expect("Failed to open hello.txt"); // Better error message
    
    println!("Unwrap/expect examples commented out to avoid panics");
    
    // Safe alternative: unwrap_or
    let default_value = Some(5).unwrap_or(0);
    println!("With default: {}", default_value);
    
    let none_value: Option<i32> = None;
    let with_default = none_value.unwrap_or(10);
    println!("None with default: {}", with_default);
}

/// Demonstrates error propagation
/// 
/// ERROR PROPAGATION:
/// - Return errors to calling code
/// - Use ? operator for concise propagation
pub fn error_propagation() {
    println!("\n--- Error Propagation ---");
    
    match read_username_from_file() {
        Ok(username) => println!("Username: {}", username),
        Err(e) => println!("Error reading username: {}", e),
    }
    
    match read_username_short() {
        Ok(username) => println!("Username (short): {}", username),
        Err(e) => println!("Error reading username: {}", e),
    }
}

// Verbose error propagation
fn read_username_from_file() -> Result<String, io::Error> {
    let file_result = File::open("username.txt");
    
    let mut file = match file_result {
        Ok(file) => file,
        Err(e) => return Err(e), // Propagate error
    };
    
    let mut username = String::new();
    
    match file.read_to_string(&mut username) {
        Ok(_) => Ok(username),
        Err(e) => Err(e), // Propagate error
    }
}

// Concise error propagation with ?
fn read_username_short() -> Result<String, io::Error> {
    let mut file = File::open("username.txt")?; // ? propagates error
    let mut username = String::new();
    file.read_to_string(&mut username)?;
    Ok(username)
}

// Even shorter with chaining
fn read_username_shortest() -> Result<String, io::Error> {
    let mut username = String::new();
    File::open("username.txt")?.read_to_string(&mut username)?;
    Ok(username)
}

/// Demonstrates the ? operator
/// 
/// ? OPERATOR:
/// - Returns error if Result is Err
/// - Unwraps value if Result is Ok
/// - Automatically converts error types (with From trait)
pub fn question_mark_operator() {
    println!("\n--- ? Operator ---");
    
    // ? can only be used in functions that return Result or Option
    fn divide(a: i32, b: i32) -> Result<i32, String> {
        if b == 0 {
            Err(String::from("Division by zero"))
        } else {
            Ok(a / b)
        }
    }
    
    fn calculate() -> Result<i32, String> {
        let result = divide(10, 2)?; // Propagates error if any
        let result2 = divide(result, 2)?;
        Ok(result2)
    }
    
    match calculate() {
        Ok(result) => println!("Calculation result: {}", result),
        Err(e) => println!("Calculation error: {}", e),
    }
}

/// Demonstrates Option<T> for optional values
/// 
/// OPTION:
/// - Represents a value that might be absent
/// - Variants: Some(T) or None
/// - No null in Rust!
pub fn option_handling() {
    println!("\n--- Option Handling ---");
    
    let some_number = Some(5);
    let no_number: Option<i32> = None;
    
    // Pattern matching
    match some_number {
        Some(n) => println!("Got number: {}", n),
        None => println!("No number"),
    }
    
    // if let
    if let Some(n) = some_number {
        println!("Number via if let: {}", n);
    }
    
    // Combinators
    let doubled = some_number.map(|n| n * 2);
    println!("Doubled: {:?}", doubled);
    
    let or_else = no_number.or(Some(10));
    println!("With or: {:?}", or_else);
    
    let and_then = some_number.and_then(|n| Some(n + 5));
    println!("And then: {:?}", and_then);
    
    // Converting Option to Result
    let result: Result<i32, &str> = some_number.ok_or("No value");
    println!("Option to Result: {:?}", result);
}

/// Demonstrates custom error types
/// 
/// CUSTOM ERRORS:
/// - Create your own error types
/// - Implement Error trait
/// - Provide better error information
pub fn custom_errors() {
    println!("\n--- Custom Errors ---");
    
    match parse_age("25") {
        Ok(age) => println!("Parsed age: {}", age),
        Err(e) => println!("Error: {}", e),
    }
    
    match parse_age("-5") {
        Ok(age) => println!("Parsed age: {}", age),
        Err(e) => println!("Error: {}", e),
    }
    
    match parse_age("abc") {
        Ok(age) => println!("Parsed age: {}", age),
        Err(e) => println!("Error: {}", e),
    }
}

#[derive(Debug)]
enum AgeError {
    InvalidFormat,
    NegativeAge,
    TooOld,
}

impl fmt::Display for AgeError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            AgeError::InvalidFormat => write!(f, "Invalid age format"),
            AgeError::NegativeAge => write!(f, "Age cannot be negative"),
            AgeError::TooOld => write!(f, "Age is unrealistically high"),
        }
    }
}

impl std::error::Error for AgeError {}

fn parse_age(input: &str) -> Result<u32, AgeError> {
    let age: u32 = input.parse().map_err(|_| AgeError::InvalidFormat)?;
    
    if age > 150 {
        return Err(AgeError::TooOld);
    }
    
    Ok(age)
}

/// Demonstrates combining different error types
pub fn combining_errors() {
    println!("\n--- Combining Errors ---");
    
    match read_and_parse() {
        Ok(number) => println!("Parsed number: {}", number),
        Err(e) => println!("Error: {}", e),
    }
}

// Custom error that can hold different error types
#[derive(Debug)]
enum MyError {
    Io(io::Error),
    Parse(std::num::ParseIntError),
}

impl fmt::Display for MyError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            MyError::Io(e) => write!(f, "IO error: {}", e),
            MyError::Parse(e) => write!(f, "Parse error: {}", e),
        }
    }
}

impl std::error::Error for MyError {}

impl From<io::Error> for MyError {
    fn from(error: io::Error) -> Self {
        MyError::Io(error)
    }
}

impl From<std::num::ParseIntError> for MyError {
    fn from(error: std::num::ParseIntError) -> Self {
        MyError::Parse(error)
    }
}

fn read_and_parse() -> Result<i32, MyError> {
    let mut file = File::open("number.txt")?; // io::Error converted to MyError
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    let number: i32 = contents.trim().parse()?; // ParseIntError converted to MyError
    Ok(number)
}

/// Demonstrates Result methods and combinators
pub fn result_methods() {
    println!("\n--- Result Methods ---");
    
    let ok_result: Result<i32, &str> = Ok(10);
    let err_result: Result<i32, &str> = Err("error");
    
    // is_ok and is_err
    println!("Is ok: {}", ok_result.is_ok());
    println!("Is err: {}", err_result.is_err());
    
    // map: transform Ok value
    let doubled = ok_result.map(|x| x * 2);
    println!("Mapped: {:?}", doubled);
    
    // map_err: transform Err value
    let mapped_err = err_result.map_err(|e| format!("Error: {}", e));
    println!("Mapped error: {:?}", mapped_err);
    
    // and_then: chain operations
    let result = ok_result.and_then(|x| Ok(x + 5));
    println!("And then: {:?}", result);
    
    // or_else: provide alternative
    let result = err_result.or_else(|_| Ok(0));
    println!("Or else: {:?}", result);
    
    // unwrap_or_else: compute default
    let value = err_result.unwrap_or_else(|_| 42);
    println!("Unwrap or else: {}", value);
}

/// Demonstrates early returns with ?
pub fn early_returns() {
    println!("\n--- Early Returns ---");
    
    fn process_data() -> Result<String, String> {
        let step1 = validate_input("valid")?;
        let step2 = transform_data(step1)?;
        let step3 = finalize(step2)?;
        Ok(step3)
    }
    
    fn validate_input(input: &str) -> Result<String, String> {
        if input.is_empty() {
            Err(String::from("Input is empty"))
        } else {
            Ok(input.to_uppercase())
        }
    }
    
    fn transform_data(data: String) -> Result<String, String> {
        Ok(format!("Transformed: {}", data))
    }
    
    fn finalize(data: String) -> Result<String, String> {
        Ok(format!("Final: {}", data))
    }
    
    match process_data() {
        Ok(result) => println!("Success: {}", result),
        Err(e) => println!("Failed: {}", e),
    }
}

/// Demonstrates when to use panic vs Result
pub fn panic_vs_result() {
    println!("\n--- Panic vs Result ---");
    
    println!("Use panic! when:");
    println!("  - Unrecoverable errors (programmer mistakes)");
    println!("  - Prototyping (unwrap/expect)");
    println!("  - Tests (should panic)");
    println!("  - Impossible situations");
    
    println!("\nUse Result when:");
    println!("  - Recoverable errors");
    println!("  - Expected failures (file not found, network error)");
    println!("  - Library code (let caller decide)");
    println!("  - User input validation");
}

// ============================================================================
// PUBLIC INTERFACE
// ============================================================================

pub fn run_all_examples() {
    panic_basics();
    result_basics();
    unwrap_and_expect();
    error_propagation();
    question_mark_operator();
    option_handling();
    custom_errors();
    combining_errors();
    result_methods();
    early_returns();
    panic_vs_result();
}

// ============================================================================
// UNIT TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_parse_age_valid() {
        assert!(parse_age("25").is_ok());
        assert_eq!(parse_age("25").unwrap(), 25);
    }
    
    #[test]
    fn test_parse_age_invalid() {
        assert!(parse_age("abc").is_err());
        assert!(parse_age("200").is_err());
    }
    
    #[test]
    fn test_option_map() {
        let some = Some(5);
        let doubled = some.map(|x| x * 2);
        assert_eq!(doubled, Some(10));
    }
    
    #[test]
    fn test_result_and_then() {
        let ok: Result<i32, &str> = Ok(5);
        let result = ok.and_then(|x| Ok(x * 2));
        assert_eq!(result, Ok(10));
    }
    
    #[test]
    #[should_panic]
    fn test_panic() {
        panic!("This test should panic");
    }
}
