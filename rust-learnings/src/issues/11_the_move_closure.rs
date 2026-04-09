// ============================================================
// ISSUE 11 — The Move Closure
// Concept: Capturing Variables by Value
// Difficulty: Intermediate
//
// Your Task:
// By default, closures capture variables by reference. However, when moving
// a closure to another thread, it must take ownership of any variables
// it uses to ensure they live long enough.
//
// TODO: Fix the closure below so it can be moved into the thread.
//
// Resources: https://doc.rust-lang.org/book/ch13-01-closures.html#capturing-references-or-moving-ownership
// ============================================================

use std::thread;

pub fn spawn_worker() {
    let message = String::from("Hello from the main thread");

    // TODO: Fix the closure or the line below so 'message' is moved into the thread.
    let handle = thread::spawn(|| {
        println!("{}", message);
    });

    handle.join().unwrap();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_spawn_worker() {
        spawn_worker();
    }
}
