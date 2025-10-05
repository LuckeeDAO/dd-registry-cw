pub mod contract;
pub mod error;
pub mod execute;
pub mod msg;
pub mod query;
pub mod state;
pub mod user;
pub mod referral;
pub mod points;
pub mod security;

pub use crate::error::ContractError;
pub use crate::msg::{ExecuteMsg, InstantiateMsg, QueryMsg};
pub use crate::state::{UserInfo, UserLevel, SystemConfig, PointsRules};

// 版本信息
pub const CONTRACT_NAME: &str = "dd-registry-cw";
pub const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");
pub const CONTRACT_SHA256SUM: &str = "unknown";

// 重新导出 cosmwasm_std 的常用类型
pub use cosmwasm_std::{
    entry_point, to_json_binary, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult,
};