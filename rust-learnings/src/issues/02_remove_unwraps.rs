// ============================================================
// ISSUE 2 — Remove Unwraps
// Concept: Error handling with Result, Option, and the ? operator
// Difficulty: Beginner
//
// Your Task:
// The code below parses a user config string and reads a file, but it uses
// `.unwrap()` everywhere, which can cause the program to panic on bad input.
// Refactor the code to use proper error handling by returning `Result` types 
// and using the `?` operator to propagate errors.
//
// Hint: A function returning Result<T, E> can use `?` on operations that also return Result.
//
// Resources: https://doc.rust-lang.org/book/ch09-02-recoverable-errors-with-result.html
// ============================================================

use std::fs;
use thiserror::Error;

#[derive(Debug, PartialEq)]
pub struct User {
    pub name: String,
    pub age: u8,
    pub email: String,
}

#[derive(Error, Debug, PartialEq)]
pub enum ConfigError {
    #[error("Missing field in config")]
    MissingField,
    #[error("Invalid age: {0}")]
    InvalidAge(String),
    #[error("Invalid config format")]
    InvalidFormat,
}

// TODO: Change parse_user_config to return Result<User, ConfigError>
// TODO: Replace every .unwrap() with proper ? propagation
// TODO: Use the ConfigError variants you defined
pub fn parse_user_config(input: &str) -> User {
    let parts: Vec<&str> = input.split(':').collect();
    
    // Panics if the format doesn't have 3 parts
    let name = parts.get(0).unwrap().to_string();
    let age_str = parts.get(1).unwrap();
    let age = age_str.parse::<u8>().unwrap();
    let email = parts.get(2).unwrap().to_string();

    User { name, age, email }
}

// TODO: Change load_config_file to return Result<String, ConfigError>
pub fn load_config_file(path: &str) -> String {
    // Panics if the file doesn't exist
    fs::read_to_string(path).unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    // NOTE: Passing these tests is not enough.
    //       The real tests live in tests/ and run on CI.
    #[test]
    fn test_valid_parse_local_hint() {
        // let result = parse_user_config("alice:25:alice@email.com");
        // assert!(result.is_ok());
    }

    #[test]
    fn test_invalid_age_local_hint() {
        // let result = parse_user_config("bob:twenty:bob@email.com");
        // assert_eq!(result, Err(ConfigError::InvalidAge("twenty".to_string())));
    }
}
