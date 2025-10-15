// ============================================================================
// MODULE 8: SMART POINTERS
// ============================================================================
// This module covers Rust's smart pointer types:
// - Box<T> for heap allocation
// - Rc<T> for reference counting
// - RefCell<T> for interior mutability
// - Arc<T> for atomic reference counting
// - Weak<T> for breaking reference cycles
// - Deref and Drop traits
// ============================================================================

use std::rc::Rc;
use std::cell::RefCell;
use std::ops::Deref;

/// Demonstrates Box<T> for heap allocation
/// 
/// BOX:
/// - Allocates data on the heap
/// - Fixed size (pointer on stack)
/// - Useful for recursive types, large data, trait objects
pub fn box_basics() {
    println!("\n--- Box Basics ---");
    
    // Simple heap allocation
    let b = Box::new(5);
    println!("Boxed value: {}", b);
    
    // Box is automatically dereferenced
    let x = *b + 1;
    println!("Dereferenced: {}", x);
    
    // Recursive type with Box
    let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));
    println!("List created: {:?}", list);
    
    // Large data on heap
    let large_array = Box::new([0; 1000]);
    println!("Large array on heap (first element): {}", large_array[0]);
}

#[derive(Debug)]
enum List {
    Cons(i32, Box<List>),
    Nil,
}

use List::{Cons, Nil};

/// Demonstrates the Deref trait
/// 
/// DEREF:
/// - Allows treating smart pointers like references
/// - Enables deref coercion
pub fn deref_trait() {
    println!("\n--- Deref Trait ---");
    
    let x = 5;
    let y = Box::new(x);
    
    assert_eq!(5, x);
    assert_eq!(5, *y); // Deref Box to get value
    
    // Custom smart pointer
    let z = MyBox::new(x);
    assert_eq!(5, *z); // Works because MyBox implements Deref
    
    // Deref coercion
    let m = MyBox::new(String::from("Rust"));
    hello(&m); // &MyBox<String> -> &String -> &str
    
    println!("Deref coercion works!");
}

struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

impl<T> Deref for MyBox<T> {
    type Target = T;
    
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

fn hello(name: &str) {
    println!("Hello, {}!", name);
}

/// Demonstrates the Drop trait
/// 
/// DROP:
/// - Customize cleanup when value goes out of scope
/// - Called automatically
/// - Can't call drop() manually (use std::mem::drop)
pub fn drop_trait() {
    println!("\n--- Drop Trait ---");
    
    let c = CustomSmartPointer {
        data: String::from("my stuff"),
    };
    
    let d = CustomSmartPointer {
        data: String::from("other stuff"),
    };
    
    println!("CustomSmartPointers created.");
    
    // Early drop with std::mem::drop
    drop(c);
    println!("CustomSmartPointer c dropped early.");
    
    println!("End of function (d will be dropped).");
}

struct CustomSmartPointer {
    data: String,
}

impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        println!("Dropping CustomSmartPointer with data: {}", self.data);
    }
}

/// Demonstrates Rc<T> for reference counting
/// 
/// RC (REFERENCE COUNTED):
/// - Multiple ownership
/// - Keeps track of number of references
/// - Deallocates when count reaches zero
/// - Only for single-threaded scenarios
pub fn rc_basics() {
    println!("\n--- Rc Basics ---");
    
    let a = Rc::new(Cons2(5, Rc::new(Cons2(10, Rc::new(Nil2)))));
    println!("Reference count after creating a: {}", Rc::strong_count(&a));
    
    let b = Cons2(3, Rc::clone(&a)); // Increment reference count
    println!("Reference count after creating b: {}", Rc::strong_count(&a));
    
    {
        let c = Cons2(4, Rc::clone(&a));
        println!("Reference count after creating c: {}", Rc::strong_count(&a));
    } // c goes out of scope
    
    println!("Reference count after c goes out of scope: {}", Rc::strong_count(&a));
    
    println!("List b: {:?}", b);
}

