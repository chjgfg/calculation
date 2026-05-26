use std::{f64, fmt::Display};

#[derive(Debug, Clone, PartialEq)]
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

// 先给 Constant 实现 Display --------------------------
impl Display for Constant {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Constant::E => write!(f, "e"),
            Constant::Pi => write!(f, "π"),
            Constant::Infinity => write!(f, "∞"),
            Constant::NaN => write!(f, "NaN"),
        }
    }
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

#[derive(Debug, Clone, PartialEq)]
pub enum Expression {
    /// 常量
    Constant(Constant),

    /// 返回数字
    Number(f64),

    /// 加法
    Add {
        lhs: Box<Expression>,
        rhs: Box<Expression>,
    },

    /// 减法
    Subtract {
        lhs: Box<Expression>,
        rhs: Box<Expression>,
    },

    /// 乘法
    Multiply {
        lhs: Box<Expression>,
        rhs: Box<Expression>,
    },

    /// 除法
    Divide {
        lhs: Box<Expression>,
        rhs: Box<Expression>,
    },

    /// 取模运算
    Modulo {
        lhs: Box<Expression>,
        rhs: Box<Expression>,
    },

    /// 取反
    Negate(Box<Expression>),

    /// sin()函数
    Sine(Box<Expression>),

    /// cos()函数
    Cosine(Box<Expression>),

    /// tan()函数
    Tangent(Box<Expression>),

    /// 幂运算
    Exponentiate {
        lhs: Box<Expression>,
        rhs: Box<Expression>,
    },

    /// 阶乘
    Factorial(Box<Expression>),

    /// 弧度转角度函数
    Degrees(Box<Expression>),

    /// 角度转换成弧度
    Radians(Box<Expression>),

    /// 四舍五入
    Round {
        value: Box<Expression>,
        decimals: Box<Expression>,
    },

    /// 开方
    SquareRoot(Box<Expression>),
}

impl Expression {
    pub fn evaluate(&self) -> f64 {
        let evaluate = match self {
            Expression::Constant(constant) => constant.into(),
            Expression::Number(number) => *number,
            Expression::Add { lhs, rhs } => lhs.evaluate() + rhs.evaluate(),
            Expression::Subtract { lhs, rhs } => lhs.evaluate() - rhs.evaluate(),
            Expression::Multiply { lhs, rhs } => lhs.evaluate() * rhs.evaluate(),
            Expression::Divide { lhs, rhs } => lhs.evaluate() / rhs.evaluate(),
            Expression::Modulo { lhs, rhs } => {
                let l = lhs.evaluate();
                let r = rhs.evaluate();
                ((l % r) + r) % r
            }
            Expression::Negate(expression) => -expression.evaluate(),
            Expression::Sine(expression) => expression.evaluate().sin(),
            Expression::Cosine(expression) => expression.evaluate().cos(),
            Expression::Tangent(expression) => expression.evaluate().tan(),
            Expression::Exponentiate { lhs, rhs } => lhs.evaluate().powf(rhs.evaluate()),
            Expression::Factorial(expression) => match expression.evaluate() {
                n if n == f64::INFINITY => n,
                n if n < 0.0 || n.fract() != 0.0 => f64::NAN,
                n => (1..=n.trunc() as i64).fold(1.0, |a, b| a * b as f64),
            },
            Expression::Degrees(expression) => expression.evaluate().to_degrees(),
            Expression::Radians(expression) => expression.evaluate().to_radians(),
            Expression::Round { value, decimals } => {
                let n = value.evaluate();
                let d = decimals.evaluate();
                if d < 0.0 || d.fract() != 0.0 {
                    return f64::NAN;
                };
                let scale = 10_f64.powf(d);
                (scale * n).round() / scale
            }
            Expression::SquareRoot(expression) => expression.evaluate().sqrt(),
        };
        println!("evaluate:{}", evaluate);
        evaluate
    }
}

impl From<Constant> for Expression {
    fn from(value: Constant) -> Self {
        Expression::Constant(value)
    }
}

impl From<f64> for Expression {
    fn from(value: f64) -> Self {
        Expression::Number(value)
    }
}

impl From<f64> for Box<Expression> {
    fn from(n: f64) -> Self {
        Box::new(Expression::Number(n))
    }
}

impl From<&Expression> for f64 {
    fn from(expr: &Expression) -> Self {
        expr.evaluate()
    }
}

