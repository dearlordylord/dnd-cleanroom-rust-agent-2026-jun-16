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

## T079-T060: Stat-Block Multi-Damage Battle-Spine Diagnostic

- Manifest source commit SHA: `829aee6441d76a921c9d9c14a0d0221062975334`
- Source branch inventory SHA: `0a5eaa1f6f79fddbe441dc94500a0dac5644ba7fc392fc6baa3d44da1f2e3248`
- Selected driver:
  `cleanroom-input/qnt/battle-runtime/battle-runtime-stat-block-multi-damage.mbt.qnt`
- Reused dependency driver:
  `cleanroom-input/qnt/battle-runtime/battle-runtime-stat-block-action-ordering.mbt.qnt`
- Branch obligations:
  - `step:doFillHitAttackRoll`
  - `step:doFillTargetChoice`
  - `step:doResolveRolledDamage`
- Allowed inputs used:
  - `cleanroom-input/MANIFEST.md`
  - `cleanroom-input/branch-coverage/source-branch-inventory.json`
  - `cleanroom-input/qnt/battle-runtime/battle-runtime-stat-block-multi-damage.mbt.qnt`
  - `cleanroom-input/qnt/battle-runtime/battle-runtime-stat-block-action-ordering.mbt.qnt`
  - `cleanroom-input/qnt/battle-runtime/battle-runtime-stat-block-action-ordering.qnt`
  - `cleanroom-input/raw/srd-5.2.1/Monsters/Overview.md`
  - `cleanroom-input/raw/srd-5.2.1/Playing-the-Game.md`
  - `cleanroom-input/domain/UBIQUITOUS_LANGUAGE.md`

Behavior implemented:

- `src/rules/battle_reducer_spine.rs` now represents stat-block attack damage
  mode as `StatBlockActionDamageMode::Rolled` or
  `StatBlockActionDamageMode::Static { damage }` instead of a bare rolled
  boolean.
- Rolled stat-block hit resolution still opens a damage-roll hole; resolving
  that damage roll mutates the durable target HP through `BattleState`.
- Static stat-block hit resolution now applies the static hit damage directly
  during the attack-roll fill and resolves without a damage-roll hole.
- `src/qnt_adapters/battle_runtime_stat_block_multi_damage.rs` keeps the T079
  QNT literals as the expected witness, while the observed replay uses
  `start_goblin_stat_block_battle` and `resolve_stat_block_action_subject`.

Generated branch coverage:

| Obligation | Target replay evidence | Diagnostic tests | Status |
| --- | --- | --- | --- |
| `cleanroom-input/qnt/battle-runtime/battle-runtime-stat-block-multi-damage.mbt.qnt#step:doFillHitAttackRoll` | `tasks/target-replay-evidence/T079-battle-runtime-stat-block-multi-damage.json#T079-fill-hit-attack-roll-static#step:doFillHitAttackRoll` | `cargo test stat_block` | `covered` |
| `cleanroom-input/qnt/battle-runtime/battle-runtime-stat-block-multi-damage.mbt.qnt#step:doFillTargetChoice` | `tasks/target-replay-evidence/T079-battle-runtime-stat-block-multi-damage.json#T079-fill-target-choice-rolled#step:doFillTargetChoice` | `cargo test stat_block` | `covered` |
| `cleanroom-input/qnt/battle-runtime/battle-runtime-stat-block-multi-damage.mbt.qnt#step:doResolveRolledDamage` | `tasks/target-replay-evidence/T079-battle-runtime-stat-block-multi-damage.json#T079-resolve-rolled-damage#step:doResolveRolledDamage` | `cargo test stat_block` | `covered` |

Target replay evidence:

- Evidence file:
  `tasks/target-replay-evidence/T079-battle-runtime-stat-block-multi-damage.json`
- Target profile: `rust`
- Target profile SHA-256:
  `6d4cc6c6a4769962798133d57aff01438fb2b661941f71d1aa8a3333f4b7ecc1`
- Quint binding: Rust quint-connect harness
- Additional target replay trace:
  `T079-fill-hit-attack-roll-rolled` records the rolled branch of
  `doFillHitAttackRoll`; the coverage row above cites the static branch because
  static damage application is the new reducer behavior.

