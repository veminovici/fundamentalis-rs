enum Expr {
    Lit(i32),                  // integer literal
    Neg(Box<Expr>),            // negation
    Add(Box<Expr>, Box<Expr>), // addition
}

impl Expr {
    fn lit(i: i32) -> Self {
        Expr::Lit(i)
    }

    fn neg(r: Self) -> Self {
        Expr::Neg(Box::new(r))
    }

    fn add(r1: Self, r2: Self) -> Self {
        Expr::Add(Box::new(r1), Box::new(r2))
    }

    fn eval(&self) -> i32 {
        match self {
            Expr::Lit(i) => *i,
            Expr::Neg(r) => -r.eval(),
            Expr::Add(r1, r2) => r1.eval() + r2.eval(),
        }
    }

    fn view(&self) -> String {
        match self {
            Expr::Lit(i) => i.to_string(),
            Expr::Neg(r) => format!("(-{})", r.view()),
            Expr::Add(r1, r2) => format!("({}+{})", r1.view(), r2.view()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn eval_expr() {
        let e = Expr::add(
            Expr::lit(8),
            Expr::neg(Expr::add(Expr::lit(1), Expr::lit(2))),
        );

        let r = e.eval();
        assert_eq!(r, 5);
    }

    #[test]
    fn view_expr() {
        let e = Expr::add(
            Expr::lit(8),
            Expr::neg(Expr::add(Expr::lit(1), Expr::lit(2))),
        );

        let s = e.view();
        assert_eq!(s, "(8+(-(1+2)))");
    }
}
