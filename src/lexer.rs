use std::{iter::Peekable, str::Chars};

use crate::token::Token;
use crate::error::{Error, Result};

#[derive(Debug)]
pub struct Lexer<'a> {
    pub iter: Peekable<Chars<'a>>,
}

impl<'a> Lexer<'a> {
    pub fn new(input: &'a str) -> Self {
        Self {
            iter: input.chars().peekable(),
        }
    }

    fn next_if<F: Fn(char) -> bool>(&mut self, predicate: F) -> Option<char> {
        // 只看一眼，不拿走
        self.iter.peek().filter(|&c| predicate(*c))?;
        // 拿走第一个字符
        self.iter.next()
    }

    fn next_if_token<F: Fn(char) -> Option<Token>>(&mut self, tokenizer: F) -> Option<Token> {
        // 1. peek() 偷看下一个字符，不拿走
        // 2. and_then(...) 用你给的 tokenizer 尝试把字符转成 Token
        let token = self.iter.peek().and_then(|&c| tokenizer(c))?;
        // 拿走第一个字符
        self.iter.next();
        Some(token)
    }

    fn next_while<F: Fn(char) -> bool>(&mut self, predicate: F) -> Option<String> {
        let mut value = String::new();
        // 这个predicate在每次循环的时候都会调用, 如果单单传一个predicate就是值传递, 循环的第一遍就被借走了, 如果写成 &predicate 引用传递就没事, 每次循环只是借走了引用
        while let Some(c) = self.next_if(&predicate) {
            value.push(c);
        }
        Some(value).filter(|v| !v.is_empty())
    }

    /// 跳过空格
    fn consume_whitespace(&mut self) {
        self.next_while(|c| c.is_whitespace());
    }
}


impl<'a> Lexer<'a> {
    /// 扫描
    fn scan(&mut self) -> Option<Token> {
        self.consume_whitespace();
        None.or_else(|| self.scan_ident())
            .or_else(|| self.scan_number())
            .or_else(|| self.scan_operator())
            .or_else(|| self.scan_punctuation())
    }

    /// 标识符（变量名、函数名，例如 "pi"、"sin"、"x"）
    fn scan_ident(&mut self) -> Option<Token> {
        // 必须以字母开头，后面可以跟字母 / 下划线
        let mut name = self.next_if(|c| c.is_alphabetic())?.to_string();
        while let Some(c) = self.next_if(|c| c.is_alphabetic() || c == '_') {
            name.push(c);
        }
        Some(Token::Ident(name))
    }

    /// 数字, 123.45e+67 表示 123.45 × 10^67（123.45 乘以 10 的 67 次方）
    fn scan_number(&mut self) -> Option<Token> {
        // c.is_digit(10) 判断这个字符是不是 0-9 的十进制数字
        // c.is_digit(10) = c.is_ascii_digit()
        let mut name = self.next_while(|c| c.is_digit(10))?;
        if let Some(point) = self.next_if(|c| c == '.') {
            name.push(point);
            while let Some(num) = self.next_if(|c| c.is_digit(10)) {
                name.push(num);
            }
        }
        if let Some(e) = self.next_if(|c| c == 'e' || c == 'E') {
            name.push(e);
            if let Some(sign) = self.next_if(|c| c == '+' || c == '-') {
                name.push(sign);
            }
            while let Some(num) = self.next_if(|c| c.is_digit(10)) {
                name.push(num);
            }
        }
        Some(Token::Number(name))
    }

    /// 操作符
    fn scan_operator(&mut self) -> Option<Token> {
        self.next_if_token(|c| match c {
            '+' => Some(Token::Plus),
            '-' => Some(Token::Minus),
            '*' => Some(Token::Asterisk),
            '/' => Some(Token::Slash),
            '^' => Some(Token::Caret),
            '√' => Some(Token::SquareRoot),
            '%' => Some(Token::Percent),
            '!' => Some(Token::Exclamation),
            _ => None,
        })
    }

    fn scan_punctuation(&mut self) -> Option<Token> {
        self.next_if_token(|c| match c {
            '(' => Some(Token::OpenParen),
            ')' => Some(Token::CloseParen),
            ',' => Some(Token::Comma),
            _ => None,
        })
    }
}

impl<'a> Iterator for Lexer<'a> {
    type Item = Result<Token>;
    
    fn next(&mut self) -> Option<Self::Item> {
        self.consume_whitespace();
        self.scan().map(Ok).or_else(|| {
            // 要判断是从哪里开始报的错
            self.iter.peek().map(|c| Err(Error::Parse(format!("Unexpected character {}", c))))
        })
    }
    
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_peekable() {
        let input = "1+2";
        let mut lexer = Lexer::new(input);
        println!("lexer: {:?}", lexer);
        while let Some(next) = lexer.next() {
            println!("next: {:?}", next);
        }
    }
}
