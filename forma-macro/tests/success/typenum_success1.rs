use forma_macro::typenum;
use forma_typenum::typenum::*;

type ONE = typenum!(1);
type TWO = typenum!(2);

fn main() {
    let o1 = ONE::new();
    let o2 = ONE::new();
    let _: TWO = o1 + o2;
}
