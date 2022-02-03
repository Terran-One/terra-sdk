use darling::util::parse_attribute_to_meta_list;
use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput, Path};

pub(crate) fn do_derive_msg(input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as DeriveInput);
    let DeriveInput {
        ident, data, attrs, ..
    } = ast;

    let paths: Vec<Path> = attrs
        .iter()
        .map(|x| parse_attribute_to_meta_list(x))
        .map(|x| x.unwrap())
        .map(|x| x.path)
        .collect();

    let expanded = quote! {
        impl Msg for #ident {
            fn to_json(&self) -> String {
                "hello".into()
            }
        }

        #(#paths,)*
    };

    expanded.into()
}
