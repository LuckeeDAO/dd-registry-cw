use cosmwasm_std::{Addr, Uint128, Decimal};
use cw_storage_plus::{Item, Map};
use serde::{Deserialize, Serialize};
use schemars::JsonSchema;
use std::str::FromStr;

// 用户信息结构体
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct UserInfo {
    pub recommender: Option<Addr>,
    pub direct_referrals: Vec<Addr>,
    pub reward_points: Uint128,
    pub registered_at: u64,
    pub last_active_at: u64,
    pub user_level: UserLevel,
    pub referral_stats: ReferralStats,
    pub points_history: Vec<PointsRecord>,
    pub status: UserStatus,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Hash, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum UserLevel {
    Bronze,
    Silver,
    Gold,
    Platinum,
}

impl UserLevel {
    pub fn from_referral_count(count: u32) -> Self {
        match count {
            0..=9 => UserLevel::Bronze,
            10..=49 => UserLevel::Silver,
            50..=99 => UserLevel::Gold,
            _ => UserLevel::Platinum,
        }
    }

    pub fn multiplier(&self) -> Decimal {
        match self {
            UserLevel::Bronze => Decimal::from_str("1.0").unwrap(),
            UserLevel::Silver => Decimal::from_str("1.2").unwrap(),
            UserLevel::Gold => Decimal::from_str("1.5").unwrap(),
            UserLevel::Platinum => Decimal::from_str("2.0").unwrap(),
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct ReferralStats {
    pub total_referrals: u32,
    pub active_referrals: u32,
    pub monthly_referrals: u32,
    pub last_referral_time: Option<u64>,
    pub success_rate: Decimal,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct PointsRecord {
    pub points_change: Uint128,
    pub reason: PointsReason,
    pub timestamp: u64,
    pub related_user: Option<Addr>,
    pub event_id: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum PointsReason {
    ReferralReward,
    LevelUpBonus,
    ActivityBonus,
    Penalty,
    ManualAdjustment,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum UserStatus {
    Active,
    Suspended,
    Banned,
}

// 系统配置
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct SystemConfig {
    pub enabled: bool,
    pub max_referral_depth: u32,
    pub referral_cooldown: u64,
    pub max_daily_referrals: u32,
    pub points_decay_period: u64,
    pub points_decay_rate: Decimal,
    pub min_withdrawal_amount: Uint128,
    pub admin: Addr,
    pub emergency_paused: bool,
}

// 积分规则
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct PointsRules {
    pub direct_referral_rate: Decimal,
    pub level_2_rate: Decimal,
    pub level_3_rate: Decimal,
    pub base_points: Uint128,
    pub level_multipliers: std::collections::HashMap<UserLevel, Decimal>,
    pub activity_rules: Vec<ActivityRule>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct ActivityRule {
    pub activity_type: String,
    pub reward_points: Uint128,
    pub enabled: bool,
    pub cooldown: u64,
}

// 存储定义
pub const CONFIG: Item<SystemConfig> = Item::new("config");
pub const POINTS_RULES: Item<PointsRules> = Item::new("points_rules");
pub const USER_MAP: Map<&Addr, UserInfo> = Map::new("user_map");
pub const REFERRAL_CHAIN: Map<&Addr, Addr> = Map::new("referral_chain");
pub const POINTS_LEADERBOARD: Map<u128, Addr> = Map::new("points_leaderboard");
pub const LEVEL_STATS: Map<UserLevel, u32> = Map::new("level_stats");
pub const REENTRANCY_LOCK: Item<bool> = Item::new("reentrancy_lock");