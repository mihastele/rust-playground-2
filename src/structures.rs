// ============================================================================
// MODULE 3: STRUCTURES & ENUMS
// ============================================================================
// This module covers custom data types in Rust:
// - Structs (classic, tuple, unit)
// - Methods and associated functions
// - Enums and variants
// - Pattern matching with match
// - if let and while let
// - Option and Result enums
// ============================================================================

/// Demonstrates classic struct definition and usage
/// 
/// STRUCTS:
/// - Custom data types that group related data
/// - Named fields with types
/// - Similar to classes in other languages (but no inheritance)
pub fn struct_basics() {
    println!("\n--- Struct Basics ---");
    
    // Creating a struct instance
    let user1 = User {
        username: String::from("alice"),
        email: String::from("alice@example.com"),
        sign_in_count: 1,
        active: true,
    };
    
    println!("User: {} ({})", user1.username, user1.email);
    println!("Active: {}, Sign-ins: {}", user1.active, user1.sign_in_count);
    
    // Mutable struct
    let mut user2 = User {
        username: String::from("bob"),
        email: String::from("bob@example.com"),
        sign_in_count: 0,
        active: false,
    };
    
    user2.sign_in_count += 1;
    user2.active = true;
    println!("Updated user: {} with {} sign-ins", user2.username, user2.sign_in_count);
    
    // Struct update syntax (copy fields from another struct)
    let user3 = User {
        email: String::from("charlie@example.com"),
        username: String::from("charlie"),
        ..user1 // Copy remaining fields from user1
    };
    println!("User3: {} ({})", user3.username, user3.email);
}

// Classic struct with named fields
struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

/// Demonstrates tuple structs
/// 
/// TUPLE STRUCTS:
/// - Structs without named fields
/// - Useful for creating distinct types
pub fn tuple_structs() {
    println!("\n--- Tuple Structs ---");
    
    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);
    
    println!("Color: RGB({}, {}, {})", black.0, black.1, black.2);
    println!("Point: ({}, {}, {})", origin.0, origin.1, origin.2);
    
    // Different types even with same structure
    // let color: Color = origin; // ❌ Error! Different types
}

struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

/// Demonstrates unit structs
/// 
/// UNIT STRUCTS:
/// - Structs with no fields
/// - Useful for implementing traits without data
pub fn unit_structs() {
    println!("\n--- Unit Structs ---");
    
    let _marker = AlwaysEqual;
    println!("Unit struct created (no data stored)");
}

struct AlwaysEqual;

/// Demonstrates methods on structs
/// 
/// METHODS:
/// - Functions defined within impl block
/// - First parameter is &self, &mut self, or self
/// - Called with dot notation
pub fn struct_methods() {
    println!("\n--- Struct Methods ---");
    
    let rect = Rectangle {
        width: 30,
        height: 50,
    };
    
    println!("Rectangle: {}x{}", rect.width, rect.height);
    println!("Area: {}", rect.area());
    println!("Is square: {}", rect.is_square());
    
    let rect2 = Rectangle {
        width: 10,
        height: 40,
    };
    
    println!("Can hold rect2: {}", rect.can_hold(&rect2));
    
    // Method that takes ownership
    let rect3 = Rectangle::square(25);
    println!("Square: {}x{}", rect3.width, rect3.height);
}

struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    // Method: takes &self
    fn area(&self) -> u32 {
        self.width * self.height
    }
    
    // Method: can have multiple parameters
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
    
    // Method: returns bool
    fn is_square(&self) -> bool {
        self.width == self.height
    }
    
    // Associated function: doesn't take self
    // Called with :: syntax (Rectangle::square)
    fn square(size: u32) -> Rectangle {
        Rectangle {
            width: size,
            height: size,
        }
    }
}

// Multiple impl blocks are allowed
impl Rectangle {
    fn perimeter(&self) -> u32 {
        2 * (self.width + self.height)
    }
}

