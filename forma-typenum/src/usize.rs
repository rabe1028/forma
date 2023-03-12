use std::ops::Add;
use std::{marker::PhantomData, ops::Sub};

use crate::typenum::{TDiff, TSum, ToUsize};

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
#[cfg(not(feature = "experimental"))]
pub struct UsizeS<X: ToUsize> {
    _phantom: PhantomData<X>,
}

impl<X: ToUsize> UsizeS<X> {
    pub const fn new() -> Self {
        Self {
            _phantom: PhantomData,
        }
    }
}

pub struct UsizeD(usize);

#[cfg(feature = "experimental")]
impl<const X: usize> const UsizeTr for UsizeS<X> {
    fn value(&self) -> usize {
        X
    }
}

impl<X: ToUsize> UsizeTr for UsizeS<X> {
    fn value(&self) -> usize {
        X::OUTPUT
    }
}

impl UsizeTr for UsizeD {
    fn value(&self) -> usize {
        self.0
    }
}

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

impl<X: ToUsize, Y: ToUsize> Add<UsizeS<Y>> for UsizeS<X>
where
    X: Add<Y>,
    TSum<X, Y>: ToUsize,
{
    type Output = UsizeS<TSum<X, Y>>;
    fn add(self, _rhs: UsizeS<Y>) -> Self::Output {
        UsizeS {
            _phantom: PhantomData,
        }
    }
}

impl Add<UsizeD> for UsizeD {
    type Output = Self;
    fn add(self, rhs: UsizeD) -> Self::Output {
        UsizeD(self.0 + rhs.0)
    }
}

#[cfg(feature = "experimental")]
impl<const X: usize, const Y: usize> Sub<UsizeS<Y>> for UsizeS<X>
where
    [(); X - Y]:,
{
    type Output = UsizeS<{ X - Y }>;
    fn sub(self, _rhs: UsizeS<Y>) -> Self::Output {
        UsizeS::<{ X - Y }>
    }
}

impl<X: ToUsize, Y: ToUsize> Sub<UsizeS<Y>> for UsizeS<X>
where
    X: Sub<Y>,
    TDiff<X, Y>: ToUsize,
{
    type Output = UsizeS<TDiff<X, Y>>;
    fn sub(self, _rhs: UsizeS<Y>) -> Self::Output {
        UsizeS {
            _phantom: PhantomData,
        }
    }
}

impl Sub<UsizeD> for UsizeD {
    type Output = Self;
    fn sub(self, rhs: UsizeD) -> Self::Output {
        UsizeD(self.0 - rhs.0)
    }
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

    #[test]
    #[cfg(not(feature = "experimental"))]
    fn add_check() {
        let a = UsizeS::<U0>::new();
        let b = UsizeS::<U1>::new();
        let c: UsizeS<U1> = a + b;
        assert_eq!(c.value(), 1);

        // can't compile
        // let c = UsizeS::<{ usize::MAX }>;
        // let _ = c + a;
    }
}
