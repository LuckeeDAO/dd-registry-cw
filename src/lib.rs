use cosmwasm_std::{entry_point, Deps, DepsMut, Env, MessageInfo, Response, StdResult, to_json_binary, Binary, Order};
use cosmwasm_std::{attr, StdError};
use cw2::set_contract_version;
use sha2::{Digest, Sha256};

mod msg;
mod state;

use crate::msg::*;
use crate::state::*;

const CONTRACT_NAME: &str = "crates.io/luckee-dd-registry";
const CONTRACT_VERSION: &str = "0.1.0";
const INTERFACE_ID: &str = "luckee.dd.registry.v1";

#[entry_point]
pub fn instantiate(deps: DepsMut, _env: Env, info: MessageInfo, msg: InstantiateMsg) -> StdResult<Response> {
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;
    let admin = msg.admin.unwrap_or(info.sender.to_string());
    ADMIN.save(deps.storage, &admin)?;
    // default guards
    let guards = GuardParamsState {
        min_reveal_window_sec: 600,
        max_reveal_window_sec: 7 * 24 * 3600,
        min_timelock_sec: 3600,
        max_drift_limit_bps: 10_000,
        require_sealed_commit: true,
        require_forbid_subset: true,
        require_linear_weighting: true,
        require_default_ballot: true,
    };
    GUARDS.save(deps.storage, &guards)?;
    Ok(Response::new().add_attributes(vec![attr("action", "instantiate"), attr("admin", admin)]))
}

#[entry_point]
pub fn execute(deps: DepsMut, _env: Env, info: MessageInfo, msg: ExecuteMsg) -> StdResult<Response> {
    match msg {
        ExecuteMsg::RegisterMethod { m } => register_method(deps, info, m),
        ExecuteMsg::UpdateGuards { guards } => update_guards(deps, info, guards),
        ExecuteMsg::UpdateAdmin { admin } => update_admin(deps, info, admin),
    }
}

fn update_admin(deps: DepsMut, info: MessageInfo, admin: String) -> StdResult<Response> {
    let cur = ADMIN.load(deps.storage)?;
    if info.sender.as_str() != cur {
        return Err(StdError::generic_err("unauthorized"));
    }
    ADMIN.save(deps.storage, &admin)?;
    Ok(Response::new().add_attributes(vec![attr("action", "update_admin"), attr("admin", admin)]))
}

fn update_guards(deps: DepsMut, info: MessageInfo, guards: GuardParams) -> StdResult<Response> {
    let admin = ADMIN.load(deps.storage)?;
    if info.sender.as_str() != admin {
        return Err(StdError::generic_err("unauthorized"));
    }
    let new_state = GuardParamsState {
        min_reveal_window_sec: guards.min_reveal_window_sec,
        max_reveal_window_sec: guards.max_reveal_window_sec,
        min_timelock_sec: guards.min_timelock_sec,
        max_drift_limit_bps: guards.max_drift_limit_bps,
        require_sealed_commit: guards.require_sealed_commit,
        require_forbid_subset: guards.require_forbid_subset,
        require_linear_weighting: guards.require_linear_weighting,
        require_default_ballot: guards.require_default_ballot,
    };
    GUARDS.save(deps.storage, &new_state)?;
    Ok(Response::new().add_attribute("action", "update_guards"))
}

fn register_method(deps: DepsMut, info: MessageInfo, m: DecisionMethod) -> StdResult<Response> {
    let admin = ADMIN.load(deps.storage)?;
    if info.sender.as_str() != admin {
        return Err(StdError::generic_err("unauthorized"));
    }
    let method_id = calc_method_id(&m);
    let idx = methods();
    idx.save(deps.storage, method_id.clone(), &m)?;
    Ok(Response::new().add_attributes(vec![attr("action", "register_method"), attr("method_id", method_id)]))
}

fn calc_method_id(m: &DecisionMethod) -> String {
    let payload = format!(
        "{}|{}|{:?}|{:?}|{:?}|{}|{}|{}|{}|{}",
        m.ft_token,
        m.linear_fungible_weighting,
        m.aggregation,
        m.commit_scheme,
        m.default_ballot,
        m.reveal_window_sec,
        m.timelock_sec,
        m.audit_schema_cid,
        m.drift_limit_bps,
        m.forbid_subset_quorums
    );
    let mut hasher = Sha256::new();
    hasher.update(payload.as_bytes());
    let digest = hasher.finalize();
    hex::encode(digest)
}

