mod expr;
mod parser;

use std::fs;

use parser::parse;

fn main() -> std::io::Result<()> {
    let content = fs::read_to_string("example/example.pie")?;
    let result = parse(&content);
    
    println!("{}", result);
    Ok(())
}
