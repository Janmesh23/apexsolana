// ============================================================
// ISSUE 16 — Threaded Counter
// Concept: Shared Mutability with `Arc` and `Mutex`
// Difficulty: Intermediate
//
// Your Task:
// To safely share and modify data across multiple threads, we use `Arc`
// (Atomic Reference Counting) and `Mutex` (Mutual Exclusion).
//
// TODO: Use `Arc` and `Mutex` to allow 10 threads to increment the counter.
//
// Resources: https://doc.rust-lang.org/book/ch16-03-shared-state.html
// ============================================================

use std::sync::{Arc, Mutex};
use std::thread;

pub fn parallel_increment() -> i32 {
    let counter = Mutex::new(0); // TODO: Wrap in Arc

    let mut handles = vec![];

    for _ in 0..10 {
        // TODO: Clone the Arc for each thread
        let handle = thread::spawn(move || {
            // let mut num = counter.lock().unwrap();
            // *num += 1;
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    // TODO: Return the final value
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parallel_increment() {
        // let result = parallel_increment();
        // assert_eq!(result, 10);
    }
}
