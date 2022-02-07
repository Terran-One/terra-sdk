use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput, Path};

pub(crate) fn do_derive_msg_type(input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as DeriveInput);
    let DeriveInput {
        ident, data, attrs, ..
    } = ast;

    let expanded = quote! {
        impl<'de> MsgType<'de> for #ident {
            fn to_json(&self) -> ::serde_json::Value {
                "hello".into()
            }

            fn from_json(data: &::serde_json::Value) -> Self {

            }
        }

    };

    expanded.into()
}