Harness artifacts:

- Start gate: `tasks/START_GATE.json`
- Engine depth: `tasks/ENGINE_DEPTH_MANIFEST.json`
- State ownership: `tasks/STATE_OWNER_MANIFEST.json`
- Reviewer loop: `tasks/REVIEW_LOOP.json`
- Decider decision: `tasks/DECIDER_DECISION.json`
- Run ledger: `tasks/RUN_LEDGER.json` is still missing; repo-wide acceptance
  remains blocked by global accounting debt outside this combined diagnostic.

Remaining gaps:

- T079 proves rolled/static stat-block damage through the battle spine for the
  Goblin fixture route, but not size-gated condition riders.
- General stat-block action catalog admission remains future work.
- Repo-wide harness acceptance still fails on the known global denominator:
  missing run ledger, 78 undeclared historical evidence files, 159 witness
  protocol leak findings from undeclared adapters, and 17 authored-identity
  scan findings outside this change.

Verification results:

- `node scripts/check-target-replay-evidence-file.cjs --driver cleanroom-input/qnt/battle-runtime/battle-runtime-stat-block-multi-damage.mbt.qnt --evidence tasks/target-replay-evidence/T079-battle-runtime-stat-block-multi-damage.json` passed.
- `cargo test stat_block -- --nocapture` passed.
- `cargo fmt --check` passed.
- `cargo test` passed.
- `cargo clippy --all-targets -- -D warnings` passed.
- `node scripts/check-cleanroom-harness.cjs` failed on known repo-wide
  accounting debt outside this combined diagnostic: missing
  `tasks/RUN_LEDGER.json`, 78 undeclared evidence files, 159 adapter quarantine
  findings from undeclared adapters, and 17 authored-identity scan findings.

## T060-T074: Stat-Block Action Ordering Battle-Spine Diagnostic

- Manifest source commit SHA: `829aee6441d76a921c9d9c14a0d0221062975334`
- Source branch inventory SHA: `0a5eaa1f6f79fddbe441dc94500a0dac5644ba7fc392fc6baa3d44da1f2e3248`
- Selected driver:
  `cleanroom-input/qnt/battle-runtime/battle-runtime-stat-block-action-ordering.mbt.qnt`
- Reused dependency driver:
  `cleanroom-input/qnt/battle-runtime/rule-core-stat-block-controls.mbt.qnt`
- Branch obligations:
  - `step:doDiscoverRolledActionAttackControl`
  - `step:doDiscoverStaticActionAttackControl`
  - `step:doFillAttackRollMiss`
  - `step:doFillDamageDice`
  - `step:doFillRechargeRoll`
  - `step:doFillRolledAttackRollHit`
  - `step:doFillStaticAttackRollHit`
  - `step:doFillTargetChoice`
  - `step:doRejectAttackRollBeforeTargetChoice`
  - `step:doRejectDamageBeforeAttackRoll`
  - `step:doSpendRechargeGatedRolledAttack`
  - `step:doStartMultiattackControl`
- Allowed inputs used:
  - `cleanroom-input/MANIFEST.md`
  - `cleanroom-input/branch-coverage/source-branch-inventory.json`
  - `cleanroom-input/qnt/battle-runtime/battle-runtime-stat-block-action-ordering.mbt.qnt`
  - `cleanroom-input/qnt/battle-runtime/battle-runtime-stat-block-action-ordering.qnt`
  - `cleanroom-input/qnt/battle-runtime/battle-runtime-stat-block-bridge.qnt`
  - `cleanroom-input/qnt/battle-runtime/battle-runtime-stat-block-bridge-examples.qnt`
  - `cleanroom-input/qnt/battle-runtime/rule-core-stat-block-controls.mbt.qnt`
  - `cleanroom-input/qnt/shared-algebras/proofs/rule-core/stat-block-controls.qnt`
  - `cleanroom-input/raw/srd-5.2.1/Monsters/Overview.md`
  - `cleanroom-input/domain/UBIQUITOUS_LANGUAGE.md`

Behavior implemented:

- `src/rules/battle_reducer_spine.rs` now exposes stat-block action subjects,
  fills, and resolution results for the Goblin stat-block action-ordering
  path.
