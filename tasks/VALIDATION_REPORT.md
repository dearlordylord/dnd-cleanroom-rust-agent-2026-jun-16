# Validation Report

## Work Loop Status

- Current manifest source commit SHA: `829aee6441d76a921c9d9c14a0d0221062975334`
- Source branch inventory SHA: `0a5eaa1f6f79fddbe441dc94500a0dac5644ba7fc392fc6baa3d44da1f2e3248`
- Scope file: `tasks/LEVEL_1_2_SCOPE.md`
- Work Loop instructions: `tasks/WORK_LOOP.md`
- Machine-readable run ledger: `tasks/RUN_LEDGER.json`
- Last completed current-snapshot queued branch set: `<none>`
- Next queued driver: `cleanroom-input/qnt/character-creation-runtime/character-creation-class-feature-projections.mbt.qnt`
- Next task id: `T001`

Completion rule: a queued branch set is complete only when this report has an
entry that names the exact `.mbt.qnt` driver, records the current manifest
source commit SHA, records the current source branch inventory SHA, lists the
allowed inputs used, renders branch coverage from harness-generated target
replay evidence, and records verification results. Entries with older manifest
source commit SHAs or inventory SHAs are historical unless they include a
current-snapshot revalidation note.

## T074: Rule-Core Stat-Block Controls Transition Diagnostic

- Manifest source commit SHA: `829aee6441d76a921c9d9c14a0d0221062975334`
- Source branch inventory SHA: `0a5eaa1f6f79fddbe441dc94500a0dac5644ba7fc392fc6baa3d44da1f2e3248`
- Driver: `cleanroom-input/qnt/battle-runtime/rule-core-stat-block-controls.mbt.qnt`
- Branch obligations:
  - `step:doEndTurnClosesDispatches`
  - `step:doMoveDuringDispatch`
  - `step:doRejectBonusActionDuringDispatch`
  - `step:doRejectOrdinaryActionDuringDispatch`
  - `step:doResolvePrimaryDispatch`
  - `step:doResolveSecondaryDispatch`
  - `step:doStartMultiattack`
- Allowed inputs used:
  - `cleanroom-input/MANIFEST.md`
  - `cleanroom-input/branch-coverage/source-branch-inventory.json`
  - `cleanroom-input/qnt/battle-runtime/rule-core-stat-block-controls.mbt.qnt`
  - `cleanroom-input/qnt/shared-algebras/proofs/rule-core/stat-block-controls.qnt`
  - `cleanroom-input/raw/srd-5.2.1/Monsters/Overview.md`
  - `cleanroom-input/raw/srd-5.2.1/Playing-the-Game.md`

Behavior implemented:

- `src/rules/rule_core_stat_block_controls.rs` now exposes a transition-shaped
  stat-block multiattack control API derived from `stat-block-controls.qnt`:
  start a profile, resolve a permitted dispatch/movement/end-turn subject, and
  reject bonus or ordinary actions while continuation dispatches remain open.
- `src/qnt_adapters/rule_core_stat_block_controls_mbt.rs` routes observed T074
  replay through `replay_observed_action_through_control_transition`; the
  expected side remains the branch witness projection.

Generated branch coverage:

| Obligation | Target replay evidence | Diagnostic tests | Status |
| --- | --- | --- | --- |
| `cleanroom-input/qnt/battle-runtime/rule-core-stat-block-controls.mbt.qnt#step:doEndTurnClosesDispatches` | `tasks/target-replay-evidence/T074-rule-core-stat-block-controls.json#T074-end-turn-closes-dispatches#step:doEndTurnClosesDispatches` | `cargo test stat_block_controls` | `covered` |
| `cleanroom-input/qnt/battle-runtime/rule-core-stat-block-controls.mbt.qnt#step:doMoveDuringDispatch` | `tasks/target-replay-evidence/T074-rule-core-stat-block-controls.json#T074-move-during-dispatch#step:doMoveDuringDispatch` | `cargo test stat_block_controls` | `covered` |
| `cleanroom-input/qnt/battle-runtime/rule-core-stat-block-controls.mbt.qnt#step:doRejectBonusActionDuringDispatch` | `tasks/target-replay-evidence/T074-rule-core-stat-block-controls.json#T074-reject-bonus-action-during-dispatch#step:doRejectBonusActionDuringDispatch` | `cargo test stat_block_controls` | `covered` |
| `cleanroom-input/qnt/battle-runtime/rule-core-stat-block-controls.mbt.qnt#step:doRejectOrdinaryActionDuringDispatch` | `tasks/target-replay-evidence/T074-rule-core-stat-block-controls.json#T074-reject-ordinary-action-during-dispatch#step:doRejectOrdinaryActionDuringDispatch` | `cargo test stat_block_controls` | `covered` |
| `cleanroom-input/qnt/battle-runtime/rule-core-stat-block-controls.mbt.qnt#step:doResolvePrimaryDispatch` | `tasks/target-replay-evidence/T074-rule-core-stat-block-controls.json#T074-resolve-primary-dispatch#step:doResolvePrimaryDispatch` | `cargo test stat_block_controls` | `covered` |
| `cleanroom-input/qnt/battle-runtime/rule-core-stat-block-controls.mbt.qnt#step:doResolveSecondaryDispatch` | `tasks/target-replay-evidence/T074-rule-core-stat-block-controls.json#T074-resolve-secondary-dispatch#step:doResolveSecondaryDispatch` | `cargo test stat_block_controls` | `covered` |
| `cleanroom-input/qnt/battle-runtime/rule-core-stat-block-controls.mbt.qnt#step:doStartMultiattack` | `tasks/target-replay-evidence/T074-rule-core-stat-block-controls.json#T074-start-multiattack#step:doStartMultiattack` | `cargo test stat_block_controls` | `covered` |

