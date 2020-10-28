use crate::env::Env;
use crate::expr::Expr;
use crate::utils::*;

#[derive(Debug, Eq, PartialEq)]
pub struct Binding {
    name: String,
    val: Expr,
}

impl Binding {
    pub fn new(s: &str) -> (Self, &str) {
        let s = tag("let", s);
        let (_, s) = extract_whitespace(s);

        let (name, s) = extract_ident(s);
        let (_, s) = extract_whitespace(s);

        let s = tag("=", s);
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

    pub fn eval(&self, env: &mut Env) {
        env.store_binding(self.name.clone(), self.val.eval())
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
