use crate::{expression::Expression, token::Token};

/// 左结合
const ASSOC_LEFT: u8 = 1;
/// 右结合
const ASSOC_RIGHT: u8 = 0;

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
    /// 负号
    Minus,
    /// 前缀的加号
    Plus,
    /// 开方
    SquareRoot,
}

impl PrefixOperator {
    fn build(&self, operand: Expression) -> Expression {
        match self {
            PrefixOperator::Minus => Expression::Negate(operand.into()),
            PrefixOperator::Plus => operand,
            PrefixOperator::SquareRoot => Expression::SquareRoot(operand.into()),
        }
    }
}

impl Operator for PrefixOperator {
    fn from(token: &Token) -> Option<Self> {
        match token {
            Token::Minus => Some(Self::Minus),
            Token::Plus => Some(Self::Plus),
            Token::SquareRoot => Some(Self::SquareRoot),
            _ => None,
        }
    }

    /// 右结合, 从右往左算
    fn associativity(&self) -> u8 {
        ASSOC_RIGHT
    }

    /// 优先级 5
    fn precedence(&self) -> u8 {
        5
    }
}

/// 中缀表达式
pub enum InfixOperator {
    /// 加号
    Add,
    /// 减号
    Subtract,
    /// 乘号
    Multiply,
    /// 除号
    Divide,
    /// 幂运算
    Exponentiate,
    /// 取模
    Modulo,
}

impl InfixOperator {
    pub fn build(&self, lhs: Expression, rhs: Expression) -> Expression {
        match self {
            InfixOperator::Add => Expression::Add {
                lhs: lhs.into(),
                rhs: rhs.into(),
            },
            InfixOperator::Subtract => Expression::Subtract {
                lhs: lhs.into(),
                rhs: rhs.into(),
            },
            InfixOperator::Multiply => Expression::Multiply {
                lhs: lhs.into(),
                rhs: rhs.into(),
            },
            InfixOperator::Divide => Expression::Divide {
                lhs: lhs.into(),
                rhs: rhs.into(),
            },
            InfixOperator::Exponentiate => Expression::Exponentiate {
                lhs: lhs.into(),
                rhs: rhs.into(),
            },
            InfixOperator::Modulo => Expression::Modulo {
                lhs: lhs.into(),
                rhs: rhs.into(),
            },
        }
    }
}

impl Operator for InfixOperator {
    fn from(token: &Token) -> Option<Self> {
        match token {
            Token::Plus => Some(Self::Add),
            Token::Minus => Some(Self::Subtract),
            Token::Asterisk => Some(Self::Multiply),
            Token::Slash => Some(Self::Divide),
            Token::Caret => Some(Self::Exponentiate),
            Token::Percent => Some(Self::Modulo),
            _ => None,
        }
    }

    fn associativity(&self) -> u8 {
        match self {
            Self::Exponentiate => ASSOC_RIGHT, // 幂运算是右结合
            _ => ASSOC_LEFT,                   // 其他都是左结合
        }
    }

    fn precedence(&self) -> u8 {
        match self {
            Self::Add | Self::Subtract => 1,
            Self::Multiply | Self::Divide | Self::Modulo => 2,
            Self::Exponentiate => 3,
        }
    }
}

/// 后缀表达式
pub enum PostfixOperator {
    Factorial,
}

impl PostfixOperator {
    pub fn build(&self, operand: Expression) -> Expression {
        match self {
            PostfixOperator::Factorial => Expression::Factorial(operand.into()),
        }
    }
}

impl Operator for PostfixOperator {
    fn from(token: &Token) -> Option<Self> {
        match token {
            Token::Exclamation => Some(Self::Factorial),
            _ => None,
        }
    }

    fn associativity(&self) -> u8 {
        ASSOC_LEFT
    }

    fn precedence(&self) -> u8 {
        4
    }
}
