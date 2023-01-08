use std::fmt;

pub enum BitRepr {
    B0,
    B1,
}

impl fmt::Display for BitRepr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            BitRepr::B0 => write!(f, "B0"),
            BitRepr::B1 => write!(f, "B1"),
        }
    }
}

pub enum UsizeSyntax {
    Term,
    BitArray(Box<UsizeSyntax>, BitRepr),
}

impl UsizeSyntax {
    pub fn add_bitrepr(self, bitrepr: BitRepr) -> Self {
        Self::BitArray(Box::new(self), bitrepr)
    }
}

impl fmt::Display for UsizeSyntax {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            UsizeSyntax::Term => write!(f, "NNil"),
            UsizeSyntax::BitArray(ref inner, ref bitrepr) => {
                write!(f, "NCons<{}, {}>", inner, bitrepr)
            }
        }
    }
}

pub fn create_typenum_structs(n: usize) -> UsizeSyntax {
    // 1Bit is required even when n is zero, so it must be greater than or equal to 1
    let range_max = highest_one_bit(n).max(1);
    let mut result = UsizeSyntax::Term;
    for i in (0..range_max).rev() {
        let bitrepr = match n & 1 << i == 0 {
            true => BitRepr::B0,
            false => BitRepr::B1,
        };
        result = result.add_bitrepr(bitrepr);
    }

    result
}

pub fn highest_one_bit(n: usize) -> usize {
    let mut u = n;
    for i in 0..(usize::BITS.ilog2() as usize) {
        u |= u >> (1 << i);
    }

    if u == 0 {
        0
    } else {
        u.count_ones() as usize
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn check_highest_one_bits() {
        assert_eq!(highest_one_bit(0), 0);
        assert_eq!(highest_one_bit(usize::MAX), usize::BITS as usize);
    }
}
