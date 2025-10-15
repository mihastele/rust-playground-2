// ============================================================================
// MODULE 10: ADVANCED FEATURES
// ============================================================================
// This module covers advanced Rust features:
// - Unsafe Rust
// - Advanced traits (associated types, default type parameters)
// - Advanced types (type aliases, never type, DST)
// - Macros (declarative and procedural)
// - Attributes and conditional compilation
// ============================================================================

/// Demonstrates unsafe Rust basics
/// 
/// UNSAFE:
/// - Opt out of Rust's safety guarantees
/// - Five unsafe superpowers:
///   1. Dereference raw pointers
///   2. Call unsafe functions
///   3. Access/modify mutable static variables
///   4. Implement unsafe traits
///   5. Access fields of unions
pub fn unsafe_basics() {
    println!("\n--- Unsafe Basics ---");
    
    // Raw pointers
    let mut num = 5;
    
    let r1 = &num as *const i32; // Immutable raw pointer
    let r2 = &mut num as *mut i32; // Mutable raw pointer
    
    println!("Raw pointers created (safe)");
    
    // Dereferencing requires unsafe
    unsafe {
        println!("r1 points to: {}", *r1);
        println!("r2 points to: {}", *r2);
    }
    
    // Creating arbitrary raw pointer (dangerous!)
    let address = 0x012345usize;
    let _r = address as *const i32;
    // unsafe { println!("{}", *r); } // Would likely crash!
}

/// Demonstrates unsafe functions
pub fn unsafe_functions() {
    println!("\n--- Unsafe Functions ---");
    
    unsafe fn dangerous() {
        println!("This is an unsafe function!");
    }
    
    // Must call in unsafe block
    unsafe {
        dangerous();
    }
    
    // Safe abstraction over unsafe code
    let mut v = vec![1, 2, 3, 4, 5, 6];
    let r = &mut v[..];
    
    let (a, b) = split_at_mut(r, 3);
    println!("First half: {:?}", a);
    println!("Second half: {:?}", b);
}

fn split_at_mut(slice: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]) {
    let len = slice.len();
    let ptr = slice.as_mut_ptr();
    
    assert!(mid <= len);
    
    unsafe {
        (
            std::slice::from_raw_parts_mut(ptr, mid),
            std::slice::from_raw_parts_mut(ptr.add(mid), len - mid),
        )
    }
}

/// Demonstrates calling external C functions
pub fn extern_functions() {
    println!("\n--- Extern Functions ---");
    
    // Calling C standard library
    unsafe {
        println!("Absolute value of -3: {}", abs(-3));
    }
    
    println!("FFI (Foreign Function Interface) allows calling C code");
}

extern "C" {
    fn abs(input: i32) -> i32;
}

/// Demonstrates static variables
/// 
/// STATIC:
/// - Global variables with 'static lifetime
/// - Must be immutable or accessed in unsafe block
pub fn static_variables() {
    println!("\n--- Static Variables ---");
    
    static HELLO_WORLD: &str = "Hello, world!";
    println!("Static string: {}", HELLO_WORLD);
    
    // Mutable static (unsafe to access)
    static mut COUNTER: u32 = 0;
    
    unsafe {
        COUNTER += 1;
        println!("Counter: {}", COUNTER);
    }
    
    println!("Note: Mutable statics are unsafe due to data races!");
}

/// Demonstrates unsafe traits
pub fn unsafe_traits() {
    println!("\n--- Unsafe Traits ---");
    
    println!("Unsafe traits require unsafe impl");
    println!("Example: Send and Sync are unsafe traits");
    println!("Compiler implements them automatically when safe");
    
    // Custom unsafe trait
    unsafe trait Foo {
        fn foo(&self);
    }
    
    struct Bar;
    
    unsafe impl Foo for Bar {
        fn foo(&self) {
            println!("Foo implementation");
        }
    }
    
    let bar = Bar;
    bar.foo();
}

/// Demonstrates advanced trait features
pub fn advanced_traits() {
    println!("\n--- Advanced Traits ---");
    
    // Associated types
    let counter = Counter { count: 0 };
    for (i, num) in counter.enumerate().take(5) {
        println!("  Item {}: {}", i, num);
    }
    
    // Default type parameters
    let p1 = Point { x: 5, y: 10 };
    let p2 = Point { x: 1, y: 2 };
    let p3 = p1 + p2;
    println!("Point addition: ({}, {})", p3.x, p3.y);
    
    // Fully qualified syntax
    let person = Human;
    Pilot::fly(&person);
    Wizard::fly(&person);
    person.fly();
    
    // Disambiguate with fully qualified syntax
    println!("Human's name: {}", <Human as Animal>::baby_name());
}

