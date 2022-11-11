use std::{any::Any, marker::PhantomData};

pub trait HList {
    const LEN: usize;
    type Constraint: HListFilter;
}

pub struct HNil<Filter = AnyF>
where
    Filter: HListFilter,
{
    _phantom: PhantomData<Filter>,
}

impl<A: HListFilter> HNil<A> {
    pub fn new() -> Self {
        Self {
            _phantom: PhantomData,
        }
    }
}

impl<A: HListFilter> HList for HNil<A> {
    const LEN: usize = 0;
    type Constraint = A;
}

pub struct HCons<I, Tail>
where
    Tail: HList,
{
    pub value: I,
    pub tail: Tail,
}

impl<I, Tail> HCons<I, Tail>
where
    Tail: HList,
{
    pub fn new(value: I, tail: Tail) -> HCons<I, Tail> {
        HCons { value, tail }
    }
}

impl<I, Tail> HList for HCons<I, Tail>
where
    Tail: HList,
    I: TInherit<<<Tail as HList>::Constraint as HListFilter>::Ty>,
{
    const LEN: usize = <Tail as HList>::LEN + 1;
    type Constraint = <Tail as HList>::Constraint;
}

pub trait TInherit<T: ?Sized> {}

pub trait HListFilter {
    type Ty: ?Sized;
}

// Default Any Filter
impl<A: Any> TInherit<dyn Any> for A {}

// Filter impl which allow Any Trait
pub enum AnyF {}

impl HListFilter for AnyF {
    type Ty = dyn Any;
}

#[cfg(test)]
mod test {
    use std::fmt::Display;

    use super::*;

    #[test]
    fn compile_check() {
        let _ = HCons::new(1, HCons::new("test", HNil::<AnyF>::new()));
    }

    impl<A: Display> TInherit<dyn Display> for A {}

    pub enum DisplayF {}
    impl HListFilter for DisplayF {
        type Ty = dyn Display;
    }

    #[test]
    fn displayable_filter_check() {
        let _ = HCons::new(1, HCons::new("test", HNil::<DisplayF>::new()));
    }
}
