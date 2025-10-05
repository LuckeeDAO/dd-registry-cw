use cosmwasm_std::testing::{mock_dependencies, mock_env, message_info};
use cosmwasm_std::{coins, from_json, Addr, Uint128, Decimal};
use std::str::FromStr;
use dd_registry_cw::contract::{instantiate, execute, query};
use dd_registry_cw::msg::{InstantiateMsg, ExecuteMsg, QueryMsg};
use dd_registry_cw::state::{SystemConfig, PointsRules, PointsReason};

#[test]
fn test_user_registration() {
    let mut deps = mock_dependencies();
    let env = mock_env();
    let info = message_info(&Addr::unchecked("user1"), &coins(1000, "uluna"));
    
    // 初始化合约
    let init_msg = InstantiateMsg {
        admin: "admin".to_string(),
        config: SystemConfig {
            enabled: true,
            max_referral_depth: 3,
            referral_cooldown: 3600,
            max_daily_referrals: 10,
            points_decay_period: 30,
            points_decay_rate: Decimal::from_str("0.01").unwrap(),
            min_withdrawal_amount: Uint128::from(1000u128),
            admin: Addr::unchecked("admin"),
            emergency_paused: false,
        },
        points_rules: PointsRules {
            direct_referral_rate: Decimal::from_str("0.5").unwrap(),
            level_2_rate: Decimal::from_str("0.2").unwrap(),
            level_3_rate: Decimal::from_str("0.1").unwrap(),
            base_points: Uint128::from(100u128),
            level_multipliers: std::collections::HashMap::new(),
            activity_rules: Vec::new(),
        },
    };
    
    let admin_info = message_info(&Addr::unchecked("admin"), &coins(1000, "uluna"));
    instantiate(deps.as_mut(), env.clone(), admin_info, init_msg).unwrap();
    
    // 测试用户注册
    let msg = ExecuteMsg::Register {
        referrer: None,
    };
    
    let res = execute(deps.as_mut(), env, info, msg).unwrap();
    assert_eq!(res.attributes.len(), 4);
    assert_eq!(res.attributes[0].key, "action");
    assert_eq!(res.attributes[0].value, "user_registered");
}

#[test]
fn test_referral_relation() {
    let mut deps = mock_dependencies();
    let env = mock_env();
    
    // 初始化合约
    let init_msg = InstantiateMsg {
        admin: "admin".to_string(),
        config: SystemConfig {
            enabled: true,
            max_referral_depth: 3,
            referral_cooldown: 3600,
            max_daily_referrals: 10,
            points_decay_period: 30,
            points_decay_rate: Decimal::from_str("0.01").unwrap(),
            min_withdrawal_amount: Uint128::from(1000u128),
            admin: Addr::unchecked("admin"),
            emergency_paused: false,
        },
        points_rules: PointsRules {
            direct_referral_rate: Decimal::from_str("0.5").unwrap(),
            level_2_rate: Decimal::from_str("0.2").unwrap(),
            level_3_rate: Decimal::from_str("0.1").unwrap(),
            base_points: Uint128::from(100u128),
            level_multipliers: std::collections::HashMap::new(),
            activity_rules: Vec::new(),
        },
    };
    
    let admin_info = message_info(&Addr::unchecked("admin"), &coins(1000, "uluna"));
    instantiate(deps.as_mut(), env.clone(), admin_info, init_msg).unwrap();
    
    // 注册推荐人
    let referrer_info = message_info(&Addr::unchecked("referrer"), &coins(1000, "uluna"));
    let register_referrer = ExecuteMsg::Register { referrer: None };
    execute(deps.as_mut(), env.clone(), referrer_info, register_referrer).unwrap();
    
    // 注册被推荐人
    let referee_info = message_info(&Addr::unchecked("referee"), &coins(1000, "uluna"));
    let register_referee = ExecuteMsg::Register {
        referrer: Some("referrer".to_string()),
    };
    
    let res = execute(deps.as_mut(), env, referee_info, register_referee).unwrap();
    assert_eq!(res.attributes[0].value, "user_registered");
}

