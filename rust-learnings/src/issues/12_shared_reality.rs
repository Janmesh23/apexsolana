// ============================================================
// ISSUE 12 — Shared Reality
// Concept: Reference Counting with `Rc<T>`
// Difficulty: Intermediate
//
// Your Task:
// Standard Rust ownership rules only allow one owner. For cases where
// multiple parts of a single-threaded program need to share ownership
// of some data, we use `Rc<T>`.
//
// TODO: Refactor the code below to use `Rc` so that both `car1` and `car2`
// can share the same `Engine`.
//
// Resources: https://doc.rust-lang.org/book/ch15-04-rc.html
// ============================================================

use std::rc::Rc;

#[derive(Debug)]
pub struct Engine {
    pub power: u32,
}

pub struct Car {
    pub model: String,
    pub engine: Engine, // TODO: Change to Rc<Engine>
}

pub fn create_cars() -> (Car, Car) {
    let common_engine = Engine { power: 150 };

    // This will fail because common_engine is moved into car1
    let car1 = Car {
        model: "Sedan".to_string(),
        engine: common_engine,
    };

    let car2 = Car {
        model: "Hatchback".to_string(),
        engine: common_engine, // Error: use after move
    };

    (car1, car2)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_shared_engine() {
        // let (c1, c2) = create_cars();
        // assert_eq!(c1.engine.power, 150);
        // assert_eq!(c2.engine.power, 150);
    }
}
