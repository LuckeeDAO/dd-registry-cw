use cosmwasm_std::{Uint128, Deps, StdResult, Order};
use crate::msg::{
    UserInfoResponse, ReferrerResponse, DirectReferralsResponse, ReferralChainResponse,
    ReferralNode, UserPointsResponse, LeaderboardResponse, LeaderboardEntry,
    PointsHistoryResponse, ConfigResponse, PointsRulesResponse, LevelStatsResponse,
    ValidationResponse,
};
use crate::state::{CONFIG, POINTS_RULES, USER_MAP, REFERRAL_CHAIN, POINTS_LEADERBOARD};

/// 查询用户信息
pub fn query_user_info(deps: Deps, user: String) -> StdResult<UserInfoResponse> {
    let user_addr = deps.api.addr_validate(&user)?;
    let user_info = USER_MAP.load(deps.storage, &user_addr)?;
    
    Ok(UserInfoResponse { user_info })
}

/// 查询推荐人
pub fn query_referrer(deps: Deps, user: String) -> StdResult<ReferrerResponse> {
    let user_addr = deps.api.addr_validate(&user)?;
    let user_info = USER_MAP.load(deps.storage, &user_addr)?;
    
    Ok(ReferrerResponse {
        referrer: user_info.recommender,
    })
}

/// 查询直接推荐的下级
pub fn query_direct_referrals(
    deps: Deps,
    user: String,
    limit: Option<u32>,
    start_after: Option<String>,
) -> StdResult<DirectReferralsResponse> {
    let user_addr = deps.api.addr_validate(&user)?;
    let user_info = USER_MAP.load(deps.storage, &user_addr)?;
    
    let limit = limit.unwrap_or(100);
    let mut referrals = user_info.direct_referrals;
    
    // 应用分页
    if let Some(start) = start_after {
        let start_addr = deps.api.addr_validate(&start)?;
        if let Some(pos) = referrals.iter().position(|addr| addr == &start_addr) {
            referrals = referrals.into_iter().skip(pos + 1).collect();
        }
    }
    
    let total = referrals.len() as u32;
    referrals.truncate(limit as usize);
    
    Ok(DirectReferralsResponse {
        referrals,
        total,
    })
}

/// 查询推荐链
pub fn query_referral_chain(
    deps: Deps,
    user: String,
    max_depth: Option<u32>,
) -> StdResult<ReferralChainResponse> {
    let user_addr = deps.api.addr_validate(&user)?;
    let config = CONFIG.load(deps.storage)?;
    let max_depth = max_depth.unwrap_or(config.max_referral_depth);
    
    let mut chain = Vec::new();
    let mut current = user_addr.clone();
    let mut depth = 0;
    
    while depth < max_depth {
        if let Ok(Some(parent)) = REFERRAL_CHAIN.may_load(deps.storage, &current) {
            let parent_info = USER_MAP.load(deps.storage, &parent)?;
            chain.push(ReferralNode {
                user: parent.clone(),
                level: depth + 1,
                registered_at: parent_info.registered_at,
            });
            current = parent;
            depth += 1;
        } else {
            break;
        }
    }
    
    Ok(ReferralChainResponse {
        chain,
        depth,
    })
}

/// 查询用户积分
pub fn query_user_points(deps: Deps, user: String) -> StdResult<UserPointsResponse> {
    let user_addr = deps.api.addr_validate(&user)?;
    let user_info = USER_MAP.load(deps.storage, &user_addr)?;
    
    // 计算排名
    let rank = calculate_user_rank(deps, &user_info.reward_points)?;
    
    Ok(UserPointsResponse {
        points: user_info.reward_points,
        level: user_info.user_level,
        rank,
    })
}