#[test]
fn test_circular_referral_prevention() {
    let mut deps = mock_dependencies();
    let env = mock_env();
    
    // 初始化合约
    let init_msg = InstantiateMsg {
        admin: "admin".to_string(),
        config: SystemConfig {
            enabled: true,
            max_referral_depth: 3,
            referral_cooldown: 3600,
            max_daily_referrals: 10,
            points_decay_period: 30,
            points_decay_rate: Decimal::from_str("0.01").unwrap(),
            min_withdrawal_amount: Uint128::from(1000u128),
            admin: Addr::unchecked("admin"),
            emergency_paused: false,
        },
        points_rules: PointsRules {
            direct_referral_rate: Decimal::from_str("0.5").unwrap(),
            level_2_rate: Decimal::from_str("0.2").unwrap(),
            level_3_rate: Decimal::from_str("0.1").unwrap(),
            base_points: Uint128::from(100u128),
            level_multipliers: std::collections::HashMap::new(),
            activity_rules: Vec::new(),
        },
    };
    
    let admin_info = message_info(&Addr::unchecked("admin"), &coins(1000, "uluna"));
    instantiate(deps.as_mut(), env.clone(), admin_info, init_msg).unwrap();
    
    // 注册用户A
    let user_a_info = message_info(&Addr::unchecked("user_a"), &coins(1000, "uluna"));
    let register_a = ExecuteMsg::Register { referrer: None };
    execute(deps.as_mut(), env.clone(), user_a_info, register_a).unwrap();
    
    // 注册用户B，推荐人为A
    let user_b_info = message_info(&Addr::unchecked("user_b"), &coins(1000, "uluna"));
    let register_b = ExecuteMsg::Register {
        referrer: Some("user_a".to_string()),
    };
    execute(deps.as_mut(), env.clone(), user_b_info, register_b).unwrap();
    
    // 尝试让A推荐B（应该失败）
    let user_a_info = message_info(&Addr::unchecked("user_a"), &coins(1000, "uluna"));
    let register_circular = ExecuteMsg::Register {
        referrer: Some("user_b".to_string()),
    };
    
    let res = execute(deps.as_mut(), env, user_a_info, register_circular);
    assert!(res.is_err());
}

#[test]
fn test_points_allocation() {
    let mut deps = mock_dependencies();
    let env = mock_env();
    
    // 初始化合约
    let init_msg = InstantiateMsg {
        admin: "admin".to_string(),
        config: SystemConfig {
            enabled: true,
            max_referral_depth: 3,
            referral_cooldown: 3600,
            max_daily_referrals: 10,
            points_decay_period: 30,
            points_decay_rate: Decimal::from_str("0.01").unwrap(),
            min_withdrawal_amount: Uint128::from(1000u128),
            admin: Addr::unchecked("admin"),
            emergency_paused: false,
        },
        points_rules: PointsRules {
            direct_referral_rate: Decimal::from_str("0.5").unwrap(),
            level_2_rate: Decimal::from_str("0.2").unwrap(),
            level_3_rate: Decimal::from_str("0.1").unwrap(),
            base_points: Uint128::from(100u128),
            level_multipliers: std::collections::HashMap::new(),
            activity_rules: Vec::new(),
        },
    };
    
    let admin_info = message_info(&Addr::unchecked("admin"), &coins(1000, "uluna"));
    instantiate(deps.as_mut(), env.clone(), admin_info, init_msg).unwrap();
    
    // 注册用户
    let user_info = message_info(&Addr::unchecked("user"), &coins(1000, "uluna"));
    let register_user = ExecuteMsg::Register { referrer: None };
    execute(deps.as_mut(), env.clone(), user_info, register_user).unwrap();
    
    // 分配积分
    let admin_info = message_info(&Addr::unchecked("admin"), &coins(1000, "uluna"));
    let allocate_msg = ExecuteMsg::AllocateRewards {
        user: "user".to_string(),
        points: Uint128::from(100u128),
        reason: PointsReason::ActivityBonus,
        related_user: None,
        event_id: Some("test_event".to_string()),
    };
    
    let res = execute(deps.as_mut(), env.clone(), admin_info, allocate_msg).unwrap();
    assert_eq!(res.attributes[0].value, "points_allocated");
    
    // 查询用户积分
    let query_msg = QueryMsg::GetUserPoints {
        user: "user".to_string(),
    };
    let res = query(deps.as_ref(), env, query_msg).unwrap();
    let points_res: dd_registry_cw::msg::UserPointsResponse = from_json(&res).unwrap();
    
    assert_eq!(points_res.points, Uint128::from(100u128));
}
