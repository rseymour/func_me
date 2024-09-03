extern crate proc_macro;
use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, ItemFn};

#[proc_macro_attribute]
pub fn print_signature_bad(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as ItemFn);
    let name = &input.sig.ident;
    let inputs = &input.sig.inputs;
    let output = &input.sig.output;

    let expanded = quote! {
        fn #name(&self) -> #output {
            println!("Function name: {}", stringify!(#name));
            println!("Input: {:?}", stringify!(#inputs));
            println!("Output: {:?}", stringify!(#output));
            #input
        }
    };

    TokenStream::from(expanded)
}

#[proc_macro_attribute]
pub fn print_signature(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as ItemFn);
    let name = &input.sig.ident;
    let inputs = &input.sig.inputs;
    let output = &input.sig.output;
    let stmts = &input.block.stmts;

    quote! {fn #name(#inputs) #output { #(#stmts)* }}.into()
}
