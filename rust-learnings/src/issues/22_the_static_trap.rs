// ============================================================
// ISSUE 22 — The Static Trap
// Concept: `'static` Lifetime Bounds in Threads
// Difficulty: Advanced
//
// Your Task:
// `std::thread::spawn` requires that the closure it takes has a
// `'static` lifetime. This means the closure cannot capture any
// references to variables that might not live as long as the whole program.
//
// TODO: Fix the code below so it can safely use `local_val` in a new thread.
//
// Resources: https://doc.rust-lang.org/book/ch16-01-threads.html
// ============================================================

use std::thread;

pub fn thread_with_local() {
    let local_val = vec![1, 2, 3];

    // TODO: This fails because the thread might outlive local_val.
    // Fix it using the 'move' keyword or another safe approach.
    let handle = thread::spawn(|| {
        println!("Values: {:?}", local_val);
    });

    handle.join().unwrap();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_static_trap_fix() {
        thread_with_local();
    }
}
