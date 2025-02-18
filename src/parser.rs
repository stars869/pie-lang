use nom::character::complete::{alpha1, alphanumeric1, char, line_ending, space0, space1};
use nom::multi::many0;
use nom::sequence::{delimited, pair, preceded, terminated};
use nom::{IResult, Parser};
use nom::branch::alt;
use nom::combinator::{map, opt};
use nom::bytes::complete::{tag, is_not};

use crate::expr::{Identifier, Expr};

// Parse comments starting with '--'
fn comment(input: &str) -> IResult<&str, Expr> {
    let (input, _) = preceded(tag("--"), is_not("\n")).parse(input)?;
    Ok((input, Expr::Identifier("Comment".to_string())))
}

// Parse identifiers (variable names, keywords)
fn identifier(input: &str) -> IResult<&str, Identifier> {
    map(pair(alpha1, opt(alphanumeric1)), |(a, b)| {
        let mut s = a.to_string();
        if let Some(b) = b {
            s.push_str(b);
        }
        Identifier(s)
    })(input)
}

// Parse (postulate name)
fn postulate(input: &str) -> IResult<&str, Expr> {
    let (input, _) = tag("(postulate ")(input)?;
    let (input, name) = identifier(input)?;
    let (input, _) = char(')')(input)?;
    Ok((input, Expr::Postulate(name)))
}

// Parse (claim name type)
fn claim(input: &str) -> IResult<&str, Expr> {
    let (input, _) = tag("(claim ")(input)?;
    let (input, name) = identifier(input)?;
    let (input, _) = space1(input)?;
    let (input, typ) = expr(input)?;
    let (input, _) = char(')')(input)?;
    Ok((input, Expr::Claim(name, Box::new(typ))))
}

// Parse (define name expr)
fn define(input: &str) -> IResult<&str, Expr> {
    let (input, _) = tag("(define ")(input)?;
    let (input, name) = identifier(input)?;
    let (input, _) = space1(input)?;
    let (input, expr) = expr(input)?;
    let (input, _) = char(')')(input)?;
    Ok((input, Expr::Define(name, Box::new(expr))))
}

// Parse expressions
fn expr(input: &str) -> IResult<&str, Expr> {
    alt((
        postulate,
        claim,
        define,
        map(identifier, |id| Expr::Identifier(id.0)),
        comment,
    ))(input)
}

// Parse multiple expressions, ignoring comments
fn parse_program(input: &str) -> IResult<&str, Vec<Expr>> {
    many0(terminated(expr, many0(line_ending)))(input)
}

