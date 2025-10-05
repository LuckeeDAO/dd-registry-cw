use cosmwasm_std::{Addr, Deps, DepsMut};
use crate::state::{CONFIG, REENTRANCY_LOCK};
use crate::error::ContractError;

/// 检查管理员权限
pub fn check_admin_permission(
    deps: Deps,
    sender: &Addr,
) -> Result<(), ContractError> {
    let config = CONFIG.load(deps.storage)?;
    
    if sender != &config.admin {
        return Err(ContractError::Unauthorized {
            message: "Admin permission required".to_string(),
        });
    }
    
    Ok(())
}

/// 检查系统是否暂停
pub fn check_system_paused(
    deps: Deps,
) -> Result<(), ContractError> {
    let config = CONFIG.load(deps.storage)?;
    
    if config.emergency_paused {
        return Err(ContractError::SystemPaused);
    }
    
    Ok(())
}

/// 检查重入锁
pub fn check_reentrancy_lock(
    deps: Deps,
) -> Result<(), ContractError> {
    if let Ok(Some(true)) = REENTRANCY_LOCK.may_load(deps.storage) {
        return Err(ContractError::ReentrancyDetected);
    }
    
    Ok(())
}

/// 设置重入锁
pub fn set_reentrancy_lock(
    deps: DepsMut,
    locked: bool,
) -> Result<(), ContractError> {
    REENTRANCY_LOCK.save(deps.storage, &locked)?;
    Ok(())
}

/// 验证用户地址
pub fn validate_address(
    deps: Deps,
    address: &str,
) -> Result<Addr, ContractError> {
    deps.api.addr_validate(address)
        .map_err(|_| ContractError::InvalidParameter {
            parameter: "address".to_string(),
            value: address.to_string(),
        })
}

/// 检查冷却时间
pub fn check_cooldown(
    _deps: Deps,
    _user: &Addr,
    _cooldown_type: &str,
    _cooldown_duration: u64,
) -> Result<(), ContractError> {
    // 这里需要根据具体的冷却时间存储结构来实现
    // 暂时返回成功
    Ok(())
}