#[derive(Debug)]
enum List2 {
    Cons2(i32, Rc<List2>),
    Nil2,
}

use List2::{Cons2, Nil2};

/// Demonstrates RefCell<T> for interior mutability
/// 
/// REFCELL:
/// - Allows mutation through immutable reference
/// - Enforces borrowing rules at runtime (not compile time)
/// - Panics if rules violated
/// - Single-threaded only
pub fn refcell_basics() {
    println!("\n--- RefCell Basics ---");
    
    let value = RefCell::new(5);
    
    // Borrow immutably
    {
        let borrowed = value.borrow();
        println!("Borrowed value: {}", *borrowed);
    } // borrowed goes out of scope
    
    // Borrow mutably
    {
        let mut borrowed_mut = value.borrow_mut();
        *borrowed_mut += 10;
        println!("Modified value: {}", *borrowed_mut);
    }
    
    println!("Final value: {}", *value.borrow());
    
    // Multiple immutable borrows OK
    let borrow1 = value.borrow();
    let borrow2 = value.borrow();
    println!("Multiple borrows: {} and {}", *borrow1, *borrow2);
}

/// Demonstrates combining Rc and RefCell
/// 
/// RC + REFCELL:
/// - Multiple ownership with interior mutability
/// - Common pattern in Rust
pub fn rc_refcell_combination() {
    println!("\n--- Rc + RefCell ---");
    
    let value = Rc::new(RefCell::new(5));
    
    let a = Rc::clone(&value);
    let b = Rc::clone(&value);
    
    *a.borrow_mut() += 10;
    println!("After a modifies: {}", *value.borrow());
    
    *b.borrow_mut() += 20;
    println!("After b modifies: {}", *value.borrow());
    
    println!("Reference count: {}", Rc::strong_count(&value));
}

/// Demonstrates reference cycles and memory leaks
pub fn reference_cycles() {
    println!("\n--- Reference Cycles ---");
    
    println!("Reference cycles can cause memory leaks!");
    println!("Example: Two Rc values pointing to each other");
    println!("Solution: Use Weak<T> to break cycles");
    
    // This would create a cycle (commented to avoid leak):
    // let a = Rc::new(RefCell::new(Cons3(5, RefCell::new(Rc::new(Nil3)))));
    // let b = Rc::new(Cons3(10, RefCell::new(Rc::clone(&a))));
    // if let Some(link) = a.borrow_mut().tail() {
    //     *link = Rc::clone(&b); // Creates cycle!
    // }
}

/// Demonstrates Weak<T> for breaking cycles
/// 
/// WEAK:
/// - Non-owning reference
/// - Doesn't increase reference count
/// - Must upgrade to Rc to use
/// - Returns None if value dropped
pub fn weak_references() {
    println!("\n--- Weak References ---");
    
    let leaf = Rc::new(Node {
        value: 3,
        parent: RefCell::new(std::rc::Weak::new()),
        children: RefCell::new(vec![]),
    });
    
    println!("Leaf strong: {}, weak: {}", 
             Rc::strong_count(&leaf), 
             Rc::weak_count(&leaf));
    
    {
        let branch = Rc::new(Node {
            value: 5,
            parent: RefCell::new(std::rc::Weak::new()),
            children: RefCell::new(vec![Rc::clone(&leaf)]),
        });
        
        *leaf.parent.borrow_mut() = Rc::downgrade(&branch);
        
        println!("Branch strong: {}, weak: {}", 
                 Rc::strong_count(&branch), 
                 Rc::weak_count(&branch));
        
        println!("Leaf strong: {}, weak: {}", 
                 Rc::strong_count(&leaf), 
                 Rc::weak_count(&leaf));
    }
    
    println!("Leaf parent after branch dropped: {:?}", 
             leaf.parent.borrow().upgrade());
}

struct Node {
    value: i32,
    parent: RefCell<std::rc::Weak<Node>>,
    children: RefCell<Vec<Rc<Node>>>,
}

