# 🦀 Rust Quick Reference Cheat Sheet

## Variables & Types

```rust
// Variables (immutable by default)
let x = 5;
let mut y = 10;        // Mutable
const MAX: u32 = 100;  // Constant

// Shadowing
let x = x + 1;
let x = "now a string";

// Types
let a: i32 = -42;      // Signed integer
let b: u32 = 42;       // Unsigned integer
let c: f64 = 3.14;     // Float
let d: bool = true;    // Boolean
let e: char = '😊';    // Character (Unicode)

// Tuples
let tup: (i32, f64, u8) = (500, 6.4, 1);
let (x, y, z) = tup;   // Destructuring

// Arrays
let arr = [1, 2, 3, 4, 5];
let arr: [i32; 5] = [1, 2, 3, 4, 5];
```

## Ownership & Borrowing

```rust
// Ownership (move)
let s1 = String::from("hello");
let s2 = s1;  // s1 no longer valid

// Borrowing (immutable reference)
let s1 = String::from("hello");
let len = calculate_length(&s1);  // s1 still valid

// Mutable reference
let mut s = String::from("hello");
change(&mut s);

// Rules:
// 1. One mutable reference OR multiple immutable references
// 2. References must always be valid
```

## Functions

```rust
fn function_name(param: Type) -> ReturnType {
    // body
    return_value  // No semicolon = return
}

// No return value (returns unit type ())
fn no_return(x: i32) {
    println!("{}", x);
}
```

## Control Flow

```rust
// If expression
let number = if condition { 5 } else { 6 };

// Loop
loop {
    if condition { break; }
}

// While
while condition {
    // body
}

// For
for element in collection.iter() {
    // body
}

for i in 0..10 {  // Range
    // body
}
```

## Structs

```rust
// Define
struct User {
    username: String,
    email: String,
    active: bool,
}

// Create
let user = User {
    username: String::from("alice"),
    email: String::from("alice@example.com"),
    active: true,
};

// Methods
impl User {
    fn new(username: String, email: String) -> User {
        User { username, email, active: true }
    }
    
    fn deactivate(&mut self) {
        self.active = false;
    }
}
```

## Enums & Pattern Matching

```rust
// Define enum
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
}

// Match
match message {
    Message::Quit => println!("Quit"),
    Message::Move { x, y } => println!("Move to {}, {}", x, y),
    Message::Write(text) => println!("{}", text),
}

// if let
if let Some(value) = optional {
    println!("{}", value);
}

// Option<T>
let some = Some(5);
let none: Option<i32> = None;
```

## Error Handling

```rust
// Result<T, E>
fn might_fail() -> Result<i32, String> {
    if success {
        Ok(42)
    } else {
        Err(String::from("failed"))
    }
}

// Using Result
match might_fail() {
    Ok(value) => println!("{}", value),
    Err(e) => println!("Error: {}", e),
}

// ? operator (propagate error)
fn chained() -> Result<i32, String> {
    let value = might_fail()?;
    Ok(value + 1)
}

// unwrap/expect (panics on error)
let value = might_fail().unwrap();
let value = might_fail().expect("Custom message");
```

## Collections

```rust
// Vector
let mut v = Vec::new();
let v = vec![1, 2, 3];
v.push(4);
let third = &v[2];
let third = v.get(2);  // Returns Option<&T>

// String
let mut s = String::from("hello");
s.push_str(" world");
let s = format!("{} {}", "hello", "world");

// HashMap
use std::collections::HashMap;
let mut map = HashMap::new();
map.insert(String::from("key"), 10);
let value = map.get("key");  // Returns Option<&V>
```

## Traits

```rust
// Define trait
trait Summary {
    fn summarize(&self) -> String;
}

// Implement trait
impl Summary for Article {
    fn summarize(&self) -> String {
        format!("{}", self.headline)
    }
}

// Trait bounds
fn notify<T: Summary>(item: &T) {
    println!("{}", item.summarize());
}

// Multiple bounds
fn notify<T: Summary + Display>(item: &T) { }

// Where clause
fn notify<T>(item: &T)
where
    T: Summary + Display,
{ }
```

## Generics

```rust
// Generic function
fn largest<T: PartialOrd>(list: &[T]) -> &T {
    // implementation
}

// Generic struct
struct Point<T> {
    x: T,
    y: T,
}

// Generic impl
impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
}
```

