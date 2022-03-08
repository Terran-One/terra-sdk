pub struct MsgWithdrawValidatorCommission {
    pub validator_address: ValAddress,
}

pub struct MsgWithdrawDelegatorReward {
    pub delegator_address: AccAddress,
    pub validator_address: ValAddress,
}

pub struct MsgSetWithdrawAddress {
    pub delegator_address: AccAddress,
    pub withdraw_address: AccAddress,
}

pub struct MsgFundCommunityPool {
    pub depositor: AccAddress,
    pub amount: Coins,
}
