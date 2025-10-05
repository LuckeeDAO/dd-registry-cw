use cosmwasm_std::{Addr, Uint128, Deps, DepsMut, Env, Response, Decimal};
use crate::state::{UserInfo, UserLevel, ReferralStats, UserStatus, USER_MAP, REFERRAL_CHAIN};
use crate::error::ContractError;

/// 创建新用户信息
pub fn create_user_info(
    _user: &Addr,
    referrer: Option<Addr>,
    env: &Env,
) -> UserInfo {
    UserInfo {
        recommender: referrer.clone(),
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
            success_rate: Decimal::zero(),
        },
        points_history: Vec::new(),
        status: UserStatus::Active,
    }
}

/// 更新用户等级
pub fn update_user_level(
    deps: &mut DepsMut,
    user: &Addr,
    referral_count: u32,
) -> Result<Response, ContractError> {
    let mut user_info = USER_MAP.load(deps.storage, user)?;
    let old_level = user_info.user_level.clone();
    
    // 计算新等级
    let new_level = UserLevel::from_referral_count(referral_count);
    
    if new_level != old_level {
        user_info.user_level = new_level.clone();
        USER_MAP.save(deps.storage, user, &user_info)?;
        
        Ok(Response::new()
            .add_attribute("action", "level_up")
            .add_attribute("user", user.to_string())
            .add_attribute("old_level", format!("{:?}", old_level))
            .add_attribute("new_level", format!("{:?}", new_level)))
    } else {
        Ok(Response::new())
    }
}

/// 添加推荐关系
pub fn add_referral_relation(
    deps: DepsMut,
    env: &Env,
    referrer: &Addr,
    referee: &Addr,
) -> Result<(), ContractError> {
    // 更新推荐人的直接推荐列表
    let mut referrer_info = USER_MAP.load(deps.storage, referrer)?;
    referrer_info.direct_referrals.push(referee.clone());
    referrer_info.referral_stats.total_referrals += 1;
    referrer_info.referral_stats.active_referrals += 1;
    referrer_info.referral_stats.last_referral_time = Some(env.block.time.seconds());
    
    USER_MAP.save(deps.storage, referrer, &referrer_info)?;
    
    // 建立推荐链映射
    REFERRAL_CHAIN.save(deps.storage, referee, referrer)?;
    
    Ok(())
}

/// 检查用户是否存在
pub fn user_exists(deps: Deps, user: &Addr) -> bool {
    USER_MAP.has(deps.storage, user)
}

/// 获取用户信息
pub fn get_user_info(deps: Deps, user: &Addr) -> Result<UserInfo, ContractError> {
    Ok(USER_MAP.load(deps.storage, user)?)
}

/// 更新用户活跃时间
pub fn update_user_activity(
    deps: DepsMut,
    user: &Addr,
    timestamp: u64,
) -> Result<(), ContractError> {
    let mut user_info = USER_MAP.load(deps.storage, user)?;
    user_info.last_active_at = timestamp;
    USER_MAP.save(deps.storage, user, &user_info)?;
    Ok(())
}