## Closures

```rust
// Syntax
let closure = |x| x + 1;
let closure = |x: i32| -> i32 { x + 1 };

// Capturing environment
let x = 10;
let closure = |y| x + y;

// Move closure
let s = String::from("hello");
let closure = move || println!("{}", s);
```

## Iterators

```rust
// Create iterator
let v = vec![1, 2, 3];
let iter = v.iter();

// Adaptors (lazy)
let doubled: Vec<i32> = v.iter().map(|x| x * 2).collect();
let evens: Vec<i32> = v.iter().filter(|x| *x % 2 == 0).collect();

// Consumers (eager)
let sum: i32 = v.iter().sum();
let count = v.iter().count();
let any = v.iter().any(|x| *x > 5);
let all = v.iter().all(|x| *x > 0);

// Chaining
let result: Vec<i32> = v.iter()
    .filter(|x| *x % 2 == 0)
    .map(|x| x * 2)
    .collect();
```

## Smart Pointers

```rust
// Box<T> - heap allocation
let b = Box::new(5);

// Rc<T> - reference counting (single-threaded)
use std::rc::Rc;
let a = Rc::new(5);
let b = Rc::clone(&a);

// RefCell<T> - interior mutability
use std::cell::RefCell;
let value = RefCell::new(5);
*value.borrow_mut() += 1;

// Arc<T> - atomic reference counting (multi-threaded)
use std::sync::Arc;
let a = Arc::new(5);
let b = Arc::clone(&a);
```

## Concurrency

```rust
// Spawn thread
use std::thread;
let handle = thread::spawn(|| {
    // thread code
});
handle.join().unwrap();

// Channels
use std::sync::mpsc;
let (tx, rx) = mpsc::channel();
tx.send(10).unwrap();
let received = rx.recv().unwrap();

// Mutex
use std::sync::Mutex;
let m = Mutex::new(5);
let mut num = m.lock().unwrap();
*num = 6;

// Arc + Mutex (shared state)
use std::sync::{Arc, Mutex};
let counter = Arc::new(Mutex::new(0));
let counter_clone = Arc::clone(&counter);
thread::spawn(move || {
    let mut num = counter_clone.lock().unwrap();
    *num += 1;
});
```

## Lifetimes

```rust
// Lifetime annotation
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() { x } else { y }
}

// Struct with lifetime
struct ImportantExcerpt<'a> {
    part: &'a str,
}

// Static lifetime
let s: &'static str = "I live forever";
```

## Common Patterns

```rust
// Option handling
let value = some_option.unwrap_or(default);
let value = some_option.unwrap_or_else(|| compute_default());
let doubled = some_option.map(|x| x * 2);
let result = some_option.ok_or("error message")?;

// Result handling
let value = result.unwrap_or(default);
let value = result.unwrap_or_else(|_| default);
let doubled = result.map(|x| x * 2);
let mapped_err = result.map_err(|e| format!("Error: {}", e));

// Chaining
let result = step1()?
    .step2()?
    .step3()?;
```

## Useful Attributes

```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Point { x: i32, y: i32 }

#[allow(dead_code)]
fn unused_function() { }

#[cfg(test)]
mod tests {
    #[test]
    fn test_something() {
        assert_eq!(2 + 2, 4);
    }
}

#[cfg(target_os = "windows")]
fn windows_only() { }
```

## Cargo Commands

```bash
cargo new project_name      # Create new project
cargo build                 # Build project
cargo build --release       # Build optimized
cargo run                   # Build and run
cargo test                  # Run tests
cargo check                 # Check without building
cargo fmt                   # Format code
cargo clippy                # Run linter
cargo doc --open            # Generate and open docs
```

## Common Macros

```rust
println!("Hello, {}!", name);
eprintln!("Error: {}", error);
format!("Formatted: {}", value);
vec![1, 2, 3];
panic!("Something went wrong!");
assert_eq!(left, right);
assert!(condition);
dbg!(variable);
```

## Tips

- Use `cargo clippy` for linting suggestions
- Use `cargo fmt` to format code automatically
- Read compiler error messages carefully - they're helpful!
- When stuck with borrowing, try cloning first, optimize later
- Use `#[derive(Debug)]` to print structs with `{:?}`
- Use `unwrap()` for prototyping, proper error handling for production
- Iterators are zero-cost abstractions - use them!
