extern crate proc_macro;

use quote::quote;
use syn::{parse_macro_input, ItemFn};
use proc_macro2::{Ident, Span};

#[proc_macro_attribute]
pub fn main(_attr: proc_macro::TokenStream, item: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let mut function = parse_macro_input!(item as ItemFn);

    let vis = &function.vis;
    let gen = &function.sig.generics;
    let output = &function.sig.output;
    let ident = function.sig.ident.clone();

    let generated_ident = Ident::new(format!("__generated_{}", ident).as_str(), Span::call_site());
    function.sig.ident = generated_ident.clone();

    let result = quote! {
        #function

        #vis fn #ident #gen () #output {
            tokio_env::start_with(#generated_ident()).expect("Failed to start runtime")
        }
    };

    return result.into();
}