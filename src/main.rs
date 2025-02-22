use std::fs;

mod sexpr;
mod sexpr_parser;
mod expr;
mod expr_parser;

use sexpr_parser::parse_program;
use expr_parser::parse_statement;

fn main() -> std::io::Result<()> {
    let content = fs::read_to_string("example/example.pie")?;
    let result = parse_program(&content).map_err(|e| format!("Parse S-Exprssion Error: {:?}", e));
    
    match result {
        Ok((remain, sexprs)) => {
            assert!(remain.is_empty());
            for sexpr in &sexprs {
                let result2 = parse_statement(sexpr);
                match result2{
                    Ok(statement) => println!("{}", statement),
                    Err(err_msg) => println!("Syntax Error: {}", err_msg),
                }
            }
        },
        Err(error) => println!("{}", error),
    }
    
    Ok(())
}
