// ============================================================
// ISSUE 8 — The Long-Lived Struct
// Concept: Lifetimes in Structs
// Difficulty: Advanced
//
// Your Task:
// When a struct contains a reference, it MUST explicitly specify
// a lifetime parameter so the compiler can ensure the reference
// lives at least as long as the struct.
//
// TODO: Fix the `Message` struct and its implementation to use lifetime annotations.
// TODO: Complete the `get_content` function.
//
// Resources: https://doc.rust-lang.org/book/ch10-03-lifetime-syntax.html
// ============================================================

// TODO: Fix this struct definition
pub struct Message {
    pub content: &str, // ERROR: missing lifetime specifier
}

impl Message {
    // TODO: Fix the constructor signature
    pub fn new(content: &str) -> Message {
        Message { content }
    }
}

pub fn get_preview(m: Message) -> String {
    format!("Preview: {}", m.content)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_message_lifetime() {
        let text = String::from("Secret Data");
        let msg = Message::new(&text);
        assert_eq!(get_preview(msg), "Preview: Secret Data");
    }
}
