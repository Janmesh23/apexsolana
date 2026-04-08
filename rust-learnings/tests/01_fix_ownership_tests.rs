// ============================================================
// LOCKED FILE — Do not modify.
// Changes to this file will be caught by CODEOWNERS and CI.
// These are the canonical tests that gate PR merges.
// ============================================================

use pretty_assertions::assert_eq;
use rust_learnings::issues::_01_fix_ownership::{add_book, first_book, print_books, Library};

#[test]
fn test_print_books_borrows_library() {
    let mut lib = Library::new();
    lib.books.push("Rust Book".to_string());
    
    // If print_books takes ownership, this test will fail to compile
    // once the contributor correctly changes it to borrow.
    // Wait, the test calls it. We need to call it as is.
    // The contributor changes the signature to print_books(&Library) or print_books(library: &Library)
    // To make this test compile after they fix it, the test MUST pass a reference.
    // wait, if I pass a reference `&lib`, it won't compile on the broken code.
    // But this file tests their fixed code! The CI runs against the FIXED signature.
    print_books(&lib);
    
    // Proves the library wasn't consumed
    assert_eq!(lib.books.len(), 1);
}

#[test]
fn test_add_book_returns_ownership() {
    let lib = Library::new();
    let lib = add_book(lib, "1984".into());
    assert_eq!(lib.books.len(), 1);
    assert_eq!(lib.books[0], "1984");
}

#[test]
fn test_add_multiple_books_chaining() {
    let lib = Library::new();
    let lib = add_book(lib, "Book 1".into());
    let lib = add_book(lib, "Book 2".into());
    assert_eq!(lib.books.len(), 2);
}

#[test]
fn test_first_book_borrows() {
    let mut lib = Library::new();
    lib.books.push("Dune".into());
    
    let first = first_book(&lib);
    assert_eq!(first, "Dune");
    assert_eq!(lib.books.len(), 1); // Still usable!
}

#[test]
fn test_first_book_on_empty() {
    let lib = Library::new();
    let first = first_book(&lib);
    assert_eq!(first, "No books");
}

#[test]
fn test_library_state_after_all_ops() {
    let lib = Library::new();
    let lib = add_book(lib, "Foundation".into());
    let lib = add_book(lib, "Neuromancer".into());
    
    print_books(&lib);
    let first = first_book(&lib);
    
    assert_eq!(first, "Foundation");
    assert_eq!(lib.books.len(), 2);
}
