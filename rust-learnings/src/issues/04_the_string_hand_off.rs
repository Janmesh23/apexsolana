// ============================================================
// ISSUE 4 — The String Hand-off
// Concept: Ownership and Move Semantics
// Difficulty: Beginner
//
// Your Task:
// The code below fails because the `process_name` function takes ownership
// of a String. When we try to use it again in the println!, the compiler
// complains that the value has been moved.
// Fix this code using the most appropriate method (hint: consider borrowing).
//
// Resources: https://doc.rust-lang.org/book/ch04-01-what-is-ownership.html
// ============================================================

pub fn process_name(name: String) -> usize {
    name.len()
}

pub fn main_logic() -> String {
    let name = String::from("Rustacean");

    // TODO: Fix the call below so that 'name' is still usable afterwards.
    let _len = process_name(name);

    // This should work after your fix
    format!("Hello, {}!", name)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ownership_fix() {
        let result = main_logic();
        assert_eq!(result, "Hello, Rustacean!");
    }
}
