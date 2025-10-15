// ============================================================================
// MODULE 7: FUNCTIONAL PROGRAMMING
// ============================================================================
// This module covers functional programming features in Rust:
// - Closures (anonymous functions)
// - Iterators and iterator adaptors
// - Functional patterns (map, filter, fold, etc.)
// - Performance considerations
// ============================================================================

/// Demonstrates closure basics
/// 
/// CLOSURES:
/// - Anonymous functions that can capture environment
/// - Syntax: |params| expression or |params| { body }
/// - Type inference for parameters and return type
pub fn closure_basics() {
    println!("\n--- Closure Basics ---");
    
    // Simple closure
    let add_one = |x| x + 1;
    println!("5 + 1 = {}", add_one(5));
    
    // Closure with type annotations
    let add_two = |x: i32| -> i32 { x + 2 };
    println!("5 + 2 = {}", add_two(5));
    
    // Multi-line closure
    let multiply = |x, y| {
        let result = x * y;
        println!("Multiplying {} * {}", x, y);
        result
    };
    println!("Result: {}", multiply(3, 4));
    
    // Closure with no parameters
    let say_hello = || println!("Hello from closure!");
    say_hello();
}

/// Demonstrates closure capturing
/// 
/// CAPTURING:
/// - Closures can capture variables from environment
/// - Three ways: by reference, by mutable reference, by value
pub fn closure_capturing() {
    println!("\n--- Closure Capturing ---");
    
    // Capture by immutable reference
    let x = 10;
    let print_x = || println!("x = {}", x);
    print_x();
    println!("x is still valid: {}", x);
    
    // Capture by mutable reference
    let mut count = 0;
    let mut increment = || {
        count += 1;
        println!("Count: {}", count);
    };
    increment();
    increment();
    increment();
    println!("Final count: {}", count);
    
    // Capture by value (move)
    let s = String::from("hello");
    let consume = move || {
        println!("Consumed: {}", s);
        // s is moved into closure
    };
    consume();
    // println!("{}", s); // ❌ Error! s was moved
}

/// Demonstrates closure traits: Fn, FnMut, FnOnce
/// 
/// CLOSURE TRAITS:
/// - FnOnce: Takes ownership, can be called once
/// - FnMut: Mutable borrow, can be called multiple times
/// - Fn: Immutable borrow, can be called multiple times
pub fn closure_traits() {
    println!("\n--- Closure Traits ---");
    
    // FnOnce: consumes captured variables
    let s = String::from("hello");
    let consume_string = move || {
        println!("Consuming: {}", s);
        drop(s); // Takes ownership
    };
    call_once(consume_string);
    // consume_string(); // ❌ Error! Already called
    
    // FnMut: mutates captured variables
    let mut count = 0;
    let mut increment = || count += 1;
    call_mut(&mut increment);
    call_mut(&mut increment);
    println!("Count after FnMut: {}", count);
    
    // Fn: only reads captured variables
    let x = 10;
    let read_x = || println!("Reading x: {}", x);
    call_fn(&read_x);
    call_fn(&read_x);
}

fn call_once<F: FnOnce()>(f: F) {
    f();
}

fn call_mut<F: FnMut()>(f: &mut F) {
    f();
}

fn call_fn<F: Fn()>(f: &F) {
    f();
}

/// Demonstrates iterator basics
/// 
/// ITERATORS:
/// - Lazy: don't do work until consumed
/// - Implement Iterator trait
/// - next() method returns Option<Item>
pub fn iterator_basics() {
    println!("\n--- Iterator Basics ---");
    
    let v = vec![1, 2, 3, 4, 5];
    
    // Creating iterators
    let iter = v.iter(); // Immutable references
    println!("Iterating with iter():");
    for val in iter {
        println!("  {}", val);
    }
    
    // iter_mut for mutable references
    let mut v2 = vec![1, 2, 3];
    for val in v2.iter_mut() {
        *val *= 2;
    }
    println!("After iter_mut: {:?}", v2);
    
    // into_iter takes ownership
    let v3 = vec![1, 2, 3];
    for val in v3.into_iter() {
        println!("  Owned: {}", val);
    }
    // println!("{:?}", v3); // ❌ Error! v3 was moved
}