- Multiattack start in this path calls `start_stat_block_multiattack_from`
  with the Goblin QNT bridge profile, so the projected dispatch count comes
  from stat-block control state instead of an adapter literal.
- Rolled/static stat-block attack ordering, attack-roll-before-target rejection,
  damage-before-attack-roll rejection, damage dice completion, and recharge
  roll completion now replay through `BattleState` and
  `resolve_stat_block_action_subject`.
- `src/qnt_adapters/battle_runtime_stat_block_action_ordering.rs` keeps the
  focused QNT helper as the expected witness, while the observed replay uses
  the reducer-spine APIs.

Generated branch coverage:

| Obligation | Target replay evidence | Diagnostic tests | Status |
| --- | --- | --- | --- |
| `cleanroom-input/qnt/battle-runtime/battle-runtime-stat-block-action-ordering.mbt.qnt#step:doDiscoverRolledActionAttackControl` | `tasks/target-replay-evidence/T060-battle-runtime-stat-block-action-ordering.json#T060-discover-rolled-action-attack-control#step:doDiscoverRolledActionAttackControl` | `cargo test stat_block_action_ordering` | `prior diagnostic` |
| `cleanroom-input/qnt/battle-runtime/battle-runtime-stat-block-action-ordering.mbt.qnt#step:doDiscoverStaticActionAttackControl` | `tasks/target-replay-evidence/T060-battle-runtime-stat-block-action-ordering.json#T060-discover-static-action-attack-control#step:doDiscoverStaticActionAttackControl` | `cargo test stat_block_action_ordering` | `prior diagnostic` |
| `cleanroom-input/qnt/battle-runtime/battle-runtime-stat-block-action-ordering.mbt.qnt#step:doFillAttackRollMiss` | `tasks/target-replay-evidence/T060-battle-runtime-stat-block-action-ordering.json#T060-fill-attack-roll-miss#step:doFillAttackRollMiss` | `cargo test stat_block_action_ordering` | `prior diagnostic` |
| `cleanroom-input/qnt/battle-runtime/battle-runtime-stat-block-action-ordering.mbt.qnt#step:doFillDamageDice` | `tasks/target-replay-evidence/T060-battle-runtime-stat-block-action-ordering.json#T060-fill-damage-dice#step:doFillDamageDice` | `cargo test stat_block_action_ordering` | `prior diagnostic` |
| `cleanroom-input/qnt/battle-runtime/battle-runtime-stat-block-action-ordering.mbt.qnt#step:doFillRechargeRoll` | `tasks/target-replay-evidence/T060-battle-runtime-stat-block-action-ordering.json#T060-fill-recharge-roll#step:doFillRechargeRoll` | `cargo test stat_block_action_ordering` | `prior diagnostic` |
| `cleanroom-input/qnt/battle-runtime/battle-runtime-stat-block-action-ordering.mbt.qnt#step:doFillRolledAttackRollHit` | `tasks/target-replay-evidence/T060-battle-runtime-stat-block-action-ordering.json#T060-fill-rolled-attack-roll-hit#step:doFillRolledAttackRollHit` | `cargo test stat_block_action_ordering` | `prior diagnostic` |
| `cleanroom-input/qnt/battle-runtime/battle-runtime-stat-block-action-ordering.mbt.qnt#step:doFillStaticAttackRollHit` | `tasks/target-replay-evidence/T060-battle-runtime-stat-block-action-ordering.json#T060-fill-static-attack-roll-hit#step:doFillStaticAttackRollHit` | `cargo test stat_block_action_ordering` | `prior diagnostic` |
| `cleanroom-input/qnt/battle-runtime/battle-runtime-stat-block-action-ordering.mbt.qnt#step:doFillTargetChoice` | `tasks/target-replay-evidence/T060-battle-runtime-stat-block-action-ordering.json#T060-fill-target-choice#step:doFillTargetChoice` | `cargo test stat_block_action_ordering` | `prior diagnostic` |
| `cleanroom-input/qnt/battle-runtime/battle-runtime-stat-block-action-ordering.mbt.qnt#step:doRejectAttackRollBeforeTargetChoice` | `tasks/target-replay-evidence/T060-battle-runtime-stat-block-action-ordering.json#T060-reject-attack-roll-before-target-choice#step:doRejectAttackRollBeforeTargetChoice` | `cargo test stat_block_action_ordering` | `prior diagnostic` |
| `cleanroom-input/qnt/battle-runtime/battle-runtime-stat-block-action-ordering.mbt.qnt#step:doRejectDamageBeforeAttackRoll` | `tasks/target-replay-evidence/T060-battle-runtime-stat-block-action-ordering.json#T060-reject-damage-before-attack-roll#step:doRejectDamageBeforeAttackRoll` | `cargo test stat_block_action_ordering` | `prior diagnostic` |
| `cleanroom-input/qnt/battle-runtime/battle-runtime-stat-block-action-ordering.mbt.qnt#step:doSpendRechargeGatedRolledAttack` | `tasks/target-replay-evidence/T060-battle-runtime-stat-block-action-ordering.json#T060-spend-recharge-gated-rolled-attack#step:doSpendRechargeGatedRolledAttack` | `cargo test stat_block_action_ordering` | `prior diagnostic` |
| `cleanroom-input/qnt/battle-runtime/battle-runtime-stat-block-action-ordering.mbt.qnt#step:doStartMultiattackControl` | `tasks/target-replay-evidence/T060-battle-runtime-stat-block-action-ordering.json#T060-start-multiattack-control#step:doStartMultiattackControl` | `cargo test stat_block_action_ordering` | `prior diagnostic` |

