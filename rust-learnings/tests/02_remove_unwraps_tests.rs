// ============================================================
// LOCKED FILE — Do not modify.
// Changes to this file will be caught by CODEOWNERS and CI.
// These are the canonical tests that gate PR merges.
// ============================================================

use pretty_assertions::assert_eq;
use rust_learnings::issues::_02_remove_unwraps::{load_config_file, parse_user_config, ConfigError, User};
use std::fs;

#[test]
fn test_valid_config_string() {
    let result = parse_user_config("alice:25:alice@email.com");
    assert_eq!(
        result,
        Ok(User {
            name: "alice".to_string(),
            age: 25,
            email: "alice@email.com".to_string(),
        })
    );
}

#[test]
fn test_missing_field_returns_error() {
    let result = parse_user_config("bob:30");
    assert_eq!(result, Err(ConfigError::MissingField));
}

#[test]
fn test_non_numeric_age_returns_invalid_age() {
    let result = parse_user_config("charlie:thirty:charlie@email.com");
    assert_eq!(result, Err(ConfigError::InvalidAge("thirty".to_string())));
}

#[test]
fn test_completely_malformed_string() {
    let result = parse_user_config("");
    assert_eq!(result, Err(ConfigError::MissingField));
}

#[test]
fn test_load_config_file_missing() {
    let result = load_config_file("nonexistent_file.txt");
    assert_eq!(result, Err(ConfigError::InvalidFormat)); // Or a similar error expected by the contributor
}

#[test]
fn test_no_unwrap_or_expect_in_source() {
    let source = fs::read_to_string("src/issues/02_remove_unwraps.rs")
        .expect("Failed to read source file for meta-test");
    
    assert!(
        !source.contains(".unwrap()"),
        "Your code still contains `.unwrap()`. Please handle the error properly!"
    );
    assert!(
        !source.contains(".expect("),
        "Your code contains `.expect()`. Please return a Result and use the `?` operator!"
    );
}
