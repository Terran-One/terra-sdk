pub struct MsgDeposit {
    pub proposal_id: u128,
    pub depositor: AccAddress,
    pub amount: Coins,
}

pub struct MsgSubmitProposal {
    pub content: Proposal,
    pub initial_deposit: Coins,
    pub proposer: AccAddress,
}
