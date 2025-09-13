use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub enum AggregationKind { None, ScoreSum, RankAggregation, Quadratic, UniformPriceClear }

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub enum CommitScheme { None, SealedCommitReveal }

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub enum DefaultBallotMode { None, DefaultVector, Delegate }

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct DecisionMethod {
    pub ft_token: String,
    pub linear_fungible_weighting: bool,
    pub aggregation: AggregationKind,
    pub commit_scheme: CommitScheme,
    pub default_ballot: DefaultBallotMode,
    pub reveal_window_sec: u64,
    pub timelock_sec: u64,
    pub audit_schema_cid: String,
    pub drift_limit_bps: u16,
    pub forbid_subset_quorums: bool,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct GuardParams {
    pub min_reveal_window_sec: u64,
    pub max_reveal_window_sec: u64,
    pub min_timelock_sec: u64,
    pub max_drift_limit_bps: u16,
    pub require_sealed_commit: bool,
    pub require_forbid_subset: bool,
    pub require_linear_weighting: bool,
    pub require_default_ballot: bool,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct InstantiateMsg {
    pub admin: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub enum ExecuteMsg {
    RegisterMethod { m: DecisionMethod },
    UpdateGuards { guards: GuardParams },
    UpdateAdmin { admin: String },
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub enum QueryMsg {
    GetMethod { method_id: String },
    ListMethods { start_after: Option<String>, limit: Option<u32> },
    IsMethodDecentralized { method_id: String },
    ContractInterface {},
    CalcMethodId { m: DecisionMethod },
    GetGuards {},
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct IsMethodDecentralizedResp {
    pub ok: bool,
    pub failures: Vec<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct ContractInterfaceResp {
    pub interface_id: String,
    pub version: String,
}
