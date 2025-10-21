mod parser;

fn main() {
    let code = "new myVar as int: 10;";
    match parser::parse_variable_declaration(code) {
        Ok((remaining_input, variable)) => {
            println!("Parsed variable: {:?}", variable);
            println!("Remaining input: {}", remaining_input);
        }
        Err(e) => {
            println!("Error parsing code: {:?}", e);
        }
    }
}
