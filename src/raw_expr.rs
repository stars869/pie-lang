use std::fmt;

#[derive(Debug, Clone)]
pub enum RawExpr {
    Identifier(String),
    List(Vec<RawExpr>),
}

impl fmt::Display for RawExpr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let output = match self {
            RawExpr::Identifier(s) => s.clone(),
            RawExpr::List(items) => {
                let inner: Vec<String> = items.iter().map(|item| item.to_string()).collect();
                format!("({})", inner.join(" "))
            }
        };
        write!(f, "{}", output)
    }
}