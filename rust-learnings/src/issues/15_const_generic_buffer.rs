// ============================================================
// ISSUE 15 — Const Generic Buffer
// Concept: Const Generics
// Difficulty: Intermediate
//
// Your Task:
// Const generics allow you to use values (like integers) as generic
// parameters. This is useful for types like fixed-size arrays.
//
// TODO: Implement the `Buffer` struct using a const generic parameter `N`.
//
// Resources: https://doc.rust-lang.org/book/ch10-01-syntax.html#const-generics
// ============================================================

// TODO: Fix the struct definition
pub struct Buffer {
    pub data: [u8; 1024], // Should be size N
}

impl Buffer {
    // TODO: Implement a constructor that takes no arguments
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_const_generic_buffer() {
        // let b: Buffer<64> = Buffer::new();
        // assert_eq!(b.data.len(), 64);
    }
}
