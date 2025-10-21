use poly::ast::{Expression, Operator};
use poly::evaluator::evaluate_expression;

#[test]
fn test_evaluate_addition() {
    let expression = Expression::BinaryOperation {
        left: Box::new(Expression::Number(2)),
        operator: Operator::Add,
        right: Box::new(Expression::Number(3)),
    };
    assert_eq!(evaluate_expression(&expression), 5);
}

#[test]
fn test_evaluate_subtraction() {
    let expression = Expression::BinaryOperation {
        left: Box::new(Expression::Number(10)),
        operator: Operator::Subtract,
        right: Box::new(Expression::Number(4)),
    };
    assert_eq!(evaluate_expression(&expression), 6);
}

#[test]
fn test_evaluate_multiplication() {
    let expression = Expression::BinaryOperation {
        left: Box::new(Expression::Number(5)),
        operator: Operator::Multiply,
        right: Box::new(Expression::Number(6)),
    };
    assert_eq!(evaluate_expression(&expression), 30);
}

#[test]
fn test_evaluate_division() {
    let expression = Expression::BinaryOperation {
        left: Box::new(Expression::Number(20)),
        operator: Operator::Divide,
        right: Box::new(Expression::Number(4)),
    };
    assert_eq!(evaluate_expression(&expression), 5);
}

#[test]
fn test_evaluate_precedence() {
    let expression = Expression::BinaryOperation {
        left: Box::new(Expression::Number(2)),
        operator: Operator::Add,
        right: Box::new(Expression::BinaryOperation {
            left: Box::new(Expression::Number(3)),
            operator: Operator::Multiply,
            right: Box::new(Expression::Number(4)),
        }),
    };
    assert_eq!(evaluate_expression(&expression), 14);
}
