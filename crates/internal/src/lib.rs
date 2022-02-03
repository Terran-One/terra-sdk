use proc_macro::TokenStream;
mod msg;

#[proc_macro_derive(Msg)]
pub fn derive_msg(input: TokenStream) -> TokenStream {
    msg::do_derive_msg(input)
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
