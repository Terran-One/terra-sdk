use crate::{AccAddress, Coins};

pub struct MsgStoreCode {
    pub sender: AccAddress,
    pub wasm_byte_code: String,
}

pub struct MsgMigrateCode {
    pub sender: AccAddress,
    pub code_id: u64,
    pub wasm_byte_code: String,
}

pub struct MsgInstantiateContract {
    pub sender: AccAddress,
    pub admin: AccAddress,
    pub code_id: u64,
    pub init_msg: InstantiateMsg,
    pub init_coins: Coins,
}

pub struct MsgExecuteContract {
    pub sender: AccAddress,
    pub contract: AccAddress,
    pub execute_msg: ExecuteMsg,
    pub coins: Coins,
}

pub struct MsgMigrateContract {
    pub admin: AccAddress,
    pub contract: AccAddress,
    pub new_code_id: u64,
    pub migrate_msg: MigrateMsg,
}

pub struct MsgClearContractAdmin {
    pub admin: AccAddress,
    pub contract: AccAddress,
}

pub struct MsgUpdateContractAdmin {
    pub admin: AccAddress,
    pub new_admin: AccAddress,
    pub contract: AccAddress,
}