/// Demonstrates iterator adaptors
/// 
/// ADAPTORS:
/// - Transform iterators into different iterators
/// - Lazy: don't do work until consumed
/// - Common: map, filter, take, skip, zip, etc.
pub fn iterator_adaptors() {
    println!("\n--- Iterator Adaptors ---");
    
    let v = vec![1, 2, 3, 4, 5];
    
    // map: transform each element
    let doubled: Vec<i32> = v.iter().map(|x| x * 2).collect();
    println!("Doubled: {:?}", doubled);
    
    // filter: keep elements matching predicate
    let evens: Vec<i32> = v.iter().filter(|x| *x % 2 == 0).cloned().collect();
    println!("Evens: {:?}", evens);
    
    // Chaining adaptors
    let result: Vec<i32> = v.iter()
        .filter(|x| *x % 2 == 0)
        .map(|x| x * 2)
        .collect();
    println!("Filtered and doubled: {:?}", result);
    
    // take: first n elements
    let first_three: Vec<i32> = v.iter().take(3).cloned().collect();
    println!("First three: {:?}", first_three);
    
    // skip: skip first n elements
    let skip_two: Vec<i32> = v.iter().skip(2).cloned().collect();
    println!("Skip two: {:?}", skip_two);
    
    // enumerate: add index
    for (i, val) in v.iter().enumerate() {
        println!("  Index {}: {}", i, val);
    }
    
    // zip: combine two iterators
    let names = vec!["Alice", "Bob", "Charlie"];
    let ages = vec![25, 30, 35];
    let combined: Vec<_> = names.iter().zip(ages.iter()).collect();
    println!("Zipped: {:?}", combined);
}

/// Demonstrates consuming adaptors
/// 
/// CONSUMERS:
/// - Consume iterator and produce final value
/// - Common: collect, sum, count, fold, any, all, find
pub fn consuming_adaptors() {
    println!("\n--- Consuming Adaptors ---");
    
    let v = vec![1, 2, 3, 4, 5];
    
    // sum: add all elements
    let total: i32 = v.iter().sum();
    println!("Sum: {}", total);
    
    // count: number of elements
    let count = v.iter().count();
    println!("Count: {}", count);
    
    // any: check if any element matches
    let has_even = v.iter().any(|x| x % 2 == 0);
    println!("Has even: {}", has_even);
    
    // all: check if all elements match
    let all_positive = v.iter().all(|x| *x > 0);
    println!("All positive: {}", all_positive);
    
    // find: first element matching predicate
    let first_even = v.iter().find(|x| *x % 2 == 0);
    println!("First even: {:?}", first_even);
    
    // max and min
    let max = v.iter().max();
    let min = v.iter().min();
    println!("Max: {:?}, Min: {:?}", max, min);
    
    // collect: build collection
    let doubled: Vec<i32> = v.iter().map(|x| x * 2).collect();
    println!("Collected: {:?}", doubled);
}

/// Demonstrates fold and reduce
/// 
/// FOLD/REDUCE:
/// - Accumulate values into single result
/// - fold: with initial value
/// - reduce: without initial value (returns Option)
pub fn fold_and_reduce() {
    println!("\n--- Fold and Reduce ---");
    
    let v = vec![1, 2, 3, 4, 5];
    
    // fold: accumulate with initial value
    let sum = v.iter().fold(0, |acc, x| acc + x);
    println!("Sum with fold: {}", sum);
    
    // fold for product
    let product = v.iter().fold(1, |acc, x| acc * x);
    println!("Product: {}", product);
    
    // reduce: like fold but no initial value
    let sum2 = v.iter().copied().reduce(|acc, x| acc + x);
    println!("Sum with reduce: {:?}", sum2);
    
    // Building a string
    let words = vec!["Hello", "from", "Rust"];
    let sentence = words.iter().fold(String::new(), |mut acc, word| {
        if !acc.is_empty() {
            acc.push(' ');
        }
        acc.push_str(word);
        acc
    });
    println!("Sentence: {}", sentence);
}

/// Demonstrates custom iterators
pub fn custom_iterators() {
    println!("\n--- Custom Iterators ---");
    
    let counter = Counter::new();
    
    for num in counter.take(5) {
        println!("Counter: {}", num);
    }
    
    // Using iterator methods
    let sum: u32 = Counter::new().take(10).sum();
    println!("Sum of first 10: {}", sum);
    
    // Chaining operations
    let result: Vec<u32> = Counter::new()
        .take(10)
        .filter(|x| x % 2 == 0)
        .map(|x| x * x)
        .collect();
    println!("Even squares: {:?}", result);
}

struct Counter {
    count: u32,
}

impl Counter {
    fn new() -> Counter {
        Counter { count: 0 }
    }
}

impl Iterator for Counter {
    type Item = u32;
    
    fn next(&mut self) -> Option<Self::Item> {
        self.count += 1;
        
        if self.count <= 100 {
            Some(self.count)
        } else {
            None
        }
    }
}