Target replay evidence:

- Evidence file:
  `tasks/target-replay-evidence/T060-battle-runtime-stat-block-action-ordering.json`
- Target profile: `rust`
- Target profile SHA-256:
  `6d4cc6c6a4769962798133d57aff01438fb2b661941f71d1aa8a3333f4b7ecc1`
- Quint binding: Rust quint-connect harness

Harness artifacts:

- Start gate: `tasks/START_GATE.json`
- Engine depth: `tasks/ENGINE_DEPTH_MANIFEST.json`
- State ownership: `tasks/STATE_OWNER_MANIFEST.json`
- Reviewer loop: `tasks/REVIEW_LOOP.json`
- Decider decision: `tasks/DECIDER_DECISION.json`
- Run ledger: `tasks/RUN_LEDGER.json` is still missing; repo-wide acceptance
  remains blocked by global accounting debt outside this combined diagnostic.

Remaining gaps:

- T060 proves stat-block action ordering and projection through the battle
  spine, but not full stat-block damage semantics for all targets.
- The Goblin profile is still a QNT bridge fixture; general stat-block action
  catalog admission remains future work.
- Repo-wide harness acceptance still fails on missing run ledger, undeclared
  historical evidence files, and existing adapter quarantine findings outside
  this change.

Verification results:

- `node scripts/check-target-replay-evidence-file.cjs --driver cleanroom-input/qnt/battle-runtime/battle-runtime-stat-block-action-ordering.mbt.qnt --evidence tasks/target-replay-evidence/T060-battle-runtime-stat-block-action-ordering.json` passed.
- `cargo test stat_block_action_ordering -- --nocapture` passed.
- `cargo fmt --check` passed.
- `cargo test` passed.
- `cargo clippy --all-targets -- -D warnings` passed.
- `node scripts/check-cleanroom-harness.cjs` failed on known repo-wide
  accounting debt outside this combined diagnostic.

## T064-T074: Skeleton Battle-Spine Stat-Block Control Diagnostic

- Manifest source commit SHA: `829aee6441d76a921c9d9c14a0d0221062975334`
- Source branch inventory SHA: `0a5eaa1f6f79fddbe441dc94500a0dac5644ba7fc392fc6baa3d44da1f2e3248`
- Selected driver:
  `cleanroom-input/qnt/battle-runtime/battle-runtime-weapon-attack-skeleton.mbt.qnt`
- Reused dependency driver:
  `cleanroom-input/qnt/battle-runtime/rule-core-stat-block-controls.mbt.qnt`
