// ============================================================
// ISSUE 13 — Object Safety
// Concept: Dynamic Dispatch with `dyn Trait`
// Difficulty: Advanced
//
// Your Task:
// Standard generics (`impl Trait` or `<T: Trait>`) use static dispatch,
// which means the compiler must know the concrete type at compile time.
//
// To store different types in a single collection (heterogeneous collection),
// you must use dynamic dispatch (`Box<dyn Trait>`). However, only "Object Safe"
// traits can be used this way.
//
// TODO: Fix the `list_shapes` function so it can handle both `Circle` and `Square`.
//
// Resources: https://doc.rust-lang.org/book/ch17-02-trait-objects.html
// ============================================================

pub trait Draw {
    fn draw(&self) -> String;
}

pub struct Circle;
impl Draw for Circle {
    fn draw(&self) -> String { "Circle".to_string() }
}

pub struct Square;
impl Draw for Square {
    fn draw(&self) -> String { "Square".to_string() }
}

// TODO: Refactor the signature and body to handle multiple types of Draw
pub fn list_shapes(shapes: Vec<impl Draw>) -> Vec<String> {
    shapes.iter().map(|s| s.draw()).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_heterogeneous_draw() {
        // let shapes: Vec<Box<dyn Draw>> = vec![
        //     Box::new(Circle),
        //     Box::new(Square),
        // ];
        // let results = list_shapes(shapes);
        // assert_eq!(results, vec!["Circle", "Square"]);
    }
}
