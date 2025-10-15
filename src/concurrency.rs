// ============================================================================
// MODULE 9: CONCURRENCY
// ============================================================================
// This module covers concurrent programming in Rust:
// - Threads and thread spawning
// - Message passing with channels
// - Shared state with Mutex and Arc
// - Send and Sync traits
// - Thread safety guarantees
// ============================================================================

use std::thread;
use std::time::Duration;
use std::sync::{mpsc, Mutex, Arc};

/// Demonstrates basic thread creation
/// 
/// THREADS:
/// - Concurrent execution
/// - thread::spawn creates new thread
/// - join() waits for thread to finish
pub fn thread_basics() {
    println!("\n--- Thread Basics ---");
    
    // Spawn a new thread
    let handle = thread::spawn(|| {
        for i in 1..=5 {
            println!("  Thread: count {}", i);
            thread::sleep(Duration::from_millis(1));
        }
    });
    
    // Main thread continues
    for i in 1..=3 {
        println!("Main: count {}", i);
        thread::sleep(Duration::from_millis(1));
    }
    
    // Wait for spawned thread to finish
    handle.join().unwrap();
    println!("Thread finished!");
}

/// Demonstrates move closures with threads
/// 
/// MOVE:
/// - Threads need ownership of captured variables
/// - Use move keyword to transfer ownership
pub fn thread_move() {
    println!("\n--- Thread Move ---");
    
    let v = vec![1, 2, 3];
    
    let handle = thread::spawn(move || {
        println!("  Thread has vector: {:?}", v);
    });
    
    // v is moved, can't use here
    // println!("{:?}", v); // ❌ Error!
    
    handle.join().unwrap();
}

/// Demonstrates message passing with channels
/// 
/// CHANNELS:
/// - mpsc: multiple producer, single consumer
/// - tx: transmitter (sender)
/// - rx: receiver
/// - Thread-safe communication
pub fn channel_basics() {
    println!("\n--- Channel Basics ---");
    
    let (tx, rx) = mpsc::channel();
    
    thread::spawn(move || {
        let val = String::from("hello from thread");
        tx.send(val).unwrap();
        // val is moved, can't use here
    });
    
    let received = rx.recv().unwrap();
    println!("Main received: {}", received);
}

/// Demonstrates sending multiple messages
pub fn channel_multiple_messages() {
    println!("\n--- Multiple Messages ---");
    
    let (tx, rx) = mpsc::channel();
    
    thread::spawn(move || {
        let vals = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("thread"),
        ];
        
        for val in vals {
            tx.send(val).unwrap();
            thread::sleep(Duration::from_millis(100));
        }
    });
    
    // Receive messages as they arrive
    for received in rx {
        println!("Main received: {}", received);
    }
}

/// Demonstrates multiple producers
pub fn channel_multiple_producers() {
    println!("\n--- Multiple Producers ---");
    
    let (tx, rx) = mpsc::channel();
    
    // Clone transmitter for multiple producers
    let tx1 = tx.clone();
    thread::spawn(move || {
        let vals = vec![
            String::from("thread 1: hi"),
            String::from("thread 1: more"),
        ];
        for val in vals {
            tx1.send(val).unwrap();
            thread::sleep(Duration::from_millis(100));
        }
    });
    
    thread::spawn(move || {
        let vals = vec![
            String::from("thread 2: hello"),
            String::from("thread 2: world"),
        ];
        for val in vals {
            tx.send(val).unwrap();
            thread::sleep(Duration::from_millis(100));
        }
    });
    
    // Receive from both threads
    for received in rx.iter().take(4) {
        println!("Received: {}", received);
    }
}

/// Demonstrates shared state with Mutex
/// 
/// MUTEX:
/// - Mutual exclusion lock
/// - Only one thread can access data at a time
/// - lock() acquires the lock (blocks if held)
/// - Automatically released when guard goes out of scope
pub fn mutex_basics() {
    println!("\n--- Mutex Basics ---");
    
    let m = Mutex::new(5);
    
    {
        let mut num = m.lock().unwrap();
        *num = 6;
        println!("Modified value: {}", *num);
    } // Lock released here
    
    println!("Value after lock released: {:?}", m);
}

