type Repr = i32;

fn lit(i: i32) -> Repr {
    i
}

fn neg(r: Repr) -> Repr {
    -r
}

fn add(r1: Repr, r2: Repr) -> Repr {
    r1 + r2
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn eval_expr() {
        let e = add(lit(8), neg(add(lit(1), lit(2))));
        assert_eq!(e, 5)
    }
}
