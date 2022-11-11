use std::{any::Any, marker::PhantomData};

pub trait HList {
    const LEN: usize;
    type Restriction: HListGuarantee;
}

pub struct HNil<Filter: HListGuarantee> {
    _phantom: PhantomData<Filter>,
}

impl<A: HListGuarantee> HNil<A> {
    pub fn new() -> Self {
        Self {
            _phantom: PhantomData,
        }
    }
}

impl<A: HListGuarantee> HList for HNil<A> {
    const LEN: usize = 0;
    type Restriction = A;
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
    <Tail as HList>::Restriction: AllowT<I>,
{
    const LEN: usize = <Tail as HList>::LEN + 1;
    type Restriction = <Tail as HList>::Restriction;
}

// marker trait for restrict HList item type
pub trait HListGuarantee {}

// support trait of HListGuarantee
pub trait AllowT<I> {}

enum AnyF {}
impl HListGuarantee for AnyF {}

impl<A: Any> AllowT<A> for AnyF {}

#[cfg(test)]
mod test {
    use std::fmt::Display;

    use super::*;

    #[test]
    fn compile_check() {
        let _ = HCons::new(1, HCons::new("test", HNil::<AnyF>::new()));
    }

    enum DisplayF {}
    impl HListGuarantee for DisplayF {}
    impl<A: Display> AllowT<A> for DisplayF {}

    #[test]
    fn displayable_filter_check() {
        let _ = HCons::new(1, HCons::new("test", HNil::<DisplayF>::new()));
    }

    // struct Test {}

    // #[test]
    // fn cannot_compile() {
    //     let _ = HCons::new(1, HCons::new(Test {}, HNil::<DisplayF>::new()));
    // }
}