- Branch obligations:
  - `step:doDiscoverAttack`
  - `step:doFillTarget`
  - `step:doRejectWrongTarget`
  - `step:doFillAttackRollMiss`
  - `step:doFillAttackRollHit`
  - `step:doFillDamageLow`
  - `step:doFillDamageHigh`
  - `step:doFillDamageLowSneakAttack`
  - `step:doFillDamageHighSneakAttack`
  - `step:doRejectStaleAfterResolved`
  - `step:doStartSkeletonTurn`
  - `step:doResolveSkeletonMultiattack`
  - `step:doRejectRecursiveSkeletonMultiattack`
  - `step:doSpendSkeletonMultiattackDispatch`
- Allowed inputs used:
  - `cleanroom-input/MANIFEST.md`
  - `cleanroom-input/branch-coverage/source-branch-inventory.json`
  - `cleanroom-input/qnt/battle-runtime/battle-runtime-weapon-attack-skeleton.mbt.qnt`
  - `cleanroom-input/qnt/battle-runtime/rule-core-stat-block-controls.mbt.qnt`
  - `cleanroom-input/qnt/shared-algebras/proofs/rule-core/stat-block-controls.qnt`
  - `cleanroom-input/raw/srd-5.2.1/Monsters/Overview.md`
  - `cleanroom-input/raw/srd-5.2.1/Playing-the-Game.md`

Behavior implemented:

- `src/rules/battle_reducer_spine.rs` now stores
  `BattleState.stat_block_control` and no longer stores a separate Skeleton
  multiattack dispatch counter on `Combatant`.
- Skeleton multiattack resolution calls
  `start_stat_block_multiattack_from`; dispatch spending calls
  `resolve_stat_block_control_subject`.
- `src/qnt_adapters/battle_runtime_weapon_attack_skeleton.rs` still exposes
  the T064 witness projection, but `qMultiattackDispatchesAvailable` is now
  derived from the battle spine's stat-block control dispatch counts.

Generated branch coverage:

