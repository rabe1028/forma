use forma_core::Evaluatable;
use std::ops::Add;

pub trait Bit: Evaluatable<Result = bool> {
    fn new() -> Self;
}

pub struct B0;
pub struct B1;

impl Evaluatable for B0 {
    type Result = bool;
    fn eval() -> Self::Result {
        false
    }
}
impl Bit for B0 {
    fn new() -> Self {
        B0
    }
}

impl Evaluatable for B1 {
    type Result = bool;
    fn eval() -> Self::Result {
        true
    }
}

impl Bit for B1 {
    fn new() -> Self {
        B1
    }
}

pub trait NumTr {
    const BIT_LENGTH: usize;
    fn new() -> Self;
}

pub struct NNil;
impl NumTr for NNil {
    const BIT_LENGTH: usize = 0;
    fn new() -> Self {
        NNil {}
    }
}

pub struct NCons<BitArr: NumTr, B: Bit> {
    arr: BitArr,
    last: B,
}

impl<BitArr: NumTr, B: Bit> NumTr for NCons<BitArr, B> {
    const BIT_LENGTH: usize = BitArr::BIT_LENGTH + 1;
    fn new() -> Self {
        NCons {
            arr: BitArr::new(),
            last: B::new(),
        }
    }
}

// Helper trait
pub trait Shrinkable {
    type Shrinked: NumTr;
}

impl Shrinkable for NNil {
    type Shrinked = NNil;
}

impl Shrinkable for NCons<NNil, B0> {
    type Shrinked = NNil;
}

impl<BitArr, B, BB> Shrinkable for NCons<NCons<BitArr, B>, BB>
where
    B: Bit,
    BB: Bit,
    BitArr: NumTr + Shrinkable,
{
    type Shrinked = NCons<NCons<<BitArr as Shrinkable>::Shrinked, B>, BB>;
}

pub type Shrink<A> = <A as Shrinkable>::Shrinked;

#[allow(dead_code)]
type ZERO = NCons<NNil, B0>;
#[allow(dead_code)]
type ONE = NCons<NNil, B1>;

pub mod add;
pub use add::*;

pub mod sub;
pub use sub::*;

mod consts {
    include!(env!("TYPENUM_BUILD_CONSTS_TYPENUM"));
}

// bit length with TypeNum for NumTr
pub trait TypedBitLength: NumTr {
    type BitLength: NumTr;
}

impl TypedBitLength for NNil {
    type BitLength = consts::N0;
}

#[allow(dead_code)]
pub type BitsOf<N> = <N as TypedBitLength>::BitLength;

impl<BitArr> TypedBitLength for NCons<BitArr, B0>
where
    BitArr: NumTr + TypedBitLength,
    BitsOf<BitArr>: Add<consts::N1>,
    TSum<BitsOf<BitArr>, consts::N1>: NumTr,
{
    type BitLength = TSum<BitsOf<BitArr>, consts::N1>;
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn compile_check() {
        let a: NCons<NNil, B0> = ZERO::new();
        let b: NCons<NNil, B1> = ONE::new();
        let _: NCons<NNil, B1> = a + b;
    }
}