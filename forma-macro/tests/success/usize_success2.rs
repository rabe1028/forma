use forma_macro::UsizeS;
use forma_typenum::typenum::*;
use forma_typenum::usize::*;

type ONE = UsizeS!(1);

fn main() {
    // use type alias
    let o1 = ONE::new();
    let o2 = <UsizeS!(1)>::new();
    let _: UsizeS!(2) = o1 + o2;
}
