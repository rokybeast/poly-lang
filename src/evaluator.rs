use crate::ast::{Expression, Operator};

pub fn evaluate_expression(expression: &Expression) -> i32 {
    match expression {
        Expression::Number(n) => *n,
        Expression::BinaryOperation { left, operator, right } => {
            let left_value = evaluate_expression(left);
            let right_value = evaluate_expression(right);
            match operator {
                Operator::Add => left_value + right_value,
                Operator::Subtract => left_value - right_value,
                Operator::Multiply => left_value * right_value,
                Operator::Divide => left_value / right_value,
            }
        }
    }
}
