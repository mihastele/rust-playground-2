// ============================================================================
// MODULE 4: TRAITS & GENERICS
// ============================================================================
// This module covers Rust's abstraction mechanisms:
// - Generic types and functions
// - Trait definitions and implementations
// - Trait bounds
// - Default implementations
// - Operator overloading
// - Associated types
// ============================================================================

/// Demonstrates generic functions
/// 
/// GENERICS:
/// - Write code that works with multiple types
/// - Type parameters in angle brackets: <T>
/// - No runtime cost (monomorphization)
pub fn generic_functions() {
    println!("\n--- Generic Functions ---");
    
    let numbers = vec![34, 50, 25, 100, 65];
    let result = largest(&numbers);
    println!("Largest number: {}", result);
    
    let chars = vec!['y', 'm', 'a', 'q'];
    let result = largest(&chars);
    println!("Largest char: {}", result);
    
    // Multiple type parameters
    let p1 = Point { x: 5, y: 10 };
    let p2 = Point { x: 1.0, y: 4.0 };
    println!("Integer point: ({}, {})", p1.x, p1.y);
    println!("Float point: ({}, {})", p2.x, p2.y);
    
    let p3 = MixedPoint { x: 5, y: 4.0 };
    println!("Mixed point: ({}, {})", p3.x, p3.y);
}

// Generic function with trait bound
fn largest<T: PartialOrd + Copy>(list: &[T]) -> T {
    let mut largest = list[0];
    
    for &item in list.iter() {
        if item > largest {
            largest = item;
        }
    }
    
    largest
}

// Generic struct with single type parameter
struct Point<T> {
    x: T,
    y: T,
}

// Generic struct with multiple type parameters
struct MixedPoint<T, U> {
    x: T,
    y: U,
}

/// Demonstrates generic implementations
pub fn generic_implementations() {
    println!("\n--- Generic Implementations ---");
    
    let p = Point { x: 5, y: 10 };
    println!("X coordinate: {}", p.x());
    
    let p2 = Point { x: 1.0, y: 4.0 };
    println!("Distance from origin: {:.2}", p2.distance_from_origin());
    
    let p3 = Point { x: 5, y: 10 };
    let p4 = Point { x: 1, y: 2 };
    let p5 = p3.mixup(p4);
    println!("Mixed up point: ({}, {})", p5.x, p5.y);
}

// Implementation for all types T
impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
}

// Implementation only for f64
impl Point<f64> {
    fn distance_from_origin(&self) -> f64 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}

// Implementation with multiple type parameters
impl<T, U> MixedPoint<T, U> {
    fn mixup<V, W>(self, other: MixedPoint<V, W>) -> MixedPoint<T, W> {
        MixedPoint {
            x: self.x,
            y: other.y,
        }
    }
}

/// Demonstrates trait definitions and implementations
/// 
/// TRAITS:
/// - Define shared behavior
/// - Similar to interfaces in other languages
/// - Can have default implementations
pub fn trait_basics() {
    println!("\n--- Trait Basics ---");
    
    let article = NewsArticle {
        headline: String::from("Rust 2.0 Released!"),
        location: String::from("San Francisco, CA"),
        author: String::from("Jane Doe"),
        content: String::from("The Rust team is excited to announce..."),
    };
    
    let tweet = Tweet {
        username: String::from("rustlang"),
        content: String::from("Rust is amazing! #rustlang"),
        reply: false,
        retweet: false,
    };
    
    println!("Article summary: {}", article.summarize());
    println!("Tweet summary: {}", tweet.summarize());
    
    // Using default implementation
    println!("Article author: {}", article.author_summary());
}

// Trait definition
pub trait Summary {
    fn summarize(&self) -> String;
    
    // Default implementation
    fn author_summary(&self) -> String {
        String::from("(Read more...)")
    }
}

struct NewsArticle {
    headline: String,
    location: String,
    author: String,
    content: String,
}

impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }
    
    // Override default implementation
    fn author_summary(&self) -> String {
        format!("By {}", self.author)
    }
}

struct Tweet {
    username: String,
    content: String,
    reply: bool,
    retweet: bool,
}

impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}

/// Demonstrates trait bounds
/// 
/// TRAIT BOUNDS:
/// - Restrict generic types to those implementing specific traits
/// - Syntax: <T: Trait> or where T: Trait
pub fn trait_bounds() {
    println!("\n--- Trait Bounds ---");
    
    let article = NewsArticle {
        headline: String::from("Breaking News"),
        location: String::from("New York"),
        author: String::from("John Smith"),
        content: String::from("Important content..."),
    };
    
    notify(&article);
    notify_verbose(&article);
    
    let tweet = Tweet {
        username: String::from("user123"),
        content: String::from("Hello world!"),
        reply: false,
        retweet: false,
    };
    
    notify(&tweet);
}

// Trait bound syntax
pub fn notify(item: &impl Summary) {
    println!("Breaking news! {}", item.summarize());
}

// Trait bound with generic type
pub fn notify_verbose<T: Summary>(item: &T) {
    println!("Notification: {}", item.summarize());
}

// Multiple trait bounds
pub fn notify_display<T: Summary + std::fmt::Display>(item: &T) {
    println!("Display: {}", item);
}

// Where clause for complex bounds
pub fn some_function<T, U>(_t: &T, _u: &U) -> i32
where
    T: std::fmt::Display + Clone,
    U: Clone + std::fmt::Debug,
{
    42
}

/// Demonstrates returning types that implement traits
pub fn returning_traits() {
    println!("\n--- Returning Traits ---");
    
    let tweet = returns_summarizable(true);
    println!("Returned: {}", tweet.summarize());
}

// Return type that implements trait
fn returns_summarizable(is_tweet: bool) -> impl Summary {
    // Note: Can only return one concrete type
    // This would fail if we tried to return different types
    Tweet {
        username: String::from("rustacean"),
        content: String::from("Learning Rust!"),
        reply: false,
        retweet: false,
    }
}

/// Demonstrates trait bounds with conditional implementations
pub fn conditional_implementations() {
    println!("\n--- Conditional Implementations ---");
    
    let pair1 = Pair::new(10, 20);
    pair1.cmp_display();
    
    let pair2 = Pair::new("hello", "world");
    // pair2.cmp_display(); // Would work if &str implemented PartialOrd
}

struct Pair<T> {
    x: T,
    y: T,
}

impl<T> Pair<T> {
    fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}

// Conditional implementation: only if T implements Display + PartialOrd
impl<T: std::fmt::Display + PartialOrd> Pair<T> {
    fn cmp_display(&self) {
        if self.x >= self.y {
            println!("The largest member is x = {}", self.x);
        } else {
            println!("The largest member is y = {}", self.y);
        }
    }
}

