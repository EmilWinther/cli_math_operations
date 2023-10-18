extern crate clap;

use clap::{App, Arg};
use std::num::ParseFloatError;

enum Operation {
    Add,
    Subtract,
    Multiply,
    Divide,
}

#[derive(Debug)]
enum CalcError {
    OperandParseError(String),
    DivisionByZero,
}

impl From<ParseFloatError> for CalcError {
    fn from(_: ParseFloatError) -> Self {
        CalcError::OperandParseError("Failed to parse operand".to_string())
    }
}

fn parse_operand(s: &str) -> Result<f64, CalcError> {
    s.parse().map_err(|e: ParseFloatError| e.into())
}

fn perform_operation(operation: Operation, operand1: f64, operand2: f64) -> Result<f64, CalcError> {
    match operation {
        Operation::Add => Ok(operand1 + operand2),
        Operation::Subtract => Ok(operand1 - operand2),
        Operation::Multiply => Ok(operand1 * operand2),
        Operation::Divide => {
            if operand2 == 0.0 {
                Err(CalcError::DivisionByZero)
            } else {
                Ok(operand1 / operand2)
            }
        }
    }
}

fn main() -> Result<(), CalcError> {
    let matches = App::new("calc")
        .version("1.0.1")
        .author("Emil winther")
        .about("A custom command-line app for mathematical operations")
        .arg(
            Arg::with_name("operation")
                .short("o")
                .long("operation")
                .takes_value(true)
                .possible_values(&["add", "subtract", "multiply", "divide"])
                .required(true)
                .help("Specify the operation: add, subtract, multiply, or divide"),
        )
        .arg(
            Arg::with_name("operand1")
                .help("The first operand")
                .required(true)
                .index(1),
        )
        .arg(
            Arg::with_name("operand2")
                .help("The second operand")
                .required(true)
                .index(2),
        )
        .get_matches();

    let operation = match matches.value_of("operation").unwrap() {
        "add" => Operation::Add,
        "subtract" => Operation::Subtract,
        "multiply" => Operation::Multiply,
        "divide" => Operation::Divide,
        _ => unreachable!(), // clap ensures only valid values
    };

    let operand1 = parse_operand(matches.value_of("operand1").unwrap())?;
    let operand2 = parse_operand(matches.value_of("operand2").unwrap())?;

    match perform_operation(operation, operand1, operand2) {
        Ok(result) => {
            println!("Result: {}", result);
            Ok(())
        }
        Err(CalcError::OperandParseError(err)) => {
            println!("Error: {}", err);
            Err(CalcError::OperandParseError(err.to_string()))
        }
        Err(CalcError::DivisionByZero) => {
            println!("Error: Division by zero is not allowed.");
            Err(CalcError::DivisionByZero)
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_operand_valid() {
        assert_eq!(parse_operand("5.2").unwrap(), 5.2);
        assert_eq!(parse_operand("-3.0").unwrap(), -3.0);
    }

    #[test]
    fn test_parse_operand_invalid() {
        assert!(parse_operand("abc").is_err());
    }

    #[test]
    fn test_perform_operation_add() {
        let result = perform_operation(Operation::Add, 5.0, 3.0);
        assert_eq!(result.unwrap(), 8.0);
    }

    #[test]
    fn test_perform_operation_subtract() {
        let result = perform_operation(Operation::Subtract, 5.0, 3.0);
        assert_eq!(result.unwrap(), 2.0);
    }

    #[test]
    fn test_perform_operation_multiply() {
        let result = perform_operation(Operation::Multiply, 5.0, 3.0);
        assert_eq!(result.unwrap(), 15.0);
    }

    #[test]
    fn test_perform_operation_divide() {
        let result = perform_operation(Operation::Divide, 6.0, 3.0);
        assert_eq!(result.unwrap(), 2.0);
    }

    #[test]
    fn test_perform_operation_divide_by_zero() {
        let result = perform_operation(Operation::Divide, 5.0, 0.0);
    
        match result {
            Err(CalcError::DivisionByZero) => assert!(true),
            _ => assert!(false, "Expected a DivisionByZero error."),
        }
    }
    
}
