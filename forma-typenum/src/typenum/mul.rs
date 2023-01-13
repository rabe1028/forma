use crate::typenum::*;
use std::ops::Mul;

pub type TProd<Lhs, Rhs> = <Lhs as Mul<Rhs>>::Output;

impl<A: NumTr> Mul<A> for NNil {
    type Output = NNil;
    fn mul(self, _rhs: A) -> Self::Output {
        self
    }
}

impl<LhsBitArr, B> Mul<NNil> for NCons<LhsBitArr, B>
where
    LhsBitArr: NumTr,
    B: Bit,
{
    type Output = NNil;
    fn mul(self, _rhs: NNil) -> Self::Output {
        NNil
    }
}

impl<LhsBitArr, RhsBitArr, B> Mul<NCons<RhsBitArr, B>> for NCons<LhsBitArr, B0>
where
    B: Bit,
    LhsBitArr: NumTr + Mul<NCons<RhsBitArr, B>>,
    RhsBitArr: NumTr,
    TProd<LhsBitArr, NCons<RhsBitArr, B>>: NumTr + Shrinkable,
{
    type Output = NCons<Shrink<TProd<LhsBitArr, NCons<RhsBitArr, B>>>, B0>;
    fn mul(self, _rhs: NCons<RhsBitArr, B>) -> Self::Output {
        Self::Output::new()
    }
}

impl<LhsBitArr, RhsBitArr, B> Mul<NCons<RhsBitArr, B>> for NCons<LhsBitArr, B1>
where
    B: Bit,
    LhsBitArr: NumTr + Mul<NCons<RhsBitArr, B>>,
    RhsBitArr: NumTr,
    TProd<LhsBitArr, NCons<RhsBitArr, B>>: NumTr + Shrinkable,
    NCons<Shrink<TProd<LhsBitArr, NCons<RhsBitArr, B>>>, B0>: NumTr + Add<NCons<RhsBitArr, B>>,
    TSum<NCons<Shrink<TProd<LhsBitArr, NCons<RhsBitArr, B>>>, B0>, NCons<RhsBitArr, B>>: NumTr,
{
    type Output =
        TSum<NCons<Shrink<TProd<LhsBitArr, NCons<RhsBitArr, B>>>, B0>, NCons<RhsBitArr, B>>;
    fn mul(self, _rhs: NCons<RhsBitArr, B>) -> Self::Output {
        Self::Output::new()
    }
}

#[cfg(test)]
mod test {
    use crate::typenum::consts::*;
    use crate::typenum::*;

    #[test]
    fn compile_check_consts() {
        let _: N0 = N1::new() * N0::new();
        let _: N0 = N2::new() * N0::new();
        let _: N0 = N0::new() * N1::new();
        let _: N0 = N0::new() * N2::new();
        let _: N1 = N1::new() * N1::new();
        let _: N2 = N2::new() * N1::new();
        let _: N31 = N31::new() * N1::new();
        let _: N32 = N8::new() * N4::new();
        let _: N27 = N9::new() * N3::new();
    }
}
