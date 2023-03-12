use crate::typenum::*;
use std::ops::Sub;

// Type Alias for easily typeint implemented
pub type TDiff<Lhs, Rhs> = <Lhs as Sub<Rhs>>::Output;

// Difference trait impl
impl Sub<NNil> for NNil {
    type Output = NNil;
    fn sub(self, _rhs: NNil) -> Self::Output {
        self
    }
}

impl<BitArr> Sub<NNil> for NCons<BitArr, B0>
where
    BitArr: NumTr,
{
    type Output = Self;
    fn sub(self, _rhs: NNil) -> Self::Output {
        self
    }
}

impl<BitArr> Sub<NNil> for NCons<BitArr, B1>
where
    BitArr: NumTr,
{
    type Output = Self;
    fn sub(self, _rhs: NNil) -> Self::Output {
        self
    }
}

impl<LhsBitArr, RhsBitArr> Sub<NCons<RhsBitArr, B0>> for NCons<LhsBitArr, B0>
where
    LhsBitArr: NumTr + Sub<RhsBitArr>,
    RhsBitArr: NumTr,
    TDiff<LhsBitArr, RhsBitArr>: NumTr + Shrinkable,
{
    type Output = NCons<Shrink<TDiff<LhsBitArr, RhsBitArr>>, B0>;
    fn sub(self, _rhs: NCons<RhsBitArr, B0>) -> Self::Output {
        NCons {
            arr: Shrink::<TDiff<LhsBitArr, RhsBitArr>>::new(),
            _last: B0,
        }
    }
}

impl<LhsBitArr, RhsBitArr> Sub<NCons<RhsBitArr, B0>> for NCons<LhsBitArr, B1>
where
    LhsBitArr: NumTr + Sub<RhsBitArr>,
    RhsBitArr: NumTr,
    TDiff<LhsBitArr, RhsBitArr>: NumTr + Shrinkable,
{
    type Output = NCons<Shrink<TDiff<LhsBitArr, RhsBitArr>>, B1>;
    fn sub(self, _rhs: NCons<RhsBitArr, B0>) -> Self::Output {
        NCons {
            arr: Shrink::<TDiff<LhsBitArr, RhsBitArr>>::new(),
            _last: B1,
        }
    }
}

impl<LhsBitArr, RhsBitArr> Sub<NCons<RhsBitArr, B1>> for NCons<LhsBitArr, B0>
where
    LhsBitArr: NumTr + Sub<TSum<RhsBitArr, One>>,
    RhsBitArr: NumTr + Add<One>,
    TSum<RhsBitArr, One>: NumTr,
    TDiff<LhsBitArr, TSum<RhsBitArr, One>>: NumTr + Shrinkable,
{
    type Output = NCons<Shrink<TDiff<LhsBitArr, TSum<RhsBitArr, One>>>, B1>;
    fn sub(self, _rhs: NCons<RhsBitArr, B1>) -> Self::Output {
        NCons {
            arr: Shrink::<TDiff<LhsBitArr, TSum<RhsBitArr, One>>>::new(),
            _last: B1,
        }
    }
}

impl<LhsBitArr, RhsBitArr> Sub<NCons<RhsBitArr, B1>> for NCons<LhsBitArr, B1>
where
    LhsBitArr: NumTr + Sub<RhsBitArr>,
    RhsBitArr: NumTr,
    TDiff<LhsBitArr, RhsBitArr>: NumTr + Shrinkable,
{
    type Output = NCons<Shrink<TDiff<LhsBitArr, RhsBitArr>>, B0>;
    fn sub(self, _rhs: NCons<RhsBitArr, B1>) -> Self::Output {
        NCons {
            arr: Shrink::<TDiff<LhsBitArr, RhsBitArr>>::new(),
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
        let n2 = N2::new();
        let n1 = N1::new();
        let _: N1 = n2 - n1;

        let n2_1 = N2::new();
        let n2_2 = N2::new();
        let _: N0 = n2_1 - n2_2;
    }

    // #[test]
    // fn cannot_compile() {
    //     let n2 = N2::new();
    //     let n1 = N1::new();
    //     let _ = n1 - n2;
    // }
}
