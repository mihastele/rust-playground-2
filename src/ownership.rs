// ============================================================================
// MODULE 2: OWNERSHIP & BORROWING
// ============================================================================
// This module covers Rust's most unique feature: the ownership system
// - Ownership rules and move semantics
// - References and borrowing
// - Mutable and immutable references
// - Lifetimes
// - The slice type
// ============================================================================

/// Demonstrates the three ownership rules in Rust
/// 
/// OWNERSHIP RULES:
/// 1. Each value has a single owner
/// 2. When the owner goes out of scope, the value is dropped
/// 3. Values can be moved or borrowed, but not both simultaneously
pub fn ownership_basics() {
    println!("\n--- Ownership Basics ---");
    
    // Rule 1: Each value has an owner
    let s1 = String::from("hello");
    println!("s1 owns: {}", s1);
    
    // Rule 3: Move semantics (ownership transfer)
    let s2 = s1; // s1's ownership moves to s2
    // println!("{}", s1); // ❌ Error! s1 no longer valid
    println!("s2 now owns: {}", s2);
    
    // Rule 2: Value dropped when owner goes out of scope
    {
        let s3 = String::from("scoped");
        println!("s3 in scope: {}", s3);
    } // s3 is dropped here
    // println!("{}", s3); // ❌ Error! s3 out of scope
    
    // Copy types (stored on stack) don't move
    let x = 5;
    let y = x; // Copy, not move
    println!("x: {}, y: {} (both valid!)", x, y);
    
    // Types that implement Copy trait:
    // - All integers, floats, booleans, char
    // - Tuples containing only Copy types
    let tuple = (1, 2.5, true);
    let tuple_copy = tuple;
    println!("Original tuple: {:?}, Copy: {:?}", tuple, tuple_copy);
}

/// Demonstrates ownership with functions
/// 
/// KEY CONCEPTS:
/// - Passing a value to a function moves or copies it
/// - Returning values transfers ownership
pub fn ownership_and_functions() {
    println!("\n--- Ownership and Functions ---");
    
    let s = String::from("hello");
    println!("Before function: {}", s);
    
    takes_ownership(s); // s moves into function
    // println!("{}", s); // ❌ Error! s no longer valid
    
    let x = 5;
    makes_copy(x); // x is copied (i32 is Copy)
    println!("After function: {} (still valid!)", x);
    
    // Getting ownership back via return
    let s1 = String::from("world");
    let s2 = takes_and_gives_back(s1);
    println!("Got ownership back: {}", s2);
    
    // Tedious pattern: take and return ownership
    let s3 = String::from("data");
    let (s4, len) = calculate_length_with_ownership(s3);
    println!("String '{}' has length {}", s4, len);
}

fn takes_ownership(s: String) {
    println!("Function owns: {}", s);
} // s is dropped here

fn makes_copy(x: i32) {
    println!("Function has copy: {}", x);
}

fn takes_and_gives_back(s: String) -> String {
    s // Return ownership
}

fn calculate_length_with_ownership(s: String) -> (String, usize) {
    let length = s.len();
    (s, length) // Return ownership + result
}

/// Demonstrates references and borrowing
/// 
/// BORROWING:
/// - References allow using values without taking ownership
/// - & creates a reference (immutable by default)
/// - References must always be valid (no dangling references)
pub fn references_and_borrowing() {
    println!("\n--- References and Borrowing ---");
    
    let s1 = String::from("hello");
    
    // Borrowing: pass a reference instead of ownership
    let len = calculate_length(&s1);
    println!("Length of '{}' is {}", s1, len); // s1 still valid!
    
    // Multiple immutable references are allowed
    let r1 = &s1;
    let r2 = &s1;
    println!("References: {} and {}", r1, r2);
    
    // References are like pointers but guaranteed to be valid
    let x = 5;
    let r = &x;
    println!("Value: {}, Reference: {}", x, *r); // Dereference with *
}

fn calculate_length(s: &String) -> usize {
    s.len()
} // s goes out of scope, but doesn't drop the String (not owner)

/// Demonstrates mutable references
/// 
/// MUTABLE REFERENCES:
/// - &mut creates a mutable reference
/// - Only ONE mutable reference allowed at a time
/// - Cannot have mutable and immutable references simultaneously
pub fn mutable_references() {
    println!("\n--- Mutable References ---");
    
    let mut s = String::from("hello");
    println!("Before: {}", s);
    
    change(&mut s); // Pass mutable reference
    println!("After: {}", s);
    
    // Only one mutable reference at a time
    let r1 = &mut s;
    r1.push_str("!");
    // let r2 = &mut s; // ❌ Error! Can't have two mutable refs
    println!("Modified: {}", r1);
    
    // Can have multiple mutable refs in different scopes
    {
        let r2 = &mut s;
        r2.push_str("!");
    } // r2 goes out of scope
    
    let r3 = &mut s; // Now this is okay
    println!("Final: {}", r3);
    
    // Cannot mix mutable and immutable references
    let mut value = 10;
    let r_immut = &value;
    // let r_mut = &mut value; // ❌ Error! Can't borrow as mutable
    println!("Immutable ref: {}", r_immut);
    
    // After last use of immutable refs, can create mutable ref
    let r_mut = &mut value;
    *r_mut += 5;
    println!("Mutable ref: {}", r_mut);
}

fn change(s: &mut String) {
    s.push_str(", world");
}