Target replay evidence:

- Evidence file: `tasks/target-replay-evidence/T074-rule-core-stat-block-controls.json`
- Target profile: `rust`
- Target profile SHA-256: `6d4cc6c6a4769962798133d57aff01438fb2b661941f71d1aa8a3333f4b7ecc1`
- Quint binding: Rust quint-connect harness
- Reproduction trace ids: `T074-end-turn-closes-dispatches`,
  `T074-move-during-dispatch`, `T074-reject-bonus-action-during-dispatch`,
  `T074-reject-ordinary-action-during-dispatch`,
  `T074-resolve-primary-dispatch`, `T074-resolve-secondary-dispatch`,
  `T074-start-multiattack`

Harness artifacts:

- Start gate: `tasks/START_GATE.json`
- Engine depth: `tasks/ENGINE_DEPTH_MANIFEST.json`
- State ownership: `tasks/STATE_OWNER_MANIFEST.json`
- Run ledger: `tasks/RUN_LEDGER.json` is still missing; repo-wide acceptance
  remains blocked by global accounting debt outside this T074 transition
  diagnostic.

Remaining gaps:

- T074 is a reusable rule-core control transition, not yet integrated into
  durable battle `BattleState`.
- Repo-wide harness acceptance still fails on missing run ledger, undeclared
  historical evidence files, and existing adapter quarantine findings outside
  this T074 change.

Verification results:

- `node scripts/check-target-replay-evidence-file.cjs --driver cleanroom-input/qnt/battle-runtime/rule-core-stat-block-controls.mbt.qnt --evidence tasks/target-replay-evidence/T074-rule-core-stat-block-controls.json` passed.
- `cargo fmt --check` passed.
- `cargo test` passed.
- `cargo clippy --all-targets -- -D warnings` passed.
- `node scripts/check-cleanroom-harness.cjs` failed on known repo-wide
  accounting debt outside this T074 transition diagnostic.

## T000: Report Shape Example Only

- Manifest source commit SHA: `829aee6441d76a921c9d9c14a0d0221062975334`
- Source branch inventory SHA: `0a5eaa1f6f79fddbe441dc94500a0dac5644ba7fc392fc6baa3d44da1f2e3248`
- Driver: `cleanroom-input/qnt/<package>/<driver>.mbt.qnt`
- Branch obligations:
  - `step:<branch action>`
- Allowed inputs used:
  - `cleanroom-input/MANIFEST.md`
  - `cleanroom-input/branch-coverage/source-branch-inventory.json`
  - `cleanroom-input/qnt/<package>/<driver>.mbt.qnt`
  - `cleanroom-input/raw/srd-5.2.1/<file>.md`
  - `cleanroom-input/domain/UBIQUITOUS_LANGUAGE.md`

Behavior implemented:

- Describe the domain behavior implemented in `src`.
- Cite the QNT and RAW source files that define the behavior.

Generated branch coverage:

| Obligation | Target replay evidence | Diagnostic tests | Status |
| --- | --- | --- | --- |
| `cleanroom-input/qnt/<package>/<driver>.mbt.qnt#<branch family>:<branch action>` | `_pending_` | `_none_` | `pending` |

Target replay evidence:

- Evidence file: `tasks/target-replay-evidence/<file>.json`
- Target profile: `rust`
- Target profile SHA-256: `6d4cc6c6a4769962798133d57aff01438fb2b661941f71d1aa8a3333f4b7ecc1`
- Quint binding: Rust quint-connect harness
- Reproduction seed or trace id: `<seed or trace id>`
- Accepted evidence refs use `tasks/target-replay-evidence/<file>.json#<trace id>#<branch family>:<branch action>`.

Harness artifacts:

- Start gate: `tasks/START_GATE.json`
- Engine depth: `tasks/ENGINE_DEPTH_MANIFEST.json`
- State ownership: `tasks/STATE_OWNER_MANIFEST.json`
- Reviewer loop: `tasks/REVIEW_LOOP.json`
- Decider decision: `tasks/DECIDER_DECISION.json`
- Immutable history: `tasks/history/<taskId>/`
- Run ledger: `tasks/RUN_LEDGER.json`

Diagnostic tests:

- Focused target-language tests may be listed here, but they do not close
  branch coverage.

Remaining gaps:

- `_none_`

Verification results:

- `cargo fmt --check` passed.
- `cargo test` passed.
- `cargo clippy --all-targets -- -D warnings` passed.
- `node scripts/check-cleanroom-harness.cjs` passed.
