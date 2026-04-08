// ============================================================
// ISSUE 3 — Fix Iterator Adaptation
// Concept: Iterator adaptors, chaining, and the collect() method
// Difficulty: Beginner-Intermediate
//
// Your Task:
// The code below works, but uses imperative loop styles (`for` loops and mutations).
// Idiomatic Rust heavily relies on iterators (`.iter()`, `.map()`, `.filter()`, etc.).
// Rewrite the functions below to use iterator chains instead of manual loops.
//
// Hint: Iterators are lazy, you must call a consuming method like `collect()`
// or `sum()` to execute them.
//
// Resources: https://doc.rust-lang.org/book/ch13-02-iterators.html
// ============================================================

use std::collections::HashMap;

#[derive(Debug, PartialEq, Clone)]
pub struct Student {
    pub name: String,
    pub grade: u32,
    pub passed: bool,
}

// TODO: Rewrite using .iter().filter().map().collect()
// Must return names of only students who passed, sorted alphabetically
pub fn passing_students(students: &[Student]) -> Vec<String> {
    let mut passed_names = Vec::new();
    for student in students {
        if student.passed {
            passed_names.push(student.name.clone());
        }
    }
    passed_names.sort();
    passed_names
}

// TODO: Rewrite using .iter().map().sum() or .fold()
// Must return 0.0 for an empty slice, not panic
pub fn average_grade(students: &[Student]) -> f64 {
    if students.is_empty() {
        return 0.0;
    }

    let mut total = 0;
    for student in students {
        total += student.grade;
    }

    total as f64 / students.len() as f64
}

// TODO: Implement using .iter().max_by_key()
// Returns None for empty slice, Some(&Student) for the highest grade
pub fn top_student(_students: &[Student]) -> Option<&Student> {
    todo!("Find the top student using iterators")
}

// TODO: Use .iter().fold() or a clever .collect() into a HashMap
// Returns a map of grade → count of students with that grade
pub fn grade_distribution(_students: &[Student]) -> HashMap<u32, usize> {
    todo!("Calculate grade distribution")
}

#[cfg(test)]
mod tests {
    use super::*;

    // NOTE: Passing these tests is not enough.
    //       The real tests live in tests/ and run on CI.
    #[test]
    fn test_passing_students_local_hint() {
        let students = vec![
            Student {
                name: "Alice".into(),
                grade: 90,
                passed: true,
            },
            Student {
                name: "Bob".into(),
                grade: 40,
                passed: false,
            },
        ];
        let passed = passing_students(&students);
        assert_eq!(passed, vec!["Alice"]);
    }

    #[test]
    fn test_average_grade_empty_local_hint() {
        let students: Vec<Student> = vec![];
        let avg = average_grade(&students);
        assert_eq!(avg, 0.0);
    }
}
