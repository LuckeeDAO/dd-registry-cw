use cosmwasm_std::{
    entry_point, to_json_binary, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult,
};
use crate::error::ContractError;
use crate::msg::{ExecuteMsg, InstantiateMsg, QueryMsg};

#[entry_point]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    // 验证管理员地址
    let admin = deps.api.addr_validate(&msg.admin)?;
    
    // 保存配置
    crate::state::CONFIG.save(deps.storage, &msg.config)?;
    crate::state::POINTS_RULES.save(deps.storage, &msg.points_rules)?;
    
    Ok(Response::new()
        .add_attribute("method", "instantiate")
        .add_attribute("admin", admin.to_string())
        .add_attribute("contract_name", crate::CONTRACT_NAME)
        .add_attribute("contract_version", crate::CONTRACT_VERSION))
}

#[entry_point]
pub fn execute(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        ExecuteMsg::Register { referrer } => {
            crate::execute::execute_register(deps, env, info, referrer)
        }
        ExecuteMsg::AllocateRewards {
            user,
            points,
            reason,
            related_user,
            event_id,
        } => {
            crate::execute::execute_allocate_rewards(
                deps, env, info, user, points, reason, related_user, event_id,
            )
        }
        ExecuteMsg::BatchAllocateRewards { rewards } => {
            crate::execute::execute_batch_allocate_rewards(deps, env, info, rewards)
        }
        ExecuteMsg::WithdrawPoints { amount } => {
            crate::execute::execute_withdraw_points(deps, env, info, amount)
        }
        ExecuteMsg::UpdateConfig { config } => {
            crate::execute::execute_update_config(deps, env, info, config)
        }
        ExecuteMsg::UpdatePointsRules { rules } => {
            crate::execute::execute_update_points_rules(deps, env, info, rules)
        }
        ExecuteMsg::EmergencyPause { paused } => {
            crate::execute::execute_emergency_pause(deps, env, info, paused)
        }
    }
}

#[entry_point]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::GetUserInfo { user } => {
            to_json_binary(&crate::query::query_user_info(deps, user)?)
        }
        QueryMsg::GetReferrer { user } => {
            to_json_binary(&crate::query::query_referrer(deps, user)?)
        }
        QueryMsg::GetDirectReferrals {
            user,
            limit,
            start_after,
        } => {
            to_json_binary(&crate::query::query_direct_referrals(deps, user, limit, start_after)?)
        }
        QueryMsg::GetReferralChain { user, max_depth } => {
            to_json_binary(&crate::query::query_referral_chain(deps, user, max_depth)?)
        }
        QueryMsg::GetUserPoints { user } => {
            to_json_binary(&crate::query::query_user_points(deps, user)?)
        }
        QueryMsg::GetPointsLeaderboard { limit, start_after } => {
            to_json_binary(&crate::query::query_points_leaderboard(deps, limit, start_after)?)
        }
        QueryMsg::GetPointsHistory {
            user,
            limit,
            start_after,
        } => {
            to_json_binary(&crate::query::query_points_history(deps, user, limit, start_after)?)
        }
        QueryMsg::GetConfig {} => {
            to_json_binary(&crate::query::query_config(deps)?)
        }
        QueryMsg::GetPointsRules {} => {
            to_json_binary(&crate::query::query_points_rules(deps)?)
        }
        QueryMsg::GetLevelStats {} => {
            to_json_binary(&crate::query::query_level_stats(deps)?)
        }
        QueryMsg::ValidateReferral { referrer, referee } => {
            to_json_binary(&crate::query::query_validate_referral(deps, referrer, referee)?)
        }
    }
}
