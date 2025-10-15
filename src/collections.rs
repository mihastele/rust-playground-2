// ============================================================================
// MODULE 5: COLLECTIONS
// ============================================================================
// This module covers Rust's standard collection types:
// - Vectors (Vec<T>)
// - Strings (String)
// - Hash Maps (HashMap<K, V>)
// - Other collections (VecDeque, HashSet, BTreeMap, etc.)
// ============================================================================

use std::collections::HashMap;
use std::collections::HashSet;
use std::collections::VecDeque;
use std::collections::BTreeMap;

/// Demonstrates vector basics
/// 
/// VECTORS:
/// - Growable arrays stored on the heap
/// - Type: Vec<T>
/// - Can only store values of the same type
pub fn vector_basics() {
    println!("\n--- Vector Basics ---");
    
    // Creating vectors
    let v1: Vec<i32> = Vec::new();
    println!("Empty vector: {:?}", v1);
    
    let v2 = vec![1, 2, 3]; // vec! macro
    println!("Vector with values: {:?}", v2);
    
    // Adding elements
    let mut v3 = Vec::new();
    v3.push(5);
    v3.push(6);
    v3.push(7);
    println!("After pushes: {:?}", v3);
    
    // Accessing elements
    let third = &v2[2]; // Panics if out of bounds
    println!("Third element: {}", third);
    
    match v2.get(2) { // Returns Option<&T>
        Some(third) => println!("Third element (safe): {}", third),
        None => println!("No third element"),
    }
    
    // Iterating
    println!("Iterating over vector:");
    for i in &v2 {
        println!("  {}", i);
    }
    
    // Mutable iteration
    let mut v4 = vec![1, 2, 3];
    for i in &mut v4 {
        *i += 50;
    }
    println!("After mutation: {:?}", v4);
}

/// Demonstrates advanced vector operations
pub fn vector_operations() {
    println!("\n--- Vector Operations ---");
    
    let mut v = vec![1, 2, 3, 4, 5];
    
    // Pop removes last element
    let last = v.pop();
    println!("Popped: {:?}, Vector: {:?}", last, v);
    
    // Insert at index
    v.insert(2, 99);
    println!("After insert: {:?}", v);
    
    // Remove at index
    let removed = v.remove(2);
    println!("Removed: {}, Vector: {:?}", removed, v);
    
    // Length and capacity
    println!("Length: {}, Capacity: {}", v.len(), v.capacity());
    
    // Clear vector
    let mut v2 = vec![1, 2, 3];
    v2.clear();
    println!("After clear: {:?}", v2);
    
    // Extend with another vector
    let mut v3 = vec![1, 2, 3];
    let v4 = vec![4, 5, 6];
    v3.extend(v4);
    println!("After extend: {:?}", v3);
    
    // Concatenate with append
    let mut v5 = vec![1, 2];
    let mut v6 = vec![3, 4];
    v5.append(&mut v6);
    println!("After append: {:?}, v6: {:?}", v5, v6);
}

/// Demonstrates storing different types in vectors using enums
pub fn vector_enum_storage() {
    println!("\n--- Vector with Different Types ---");
    
    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Float(10.12),
        SpreadsheetCell::Text(String::from("blue")),
    ];
    
    for cell in &row {
        match cell {
            SpreadsheetCell::Int(i) => println!("Integer: {}", i),
            SpreadsheetCell::Float(f) => println!("Float: {}", f),
            SpreadsheetCell::Text(s) => println!("Text: {}", s),
        }
    }
}

enum SpreadsheetCell {
    Int(i32),
    Float(f64),
    Text(String),
}

