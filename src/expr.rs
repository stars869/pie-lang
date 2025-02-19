#[derive(Debug, Clone)]
pub struct Identifier(pub String);

#[derive(Debug, Clone)]
pub struct Parameter(Identifier, Box<Expr>);

#[derive(Debug, Clone)]
pub enum Expr {
    Identifier(Identifier),
    Pi(Vec<Parameter>, Box<Expr>),
    Lambda(Vec<Parameter>, Box<Expr>),
    Application(Box<Expr>, Vec<Expr>),
    Match(Box<Expr>, Vec<(Box<Expr>, Box<Expr>)>),
}

#[derive(Debug, Clone)]
pub enum Statement {
    Postulate(Identifier),
    Claim(Identifier, Box<Expr>),
    Define(Identifier, Box<Expr>, Box<Expr>),
}