// ============================================================
// ISSUE 9 — Map vs ForEach
// Concept: Lazy Evaluation of Iterators
// Difficulty: Beginner
//
// Your Task:
// Iterators in Rust are "lazy." Methods like `map` return a new iterator
// but don't actually process any elements until the iterator is consumed.
//
// The code below tries to use `map` to print elements, but nothing happens.
// Fix the code so the side effects (printing) actually occur.
//
// Resources: https://doc.rust-lang.org/book/ch13-02-iterators.html
// ============================================================

pub fn print_numbers(nums: Vec<i32>) {
    // TODO: Fix this line so it actually prints each number.
    // Hint: map is for transformation, not for side effects.
    nums.iter().map(|n| println!("Number: {}", n));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_print_numbers() {
        // Since we can't easily capture stdout here, we'll just ensure it compiles
        // and doesn't panic. The real test is the visual side effect.
        print_numbers(vec![1, 2, 3]);
    }
}
