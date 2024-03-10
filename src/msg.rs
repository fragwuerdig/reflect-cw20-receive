use cosmwasm_schema::cw_serde;
use cosmwasm_std::{CosmosMsg, CustomQuery, SubMsg};
use cw20::Cw20ReceiveMsg;

#[cw_serde]
pub struct InstantiateMsg {}

#[cw_serde]
pub enum ExecuteMsg {
    ReflectMsg { msgs: Vec<CosmosMsg> },
    ReflectSubMsg { msgs: Vec<SubMsg> },
    Receive(Cw20ReceiveMsg)
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
