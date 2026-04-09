// ============================================================
// ISSUE 18 — Unsafe Raw Pointers
// Concept: Raw Pointer Dereferencing
// Difficulty: Advanced
//
// Your Task:
// Raw pointers (`*const T` and `*mut T`) do not have automatic memory
// safety guarantees. Accessing them requires an `unsafe` block.
//
// TODO: Read the value of `x` using the raw pointer `p`.
//
// Resources: https://doc.rust-lang.org/book/ch19-01-unsafe-rust.html#dereferencing-a-raw-pointer
// ============================================================

pub fn read_from_raw_pointer() -> i32 {
    let x = 42;
    let p = &x as *const i32;

    // TODO: Dereference 'p' inside an unsafe block to get the value.
    // Hint: Use *p
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_raw_pointer_read() {
        assert_eq!(read_from_raw_pointer(), 42);
    }
}
