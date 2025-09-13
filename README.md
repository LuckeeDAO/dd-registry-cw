# dd-registry-cw

Luckee decentralized decision registry (CosmWasm). Enforces NFR-D1..D5 via query-based validation and configurable guards.

## Interface
- Instantiate: `{ admin }` (sets default guard parameters)
- Execute:
  - `RegisterMethod { m: DecisionMethod }` -> store method and derive `method_id`
  - `UpdateGuards { guards: GuardParams }` (admin)
  - `UpdateAdmin { admin }` (admin)
- Query:
  - `ContractInterface {}` -> `{ interface_id: "luckee.dd.registry.v1", version }`
  - `IsMethodDecentralized { method_id }` -> `{ ok, failures[] }` (uses current guards)
  - `GetMethod { method_id }` -> `DecisionMethod`
  - `ListMethods { start_after?, limit? }` -> `[(method_id, DecisionMethod)]`
  - `CalcMethodId { m }` -> `String`
  - `GetGuards {}` -> `GuardParams`

## Integration (luckee-question-nft)
1) Store `decision_registry_addr` in config.
2) Gate all write ops:
   - Call `ContractInterface{}` and require `interface_id == "luckee.dd.registry.v1"`
   - Call `IsMethodDecentralized{ method_id }` and require `ok == true`

## Build
- `cargo wasm` (or use workspace-optimizer)

## Test
- `cargo test -q`

## Governance
- `admin` controls `UpdateGuards`/`UpdateAdmin` and method registration policy; recommend multisig/governance with timelock.
