use proc_macro::TokenStream;
mod msg_type;

#[proc_macro_derive(MsgType, attributes(msgtype, msgtype_pb))]
pub fn derive_msg_type(input: TokenStream) -> TokenStream {
    msg_type::do_derive_msg_type(input)
}