/// Demonstrates Arc with Mutex for shared state
/// 
/// ARC:
/// - Atomic Reference Counted (thread-safe Rc)
/// - Allows multiple ownership across threads
/// - Combined with Mutex for shared mutable state
pub fn arc_mutex() {
    println!("\n--- Arc + Mutex ---");
    
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];
    
    for i in 0..10 {
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();
            *num += 1;
            println!("  Thread {} incremented counter", i);
        });
        handles.push(handle);
    }
    
    // Wait for all threads
    for handle in handles {
        handle.join().unwrap();
    }
    
    println!("Final counter value: {}", *counter.lock().unwrap());
}

/// Demonstrates Send and Sync traits
/// 
/// SEND:
/// - Type can be transferred between threads
/// - Most types are Send
/// 
/// SYNC:
/// - Type can be referenced from multiple threads
/// - &T is Send if T is Sync
pub fn send_sync_traits() {
    println!("\n--- Send and Sync ---");
    
    println!("Send trait:");
    println!("  - Allows transferring ownership between threads");
    println!("  - Most types are Send (except Rc, raw pointers)");
    
    println!("\nSync trait:");
    println!("  - Allows multiple threads to access via &T");
    println!("  - Types like Mutex, Arc are Sync");
    println!("  - RefCell is NOT Sync (not thread-safe)");
    
    // Example: i32 is both Send and Sync
    let num = 42;
    let handle = thread::spawn(move || {
        println!("  Thread has num: {}", num);
    });
    handle.join().unwrap();
}

/// Demonstrates thread pools (conceptual)
pub fn thread_pool_concept() {
    println!("\n--- Thread Pool Concept ---");
    
    println!("Thread pools reuse threads for multiple tasks");
    println!("Benefits:");
    println!("  - Reduced overhead (no thread creation per task)");
    println!("  - Limited concurrency (control resource usage)");
    println!("  - Better performance for many small tasks");
    
    // Simple example with fixed number of threads
    let handles: Vec<_> = (0..4)
        .map(|i| {
            thread::spawn(move || {
                println!("  Worker thread {} processing", i);
                thread::sleep(Duration::from_millis(100));
                i * 2
            })
        })
        .collect();
    
    let results: Vec<_> = handles
        .into_iter()
        .map(|h| h.join().unwrap())
        .collect();
    
    println!("Results: {:?}", results);
}

/// Demonstrates deadlock prevention
pub fn deadlock_prevention() {
    println!("\n--- Deadlock Prevention ---");
    
    println!("Deadlock occurs when:");
    println!("  - Thread A holds lock 1, waits for lock 2");
    println!("  - Thread B holds lock 2, waits for lock 1");
    
    println!("\nPrevention strategies:");
    println!("  1. Always acquire locks in same order");
    println!("  2. Use try_lock() instead of lock()");
    println!("  3. Use timeout with lock acquisition");
    println!("  4. Minimize lock scope");
    println!("  5. Avoid nested locks when possible");
    
    // Example: proper lock ordering
    let lock1 = Arc::new(Mutex::new(1));
    let lock2 = Arc::new(Mutex::new(2));
    
    let lock1_clone = Arc::clone(&lock1);
    let lock2_clone = Arc::clone(&lock2);
    
    let handle1 = thread::spawn(move || {
        let _l1 = lock1_clone.lock().unwrap();
        thread::sleep(Duration::from_millis(10));
        let _l2 = lock2_clone.lock().unwrap();
        println!("  Thread 1 acquired both locks");
    });
    
    let handle2 = thread::spawn(move || {
        let _l1 = lock1.lock().unwrap(); // Same order!
        thread::sleep(Duration::from_millis(10));
        let _l2 = lock2.lock().unwrap();
        println!("  Thread 2 acquired both locks");
    });
    
    handle1.join().unwrap();
    handle2.join().unwrap();
}

/// Demonstrates parallel computation
pub fn parallel_computation() {
    println!("\n--- Parallel Computation ---");
    
    // Compute sum of squares in parallel
    let data: Vec<i32> = (1..=100).collect();
    let chunk_size = 25;
    
    let handles: Vec<_> = data
        .chunks(chunk_size)
        .map(|chunk| {
            let chunk = chunk.to_vec();
            thread::spawn(move || {
                chunk.iter().map(|x| x * x).sum::<i32>()
            })
        })
        .collect();
    
    let total: i32 = handles
        .into_iter()
        .map(|h| h.join().unwrap())
        .sum();
    
    println!("Sum of squares (1-100): {}", total);
}