| Obligation | Target replay evidence | Diagnostic tests | Status |
| --- | --- | --- | --- |
| `cleanroom-input/qnt/battle-runtime/battle-runtime-weapon-attack-skeleton.mbt.qnt#step:doDiscoverAttack` | `tasks/target-replay-evidence/T064-battle-runtime-weapon-attack-skeleton.json#T064-discover-attack#step:doDiscoverAttack` | `cargo test weapon_attack_skeleton` | `prior diagnostic` |
| `cleanroom-input/qnt/battle-runtime/battle-runtime-weapon-attack-skeleton.mbt.qnt#step:doFillTarget` | `tasks/target-replay-evidence/T064-battle-runtime-weapon-attack-skeleton.json#T064-fill-target#step:doFillTarget` | `cargo test weapon_attack_skeleton` | `prior diagnostic` |
| `cleanroom-input/qnt/battle-runtime/battle-runtime-weapon-attack-skeleton.mbt.qnt#step:doRejectWrongTarget` | `tasks/target-replay-evidence/T064-battle-runtime-weapon-attack-skeleton.json#T064-reject-wrong-target#step:doRejectWrongTarget` | `cargo test weapon_attack_skeleton` | `prior diagnostic` |
| `cleanroom-input/qnt/battle-runtime/battle-runtime-weapon-attack-skeleton.mbt.qnt#step:doFillAttackRollMiss` | `tasks/target-replay-evidence/T064-battle-runtime-weapon-attack-skeleton.json#T064-fill-attack-roll-miss#step:doFillAttackRollMiss` | `cargo test weapon_attack_skeleton` | `prior diagnostic` |
| `cleanroom-input/qnt/battle-runtime/battle-runtime-weapon-attack-skeleton.mbt.qnt#step:doFillAttackRollHit` | `tasks/target-replay-evidence/T064-battle-runtime-weapon-attack-skeleton.json#T064-fill-attack-roll-hit#step:doFillAttackRollHit` | `cargo test weapon_attack_skeleton` | `prior diagnostic` |
| `cleanroom-input/qnt/battle-runtime/battle-runtime-weapon-attack-skeleton.mbt.qnt#step:doFillDamageLow` | `tasks/target-replay-evidence/T064-battle-runtime-weapon-attack-skeleton.json#T064-fill-damage-low#step:doFillDamageLow` | `cargo test weapon_attack_skeleton` | `prior diagnostic` |
| `cleanroom-input/qnt/battle-runtime/battle-runtime-weapon-attack-skeleton.mbt.qnt#step:doFillDamageHigh` | `tasks/target-replay-evidence/T064-battle-runtime-weapon-attack-skeleton.json#T064-fill-damage-high#step:doFillDamageHigh` | `cargo test weapon_attack_skeleton` | `prior diagnostic` |
| `cleanroom-input/qnt/battle-runtime/battle-runtime-weapon-attack-skeleton.mbt.qnt#step:doFillDamageLowSneakAttack` | `tasks/target-replay-evidence/T064-battle-runtime-weapon-attack-skeleton.json#T064-fill-damage-low-sneak-attack#step:doFillDamageLowSneakAttack` | `cargo test weapon_attack_skeleton` | `prior diagnostic` |
| `cleanroom-input/qnt/battle-runtime/battle-runtime-weapon-attack-skeleton.mbt.qnt#step:doFillDamageHighSneakAttack` | `tasks/target-replay-evidence/T064-battle-runtime-weapon-attack-skeleton.json#T064-fill-damage-high-sneak-attack#step:doFillDamageHighSneakAttack` | `cargo test weapon_attack_skeleton` | `prior diagnostic` |
| `cleanroom-input/qnt/battle-runtime/battle-runtime-weapon-attack-skeleton.mbt.qnt#step:doRejectStaleAfterResolved` | `tasks/target-replay-evidence/T064-battle-runtime-weapon-attack-skeleton.json#T064-reject-stale-after-resolved#step:doRejectStaleAfterResolved` | `cargo test weapon_attack_skeleton` | `prior diagnostic` |
| `cleanroom-input/qnt/battle-runtime/battle-runtime-weapon-attack-skeleton.mbt.qnt#step:doStartSkeletonTurn` | `tasks/target-replay-evidence/T064-battle-runtime-weapon-attack-skeleton.json#T064-start-skeleton-turn#step:doStartSkeletonTurn` | `cargo test weapon_attack_skeleton` | `prior diagnostic` |
| `cleanroom-input/qnt/battle-runtime/battle-runtime-weapon-attack-skeleton.mbt.qnt#step:doResolveSkeletonMultiattack` | `tasks/target-replay-evidence/T064-battle-runtime-weapon-attack-skeleton.json#T064-resolve-skeleton-multiattack#step:doResolveSkeletonMultiattack` | `cargo test weapon_attack_skeleton` | `prior diagnostic` |
| `cleanroom-input/qnt/battle-runtime/battle-runtime-weapon-attack-skeleton.mbt.qnt#step:doRejectRecursiveSkeletonMultiattack` | `tasks/target-replay-evidence/T064-battle-runtime-weapon-attack-skeleton.json#T064-reject-recursive-skeleton-multiattack#step:doRejectRecursiveSkeletonMultiattack` | `cargo test weapon_attack_skeleton` | `prior diagnostic` |
| `cleanroom-input/qnt/battle-runtime/battle-runtime-weapon-attack-skeleton.mbt.qnt#step:doSpendSkeletonMultiattackDispatch` | `tasks/target-replay-evidence/T064-battle-runtime-weapon-attack-skeleton.json#T064-spend-skeleton-multiattack-dispatch#step:doSpendSkeletonMultiattackDispatch` | `cargo test weapon_attack_skeleton` | `prior diagnostic` |

Target replay evidence:

- Evidence file:
  `tasks/target-replay-evidence/T064-battle-runtime-weapon-attack-skeleton.json`
- Target profile: `rust`
- Target profile SHA-256:
  `6d4cc6c6a4769962798133d57aff01438fb2b661941f71d1aa8a3333f4b7ecc1`
- Quint binding: Rust quint-connect harness

Harness artifacts:

- Start gate: `tasks/START_GATE.json`
- Engine depth: `tasks/ENGINE_DEPTH_MANIFEST.json`
- State ownership: `tasks/STATE_OWNER_MANIFEST.json`
- Run ledger: `tasks/RUN_LEDGER.json` is still missing; repo-wide acceptance
  remains blocked by global accounting debt outside this combined diagnostic.

