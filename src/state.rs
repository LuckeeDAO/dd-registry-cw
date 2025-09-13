use serde::{Deserialize, Serialize};
use cw_storage_plus::{Item, Map};
use crate::msg::DecisionMethod;

pub const ADMIN: Item<String> = Item::new("admin");

pub fn methods<'a>() -> Map<'a, String, DecisionMethod> { Map::new("methods") }


#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct GuardParamsState {
    pub min_reveal_window_sec: u64,
    pub max_reveal_window_sec: u64,
    pub min_timelock_sec: u64,
    pub max_drift_limit_bps: u16,
    pub require_sealed_commit: bool,
    pub require_forbid_subset: bool,
    pub require_linear_weighting: bool,
    pub require_default_ballot: bool,
}

pub const GUARDS: Item<GuardParamsState> = Item::new("guards");
