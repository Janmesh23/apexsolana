// ============================================================
// ISSUE 6 — The Double Agent (E0502)
// Concept: Shared vs Mutable Borrowing
// Difficulty: Intermediate
//
// Your Task:
// Rust's aliasing rules prevent multiple mutable borrows OR a mutable
// borrow simultaneous with shared borrows.
//
// The code below tries to push an element to a Vec while holding a reference
// to one of its elements. Refactor the code to satisfy the borrow checker.
//
// Resources: https://doc.rust-lang.org/book/ch04-02-references-and-borrowing.html
// ============================================================

pub fn extend_list() -> Vec<i32> {
    let mut numbers = vec![1, 2, 3];

    // TODO: Fix the violation below.
    // Hint: Shared borrows cannot exist when a mutable borrow (push) happens.
    let first = &numbers[0];

    numbers.push(4);

    // We want to return the version of the list that includes 'first' as a prefix
    // (This is a contrived example to show the borrow error)
    println!("First element was: {}", first);

    numbers
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extend_list() {
        let v = extend_list();
        assert_eq!(v, vec![1, 2, 3, 4]);
    }
}
