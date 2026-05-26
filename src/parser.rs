use std::iter::Peekable;

use crate::{
    error::{Error, Result},
    expression::{Constant, Expression},
    lexer::Lexer,
    operator::{InfixOperator, Operator, PostfixOperator, PrefixOperator},
    token::{self, Token},
};

pub struct Parser<'a> {
    lexer: Peekable<Lexer<'a>>,
}

impl<'a> Parser<'a> {
    pub fn new(input: &'a str) -> Self {
        Parser {
            lexer: Lexer::new(input).peekable(),
        }
    }

    fn next(&mut self) -> Result<Token> {
        self.lexer
            .next()
            .unwrap_or_else(|| Err(Error::Parse("Unexpected end of input".into())))
    }

    fn peek(&mut self) -> Result<Option<Token>> {
        self.lexer
            .peek()
            .cloned()
            .map_or(Ok(None), |r| Ok(Some(r?)))
    }

    fn next_if<F: Fn(&Token) -> bool>(&mut self, predicate: F) -> Option<Token> {
        let _ = self.peek().unwrap_or(None).filter(|t| predicate(&t))?;
        self.next().ok()
    }

    fn next_if_operator<T: Operator>(&mut self, min_prec: u8) -> Option<T> {
        let operator = self
            .peek()
            .unwrap_or(None)
            .and_then(|token| T::from(&token))
            .filter(|predicate| predicate.precedence() >= min_prec)?;
        self.next().ok();
        Some(operator)
    }

    fn next_expect(&mut self, expect: Option<Token>) -> Result<Option<Token>> {
        if let Some(t) = expect {
            let token = self.next()?;
            if token == t {
                return Ok(Some(token));
            } else {
                return Err(Error::Parse(format!(
                    "Expected token {}, found {}",
                    t, token
                )));
            }
        } else if let Some(token) = self.peek()? {
            return Err(Error::Parse(format!("Unexpected token {}", token)));
        } else {
            return Ok(None);
        }
    }
}

impl<'a> Parser<'a> {
    fn build_constant(&self, name: String) -> Result<Expression> {
        // println!("name:{}", name);
        match name.to_lowercase().as_str() {
            "e" => Ok(Constant::E.into()),
            "inf" => Ok(Constant::Infinity.into()),
            "nan" => Ok(Constant::NaN.into()),
            "pi" => Ok(Constant::Pi.into()),
            "π" => Ok(Constant::Pi.into()),
            _ => Err(Error::Parse(format!("Unknown constant {}", name))),
        }
    }

    fn build_function(&mut self, name: String, mut args: Vec<Expression>) -> Result<Expression> {
        args.reverse();
        let mut arg = || {
            args.pop()
                .map(|e| e.into())
                .ok_or_else(|| Error::Parse(format!("Missing argument for {}()", name)))
        };

        let fun = match name.to_lowercase().as_str() {
            "cos" => Expression::Cosine(arg()?),
            "sin" => Expression::Sine(arg()?),
            "tan" => Expression::Tangent(arg()?),
            "degrees" => Expression::Degrees(arg()?),
            "radians" => Expression::Radians(arg()?),
            "sqrt" => Expression::SquareRoot(arg()?),
            "round" => Expression::Round {
                value: arg()?,
                decimals: arg().unwrap_or_else(|_| 0.0.into()),
            },
            _ => return Err(Error::Parse(format!("Unknown function {}", name))),
        };
        if args.is_empty() {
            Ok(fun)
        } else {
            Err(Error::Parse(format!("Unexpected argument for {}()", name)))
        }
    }

    fn build_number(&mut self, num: String) -> Result<Expression> {
        Ok(num.parse::<f64>()?.into())
    }
}

impl<'a> Parser<'a> {
    fn parse_expression(&mut self, min_prec: u8) -> Result<Expression> {
        let mut lhs = if let Some(prefix) = self.next_if_operator::<PrefixOperator>(min_prec) {
            prefix.build(self.parse_expression(prefix.precedence() + prefix.associativity())?)
        } else {
            self.parse_atom()?
        };
        while let Some(postfix) = self.next_if_operator::<PostfixOperator>(min_prec) {
            lhs = postfix.build(lhs)
        }
        while let Some(infix) = self.next_if_operator::<InfixOperator>(min_prec) {
            lhs = infix.build(
                lhs,
                self.parse_expression(infix.precedence() + infix.associativity())?,
            )
        }
        Ok(lhs)
    }

    fn parse_atom(&mut self) -> Result<Expression> {
        match self.next()? {
            Token::Ident(name) => {
                // println!("name:{}", name);
                if self.next_if(|t| *t == Token::OpenParen).is_some() {
                    let mut args = Vec::new();
                    while self.next_if(|t| *t == Token::CloseParen).is_none() {
                        if !args.is_empty() {
                            self.next_expect(Some(Token::Comma))?;
                        }
                        args.push(self.parse_expression(0)?)
                    }
                    self.build_function(name, args)
                } else {
                    self.build_constant(name)
                }
            }
            Token::Number(num) => self.build_number(num),
            Token::OpenParen => {
                let expr = self.parse_expression(0)?;
                self.next_expect(Some(Token::CloseParen))?;
                Ok(expr)
            }
            t => return Err(Error::Parse(format!("Expected value, found {}", t))),
        }
    }

    pub fn parse(&mut self) -> Result<Expression> {
        let expr = self.parse_expression(0)?;
        self.next_expect(None)?;
        Ok(expr)
    }
}
