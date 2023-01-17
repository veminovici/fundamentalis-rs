#![feature(try_trait_v2)]

use std::ops::{Try, FromResidual, ControlFlow};
use std::num::NonZeroI32;

#[derive(Debug)]
struct ResultCode(i32);

impl ResultCode {
    const SUCCESS: Self  = Self(0);
}

struct ResultCodeResidual(NonZeroI32);

impl Try for ResultCode {
    type Output = ();
    type Residual = ResultCodeResidual;

    fn from_output(_output: Self::Output) -> Self {
        Self::SUCCESS
    }

    fn branch(self) -> ControlFlow<Self::Residual, Self::Output> {
        match NonZeroI32::new(self.0) {
            Some(r) => ControlFlow::Break(ResultCodeResidual(r)),
            None => ControlFlow::Continue(()),
        }
    }
}

impl FromResidual<ResultCodeResidual> for ResultCode {
    fn from_residual(residual: ResultCodeResidual) -> Self {
        ResultCode(residual.0.into())
    }
}

fn test() -> ResultCode {
    let _x = ResultCode(10)?;

    ResultCode::SUCCESS
}