/// Demonstrates String operations
/// 
/// STRINGS:
/// - UTF-8 encoded, growable text
/// - Type: String (owned) vs &str (borrowed)
/// - Collection of bytes
pub fn string_operations() {
    println!("\n--- String Operations ---");
    
    // Creating strings
    let mut s1 = String::new();
    let s2 = "initial contents".to_string();
    let s3 = String::from("initial contents");
    println!("Strings: '{}', '{}', '{}'", s1, s2, s3);
    
    // Updating strings
    s1.push_str("hello");
    s1.push(' '); // Push single char
    s1.push_str("world");
    println!("Built string: '{}'", s1);
    
    // Concatenation with +
    let s4 = String::from("Hello, ");
    let s5 = String::from("world!");
    let s6 = s4 + &s5; // s4 moved, s5 borrowed
    println!("Concatenated: '{}'", s6);
    // println!("{}", s4); // ❌ Error! s4 was moved
    
    // Format macro (doesn't take ownership)
    let s7 = String::from("tic");
    let s8 = String::from("tac");
    let s9 = String::from("toe");
    let s10 = format!("{}-{}-{}", s7, s8, s9);
    println!("Formatted: '{}'", s10);
    println!("Still valid: {}, {}, {}", s7, s8, s9);
}

/// Demonstrates string indexing and slicing
pub fn string_indexing() {
    println!("\n--- String Indexing ---");
    
    let s = String::from("hello");
    // let c = s[0]; // ❌ Error! Can't index strings directly
    
    // String slicing (be careful with UTF-8!)
    let hello = "Здравствуйте"; // Russian "Hello"
    let s = &hello[0..4]; // First 4 bytes (2 chars in Cyrillic)
    println!("Slice: '{}'", s);
    
    // Iterating over chars
    println!("Characters:");
    for c in "नमस्ते".chars() { // Hindi "Hello"
        println!("  {}", c);
    }
    
    // Iterating over bytes
    println!("Bytes:");
    for b in "hello".bytes() {
        println!("  {}", b);
    }
}

/// Demonstrates HashMap basics
/// 
/// HASH MAPS:
/// - Store key-value pairs
/// - Type: HashMap<K, V>
/// - Keys must implement Eq and Hash
pub fn hashmap_basics() {
    println!("\n--- HashMap Basics ---");
    
    // Creating hash maps
    let mut scores = HashMap::new();
    
    // Inserting values
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);
    println!("Scores: {:?}", scores);
    
    // Accessing values
    let team_name = String::from("Blue");
    let score = scores.get(&team_name);
    match score {
        Some(s) => println!("Blue team score: {}", s),
        None => println!("Team not found"),
    }
    
    // Iterating
    println!("All scores:");
    for (key, value) in &scores {
        println!("  {}: {}", key, value);
    }
    
    // Creating from vectors
    let teams = vec![String::from("Red"), String::from("Green")];
    let initial_scores = vec![20, 30];
    let scores2: HashMap<_, _> = teams.iter().zip(initial_scores.iter()).collect();
    println!("Scores from vectors: {:?}", scores2);
}

/// Demonstrates HashMap ownership
pub fn hashmap_ownership() {
    println!("\n--- HashMap Ownership ---");
    
    let field_name = String::from("Favorite color");
    let field_value = String::from("Blue");
    
    let mut map = HashMap::new();
    map.insert(field_name, field_value);
    // field_name and field_value are moved
    // println!("{}", field_name); // ❌ Error!
    
    // References don't move (but must be valid)
    let key = String::from("Key");
    let value = String::from("Value");
    let mut map2 = HashMap::new();
    map2.insert(&key, &value);
    println!("Key still valid: {}", key);
}

/// Demonstrates HashMap updates
pub fn hashmap_updates() {
    println!("\n--- HashMap Updates ---");
    
    let mut scores = HashMap::new();
    
    // Overwriting values
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Blue"), 25);
    println!("After overwrite: {:?}", scores);
    
    // Only insert if key doesn't exist
    scores.entry(String::from("Yellow")).or_insert(50);
    scores.entry(String::from("Blue")).or_insert(50); // Won't insert
    println!("After or_insert: {:?}", scores);
    
    // Update based on old value
    let text = "hello world wonderful world";
    let mut map = HashMap::new();
    
    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }
    
    println!("Word counts: {:?}", map);
}

