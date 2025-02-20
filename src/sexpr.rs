use std::fmt;

#[derive(Debug, Clone)]
pub enum SExpr {
    Identifier(String),
    List(Vec<SExpr>),
}

impl fmt::Display for SExpr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let output = match self {
            SExpr::Identifier(s) => s.clone(),
            SExpr::List(items) => {
                let inner: Vec<String> = items.iter().map(|item| item.to_string()).collect();
                format!("({})", inner.join(" "))
            }
        };
        write!(f, "{}", output)
    }
}