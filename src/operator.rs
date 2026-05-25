use crate::token::Token;

pub trait Operator: Sized {
    /// 创建Token
    fn from(token: &Token) -> Option<Self>;
    /// 返回结合性
    fn associativity(&self) -> u8;
    /// 返回优先级
    fn precedence(&self) -> u8;
}

/// 前缀表达式
enum PrefixOperator {
    Minus,
    Plus,
    SquareRoot,
}

impl PrefixOperator {
    // fn build(&self, operand: Expression) -> Expression {
        
    // }
}