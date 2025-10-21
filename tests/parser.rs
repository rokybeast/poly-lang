use poly::ast::{Expression, Operator};
use poly::parser::parse_expression;

#[test]
fn test_parse_addition() {
    let input = "1 + 2";
    let expected = Expression::BinaryOperation {
        left: Box::new(Expression::Number(1)),
        operator: Operator::Add,
        right: Box::new(Expression::Number(2)),
    };
    let (_, actual) = parse_expression(input).unwrap();
    assert_eq!(expected, actual);
}

#[test]
fn test_parse_subtraction() {
    let input = "10 - 5";
    let expected = Expression::BinaryOperation {
        left: Box::new(Expression::Number(10)),
        operator: Operator::Subtract,
        right: Box::new(Expression::Number(5)),
    };
    let (_, actual) = parse_expression(input).unwrap();
    assert_eq!(expected, actual);
}

#[test]
fn test_parse_multiplication() {
    let input = "3 * 4";
    let expected = Expression::BinaryOperation {
        left: Box::new(Expression::Number(3)),
        operator: Operator::Multiply,
        right: Box::new(Expression::Number(4)),
    };
    let (_, actual) = parse_expression(input).unwrap();
    assert_eq!(expected, actual);
}

#[test]
fn test_parse_division() {
    let input = "20 / 5";
    let expected = Expression::BinaryOperation {
        left: Box::new(Expression::Number(20)),
        operator: Operator::Divide,
        right: Box::new(Expression::Number(5)),
    };
    let (_, actual) = parse_expression(input).unwrap();
    assert_eq!(expected, actual);
}

#[test]
fn test_parse_precedence() {
    let input = "2 + 3 * 4";
    let expected = Expression::BinaryOperation {
        left: Box::new(Expression::Number(2)),
        operator: Operator::Add,
        right: Box::new(Expression::BinaryOperation {
            left: Box::new(Expression::Number(3)),
            operator: Operator::Multiply,
            right: Box::new(Expression::Number(4)),
        }),
    };
    let (_, actual) = parse_expression(input).unwrap();
    assert_eq!(expected, actual);
}
