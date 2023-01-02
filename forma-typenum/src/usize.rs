use std::ops::Add;

mod consts {
    include!(env!("TYPENUM_BUILD_CONSTS_USIZE"));
}

#[cfg(feature = "experimental")]
#[const_trait]
pub trait UsizeTr {
    fn value(&self) -> usize;
}
#[cfg(not(feature = "experimental"))]
pub trait UsizeTr {
    fn value(&self) -> usize;
}

#[cfg(feature = "experimental")]
pub struct UsizeS<const X: usize>;
pub struct UsizeD(usize);

#[cfg(feature = "experimental")]
impl<const X: usize> const UsizeTr for UsizeS<X> {
    fn value(&self) -> usize {
        X
    }
}

impl UsizeTr for UsizeD {
    fn value(&self) -> usize {
        self.0
    }
}

// this code is experimental!!!
#[cfg(feature = "experimental")]
impl<const X: usize, const Y: usize> Add<UsizeS<Y>> for UsizeS<X>
where
    [(); X + Y]:,
{
    type Output = UsizeS<{ X + Y }>;
    fn add(self, _rhs: UsizeS<Y>) -> Self::Output {
        UsizeS::<{ X + Y }>
    }
}

pub trait ToTypeNum {
    type Output;
}

pub trait FromTypeNum {
    type Output;
}

#[cfg(test)]
mod test {
    use super::consts::*;
    use super::*;

    #[test]
    #[cfg(feature = "experimental")]
    fn add_check() {
        let a = UsizeS::<1>;
        let b = UsizeS::<2>;
        let _: UsizeS<3> = a + b;

        // can't compile
        // let c = UsizeS::<{ usize::MAX }>;
        // let _ = c + a;
    }
}