/// Demonstrates practical use cases
pub fn practical_examples() {
    println!("\n--- Practical Examples ---");
    
    // Use case 1: Trait objects with Box
    let shapes: Vec<Box<dyn Shape>> = vec![
        Box::new(Circle { radius: 5.0 }),
        Box::new(Rectangle { width: 10.0, height: 20.0 }),
    ];
    
    for shape in shapes {
        println!("Area: {}", shape.area());
    }
    
    // Use case 2: Shared state with Rc
    let shared_data = Rc::new(vec![1, 2, 3, 4, 5]);
    let reader1 = Rc::clone(&shared_data);
    let reader2 = Rc::clone(&shared_data);
    
    println!("Reader 1: {:?}", reader1);
    println!("Reader 2: {:?}", reader2);
    
    // Use case 3: Mock object with RefCell
    let mock = MockDatabase {
        data: RefCell::new(vec![]),
    };
    
    mock.insert("key1".to_string(), "value1".to_string());
    mock.insert("key2".to_string(), "value2".to_string());
    
    println!("Mock database: {:?}", mock.data.borrow());
}

trait Shape {
    fn area(&self) -> f64;
}

struct Circle {
    radius: f64,
}

impl Shape for Circle {
    fn area(&self) -> f64 {
        std::f64::consts::PI * self.radius * self.radius
    }
}

struct Rectangle {
    width: f64,
    height: f64,
}

impl Shape for Rectangle {
    fn area(&self) -> f64 {
        self.width * self.height
    }
}

struct MockDatabase {
    data: RefCell<Vec<(String, String)>>,
}

impl MockDatabase {
    fn insert(&self, key: String, value: String) {
        self.data.borrow_mut().push((key, value));
    }
}

/// Demonstrates when to use each smart pointer
pub fn choosing_smart_pointers() {
    println!("\n--- Choosing Smart Pointers ---");
    
    println!("Use Box<T> when:");
    println!("  - You have a large amount of data to transfer ownership");
    println!("  - You want to own a value and only care that it implements a trait");
    println!("  - You need recursive types");
    
    println!("\nUse Rc<T> when:");
    println!("  - You need multiple owners of the same data");
    println!("  - Single-threaded scenarios");
    println!("  - Read-only shared data");
    
    println!("\nUse RefCell<T> when:");
    println!("  - You need interior mutability");
    println!("  - You're sure borrowing rules are followed (runtime check)");
    println!("  - Single-threaded scenarios");
    
    println!("\nUse Arc<T> when:");
    println!("  - Like Rc<T> but for multi-threaded scenarios");
    println!("  - Atomic reference counting (thread-safe)");
    
    println!("\nUse Weak<T> when:");
    println!("  - You need to break reference cycles");
    println!("  - Parent-child relationships");
}

// ============================================================================
// PUBLIC INTERFACE
// ============================================================================

pub fn run_all_examples() {
    box_basics();
    deref_trait();
    drop_trait();
    rc_basics();
    refcell_basics();
    rc_refcell_combination();
    reference_cycles();
    weak_references();
    practical_examples();
    choosing_smart_pointers();
}

// ============================================================================
// UNIT TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_box_deref() {
        let x = Box::new(5);
        assert_eq!(5, *x);
    }
    
    #[test]
    fn test_rc_clone() {
        let a = Rc::new(5);
        let b = Rc::clone(&a);
        assert_eq!(2, Rc::strong_count(&a));
        assert_eq!(*a, *b);
    }
    
    #[test]
    fn test_refcell_borrow() {
        let value = RefCell::new(5);
        *value.borrow_mut() += 10;
        assert_eq!(15, *value.borrow());
    }
    
    #[test]
    #[should_panic]
    fn test_refcell_panic() {
        let value = RefCell::new(5);
        let _borrow1 = value.borrow_mut();
        let _borrow2 = value.borrow_mut(); // Panics!
    }
}
