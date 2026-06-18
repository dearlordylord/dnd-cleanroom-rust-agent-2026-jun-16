# Validation Report - T086 Battle Runtime Reducer Spine Contract

- Manifest source commit SHA: `1da73ecc2a230aecf2bf43d5c1669e557d47c5b3`
- Source branch inventory SHA: `7317a1acf0dce82ff0cf62330081ced4d8d77460134527048c7beb9a60b83beb`
- Machine-readable run ledger: `tasks/RUN_LEDGER.json`
- Selected driver: `cleanroom-input/qnt/battle-runtime/battle-runtime-reducer-spine-contract.mbt.qnt`
- Target replay evidence: `tasks/target-replay-evidence/T086-battle-runtime-reducer-spine-contract.json`

## Allowed Inputs Used

Allowed inputs used:

- `cleanroom-input/qnt/battle-runtime/battle-runtime-reducer-spine-contract.mbt.qnt`
- `cleanroom-input/qnt/battle-runtime/battle-runtime-magic-missile.mbt.qnt`
- `cleanroom-input/qnt/battle-runtime/battle-runtime-turn-advancement.qnt`
- `cleanroom-input/qnt/battle-runtime/battle-runtime-weapon-attack-skeleton.mbt.qnt`
- `cleanroom-input/qnt/battle-runtime/battle-runtime-hole-kinds.qnt`
- `cleanroom-input/qnt/battle-runtime/battle-runtime-witness-protocol.qnt`
- `cleanroom-input/raw/srd-5.2.1/Spells/Descriptions-M-P.md`
- `cleanroom-input/raw/srd-5.2.1/Playing-the-Game.md`
- `cleanroom-input/domain/UBIQUITOUS_LANGUAGE.md`

## Behavior Implemented

Behavior implemented:

The Rust battle spine now routes the copied reducer-spine witness sequence:
`start battle -> discover Magic Missile -> fill target allocation -> fill damage -> end turn -> discover Skeleton weapon attack -> fill target -> fill attack roll -> fill damage`.

The new production state is `BattleState.slot_spell_procedure`, a battle-owned slot-spell frontier used to carry Magic Missile allocation and damage through durable reducer state. The adapter maps this domain state into the QNT witness vocabulary; QNT stage, entrypoint, and protocol names remain quarantined in `src/qnt_adapters/battle_runtime_reducer_spine_contract.rs`.

## Branch Coverage

Generated branch coverage:

| Obligation | Target replay evidence | Diagnostic tests | Status |
| --- | --- | --- | --- |
| `cleanroom-input/qnt/battle-runtime/battle-runtime-reducer-spine-contract.mbt.qnt#step:doStartBattle` | `tasks/target-replay-evidence/T086-battle-runtime-reducer-spine-contract.json#T086-do-start-battle#step:doStartBattle` | `cargo test reducer_spine_contract_adapter_replays_all_branches` | `covered` |
| `cleanroom-input/qnt/battle-runtime/battle-runtime-reducer-spine-contract.mbt.qnt#step:doDiscoverSlotSpell` | `tasks/target-replay-evidence/T086-battle-runtime-reducer-spine-contract.json#T086-do-discover-slot-spell#step:doDiscoverSlotSpell` | `cargo test reducer_spine_contract_adapter_replays_all_branches` | `covered` |
| `cleanroom-input/qnt/battle-runtime/battle-runtime-reducer-spine-contract.mbt.qnt#step:doResolveSlotSpellTargets` | `tasks/target-replay-evidence/T086-battle-runtime-reducer-spine-contract.json#T086-do-resolve-slot-spell-targets#step:doResolveSlotSpellTargets` | `cargo test reducer_spine_contract_adapter_replays_all_branches` | `covered` |
| `cleanroom-input/qnt/battle-runtime/battle-runtime-reducer-spine-contract.mbt.qnt#step:doResolveSlotSpellDamage` | `tasks/target-replay-evidence/T086-battle-runtime-reducer-spine-contract.json#T086-do-resolve-slot-spell-damage#step:doResolveSlotSpellDamage` | `cargo test reducer_spine_contract_adapter_replays_all_branches` | `covered` |
| `cleanroom-input/qnt/battle-runtime/battle-runtime-reducer-spine-contract.mbt.qnt#step:doEndTurnToTarget` | `tasks/target-replay-evidence/T086-battle-runtime-reducer-spine-contract.json#T086-do-end-turn-to-target#step:doEndTurnToTarget` | `cargo test reducer_spine_contract_adapter_replays_all_branches` | `covered` |
| `cleanroom-input/qnt/battle-runtime/battle-runtime-reducer-spine-contract.mbt.qnt#step:doDiscoverWeaponAttack` | `tasks/target-replay-evidence/T086-battle-runtime-reducer-spine-contract.json#T086-do-discover-weapon-attack#step:doDiscoverWeaponAttack` | `cargo test reducer_spine_contract_adapter_replays_all_branches` | `covered` |
| `cleanroom-input/qnt/battle-runtime/battle-runtime-reducer-spine-contract.mbt.qnt#step:doResolveWeaponTarget` | `tasks/target-replay-evidence/T086-battle-runtime-reducer-spine-contract.json#T086-do-resolve-weapon-target#step:doResolveWeaponTarget` | `cargo test reducer_spine_contract_adapter_replays_all_branches` | `covered` |
| `cleanroom-input/qnt/battle-runtime/battle-runtime-reducer-spine-contract.mbt.qnt#step:doResolveWeaponAttackHit` | `tasks/target-replay-evidence/T086-battle-runtime-reducer-spine-contract.json#T086-do-resolve-weapon-attack-hit#step:doResolveWeaponAttackHit` | `cargo test reducer_spine_contract_adapter_replays_all_branches` | `covered` |
| `cleanroom-input/qnt/battle-runtime/battle-runtime-reducer-spine-contract.mbt.qnt#step:doResolveWeaponDamage` | `tasks/target-replay-evidence/T086-battle-runtime-reducer-spine-contract.json#T086-do-resolve-weapon-damage#step:doResolveWeaponDamage` | `cargo test reducer_spine_contract_adapter_replays_all_branches` | `covered` |

## Harness Artifacts

Harness artifacts:

- `tasks/START_GATE.json`
- `tasks/ENGINE_DEPTH_MANIFEST.json`
- `tasks/STATE_OWNER_MANIFEST.json`
- `tasks/REVIEW_LOOP.json`
- `tasks/DECIDER_DECISION.json`
- `tasks/target-replay-evidence/T086-battle-runtime-reducer-spine-contract.json`

Target replay evidence:

- Evidence file: `tasks/target-replay-evidence/T086-battle-runtime-reducer-spine-contract.json`

## Verification Results

Verification results:

- `cargo fmt --check` passed.
- `cargo test` passed: 170 tests.
- `cargo clippy --all-targets -- -D warnings` passed.
- `node scripts/check-cleanroom-harness.cjs` still fails repo-wide because this dirty rehearsal repo has pre-existing accounting debt: `tasks/RUN_LEDGER.json` and `tasks/history/<taskId>/*.json` are absent, historical evidence files are not ledger-accounted, and the stale historical adapter/source-scan denominator is outside this task. This task does not fabricate ledger entries for stale historical evidence; see `tasks/DIRTY_REHEARSAL_STATUS.md`.

## Remaining Gaps

Remaining gaps:

- The selected reducer-spine contract driver has task-local replay evidence for all nine in-scope branches.
- Repo-wide cleanroom harness acceptance still requires the separate ledger/history and historical adapter-denominator repair documented in `tasks/DIRTY_REHEARSAL_STATUS.md`.
