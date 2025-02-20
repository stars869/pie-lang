use nom::character::complete::{char, multispace0};
use nom::multi::many0;
use nom::sequence::{delimited, preceded, terminated};
use nom::{IResult, Parser};
use nom::branch::alt;
use nom::combinator::map;
use nom::bytes::complete::{tag, is_not};

use crate::sexpr::SExpr;

// Parse comments starting with '--'
fn comment(input: &str) -> IResult<&str, ()> {
    let (input, _) = preceded(tag("--"), is_not("\n")).parse(input)?;
    Ok((input, ()))
}

// Parse identifiers (any non-space string)
fn identifier(input: &str) -> IResult<&str, SExpr> {
    map(is_not(" \t\n()"), |s: &str| SExpr::Identifier(s.to_string())).parse(input)
}

// Parse lists (expressions in parentheses)
fn list(input: &str) -> IResult<&str, SExpr> {
    let (input, items) = delimited(
        preceded(multispace0, char('(')), 
        many0(sexpr), 
        preceded(multispace0, char(')'))).parse(input)?;
    Ok((input, SExpr::List(items)))
}

// Parse raw expressions (identifiers or lists)
fn sexpr(input: &str) -> IResult<&str, SExpr> {
    alt(
        (terminated(preceded(multispace0, identifier), multispace0), 
        terminated(preceded(multispace0, list), multispace0))
    ).parse(input)
}

// Parse multiple raw expressions, ignoring comments
pub fn parse_program(input: &str) -> IResult<&str, Vec<SExpr>> {
    many0(alt((
        map(preceded(multispace0, comment), |_| None),
        map(sexpr, Some)
    ))).parse(input)
    .map(|(remaining, exprs)| (remaining, exprs.into_iter().flatten().collect()))
}
