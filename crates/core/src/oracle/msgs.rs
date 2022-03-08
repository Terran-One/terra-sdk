pub struct MsgAggregateExchangeRatePrevote {
    pub hash: String,
    pub feeder: AccAddress,
    pub validator: ValAddress,
}

pub struct MsgAggregateExchangeRateVote {
    pub exchange_rates: Coins,
    pub salt: String,
    pub feeder: AccAddress,
    pub validator: ValAddress,
}

pub struct MsgDelegateFeedConsent {
    pub operator: ValAddress,
    pub delegate: AccAddress,
}
