use cosmwasm_schema::{cw_serde, QueryResponses};
use cosmwasm_std::{Addr, Uint128};
use crate::state::{UserInfo, UserLevel, SystemConfig, PointsRules, PointsReason};

#[cw_serde]
pub struct InstantiateMsg {
    pub admin: String,
    pub config: SystemConfig,
    pub points_rules: PointsRules,
}

#[cw_serde]
pub enum ExecuteMsg {
    /// 用户注册
    Register {
        referrer: Option<String>,
    },
    
    /// 分配积分
    AllocateRewards {
        user: String,
        points: Uint128,
        reason: PointsReason,
        related_user: Option<String>,
        event_id: Option<String>,
    },
    
    /// 批量分配积分
    BatchAllocateRewards {
        rewards: Vec<RewardAllocation>,
    },
    
    /// 提取积分
    WithdrawPoints {
        amount: Uint128,
    },
    
    /// 更新系统配置
    UpdateConfig {
        config: SystemConfig,
    },
    
    /// 更新积分规则
    UpdatePointsRules {
        rules: PointsRules,
    },
    
    /// 紧急暂停
    EmergencyPause {
        paused: bool,
    },
}

#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
    /// 查询用户信息
    #[returns(UserInfoResponse)]
    GetUserInfo { user: String },
    
    /// 查询推荐人
    #[returns(ReferrerResponse)]
    GetReferrer { user: String },
    
    /// 查询直接推荐的下级
    #[returns(DirectReferralsResponse)]
    GetDirectReferrals {
        user: String,
        limit: Option<u32>,
        start_after: Option<String>,
    },
    
    /// 查询推荐链
    #[returns(ReferralChainResponse)]
    GetReferralChain {
        user: String,
        max_depth: Option<u32>,
    },
    
    /// 查询用户积分
    #[returns(UserPointsResponse)]
    GetUserPoints { user: String },
    
    /// 查询积分排行榜
    #[returns(LeaderboardResponse)]
    GetPointsLeaderboard {
        limit: Option<u32>,
        start_after: Option<String>,
    },
    
    /// 查询用户积分历史
    #[returns(PointsHistoryResponse)]
    GetPointsHistory {
        user: String,
        limit: Option<u32>,
        start_after: Option<u64>,
    },
    
    /// 查询系统配置
    #[returns(ConfigResponse)]
    GetConfig {},
    
    /// 查询积分规则
    #[returns(PointsRulesResponse)]
    GetPointsRules {},
    
    /// 查询用户等级统计
    #[returns(LevelStatsResponse)]
    GetLevelStats {},
    
    /// 验证推荐关系
    #[returns(ValidationResponse)]
    ValidateReferral {
        referrer: String,
        referee: String,
    },
}

// 响应结构体定义
#[cw_serde]
pub struct UserInfoResponse {
    pub user_info: UserInfo,
}

#[cw_serde]
pub struct ReferrerResponse {
    pub referrer: Option<Addr>,
}

#[cw_serde]
pub struct DirectReferralsResponse {
    pub referrals: Vec<Addr>,
    pub total: u32,
}

#[cw_serde]
pub struct ReferralChainResponse {
    pub chain: Vec<ReferralNode>,
    pub depth: u32,
}

#[cw_serde]
pub struct ReferralNode {
    pub user: Addr,
    pub level: u32,
    pub registered_at: u64,
}

#[cw_serde]
pub struct UserPointsResponse {
    pub points: Uint128,
    pub level: UserLevel,
    pub rank: Option<u32>,
}

#[cw_serde]
pub struct LeaderboardResponse {
    pub entries: Vec<LeaderboardEntry>,
    pub total: u32,
}

#[cw_serde]
pub struct LeaderboardEntry {
    pub user: Addr,
    pub points: Uint128,
    pub level: UserLevel,
    pub rank: u32,
}

#[cw_serde]
pub struct PointsHistoryResponse {
    pub records: Vec<crate::state::PointsRecord>,
    pub total: u32,
}

#[cw_serde]
pub struct ConfigResponse {
    pub config: SystemConfig,
}

#[cw_serde]
pub struct PointsRulesResponse {
    pub rules: PointsRules,
}

#[cw_serde]
pub struct LevelStatsResponse {
    pub stats: std::collections::HashMap<UserLevel, u32>,
    pub total_users: u32,
}

#[cw_serde]
pub struct ValidationResponse {
    pub is_valid: bool,
    pub reason: Option<String>,
    pub depth: Option<u32>,
}

#[cw_serde]
pub struct RewardAllocation {
    pub user: String,
    pub points: Uint128,
    pub reason: PointsReason,
    pub related_user: Option<String>,
    pub event_id: Option<String>,
}