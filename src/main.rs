// ============================================================================
// HAPPYR - A Comprehensive Rust Learning Project
// ============================================================================
// This project demonstrates ALL major Rust concepts with detailed explanations
// and practical examples. Perfect for beginners who want to master Rust!
//
// HOW TO USE THIS PROJECT:
// 1. Read through each module in order (they build on each other)
// 2. Run the entire project: cargo run
// 3. Run specific examples by uncommenting them in main()
// 4. Experiment by modifying the code and seeing what happens!
// 5. Run tests: cargo test
//
// MODULE ORGANIZATION:
// - basics: Variables, data types, functions, control flow
// - ownership: Ownership rules, borrowing, references, lifetimes
// - structures: Structs, enums, pattern matching
// - traits_generics: Traits, generics, trait bounds
// - collections: Vectors, HashMaps, strings
// - error_handling: Result, Option, custom errors
// - functional: Closures, iterators, functional programming
// - smart_pointers: Box, Rc, RefCell, Arc
// - concurrency: Threads, channels, shared state
// - advanced: Macros, unsafe, FFI, and more
// ============================================================================

// Declare all modules
mod basics;
mod ownership;
mod structures;
mod traits_generics;
mod collections;
mod error_handling;
mod functional;
mod smart_pointers;
mod concurrency;
mod advanced;

fn main() {
    println!("🦀 Welcome to HappyR - Your Comprehensive Rust Learning Journey! 🦀\n");
    println!("=" .repeat(70));
    
    // ========================================================================
    // SECTION 1: BASICS
    // ========================================================================
    println!("\n📚 SECTION 1: RUST BASICS");
    println!("=" .repeat(70));
    basics::run_all_examples();
    
    // ========================================================================
    // SECTION 2: OWNERSHIP & BORROWING
    // ========================================================================
    println!("\n🔐 SECTION 2: OWNERSHIP & BORROWING");
    println!("=" .repeat(70));
    ownership::run_all_examples();
    
    // ========================================================================
    // SECTION 3: STRUCTURES & ENUMS
    // ========================================================================
    println!("\n🏗️  SECTION 3: STRUCTURES & ENUMS");
    println!("=" .repeat(70));
    structures::run_all_examples();
    
    // ========================================================================
    // SECTION 4: TRAITS & GENERICS
    // ========================================================================
    println!("\n🎭 SECTION 4: TRAITS & GENERICS");
    println!("=" .repeat(70));
    traits_generics::run_all_examples();
    
    // ========================================================================
    // SECTION 5: COLLECTIONS
    // ========================================================================
    println!("\n📦 SECTION 5: COLLECTIONS");
    println!("=" .repeat(70));
    collections::run_all_examples();
    
    // ========================================================================
    // SECTION 6: ERROR HANDLING
    // ========================================================================
    println!("\n⚠️  SECTION 6: ERROR HANDLING");
    println!("=" .repeat(70));
    error_handling::run_all_examples();
    
    // ========================================================================
    // SECTION 7: FUNCTIONAL PROGRAMMING
    // ========================================================================
    println!("\n🔄 SECTION 7: FUNCTIONAL PROGRAMMING");
    println!("=" .repeat(70));
    functional::run_all_examples();
    
    // ========================================================================
    // SECTION 8: SMART POINTERS
    // ========================================================================
    println!("\n🧠 SECTION 8: SMART POINTERS");
    println!("=" .repeat(70));
    smart_pointers::run_all_examples();
    
    // ========================================================================
    // SECTION 9: CONCURRENCY
    // ========================================================================
    println!("\n⚡ SECTION 9: CONCURRENCY");
    println!("=" .repeat(70));
    concurrency::run_all_examples();
    
    // ========================================================================
    // SECTION 10: ADVANCED FEATURES
    // ========================================================================
    println!("\n🚀 SECTION 10: ADVANCED FEATURES");
    println!("=" .repeat(70));
    advanced::run_all_examples();
    
    // ========================================================================
    // CONCLUSION
    // ========================================================================
    println!("\n" .repeat(2));
    println!("=" .repeat(70));
    println!("🎉 Congratulations! You've completed the Rust learning journey! 🎉");
    println!("=" .repeat(70));
    println!("\n💡 Next Steps:");
    println!("   1. Modify the examples and experiment");
    println!("   2. Run 'cargo test' to see all unit tests");
    println!("   3. Build your own project using these concepts");
    println!("   4. Check out the Rust Book: https://doc.rust-lang.org/book/");
    println!("\nHappy Coding! 🦀\n");
}
