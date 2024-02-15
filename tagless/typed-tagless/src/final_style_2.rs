use std::marker::PhantomData;

trait ExprSym {
    type Repr;

    fn lit(i: i32) -> Self::Repr;
    fn neg(r: Self::Repr) -> Self::Repr;
    fn add(r1: Self::Repr, r2: Self::Repr) -> Self::Repr;
}

struct Eval;

impl ExprSym for Eval {
    type Repr = i32;

    fn lit(i: i32) -> Self::Repr {
        i
    }

    fn neg(r: Self::Repr) -> Self::Repr {
        -r
    }

    fn add(r1: Self::Repr, r2: Self::Repr) -> Self::Repr {
        r1 + r2
    }
}

struct View;

impl ExprSym for View {
    type Repr = String;

    fn lit(i: i32) -> Self::Repr {
        i.to_string()
    }

    fn neg(r: Self::Repr) -> Self::Repr {
        format!("(-{})", r)
    }

    fn add(r1: Self::Repr, r2: Self::Repr) -> Self::Repr {
        format!("({}+{})", r1, r2)
    }
}

fn expr_eval(e: i32) -> i32 {
    e
}

fn expr_view(e: String) -> String {
    e
}

/// Mapping from E::Repr to E
trait HasExprSym {
    type ES: ExprSym;
}

// i32 -> Eval
impl HasExprSym for i32 {
    type ES = Eval;
}

// String -> Eval
impl HasExprSym for String {
    type ES = View;
}

fn tf1<E>() -> E::Repr
where
    E: ExprSym,
{
    E::add(E::lit(8), E::neg(E::add(E::lit(1), E::lit(2))))
}

// Introduce new type argument T = E::Repr, and wire everything up.
fn tf2<E: ExprSym<Repr = T>, T: HasExprSym<ES = E>>() -> T {
    E::add(E::lit(8), E::neg(E::add(E::lit(1), E::lit(2))))
}

//
// Adding new operations
//

trait MulExprSym: ExprSym {
    fn mul(r1: Self::Repr, r2: Self::Repr) -> Self::Repr;
}

impl MulExprSym for Eval {
    fn mul(r1: Self::Repr, r2: Self::Repr) -> Self::Repr {
        r1 * r2
    }
}

fn tf3<E: MulExprSym<Repr = T>, T: HasExprSym<ES = E>>() -> T {
    E::add(E::lit(7), E::neg(E::mul(E::lit(1), E::lit(2))))
}

enum Ctx {
    Pos,
    Neg,
}

// What's this business with the phantom data?
// (Should introduce the PushNeg without PhantomData first)
// It is only needed to implement HasExprSym.
struct CtxFunPh<TRepr, T>(Box<dyn Fn(&Ctx) -> TRepr>, PhantomData<T>);

impl<TRepr, T> CtxFunPh<TRepr, T> {
    fn new(f: impl Fn(&Ctx) -> TRepr + 'static) -> Self {
        CtxFunPh(Box::new(f), PhantomData)
    }
}

// PhantomData here to get around "unconstrained type parameter T" in trait impl.
struct PushNegPh<T>(PhantomData<T>);

impl<T: ExprSym + 'static> ExprSym for PushNegPh<T> {
    type Repr = CtxFunPh<T::Repr, T>;

    fn lit(i: i32) -> Self::Repr {
        CtxFunPh::new(move |ctx| match ctx {
            Ctx::Pos => T::lit(i),
            Ctx::Neg => T::neg(T::lit(i)),
        })
    }

    fn neg(r: Self::Repr) -> Self::Repr {
        CtxFunPh::new(move |ctx| match ctx {
            Ctx::Pos => r.0(&Ctx::Neg),
            Ctx::Neg => r.0(&Ctx::Pos),
        })
    }

    fn add(r1: Self::Repr, r2: Self::Repr) -> Self::Repr {
        CtxFunPh::new(move |ctx| T::add(r1.0(ctx), r2.0(ctx)))
    }
}

// Here I'd love to write just CtxFun<T::Repr>, but then the compiler complains
// T is not constrained. So we pass on the T into CtxFun as phantomdata.
impl<T: ExprSym + 'static> HasExprSym for CtxFunPh<T::Repr, T> {
    type ES = PushNegPh<T>;
}

fn exprsym_push_neg<S: ExprSym<Repr = T>, T: HasExprSym<ES = S>>(e: &CtxFunPh<T, S>) -> T {
    e.0(&Ctx::Pos)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn tf1<E: ExprSym<Repr = T>, T: HasExprSym<ES = E>>() -> T {
        E::add(E::lit(8), E::neg(E::add(E::lit(1), E::lit(2))))
    }

    fn tf1_pre<E: ExprSym>() -> E::Repr {
        E::add(E::lit(8), E::neg(E::add(E::lit(1), E::lit(2))))
    }

    #[test]
    fn eval_expr() {
        // Does not compile
        // expr_eval(tf1());

        let e = expr_eval(tf2());
        assert_eq!(e, 5);
    }

    #[test]
    fn view_expr() {
        // Does not compile
        // expr_view(tf1());

        let s = expr_view(tf2());
        assert_eq!(s, "(8+(-(1+2)))");
    }

    #[test]
    fn eval_expr_3() {
        let e = expr_eval(tf3());
        assert_eq!(e, 5);
    }

    #[test]
    fn push_neg_equal() {
        // let final_style = exprsym_push_neg(&tf1());

        // let r = tf1_pre::<PushNeg<View>>();
        // dbg!(r.0(&Ctx::Pos));
    }
}