/// Demonstrates scoped threads (conceptual for older Rust)
pub fn scoped_threads_concept() {
    println!("\n--- Scoped Threads ---");
    
    println!("Scoped threads (std::thread::scope in Rust 1.63+):");
    println!("  - Can borrow local variables");
    println!("  - Guaranteed to finish before scope ends");
    println!("  - No need for Arc or move in many cases");
    
    let mut data = vec![1, 2, 3, 4, 5];
    
    thread::scope(|s| {
        s.spawn(|| {
            println!("  Thread can read data: {:?}", data);
        });
        
        s.spawn(|| {
            println!("  Another thread reading: len = {}", data.len());
        });
    }); // All threads guaranteed to finish here
    
    data.push(6);
    println!("Data after threads: {:?}", data);
}

/// Demonstrates practical concurrency patterns
pub fn practical_patterns() {
    println!("\n--- Practical Patterns ---");
    
    // Pattern 1: Worker pool
    println!("\n1. Worker Pool Pattern:");
    let (tx, rx) = mpsc::channel();
    let rx = Arc::new(Mutex::new(rx));
    
    let mut workers = vec![];
    for id in 0..3 {
        let rx = Arc::clone(&rx);
        let worker = thread::spawn(move || {
            loop {
                let job = rx.lock().unwrap().recv();
                match job {
                    Ok(num) => {
                        println!("  Worker {} processing: {}", id, num);
                        thread::sleep(Duration::from_millis(50));
                    }
                    Err(_) => break,
                }
            }
        });
        workers.push(worker);
    }
    
    // Send jobs
    for i in 1..=9 {
        tx.send(i).unwrap();
    }
    drop(tx); // Close channel
    
    // Wait for workers
    for worker in workers {
        worker.join().unwrap();
    }
    
    // Pattern 2: Fan-out, Fan-in
    println!("\n2. Fan-out, Fan-in Pattern:");
    let (tx, rx) = mpsc::channel();
    
    // Fan-out: multiple workers
    for i in 0..3 {
        let tx = tx.clone();
        thread::spawn(move || {
            let result = i * i;
            tx.send(result).unwrap();
        });
    }
    drop(tx);
    
    // Fan-in: collect results
    let results: Vec<_> = rx.iter().collect();
    println!("  Results: {:?}", results);
}

/// Demonstrates thread safety guarantees
pub fn thread_safety() {
    println!("\n--- Thread Safety ---");
    
    println!("Rust's thread safety guarantees:");
    println!("  1. Data races impossible at compile time");
    println!("  2. Send trait prevents unsafe transfers");
    println!("  3. Sync trait prevents unsafe sharing");
    println!("  4. Mutex ensures exclusive access");
    println!("  5. Arc provides thread-safe reference counting");
    
    println!("\nCommon thread-safe types:");
    println!("  - Arc<T> (atomic reference counting)");
    println!("  - Mutex<T> (mutual exclusion)");
    println!("  - RwLock<T> (reader-writer lock)");
    println!("  - Atomic types (AtomicBool, AtomicI32, etc.)");
}

// ============================================================================
// PUBLIC INTERFACE
// ============================================================================

pub fn run_all_examples() {
    thread_basics();
    thread_move();
    channel_basics();
    channel_multiple_messages();
    channel_multiple_producers();
    mutex_basics();
    arc_mutex();
    send_sync_traits();
    thread_pool_concept();
    deadlock_prevention();
    parallel_computation();
    scoped_threads_concept();
    practical_patterns();
    thread_safety();
}

// ============================================================================
// UNIT TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_thread_spawn() {
        let handle = thread::spawn(|| {
            42
        });
        assert_eq!(handle.join().unwrap(), 42);
    }
    
    #[test]
    fn test_channel() {
        let (tx, rx) = mpsc::channel();
        tx.send(10).unwrap();
        assert_eq!(rx.recv().unwrap(), 10);
    }
    
    #[test]
    fn test_mutex() {
        let m = Mutex::new(5);
        {
            let mut num = m.lock().unwrap();
            *num = 10;
        }
        assert_eq!(*m.lock().unwrap(), 10);
    }
    
    #[test]
    fn test_arc_mutex() {
        let counter = Arc::new(Mutex::new(0));
        let mut handles = vec![];
        
        for _ in 0..10 {
            let counter = Arc::clone(&counter);
            let handle = thread::spawn(move || {
                let mut num = counter.lock().unwrap();
                *num += 1;
            });
            handles.push(handle);
        }
        
        for handle in handles {
            handle.join().unwrap();
        }
        
        assert_eq!(*counter.lock().unwrap(), 10);
    }
}
