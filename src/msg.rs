use cosmwasm_schema::cw_serde;
use cosmwasm_std::{Binary, CosmosMsg, CustomQuery, SubMsg};
use cw20::Cw20ReceiveMsg;

#[cw_serde]
pub struct InstantiateMsg {}

#[cw_serde]
pub enum ExecuteMsg {
    ReflectMsg { msgs: Vec<CosmosMsg<CustomMsg>> },
    ReflectSubMsg { msgs: Vec<SubMsg<CustomMsg>> },
    Receive(Cw20ReceiveMsg)
}

#[cw_serde]
/// CustomMsg is an override of CosmosMsg::Custom to show this works and can be extended in the contract
pub enum CustomMsg {
    Debug(String),
    Raw(Binary),
}

impl cosmwasm_std::CustomMsg for CustomMsg {}

impl From<CustomMsg> for CosmosMsg<CustomMsg> {
    fn from(original: CustomMsg) -> Self {
        CosmosMsg::Custom(original)
    }
}

#[cw_serde]
/// An implementation of QueryRequest::Custom to show this works and can be extended in the contract
pub enum SpecialQuery {
    Ping {},
    Capitalized { text: String },
}

impl CustomQuery for SpecialQuery {}

#[cw_serde]
/// The response data for all `SpecialQuery`s
pub struct SpecialResponse {
    pub msg: String,
}
