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
    _last: B,
}

impl<BitArr: NumTr, B: Bit> NumTr for NCons<BitArr, B> {
    const BIT_LENGTH: usize = BitArr::BIT_LENGTH + 1;
    fn new() -> Self {
        NCons {
            arr: BitArr::new(),
            _last: B::new(),
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

impl Shrinkable for NCons<NNil, B1> {
    type Shrinked = Self;
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
type Zero = NCons<NNil, B0>;
#[allow(dead_code)]
type One = NCons<NNil, B1>;

pub mod add;
pub use add::*;

pub mod sub;
pub use sub::*;

pub mod mul;
pub use mul::*;

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

pub type BitsOf<N> = <N as TypedBitLength>::BitLength;

impl<BitArr> TypedBitLength for NCons<BitArr, B0>
where
    BitArr: NumTr + TypedBitLength,
    BitsOf<BitArr>: Add<consts::N1>,
    TSum<BitsOf<BitArr>, consts::N1>: NumTr,
{
    type BitLength = TSum<BitsOf<BitArr>, consts::N1>;
}

impl<BitArr> TypedBitLength for NCons<BitArr, B1>
where
    BitArr: NumTr + TypedBitLength,
    BitsOf<BitArr>: Add<consts::N1>,
    TSum<BitsOf<BitArr>, consts::N1>: NumTr,
{
    type BitLength = TSum<BitsOf<BitArr>, consts::N1>;
}

pub trait ToUsize: TypedBitLength {
    const OUTPUT: usize;
}

// code generation by build/main.rs
include!(env!("TYPENUM_BUILD_CONSTS_TYPENUM_IMPLS"));

// The following code creates implementation conflicts
// macro_rules! impl_to_usize {
//     ($b_len: ty) => {
//         impl<BitArr> ToUsize for NCons<BitArr, B0>
//         where
//             BitArr: NumTr + ToUsize,
//             Self: TypedBitLength<BitLength = $b_len>,
//         {
//             const OUTPUT: usize = <BitArr as ToUsize>::OUTPUT << 1;
//         }

//         // impl<BitArr> ToUsize for NCons<BitArr, B0>
//         // where
//         //     BitArr: NumTr + ToUsize,
//         //     Self: TypedBitLength<BitLength = $b_len>,
//         // {
//         //     const OUTPUT: usize = <BitArr as ToUsize>::OUTPUT << 1 + 1;
//         // }
//     };
// }
// impl_to_usize!(consts::N1);
// impl_to_usize!(consts::N2);

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn compile_check() {
        let a: NCons<NNil, B0> = Zero::new();
        let b: NCons<NNil, B1> = One::new();
        let _: NCons<NNil, B1> = a + b;
    }
}
