use either::Either;

use crate::raw_expr::RawExpr;
use crate::expr::{Identifier, Parameter, Expr, Statement};


// Convert RawExpr to either Statement or Expr
fn parse_semantics(raw: RawExpr) -> Result<Either<Statement, Expr>, String> {
    if let Ok(stmt) = parse_statement(&raw) {
        return Ok(Either::Left(stmt));
    }
    if let Ok(expr) = parse_expr(raw) {
        return Ok(Either::Right(expr));
    }
    Err("Failed to parse semantics".to_string())
}

fn parse_statement(raw: &RawExpr) -> Result<Statement, String> {
    match raw {
        RawExpr::List(items) => match items.as_slice() {
            [RawExpr::Identifier(ref name), RawExpr::Identifier(ref id)] if name == "postulate" => {
                Ok(Statement::Postulate(Identifier(id.clone())))
            }
            [RawExpr::Identifier(ref name), RawExpr::Identifier(ref id), expr] if name == "claim" => {
                Ok(Statement::Claim(Identifier(id.clone()), Box::new(parse_expr(expr.clone())?)))
            }
            [RawExpr::Identifier(ref name), RawExpr::Identifier(ref id), expr_type, expr_value] if name == "define" => {
                Ok(Statement::Define(Identifier(id.clone()), Box::new(parse_expr(expr_type.clone())?), Box::new(parse_expr(expr_value.clone())?)))
            }
            _ => Err("Unknown statement".to_string()),
        },
        _ => Err("Not a statement".to_string()),
    }
}

fn parse_expr(raw: RawExpr) -> Result<Expr, String> {
    match raw {
        RawExpr::Identifier(s) => Ok(Expr::Identifier(s)),
        RawExpr::List(items) => match items.as_slice() {
            [RawExpr::Identifier(ref name), args @ ..] if name == "Pi" => {
                let params = args.iter().map(|p| parse_parameter(p.clone())).collect::<Result<Vec<_>, _>>()?;
                Ok(Expr::Pi(params, Box::new(Expr::Identifier("Unknown".to_string()))))
            }
            _ => Err("Unknown expression".to_string()),
        },
    }
}

fn parse_parameter(raw: RawExpr) -> Result<Parameter, String> {
    match raw {
        RawExpr::List(items) if items.len() == 2 => {
            if let RawExpr::Identifier(name) = &items[0] {
                let param_expr = parse_expr(items[1].clone())?;
                Ok(Parameter(Identifier(name.clone()), Box::new(param_expr)))
            } else {
                Err("Invalid parameter format".to_string())
            }
        }
        _ => Err("Invalid parameter format".to_string()),
    }
}