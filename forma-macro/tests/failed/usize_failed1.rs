use forma_macro::UsizeS;
use forma_typenum::typenum::*;
use forma_typenum::usize::*;

type ONE = UsizeS!(1);

fn main() {
    let o1 = ONE::new();
    let o2 = <UsizeS!(18_446_744_073_709_551_615)>::new();
    // usize::MAX + 1 is overflowed, so this case should fail compiling
    let _ = o1 + o2;
}