/// 查询积分排行榜
pub fn query_points_leaderboard(
    deps: Deps,
    limit: Option<u32>,
    _start_after: Option<String>,
) -> StdResult<LeaderboardResponse> {
    let limit = limit.unwrap_or(100);
    let mut entries = Vec::new();
    let mut rank = 1;
    
    // 按积分降序排列
    let range = POINTS_LEADERBOARD
        .range(deps.storage, None, None, Order::Descending)
        .take(limit as usize);
    
    for result in range {
        let (points, user) = result?;
        let user_info = USER_MAP.load(deps.storage, &user)?;
        entries.push(LeaderboardEntry {
            user: user.clone(),
            points: Uint128::from(points),
            level: user_info.user_level,
            rank,
        });
        rank += 1;
    }
    
    Ok(LeaderboardResponse {
        entries: entries.clone(),
        total: entries.len() as u32,
    })
}

/// 查询用户积分历史
pub fn query_points_history(
    deps: Deps,
    user: String,
    limit: Option<u32>,
    start_after: Option<u64>,
) -> StdResult<PointsHistoryResponse> {
    let user_addr = deps.api.addr_validate(&user)?;
    let user_info = USER_MAP.load(deps.storage, &user_addr)?;
    
    let limit = limit.unwrap_or(100);
    let mut records = user_info.points_history;
    
    // 应用分页
    if let Some(start) = start_after {
        records = records.into_iter()
            .filter(|record| record.timestamp > start)
            .collect();
    }
    
    let total = records.len() as u32;
    records.truncate(limit as usize);
    
    Ok(PointsHistoryResponse {
        records,
        total,
    })
}

/// 查询系统配置
pub fn query_config(deps: Deps) -> StdResult<ConfigResponse> {
    let config = CONFIG.load(deps.storage)?;
    Ok(ConfigResponse { config })
}

/// 查询积分规则
pub fn query_points_rules(deps: Deps) -> StdResult<PointsRulesResponse> {
    let rules = POINTS_RULES.load(deps.storage)?;
    Ok(PointsRulesResponse { rules })
}

/// 查询用户等级统计
pub fn query_level_stats(deps: Deps) -> StdResult<LevelStatsResponse> {
    let mut stats = std::collections::HashMap::new();
    let mut total_users = 0;
    
    // 遍历所有用户统计等级
    let users: Vec<_> = USER_MAP
        .range(deps.storage, None, None, Order::Ascending)
        .collect::<StdResult<Vec<_>>>()?;
    
    for (_, user_info) in users {
        let count = stats.entry(user_info.user_level).or_insert(0);
        *count += 1;
        total_users += 1;
    }
    
    Ok(LevelStatsResponse {
        stats,
        total_users,
    })
}

/// 验证推荐关系
pub fn query_validate_referral(
    deps: Deps,
    referrer: String,
    referee: String,
) -> StdResult<ValidationResponse> {
    let referrer_addr = deps.api.addr_validate(&referrer)?;
    let referee_addr = deps.api.addr_validate(&referee)?;
    
    // 检查推荐人是否存在
    if !USER_MAP.has(deps.storage, &referrer_addr) {
        return Ok(ValidationResponse {
            is_valid: false,
            reason: Some("Referrer not registered".to_string()),
            depth: None,
        });
    }
    
    // 检查被推荐人是否已注册
    if USER_MAP.has(deps.storage, &referee_addr) {
        return Ok(ValidationResponse {
            is_valid: false,
            reason: Some("Referee already registered".to_string()),
            depth: None,
        });
    }
    
    // 检查循环推荐
    match crate::referral::check_circular_referral(deps, &referrer_addr, &referee_addr) {
        Ok(_) => Ok(ValidationResponse {
            is_valid: true,
            reason: None,
            depth: Some(1),
        }),
        Err(_) => Ok(ValidationResponse {
            is_valid: false,
            reason: Some("Circular referral detected".to_string()),
            depth: None,
        }),
    }
}

/// 计算用户排名
fn calculate_user_rank(deps: Deps, points: &Uint128) -> StdResult<Option<u32>> {
    let mut rank = 1;
    
    // 遍历排行榜计算排名
    let range = POINTS_LEADERBOARD
        .range(deps.storage, None, None, Order::Descending);
    
    for result in range {
        let (leaderboard_points, _) = result?;
        if Uint128::from(leaderboard_points) > *points {
            rank += 1;
        } else {
            break;
        }
    }
    
    Ok(Some(rank))
}