/// Demonstrates HashSet
/// 
/// HASH SETS:
/// - Collection of unique values
/// - Type: HashSet<T>
/// - Like HashMap but only keys, no values
pub fn hashset_basics() {
    println!("\n--- HashSet Basics ---");
    
    let mut books = HashSet::new();
    
    // Adding values
    books.insert("The Rust Book");
    books.insert("Programming Rust");
    books.insert("The Rust Book"); // Duplicate, won't be added
    
    println!("Books: {:?}", books);
    println!("Number of books: {}", books.len());
    
    // Checking membership
    if books.contains("The Rust Book") {
        println!("We have The Rust Book!");
    }
    
    // Set operations
    let set1: HashSet<_> = [1, 2, 3, 4].iter().cloned().collect();
    let set2: HashSet<_> = [3, 4, 5, 6].iter().cloned().collect();
    
    println!("Set 1: {:?}", set1);
    println!("Set 2: {:?}", set2);
    
    // Union
    let union: HashSet<_> = set1.union(&set2).cloned().collect();
    println!("Union: {:?}", union);
    
    // Intersection
    let intersection: HashSet<_> = set1.intersection(&set2).cloned().collect();
    println!("Intersection: {:?}", intersection);
    
    // Difference
    let difference: HashSet<_> = set1.difference(&set2).cloned().collect();
    println!("Difference: {:?}", difference);
}

/// Demonstrates VecDeque (double-ended queue)
pub fn vecdeque_basics() {
    println!("\n--- VecDeque Basics ---");
    
    let mut deque = VecDeque::new();
    
    // Push to both ends
    deque.push_back(1);
    deque.push_back(2);
    deque.push_front(0);
    
    println!("Deque: {:?}", deque);
    
    // Pop from both ends
    let back = deque.pop_back();
    let front = deque.pop_front();
    println!("Popped back: {:?}, front: {:?}", back, front);
    println!("Remaining: {:?}", deque);
}

/// Demonstrates BTreeMap (sorted map)
pub fn btreemap_basics() {
    println!("\n--- BTreeMap Basics ---");
    
    let mut map = BTreeMap::new();
    
    map.insert(3, "three");
    map.insert(1, "one");
    map.insert(2, "two");
    
    // Automatically sorted by key
    println!("BTreeMap (sorted): {:?}", map);
    
    for (key, value) in &map {
        println!("  {}: {}", key, value);
    }
}

// ============================================================================
// PUBLIC INTERFACE
// ============================================================================

pub fn run_all_examples() {
    vector_basics();
    vector_operations();
    vector_enum_storage();
    string_operations();
    string_indexing();
    hashmap_basics();
    hashmap_ownership();
    hashmap_updates();
    hashset_basics();
    vecdeque_basics();
    btreemap_basics();
}

// ============================================================================
// UNIT TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_vector_operations() {
        let mut v = vec![1, 2, 3];
        v.push(4);
        assert_eq!(v.len(), 4);
        assert_eq!(v.pop(), Some(4));
        assert_eq!(v.len(), 3);
    }
    
    #[test]
    fn test_hashmap_insert() {
        let mut map = HashMap::new();
        map.insert("key", "value");
        assert_eq!(map.get("key"), Some(&"value"));
        assert_eq!(map.get("missing"), None);
    }
    
    #[test]
    fn test_hashset_unique() {
        let mut set = HashSet::new();
        assert!(set.insert(1));
        assert!(!set.insert(1)); // Returns false for duplicate
        assert_eq!(set.len(), 1);
    }
    
    #[test]
    fn test_string_concatenation() {
        let s1 = String::from("Hello, ");
        let s2 = String::from("world!");
        let s3 = s1 + &s2;
        assert_eq!(s3, "Hello, world!");
    }
}
