// ============================================================
// ISSUE 20 — Blocking the Executor
// Concept: Async Concurrency Pitfalls
// Difficulty: Advanced
//
// Your Task:
// Async functions should NEVER use synchronous, blocking calls like
// `std::thread::sleep` or long CPU-bound loops. These block the entire
// executor thread, preventing other tasks from running.
//
// TODO: Replace the blocking sleep with the tokio equivalent.
//
// Resources: https://ryhl.us/blog/async-what-is-blocking/
// ============================================================

use std::time::Duration;
// use tokio::time::sleep;

pub async fn background_task() {
    println!("Starting task...");

    // TODO: This blocks the executor. Change it to an async sleep.
    std::thread::sleep(Duration::from_millis(100));

    println!("Task finished!");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_blocking_executor_fix() {
        background_task().await;
    }
}
