// ============================================================
// ISSUE 5 — Enum Exhaustiveness
// Concept: The Type System and Pattern Matching
// Difficulty: Beginner
//
// Your Task:
// Rust's match expressions must be exhaustive. The code below fails to compile
// because the `Status` enum has a new variant `Maintenance` that isn't handled.
//
// Update the `handle_status` function to handle the new variant.
//
// Resources: https://doc.rust-lang.org/book/ch06-02-match.html
// ============================================================

#[derive(Debug, PartialEq)]
pub enum Status {
    Active,
    Inactive,
    Maintenance, // Added this recently
}

pub fn handle_status(status: Status) -> &'static str {
    // TODO: Fix the match expression to handle Status::Maintenance
    match status {
        Status::Active => "System is live",
        Status::Inactive => "System is down",
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_maintenance_status() {
        assert_eq!(handle_status(Status::Maintenance), "System is under maintenance");
    }
}
