use crate::ast::{Expression, Operator};
use nom::{
    IResult,
    branch::alt,
    bytes::complete::tag,
    character::complete::{alphanumeric1, digit1, i32, multispace0, space0, space1},
    combinator::map,
    multi::fold_many0,
    sequence::{delimited, pair, preceded},
};

#[derive(Debug, PartialEq)]
pub struct VariableDeclaration<'a> {
    pub name: &'a str,
    pub var_type: &'a str,
    pub value: &'a str,
}

pub fn parse_variable_declaration(input: &str) -> IResult<&str, VariableDeclaration> {
    let (input, _) = preceded(multispace0, tag("new"))(input)?;
    let (input, _) = space1(input)?;
    let (input, var_name) = alphanumeric1(input)?;
    let (input, _) = delimited(space1, tag("as"), space1)(input)?;
    let (input, var_type) = alphanumeric1(input)?;
    let (input, _) = delimited(space0, tag(":"), space0)(input)?;
    let (input, value) = digit1(input)?;
    let (input, _) = preceded(space0, tag(";"))(input)?;

    Ok((
        input,
        VariableDeclaration {
            name: var_name,
            var_type,
            value,
        },
    ))
}

fn parse_factor(input: &str) -> IResult<&str, Expression> {
    delimited(multispace0, map(i32, Expression::Number), multispace0)(input)
}

fn parse_term(input: &str) -> IResult<&str, Expression> {
    let (input, init) = parse_factor(input)?;

    fold_many0(
        pair(
            alt((
                map(tag("*"), |_| Operator::Multiply),
                map(tag("/"), |_| Operator::Divide),
            )),
            parse_factor,
        ),
        move || init.clone(),
        |acc, (op, val)| Expression::BinaryOperation {
            left: Box::new(acc),
            operator: op,
            right: Box::new(val),
        },
    )(input)
}

pub fn parse_expression(input: &str) -> IResult<&str, Expression> {
    let (input, init) = parse_term(input)?;

    fold_many0(
        pair(
            alt((
                map(tag("+"), |_| Operator::Add),
                map(tag("-"), |_| Operator::Subtract),
            )),
            parse_term,
        ),
        move || init.clone(),
        |acc, (op, val)| Expression::BinaryOperation {
            left: Box::new(acc),
            operator: op,
            right: Box::new(val),
        },
    )(input)
}