/// Demonstrates the rules preventing dangling references
/// 
/// DANGLING REFERENCES:
/// - Rust prevents dangling references at compile time
/// - References must always point to valid data
pub fn no_dangling_references() {
    println!("\n--- No Dangling References ---");
    
    // This works: return ownership
    let s = no_dangle();
    println!("Valid string: {}", s);
    
    // This would fail (commented out):
    // let reference = dangle(); // ❌ Error! Returns reference to dropped value
}

fn no_dangle() -> String {
    let s = String::from("hello");
    s // Return ownership (move out)
}

// This would cause a compile error:
// fn dangle() -> &String {
//     let s = String::from("hello");
//     &s // ❌ Error! s is dropped, reference would be invalid
// }

/// Demonstrates string slices
/// 
/// SLICES:
/// - Reference a contiguous sequence of elements
/// - Don't take ownership
/// - Type: &str for strings, &[T] for arrays
pub fn string_slices() {
    println!("\n--- String Slices ---");
    
    let s = String::from("hello world");
    
    // Slice syntax: &s[start..end]
    let hello = &s[0..5]; // or &s[..5]
    let world = &s[6..11]; // or &s[6..]
    println!("Slices: '{}' and '{}'", hello, world);
    
    // Full slice
    let full = &s[..];
    println!("Full slice: '{}'", full);
    
    // Finding first word
    let first = first_word(&s);
    println!("First word: '{}'", first);
    
    // String literals are slices
    let literal = "Hello, world!"; // Type: &str
    println!("Literal (slice): {}", literal);
    
    // Slices work with String and &str
    let my_string = String::from("hello world");
    let word1 = first_word(&my_string[..]); // String slice
    let word2 = first_word(literal); // String literal
    println!("Words: '{}', '{}'", word1, word2);
}

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();
    
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
    
    &s[..]
}

/// Demonstrates array slices
pub fn array_slices() {
    println!("\n--- Array Slices ---");
    
    let a = [1, 2, 3, 4, 5];
    
    let slice = &a[1..4]; // Elements at index 1, 2, 3
    println!("Array: {:?}", a);
    println!("Slice: {:?}", slice);
    
    // Slices have type &[T]
    assert_eq!(slice, &[2, 3, 4]);
}

/// Demonstrates lifetime basics
/// 
/// LIFETIMES:
/// - Ensure references are valid as long as needed
/// - Prevent dangling references
/// - Usually inferred, sometimes need explicit annotations
pub fn lifetime_basics() {
    println!("\n--- Lifetime Basics ---");
    
    let string1 = String::from("long string");
    let string2 = String::from("short");
    
    let result = longest(string1.as_str(), string2.as_str());
    println!("Longest string: '{}'", result);
    
    // Lifetime ensures result is valid
    {
        let string3 = String::from("xyz");
        let result2 = longest(string1.as_str(), string3.as_str());
        println!("Longest in scope: '{}'", result2);
    } // string3 dropped, but result2 already used
    
    // Struct with lifetime
    let novel = String::from("Call me Ishmael. Some years ago...");
    let first_sentence = novel.split('.').next().expect("No '.'");
    let excerpt = ImportantExcerpt { part: first_sentence };
    println!("Excerpt: '{}'", excerpt.part);
}

// Lifetime annotation: 'a
// Means: returned reference valid as long as both parameters are valid
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

// Struct with lifetime annotation
// Means: ImportantExcerpt can't outlive the reference it holds
struct ImportantExcerpt<'a> {
    part: &'a str,
}

/// Demonstrates lifetime elision rules
/// 
/// LIFETIME ELISION:
/// - Compiler can infer lifetimes in many cases
/// - Three rules for elision
pub fn lifetime_elision() {
    println!("\n--- Lifetime Elision ---");
    
    let s = String::from("hello");
    
    // No explicit lifetime needed (elided)
    let result = first_word_elided(&s);
    println!("First word: '{}'", result);
    
    // Compiler applies elision rules:
    // 1. Each parameter gets its own lifetime
    // 2. If one input lifetime, it's assigned to all outputs
    // 3. If multiple inputs and one is &self/&mut self, use that lifetime
}

// Lifetime elided (compiler infers it)
fn first_word_elided(s: &str) -> &str {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
    &s[..]
}

/// Demonstrates static lifetime
/// 
/// STATIC LIFETIME:
/// - 'static means reference lives for entire program
/// - All string literals have 'static lifetime
pub fn static_lifetime() {
    println!("\n--- Static Lifetime ---");
    
    // String literal has 'static lifetime
    let s: &'static str = "I live forever!";
    println!("Static string: {}", s);
    
    // Static variables
    static LANGUAGE: &str = "Rust";
    println!("Static variable: {}", LANGUAGE);
}

// ============================================================================
// PUBLIC INTERFACE
// ============================================================================

pub fn run_all_examples() {
    ownership_basics();
    ownership_and_functions();
    references_and_borrowing();
    mutable_references();
    no_dangling_references();
    string_slices();
    array_slices();
    lifetime_basics();
    lifetime_elision();
    static_lifetime();
}

// ============================================================================
// UNIT TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_first_word() {
        assert_eq!(first_word("hello world"), "hello");
        assert_eq!(first_word("hello"), "hello");
    }
    
    #[test]
    fn test_longest() {
        assert_eq!(longest("short", "longer"), "longer");
        assert_eq!(longest("same", "size"), "same");
    }
    
    #[test]
    fn test_slices() {
        let arr = [1, 2, 3, 4, 5];
        let slice = &arr[1..4];
        assert_eq!(slice, &[2, 3, 4]);
    }
}
