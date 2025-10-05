use cosmwasm_std::Uint128;
use thiserror::Error;

#[derive(Error, Debug, PartialEq)]
pub enum ContractError {
    #[error("Unauthorized: {message}")]
    Unauthorized { message: String },

    #[error("User not registered: {user}")]
    UserNotRegistered { user: String },

    #[error("User already registered: {user}")]
    UserAlreadyRegistered { user: String },

    #[error("Circular referral detected: {referrer} -> {referee}")]
    CircularReferral { referrer: String, referee: String },

    #[error("Invalid referrer: {referrer}")]
    InvalidReferrer { referrer: String },

    #[error("Insufficient points: required {required}, available {available}")]
    InsufficientPoints { required: Uint128, available: Uint128 },

    #[error("System is paused")]
    SystemPaused,

    #[error("Reentrancy attack detected")]
    ReentrancyDetected,

    #[error("Invalid parameter {parameter}: {value}")]
    InvalidParameter { parameter: String, value: String },

    #[error("Limit exceeded: {limit_type} limit {limit_value}, actual {actual_value}")]
    LimitExceeded { limit_type: String, limit_value: u32, actual_value: u32 },

    #[error("Cooldown not reached: {cooldown_type}, remaining {remaining_time}s")]
    CooldownNotReached { cooldown_type: String, remaining_time: u64 },

    #[error("System error: {message}")]
    SystemError { message: String },
}

impl From<ContractError> for cosmwasm_std::StdError {
    fn from(err: ContractError) -> Self {
        cosmwasm_std::StdError::generic_err(err.to_string())
    }
}

impl From<cosmwasm_std::StdError> for ContractError {
    fn from(err: cosmwasm_std::StdError) -> Self {
        ContractError::SystemError {
            message: err.to_string(),
        }
    }
}
