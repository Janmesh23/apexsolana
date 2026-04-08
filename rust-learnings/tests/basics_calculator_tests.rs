// ============================================================
// LOCKED FILE — Do not modify.
// These tests verify that the calculator was completed correctly.
// ============================================================

use rust_learnings::basics::calculator::{calculate, get_welcome_message, parse_equation, parse_operator, Operator};

#[test]
fn test_welcome_message_returns_owned_string() {
    let msg: String = get_welcome_message();
    assert!(!msg.is_empty(), "Greeting message must not be empty");
}

#[test]
fn test_parse_operator_all_variants() {
    assert_eq!(parse_operator('+'), Ok(Operator::Add));
    assert_eq!(parse_operator('-'), Ok(Operator::Subtract));
    assert_eq!(parse_operator('*'), Ok(Operator::Multiply));
    assert_eq!(parse_operator('/'), Ok(Operator::Divide));
}

#[test]
fn test_parse_operator_invalid() {
    let result = parse_operator('^');
    assert!(result.is_err(), "Invalid operators should return Err");
}

#[test]
fn test_calculate_math_logic() {
    assert_eq!(calculate(5, Operator::Add, 10), Ok(15));
    assert_eq!(calculate(10, Operator::Subtract, 5), Ok(5));
    assert_eq!(calculate(4, Operator::Multiply, 3), Ok(12));
    assert_eq!(calculate(15, Operator::Divide, 3), Ok(5));
}

#[test]
fn test_calculate_divide_by_zero() {
    let result = calculate(10, Operator::Divide, 0);
    assert!(result.is_err(), "Dividing by zero must return an error");
}

#[test]
fn test_parse_equation_valid() {
    let result = parse_equation("42 + 8");
    assert_eq!(result, Ok((42, Operator::Add, 8)));
}

#[test]
fn test_parse_equation_invalid_format() {
    assert!(parse_equation("42+8").is_err(), "Must reject input missing spaces");
    assert!(parse_equation("hello + world").is_err(), "Must reject non-numeric input");
}
