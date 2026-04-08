// ============================================================
// BASICS: CLI Calculator Mini-Project
//
// Welcome to Rust! This file is a safe playground where you can learn
// the basics by building a text-based calculator.
//
// Fix the functions below step-by-step.
// Run `cargo test test_calculator` locally to test your code.
// ============================================================

pub fn get_welcome_message() -> String {
    "Welcome to the Calculator!".to_string()
}

// STEP 2: Enums
// TODO: We need to represent math operators. Define four variants:
// Add, Subtract, Multiply, and Divide.
#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Operator {
    Add,
    Subtract,
    Multiply,
    Divide,
}

// STEP 3: Control Flow (Match statements)
// TODO: Take a character like '+' or '-' and return the matching Operator.
// Use a `match` statement. If the character is unknown, return an Error String.
pub fn parse_operator(op_char: char) -> Result<Operator, String> {
    match op_char {
        '+' => Ok(Operator::Add),
        '-' => Ok(Operator::Subtract),
        '*' => Ok(Operator::Multiply),
        '/' => Ok(Operator::Divide),
        _ => Err("Unknown operator".to_string()),
    }
pub fn parse_operator(_op_char: char) -> Result<Operator, String> {
    todo!("Match the char to your enum, or return Err(...)")
}

// STEP 4: Math & Error Handling
// TODO: Perform the calculation. Return an Ok(i32) with the result,
// OR an Err(String) if they try to divide by zero!
pub fn calculate(a: i32, op: Operator, b: i32) -> Result<i32, String> {
    match op {
        Operator::Add => Ok(a + b),
        Operator::Subtract => Ok(a - b),
        Operator::Multiply => Ok(a * b),
        Operator::Divide => {
            if b == 0 {
                Err("Divide by zero!".to_string())
            } else {
                Ok(a / b)
            }
        }
    }
pub fn calculate(_a: i32, _op: Operator, _b: i32) -> Result<i32, String> {
    todo!("Return Ok(result) or Err(\"Divide by zero!\")")
}

// STEP 5: String Splitting & Parsing
// TODO: Take a string like "5 + 3" and parse it into its components.
// Hint: `input.split_whitespace().collect::<Vec<&str>>()` will be very useful.
// Use `parse::<i32>()` to convert strings to numbers.
pub fn parse_equation(input: &str) -> Result<(i32, Operator, i32), String> {
    let parts: Vec<&str> = input.split_whitespace().collect();
    if parts.len() != 3 {
        return Err("Invalid equation format".to_string());
    }
    let a: i32 = parts[0].parse().map_err(|_| "Invalid number".to_string())?;
    let op_char = parts[1].chars().next().ok_or("Invalid operator".to_string())?;
    let op = parse_operator(op_char)?;
    let b: i32 = parts[2].parse().map_err(|_| "Invalid number".to_string())?;
    Ok((a, op, b))
pub fn parse_equation(_input: &str) -> Result<(i32, Operator, i32), String> {
    todo!("Split the string, parse the numbers and operator, and return them as a tuple")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calculator_step1_welcome() {
        let msg = get_welcome_message();
        assert!(msg.len() > 0, "Welcome message shouldn't be empty");
    }

    #[test]
    fn test_calculator_step3_parse_op() {
        assert_eq!(parse_operator('+'), Ok(Operator::Add));
        assert_eq!(parse_operator('-'), Ok(Operator::Subtract));
        assert!(parse_operator('x').is_err());
    }

    #[test]
    fn test_calculator_step4_math() {
        // assert_eq!(calculate(10, Operator::Add, 5), Ok(15));
        // assert_eq!(calculate(10, Operator::Divide, 2), Ok(5));
        // assert!(calculate(10, Operator::Divide, 0).is_err());
    }

    #[test]
    fn test_calculator_step5_equation() {
        // let (a, op, b) = parse_equation("10 * 5").unwrap();
        // assert_eq!(a, 10);
        // assert_eq!(op, Operator::Multiply);
        // assert_eq!(b, 5);
    }
}
