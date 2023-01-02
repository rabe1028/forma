use std::fmt;
use std::fs::File;
use std::io::Write;
use std::path::Path;
use std::{env, io::BufWriter};

fn main() -> std::io::Result<()> {
    println!("cargo:rerun-if-changed=build/main.rs");

    let outdir = env::var("OUT_DIR").unwrap();
    let outdir_path = Path::new(&outdir);
    write_usize_consts(&outdir_path)?;
    write_typenum_consts(&outdir_path)?;

    Ok(())
}

fn write_typenum_consts(outdir_path: &Path) -> std::io::Result<()> {
    let dest = Path::new(outdir_path).join("consts_typenum.rs");
    println!(
        "cargo:rustc-env=TYPENUM_BUILD_CONSTS_TYPENUM={}",
        dest.display()
    );

    let f = File::create(&dest).unwrap();
    let mut f = BufWriter::new(f);

    write_header(&mut f)?;
    for i in 0..64 {
        write!(
            f,
            "
            #[allow(dead_code)] 
            pub type N{i} = {typenum};
            ",
            typenum = create_typenum_structs(i)
        )?;
    }
    f.flush()?;
    Ok(())
}

fn write_usize_consts(outdir_path: &Path) -> std::io::Result<()> {
    let dest = Path::new(outdir_path).join("consts_usize.rs");
    println!(
        "cargo:rustc-env=TYPENUM_BUILD_CONSTS_USIZE={}",
        dest.display()
    );

    let f = File::create(&dest).unwrap();
    let mut f = BufWriter::new(f);

    // let max = usize::MAX>>3;
    let max = 1000;
    let sep = 1000;

    for chunk_index in 0..=(max / sep) {
        let from = chunk_index * sep;
        let to = (from + sep).min(max);
        let module_name = format!("consts_{chunk_index}");
        let partial_dest = Path::new(&outdir_path).join(format!("{module_name}.rs"));
        // writeln!(f, r#"include!("{}");"#, partial_dest.display())?;
        writeln!(
            f,
            r#"
        #[path="{}"]
        pub mod {module_name};
        #[allow(dead_code)] 
        pub use {module_name}::*;
        "#,
            partial_dest.display()
        )?;

        let mut buf = BufWriter::new(File::create(&partial_dest).expect("Cannot create file"));
        write_header(&mut buf)?;
        write_usize_impls(&mut buf, from..to)?;
    }

    Ok(())
}

fn write_header<W: Write>(buf: &mut W) -> std::io::Result<()> {
    writeln!(
        buf,
        "
    #[allow(unused_imports)] 
    use crate::typenum::*;
    #[allow(unused_imports)] 
    use crate::usize::*;
    "
    )
}

fn write_usize_impls<W: Write>(
    buf: &mut W,
    iter: impl Iterator<Item = usize>,
) -> std::io::Result<()> {
    for i in iter {
        write!(
            buf,
            "#[allow(dead_code)] 
            pub type U{i} = {typenum};
            // impl ToTypeNum for UsizeS<{i}> {{
            //     type Output = U{i};
            // }}
            // impl FromTypeNum for U{i} {{
            //     type Output = UsizeS<{i}>;
            // }}
            ",
            typenum = create_typenum_structs(i)
        )?;
    }
    buf.flush()?;
    Ok(())
}

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

fn create_typenum_structs(n: usize) -> UsizeSyntax {
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

fn highest_one_bit(n: usize) -> usize {
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
