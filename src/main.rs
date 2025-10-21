use poly::evaluator::evaluate_expression;
use poly::parser::parse_expression;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        println!("Usage: poly <expression>");
        return;
    }

    let input = &args[1];
    match parse_expression(input) {
        Ok((_, expression)) => {
            let result = evaluate_expression(&expression);
            println!("Result: {}", result);
        }
        Err(err) => println!("Error parsing expression: {:?}", err),
    }
}
