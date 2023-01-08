use forma_build_utils::create_typenum_structs;
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
    write_to_usize_impl(&outdir_path)?;

    Ok(())
}

fn write_to_usize_impl(outdir_path: &Path) -> std::io::Result<()> {
    let dest = Path::new(outdir_path).join("consts_tousize.rs");
    println!(
        "cargo:rustc-env=TYPENUM_BUILD_CONSTS_TYPENUM_IMPLS={}",
        dest.display()
    );
    let f = File::create(&dest).unwrap();
    let mut f = BufWriter::new(f);

    // usize is 64 bit
    let mut struct_accum = "NNil".to_string();
    let mut typearg_accum = "".to_string();
    for i in 1..=usize::BITS {
        let typearg = format!("BB{i} : Bit");
        struct_accum = format!("NCons<{struct_accum}, BB{i}>");
        typearg_accum = format!("{typearg}, {typearg_accum}");
        write!(
            f,
            "
            #[allow(dead_code)] 
            impl< {typearg_accum} > ToUsize for {struct_accum} 
            where
                Self: TypedBitLength<BitLength = consts::N{i}>
            {{
                const OUTPUT: usize = {i};
            }}
            "
        )?;
    }
    f.flush()?;
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
    for i in 0..=64 {
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
    let max = 100000;
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
