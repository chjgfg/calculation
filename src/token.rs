use std::fmt::{self, Display};

/// 词法分析器生成的 token 类型
/// 代表表达式中的一个最小语法单元（数字、标识符、运算符、括号等）
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Token {
    /// 数字（整数或小数，例如 "123"、"3.14"）
    Number(String),
    /// 标识符（变量名、函数名，例如 "pi"、"sin"、"x"）
    Ident(String),
    /// 加号 `+`
    Plus,
    /// 减号 `-`（可作为中缀减法或前缀负号）
    Minus,
    /// 乘号 `*`
    Asterisk,
    /// 除号 `/`
    Slash,
    /// 幂运算符 `^`
    Caret,
    /// 平方根符号 `√`
    SquareRoot,
    /// 取模运算符 `%`
    Percent,
    /// 阶乘运算符 `!`
    Exclamation,
    /// 左括号 `(`
    OpenParen,
    /// 右括号 `)`
    CloseParen,
    /// 参数分隔符逗号 `,`
    Comma,
}

impl Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let token = match self {
            Token::Number(n) => n,
            Token::Ident(s) => s,
            Token::Plus => "+",
            Token::Minus => "-",
            Token::Asterisk => "*",
            Token::Slash => "/",
            Token::Caret => "^",
            Token::SquareRoot => "√",
            Token::Percent => "%",
            Token::Exclamation => "!",
            Token::OpenParen => "(",
            Token::CloseParen => ")",
            Token::Comma => ",",
        };
        f.write_str(token)
    }
}
