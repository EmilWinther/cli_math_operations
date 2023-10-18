extern crate clap;

use clap::{App, Arg};
use std::num::ParseFloatError;

enum Operation {
    Add,
    Subtract,
    Multiply,
    Divide,
    Power,
    Sqrt,
    Modulo,
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

fn perform_operation(operation: Operation, operand1: f64, operand2: Option<f64>) -> Result<f64, CalcError> {
    match operation {
        Operation::Add => {
            if let Some(op2) = operand2 {
                Ok(operand1 + op2)
            } else {
                Err(CalcError::OperandParseError("Missing second operand for addition".to_string()))
            }
        },
        Operation::Subtract => {
            if let Some(op2) = operand2 {
                Ok(operand1 - op2)
            } else {
                Err(CalcError::OperandParseError("Missing second operand for subtraction".to_string()))
            }
        },
        Operation::Multiply => {
            if let Some(op2) = operand2 {
                Ok(operand1 * op2)
            } else {
                Err(CalcError::OperandParseError("Missing second operand for multiplication".to_string()))
            }
        },
        Operation::Divide => {
            if let Some(op2) = operand2 {
                if op2 == 0.0 {
                    Err(CalcError::DivisionByZero)
                } else {
                    Ok(operand1 / op2)
                }
            } else {
                Err(CalcError::OperandParseError("Missing second operand for division".to_string()))
            }
        },
        Operation::Power => {
            if let Some(op2) = operand2 {
                Ok(operand1.powf(op2))
            } else {
                Err(CalcError::OperandParseError("Missing second operand for power".to_string()))
            }
        },
        Operation::Sqrt => {
            if operand1 < 0.0 {
                Err(CalcError::OperandParseError("Cannot square root a negative number".to_string()))
            } else {
                Ok(operand1.sqrt())
            }
        },
        Operation::Modulo => {
            if let Some(op2) = operand2 {
                Ok(operand1 % op2)
            } else {
                Err(CalcError::OperandParseError("Missing second operand for modulo".to_string()))
            }
        },
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
                .possible_values(&["add", "subtract", "multiply", "divide", "power", "sqrt", "modulo"])
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
                .required(false)
                .index(2),
        )
        .get_matches();

        let operation = match matches.value_of("operation").unwrap() {
            "add" => Operation::Add,
            "subtract" => Operation::Subtract,
            "multiply" => Operation::Multiply,
            "divide" => Operation::Divide,
            "power" => Operation::Power,
            "sqrt" => Operation::Sqrt,
            "modulo" => Operation::Modulo,
            _ => unreachable!(),
        };
        

    let operand1 = parse_operand(matches.value_of("operand1").unwrap())?;
    let operand2 = if let Some(val) = matches.value_of("operand2") {
        Some(parse_operand(val)?)
    } else {
        None
    };
    

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
        let result = perform_operation(Operation::Add, 5.0, Some(3.0));
        assert_eq!(result.unwrap(), 8.0);
    }

    #[test]
    fn test_perform_operation_subtract() {
        let result = perform_operation(Operation::Subtract, 5.0, Some(3.0));
        assert_eq!(result.unwrap(), 2.0);
    }

    #[test]
    fn test_perform_operation_multiply() {
        let result = perform_operation(Operation::Multiply, 5.0, Some(3.0));
        assert_eq!(result.unwrap(), 15.0);
    }

    #[test]
    fn test_perform_operation_divide() {
        let result = perform_operation(Operation::Divide, 6.0, Some(3.0));
        assert_eq!(result.unwrap(), 2.0);
    }

    #[test]
    fn test_perform_operation_divide_by_zero() {
        let result = perform_operation(Operation::Divide, 5.0, Some(0.0));
    
        match result {
            Err(CalcError::DivisionByZero) => assert!(true),
            _ => assert!(false, "Expected a DivisionByZero error."),
        }
    }
    
}
