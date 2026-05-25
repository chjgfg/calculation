use std::f64;

#[derive(Debug, Clone)]
pub enum Constant {
    /// 自然常数，数学符号 e，值约等于 2.71828e
    E,
    /// IEEE 754 浮点数标准里的特殊值, 无穷大
    Infinity,
    /// IEEE 754 浮点数标准里的特殊值, 没有数字
    NaN,
    /// π
    Pi,
}

impl From<&Constant> for f64 {
    fn from(value: &Constant) -> Self {
        match value {
            Constant::E => f64::consts::E,
            Constant::Infinity => f64::INFINITY,
            Constant::NaN => f64::NAN,
            Constant::Pi => f64::consts::PI,
        }
    }
}

#[derive(Debug, Clone)]
pub enum Expression {
    /// 加两个值
    Add {
        lhs: Box<Expression>,
        rhs: Box<Expression>,
    },

    /// 常量
    Constant(Constant),

    /// cos() 函数
    Cosine(Box<Expression>),

    /// 弧度转角度函数
    Degree(Box<Expression>),

    Divide{
         lhs: Box<Expression>,
        rhs: Box<Expression>,
    }

}
