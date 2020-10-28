use crate::utils::*;
use crate::val::Val;

#[derive(Clone, Debug, Ord, PartialOrd, Eq, PartialEq)]
pub struct Number(pub i32);

impl Number {
    pub fn new(s: &str) -> Result<(Self, &str), String> {
        let (num, s) = extract_digits(s)?;
        Ok((Self(num.parse().unwrap()), s))
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Op {
    Add,
    Sub,
    Mul,
    Div,
}

impl Op {
    pub fn new(s: &str) -> Result<(Self, &str), String> {
        tag("+", s)
            .map(|s| (Self::Add, s))
            .or_else(|_| tag("-", s).map(|s| (Self::Sub, s)))
            .or_else(|_| tag("*", s).map(|s| (Self::Mul, s)))
            .or_else(|_| tag("/", s).map(|s| (Self::Div, s)))
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Expr {
    Number(Number),
    Operation { lhs: Number, rhs: Number, op: Op },
}

impl Expr {
    pub fn new(s: &str) -> Result<(Self, &str), String> {
        Self::new_operation(s).or_else(|_| Self::new_number(s))
    }

    pub fn new_number(s: &str) -> Result<(Self, &str), String> {
        Number::new(s).map(|(num, s)| (Self::Number(num), s))
    }

    pub fn new_operation(s: &str) -> Result<(Self, &str), String> {
        let (lhs, s) = Number::new(s)?;
        let (_, s) = extract_whitespace(s);

        let (op, s) = Op::new(s)?;
        let (_, s) = extract_whitespace(s);

        let (rhs, s) = Number::new(s)?;

        Ok((Self::Operation { lhs, rhs, op }, s))
    }

    pub fn eval(&self) -> Val {
        match self {
            Self::Number(Number(n)) => Val::Number(*n),
            Self::Operation { rhs, lhs, op } => {
                let Number(lhs) = lhs;
                let Number(rhs) = rhs;

                let result = match op {
                    Op::Add => lhs + rhs,
                    Op::Sub => lhs - rhs,
                    Op::Mul => lhs * rhs,
                    Op::Div => lhs / rhs,
                };

                Val::Number(result)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_number() {
        assert_eq!(Number::new("123"), Ok((Number(123), "")));
    }

    #[test]
    fn parse_add_op() {
        assert_eq!(Op::new("+"), Ok((Op::Add, "")));
    }

    #[test]
    fn parse_sub_op() {
        assert_eq!(Op::new("-"), Ok((Op::Sub, "")));
    }

    #[test]
    fn parse_mul_op() {
        assert_eq!(Op::new("*"), Ok((Op::Mul, "")));
    }

    #[test]
    fn parse_div_op() {
        assert_eq!(Op::new("/"), Ok((Op::Div, "")));
    }

    #[test]
    fn parse_one_plus_two() {
        assert_eq!(
            Expr::new("1+2"),
            Ok((
                Expr::Operation {
                    lhs: Number(1),
                    rhs: Number(2),
                    op: Op::Add,
                },
                "",
            )),
        );
    }

    #[test]
    fn parse_expr_with_whitespace() {
        assert_eq!(
            Expr::new("2 * 2"),
            Ok((
                Expr::Operation {
                    lhs: Number(2),
                    rhs: Number(2),
                    op: Op::Mul,
                },
                "",
            )),
        );
    }

    #[test]
    fn parse_number_as_expr() {
        assert_eq!(Expr::new("456"), Ok((Expr::Number(Number(456)), "")));
    }

    #[test]
    fn eval_add() {
        assert_eq!(
            Expr::Operation {
                lhs: Number(10),
                rhs: Number(10),
                op: Op::Add,
            }
            .eval(),
            Val::Number(20),
        );
    }

    #[test]
    fn eval_sub() {
        assert_eq!(
            Expr::Operation {
                lhs: Number(1),
                rhs: Number(5),
                op: Op::Sub,
            }
            .eval(),
            Val::Number(-4),
        );
    }

    #[test]
    fn eval_mul() {
        assert_eq!(
            Expr::Operation {
                lhs: Number(5),
                rhs: Number(6),
                op: Op::Mul,
            }
            .eval(),
            Val::Number(30),
        );
    }

    #[test]
    fn eval_div() {
        assert_eq!(
            Expr::Operation {
                lhs: Number(200),
                rhs: Number(20),
                op: Op::Div,
            }
            .eval(),
            Val::Number(10),
        );
    }
}
