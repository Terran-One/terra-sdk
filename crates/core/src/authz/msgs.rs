pub struct MsgRevokeAuthorization {
    pub granter: AccAddress,
    pub grantee: AccAddress,
    pub msg_type_url: String,
}

pub struct MsgGrantAuthorization {
    pub granter: AccAddress,
    pub grantee: AccAddress,
    /// TODO: type
    pub grant: String,
}

pub struct MsgExecAuthorized {
    pub grantee: AccAddress,
    pub msgs: Vec<dyn Msg>,
}
