
#[derive(Debug, Clone)]
pub struct Identifier(String);

#[derive(Debug, Clone)]
pub struct Parameter(Identifier, Box<Expr>);

#[derive(Debug, Clone)]
pub struct PiType(Vec<Parameter>, Box<Expr>);

#[derive(Debug, Clone)]
pub enum Expr {
    Identifier(String),
    Postulate(Identifier),
    Claim(Identifier, Box<Expr>),
    Define(Identifier, Box<Expr>),
    Parameter(Identifier, Box<Expr>), // Boxed Expr to prevent deep recursion
    PiType(Vec<Parameter>, Box<Expr>),
    Lambda(Box<PiType>, Box<Expr>),
    Match(Box<Expr>, Vec<(Box<Expr>, Box<Expr>)>), // Boxed tuple elements
}