struct Counter {
    count: u32,
}

impl Iterator for Counter {
    type Item = u32; // Associated type
    
    fn next(&mut self) -> Option<Self::Item> {
        if self.count < 5 {
            self.count += 1;
            Some(self.count)
        } else {
            None
        }
    }
}

#[derive(Debug, Copy, Clone)]
struct Point {
    x: i32,
    y: i32,
}

impl std::ops::Add for Point {
    type Output = Point; // Associated type
    
    fn add(self, other: Point) -> Point {
        Point {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

trait Pilot {
    fn fly(&self);
}

trait Wizard {
    fn fly(&self);
}

struct Human;

impl Pilot for Human {
    fn fly(&self) {
        println!("This is your captain speaking.");
    }
}

impl Wizard for Human {
    fn fly(&self) {
        println!("Up!");
    }
}

impl Human {
    fn fly(&self) {
        println!("*waving arms furiously*");
    }
}

trait Animal {
    fn baby_name() -> String;
}

impl Animal for Human {
    fn baby_name() -> String {
        String::from("baby")
    }
}

/// Demonstrates type aliases
pub fn type_aliases() {
    println!("\n--- Type Aliases ---");
    
    type Kilometers = i32;
    
    let x: i32 = 5;
    let y: Kilometers = 5;
    println!("x + y = {}", x + y); // Same type!
    
    // Useful for long types
    type Thunk = Box<dyn Fn() + Send + 'static>;
    
    let _f: Thunk = Box::new(|| println!("hi"));
    
    // Common in Result
    type Result<T> = std::result::Result<T, std::io::Error>;
    
    fn read_file() -> Result<String> {
        Ok(String::from("file contents"))
    }
    
    println!("Read result: {:?}", read_file());
}

/// Demonstrates the never type
pub fn never_type() {
    println!("\n--- Never Type ---");
    
    println!("The ! type represents computations that never return");
    
    // Functions that never return
    fn _diverges() -> ! {
        panic!("This function never returns!");
    }
    
    // Useful in match arms
    let _result = match Some(5) {
        Some(x) => x,
        None => panic!("No value!"), // ! can coerce to any type
    };
    
    // loop has type !
    let _x: i32 = loop {
        break 5; // But break can return a value
    };
    
    println!("Never type is useful for type system completeness");
}

/// Demonstrates dynamically sized types (DST)
pub fn dynamically_sized_types() {
    println!("\n--- Dynamically Sized Types ---");
    
    println!("DSTs have size known only at runtime");
    println!("Examples: str, [T], dyn Trait");
    
    // str is a DST (must use &str or Box<str>)
    let s1: &str = "Hello";
    println!("String slice: {}", s1);
    
    // [T] is a DST (must use &[T] or Box<[T]>)
    let arr: &[i32] = &[1, 2, 3];
    println!("Slice: {:?}", arr);
    
    // Trait objects are DSTs
    let shape: &dyn std::fmt::Display = &42;
    println!("Trait object: {}", shape);
    
    println!("\nSized trait:");
    println!("  - Automatically implemented for types with known size");
    println!("  - Generic functions have implicit Sized bound");
    println!("  - Use ?Sized to opt out");
}

/// Demonstrates function pointers
pub fn function_pointers() {
    println!("\n--- Function Pointers ---");
    
    fn add_one(x: i32) -> i32 {
        x + 1
    }
    
    // Function pointer type: fn
    let f: fn(i32) -> i32 = add_one;
    println!("Function pointer result: {}", f(5));
    
    // Passing function pointers
    let numbers = vec![1, 2, 3];
    let result: Vec<i32> = numbers.iter().map(|&x| add_one(x)).collect();
    println!("Mapped with function: {:?}", result);
    
    // Functions implement Fn, FnMut, and FnOnce
    do_twice(add_one, 5);
}

fn do_twice(f: fn(i32) -> i32, arg: i32) -> i32 {
    f(arg) + f(arg)
}

/// Demonstrates declarative macros
pub fn declarative_macros() {
    println!("\n--- Declarative Macros ---");
    
    // Using custom macro
    let v = my_vec![1, 2, 3];
    println!("Created with macro: {:?}", v);
    
    // Macros can take variable arguments
    say_hello!();
    say_hello!("Alice");
    say_hello!("Bob", "Charlie");
}

// Define a simple macro
macro_rules! my_vec {
    ( $( $x:expr ),* ) => {
        {
            let mut temp_vec = Vec::new();
            $(
                temp_vec.push($x);
            )*
            temp_vec
        }
    };
}

macro_rules! say_hello {
    () => {
        println!("Hello!");
    };
    ($name:expr) => {
        println!("Hello, {}!", $name);
    };
    ($name1:expr, $name2:expr) => {
        println!("Hello, {} and {}!", $name1, $name2);
    };
}

/// Demonstrates attributes
pub fn attributes() {
    println!("\n--- Attributes ---");
    
    println!("Common attributes:");
    println!("  #[derive(...)] - Auto-implement traits");
    println!("  #[cfg(...)] - Conditional compilation");
    println!("  #[test] - Mark test functions");
    println!("  #[allow(...)] - Suppress warnings");
    println!("  #[deprecated] - Mark as deprecated");
    
    #[allow(dead_code)]
    fn unused_function() {
        // This won't trigger a warning
    }
    
    #[cfg(target_os = "windows")]
    fn windows_only() {
        println!("This is Windows!");
    }
    
    #[cfg(target_os = "linux")]
    fn linux_only() {
        println!("This is Linux!");
    }
    
    #[cfg(target_os = "windows")]
    windows_only();
    
    #[cfg(target_os = "linux")]
    linux_only();
}

/// Demonstrates conditional compilation
pub fn conditional_compilation() {
    println!("\n--- Conditional Compilation ---");
    
    #[cfg(debug_assertions)]
    println!("Debug mode enabled");
    
    #[cfg(not(debug_assertions))]
    println!("Release mode");
    
    // Feature flags
    println!("Use feature flags in Cargo.toml:");
    println!("  [features]");
    println!("  feature_name = []");
    println!("\nThen use: #[cfg(feature = \"feature_name\")]");
}

/// Demonstrates advanced patterns
pub fn advanced_patterns() {
    println!("\n--- Advanced Patterns ---");
    
    // @ bindings
    let msg = Message::Hello { id: 5 };
    
    match msg {
        Message::Hello { id: id_var @ 3..=7 } => {
            println!("Found id in range: {}", id_var);
        }
        Message::Hello { id: 10..=12 } => {
            println!("Found id in another range");
        }
        Message::Hello { id } => {
            println!("Found other id: {}", id);
        }
    }
    
    // Match guards
    let num = Some(4);
    match num {
        Some(x) if x < 5 => println!("Less than five: {}", x),
        Some(x) => println!("Greater or equal: {}", x),
        None => (),
    }
}

enum Message {
    Hello { id: i32 },
}

/// Demonstrates best practices
pub fn best_practices() {
    println!("\n--- Best Practices ---");
    
    println!("1. Minimize unsafe code");
    println!("   - Encapsulate in safe abstractions");
    println!("   - Document safety invariants");
    
    println!("\n2. Use type system for correctness");
    println!("   - Leverage traits and generics");
    println!("   - Make invalid states unrepresentable");
    
    println!("\n3. Prefer iterators over loops");
    println!("   - More expressive and often faster");
    println!("   - Zero-cost abstractions");
    
    println!("\n4. Use Result for error handling");
    println!("   - Don't panic in library code");
    println!("   - Provide good error messages");
    
    println!("\n5. Write documentation");
    println!("   - Use /// for public APIs");
    println!("   - Include examples in docs");
    
    println!("\n6. Write tests");
    println!("   - Unit tests with #[test]");
    println!("   - Integration tests in tests/");
    println!("   - Doc tests in documentation");
}

// ============================================================================
// PUBLIC INTERFACE
// ============================================================================

pub fn run_all_examples() {
    unsafe_basics();
    unsafe_functions();
    extern_functions();
    static_variables();
    unsafe_traits();
    advanced_traits();
    type_aliases();
    never_type();
    dynamically_sized_types();
    function_pointers();
    declarative_macros();
    attributes();
    conditional_compilation();
    advanced_patterns();
    best_practices();
}

// ============================================================================
// UNIT TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_unsafe_dereference() {
        let x = 5;
        let r = &x as *const i32;
        unsafe {
            assert_eq!(*r, 5);
        }
    }
    
    #[test]
    fn test_split_at_mut() {
        let mut v = vec![1, 2, 3, 4, 5];
        let (a, b) = split_at_mut(&mut v, 3);
        assert_eq!(a, &[1, 2, 3]);
        assert_eq!(b, &[4, 5]);
    }
    
    #[test]
    fn test_macro() {
        let v = my_vec![1, 2, 3];
        assert_eq!(v, vec![1, 2, 3]);
    }
    
    #[test]
    fn test_function_pointer() {
        fn double(x: i32) -> i32 { x * 2 }
        let f: fn(i32) -> i32 = double;
        assert_eq!(f(5), 10);
    }
}
