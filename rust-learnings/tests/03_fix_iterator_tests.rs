// ============================================================
// LOCKED FILE — Do not modify.
// Changes to this file will be caught by CODEOWNERS and CI.
// These are the canonical tests that gate PR merges.
// ============================================================

use std::fs;
use pretty_assertions::assert_eq;
use rust_learnings::issues::_03_fix_iterator::{average_grade, grade_distribution, passing_students, top_student, Student};

#[test]
fn test_passing_students_mixed() {
    let students = vec![
        Student { name: "Charlie".into(), grade: 80, passed: true },
        Student { name: "Alice".into(), grade: 90, passed: true },
        Student { name: "Bob".into(), grade: 40, passed: false },
    ];
    let passed = passing_students(&students);
    assert_eq!(passed, vec!["Alice", "Charlie"]);
}

#[test]
fn test_passing_students_all_fail() {
    let students = vec![
        Student { name: "Bob".into(), grade: 40, passed: false },
        Student { name: "Dave".into(), grade: 20, passed: false },
    ];
    let passed = passing_students(&students);
    assert!(passed.is_empty());
}

#[test]
fn test_average_grade_known_values() {
    let students = vec![
        Student { name: "A".into(), grade: 90, passed: true },
        Student { name: "B".into(), grade: 80, passed: true },
        Student { name: "C".into(), grade: 40, passed: false },
    ];
    let avg = average_grade(&students);
    assert_eq!(avg, 70.0);
}

#[test]
fn test_average_grade_empty() {
    let students: Vec<Student> = vec![];
    let avg = average_grade(&students);
    assert_eq!(avg, 0.0);
}

#[test]
fn test_top_student_known_dataset() {
    let students = vec![
        Student { name: "A".into(), grade: 60, passed: true },
        Student { name: "Top".into(), grade: 99, passed: true },
        Student { name: "B".into(), grade: 40, passed: false },
    ];
    let top = top_student(&students);
    assert_eq!(top.unwrap().name, "Top");
}

#[test]
fn test_top_student_empty() {
    let students: Vec<Student> = vec![];
    let top = top_student(&students);
    assert_eq!(top, None);
}

#[test]
fn test_grade_distribution() {
    let students = vec![
        Student { name: "A".into(), grade: 90, passed: true },
        Student { name: "B".into(), grade: 90, passed: true },
        Student { name: "C".into(), grade: 40, passed: false },
    ];
    let dist = grade_distribution(&students);
    assert_eq!(dist.get(&90), Some(&2));
    assert_eq!(dist.get(&40), Some(&1));
    assert_eq!(dist.get(&100), None);
}

#[test]
fn test_no_manual_for_loops_in_source() {
    let source = fs::read_to_string("src/issues/03_fix_iterator.rs")
        .expect("Failed to read source file for meta-test");
    
    // We only care about function bodies, but checking for "for " is a decent heuristic
    assert!(
        !source.contains("for "),
        "Your code still contains manual `for` loops. Please use iterator adaptors!"
    );
}
