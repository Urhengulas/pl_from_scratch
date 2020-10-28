mod utils;

use utils::*;

#[derive(Clone, Debug, Ord, PartialOrd, Eq, PartialEq)]
pub struct Number(pub i32);

impl Number {
    pub fn new(s: &str) -> (Self, &str) {
        let (num, s) = extract_digits(s);
        (Self(num.parse().unwrap()), s)
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
    pub fn new(s: &str) -> (Self, &str) {
        let (op, s) = extract_op(s);

        let op = match op {
            "+" => Self::Add,
            "-" => Self::Sub,
            "*" => Self::Mul,
            "/" => Self::Div,
            _ => panic!("Couldn't parse operator: {}", s),
        };

        (op, s)
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Expr {
    pub lhs: Number,
    pub rhs: Number,
    pub op: Op,
}

impl Expr {
    pub fn new(s: &str) -> (Self, &str) {
        let (lhs, s) = Number::new(s);
        let (_, s) = extract_whitespace(s);

        let (op, s) = Op::new(s);
        let (_, s) = extract_whitespace(s);

        let (rhs, s) = Number::new(s);

        (Self { lhs, rhs, op }, s)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_number() {
        assert_eq!(Number::new("123"), (Number(123), ""));
    }

    #[test]
    fn parse_add_op() {
        assert_eq!(Op::new("+"), (Op::Add, ""));
    }

    #[test]
    fn parse_sub_op() {
        assert_eq!(Op::new("-"), (Op::Sub, ""));
    }

    #[test]
    fn parse_mul_op() {
        assert_eq!(Op::new("*"), (Op::Mul, ""));
    }

    #[test]
    fn parse_div_op() {
        assert_eq!(Op::new("/"), (Op::Div, ""));
    }

    #[test]
    fn parse_one_plus_two() {
        assert_eq!(
            Expr::new("1+2"),
            (
                Expr {
                    lhs: Number(1),
                    rhs: Number(2),
                    op: Op::Add
                },
                ""
            )
        )
    }

    #[test]
    fn parse_expr_with_whitespace() {
        assert_eq!(
            Expr::new("2 * 2"),
            (
                Expr {
                    lhs: Number(2),
                    rhs: Number(2),
                    op: Op::Mul,
                },
                ""
            ),
        );
    }
}
