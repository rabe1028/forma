use forma_macro::UsizeS;
use forma_typenum::typenum::*;
use forma_typenum::usize::*;

type ONE = UsizeS!(1);

fn main() {
    let o1 = ONE::new();
    // (64 bit) usize::MAX - 1
    let o2 = <UsizeS!(18_446_744_073_709_551_614)>::new();
    let _ = o1 + o2;
}
