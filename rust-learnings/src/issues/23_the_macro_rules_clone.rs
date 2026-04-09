// ============================================================
// ISSUE 23 — The macro_rules! Clone
// Concept: Declarative Macros and Repetition
// Difficulty: Intermediate
//
// Your Task:
// Declarative macros (`macro_rules!`) allow you to write code that
// generates other code. One common pattern is repeating an operation
// for a list of arguments.
//
// TODO: Implement a macro named `my_vec!` that creates a Vec of elements.
// The macro should handle any number of comma-separated expressions.
//
// Resources: https://doc.rust-lang.org/book/ch19-06-macros.html#declarative-macros-with-macro_rules-for-general-metaprogramming
// ============================================================

// TODO: Define your macro here
#[macro_export]
macro_rules! my_vec {
    // Hint: Use $(...),* syntax
    () => { Vec::new() };
}

#[cfg(test)]
mod tests {
    // use super::*;

    #[test]
    fn test_my_vec_macro() {
        // let v = my_vec![1, 2, 3];
        // assert_eq!(v, vec![1, 2, 3]);

        // let v2 = my_vec!["a", "b"];
        // assert_eq!(v2, vec!["a", "b"]);
    }
}
