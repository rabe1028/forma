use forma_core::hlist::*;

use std::fmt::Display;

enum DisplayF {}
impl HListGuarantee for DisplayF {}
impl<A: Display> AllowT<A> for DisplayF {}

#[test]
fn hlist_compile_test() {
    let _ = HCons::new(1, HCons::new("test", HNil::<DisplayF>::new()));
}
