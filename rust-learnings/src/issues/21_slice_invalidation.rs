// ============================================================
// ISSUE 21 — Slice Invalidation
// Concept: Borrow Checker preventing Iterator Invalidation
// Difficulty: Intermediate
//
// Your Task:
// In many languages, modifying a collection while iterating over it
// leads to undefined behavior or runtime errors. Rust prevents this
// at compile time.
//
// TODO: Find a way to add a '0' to the vector for every element
// greater than 2, without violating the borrow checker.
//
// Resources: https://doc.rust-lang.org/book/ch04-02-references-and-borrowing.html
// ============================================================

pub fn process_list() -> Vec<i32> {
    let mut v = vec![1, 2, 3];

    // TODO: This loop fails because you can't push to 'v' while
    // iterating over its references.
    for x in &v {
        if *x > 2 {
            v.push(0);
        }
    }

    v
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_slice_invalidation_fix() {
        let result = process_list();
        assert_eq!(result, vec![1, 2, 3, 0]);
    }
}
