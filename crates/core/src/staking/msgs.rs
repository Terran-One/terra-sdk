pub struct MsgDelegate {
    pub delegator_address: AccAddress,
    pub validator_address: ValAddress,
    pub amount: Coin,
}

pub struct MsgUndelegate {
    pub delegator_address: AccAddress,
    pub validator_address: ValAddress,
    pub amount: Coin,
}

pub struct MsgBeginRedelegate {
    pub delegator_address: AccAddress,
    pub validator_src_address: ValAddress,
    pub validator_dst_address: ValAddress,
    pub amount: Coin,
}

pub struct MsgEditValidator {
    pub description: ValidatorDescription,
    pub validator_address: ValAddress,
    pub commisssion_rate: Option<Dec>,
    pub min_self_delegation: Option<Uint128>,
}

pub struct MsgCreateValidator {
    pub description: ValidatorDescription,
    pub commission: ValidatorCommissionRates,
    pub min_self_delegation: Uint128,
    pub delegator_address: AccAddress,
    pub validator_address: ValAddress,
    pub pubkey: ValConsPublicKey,
    pub value: Coin,
}
