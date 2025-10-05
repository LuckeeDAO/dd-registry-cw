use cosmwasm_std::{Addr, Deps, DepsMut, Env, Response};
use crate::error::ContractError;
use crate::state::{REFERRAL_CHAIN, USER_MAP};
use crate::user::add_referral_relation;

/// 验证推荐关系
pub fn validate_referral(
    deps: Deps,
    referrer: &Addr,
    referee: &Addr,
) -> Result<(), ContractError> {
    // 检查推荐人是否存在
    if !USER_MAP.has(deps.storage, referrer) {
        return Err(ContractError::InvalidReferrer {
            referrer: referrer.to_string(),
        });
    }
    
    // 检查被推荐人是否已注册
    if USER_MAP.has(deps.storage, referee) {
        return Err(ContractError::UserAlreadyRegistered {
            user: referee.to_string(),
        });
    }
    
    // 检查循环推荐
    check_circular_referral(deps, referrer, referee)?;
    
    Ok(())
}

/// 检查循环推荐
pub fn check_circular_referral(
    deps: Deps,
    referrer: &Addr,
    referee: &Addr,
) -> Result<(), ContractError> {
    let mut current = referrer.clone();
    let mut visited = std::collections::HashSet::new();
    
    // 向上追溯推荐链
    while let Ok(Some(parent)) = REFERRAL_CHAIN.may_load(deps.storage, &current) {
        if visited.contains(&parent) {
            return Err(ContractError::CircularReferral {
                referrer: referrer.to_string(),
                referee: referee.to_string(),
            });
        }
        
        if parent == *referee {
            return Err(ContractError::CircularReferral {
                referrer: referrer.to_string(),
                referee: referee.to_string(),
            });
        }
        
        visited.insert(parent.clone());
        current = parent;
    }
    
    Ok(())
}

/// 建立推荐关系
pub fn establish_referral_relation(
    deps: DepsMut,
    env: &Env,
    referrer: &Addr,
    referee: &Addr,
) -> Result<Response, ContractError> {
    // 验证推荐关系
    validate_referral(deps.as_ref(), referrer, referee)?;
    
    // 添加推荐关系
    add_referral_relation(deps, env, referrer, referee)?;
    
    Ok(Response::new()
        .add_attribute("action", "referral_established")
        .add_attribute("referrer", referrer.to_string())
        .add_attribute("referee", referee.to_string())
        .add_attribute("timestamp", env.block.time.seconds().to_string()))
}

/// 获取推荐链
pub fn get_referral_chain(
    deps: Deps,
    user: &Addr,
    max_depth: Option<u32>,
) -> Result<Vec<Addr>, ContractError> {
    let config = crate::state::CONFIG.load(deps.storage)?;
    let max_depth = max_depth.unwrap_or(config.max_referral_depth);
    
    let mut chain = Vec::new();
    let mut current = user.clone();
    let mut depth = 0;
    
    while depth < max_depth {
        if let Ok(Some(parent)) = REFERRAL_CHAIN.may_load(deps.storage, &current) {
            chain.push(parent.clone());
            current = parent;
            depth += 1;
        } else {
            break;
        }
    }
    
    Ok(chain)
}
