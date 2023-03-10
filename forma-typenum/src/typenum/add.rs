use crate::typenum::*;
use std::ops::Add;

// Note: Don't use this trait now.
// pub trait TypeNumAdd<Lhs>: Add<Lhs>
// where
//     Lhs: NumTr,
//     <Self as Add<Lhs>>::Output: NumTr,
// {
//     type Output: NumTr,
// }

// Type Alias for easily type int implemented
pub type TSum<Lhs, Rhs> = <Lhs as Add<Rhs>>::Output;

// Add trait impl
impl<A: NumTr> Add<A> for NNil {
    type Output = A;
    fn add(self, rhs: A) -> Self::Output {
        rhs
    }
}

impl<LhsBitArr, B> Add<NNil> for NCons<LhsBitArr, B>
where
    LhsBitArr: NumTr,
    B: Bit,
{
    type Output = Self;
    fn add(self, _rhs: NNil) -> Self::Output {
        self
    }
}

impl<LhsBitArr, RhsBitArr> Add<NCons<RhsBitArr, B0>> for NCons<LhsBitArr, B0>
where
    LhsBitArr: NumTr + Add<RhsBitArr>,
    RhsBitArr: NumTr,
    TSum<LhsBitArr, RhsBitArr>: NumTr,
{
    type Output = NCons<TSum<LhsBitArr, RhsBitArr>, B0>;
    fn add(self, rhs: NCons<RhsBitArr, B0>) -> Self::Output {
        NCons {
            arr: self.arr + rhs.arr,
            _last: B0,
        }
    }
}

impl<LhsBitArr, RhsBitArr> Add<NCons<RhsBitArr, B0>> for NCons<LhsBitArr, B1>
where
    LhsBitArr: NumTr + Add<RhsBitArr>,
    RhsBitArr: NumTr,
    TSum<LhsBitArr, RhsBitArr>: NumTr,
{
    type Output = NCons<TSum<LhsBitArr, RhsBitArr>, B1>;
    fn add(self, rhs: NCons<RhsBitArr, B0>) -> Self::Output {
        NCons {
            arr: self.arr + rhs.arr,
            _last: B1,
        }
    }
}

impl<LhsBitArr, RhsBitArr> Add<NCons<RhsBitArr, B1>> for NCons<LhsBitArr, B0>
where
    LhsBitArr: NumTr + Add<RhsBitArr>,
    RhsBitArr: NumTr,
    TSum<LhsBitArr, RhsBitArr>: NumTr,
{
    type Output = NCons<TSum<LhsBitArr, RhsBitArr>, B1>;
    fn add(self, rhs: NCons<RhsBitArr, B1>) -> Self::Output {
        NCons {
            arr: self.arr + rhs.arr,
            _last: B1,
        }
    }
}

impl<LhsBitArr, RhsBitArr> Add<NCons<RhsBitArr, B1>> for NCons<LhsBitArr, B1>
where
    LhsBitArr: NumTr + Add<RhsBitArr>,
    RhsBitArr: NumTr,
    TSum<LhsBitArr, RhsBitArr>: NumTr + Add<One>,
    TSum<TSum<LhsBitArr, RhsBitArr>, One>: NumTr,
{
    type Output = NCons<TSum<TSum<LhsBitArr, RhsBitArr>, One>, B0>;
    fn add(self, rhs: NCons<RhsBitArr, B1>) -> Self::Output {
        NCons {
            arr: self.arr + rhs.arr + One::new(),
            _last: B0,
        }
    }
}

#[cfg(test)]
mod test {
    use crate::typenum::consts::*;
    use crate::typenum::*;

    #[test]
    fn compile_check_consts() {
        let n1 = N1::new();
        let n20 = N20::new();
        let _: N21 = n1 + n20;
        let _: N2 = N1::new() + N1::new();
        let _: N1 = N0::new() + N1::new();
        let _: N1 = N1::new() + N0::new();
        let _: N3 = N2::new() + N1::new();
    }
}
