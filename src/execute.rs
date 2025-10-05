use cosmwasm_std::{Uint128, DepsMut, Env, MessageInfo, Response};
use crate::error::ContractError;
use crate::msg::RewardAllocation;
use crate::state::{CONFIG, POINTS_RULES, USER_MAP, UserInfo, UserLevel, ReferralStats, UserStatus, SystemConfig, PointsRules, PointsReason};

/// 执行用户注册
pub fn execute_register(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    referrer: Option<String>,
) -> Result<Response, ContractError> {
    let config = CONFIG.load(deps.storage)?;
    
    // 检查系统是否启用
    if !config.enabled {
        return Err(ContractError::SystemPaused);
    }
    
    let user = info.sender;
    
    // 检查用户是否已注册
    if USER_MAP.has(deps.storage, &user) {
        return Err(ContractError::UserAlreadyRegistered {
            user: user.to_string(),
        });
    }
    
    // 处理推荐人
    let referrer_addr = if let Some(ref ref_addr) = referrer {
        let addr = deps.api.addr_validate(&ref_addr)?;
        
        // 检查推荐人是否存在
        if !USER_MAP.has(deps.storage, &addr) {
            return Err(ContractError::InvalidReferrer {
                referrer: ref_addr.to_string(),
            });
        }
        
        // 检查循环推荐
        crate::referral::check_circular_referral(deps.as_ref(), &addr, &user)?;
        
        Some(addr)
    } else {
        None
    };
    
    // 创建用户信息
    let user_info = UserInfo {
        recommender: referrer_addr.clone(),
        direct_referrals: Vec::new(),
        reward_points: Uint128::zero(),
        registered_at: env.block.time.seconds(),
        last_active_at: env.block.time.seconds(),
        user_level: UserLevel::Bronze,
        referral_stats: ReferralStats {
            total_referrals: 0,
            active_referrals: 0,
            monthly_referrals: 0,
            last_referral_time: None,
            success_rate: cosmwasm_std::Decimal::zero(),
        },
        points_history: Vec::new(),
        status: UserStatus::Active,
    };
    
    // 保存用户信息
    USER_MAP.save(deps.storage, &user, &user_info)?;
    
    // 如果有推荐人，建立推荐关系
    if let Some(ref_addr) = referrer_addr {
        crate::referral::establish_referral_relation(deps, &env, &ref_addr, &user)?;
    }
    
    Ok(Response::new()
        .add_attribute("action", "user_registered")
        .add_attribute("user", user.to_string())
        .add_attribute("referrer", referrer.unwrap_or_default())
        .add_attribute("timestamp", env.block.time.seconds().to_string()))
}

/// 执行积分分配
pub fn execute_allocate_rewards(
    mut deps: DepsMut,
    env: Env,
    info: MessageInfo,
    user: String,
    points: Uint128,
    reason: PointsReason,
    related_user: Option<String>,
    event_id: Option<String>,
) -> Result<Response, ContractError> {
    // 检查管理员权限
    crate::security::check_admin_permission(deps.as_ref(), &info.sender)?;
    
    let user_addr = deps.api.addr_validate(&user)?;
    let related_addr = if let Some(rel_user) = related_user {
        Some(deps.api.addr_validate(&rel_user)?)
    } else {
        None
    };
    
    // 分配积分
    crate::points::allocate_points_to_user(
        &mut deps,
        &env,
        &user_addr,
        points,
        reason,
        related_addr,
        event_id,
    )
}

/// 执行批量积分分配
pub fn execute_batch_allocate_rewards(
    mut deps: DepsMut,
    env: Env,
    info: MessageInfo,
    rewards: Vec<RewardAllocation>,
) -> Result<Response, ContractError> {
    // 检查管理员权限
    crate::security::check_admin_permission(deps.as_ref(), &info.sender)?;
    
    let mut response = Response::new();
    let mut processed_count = 0;
    
    for reward in rewards {
        let user_addr = deps.api.addr_validate(&reward.user)?;
        let related_addr = if let Some(rel_user) = reward.related_user {
            Some(deps.api.addr_validate(&rel_user)?)
        } else {
            None
        };
        
        let allocation_response = crate::points::allocate_points_to_user(
            &mut deps,
            &env,
            &user_addr,
            reward.points,
            reward.reason,
            related_addr,
            reward.event_id,
        )?;
        
        response = response.add_attributes(allocation_response.attributes);
        processed_count += 1;
    }
    
    Ok(response
        .add_attribute("action", "batch_allocate_rewards")
        .add_attribute("processed_count", processed_count.to_string()))
}

/// 执行积分提取
pub fn execute_withdraw_points(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    amount: Uint128,
) -> Result<Response, ContractError> {
    let user = info.sender;
    
    // 检查系统是否暂停
    crate::security::check_system_paused(deps.as_ref())?;
    
    // 提取积分
    crate::points::withdraw_points(deps, &env, &user, amount)
}

/// 执行更新系统配置
pub fn execute_update_config(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    config: SystemConfig,
) -> Result<Response, ContractError> {
    // 检查管理员权限
    crate::security::check_admin_permission(deps.as_ref(), &info.sender)?;
    
    // 保存新配置
    CONFIG.save(deps.storage, &config)?;
    
    Ok(Response::new()
        .add_attribute("action", "update_config")
        .add_attribute("timestamp", env.block.time.seconds().to_string()))
}

/// 执行更新积分规则
pub fn execute_update_points_rules(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    rules: PointsRules,
) -> Result<Response, ContractError> {
    // 检查管理员权限
    crate::security::check_admin_permission(deps.as_ref(), &info.sender)?;
    
    // 保存新规则
    POINTS_RULES.save(deps.storage, &rules)?;
    
    Ok(Response::new()
        .add_attribute("action", "update_points_rules")
        .add_attribute("timestamp", env.block.time.seconds().to_string()))
}

/// 执行紧急暂停
pub fn execute_emergency_pause(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    paused: bool,
) -> Result<Response, ContractError> {
    // 检查管理员权限
    crate::security::check_admin_permission(deps.as_ref(), &info.sender)?;
    
    let mut config = CONFIG.load(deps.storage)?;
    config.emergency_paused = paused;
    CONFIG.save(deps.storage, &config)?;
    
    Ok(Response::new()
        .add_attribute("action", "emergency_pause")
        .add_attribute("paused", paused.to_string())
        .add_attribute("timestamp", env.block.time.seconds().to_string()))
}
