// ============================================================
// ISSUE 14 — The Transient Reference
// Concept: Reference Lifetime and Scope
// Difficulty: Beginner
//
// Your Task:
// A reference must not outlive the value it points to.
//
// In the code below, a reference is being assigned to `r` that points
// to a variable `x` which is dropped before `r` is used.
//
// Resources: https://doc.rust-lang.org/book/ch04-02-references-and-borrowing.html
// ============================================================

pub fn use_after_drop() -> String {
    let r: &String;

    {
        let x = String::from("I am temporary");
        // TODO: This assignment causes an error because x is dropped at the end of this block.
        r = &x;
    }

    // ERROR: r points to dropped memory
    format!("The message was: {}", r)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_transient_ref() {
        // let val = use_after_drop();
        // assert_eq!(val, "The message was: I am temporary");
    }
}
