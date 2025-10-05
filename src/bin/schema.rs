use cosmwasm_schema::{export_schema, remove_schemas, schema_for};
use dd_registry_cw::msg::{ExecuteMsg, InstantiateMsg, QueryMsg};
use dd_registry_cw::msg::{
    UserInfoResponse, ReferrerResponse, DirectReferralsResponse, ReferralChainResponse,
    UserPointsResponse, LeaderboardResponse, PointsHistoryResponse, ConfigResponse,
    PointsRulesResponse, LevelStatsResponse, ValidationResponse, RewardAllocation,
};

fn main() {
    let out_dir = std::env::var("OUT_DIR").unwrap();
    let out_dir = std::path::Path::new(&out_dir).parent().unwrap().parent().unwrap().parent().unwrap();
    let schema_dir = out_dir.join("schema");

    // 创建 schema 目录
    std::fs::create_dir_all(&schema_dir).unwrap();

    // 移除旧的 schema 文件
    remove_schemas(&schema_dir).unwrap();

    // 生成新的 schema 文件
    export_schema(&schema_for!(InstantiateMsg), &schema_dir);
    export_schema(&schema_for!(ExecuteMsg), &schema_dir);
    export_schema(&schema_for!(QueryMsg), &schema_dir);
    
    // 响应类型
    export_schema(&schema_for!(UserInfoResponse), &schema_dir);
    export_schema(&schema_for!(ReferrerResponse), &schema_dir);
    export_schema(&schema_for!(DirectReferralsResponse), &schema_dir);
    export_schema(&schema_for!(ReferralChainResponse), &schema_dir);
    export_schema(&schema_for!(UserPointsResponse), &schema_dir);
    export_schema(&schema_for!(LeaderboardResponse), &schema_dir);
    export_schema(&schema_for!(PointsHistoryResponse), &schema_dir);
    export_schema(&schema_for!(ConfigResponse), &schema_dir);
    export_schema(&schema_for!(PointsRulesResponse), &schema_dir);
    export_schema(&schema_for!(LevelStatsResponse), &schema_dir);
    export_schema(&schema_for!(ValidationResponse), &schema_dir);
    export_schema(&schema_for!(RewardAllocation), &schema_dir);

    println!("Schema files generated in: {:?}", schema_dir);
}
