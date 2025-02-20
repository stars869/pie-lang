use std::fmt;

#[derive(Debug, Clone)]
pub struct Identifier(pub String);

#[derive(Debug, Clone)]
pub struct Parameter(pub Identifier, pub Box<Expr>);

#[derive(Debug, Clone)]
pub struct Pi(pub Vec<Parameter>, pub Box<Expr>);

#[derive(Debug, Clone)]
pub enum Expr {
    Identifier(Identifier),
    Lambda(Pi, Box<Expr>),
    Application(Box<Expr>, Vec<Expr>),
    Match(Box<Expr>, Vec<(Box<Expr>, Box<Expr>)>),
}

#[derive(Debug, Clone)]
pub enum Statement {
    Postulate(Identifier),
    Claim(Identifier, Box<Expr>),
    Define(Identifier, Box<Expr>),
}


impl fmt::Display for Identifier {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl fmt::Display for Parameter {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "(: {} {})", self.0, self.1)
    }
}

impl fmt::Display for Pi {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let params = self.0.iter().map(|p| p.to_string()).collect::<Vec<_>>().join(" ");
        write!(f, "(Pi {} {})", params, self.1)
    }
}

impl fmt::Display for Expr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Expr::Identifier(id) => write!(f, "{}", id),
            Expr::Lambda(pi, body) => write!(f, "(lambda {} {})", pi, body),
            Expr::Application(func, args) => {
                let args_str = args.iter().map(|arg| arg.to_string()).collect::<Vec<_>>().join(" ");
                write!(f, "({} {})", func, args_str)
            }
            Expr::Match(expr, cases) => {
                let cases_str = cases.iter()
                    .map(|(pat, body)| format!("(case {} {})", pat, body))
                    .collect::<Vec<_>>().join(" ");
                write!(f, "(match {} {})", expr, cases_str)
            }
        }
    }
}

impl fmt::Display for Statement {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Statement::Postulate(id) => write!(f, "(postulate {})", id),
            Statement::Claim(id, expr) => write!(f, "(claim {} {})", id, expr),
            Statement::Define(id, expr) => write!(f, "(define {} {})", id, expr),
        }
    }
}
