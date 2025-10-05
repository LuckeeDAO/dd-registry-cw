use cosmwasm_std::{Addr, Uint128, Deps, DepsMut, Env, Response, Decimal, Order};
use crate::state::{USER_MAP, POINTS_RULES, POINTS_LEADERBOARD, PointsRecord, PointsReason};
use crate::error::ContractError;
use crate::user::{get_user_info, update_user_level};

/// 分配积分给用户
pub fn allocate_points_to_user(
    deps: &mut DepsMut,
    env: &Env,
    user: &Addr,
    points: Uint128,
    reason: PointsReason,
    related_user: Option<Addr>,
    event_id: Option<String>,
) -> Result<Response, ContractError> {
    let mut user_info = get_user_info(deps.as_ref(), user)?;
    
    // 添加积分
    user_info.reward_points += points;
    
    // 记录积分历史
    user_info.points_history.push(PointsRecord {
        points_change: points,
        reason: reason.clone(),
        timestamp: env.block.time.seconds(),
        related_user,
        event_id,
    });
    
    // 保存用户信息
    USER_MAP.save(deps.storage, user, &user_info)?;
    
    // 更新用户等级
    let _level_response = update_user_level(deps, user, user_info.referral_stats.total_referrals)?;
    
    // 更新排行榜
    update_leaderboard(deps.storage, user, user_info.reward_points)?;
    
    Ok(Response::new()
        .add_attribute("action", "points_allocated")
        .add_attribute("user", user.to_string())
        .add_attribute("points", points.to_string())
        .add_attribute("reason", format!("{:?}", reason))
        .add_attribute("total_points", user_info.reward_points.to_string()))
}

/// 分配多层级推荐奖励
pub fn allocate_multi_level_rewards(
    deps: &mut DepsMut,
    env: &Env,
    referee: &Addr,
    base_points: Uint128,
    reason: PointsReason,
    event_id: Option<String>,
) -> Result<Response, ContractError> {
    let config = crate::state::CONFIG.load(deps.storage)?;
    let rules = POINTS_RULES.load(deps.storage)?;
    
    let mut response = Response::new();
    let mut current_user = referee.clone();
    let mut level = 1;
    
    // 向上追溯推荐链
    while level <= config.max_referral_depth {
        if let Ok(Some(referrer)) = crate::state::REFERRAL_CHAIN.may_load(deps.storage, &current_user) {
            // 计算当前层级的奖励比例
            let rate = match level {
                1 => rules.direct_referral_rate,
                2 => rules.level_2_rate,
                3 => rules.level_3_rate,
                _ => Decimal::zero(),
            };
            
            if !rate.is_zero() {
                // 计算奖励积分
                let reward_points = base_points.multiply_ratio(
                    rate.atomics(),
                    Uint128::from(10_u128.pow(rate.decimal_places()))
                );
                
                // 应用等级倍数
                let user_info = get_user_info(deps.as_ref(), &referrer)?;
                let default_multiplier = Decimal::one();
                let level_multiplier = rules.level_multipliers
                    .get(&user_info.user_level)
                    .unwrap_or(&default_multiplier);
                
                let final_points = reward_points.multiply_ratio(
                    level_multiplier.atomics(),
                    Uint128::from(10_u128.pow(level_multiplier.decimal_places()))
                );
                
                // 分配积分
                let allocation_response = allocate_points_to_user(
                    deps,
                    env,
                    &referrer,
                    final_points,
                    reason.clone(),
                    Some(referee.clone()),
                    event_id.clone(),
                )?;
                
                response = response.add_attributes(allocation_response.attributes);
                
                // 添加事件属性
                response = response
                    .add_attribute("level", level.to_string())
                    .add_attribute("referrer", referrer.to_string())
                    .add_attribute("points", final_points.to_string());
            }
            
            current_user = referrer;
            level += 1;
        } else {
            break;
        }
    }
    
    Ok(response)
}

/// 更新排行榜
pub fn update_leaderboard(
    storage: &mut dyn cosmwasm_std::Storage,
    user: &Addr,
    points: Uint128,
) -> Result<(), ContractError> {
    // 使用积分作为键，用户地址作为值
    // 注意：这里需要处理相同积分的情况
    let key = points.u128();
    POINTS_LEADERBOARD.save(storage, key, user)?;
    Ok(())
}

/// 获取积分排行榜
pub fn get_leaderboard(
    deps: Deps,
    limit: Option<u32>,
    _start_after: Option<String>,
) -> Result<Vec<(Addr, Uint128, u32)>, ContractError> {
    let limit = limit.unwrap_or(100);
    let mut entries = Vec::new();
    let mut rank = 1;
    
    // 按积分降序排列
    let range = POINTS_LEADERBOARD
        .range(deps.storage, None, None, Order::Descending)
        .take(limit as usize);
    
    for result in range {
        let (points, user) = result?;
        entries.push((user, Uint128::from(points), rank));
        rank += 1;
    }
    
    Ok(entries)
}

/// 提取积分
pub fn withdraw_points(
    deps: DepsMut,
    env: &Env,
    user: &Addr,
    amount: Uint128,
) -> Result<Response, ContractError> {
    let mut user_info = get_user_info(deps.as_ref(), user)?;
    
    // 检查积分余额
    if user_info.reward_points < amount {
        return Err(ContractError::InsufficientPoints {
            required: amount,
            available: user_info.reward_points,
        });
    }
    
    // 扣除积分
    user_info.reward_points -= amount;
    
    // 记录提取历史
    user_info.points_history.push(PointsRecord {
        points_change: Uint128::zero().saturating_sub(amount),
        reason: PointsReason::ManualAdjustment,
        timestamp: env.block.time.seconds(),
        related_user: None,
        event_id: Some("withdrawal".to_string()),
    });
    
    // 保存用户信息
    USER_MAP.save(deps.storage, user, &user_info)?;
    
    // 更新排行榜
    update_leaderboard(deps.storage, user, user_info.reward_points)?;
    
    Ok(Response::new()
        .add_attribute("action", "points_withdrawn")
        .add_attribute("user", user.to_string())
        .add_attribute("amount", amount.to_string())
        .add_attribute("remaining_points", user_info.reward_points.to_string()))
}
