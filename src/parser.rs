use nom::{
    IResult,
    bytes::complete::tag,
    character::complete::{alphanumeric1, digit1, multispace0, space0, space1},
    sequence::{delimited, preceded},
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
  