// ============================================================
// ISSUE 1 — Fix Ownership and Move Semantics
// Concept: Ownership and move semantics
// Difficulty: Beginner
//
// Your Task:
// The code below does not compile because it violates Rust's ownership rules.
// Fix the function signatures and bodies so that ownership is either passed back
// or elements are borrowed correctly. See the TODO comments below.
//
// Hint: If a function only needs to read data, it should take a reference (&T).
// If it needs to modify and return ownership, make sure to return the value!
//
// Resources: https://doc.rust-lang.org/book/ch04-00-understanding-ownership.html
// ============================================================

#[derive(Debug, Clone, PartialEq)]
pub struct Library {
    pub books: Vec<String>,
}

impl Library {
    pub fn new() -> Self {
        Library { books: Vec::new() }
    }
}

// TODO: Fix the function signature and body so it returns the library
// after adding the book.
pub fn add_book(mut library: Library, title: String) -> Library {
    library.books.push(title);
    library
}

// TODO: Change this to borrow library instead of taking ownership.
pub fn print_books(library: &Library) {
    for book in &library.books {
        println!("Book: {}", book);
    }
}

// TODO: Change to borrowing and return a &str or clone only what's needed.
pub fn first_book(library: &Library) -> String {
    if library.books.is_empty() {
        "No books".to_string()
    } else {
        library.books[0].clone()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // NOTE: Passing these tests is not enough.
    //       The real tests live in tests/ and run on CI.
    #[test]
    fn test_add_book_local_hint() {
        let lib = Library::new();
        // let lib = add_book(lib, "1984".to_string());
        // assert_eq!(lib.books.len(), 1);
    }

    #[test]
    fn test_print_books_local_hint() {
        let mut lib = Library::new();
        lib.books.push("Dune".to_string());
        // print_books(&lib); // Should be able to pass by reference
        // assert_eq!(lib.books.len(), 1); // library still usable
    }
}
