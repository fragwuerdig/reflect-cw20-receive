use cosmwasm_std::{
    entry_point, from_binary, CosmosMsg, DepsMut, Empty, Env, MessageInfo, Reply, Response, StdResult, SubMsg
};
use cw20::Cw20ReceiveMsg;

use crate::errors::ReflectError;
use crate::msg::{
    ExecuteMsg, InstantiateMsg, SpecialQuery,
};

#[entry_point]
pub fn instantiate(
    _deps: DepsMut<SpecialQuery>,
    _env: Env,
    _info: MessageInfo,
    _msg: InstantiateMsg,
) -> StdResult<Response> {
    Ok(Response::default())
}

#[entry_point]
pub fn execute(
    deps: DepsMut<SpecialQuery>,
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ReflectError> {
    match msg {
        ExecuteMsg::ReflectMsg { msgs } => try_reflect(deps, env, info, msgs),
        ExecuteMsg::ReflectSubMsg { msgs } => try_reflect_subcall(deps, env, info, msgs),
        ExecuteMsg::Receive(msg) => try_receive(deps, env, info, msg),
    }
}

pub fn try_receive(
    _deps: DepsMut<SpecialQuery>,
    _env: Env,
    _info: MessageInfo,
    msg: Cw20ReceiveMsg,
) -> Result<Response, ReflectError> {
    let unwrapped: CosmosMsg = from_binary(&msg.msg)?;
    Ok(Response::new().add_message(unwrapped))
}

pub fn try_reflect(
    _deps: DepsMut<SpecialQuery>,
    _env: Env,
    _info: MessageInfo,
    msgs: Vec<CosmosMsg>,
) -> Result<Response, ReflectError> {

    if msgs.is_empty() {
        return Err(ReflectError::MessagesEmpty);
    }

    Ok(Response::new()
        .add_attribute("action", "reflect")
        .add_messages(msgs))
}

pub fn try_reflect_subcall(
    _deps: DepsMut<SpecialQuery>,
    _env: Env,
    _info: MessageInfo,
    msgs: Vec<SubMsg>,
) -> Result<Response, ReflectError> {
    if msgs.is_empty() {
        return Err(ReflectError::MessagesEmpty);
    }

    Ok(Response::new()
        .add_attribute("action", "reflect_subcall")
        .add_submessages(msgs))
}

/// This just stores the result for future query
#[entry_point]
pub fn reply(_deps: DepsMut<SpecialQuery>, _env: Env, _msg: Reply) -> Result<Response, ReflectError> {
    Ok(Response::default())
}