/// Demonstrates common standard library traits
pub fn standard_traits() {
    println!("\n--- Standard Library Traits ---");
    
    // Clone trait
    let s1 = String::from("hello");
    let s2 = s1.clone();
    println!("Original: {}, Clone: {}", s1, s2);
    
    // Copy trait (implicit)
    let x = 5;
    let y = x; // Copy, not move
    println!("x: {}, y: {}", x, y);
    
    // Debug trait
    let point = DebugPoint { x: 10, y: 20 };
    println!("Debug: {:?}", point);
    println!("Pretty debug: {:#?}", point);
    
    // PartialEq and Eq
    let p1 = DebugPoint { x: 1, y: 2 };
    let p2 = DebugPoint { x: 1, y: 2 };
    println!("Points equal: {}", p1 == p2);
    
    // PartialOrd and Ord
    println!("Point comparison: {:?}", p1.partial_cmp(&p2));
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
struct DebugPoint {
    x: i32,
    y: i32,
}

/// Demonstrates operator overloading with traits
pub fn operator_overloading() {
    println!("\n--- Operator Overloading ---");
    
    let p1 = AddPoint { x: 1, y: 2 };
    let p2 = AddPoint { x: 3, y: 4 };
    let p3 = p1 + p2; // Uses Add trait
    
    println!("({}, {}) + ({}, {}) = ({}, {})", 
             p1.x, p1.y, p2.x, p2.y, p3.x, p3.y);
}

#[derive(Debug, Copy, Clone)]
struct AddPoint {
    x: i32,
    y: i32,
}

// Implement Add trait for operator overloading
impl std::ops::Add for AddPoint {
    type Output = AddPoint;
    
    fn add(self, other: AddPoint) -> AddPoint {
        AddPoint {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

/// Demonstrates associated types
/// 
/// ASSOCIATED TYPES:
/// - Placeholder types in trait definitions
/// - Specified when implementing the trait
pub fn associated_types() {
    println!("\n--- Associated Types ---");
    
    let counter = Counter { count: 0 };
    
    for (i, num) in counter.enumerate() {
        if i >= 5 {
            break;
        }
        println!("Count: {}", num);
    }
}

struct Counter {
    count: u32,
}

impl Iterator for Counter {
    type Item = u32; // Associated type
    
    fn next(&mut self) -> Option<Self::Item> {
        if self.count < 10 {
            self.count += 1;
            Some(self.count)
        } else {
            None
        }
    }
}

/// Demonstrates supertraits
pub fn supertraits() {
    println!("\n--- Supertraits ---");
    
    let point = OutlinePoint { x: 1, y: 2 };
    point.outline_print();
}

struct OutlinePoint {
    x: i32,
    y: i32,
}

// Display is a supertrait of OutlinePrint
trait OutlinePrint: std::fmt::Display {
    fn outline_print(&self) {
        let output = self.to_string();
        let len = output.len();
        println!("{}", "*".repeat(len + 4));
        println!("*{}*", " ".repeat(len + 2));
        println!("* {} *", output);
        println!("*{}*", " ".repeat(len + 2));
        println!("{}", "*".repeat(len + 4));
    }
}

impl OutlinePrint for OutlinePoint {}

impl std::fmt::Display for OutlinePoint {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

/// Demonstrates newtype pattern
pub fn newtype_pattern() {
    println!("\n--- Newtype Pattern ---");
    
    let w = Wrapper(vec![String::from("hello"), String::from("world")]);
    println!("Wrapper: {}", w);
}

// Newtype: wrapper around Vec to implement Display
struct Wrapper(Vec<String>);

impl std::fmt::Display for Wrapper {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "[{}]", self.0.join(", "))
    }
}

// ============================================================================
// PUBLIC INTERFACE
// ============================================================================

pub fn run_all_examples() {
    generic_functions();
    generic_implementations();
    trait_basics();
    trait_bounds();
    returning_traits();
    conditional_implementations();
    standard_traits();
    operator_overloading();
    associated_types();
    supertraits();
    newtype_pattern();
}

// ============================================================================
// UNIT TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_largest() {
        let numbers = vec![34, 50, 25, 100, 65];
        assert_eq!(largest(&numbers), 100);
        
        let chars = vec!['y', 'm', 'a', 'q'];
        assert_eq!(largest(&chars), 'y');
    }
    
    #[test]
    fn test_point_distance() {
        let p = Point { x: 3.0, y: 4.0 };
        assert_eq!(p.distance_from_origin(), 5.0);
    }
    
    #[test]
    fn test_add_points() {
        let p1 = AddPoint { x: 1, y: 2 };
        let p2 = AddPoint { x: 3, y: 4 };
        let p3 = p1 + p2;
        assert_eq!(p3.x, 4);
        assert_eq!(p3.y, 6);
    }
    
    #[test]
    fn test_counter() {
        let counter = Counter { count: 0 };
        let sum: u32 = counter.take(5).sum();
        assert_eq!(sum, 15); // 1+2+3+4+5
    }
}
