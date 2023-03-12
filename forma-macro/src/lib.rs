use forma_build_utils::create_typenum_structs;
use proc_macro::TokenStream;
use syn::{parse_macro_input, LitInt};

#[proc_macro]
#[allow(non_snake_case)]
pub fn typenum(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as LitInt);
    let usize_num: usize = input.base10_parse::<usize>().expect("Failed to parse");

    let typenum_struct_string = create_typenum_structs(usize_num).to_string();
    typenum_struct_string.parse().unwrap()
}

#[proc_macro]
#[allow(non_snake_case)]
pub fn UsizeS(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as LitInt);
    let usize_num: usize = input.base10_parse::<usize>().expect("Failed to parse");

    let typenum_struct_string = create_typenum_structs(usize_num).to_string();
    let tkstream: TokenStream = format!("UsizeS::< {typenum_struct_string} >")
        .parse()
        .unwrap();
    println!("{tkstream}");

    tkstream
}
