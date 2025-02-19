use nom::character::complete::{char, multispace0};
use nom::multi::many0;
use nom::sequence::{delimited, preceded};
use nom::{IResult, Parser};
use nom::branch::alt;
use nom::combinator::map;
use nom::bytes::complete::{tag, is_not};

use crate::raw_expr::RawExpr;

// Parse comments starting with '--'
fn comment(input: &str) -> IResult<&str, ()> {
    let (input, _) = preceded(tag("--"), is_not("\n")).parse(input)?;
    Ok((input, ()))
}

// Parse identifiers (any non-space string)
fn identifier(input: &str) -> IResult<&str, RawExpr> {
    map(is_not(" \t\n()"), |s: &str| RawExpr::Identifier(s.to_string())).parse(input)
}

// Parse lists (expressions in parentheses)
fn list(input: &str) -> IResult<&str, RawExpr> {
    let (input, items) = delimited(
        preceded(multispace0, char('(')), 
        many0(raw_expr), 
        preceded(multispace0, char(')'))).parse(input)?;
    Ok((input, RawExpr::List(items)))
}

// Parse raw expressions (identifiers or lists)
fn raw_expr(input: &str) -> IResult<&str, RawExpr> {
    alt(
        (preceded(multispace0, identifier), 
        preceded(multispace0, list))
    ).parse(input)
}

// Parse multiple raw expressions, ignoring comments
pub fn parse_program(input: &str) -> IResult<&str, Vec<RawExpr>> {
    many0(alt((
        map(preceded(multispace0, comment), |_| None),
        map(raw_expr, Some)
    ))).parse(input)
    .map(|(remaining, exprs)| (remaining, exprs.into_iter().flatten().collect()))
}