/// Demonstrates functional patterns
pub fn functional_patterns() {
    println!("\n--- Functional Patterns ---");
    
    let numbers = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    
    // Pattern: filter-map-collect
    let result: Vec<i32> = numbers.iter()
        .filter(|x| *x % 2 == 0)
        .map(|x| x * x)
        .collect();
    println!("Even squares: {:?}", result);
    
    // Pattern: partition (split into two collections)
    let (evens, odds): (Vec<i32>, Vec<i32>) = numbers.iter()
        .partition(|x| *x % 2 == 0);
    println!("Evens: {:?}, Odds: {:?}", evens, odds);
    
    // Pattern: flat_map (flatten nested iterators)
    let nested = vec![vec![1, 2], vec![3, 4], vec![5, 6]];
    let flattened: Vec<i32> = nested.iter()
        .flat_map(|v| v.iter())
        .cloned()
        .collect();
    println!("Flattened: {:?}", flattened);
    
    // Pattern: scan (stateful map)
    let running_sum: Vec<i32> = numbers.iter()
        .scan(0, |state, x| {
            *state += x;
            Some(*state)
        })
        .collect();
    println!("Running sum: {:?}", running_sum);
}

/// Demonstrates iterator performance
pub fn iterator_performance() {
    println!("\n--- Iterator Performance ---");
    
    println!("Iterators are zero-cost abstractions!");
    println!("They compile to the same code as hand-written loops.");
    
    let v: Vec<i32> = (1..=1000).collect();
    
    // Iterator style (zero-cost!)
    let sum1: i32 = v.iter().sum();
    
    // Loop style (same performance)
    let mut sum2 = 0;
    for i in &v {
        sum2 += i;
    }
    
    println!("Iterator sum: {}", sum1);
    println!("Loop sum: {}", sum2);
    println!("Both have identical performance!");
}

/// Demonstrates practical examples
pub fn practical_examples() {
    println!("\n--- Practical Examples ---");
    
    // Example 1: Process a list of users
    #[derive(Debug)]
    struct User {
        name: String,
        age: u32,
        active: bool,
    }
    
    let users = vec![
        User { name: "Alice".to_string(), age: 25, active: true },
        User { name: "Bob".to_string(), age: 30, active: false },
        User { name: "Charlie".to_string(), age: 35, active: true },
    ];
    
    let active_names: Vec<String> = users.iter()
        .filter(|u| u.active)
        .map(|u| u.name.clone())
        .collect();
    
    println!("Active users: {:?}", active_names);
    
    // Example 2: Word frequency counter
    let text = "hello world hello rust world rust rust";
    let mut word_count = std::collections::HashMap::new();
    
    text.split_whitespace()
        .for_each(|word| {
            *word_count.entry(word).or_insert(0) += 1;
        });
    
    println!("Word count: {:?}", word_count);
    
    // Example 3: Pipeline processing
    let result: i32 = (1..=10)
        .filter(|x| x % 2 == 0)
        .map(|x| x * x)
        .sum();
    
    println!("Sum of even squares (1-10): {}", result);
}

// ============================================================================
// PUBLIC INTERFACE
// ============================================================================

pub fn run_all_examples() {
    closure_basics();
    closure_capturing();
    closure_traits();
    iterator_basics();
    iterator_adaptors();
    consuming_adaptors();
    fold_and_reduce();
    custom_iterators();
    functional_patterns();
    iterator_performance();
    practical_examples();
}

// ============================================================================
// UNIT TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_closure() {
        let add = |x, y| x + y;
        assert_eq!(add(2, 3), 5);
    }
    
    #[test]
    fn test_iterator_map() {
        let v = vec![1, 2, 3];
        let doubled: Vec<i32> = v.iter().map(|x| x * 2).collect();
        assert_eq!(doubled, vec![2, 4, 6]);
    }
    
    #[test]
    fn test_iterator_filter() {
        let v = vec![1, 2, 3, 4, 5];
        let evens: Vec<i32> = v.iter().filter(|x| *x % 2 == 0).cloned().collect();
        assert_eq!(evens, vec![2, 4]);
    }
    
    #[test]
    fn test_fold() {
        let v = vec![1, 2, 3, 4, 5];
        let sum = v.iter().fold(0, |acc, x| acc + x);
        assert_eq!(sum, 15);
    }
    
    #[test]
    fn test_custom_iterator() {
        let counter = Counter::new();
        let sum: u32 = counter.take(5).sum();
        assert_eq!(sum, 15); // 1+2+3+4+5
    }
}