#[entry_point]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::GetMethod { method_id } => {
            let idx = methods();
            to_json_binary(&idx.load(deps.storage, method_id)?)
        }
        QueryMsg::ListMethods { start_after, limit } => {
            let idx = methods();
            let start = start_after.map(|k| cw_storage_plus::Bound::exclusive(k));
            let take = limit.unwrap_or(50).min(200) as usize;
            let items: StdResult<Vec<(String, DecisionMethod)>> = idx
                .range(deps.storage, start, None, Order::Ascending)
                .take(take)
                .collect();
            to_json_binary(&items?)
        }
        QueryMsg::IsMethodDecentralized { method_id } => {
            let idx = methods();
            let m = idx.load(deps.storage, method_id)?;
            let guards = GUARDS.load(deps.storage)?;
            let resp = is_method_decentralized(&m, &guards);
            to_json_binary(&resp)
        }
        QueryMsg::ContractInterface {} => to_json_binary(&ContractInterfaceResp { interface_id: INTERFACE_ID.to_string(), version: CONTRACT_VERSION.to_string() }),
        QueryMsg::CalcMethodId { m } => to_json_binary(&calc_method_id(&m)),
        QueryMsg::GetGuards {} => to_json_binary(&GUARDS.load(deps.storage)?),
    }
}

fn is_method_decentralized(m: &DecisionMethod, g: &GuardParamsState) -> IsMethodDecentralizedResp {
    let mut failures: Vec<String> = vec![];
    if m.ft_token.is_empty() || (g.require_linear_weighting && !m.linear_fungible_weighting) { failures.push("D1: invalid FT/weighting".into()); }
    if m.audit_schema_cid.trim().is_empty() { failures.push("D2: missing audit_schema_cid".into()); }
    if m.timelock_sec < g.min_timelock_sec { failures.push("D3: timelock_sec too small".into()); }
    if matches!(m.aggregation, AggregationKind::None) { failures.push("D4: aggregation=None".into()); }
    if g.require_forbid_subset && !m.forbid_subset_quorums { failures.push("D4: forbid_subset_quorums=false".into()); }
    if g.require_default_ballot && matches!(m.default_ballot, DefaultBallotMode::None) { failures.push("D4: default_ballot=None".into()); }
    if g.require_sealed_commit && !matches!(m.commit_scheme, CommitScheme::SealedCommitReveal) { failures.push("D5: commit_scheme != sealed".into()); }
    if m.reveal_window_sec < g.min_reveal_window_sec || m.reveal_window_sec > g.max_reveal_window_sec { failures.push("D5: reveal_window out of range".into()); }
    if m.drift_limit_bps == 0 || m.drift_limit_bps > g.max_drift_limit_bps { failures.push("D5: drift_limit_bps out of range".into()); }
    IsMethodDecentralizedResp { ok: failures.is_empty(), failures }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn guards() -> GuardParamsState {
        GuardParamsState {
            min_reveal_window_sec: 600,
            max_reveal_window_sec: 7 * 24 * 3600,
            min_timelock_sec: 3600,
            max_drift_limit_bps: 10_000,
            require_sealed_commit: true,
            require_forbid_subset: true,
            require_linear_weighting: true,
            require_default_ballot: true,
        }
    }

    fn good_method() -> DecisionMethod {
        DecisionMethod {
            ft_token: "cw20:token".to_string(),
            linear_fungible_weighting: true,
            aggregation: AggregationKind::ScoreSum,
            commit_scheme: CommitScheme::SealedCommitReveal,
            default_ballot: DefaultBallotMode::DefaultVector,
            reveal_window_sec: 3600,
            timelock_sec: 86400,
            audit_schema_cid: "ipfs://schema".to_string(),
            drift_limit_bps: 100,
            forbid_subset_quorums: true,
        }
    }

    #[test]
    fn method_id_is_deterministic() {
        let m1 = good_method();
        let m2 = good_method();
        let id1 = calc_method_id(&m1);
        let id2 = calc_method_id(&m2);
        assert_eq!(id1, id2);
        assert!(!id1.is_empty());
    }

    #[test]
    fn dd_checks_pass_for_good_method() {
        let m = good_method();
        let resp = is_method_decentralized(&m, &guards());
        assert!(resp.ok, "should pass: {:?}", resp.failures);
    }

    #[test]
    fn dd_checks_fail_ranges() {
        let g = guards();
        let mut m = good_method();
        m.reveal_window_sec = 0;
        let r = is_method_decentralized(&m, &g);
        assert!(!r.ok);
        assert!(r.failures.iter().any(|f| f.contains("reveal_window")));
    }
}
