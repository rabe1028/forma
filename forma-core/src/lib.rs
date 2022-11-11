pub mod hlist;

pub trait Evaluatable {
    type Result;
    fn eval() -> Self::Result;
}

trait BoolTr {}

pub struct TrueT;
pub struct FalseT;

impl BoolTr for TrueT {}
impl BoolTr for FalseT {}