/// Demonstrates enum basics
/// 
/// ENUMS:
/// - Type that can be one of several variants
/// - Each variant can have different data
pub fn enum_basics() {
    println!("\n--- Enum Basics ---");
    
    let ipv4 = IpAddrKind::V4;
    let ipv6 = IpAddrKind::V6;
    
    route(ipv4);
    route(ipv6);
    
    // Enums with data
    let home = IpAddr::V4(127, 0, 0, 1);
    let loopback = IpAddr::V6(String::from("::1"));
    
    println!("Home IP: {:?}", home);
    println!("Loopback IP: {:?}", loopback);
    
    // Enums can have different types per variant
    let msg1 = Message::Write(String::from("hello"));
    let msg2 = Message::Move { x: 10, y: 20 };
    let msg3 = Message::ChangeColor(255, 0, 0);
    
    msg1.call();
    msg2.call();
    msg3.call();
}

#[derive(Debug)]
enum IpAddrKind {
    V4,
    V6,
}

fn route(ip_kind: IpAddrKind) {
    println!("Routing to: {:?}", ip_kind);
}

#[derive(Debug)]
enum IpAddr {
    V4(u8, u8, u8, u8),
    V6(String),
}

enum Message {
    Quit,                       // No data
    Move { x: i32, y: i32 },   // Named fields
    Write(String),              // Single value
    ChangeColor(i32, i32, i32), // Tuple
}

impl Message {
    fn call(&self) {
        match self {
            Message::Quit => println!("Quit message"),
            Message::Move { x, y } => println!("Move to ({}, {})", x, y),
            Message::Write(text) => println!("Write: {}", text),
            Message::ChangeColor(r, g, b) => println!("Color: RGB({}, {}, {})", r, g, b),
        }
    }
}

/// Demonstrates the Option enum
/// 
/// OPTION:
/// - Rust doesn't have null
/// - Option<T> represents optional values
/// - Variants: Some(T) or None
pub fn option_enum() {
    println!("\n--- Option Enum ---");
    
    let some_number = Some(5);
    let some_string = Some("a string");
    let absent_number: Option<i32> = None;
    
    println!("Some number: {:?}", some_number);
    println!("Some string: {:?}", some_string);
    println!("Absent: {:?}", absent_number);
    
    // Must handle None case
    let x = 5;
    let y = Some(10);
    
    // Can't add Option<i32> to i32 directly
    // let sum = x + y; // ❌ Error!
    
    // Must extract value from Option
    match y {
        Some(value) => println!("Sum: {}", x + value),
        None => println!("No value to add"),
    }
    
    // Using Option methods
    let doubled = some_number.map(|n| n * 2);
    println!("Doubled: {:?}", doubled);
    
    let default = absent_number.unwrap_or(0);
    println!("With default: {}", default);
}

/// Demonstrates pattern matching with match
/// 
/// MATCH:
/// - Powerful control flow operator
/// - Must be exhaustive (handle all cases)
/// - Can bind to values in patterns
pub fn pattern_matching() {
    println!("\n--- Pattern Matching ---");
    
    let coin = Coin::Quarter(UsState::Alaska);
    let value = value_in_cents(coin);
    println!("Coin value: {} cents", value);
    
    // Matching with Option
    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);
    
    println!("Five + 1: {:?}", six);
    println!("None + 1: {:?}", none);
    
    // Catch-all pattern
    let dice_roll = 9;
    match dice_roll {
        3 => println!("You get a fancy hat!"),
        7 => println!("You lose your hat!"),
        other => println!("Move {} spaces", other), // Catch-all
    }
    
    // Ignore value with _
    match dice_roll {
        3 => println!("Special!"),
        _ => println!("Nothing special"), // Ignore value
    }
}

#[derive(Debug)]
enum UsState {
    Alaska,
    California,
    // ... other states
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => {
            println!("Lucky penny!");
            1
        }
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("Quarter from {:?}!", state);
            25
        }
    }
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}

