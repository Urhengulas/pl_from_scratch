use crate::expr::Expr;
use crate::utils::*;

#[derive(Debug, Eq, PartialEq)]
pub struct Binding {
    name: String,
    val: Expr,
}

impl Binding {
    pub fn new(s: &str) -> (Self, &str) {
        let s = match s.strip_prefix("let") {
            Some(s) => s,
            None => panic!("Expected 'let'"),
        };
        let (_, s) = extract_whitespace(s);

        let (name, s) = extract_ident(s);
        let (_, s) = extract_whitespace(s);

        let s = match s.strip_prefix("=") {
            Some(s) => s,
            None => panic!("Expected '="),
        };
        let (_, s) = extract_whitespace(s);

        let (val, s) = Expr::new(s);

        (
            Self {
                name: name.to_string(),
                val,
            },
            s,
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::expr::{Number, Op};

    #[test]
    fn parse_binding() {
        assert_eq!(
            Binding::new("let a = 10 / 2"),
            (
                Binding {
                    name: "a".to_string(),
                    val: Expr {
                        lhs: Number(10),
                        rhs: Number(2),
                        op: Op::Div
                    }
                },
                ""
            )
        )
    }
}
