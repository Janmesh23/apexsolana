
pub fn get_welcome_message() -> String {
    "Welcome to the Calculator!".to_string()
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Operator {
    Add,
    Subtract,
    Multiply,
    Divide,
}

pub fn parse_operator(op_char: char) -> Result<Operator, String> {
    match op_char {
        '+' => Ok(Operator::Add),
        '-' => Ok(Operator::Subtract),
        '*' => Ok(Operator::Multiply),
        '/' => Ok(Operator::Divide),
        _ => Err("Unknown operator".to_string()),
    }
}

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
}

pub fn parse_equation(input: &str) -> Result<(i32, Operator, i32), String> {
    let parts: Vec<&str> = input.split_whitespace().collect();
    if parts.len() != 3 {
        return Err("Invalid equation format".to_string());
    }

    let a = parts[0]
        .parse::<i32>()
        .map_err(|_| "Invalid number".to_string())?;
    let op_char = parts[1]
        .chars()
        .next()
        .ok_or_else(|| "Invalid operator".to_string())?;
    let operator = parse_operator(op_char)?;
    let b = parts[2]
        .parse::<i32>()
        .map_err(|_| "Invalid number".to_string())?;

    Ok((a, operator, b))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calculator_step1_welcome() {
        let msg = get_welcome_message();
        assert!(!msg.is_empty(), "Welcome message shouldn't be empty");
    }

    #[test]
    fn test_calculator_step3_parse_op() {
        assert_eq!(parse_operator('+'), Ok(Operator::Add));
        assert_eq!(parse_operator('-'), Ok(Operator::Subtract));
        assert!(parse_operator('x').is_err());
    }

    #[test]
    fn test_calculator_step4_math() {
        assert_eq!(calculate(10, Operator::Add, 5), Ok(15));
        assert_eq!(calculate(10, Operator::Divide, 2), Ok(5));
        assert!(calculate(10, Operator::Divide, 0).is_err());
    }

    #[test]
    fn test_calculator_step5_equation() {
        let (a, op, b) = parse_equation("10 * 5").unwrap();
        assert_eq!(a, 10);
        assert_eq!(op, Operator::Multiply);
        assert_eq!(b, 5);
    }
}
