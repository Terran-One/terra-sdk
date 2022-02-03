use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

pub(crate) fn do_derive_msg(input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as DeriveInput);
    let DeriveInput {
        ident, data, attrs, ..
    } = ast;

    let expanded = quote! {
        impl Msg for #ident {
            fn to_json(&self) -> String {
                "hello".into()
            }
        }
    };

    expanded.into()
}
