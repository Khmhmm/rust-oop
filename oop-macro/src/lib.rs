
use quote::quote;
use syn::{parse_macro_input, DeriveInput};
use proc_macro::TokenStream;


#[proc_macro_derive(Inherit)]
pub fn derive_inherit(input: TokenStream) -> TokenStream {
    // Parse the input tokens into a syntax tree
    let input = parse_macro_input!(input as DeriveInput);
    let name = input.ident;

    // Build the output, possibly using quasi-quotation
    let expanded = quote! {
        impl<T> Inherit<T> for #name{
            fn inherent_call(t: &T, arg: BoxedArg){
                
            }
        }
    };

    // Hand the output tokens back to the compiler
    TokenStream::from(expanded)
}
