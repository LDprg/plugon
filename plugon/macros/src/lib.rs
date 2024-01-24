use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, ItemFn};

#[proc_macro_attribute]
pub fn export(
    _attr: proc_macro::TokenStream,
    input: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    // Parse the input tokens into a syntax tree
    let input = parse_macro_input!(input as ItemFn);

    let vis = input.vis;
    let sig = input.sig;
    let block = input.block;

    // Build the output, possibly using quasi-quotation
    let expanded = quote! {
       #[no_mangle]
       #vis extern #sig #block
    };

    
    // Hand the output tokens back to the compiler
    let expanded = TokenStream::from(expanded);

    //println!("item: \"{}\"", expanded.to_string());

    expanded
}
