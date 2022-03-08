pub struct MsgGrantAllowance {
    pub granter: AccAddress,
    pub grantee: AccAddress,
    pub allowance: Allowance,
}

pub struct MsgRevokeAllowance {
    pub granter: AccAddress,
    pub grantee: AccAddress,
}
