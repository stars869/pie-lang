mod expr;
mod expr_parser;
mod raw_expr;
mod raw_expr_parser;


use std::fs;

// use parser::parse;
use raw_expr_parser::parse_program;

fn main() -> std::io::Result<()> {
    let content = fs::read_to_string("example/example.pie")?;
    let result = parse_program(&content);
    
    match result {
        Ok((_, raw_exprs)) => {
            for raw_expr in &raw_exprs {
                println!("{}", raw_expr)
            }
        },
        Err(error) => println!("{}", error),
    }
    
    Ok(())
}