// 再给 Expression 实现 Display --------------------------
impl Display for Expression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            // 数字
            Expression::Number(n) => write!(f, "{}", n),

            // 常量
            Expression::Constant(c) => write!(f, "{}", c),

            // 加法
            Expression::Add { lhs, rhs } => write!(f, "({} + {})", lhs, rhs),

            // 减法
            Expression::Subtract { lhs, rhs } => write!(f, "({} - {})", lhs, rhs),

            // 乘法
            Expression::Multiply { lhs, rhs } => write!(f, "({} * {})", lhs, rhs),

            // 除法
            Expression::Divide { lhs, rhs } => write!(f, "({} / {})", lhs, rhs),

            // 取模
            Expression::Modulo { lhs, rhs } => write!(f, "({} % {})", lhs, rhs),

            // 取反
            Expression::Negate(expr) => write!(f, "-{}", expr),

            // 幂运算
            Expression::Exponentiate { lhs, rhs } => write!(f, "({} ^ {})", lhs, rhs),

            // 阶乘
            Expression::Factorial(expr) => write!(f, "{}!", expr),

            // 开方
            Expression::SquareRoot(expr) => write!(f, "√{}", expr),

            // 三角函数
            Expression::Sine(expr) => write!(f, "sin({})", expr),
            Expression::Cosine(expr) => write!(f, "cos({})", expr),
            Expression::Tangent(expr) => write!(f, "tan({})", expr),

            // 角度/弧度
            Expression::Degrees(expr) => write!(f, "degrees({})", expr),
            Expression::Radians(expr) => write!(f, "radians({})", expr),

            // 四舍五入
            Expression::Round { value, decimals } => {
                write!(f, "round({}, {})", value, decimals)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use std::f64;

    use super::*;

    // 包装一下
    fn box_expression(expression: Expression) -> Box<Expression> {
        Box::new(expression)
    }

    #[test]
    fn test_from() {
        let e: Expression = Constant::E.into();
        assert_eq!(e, Expression::Constant(Constant::E));
        let n: Expression = 6.0.into();
        assert_eq!(n, Expression::Number(6.0));
        let bn: Box<Expression> = 6.0.into();
        assert_eq!(bn, Box::new(Expression::Number(6.0)));
        let f: f64= (&Expression::Number(6.0)).into();
        assert_eq!(f, 6.0);
    }

    #[test]
    fn test_constants() {
        assert_eq!(
            f64::from(&Expression::Constant(Constant::E)),
            f64::consts::E
        );
        assert!(f64::from(&Expression::Constant(Constant::Infinity)).is_infinite(),);
        assert!(f64::from(&Expression::Constant(Constant::NaN)).is_nan());
        assert_eq!(
            f64::from(&Expression::Constant(Constant::Pi)),
            f64::consts::PI
        );
    }

    #[test]
    fn test_basic_arithmetic() {
        let add = Expression::Add {
            lhs: box_expression(Expression::Number(2.0)),
            rhs: box_expression(Expression::Number(1.0)),
        };
        assert_eq!(add.evaluate(), 3.0);

        let sub = Expression::Subtract {
            lhs: box_expression(Expression::Number(2.0)),
            rhs: box_expression(Expression::Number(1.0)),
        };
        assert_eq!(sub.evaluate(), 1.0);

        let mul = Expression::Multiply {
            lhs: box_expression(Expression::Number(2.0)),
            rhs: box_expression(Expression::Number(1.0)),
        };
        assert_eq!(mul.evaluate(), 2.0);

        let div = Expression::Divide {
            lhs: box_expression(Expression::Number(2.0)),
            rhs: box_expression(Expression::Number(1.0)),
        };
        assert_eq!(div.evaluate(), 2.0);

        let modu = Expression::Modulo {
            lhs: box_expression(Expression::Number(3.0)),
            rhs: box_expression(Expression::Number(2.0)),
        };
        assert_eq!(modu.evaluate(), 1.0);

        let neg = Expression::Negate(box_expression(Expression::Number(2.0)));
        assert_eq!(neg.evaluate(), -2.0);
    }

    #[test]
    fn test_exponentiate() {
        let pow = Expression::Exponentiate {
            lhs: box_expression(Expression::Number(3.0)),
            rhs: box_expression(Expression::Number(2.0)),
        };
        assert_eq!(pow.evaluate(), 9.0);
    }

    #[test]
    fn test_factorial() {
        let fac = Expression::Factorial(box_expression(Expression::Number(5.0)));
        assert_eq!(fac.evaluate(), 120.0);
    }

    #[test]
    fn test_trigonometry() {
        let sin = Expression::Sine(box_expression(Expression::Number(f64::consts::PI / 6.0)));
        assert!((sin.evaluate() - 0.5).abs() < 1e-10); // 允许误差在 1e-10 以内
        let cos = Expression::Cosine(box_expression(Expression::Number(f64::consts::PI / 3.0)));
        assert!((cos.evaluate() - 0.5).abs() < 1e-10);
        let tan = Expression::Tangent(box_expression(Expression::Number(f64::consts::PI / 4.0)));
        assert!((tan.evaluate() - 1.0).abs() < 1e-10);
    }

    #[test]
    fn test_degrees_radians() {
        // 弧度转角度（π 弧度 = 180 度）
        let rad_to_deg = Expression::Degrees(box_expression(Expression::Constant(Constant::Pi)));
        assert_eq!(rad_to_deg.evaluate(), 180.0);

        // 角度转弧度（180 度 = π 弧度）
        let deg_to_rad = Expression::Radians(box_expression(Expression::Number(180.0)));
        assert!((deg_to_rad.evaluate() - f64::consts::PI).abs() < 1e-10);
    }

    #[test]
    fn test_square_root() {
        // 合法平方根
        let sqrt_16 = Expression::SquareRoot(box_expression(Expression::Number(16.0)));
        assert_eq!(sqrt_16.evaluate(), 4.0);

        // 负数平方根（返回 NaN）
        let sqrt_neg = Expression::SquareRoot(box_expression(Expression::Number(-4.0)));
        assert!(sqrt_neg.evaluate().is_nan());
    }

    #[test]
    fn test_round() {
        let round2 = Expression::Round {
            value: box_expression(Expression::Number(3.1415)),
            decimals: box_expression(Expression::Number(2.0)),
        };
        assert_eq!(round2.evaluate(), 3.14);
    }

    #[test]
    fn test_complex_expression() {
        // 复合表达式：(2 + 3) * (π / 2) → 5 * 1.5708 ≈ 7.85398
        let complex = Expression::Multiply {
            lhs: box_expression(Expression::Add {
                lhs: box_expression(Expression::Number(2.0)),
                rhs: box_expression(Expression::Number(3.0)),
            }),
            rhs: box_expression(Expression::Divide {
                lhs: box_expression(Expression::Constant(Constant::Pi)),
                rhs: box_expression(Expression::Number(2.0)),
            }),
        };
        assert!((complex.evaluate() - (5.0 * f64::consts::PI / 2.0)).abs() < 1e-10);
    }
}
