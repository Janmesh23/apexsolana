// ============================================================
// ISSUE 17 — The Async Deadlock
// Concept: Mutexes in Async Code
// Difficulty: Advanced
//
// Your Task:
// Holding a standard `std::sync::MutexGuard` across an `.await` point
// can lead to deadlocks or "Send" violations because the executor
// may switch threads while the lock is held.
//
// Refactor the code below to use the Tokio Mutex or limit the lock scope.
//
// Resources: https://tokio.rs/tokio/tutorial/shared-state#holding-a-mutexguard-across-an-await
// ============================================================

use std::sync::Mutex;
use tokio::time::{sleep, Duration};

async fn async_work() {
    sleep(Duration::from_millis(10)).await;
}

pub async fn increment_shared(m: &Mutex<u32>) {
    // TODO: This guard is held across the .await below. Fix it!
    let mut guard = m.lock().unwrap();
    async_work().await;
    *guard += 1;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_async_deadlock_prevention() {
        let m = Mutex::new(0);
        increment_shared(&m).await;
        assert_eq!(*m.lock().unwrap(), 1);
    }
}
