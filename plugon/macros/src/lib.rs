#![feature(proc_macro_quote)]

use std::path::Path;
use proc_macro::{self, quote, TokenStream};

#[proc_macro]
pub fn shared(input: TokenStream) -> TokenStream {
    TokenStream::from(quote!{fn dummy(){}})
}