Remaining gaps:

- This integrates T074 control into the T064 Skeleton fixture path only.
- General stat-block action discovery, primary/secondary attack-slot selection,
  and end-turn reset inside the full battle reducer remain future work.
- Repo-wide harness acceptance still fails on missing run ledger, undeclared
  historical evidence files, and existing adapter quarantine findings outside
  this change.

Verification results:

- `node scripts/check-target-replay-evidence-file.cjs --driver cleanroom-input/qnt/battle-runtime/battle-runtime-weapon-attack-skeleton.mbt.qnt --evidence tasks/target-replay-evidence/T064-battle-runtime-weapon-attack-skeleton.json` passed.
- `node scripts/check-target-replay-evidence-file.cjs --driver cleanroom-input/qnt/battle-runtime/rule-core-stat-block-controls.mbt.qnt --evidence tasks/target-replay-evidence/T074-rule-core-stat-block-controls.json` passed.
- `cargo test weapon_attack_skeleton -- --nocapture` passed.
- `cargo test stat_block_controls -- --nocapture` passed.
- `cargo fmt --check` passed.
- `cargo test` passed.
- `cargo clippy --all-targets -- -D warnings` passed.
- `node scripts/check-cleanroom-harness.cjs` failed on known repo-wide
  accounting debt outside this combined diagnostic.

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
| `cleanroom-input/qnt/battle-runtime/rule-core-stat-block-controls.mbt.qnt#step:doEndTurnClosesDispatches` | `tasks/target-replay-evidence/T074-rule-core-stat-block-controls.json#T074-end-turn-closes-dispatches#step:doEndTurnClosesDispatches` | `cargo test stat_block_controls` | `dependency diagnostic` |
| `cleanroom-input/qnt/battle-runtime/rule-core-stat-block-controls.mbt.qnt#step:doMoveDuringDispatch` | `tasks/target-replay-evidence/T074-rule-core-stat-block-controls.json#T074-move-during-dispatch#step:doMoveDuringDispatch` | `cargo test stat_block_controls` | `dependency diagnostic` |
| `cleanroom-input/qnt/battle-runtime/rule-core-stat-block-controls.mbt.qnt#step:doRejectBonusActionDuringDispatch` | `tasks/target-replay-evidence/T074-rule-core-stat-block-controls.json#T074-reject-bonus-action-during-dispatch#step:doRejectBonusActionDuringDispatch` | `cargo test stat_block_controls` | `dependency diagnostic` |
| `cleanroom-input/qnt/battle-runtime/rule-core-stat-block-controls.mbt.qnt#step:doRejectOrdinaryActionDuringDispatch` | `tasks/target-replay-evidence/T074-rule-core-stat-block-controls.json#T074-reject-ordinary-action-during-dispatch#step:doRejectOrdinaryActionDuringDispatch` | `cargo test stat_block_controls` | `dependency diagnostic` |
| `cleanroom-input/qnt/battle-runtime/rule-core-stat-block-controls.mbt.qnt#step:doResolvePrimaryDispatch` | `tasks/target-replay-evidence/T074-rule-core-stat-block-controls.json#T074-resolve-primary-dispatch#step:doResolvePrimaryDispatch` | `cargo test stat_block_controls` | `dependency diagnostic` |
| `cleanroom-input/qnt/battle-runtime/rule-core-stat-block-controls.mbt.qnt#step:doResolveSecondaryDispatch` | `tasks/target-replay-evidence/T074-rule-core-stat-block-controls.json#T074-resolve-secondary-dispatch#step:doResolveSecondaryDispatch` | `cargo test stat_block_controls` | `dependency diagnostic` |
| `cleanroom-input/qnt/battle-runtime/rule-core-stat-block-controls.mbt.qnt#step:doStartMultiattack` | `tasks/target-replay-evidence/T074-rule-core-stat-block-controls.json#T074-start-multiattack#step:doStartMultiattack` | `cargo test stat_block_controls` | `dependency diagnostic` |

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

- T074 is integrated into durable battle `BattleState` for the T064 Skeleton
  fixture path only; general stat-block battle integration remains incomplete.
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