/// Demonstrates if let syntax
/// 
/// IF LET:
/// - Concise way to match one pattern
/// - Less verbose than match for single case
pub fn if_let_syntax() {
    println!("\n--- if let Syntax ---");
    
    let some_value = Some(3);
    
    // Using match (verbose)
    match some_value {
        Some(3) => println!("Three!"),
        _ => (),
    }
    
    // Using if let (concise)
    if let Some(3) = some_value {
        println!("Three!");
    }
    
    // With else
    let coin = Coin::Penny;
    let mut count = 0;
    
    if let Coin::Quarter(state) = coin {
        println!("Quarter from {:?}", state);
    } else {
        count += 1;
        println!("Not a quarter, count: {}", count);
    }
}

/// Demonstrates while let syntax
pub fn while_let_syntax() {
    println!("\n--- while let Syntax ---");
    
    let mut stack = vec![1, 2, 3];
    
    // Pop values while Some
    while let Some(top) = stack.pop() {
        println!("Popped: {}", top);
    }
    
    println!("Stack is empty!");
}

/// Demonstrates advanced pattern matching
pub fn advanced_patterns() {
    println!("\n--- Advanced Patterns ---");
    
    // Matching ranges
    let x = 5;
    match x {
        1..=5 => println!("Between 1 and 5"),
        _ => println!("Something else"),
    }
    
    // Destructuring structs
    let p = Point3D { x: 0, y: 7, z: 10 };
    let Point3D { x, y, z } = p;
    println!("Point: x={}, y={}, z={}", x, y, z);
    
    // Matching struct fields
    match p {
        Point3D { x: 0, y, z } => println!("On x-axis at y={}, z={}", y, z),
        Point3D { x, y: 0, z } => println!("On y-axis at x={}, z={}", x, z),
        Point3D { x, y, z: 0 } => println!("On z-axis at x={}, y={}", x, y),
        Point3D { x, y, z } => println!("Not on axis: ({}, {}, {})", x, y, z),
    }
    
    // Ignoring values
    let numbers = (2, 4, 8, 16, 32);
    match numbers {
        (first, _, third, _, fifth) => {
            println!("Some numbers: {}, {}, {}", first, third, fifth);
        }
    }
    
    // Match guards
    let num = Some(4);
    match num {
        Some(x) if x < 5 => println!("Less than five: {}", x),
        Some(x) => println!("Greater or equal to five: {}", x),
        None => println!("None"),
    }
}

struct Point3D {
    x: i32,
    y: i32,
    z: i32,
}

// ============================================================================
// PUBLIC INTERFACE
// ============================================================================

pub fn run_all_examples() {
    struct_basics();
    tuple_structs();
    unit_structs();
    struct_methods();
    enum_basics();
    option_enum();
    pattern_matching();
    if_let_syntax();
    while_let_syntax();
    advanced_patterns();
}

// ============================================================================
// UNIT TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_rectangle_area() {
        let rect = Rectangle {
            width: 10,
            height: 20,
        };
        assert_eq!(rect.area(), 200);
    }
    
    #[test]
    fn test_rectangle_can_hold() {
        let rect1 = Rectangle {
            width: 30,
            height: 50,
        };
        let rect2 = Rectangle {
            width: 10,
            height: 40,
        };
        assert!(rect1.can_hold(&rect2));
        assert!(!rect2.can_hold(&rect1));
    }
    
    #[test]
    fn test_plus_one() {
        assert_eq!(plus_one(Some(5)), Some(6));
        assert_eq!(plus_one(None), None);
    }
    
    #[test]
    fn test_coin_value() {
        assert_eq!(value_in_cents(Coin::Penny), 1);
        assert_eq!(value_in_cents(Coin::Nickel), 5);
        assert_eq!(value_in_cents(Coin::Dime), 10);
    }
}
