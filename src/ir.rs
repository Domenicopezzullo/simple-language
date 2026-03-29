#![allow(dead_code)]

use crate::lexer::Token;

#[derive(Debug, Clone, PartialEq)]
pub enum IRArg {
    Int(i32),
    Str(String),
    Var(String),
}

#[derive(Debug, Clone)]
pub enum Condition {
    Equals(IRArg, IRArg),
}

#[derive(Debug, Clone)]
pub enum IR {
    If { condition: Condition, body: Vec<IR> },
    FuncDef { name: String, body: Vec<IR> },
    Funcall { name: String, args: Vec<IRArg> },
    VarAssignment { name: String, value: i32 },
}

fn parse_one<'a, I>(tokens: &mut std::iter::Peekable<I>) -> Option<IR>
where
    I: Iterator<Item = &'a Token>,
{
    let token = tokens.next()?;

    match token {
        Token::Builtin(name) => {
            match tokens.next()? {
                Token::OpenParen => {}
                _ => return None,
            }
            let mut args = Vec::new();
            loop {
                match tokens.peek() {
                    Some(Token::CloseParen) => {
                        tokens.next();
                        break;
                    }
                    Some(Token::IntLit(_)) => {
                        if let Some(Token::IntLit(n)) = tokens.next() {
                            args.push(IRArg::Int(*n));
                        }
                    }
                    Some(Token::StringLit(_)) => {
                        if let Some(Token::StringLit(s)) = tokens.next() {
                            args.push(IRArg::Str(s.clone()));
                        }
                    }
                    Some(Token::Ident(_)) => {
                        if let Some(Token::Ident(s)) = tokens.next() {
                            args.push(IRArg::Var(s.clone()));
                        }
                    }
                    _ => {
                        tokens.next();
                    }
                }
            }
            Some(IR::Funcall {
                name: format!("@{}", name),
                args,
            })
        }
        Token::Ident(kw) if kw == "fn" => {
            let name = match tokens.next()? {
                Token::Ident(n) => n.clone(),
                _ => return None,
            };
            match tokens.next()? {
                Token::OpenParen => {}
                _ => return None,
            }
            match tokens.next()? {
                Token::CloseParen => {}
                _ => return None,
            }
            match tokens.next()? {
                Token::OpenBrace => {}
                _ => return None,
            }

            let mut body = Vec::new();
            loop {
                match tokens.peek() {
                    Some(Token::CloseBrace) => {
                        tokens.next();
                        break;
                    }
                    None => break,
                    _ => {
                        if let Some(ir) = parse_one(tokens) {
                            body.push(ir);
                        } else {
                            break;
                        }
                    }
                }
            }

            Some(IR::FuncDef { name, body })
        }

        Token::If => {
            let lhs = parse_arg(tokens)?;
            match tokens.next()? {
                Token::DEquals => {}
                _ => return None,
            }
            let rhs = parse_arg(tokens)?;
            match tokens.next()? {
                Token::OpenBrace => {}
                _ => return None,
            }
            let mut body = Vec::new();
            loop {
                match tokens.peek() {
                    Some(Token::CloseBrace) => {
                        tokens.next();
                        break;
                    }
                    None => break,
                    _ => {
                        if let Some(ir) = parse_one(tokens) {
                            body.push(ir);
                        } else {
                            break;
                        }
                    }
                }
            }
            Some(IR::If {
                condition: Condition::Equals(lhs, rhs),
                body,
            })
        }

        Token::Ident(name) => match tokens.peek() {
            Some(Token::ColonColon) => {
                tokens.next();
                match tokens.next()? {
                    Token::IntLit(n) => Some(IR::VarAssignment {
                        name: name.clone(),
                        value: *n,
                    }),
                    _ => None,
                }
            }

            Some(Token::OpenParen) => {
                tokens.next();
                let mut args = Vec::new();

                loop {
                    match tokens.peek() {
                        Some(Token::CloseParen) => {
                            tokens.next();
                            break;
                        }
                        Some(Token::IntLit(_)) => {
                            if let Some(Token::IntLit(n)) = tokens.next() {
                                args.push(IRArg::Int(*n));
                            }
                        }
                        Some(Token::Ident(_)) => {
                            if let Some(Token::Ident(n)) = tokens.next() {
                                args.push(IRArg::Var(n.clone()))
                            }
                        }
                        Some(Token::StringLit(_)) => {
                            if let Some(Token::StringLit(s)) = tokens.next() {
                                args.push(IRArg::Str(s.clone()));
                            }
                        }
                        _ => {
                            tokens.next();
                        }
                    }
                }

                Some(IR::Funcall {
                    name: name.clone(),
                    args,
                })
            }
            _ => None,
        },

        _ => None,
    }
}

fn parse_arg<'a, I>(tokens: &mut std::iter::Peekable<I>) -> Option<IRArg>
where
    I: Iterator<Item = &'a Token>,
{
    match tokens.next()? {
        Token::IntLit(n) => Some(IRArg::Int(*n)),
        Token::StringLit(s) => Some(IRArg::Str(s.clone())),
        Token::Ident(s) => Some(IRArg::Var(s.clone())),
        _ => None,
    }
}

pub fn generate_ir(vec: &[Token]) -> Vec<IR> {
    let mut irs = Vec::new();
    let mut tokens = vec.iter().peekable();
    while let Some(ir) = parse_one(&mut tokens) {
        irs.push(ir);
    }
    irs
}
