use either::Either;

use crate::sexpr::SExpr;
use crate::expr::{Identifier, Parameter, Expr, Statement, Pi};


pub fn parse_statement(raw: &SExpr) -> Result<Statement, String> {
    match raw {
        SExpr::List(items) => match items.as_slice() {
            [SExpr::Identifier(ref name), SExpr::Identifier(ref id)] if name == "postulate" => {
                Ok(Statement::Postulate(Identifier(id.clone())))
            }
            [SExpr::Identifier(ref name), SExpr::Identifier(ref id), expr] if name == "claim" => {
                Ok(Statement::Claim(Identifier(id.clone()), Box::new(parse_expr(expr.clone())?)))
            }
            [SExpr::Identifier(ref name), SExpr::Identifier(ref id), expr] if name == "define" => {
                Ok(Statement::Define(Identifier(id.clone()), Box::new(parse_expr(expr.clone())?)))
            }
            _ => Err("Unknown statement".to_string()),
        },
        _ => Err("Not a statement".to_string()),
    }
}

fn parse_pi(raw: &[SExpr]) -> Result<Pi, String> {
    if raw.is_empty() {
        return Err("Pi must have at least one parameter and a return type".to_string());
    }
    let (param_args, return_type) = raw.split_at(raw.len() - 1);
    let params = param_args.iter().map(|p| parse_parameter(p.clone())).collect::<Result<Vec<_>, _>>()?;
    let return_expr = parse_expr(return_type[0].clone())?;
    Ok(Pi(params, Box::new(return_expr)))
}

fn parse_expr(raw: SExpr) -> Result<Expr, String> {
    match raw {
        SExpr::Identifier(s) => Ok(Expr::Identifier(Identifier(s))),
        SExpr::List(items) => match items.as_slice() {
            [SExpr::Identifier(ref name), args @ ..] if name == "lambda" => {
                if args.is_empty() {
                    return Err("Lambda must have parameters and a body".to_string());
                }
                let (param_args, body_expr) = args.split_at(args.len() - 1);
                let pi = parse_pi(param_args)?;
                let body = parse_expr(body_expr[0].clone())?;
                Ok(Expr::Lambda(pi, Box::new(body)))
            }
            [SExpr::Identifier(ref name), expr, cases @ ..] if name == "match" => {
                let matched_expr = parse_expr(expr.clone())?;
                let cases_parsed = cases.iter().map(|case_expr| {
                    match case_expr {
                        SExpr::List(case_items) => {
                            if let [SExpr::Identifier(case_kw), pattern, body] = case_items.as_slice() {
                                if case_kw == "case" {
                                    let pattern_expr = parse_expr(pattern.clone())?;
                                    let body_expr = parse_expr(body.clone())?;
                                    Ok((Box::new(pattern_expr), Box::new(body_expr)))
                                } else {
                                    Err("Expected 'case' keyword before pattern".to_string())
                                }
                            } else {
                                Err("Invalid match case syntax".to_string())
                            }
                        }
                        _ => Err("Match case must be a list".to_string()),
                    }
                }).collect::<Result<Vec<_>, _>>()?;
                Ok(Expr::Match(Box::new(matched_expr), cases_parsed))
            }
            [func, args @ ..] => {
                let function_expr = parse_expr(func.clone())?;
                let arg_exprs = args.iter().map(|arg| parse_expr(arg.clone())).collect::<Result<Vec<_>, _>>()?;
                Ok(Expr::Application(Box::new(function_expr), arg_exprs))
            }
            _ => Err("Unknown expression".to_string()),
        },
    }
}

fn parse_parameter(raw: SExpr) -> Result<Parameter, String> {
    match raw {
        SExpr::List(items) if items.len() == 3 => {
            if let [SExpr::Identifier(colon), SExpr::Identifier(name), expr] = items.as_slice() {
                if colon == ":" {
                    let param_expr = parse_expr(expr.clone())?;
                    return Ok(Parameter(Identifier(name.clone()), Box::new(param_expr)));
                }
            }
            Err("Invalid parameter format: Expected (: name Type)".to_string())
        }
        _ => Err("Invalid parameter format".to_string()),
    }
}