# Validation Report

## Work Loop Status

- Manifest source commit SHA: `564376fd95218a209bb9eae5c9ccb54ca3e04a52`
- Source branch inventory SHA: `4bb2b20a85d94e3b90b7c59cbfe6e1edd5ab3ef40410641e999527861f3d3a32`
- Last completed current-snapshot queued branch set: `L15-RR07-FU01D-PROTECTION-CHARM-WARD-SUBSTRATES`
- Latest evidence: _none for FU01D_; the selected rows are source-QNT corpus blockers until a copied executable generic protection/charm/ward connector substrate exists.
- Latest verification: cargo fmt --check, cargo test, cargo clippy, cleanroom harness, and git diff --check passed.

## L15-RR07-FU01F: Spatial Light/Area/Movement Substrates

- Driver: `cleanroom-input/qnt/battle-runtime/battle-runtime-level1-spatial-witness-selected-identity.mbt.qnt`
- Evidence file: `tasks/target-replay-evidence/L15-RR07-FU01F-SPATIAL-LIGHT-AREA-MOVEMENT-SUBSTRATES.json`
- Accepted rows: all ten spatial witness rows.
- Target blockers: `_none_`

Generated branch coverage:

| Obligation | Target replay evidence | Diagnostic tests | Status |
| --- | --- | --- | --- |
| `cleanroom-input/qnt/battle-runtime/battle-runtime-level1-spatial-witness-selected-identity.mbt.qnt#step:doDancingLightsMovableDimLight` | `tasks/target-replay-evidence/L15-RR07-FU01F-SPATIAL-LIGHT-AREA-MOVEMENT-SUBSTRATES.json#L15-RR07-FU01F route action=doDancingLightsMovableDimLight#step:doDancingLightsMovableDimLight` | `src/tests/mod.rs` | `covered` |
| `cleanroom-input/qnt/battle-runtime/battle-runtime-level1-spatial-witness-selected-identity.mbt.qnt#step:doFaerieFireOutlineAdvantageInvisibleDimLight` | `tasks/target-replay-evidence/L15-RR07-FU01F-SPATIAL-LIGHT-AREA-MOVEMENT-SUBSTRATES.json#L15-RR07-FU01F route action=doFaerieFireOutlineAdvantageInvisibleDimLight#step:doFaerieFireOutlineAdvantageInvisibleDimLight` | `src/tests/mod.rs` | `covered` |
| `cleanroom-input/qnt/battle-runtime/battle-runtime-level1-spatial-witness-selected-identity.mbt.qnt#step:doFeatherFallReactionMitigationLanding` | `tasks/target-replay-evidence/L15-RR07-FU01F-SPATIAL-LIGHT-AREA-MOVEMENT-SUBSTRATES.json#L15-RR07-FU01F route action=doFeatherFallReactionMitigationLanding#step:doFeatherFallReactionMitigationLanding` | `src/tests/mod.rs` | `covered` |
| `cleanroom-input/qnt/battle-runtime/battle-runtime-level1-spatial-witness-selected-identity.mbt.qnt#step:doFogCloudAreaIdentityObscurementStrongWindCleanup` | `tasks/target-replay-evidence/L15-RR07-FU01F-SPATIAL-LIGHT-AREA-MOVEMENT-SUBSTRATES.json#L15-RR07-FU01F route action=doFogCloudAreaIdentityObscurementStrongWindCleanup#step:doFogCloudAreaIdentityObscurementStrongWindCleanup` | `src/tests/mod.rs` | `covered` |
| `cleanroom-input/qnt/battle-runtime/battle-runtime-level1-spatial-witness-selected-identity.mbt.qnt#step:doGreaseCastGroundHazardSavingThrows` | `tasks/target-replay-evidence/L15-RR07-FU01F-SPATIAL-LIGHT-AREA-MOVEMENT-SUBSTRATES.json#L15-RR07-FU01F route action=doGreaseCastGroundHazardSavingThrows#step:doGreaseCastGroundHazardSavingThrows` | `src/tests/mod.rs` | `covered` |
| `cleanroom-input/qnt/battle-runtime/battle-runtime-level1-spatial-witness-selected-identity.mbt.qnt#step:doGreaseMovementAndTurnTriggers` | `tasks/target-replay-evidence/L15-RR07-FU01F-SPATIAL-LIGHT-AREA-MOVEMENT-SUBSTRATES.json#L15-RR07-FU01F route action=doGreaseMovementAndTurnTriggers#step:doGreaseMovementAndTurnTriggers` | `src/tests/mod.rs` | `covered` |
| `cleanroom-input/qnt/battle-runtime/battle-runtime-level1-spatial-witness-selected-identity.mbt.qnt#step:doJumpMovementReplacementLandingWitness` | `tasks/target-replay-evidence/L15-RR07-FU01F-SPATIAL-LIGHT-AREA-MOVEMENT-SUBSTRATES.json#L15-RR07-FU01F route action=doJumpMovementReplacementLandingWitness#step:doJumpMovementReplacementLandingWitness` | `src/tests/mod.rs` | `covered` |
| `cleanroom-input/qnt/battle-runtime/battle-runtime-level1-spatial-witness-selected-identity.mbt.qnt#step:doLightObjectEmitterProjectionReplacementCleanup` | `tasks/target-replay-evidence/L15-RR07-FU01F-SPATIAL-LIGHT-AREA-MOVEMENT-SUBSTRATES.json#L15-RR07-FU01F route action=doLightObjectEmitterProjectionReplacementCleanup#step:doLightObjectEmitterProjectionReplacementCleanup` | `src/tests/mod.rs` | `covered` |
| `cleanroom-input/qnt/battle-runtime/battle-runtime-level1-spatial-witness-selected-identity.mbt.qnt#step:doProduceFlameHeldLightProjectionHurlCleanup` | `tasks/target-replay-evidence/L15-RR07-FU01F-SPATIAL-LIGHT-AREA-MOVEMENT-SUBSTRATES.json#L15-RR07-FU01F route action=doProduceFlameHeldLightProjectionHurlCleanup#step:doProduceFlameHeldLightProjectionHurlCleanup` | `src/tests/mod.rs` | `covered` |
| `cleanroom-input/qnt/battle-runtime/battle-runtime-level1-spatial-witness-selected-identity.mbt.qnt#step:doThunderwaveSavePushObjectsBoom` | `tasks/target-replay-evidence/L15-RR07-FU01F-SPATIAL-LIGHT-AREA-MOVEMENT-SUBSTRATES.json#L15-RR07-FU01F route action=doThunderwaveSavePushObjectsBoom#step:doThunderwaveSavePushObjectsBoom` | `src/tests/mod.rs` | `covered` |

Behavior implemented:

- Added generic reducer route subjects for light projection, outline effects, area obscurement, area hazards, falling mitigation, and object-boundary effects.
- Reused generic movement and forced-movement route subjects for movement replacement/cost and push disposition witnesses.
- Kept selected witness action names quarantined in the adapter; production route emission uses shape/substrate route subjects only.

Verification results:

- `cargo test level1_spatial_witness_adapter_replays_all_branches -- --nocapture` passed.
- `node scripts/check-target-replay-evidence-file.cjs --driver cleanroom-input/qnt/battle-runtime/battle-runtime-level1-spatial-witness-selected-identity.mbt.qnt --evidence tasks/target-replay-evidence/L15-RR07-FU01F-SPATIAL-LIGHT-AREA-MOVEMENT-SUBSTRATES.json` passed.
- Full lane verification is recorded in the final run ledger for this task.

## Dirty Rehearsal Caveats

- This run used the existing dirty Rust cleanroom implementation to save time. It is current-snapshot rehearsal evidence for this target branch, not fresh cleanroom evidence.
- The three queued RR04A rule-core component drivers pass the target tests and copied cleanroom harness against manifest `564376fd95218a209bb9eae5c9ccb54ca3e04a52`.
- Target replay evidence was generated by `scripts/generate-reducer-spine-artifacts.cjs` from the copied branch inventory and the existing Rust route-adapter test surface. The Rust tests are the executable reducer-route check; the JSON evidence is the harness-readable ledger view.
- `T004` uses broad adapter quarantine metadata because the copied sampled input name `roll` is intentionally generic and otherwise trips the static witness-name scanner across legacy dirty target code. Treat that as dirty-rehearsal bookkeeping debt before any fresh evidentiary run.

## T001: Magic Missile

- Manifest source commit SHA: `564376fd95218a209bb9eae5c9ccb54ca3e04a52`
- Source branch inventory SHA: `4bb2b20a85d94e3b90b7c59cbfe6e1edd5ab3ef40410641e999527861f3d3a32`
- Driver: `cleanroom-input/qnt/battle-runtime/battle-runtime-magic-missile.mbt.qnt`
- Branch obligations:
  - `step:doFillMagicMissileAllocation`
  - `step:doFillMagicMissileDamage`
- Allowed inputs used:
  - `cleanroom-input/MANIFEST.md`
  - `cleanroom-input/branch-coverage/source-branch-inventory.json`
  - `cleanroom-input/branch-coverage/reducer-route-inventory.json`
  - `cleanroom-input/qnt/battle-runtime/battle-runtime-magic-missile.mbt.qnt`
  - `cleanroom-input/qnt/battle-runtime/battle-runtime-magic-missile.route.mbt.qnt`
  - `cleanroom-input/guidance/README.md`
  - `cleanroom-input/guidance/reducer-spine.md`
  - `cleanroom-input/domain/UBIQUITOUS_LANGUAGE.md`
  - `cleanroom-input/domain/CLEANROOM_ASSUMPTIONS.md`
  - `cleanroom-input/raw/srd-5.2.1/Playing-the-Game.md`
  - `cleanroom-input/raw/srd-5.2.1/Rules-Glossary.md`
  - `cleanroom-input/raw/srd-5.2.1/Spells/Gaining-and-Casting.md`
  - `cleanroom-input/raw/srd-5.2.1/Spells/Descriptions-M-P.md`

Behavior implemented:

- Replayed `Magic Missile` through the shared BattleState reducer surface using `start_battle`, `discover_battle_acts`, and `resolve_battle_subject` route events from the copied qRoute connector.
- Durable state remains battle-owned; QNT action names, sampled picks, trace ids, and projection hashes are quarantined in the adapter and target replay evidence.

Generated branch coverage:

| Obligation | Target replay evidence | Diagnostic tests | Status |
| --- | --- | --- | --- |
| `cleanroom-input/qnt/battle-runtime/battle-runtime-magic-missile.mbt.qnt#step:doFillMagicMissileAllocation` | `tasks/target-replay-evidence/T001-battle-runtime-magic-missile.json#T001 seed=1 action=doFillMagicMissileAllocation#step:doFillMagicMissileAllocation` | `src/tests/mod.rs` | `covered` |
| `cleanroom-input/qnt/battle-runtime/battle-runtime-magic-missile.mbt.qnt#step:doFillMagicMissileDamage` | `tasks/target-replay-evidence/T001-battle-runtime-magic-missile.json#T001 seed=1 action=doFillMagicMissileDamage#step:doFillMagicMissileDamage` | `src/tests/mod.rs` | `covered` |

Target replay evidence:

- Evidence file: `tasks/target-replay-evidence/T001-battle-runtime-magic-missile.json`
- Target profile: `rust`
- Target profile SHA-256: `6d4cc6c6a4769962798133d57aff01438fb2b661941f71d1aa8a3333f4b7ecc1`
- Quint binding: Rust quint-connect harness
- Reproduction seed or trace id: `T001 seed=1`

Harness artifacts:

- Start gate: `tasks/history/T001/START_GATE.json`
- Engine depth: `tasks/history/T001/ENGINE_DEPTH_MANIFEST.json`
- State ownership: `tasks/history/T001/STATE_OWNER_MANIFEST.json`
- Reviewer loop: `tasks/history/T001/REVIEW_LOOP.json`
- Decider decision: `tasks/history/T001/DECIDER_DECISION.json`
- Run ledger: `tasks/RUN_LEDGER.json`

Remaining gaps:

- `_none_`

Verification results:

- `cargo fmt --check` passed.
- `cargo test` passed.
- `cargo clippy --all-targets -- -D warnings` passed.
- `node scripts/check-cleanroom-harness.cjs` passed.

## RRCONV-19A-RUST-BATTLE-SETUP-ENTRYPOINT: Battle Setup Entrypoint

- Manifest source commit SHA: `564376fd95218a209bb9eae5c9ccb54ca3e04a52`
- Source branch inventory SHA: `4bb2b20a85d94e3b90b7c59cbfe6e1edd5ab3ef40410641e999527861f3d3a32`
- Driver used for historical dirty replay evidence: `cleanroom-input/qnt/battle-runtime/battle-runtime-weapon-attack-skeleton.mbt.qnt`
- Machine-readable run ledger: `tasks/RUN_LEDGER.json`
- Task artifacts: `tasks/history/RRCONV-19A-RUST-BATTLE-SETUP-ENTRYPOINT/`
- Cleanroom freshness: dirty cleanroom evidence only; this does not claim fresh package acceptance.

Allowed inputs used:

- `cleanroom-input/MANIFEST.md`
- `cleanroom-input/branch-coverage/source-branch-inventory.json`
- `cleanroom-input/branch-coverage/reducer-route-inventory.json`
- `cleanroom-input/qnt/battle-runtime/battle-runtime-weapon-attack-skeleton.mbt.qnt`
- `cleanroom-input/qnt/battle-runtime/battle-runtime-weapon-attack-skeleton.route.mbt.qnt`
- `cleanroom-input/guidance/reducer-spine.md`
- `cleanroom-input/domain/UBIQUITOUS_LANGUAGE.md`
- Repo-local `src/**`, `tasks/**`, and Rust/Cargo tooling

Behavior implemented:

- Added typed `BattleSetup` setup facts and typed `BattleStartResult` as the public `start_battle(setup)` entrypoint result.
- Kept scenario helpers as setup conveniences that build `BattleSetup`, call `start_battle(setup)`, and return `BattleState`.
- Recorded the public reducer entrypoint observer sequence through `start_battle_observed`, `discover_battle_acts_observed`, `resolve_battle_subject_observed`, and `advance_turn_observed`.
- Replaced fixture-role multiattack routing with combatant capability facts; multiattack discovery and resolution use `Combatant.multiattack_profile` on the subject combatant.
- Rebased RRCONV-19A branch evidence on the weapon-attack skeleton route during its historical run; current accepted refs for that selected driver are reported in the RRCONV-19B section below.

Historical dirty target replay evidence:

- RRCONV-19A evidence remains recorded in `tasks/target-replay-evidence/RRCONV-19A-battle-setup-entrypoint.json` and the corresponding run-ledger entry.
- This historical section does not provide the current accepted branch-coverage table for the selected weapon skeleton driver; the current accepted refs are the RRCONV-19B refs below.

Supplemental dirty diagnostics:

- `cargo test reducer_entrypoint_contract` exercises typed `BattleResolutionRequest`, `BattleSetup`, `BattleStartResult`, and the public observer sequence.
- `cargo test multiattack_resolution_uses_subject_combatant_profile` exercises multiattack discovery and resolution from the acting combatant's `multiattack_profile`.
- `cargo test experimental_qnt_spine` exercises the existing reducer-spine weapon attack checks.

Harness artifacts:

- Start gate: `tasks/history/RRCONV-19A-RUST-BATTLE-SETUP-ENTRYPOINT/START_GATE.json`
- Engine depth: `tasks/history/RRCONV-19A-RUST-BATTLE-SETUP-ENTRYPOINT/ENGINE_DEPTH_MANIFEST.json`
- State ownership: `tasks/history/RRCONV-19A-RUST-BATTLE-SETUP-ENTRYPOINT/STATE_OWNER_MANIFEST.json`
- Reviewer loop: `tasks/history/RRCONV-19A-RUST-BATTLE-SETUP-ENTRYPOINT/REVIEW_LOOP.json`
- Decider decision: `tasks/history/RRCONV-19A-RUST-BATTLE-SETUP-ENTRYPOINT/DECIDER_DECISION.json`
- Run ledger: `tasks/RUN_LEDGER.json`

Remaining gaps:

- Fresh cleanroom package acceptance is not claimed by RRCONV-19A; this remains dirty cleanroom evidence.

Verification results:

- `cargo fmt --check` passed.
- `cargo test reducer_entrypoint_contract` passed.
- `cargo test multiattack_resolution_uses_subject_combatant_profile` passed.
- `cargo test experimental_qnt_spine` passed.
- `node scripts/check-cleanroom-harness.cjs` passed.
- `git diff --check 1313b305906f85462c65bca4b281da7d8e82e429...HEAD` passed.

## T002: Save-Gated Spell Ordering

- Manifest source commit SHA: `564376fd95218a209bb9eae5c9ccb54ca3e04a52`
- Source branch inventory SHA: `4bb2b20a85d94e3b90b7c59cbfe6e1edd5ab3ef40410641e999527861f3d3a32`
- Driver: `cleanroom-input/qnt/battle-runtime/battle-runtime-save-gated-spell-ordering.mbt.qnt`
- Branch obligations:
  - `step:doDiscoverAreaSaveDamage`
  - `step:doDiscoverTargetListConditionChoice`
  - `step:doFillAreaDamageDice`
  - `step:doFillAreaSaveFailed`
  - `step:doFillConditionChoiceAfterTargetList`
  - `step:doFillConditionChoiceBeforeTargetList`
  - `step:doFillConditionSavingThrow`
  - `step:doFillTargetListAfterConditionChoice`
  - `step:doFillTargetListBeforeConditionChoice`
  - `step:doSubmitDamageBeforeSavingThrow`
- Allowed inputs used:
  - `cleanroom-input/MANIFEST.md`
  - `cleanroom-input/branch-coverage/source-branch-inventory.json`
  - `cleanroom-input/branch-coverage/reducer-route-inventory.json`
  - `cleanroom-input/qnt/battle-runtime/battle-runtime-save-gated-spell-ordering.mbt.qnt`
  - `cleanroom-input/qnt/battle-runtime/battle-runtime-save-gated-spell-ordering.route.mbt.qnt`
  - `cleanroom-input/guidance/README.md`
  - `cleanroom-input/guidance/reducer-spine.md`
  - `cleanroom-input/domain/UBIQUITOUS_LANGUAGE.md`
  - `cleanroom-input/domain/CLEANROOM_ASSUMPTIONS.md`
  - `cleanroom-input/raw/srd-5.2.1/Playing-the-Game.md`
  - `cleanroom-input/raw/srd-5.2.1/Rules-Glossary.md`
  - `cleanroom-input/raw/srd-5.2.1/Spells/Gaining-and-Casting.md`

Behavior implemented:

- Replayed `Save-Gated Spell Ordering` through the shared BattleState reducer surface using `start_battle`, `discover_battle_acts`, and `resolve_battle_subject` route events from the copied qRoute connector.
- Durable state remains battle-owned; QNT action names, sampled picks, trace ids, and projection hashes are quarantined in the adapter and target replay evidence.

Generated branch coverage:

| Obligation | Target replay evidence | Diagnostic tests | Status |
| --- | --- | --- | --- |
| `cleanroom-input/qnt/battle-runtime/battle-runtime-save-gated-spell-ordering.mbt.qnt#step:doDiscoverAreaSaveDamage` | `tasks/target-replay-evidence/T002-battle-runtime-save-gated-spell-ordering.json#T002 seed=1 action=doDiscoverAreaSaveDamage#step:doDiscoverAreaSaveDamage` | `src/tests/mod.rs` | `covered` |
| `cleanroom-input/qnt/battle-runtime/battle-runtime-save-gated-spell-ordering.mbt.qnt#step:doDiscoverTargetListConditionChoice` | `tasks/target-replay-evidence/T002-battle-runtime-save-gated-spell-ordering.json#T002 seed=1 action=doDiscoverTargetListConditionChoice#step:doDiscoverTargetListConditionChoice` | `src/tests/mod.rs` | `covered` |
| `cleanroom-input/qnt/battle-runtime/battle-runtime-save-gated-spell-ordering.mbt.qnt#step:doFillAreaDamageDice` | `tasks/target-replay-evidence/T002-battle-runtime-save-gated-spell-ordering.json#T002 seed=1 action=doFillAreaDamageDice#step:doFillAreaDamageDice` | `src/tests/mod.rs` | `covered` |
| `cleanroom-input/qnt/battle-runtime/battle-runtime-save-gated-spell-ordering.mbt.qnt#step:doFillAreaSaveFailed` | `tasks/target-replay-evidence/T002-battle-runtime-save-gated-spell-ordering.json#T002 seed=1 action=doFillAreaSaveFailed#step:doFillAreaSaveFailed` | `src/tests/mod.rs` | `covered` |
| `cleanroom-input/qnt/battle-runtime/battle-runtime-save-gated-spell-ordering.mbt.qnt#step:doFillConditionChoiceAfterTargetList` | `tasks/target-replay-evidence/T002-battle-runtime-save-gated-spell-ordering.json#T002 seed=1 action=doFillConditionChoiceAfterTargetList#step:doFillConditionChoiceAfterTargetList` | `src/tests/mod.rs` | `covered` |
| `cleanroom-input/qnt/battle-runtime/battle-runtime-save-gated-spell-ordering.mbt.qnt#step:doFillConditionChoiceBeforeTargetList` | `tasks/target-replay-evidence/T002-battle-runtime-save-gated-spell-ordering.json#T002 seed=1 action=doFillConditionChoiceBeforeTargetList#step:doFillConditionChoiceBeforeTargetList` | `src/tests/mod.rs` | `covered` |
| `cleanroom-input/qnt/battle-runtime/battle-runtime-save-gated-spell-ordering.mbt.qnt#step:doFillConditionSavingThrow` | `tasks/target-replay-evidence/T002-battle-runtime-save-gated-spell-ordering.json#T002 seed=1 action=doFillConditionSavingThrow#step:doFillConditionSavingThrow` | `src/tests/mod.rs` | `covered` |
| `cleanroom-input/qnt/battle-runtime/battle-runtime-save-gated-spell-ordering.mbt.qnt#step:doFillTargetListAfterConditionChoice` | `tasks/target-replay-evidence/T002-battle-runtime-save-gated-spell-ordering.json#T002 seed=1 action=doFillTargetListAfterConditionChoice#step:doFillTargetListAfterConditionChoice` | `src/tests/mod.rs` | `covered` |
| `cleanroom-input/qnt/battle-runtime/battle-runtime-save-gated-spell-ordering.mbt.qnt#step:doFillTargetListBeforeConditionChoice` | `tasks/target-replay-evidence/T002-battle-runtime-save-gated-spell-ordering.json#T002 seed=1 action=doFillTargetListBeforeConditionChoice#step:doFillTargetListBeforeConditionChoice` | `src/tests/mod.rs` | `covered` |
| `cleanroom-input/qnt/battle-runtime/battle-runtime-save-gated-spell-ordering.mbt.qnt#step:doSubmitDamageBeforeSavingThrow` | `tasks/target-replay-evidence/T002-battle-runtime-save-gated-spell-ordering.json#T002 seed=1 action=doSubmitDamageBeforeSavingThrow#step:doSubmitDamageBeforeSavingThrow` | `src/tests/mod.rs` | `covered` |

Target replay evidence:

- Evidence file: `tasks/target-replay-evidence/T002-battle-runtime-save-gated-spell-ordering.json`
- Target profile: `rust`
- Target profile SHA-256: `6d4cc6c6a4769962798133d57aff01438fb2b661941f71d1aa8a3333f4b7ecc1`
- Quint binding: Rust quint-connect harness
- Reproduction seed or trace id: `T002 seed=1`

Harness artifacts:

- Start gate: `tasks/history/T002/START_GATE.json`
- Engine depth: `tasks/history/T002/ENGINE_DEPTH_MANIFEST.json`
- State ownership: `tasks/history/T002/STATE_OWNER_MANIFEST.json`
- Reviewer loop: `tasks/history/T002/REVIEW_LOOP.json`
- Decider decision: `tasks/history/T002/DECIDER_DECISION.json`
- Run ledger: `tasks/RUN_LEDGER.json`

Remaining gaps:

- `_none_`

Verification results:

- `cargo fmt --check` passed.
- `cargo test` passed.
- `cargo clippy --all-targets -- -D warnings` passed.
- `node scripts/check-cleanroom-harness.cjs` passed.

## T003: Hit-Point Restoration Ordering

- Manifest source commit SHA: `564376fd95218a209bb9eae5c9ccb54ca3e04a52`
- Source branch inventory SHA: `4bb2b20a85d94e3b90b7c59cbfe6e1edd5ab3ef40410641e999527861f3d3a32`
- Driver: `cleanroom-input/qnt/battle-runtime/battle-runtime-hit-point-restoration-ordering.mbt.qnt`
- Branch obligations:
  - `step:doDiscoverFeatureHealingPool`
  - `step:doDiscoverSingleTargetSpellHealing`
  - `step:doDiscoverTargetListSpellHealing`
  - `step:doFillFeatureHealingDistribution`
  - `step:doFillSpellHealingRoll`
  - `step:doFillSpellHealingTargetChoice`
  - `step:doFillSpellHealingTargetList`
  - `step:doSubmitHealingRollBeforeTargetChoice`
  - `step:doSubmitHealingRollBeforeTargetList`
- Allowed inputs used:
  - `cleanroom-input/MANIFEST.md`
  - `cleanroom-input/branch-coverage/source-branch-inventory.json`
  - `cleanroom-input/branch-coverage/reducer-route-inventory.json`
  - `cleanroom-input/qnt/battle-runtime/battle-runtime-hit-point-restoration-ordering.mbt.qnt`
  - `cleanroom-input/qnt/battle-runtime/battle-runtime-hit-point-restoration-ordering.route.mbt.qnt`
  - `cleanroom-input/guidance/README.md`
  - `cleanroom-input/guidance/reducer-spine.md`
  - `cleanroom-input/domain/UBIQUITOUS_LANGUAGE.md`
  - `cleanroom-input/domain/CLEANROOM_ASSUMPTIONS.md`
  - `cleanroom-input/raw/srd-5.2.1/Playing-the-Game.md`
  - `cleanroom-input/raw/srd-5.2.1/Rules-Glossary.md`
  - `cleanroom-input/raw/srd-5.2.1/Spells/Gaining-and-Casting.md`

Behavior implemented:

- Replayed `Hit-Point Restoration Ordering` through the shared BattleState reducer surface using `start_battle`, `discover_battle_acts`, and `resolve_battle_subject` route events from the copied qRoute connector.
- Durable state remains battle-owned; QNT action names, sampled picks, trace ids, and projection hashes are quarantined in the adapter and target replay evidence.

Generated branch coverage:

| Obligation | Target replay evidence | Diagnostic tests | Status |
| --- | --- | --- | --- |
| `cleanroom-input/qnt/battle-runtime/battle-runtime-hit-point-restoration-ordering.mbt.qnt#step:doDiscoverFeatureHealingPool` | `tasks/target-replay-evidence/T003-battle-runtime-hit-point-restoration-ordering.json#T003 seed=1 action=doDiscoverFeatureHealingPool#step:doDiscoverFeatureHealingPool` | `src/tests/mod.rs` | `covered` |
| `cleanroom-input/qnt/battle-runtime/battle-runtime-hit-point-restoration-ordering.mbt.qnt#step:doDiscoverSingleTargetSpellHealing` | `tasks/target-replay-evidence/T003-battle-runtime-hit-point-restoration-ordering.json#T003 seed=1 action=doDiscoverSingleTargetSpellHealing#step:doDiscoverSingleTargetSpellHealing` | `src/tests/mod.rs` | `covered` |
| `cleanroom-input/qnt/battle-runtime/battle-runtime-hit-point-restoration-ordering.mbt.qnt#step:doDiscoverTargetListSpellHealing` | `tasks/target-replay-evidence/T003-battle-runtime-hit-point-restoration-ordering.json#T003 seed=1 action=doDiscoverTargetListSpellHealing#step:doDiscoverTargetListSpellHealing` | `src/tests/mod.rs` | `covered` |
| `cleanroom-input/qnt/battle-runtime/battle-runtime-hit-point-restoration-ordering.mbt.qnt#step:doFillFeatureHealingDistribution` | `tasks/target-replay-evidence/T003-battle-runtime-hit-point-restoration-ordering.json#T003 seed=1 action=doFillFeatureHealingDistribution#step:doFillFeatureHealingDistribution` | `src/tests/mod.rs` | `covered` |
| `cleanroom-input/qnt/battle-runtime/battle-runtime-hit-point-restoration-ordering.mbt.qnt#step:doFillSpellHealingRoll` | `tasks/target-replay-evidence/T003-battle-runtime-hit-point-restoration-ordering.json#T003 seed=1 action=doFillSpellHealingRoll#step:doFillSpellHealingRoll` | `src/tests/mod.rs` | `covered` |
| `cleanroom-input/qnt/battle-runtime/battle-runtime-hit-point-restoration-ordering.mbt.qnt#step:doFillSpellHealingTargetChoice` | `tasks/target-replay-evidence/T003-battle-runtime-hit-point-restoration-ordering.json#T003 seed=1 action=doFillSpellHealingTargetChoice#step:doFillSpellHealingTargetChoice` | `src/tests/mod.rs` | `covered` |
| `cleanroom-input/qnt/battle-runtime/battle-runtime-hit-point-restoration-ordering.mbt.qnt#step:doFillSpellHealingTargetList` | `tasks/target-replay-evidence/T003-battle-runtime-hit-point-restoration-ordering.json#T003 seed=1 action=doFillSpellHealingTargetList#step:doFillSpellHealingTargetList` | `src/tests/mod.rs` | `covered` |
| `cleanroom-input/qnt/battle-runtime/battle-runtime-hit-point-restoration-ordering.mbt.qnt#step:doSubmitHealingRollBeforeTargetChoice` | `tasks/target-replay-evidence/T003-battle-runtime-hit-point-restoration-ordering.json#T003 seed=1 action=doSubmitHealingRollBeforeTargetChoice#step:doSubmitHealingRollBeforeTargetChoice` | `src/tests/mod.rs` | `covered` |
| `cleanroom-input/qnt/battle-runtime/battle-runtime-hit-point-restoration-ordering.mbt.qnt#step:doSubmitHealingRollBeforeTargetList` | `tasks/target-replay-evidence/T003-battle-runtime-hit-point-restoration-ordering.json#T003 seed=1 action=doSubmitHealingRollBeforeTargetList#step:doSubmitHealingRollBeforeTargetList` | `src/tests/mod.rs` | `covered` |

Target replay evidence:

- Evidence file: `tasks/target-replay-evidence/T003-battle-runtime-hit-point-restoration-ordering.json`
- Target profile: `rust`
- Target profile SHA-256: `6d4cc6c6a4769962798133d57aff01438fb2b661941f71d1aa8a3333f4b7ecc1`
- Quint binding: Rust quint-connect harness
- Reproduction seed or trace id: `T003 seed=1`

Harness artifacts:

- Start gate: `tasks/history/T003/START_GATE.json`
- Engine depth: `tasks/history/T003/ENGINE_DEPTH_MANIFEST.json`
- State ownership: `tasks/history/T003/STATE_OWNER_MANIFEST.json`
- Reviewer loop: `tasks/history/T003/REVIEW_LOOP.json`
- Decider decision: `tasks/history/T003/DECIDER_DECISION.json`
- Run ledger: `tasks/RUN_LEDGER.json`

Remaining gaps:

- `_none_`

Verification results:

- `cargo fmt --check` passed.
- `cargo test` passed.
- `cargo clippy --all-targets -- -D warnings` passed.
- `node scripts/check-cleanroom-harness.cjs` passed.

## T004: Death Saving Throw

- Manifest source commit SHA: `564376fd95218a209bb9eae5c9ccb54ca3e04a52`
- Source branch inventory SHA: `4bb2b20a85d94e3b90b7c59cbfe6e1edd5ab3ef40410641e999527861f3d3a32`
- Driver: `cleanroom-input/qnt/battle-runtime/battle-runtime-death-saving-throw.mbt.qnt`
- Branch obligations:
  - `step:doDiscoverEndTurnDeathSavingThrow`
  - `step:doFillDeathSavingThrow`
  - `step:doRejectWrongActorEndTurnAfterResolved`
- Allowed inputs used:
  - `cleanroom-input/MANIFEST.md`
  - `cleanroom-input/branch-coverage/source-branch-inventory.json`
  - `cleanroom-input/branch-coverage/reducer-route-inventory.json`
  - `cleanroom-input/qnt/battle-runtime/battle-runtime-death-saving-throw.mbt.qnt`
  - `cleanroom-input/qnt/battle-runtime/battle-runtime-death-saving-throw.route.mbt.qnt`
  - `cleanroom-input/guidance/README.md`
  - `cleanroom-input/guidance/reducer-spine.md`
  - `cleanroom-input/domain/UBIQUITOUS_LANGUAGE.md`
  - `cleanroom-input/domain/CLEANROOM_ASSUMPTIONS.md`
  - `cleanroom-input/raw/srd-5.2.1/Playing-the-Game.md`
  - `cleanroom-input/raw/srd-5.2.1/Rules-Glossary.md`
  - `cleanroom-input/raw/srd-5.2.1/Spells/Gaining-and-Casting.md`

Behavior implemented:

- Replayed `Death Saving Throw` through the shared BattleState reducer surface using `start_battle`, `discover_battle_acts`, and `resolve_battle_subject` route events from the copied qRoute connector.
- Durable state remains battle-owned; QNT action names, sampled picks, trace ids, and projection hashes are quarantined in the adapter and target replay evidence.

Generated branch coverage:

| Obligation | Target replay evidence | Diagnostic tests | Status |
| --- | --- | --- | --- |
| `cleanroom-input/qnt/battle-runtime/battle-runtime-death-saving-throw.mbt.qnt#step:doDiscoverEndTurnDeathSavingThrow` | `tasks/target-replay-evidence/T004-battle-runtime-death-saving-throw.json#T004 seed=1 action=doDiscoverEndTurnDeathSavingThrow#step:doDiscoverEndTurnDeathSavingThrow` | `src/tests/mod.rs` | `covered` |
| `cleanroom-input/qnt/battle-runtime/battle-runtime-death-saving-throw.mbt.qnt#step:doFillDeathSavingThrow` | `tasks/target-replay-evidence/T004-battle-runtime-death-saving-throw.json#T004 seed=1 action=doFillDeathSavingThrow#step:doFillDeathSavingThrow` | `src/tests/mod.rs` | `covered` |
| `cleanroom-input/qnt/battle-runtime/battle-runtime-death-saving-throw.mbt.qnt#step:doRejectWrongActorEndTurnAfterResolved` | `tasks/target-replay-evidence/T004-battle-runtime-death-saving-throw.json#T004 seed=1 action=doRejectWrongActorEndTurnAfterResolved#step:doRejectWrongActorEndTurnAfterResolved` | `src/tests/mod.rs` | `covered` |

Target replay evidence:

- Evidence file: `tasks/target-replay-evidence/T004-battle-runtime-death-saving-throw.json`
- Target profile: `rust`
- Target profile SHA-256: `6d4cc6c6a4769962798133d57aff01438fb2b661941f71d1aa8a3333f4b7ecc1`
- Quint binding: Rust quint-connect harness
- Reproduction seed or trace id: `T004 seed=1`

Harness artifacts:

- Start gate: `tasks/history/T004/START_GATE.json`
- Engine depth: `tasks/history/T004/ENGINE_DEPTH_MANIFEST.json`
- State ownership: `tasks/history/T004/STATE_OWNER_MANIFEST.json`
- Reviewer loop: `tasks/history/T004/REVIEW_LOOP.json`
- Decider decision: `tasks/history/T004/DECIDER_DECISION.json`
- Run ledger: `tasks/RUN_LEDGER.json`

Remaining gaps:

- `_none_`

Verification results:

- `cargo fmt --check` passed.
- `cargo test` passed.
- `cargo clippy --all-targets -- -D warnings` passed.
- `node scripts/check-cleanroom-harness.cjs` passed.

## T005: Concentration Break Teardown

- Manifest source commit SHA: `564376fd95218a209bb9eae5c9ccb54ca3e04a52`
- Source branch inventory SHA: `4bb2b20a85d94e3b90b7c59cbfe6e1edd5ab3ef40410641e999527861f3d3a32`
- Driver: `cleanroom-input/qnt/battle-runtime/battle-runtime-concentration-break-teardown.mbt.qnt`
- Branch obligations:
  - `step:doCastConcentrationSpell`
  - `step:doCastReplacementConcentrationSpell`
  - `step:doDamageRequestsConcentrationSave`
  - `step:doFailConcentrationSave`
  - `step:doVoluntaryEndConcentration`
- Allowed inputs used:
  - `cleanroom-input/MANIFEST.md`
  - `cleanroom-input/branch-coverage/source-branch-inventory.json`
  - `cleanroom-input/branch-coverage/reducer-route-inventory.json`
  - `cleanroom-input/qnt/battle-runtime/battle-runtime-concentration-break-teardown.mbt.qnt`
  - `cleanroom-input/qnt/battle-runtime/battle-runtime-concentration-break-teardown.route.mbt.qnt`
  - `cleanroom-input/guidance/README.md`
  - `cleanroom-input/guidance/reducer-spine.md`
  - `cleanroom-input/domain/UBIQUITOUS_LANGUAGE.md`
  - `cleanroom-input/domain/CLEANROOM_ASSUMPTIONS.md`
  - `cleanroom-input/raw/srd-5.2.1/Playing-the-Game.md`
  - `cleanroom-input/raw/srd-5.2.1/Rules-Glossary.md`
  - `cleanroom-input/raw/srd-5.2.1/Spells/Gaining-and-Casting.md`

Behavior implemented:

- Replayed `Concentration Break Teardown` through the shared BattleState reducer surface using `start_battle`, `discover_battle_acts`, and `resolve_battle_subject` route events from the copied qRoute connector.
- Durable state remains battle-owned; QNT action names, sampled picks, trace ids, and projection hashes are quarantined in the adapter and target replay evidence.

Generated branch coverage:

| Obligation | Target replay evidence | Diagnostic tests | Status |
| --- | --- | --- | --- |
| `cleanroom-input/qnt/battle-runtime/battle-runtime-concentration-break-teardown.mbt.qnt#step:doCastConcentrationSpell` | `tasks/target-replay-evidence/T005-battle-runtime-concentration-break-teardown.json#T005 seed=1 action=doCastConcentrationSpell#step:doCastConcentrationSpell` | `src/tests/mod.rs` | `covered` |
| `cleanroom-input/qnt/battle-runtime/battle-runtime-concentration-break-teardown.mbt.qnt#step:doCastReplacementConcentrationSpell` | `tasks/target-replay-evidence/T005-battle-runtime-concentration-break-teardown.json#T005 seed=1 action=doCastReplacementConcentrationSpell#step:doCastReplacementConcentrationSpell` | `src/tests/mod.rs` | `covered` |
| `cleanroom-input/qnt/battle-runtime/battle-runtime-concentration-break-teardown.mbt.qnt#step:doDamageRequestsConcentrationSave` | `tasks/target-replay-evidence/T005-battle-runtime-concentration-break-teardown.json#T005 seed=1 action=doDamageRequestsConcentrationSave#step:doDamageRequestsConcentrationSave` | `src/tests/mod.rs` | `covered` |
| `cleanroom-input/qnt/battle-runtime/battle-runtime-concentration-break-teardown.mbt.qnt#step:doFailConcentrationSave` | `tasks/target-replay-evidence/T005-battle-runtime-concentration-break-teardown.json#T005 seed=1 action=doFailConcentrationSave#step:doFailConcentrationSave` | `src/tests/mod.rs` | `covered` |
| `cleanroom-input/qnt/battle-runtime/battle-runtime-concentration-break-teardown.mbt.qnt#step:doVoluntaryEndConcentration` | `tasks/target-replay-evidence/T005-battle-runtime-concentration-break-teardown.json#T005 seed=1 action=doVoluntaryEndConcentration#step:doVoluntaryEndConcentration` | `src/tests/mod.rs` | `covered` |

Target replay evidence:

- Evidence file: `tasks/target-replay-evidence/T005-battle-runtime-concentration-break-teardown.json`
- Target profile: `rust`
- Target profile SHA-256: `6d4cc6c6a4769962798133d57aff01438fb2b661941f71d1aa8a3333f4b7ecc1`
- Quint binding: Rust quint-connect harness
- Reproduction seed or trace id: `T005 seed=1`

Harness artifacts:

- Start gate: `tasks/history/T005/START_GATE.json`
- Engine depth: `tasks/history/T005/ENGINE_DEPTH_MANIFEST.json`
- State ownership: `tasks/history/T005/STATE_OWNER_MANIFEST.json`
- Reviewer loop: `tasks/history/T005/REVIEW_LOOP.json`
- Decider decision: `tasks/history/T005/DECIDER_DECISION.json`
- Run ledger: `tasks/RUN_LEDGER.json`

Remaining gaps:

- `_none_`

Verification results:

- `cargo fmt --check` passed.
- `cargo test` passed.
- `cargo clippy --all-targets -- -D warnings` passed.
- `node scripts/check-cleanroom-harness.cjs` passed.

## RRCONV-19B: Act Discovery Contract

- Manifest source commit SHA: `564376fd95218a209bb9eae5c9ccb54ca3e04a52`
- Source branch inventory SHA: `4bb2b20a85d94e3b90b7c59cbfe6e1edd5ab3ef40410641e999527861f3d3a32`
- Driver used for dirty replay evidence: `cleanroom-input/qnt/battle-runtime/battle-runtime-weapon-attack-skeleton.mbt.qnt`
- Machine-readable run ledger: `tasks/RUN_LEDGER.json`
- Task artifacts: `tasks/history/RRCONV-19B-RUST-ACT-DISCOVERY-CONTRACT/`
- Cleanroom freshness: dirty cleanroom acceleration evidence only; this does not claim fresh package acceptance.

Allowed inputs used:

- `cleanroom-input/MANIFEST.md`
- `cleanroom-input/branch-coverage/source-branch-inventory.json`
- `cleanroom-input/branch-coverage/reducer-route-inventory.json`
- `cleanroom-input/qnt/battle-runtime/battle-runtime-weapon-attack-skeleton.mbt.qnt`
- `cleanroom-input/qnt/battle-runtime/battle-runtime-weapon-attack-skeleton.route.mbt.qnt`
- `cleanroom-input/guidance/reducer-spine.md`
- `cleanroom-input/domain/UBIQUITOUS_LANGUAGE.md`
- Repo-local `src/**`, `tasks/**`, and Rust/Cargo tooling

Behavior implemented:

- Added `BattleActDiscoveryResult` as the typed public result returned by `discover_battle_acts` and `discover_battle_acts_observed`.
- Kept discovered acts owned by the result and exposed intentional `available_acts` / `into_available_acts` accessors plus derivable current-actor and action-availability metadata.
- Updated cleanroom adapters and reducer-entrypoint tests to consume the typed discovery result through the shared reducer entrypoint.
- Target replay evidence for RRCONV-19B is scoped to `src/qnt_adapters/battle_runtime_weapon_attack_skeleton.rs`; other adapter changes in this diff are mechanical typed-result consumers and are not claimed as RRCONV-19B replay-evidence coverage.
- Added no durable `BattleState` fields for this task.

Generated branch coverage:

| Obligation | Target replay evidence | Harness adapter check | Status |
| --- | --- | --- | --- |
| `cleanroom-input/qnt/battle-runtime/battle-runtime-weapon-attack-skeleton.mbt.qnt#step:doDiscoverAttack` | `tasks/target-replay-evidence/RRCONV-19B-act-discovery-contract.json#RRCONV-19B dirty replay action=doDiscoverAttack#step:doDiscoverAttack` | `weapon_attack_skeleton_adapter_replays_all_branches` | `covered` |
| `cleanroom-input/qnt/battle-runtime/battle-runtime-weapon-attack-skeleton.mbt.qnt#step:doFillAttackRollHit` | `tasks/target-replay-evidence/RRCONV-19B-act-discovery-contract.json#RRCONV-19B dirty replay action=doFillAttackRollHit#step:doFillAttackRollHit` | `weapon_attack_skeleton_adapter_replays_all_branches` | `covered` |
| `cleanroom-input/qnt/battle-runtime/battle-runtime-weapon-attack-skeleton.mbt.qnt#step:doFillAttackRollMiss` | `tasks/target-replay-evidence/RRCONV-19B-act-discovery-contract.json#RRCONV-19B dirty replay action=doFillAttackRollMiss#step:doFillAttackRollMiss` | `weapon_attack_skeleton_adapter_replays_all_branches` | `covered` |
| `cleanroom-input/qnt/battle-runtime/battle-runtime-weapon-attack-skeleton.mbt.qnt#step:doFillDamageHigh` | `tasks/target-replay-evidence/RRCONV-19B-act-discovery-contract.json#RRCONV-19B dirty replay action=doFillDamageHigh#step:doFillDamageHigh` | `weapon_attack_skeleton_adapter_replays_all_branches` | `covered` |
| `cleanroom-input/qnt/battle-runtime/battle-runtime-weapon-attack-skeleton.mbt.qnt#step:doFillDamageHighSneakAttack` | `tasks/target-replay-evidence/RRCONV-19B-act-discovery-contract.json#RRCONV-19B dirty replay action=doFillDamageHighSneakAttack#step:doFillDamageHighSneakAttack` | `weapon_attack_skeleton_adapter_replays_all_branches` | `covered` |
| `cleanroom-input/qnt/battle-runtime/battle-runtime-weapon-attack-skeleton.mbt.qnt#step:doFillDamageLow` | `tasks/target-replay-evidence/RRCONV-19B-act-discovery-contract.json#RRCONV-19B dirty replay action=doFillDamageLow#step:doFillDamageLow` | `weapon_attack_skeleton_adapter_replays_all_branches` | `covered` |
| `cleanroom-input/qnt/battle-runtime/battle-runtime-weapon-attack-skeleton.mbt.qnt#step:doFillDamageLowSneakAttack` | `tasks/target-replay-evidence/RRCONV-19B-act-discovery-contract.json#RRCONV-19B dirty replay action=doFillDamageLowSneakAttack#step:doFillDamageLowSneakAttack` | `weapon_attack_skeleton_adapter_replays_all_branches` | `covered` |
| `cleanroom-input/qnt/battle-runtime/battle-runtime-weapon-attack-skeleton.mbt.qnt#step:doFillTarget` | `tasks/target-replay-evidence/RRCONV-19B-act-discovery-contract.json#RRCONV-19B dirty replay action=doFillTarget#step:doFillTarget` | `weapon_attack_skeleton_adapter_replays_all_branches` | `covered` |
| `cleanroom-input/qnt/battle-runtime/battle-runtime-weapon-attack-skeleton.mbt.qnt#step:doRejectRecursiveSkeletonMultiattack` | `tasks/target-replay-evidence/RRCONV-19B-act-discovery-contract.json#RRCONV-19B dirty replay action=doRejectRecursiveSkeletonMultiattack#step:doRejectRecursiveSkeletonMultiattack` | `weapon_attack_skeleton_adapter_replays_all_branches` | `covered` |
| `cleanroom-input/qnt/battle-runtime/battle-runtime-weapon-attack-skeleton.mbt.qnt#step:doRejectStaleAfterResolved` | `tasks/target-replay-evidence/RRCONV-19B-act-discovery-contract.json#RRCONV-19B dirty replay action=doRejectStaleAfterResolved#step:doRejectStaleAfterResolved` | `weapon_attack_skeleton_adapter_replays_all_branches` | `covered` |
| `cleanroom-input/qnt/battle-runtime/battle-runtime-weapon-attack-skeleton.mbt.qnt#step:doRejectWrongTarget` | `tasks/target-replay-evidence/RRCONV-19B-act-discovery-contract.json#RRCONV-19B dirty replay action=doRejectWrongTarget#step:doRejectWrongTarget` | `weapon_attack_skeleton_adapter_replays_all_branches` | `covered` |
| `cleanroom-input/qnt/battle-runtime/battle-runtime-weapon-attack-skeleton.mbt.qnt#step:doResolveSkeletonMultiattack` | `tasks/target-replay-evidence/RRCONV-19B-act-discovery-contract.json#RRCONV-19B dirty replay action=doResolveSkeletonMultiattack#step:doResolveSkeletonMultiattack` | `weapon_attack_skeleton_adapter_replays_all_branches` | `covered` |
| `cleanroom-input/qnt/battle-runtime/battle-runtime-weapon-attack-skeleton.mbt.qnt#step:doSpendSkeletonMultiattackDispatch` | `tasks/target-replay-evidence/RRCONV-19B-act-discovery-contract.json#RRCONV-19B dirty replay action=doSpendSkeletonMultiattackDispatch#step:doSpendSkeletonMultiattackDispatch` | `weapon_attack_skeleton_adapter_replays_all_branches` | `covered` |
| `cleanroom-input/qnt/battle-runtime/battle-runtime-weapon-attack-skeleton.mbt.qnt#step:doStartSkeletonTurn` | `tasks/target-replay-evidence/RRCONV-19B-act-discovery-contract.json#RRCONV-19B dirty replay action=doStartSkeletonTurn#step:doStartSkeletonTurn` | `weapon_attack_skeleton_adapter_replays_all_branches` | `covered` |

Target replay evidence:

- Evidence file: `tasks/target-replay-evidence/RRCONV-19B-act-discovery-contract.json`
- Target profile: `rust`
- Target profile SHA-256: `6d4cc6c6a4769962798133d57aff01438fb2b661941f71d1aa8a3333f4b7ecc1`
- Quint binding: Rust quint-connect harness
- Reproduction trace id prefix: `RRCONV-19B dirty replay action=`
- Checked route projection: `qRoute` with `route-event-list` comparator

Supplemental dirty diagnostics:

- `cargo test reducer_entrypoint_contract` asserts that callers receive `BattleActDiscoveryResult` from the shared discovery entrypoint and consume it intentionally.
- `cargo test experimental_qnt_spine` exercises reducer-spine weapon attack checks.
- `cargo test weapon_attack_skeleton_adapter_replays_all_branches` replays the selected weapon skeleton branch set through the adapter.

Harness artifacts:

- Start gate: `tasks/history/RRCONV-19B-RUST-ACT-DISCOVERY-CONTRACT/START_GATE.json`
- Engine depth: `tasks/history/RRCONV-19B-RUST-ACT-DISCOVERY-CONTRACT/ENGINE_DEPTH_MANIFEST.json`
- State ownership: `tasks/history/RRCONV-19B-RUST-ACT-DISCOVERY-CONTRACT/STATE_OWNER_MANIFEST.json`
- Reviewer loop: `tasks/history/RRCONV-19B-RUST-ACT-DISCOVERY-CONTRACT/REVIEW_LOOP.json`
- Decider decision: `tasks/history/RRCONV-19B-RUST-ACT-DISCOVERY-CONTRACT/DECIDER_DECISION.json`
- Run ledger: `tasks/RUN_LEDGER.json`

Remaining gaps:

- Fresh cleanroom package acceptance is not claimed by RRCONV-19B; this remains dirty cleanroom acceleration evidence.

Verification results:

- `cargo fmt --check` passed.
- `cargo test reducer_entrypoint_contract` passed.
- `cargo test experimental_qnt_spine` passed.
- `cargo test weapon_attack_skeleton_adapter_replays_all_branches` passed.
- `node scripts/check-cleanroom-harness.cjs` passed.
- `git diff --check 1c805870c08b6632dba560019c2c7a75dc5ed991...HEAD` passed.

## RRCONV-19D-RUST-TURN-ADVANCE-RESULT: Turn Advance Result

- Manifest source commit SHA: `564376fd95218a209bb9eae5c9ccb54ca3e04a52`
- Source branch inventory SHA: `4bb2b20a85d94e3b90b7c59cbfe6e1edd5ab3ef40410641e999527861f3d3a32`
- Driver: `cleanroom-input/qnt/battle-runtime/battle-runtime-reducer-spine-contract.mbt.qnt`
- Supplemental exercised driver: `cleanroom-input/qnt/battle-runtime/battle-runtime-turn-boundary-effect-lifecycle.mbt.qnt`
- Machine-readable run ledger: `tasks/RUN_LEDGER.json`
- Task artifacts: `tasks/history/RRCONV-19D-RUST-TURN-ADVANCE-RESULT/`
- Cleanroom freshness: dirty cleanroom evidence only; this does not claim fresh package acceptance. RRCONV-19D fixer ran in a worktree with a pre-existing untracked final report file preserved outside committed artifacts.

Allowed inputs used:

- `cleanroom-input/MANIFEST.md`
- `cleanroom-input/branch-coverage/source-branch-inventory.json`
- `cleanroom-input/branch-coverage/reducer-route-inventory.json`
- `cleanroom-input/qnt/battle-runtime/battle-runtime-reducer-spine-contract.mbt.qnt`
- `cleanroom-input/qnt/battle-runtime/battle-runtime-turn-boundary-effect-lifecycle.mbt.qnt`
- `cleanroom-input/qnt/battle-runtime/battle-runtime-turn-order.qnt`
- `cleanroom-input/raw/srd-5.2.1/Playing-the-Game.md`
- `cleanroom-input/domain/UBIQUITOUS_LANGUAGE.md`
- `cleanroom-input/domain/CLEANROOM_ASSUMPTIONS.md`
- Repo-local `src/**`, `tasks/**`, and Rust/Cargo tooling

Behavior implemented:

- Added typed `BattleTurnAdvanceResult` with next `BattleState`, previous actor, next actor, and resulting round.
- Changed public `advance_turn` and `advance_turn_observed` to return the typed result; `advance_turn_state` is the explicit state-only compatibility helper.
- Fixer note: the original RRCONV-19D artifact claim was too broad. This patch changes `doEndTurnToTarget` and the turn-boundary lifecycle adapter projections so `qCurrentActor`, `qActor`, and `qRound` for turn advancement are sourced from `BattleTurnAdvanceResult.previous_actor`, `BattleTurnAdvanceResult.next_actor`, and `BattleTurnAdvanceResult.round`; `BattleState` remains the source for non-advancement state fields.
- No new durable `BattleState` fields were added.

Generated branch coverage:

| Obligation | Target replay evidence | Harness adapter check | Status |
| --- | --- | --- | --- |
| `cleanroom-input/qnt/battle-runtime/battle-runtime-reducer-spine-contract.mbt.qnt#step:doStartBattle` | `tasks/target-replay-evidence/RRCONV-19D-turn-advance-result.json#RRCONV-19D dirty replay action=doStartBattle#step:doStartBattle` | `reducer_spine_contract_adapter_replays_all_branches` | `historical` |
| `cleanroom-input/qnt/battle-runtime/battle-runtime-reducer-spine-contract.mbt.qnt#step:doDiscoverSlotSpell` | `tasks/target-replay-evidence/RRCONV-19D-turn-advance-result.json#RRCONV-19D dirty replay action=doDiscoverSlotSpell#step:doDiscoverSlotSpell` | `reducer_spine_contract_adapter_replays_all_branches` | `historical` |
| `cleanroom-input/qnt/battle-runtime/battle-runtime-reducer-spine-contract.mbt.qnt#step:doResolveSlotSpellTargets` | `tasks/target-replay-evidence/RRCONV-19D-turn-advance-result.json#RRCONV-19D dirty replay action=doResolveSlotSpellTargets#step:doResolveSlotSpellTargets` | `reducer_spine_contract_adapter_replays_all_branches` | `historical` |
| `cleanroom-input/qnt/battle-runtime/battle-runtime-reducer-spine-contract.mbt.qnt#step:doResolveSlotSpellDamage` | `tasks/target-replay-evidence/RRCONV-19D-turn-advance-result.json#RRCONV-19D dirty replay action=doResolveSlotSpellDamage#step:doResolveSlotSpellDamage` | `reducer_spine_contract_adapter_replays_all_branches` | `historical` |
| `cleanroom-input/qnt/battle-runtime/battle-runtime-reducer-spine-contract.mbt.qnt#step:doEndTurnToTarget` | `tasks/target-replay-evidence/RRCONV-19D-turn-advance-result.json#RRCONV-19D dirty replay action=doEndTurnToTarget#step:doEndTurnToTarget` | `reducer_spine_contract_adapter_replays_all_branches` | `historical` |
| `cleanroom-input/qnt/battle-runtime/battle-runtime-reducer-spine-contract.mbt.qnt#step:doDiscoverWeaponAttack` | `tasks/target-replay-evidence/RRCONV-19D-turn-advance-result.json#RRCONV-19D dirty replay action=doDiscoverWeaponAttack#step:doDiscoverWeaponAttack` | `reducer_spine_contract_adapter_replays_all_branches` | `historical` |
| `cleanroom-input/qnt/battle-runtime/battle-runtime-reducer-spine-contract.mbt.qnt#step:doResolveWeaponTarget` | `tasks/target-replay-evidence/RRCONV-19D-turn-advance-result.json#RRCONV-19D dirty replay action=doResolveWeaponTarget#step:doResolveWeaponTarget` | `reducer_spine_contract_adapter_replays_all_branches` | `historical` |
| `cleanroom-input/qnt/battle-runtime/battle-runtime-reducer-spine-contract.mbt.qnt#step:doResolveWeaponAttackHit` | `tasks/target-replay-evidence/RRCONV-19D-turn-advance-result.json#RRCONV-19D dirty replay action=doResolveWeaponAttackHit#step:doResolveWeaponAttackHit` | `reducer_spine_contract_adapter_replays_all_branches` | `historical` |
| `cleanroom-input/qnt/battle-runtime/battle-runtime-reducer-spine-contract.mbt.qnt#step:doResolveWeaponDamage` | `tasks/target-replay-evidence/RRCONV-19D-turn-advance-result.json#RRCONV-19D dirty replay action=doResolveWeaponDamage#step:doResolveWeaponDamage` | `reducer_spine_contract_adapter_replays_all_branches` | `historical` |

Supplemental dirty diagnostics:

- `cargo test experimental_qnt_spine_advances_turn_boundary_lifecycle` exercises `battle_runtime_turn_boundary_effect_lifecycle.rs` with `BattleTurnAdvanceResult` metadata as the projection input for `qActor` and `qRound`; this is not recorded as ledger-closing target replay evidence for the RRCONV-19D selected driver.

Target replay evidence:

- Evidence file: `tasks/target-replay-evidence/RRCONV-19D-turn-advance-result.json`
- Target profile: `rust`
- Target profile SHA-256: `6d4cc6c6a4769962798133d57aff01438fb2b661941f71d1aa8a3333f4b7ecc1`
- Quint binding: Rust quint-connect harness
- Reproduction trace id prefix: `RRCONV-19D dirty replay action=`
- Checked route projection: `qRoute` with `route-event-list` comparator; the turn-advance rows name `BattleTurnAdvanceResult` fields as checked projection inputs.

Harness artifacts:

- Start gate: `tasks/history/RRCONV-19D-RUST-TURN-ADVANCE-RESULT/START_GATE.json`
- Engine depth: `tasks/history/RRCONV-19D-RUST-TURN-ADVANCE-RESULT/ENGINE_DEPTH_MANIFEST.json`
- State ownership: `tasks/history/RRCONV-19D-RUST-TURN-ADVANCE-RESULT/STATE_OWNER_MANIFEST.json`
- Reviewer loop: `tasks/history/RRCONV-19D-RUST-TURN-ADVANCE-RESULT/REVIEW_LOOP.json`
- Decider decision: `tasks/history/RRCONV-19D-RUST-TURN-ADVANCE-RESULT/DECIDER_DECISION.json`
- Run ledger: `tasks/RUN_LEDGER.json`

Remaining gaps:

- Fresh cleanroom package acceptance is not claimed by RRCONV-19D; this remains dirty cleanroom acceleration evidence.

Verification results:

- `cargo fmt --check` passed.
- `cargo test reducer_entrypoint_contract` passed.
- `cargo test experimental_qnt_spine_advances_turn_boundary_lifecycle` passed.
- `cargo test experimental_qnt_spine` passed.
- `cargo clippy --all-targets -- -D warnings` passed.
- `node scripts/check-cleanroom-harness.cjs` passed.
- `git diff --check 1c805870c08b6632dba560019c2c7a75dc5ed991...HEAD` passed.

## RRCONV-19C-RUST-RESOLUTION-RESULT-CONTRACT: Resolution Result Contract

- Manifest source commit SHA: `564376fd95218a209bb9eae5c9ccb54ca3e04a52`
- Source branch inventory SHA: `4bb2b20a85d94e3b90b7c59cbfe6e1edd5ab3ef40410641e999527861f3d3a32`
- Driver: `cleanroom-input/qnt/battle-runtime/battle-runtime-reducer-spine-contract.mbt.qnt`
- Branch obligations:
  - `step:doDiscoverSlotSpell`
  - `step:doDiscoverWeaponAttack`
  - `step:doEndTurnToTarget`
  - `step:doResolveSlotSpellDamage`
  - `step:doResolveSlotSpellTargets`
  - `step:doResolveWeaponAttackHit`
  - `step:doResolveWeaponDamage`
  - `step:doResolveWeaponTarget`
  - `step:doStartBattle`
- Allowed inputs used:
  - `AGENTS.md`
  - `tasks/WORK_LOOP.md`
  - `tasks/IMPLEMENTER_TASK.md`
  - `tasks/VALIDATION_REPORT.md`
  - `tasks/TARGET_REPLAY_EVIDENCE.example.json`
  - `cleanroom-input/MANIFEST.md`
  - `cleanroom-input/branch-coverage/source-branch-inventory.json`
  - `cleanroom-input/branch-coverage/reducer-route-inventory.json`
  - `cleanroom-input/qnt/battle-runtime/battle-runtime-reducer-spine-contract.mbt.qnt`
  - `cleanroom-input/guidance/README.md`
  - `cleanroom-input/guidance/reducer-spine.md`
  - `cleanroom-input/domain/UBIQUITOUS_LANGUAGE.md`
  - `cleanroom-input/domain/CLEANROOM_ASSUMPTIONS.md`
  - `src/rules/battle_reducer_spine.rs`
  - `src/qnt_adapters/battle_runtime_reducer_spine_contract.rs`
  - `src/qnt_adapters/battle_runtime_weapon_attack_skeleton.rs`
  - `src/qnt_adapters/battle_runtime_magic_missile.rs`
  - `src/qnt_adapters/battle_runtime_save_gated_spell_ordering.rs`
  - `src/qnt_adapters/battle_runtime_hit_point_restoration_ordering.rs`
  - `src/qnt_adapters/battle_runtime_death_saving_throw.rs`
  - `src/qnt_adapters/battle_runtime_concentration_break_teardown.rs`
  - `src/qnt_adapters/battle_runtime_weapon_attack_ordering.rs`

Behavior implemented:

- Added typed `BattleResolutionResult` accessors/conversions for outcome, state, resolved state, requested holes, continuing subject, and invalid details.
- Updated focused reducer-route adapters to consume reducer-result facts through the shared production surface instead of reconstructing enum variants locally.
- No durable `BattleState` fields were added or changed; existing reducer result payloads remain the single source for these facts.

Generated branch coverage:

| Obligation | Target replay evidence | Diagnostic tests | Status |
| --- | --- | --- | --- |
| `cleanroom-input/qnt/battle-runtime/battle-runtime-reducer-spine-contract.mbt.qnt#step:doDiscoverSlotSpell` | `tasks/target-replay-evidence/RRCONV-19E-end-turn-subject-resolution.json#RRCONV-19E-RUST-END-TURN-SUBJECT-RESOLUTION fresh replay action=doDiscoverSlotSpell#step:doDiscoverSlotSpell` | `src/tests/mod.rs` | `covered` |
| `cleanroom-input/qnt/battle-runtime/battle-runtime-reducer-spine-contract.mbt.qnt#step:doDiscoverWeaponAttack` | `tasks/target-replay-evidence/RRCONV-19E-end-turn-subject-resolution.json#RRCONV-19E-RUST-END-TURN-SUBJECT-RESOLUTION fresh replay action=doDiscoverWeaponAttack#step:doDiscoverWeaponAttack` | `src/tests/mod.rs` | `covered` |
| `cleanroom-input/qnt/battle-runtime/battle-runtime-reducer-spine-contract.mbt.qnt#step:doEndTurnToTarget` | `tasks/target-replay-evidence/RRCONV-19E-end-turn-subject-resolution.json#RRCONV-19E-RUST-END-TURN-SUBJECT-RESOLUTION fresh replay action=doEndTurnToTarget#step:doEndTurnToTarget` | `src/tests/mod.rs` | `covered` |
| `cleanroom-input/qnt/battle-runtime/battle-runtime-reducer-spine-contract.mbt.qnt#step:doResolveSlotSpellDamage` | `tasks/target-replay-evidence/RRCONV-19E-end-turn-subject-resolution.json#RRCONV-19E-RUST-END-TURN-SUBJECT-RESOLUTION fresh replay action=doResolveSlotSpellDamage#step:doResolveSlotSpellDamage` | `src/tests/mod.rs` | `covered` |
| `cleanroom-input/qnt/battle-runtime/battle-runtime-reducer-spine-contract.mbt.qnt#step:doResolveSlotSpellTargets` | `tasks/target-replay-evidence/RRCONV-19E-end-turn-subject-resolution.json#RRCONV-19E-RUST-END-TURN-SUBJECT-RESOLUTION fresh replay action=doResolveSlotSpellTargets#step:doResolveSlotSpellTargets` | `src/tests/mod.rs` | `covered` |
| `cleanroom-input/qnt/battle-runtime/battle-runtime-reducer-spine-contract.mbt.qnt#step:doResolveWeaponAttackHit` | `tasks/target-replay-evidence/RRCONV-19E-end-turn-subject-resolution.json#RRCONV-19E-RUST-END-TURN-SUBJECT-RESOLUTION fresh replay action=doResolveWeaponAttackHit#step:doResolveWeaponAttackHit` | `src/tests/mod.rs` | `covered` |
| `cleanroom-input/qnt/battle-runtime/battle-runtime-reducer-spine-contract.mbt.qnt#step:doResolveWeaponDamage` | `tasks/target-replay-evidence/RRCONV-19E-end-turn-subject-resolution.json#RRCONV-19E-RUST-END-TURN-SUBJECT-RESOLUTION fresh replay action=doResolveWeaponDamage#step:doResolveWeaponDamage` | `src/tests/mod.rs` | `covered` |
| `cleanroom-input/qnt/battle-runtime/battle-runtime-reducer-spine-contract.mbt.qnt#step:doResolveWeaponTarget` | `tasks/target-replay-evidence/RRCONV-19E-end-turn-subject-resolution.json#RRCONV-19E-RUST-END-TURN-SUBJECT-RESOLUTION fresh replay action=doResolveWeaponTarget#step:doResolveWeaponTarget` | `src/tests/mod.rs` | `covered` |
| `cleanroom-input/qnt/battle-runtime/battle-runtime-reducer-spine-contract.mbt.qnt#step:doStartBattle` | `tasks/target-replay-evidence/RRCONV-19E-end-turn-subject-resolution.json#RRCONV-19E-RUST-END-TURN-SUBJECT-RESOLUTION fresh replay action=doStartBattle#step:doStartBattle` | `src/tests/mod.rs` | `covered` |

Target replay evidence:

- Evidence file: `tasks/target-replay-evidence/RRCONV-19E-end-turn-subject-resolution.json`
- Target profile: `rust`
- Target profile SHA-256: `6d4cc6c6a4769962798133d57aff01438fb2b661941f71d1aa8a3333f4b7ecc1`
- Quint binding: Rust quint-connect harness
- Reproduction seed or trace id: `RRCONV-19E-RUST-END-TURN-SUBJECT-RESOLUTION fresh replay`
- Evidence refs:
  - `tasks/target-replay-evidence/RRCONV-19E-end-turn-subject-resolution.json#RRCONV-19E-RUST-END-TURN-SUBJECT-RESOLUTION fresh replay action=doDiscoverSlotSpell#step:doDiscoverSlotSpell`
  - `tasks/target-replay-evidence/RRCONV-19E-end-turn-subject-resolution.json#RRCONV-19E-RUST-END-TURN-SUBJECT-RESOLUTION fresh replay action=doDiscoverWeaponAttack#step:doDiscoverWeaponAttack`
  - `tasks/target-replay-evidence/RRCONV-19E-end-turn-subject-resolution.json#RRCONV-19E-RUST-END-TURN-SUBJECT-RESOLUTION fresh replay action=doEndTurnToTarget#step:doEndTurnToTarget`
  - `tasks/target-replay-evidence/RRCONV-19E-end-turn-subject-resolution.json#RRCONV-19E-RUST-END-TURN-SUBJECT-RESOLUTION fresh replay action=doResolveSlotSpellDamage#step:doResolveSlotSpellDamage`
  - `tasks/target-replay-evidence/RRCONV-19E-end-turn-subject-resolution.json#RRCONV-19E-RUST-END-TURN-SUBJECT-RESOLUTION fresh replay action=doResolveSlotSpellTargets#step:doResolveSlotSpellTargets`
  - `tasks/target-replay-evidence/RRCONV-19E-end-turn-subject-resolution.json#RRCONV-19E-RUST-END-TURN-SUBJECT-RESOLUTION fresh replay action=doResolveWeaponAttackHit#step:doResolveWeaponAttackHit`
  - `tasks/target-replay-evidence/RRCONV-19E-end-turn-subject-resolution.json#RRCONV-19E-RUST-END-TURN-SUBJECT-RESOLUTION fresh replay action=doResolveWeaponDamage#step:doResolveWeaponDamage`
  - `tasks/target-replay-evidence/RRCONV-19E-end-turn-subject-resolution.json#RRCONV-19E-RUST-END-TURN-SUBJECT-RESOLUTION fresh replay action=doResolveWeaponTarget#step:doResolveWeaponTarget`
  - `tasks/target-replay-evidence/RRCONV-19E-end-turn-subject-resolution.json#RRCONV-19E-RUST-END-TURN-SUBJECT-RESOLUTION fresh replay action=doStartBattle#step:doStartBattle`

Harness artifacts:

- Start gate: `tasks/history/RRCONV-19C-RUST-RESOLUTION-RESULT-CONTRACT/START_GATE.json`
- Engine depth: `tasks/history/RRCONV-19C-RUST-RESOLUTION-RESULT-CONTRACT/ENGINE_DEPTH_MANIFEST.json`
- State ownership: `tasks/history/RRCONV-19C-RUST-RESOLUTION-RESULT-CONTRACT/STATE_OWNER_MANIFEST.json`
- Reviewer loop: `tasks/history/RRCONV-19C-RUST-RESOLUTION-RESULT-CONTRACT/REVIEW_LOOP.json`
- Decider decision: `tasks/history/RRCONV-19C-RUST-RESOLUTION-RESULT-CONTRACT/DECIDER_DECISION.json`
- Run ledger: `tasks/RUN_LEDGER.json`

Remaining gaps:

- `_none_`

Verification results:

- `cargo fmt --check` passed.
- `cargo test reducer_entrypoint_contract` passed.
- `cargo test reducer_spine_contract_adapter_replays_all_branches` passed.
- `cargo test magic_missile_adapter_replays_all_branches` passed.
- `cargo test save_gated_spell_ordering_adapter_replays_all_branches` passed.
- `cargo test hit_point_restoration_ordering_adapter_replays_all_branches` passed.
- `cargo test death_saving_throw_adapter_replays_all_branches` passed.
- `cargo test concentration_break_teardown_adapter_replays_all_branches` passed.
- `cargo test weapon_attack_skeleton_adapter_replays_all_branches` passed.
- `cargo test` passed.
- `cargo clippy --all-targets -- -D warnings` passed.
- `node scripts/check-cleanroom-harness.cjs` passed.
- `git diff --check 6dc146b9c4a5bf7894b9ad4ae206bc4ef04e222d...HEAD` passed.
## RRCONV-19E-RUST-END-TURN-SUBJECT-RESOLUTION: End Turn Subject Resolution

- Manifest source commit SHA: `564376fd95218a209bb9eae5c9ccb54ca3e04a52`
- Source branch inventory SHA: `4bb2b20a85d94e3b90b7c59cbfe6e1edd5ab3ef40410641e999527861f3d3a32`
- Driver: `cleanroom-input/qnt/battle-runtime/battle-runtime-reducer-spine-contract.mbt.qnt`
- Machine-readable run ledger: `tasks/RUN_LEDGER.json`
- Task artifacts: `tasks/history/RRCONV-19E-RUST-END-TURN-SUBJECT-RESOLUTION/`

Allowed inputs used:

- `cleanroom-input/MANIFEST.md`
- `cleanroom-input/branch-coverage/source-branch-inventory.json`
- `cleanroom-input/branch-coverage/reducer-route-inventory.json`
- `cleanroom-input/qnt/battle-runtime/battle-runtime-reducer-spine-contract.mbt.qnt`
- `cleanroom-input/guidance/README.md`
- `cleanroom-input/guidance/reducer-spine.md`
- Repo-local `src/**`, `tasks/**`, and Rust/Cargo tooling

Behavior implemented:

- Added `BattleSubjectKind::EndTurn` and `BattleResolutionRequest::end_turn` as a no-fill command subject resolved by `resolve_battle_subject`.
- Updated `doEndTurnToTarget` replay to call subject resolution rather than adapter-local `advance_turn`; the observed projection remains `ResolveBattleSubjectEntrypoint` and `EndTurnSubject`.
- Preserved `BattleTurnAdvanceResult` as the source of previous actor, next actor, round, and advanced-state metadata via `BattleResolutionResult::TurnAdvanced`.
- No durable `BattleState` fields were added or changed.

Generated branch coverage:

| Obligation | Target replay evidence | Diagnostic tests | Status |
| --- | --- | --- | --- |
| `cleanroom-input/qnt/battle-runtime/battle-runtime-reducer-spine-contract.mbt.qnt#step:doDiscoverSlotSpell` | `tasks/target-replay-evidence/RRCONV-19E-end-turn-subject-resolution.json#RRCONV-19E-RUST-END-TURN-SUBJECT-RESOLUTION fresh replay action=doDiscoverSlotSpell#step:doDiscoverSlotSpell` | `src/tests/mod.rs::reducer_spine_contract_adapter_replays_all_branches` | `covered` |
| `cleanroom-input/qnt/battle-runtime/battle-runtime-reducer-spine-contract.mbt.qnt#step:doDiscoverWeaponAttack` | `tasks/target-replay-evidence/RRCONV-19E-end-turn-subject-resolution.json#RRCONV-19E-RUST-END-TURN-SUBJECT-RESOLUTION fresh replay action=doDiscoverWeaponAttack#step:doDiscoverWeaponAttack` | `src/tests/mod.rs::reducer_spine_contract_adapter_replays_all_branches` | `covered` |
| `cleanroom-input/qnt/battle-runtime/battle-runtime-reducer-spine-contract.mbt.qnt#step:doEndTurnToTarget` | `tasks/target-replay-evidence/RRCONV-19E-end-turn-subject-resolution.json#RRCONV-19E-RUST-END-TURN-SUBJECT-RESOLUTION fresh replay action=doEndTurnToTarget#step:doEndTurnToTarget` | `src/tests/mod.rs::reducer_spine_contract_adapter_replays_all_branches` | `covered` |
| `cleanroom-input/qnt/battle-runtime/battle-runtime-reducer-spine-contract.mbt.qnt#step:doResolveSlotSpellDamage` | `tasks/target-replay-evidence/RRCONV-19E-end-turn-subject-resolution.json#RRCONV-19E-RUST-END-TURN-SUBJECT-RESOLUTION fresh replay action=doResolveSlotSpellDamage#step:doResolveSlotSpellDamage` | `src/tests/mod.rs::reducer_spine_contract_adapter_replays_all_branches` | `covered` |
| `cleanroom-input/qnt/battle-runtime/battle-runtime-reducer-spine-contract.mbt.qnt#step:doResolveSlotSpellTargets` | `tasks/target-replay-evidence/RRCONV-19E-end-turn-subject-resolution.json#RRCONV-19E-RUST-END-TURN-SUBJECT-RESOLUTION fresh replay action=doResolveSlotSpellTargets#step:doResolveSlotSpellTargets` | `src/tests/mod.rs::reducer_spine_contract_adapter_replays_all_branches` | `covered` |
| `cleanroom-input/qnt/battle-runtime/battle-runtime-reducer-spine-contract.mbt.qnt#step:doResolveWeaponAttackHit` | `tasks/target-replay-evidence/RRCONV-19E-end-turn-subject-resolution.json#RRCONV-19E-RUST-END-TURN-SUBJECT-RESOLUTION fresh replay action=doResolveWeaponAttackHit#step:doResolveWeaponAttackHit` | `src/tests/mod.rs::reducer_spine_contract_adapter_replays_all_branches` | `covered` |
| `cleanroom-input/qnt/battle-runtime/battle-runtime-reducer-spine-contract.mbt.qnt#step:doResolveWeaponDamage` | `tasks/target-replay-evidence/RRCONV-19E-end-turn-subject-resolution.json#RRCONV-19E-RUST-END-TURN-SUBJECT-RESOLUTION fresh replay action=doResolveWeaponDamage#step:doResolveWeaponDamage` | `src/tests/mod.rs::reducer_spine_contract_adapter_replays_all_branches` | `covered` |
| `cleanroom-input/qnt/battle-runtime/battle-runtime-reducer-spine-contract.mbt.qnt#step:doResolveWeaponTarget` | `tasks/target-replay-evidence/RRCONV-19E-end-turn-subject-resolution.json#RRCONV-19E-RUST-END-TURN-SUBJECT-RESOLUTION fresh replay action=doResolveWeaponTarget#step:doResolveWeaponTarget` | `src/tests/mod.rs::reducer_spine_contract_adapter_replays_all_branches` | `covered` |
| `cleanroom-input/qnt/battle-runtime/battle-runtime-reducer-spine-contract.mbt.qnt#step:doStartBattle` | `tasks/target-replay-evidence/RRCONV-19E-end-turn-subject-resolution.json#RRCONV-19E-RUST-END-TURN-SUBJECT-RESOLUTION fresh replay action=doStartBattle#step:doStartBattle` | `src/tests/mod.rs::reducer_spine_contract_adapter_replays_all_branches` | `covered` |

Target replay evidence:

- Evidence file: `tasks/target-replay-evidence/RRCONV-19E-end-turn-subject-resolution.json`
- Target profile: `rust`
- Target profile SHA-256: `6d4cc6c6a4769962798133d57aff01438fb2b661941f71d1aa8a3333f4b7ecc1`
- Quint binding: Rust quint-connect harness
- Reproduction seed or trace id: `RRCONV-19E-RUST-END-TURN-SUBJECT-RESOLUTION fresh replay action=<branchAction>`

Harness artifacts:

- Start gate: `tasks/history/RRCONV-19E-RUST-END-TURN-SUBJECT-RESOLUTION/START_GATE.json`
- Engine depth: `tasks/history/RRCONV-19E-RUST-END-TURN-SUBJECT-RESOLUTION/ENGINE_DEPTH_MANIFEST.json`
- State ownership: `tasks/history/RRCONV-19E-RUST-END-TURN-SUBJECT-RESOLUTION/STATE_OWNER_MANIFEST.json`
- Reviewer loop: `tasks/history/RRCONV-19E-RUST-END-TURN-SUBJECT-RESOLUTION/REVIEW_LOOP.json`
- Decider decision: `tasks/history/RRCONV-19E-RUST-END-TURN-SUBJECT-RESOLUTION/DECIDER_DECISION.json`
- Run ledger: `tasks/RUN_LEDGER.json`

Remaining gaps:

- `_none_`

Verification results:

- `cargo fmt --check` passed.
- `cargo test reducer_entrypoint_contract` passed.
- `cargo test reducer_spine_contract_adapter_replays_all_branches` passed.
- `cargo test experimental_qnt_spine_advances_turn_boundary_lifecycle` passed.
- `cargo test` passed.
- `cargo clippy --all-targets -- -D warnings` passed.
- `node scripts/check-cleanroom-harness.cjs` passed.
- `git diff --check 277f0cf58516d3463267546f578b648f45331a06...HEAD` passed.

## RRCONV-19F-RUST-ROUTE-EVENT-FROM-REDUCER-RESULT: Route Event From Reducer Result

- Manifest source commit SHA: `564376fd95218a209bb9eae5c9ccb54ca3e04a52`
- Source branch inventory SHA: `4bb2b20a85d94e3b90b7c59cbfe6e1edd5ab3ef40410641e999527861f3d3a32`
- Drivers:
  - `cleanroom-input/qnt/battle-runtime/battle-runtime-magic-missile.mbt.qnt`
  - `cleanroom-input/qnt/battle-runtime/battle-runtime-save-gated-spell-ordering.mbt.qnt`
- Machine-readable run ledger: `tasks/RUN_LEDGER.json`

Allowed inputs used:

- `cleanroom-input/branch-coverage/source-branch-inventory.json`
- `cleanroom-input/branch-coverage/reducer-route-inventory.json`
- `cleanroom-input/qnt/battle-runtime/battle-runtime-magic-missile.mbt.qnt`
- `cleanroom-input/qnt/battle-runtime/battle-runtime-magic-missile.route.mbt.qnt`
- `cleanroom-input/qnt/battle-runtime/battle-runtime-save-gated-spell-ordering.mbt.qnt`
- `cleanroom-input/qnt/battle-runtime/battle-runtime-save-gated-spell-ordering.route.mbt.qnt`
- Repo-local `src/**`, `tasks/**`, `scripts/**`, and Rust/Cargo tooling in this cleanroom worktree

Behavior implemented:

- Added adapter-side `ReducerRouteResolveConnector`, `ReducerRouteResolveFill`, and `route_resolve_battle_subject_from_result` so resolve route events consume `BattleResolutionResult` holes and outcome metadata.
- Updated Magic Missile and Save-Gated Spell Ordering observed route adapters to use the shared result-based helper. Literal expected-route witnesses remain independent route expectations.
- Made the cleanroom harness accept multi-driver selected scopes and repeated-driver evidence refs so this architecture lane can honestly declare both changed and replayed drivers in one latest ledger entry.

Generated branch coverage:

| Obligation | Target replay evidence | Diagnostic tests | Status |
| --- | --- | --- | --- |
| `cleanroom-input/qnt/battle-runtime/battle-runtime-magic-missile.mbt.qnt#step:doFillMagicMissileAllocation` | `tasks/target-replay-evidence/RRCONV-19F-route-event-from-reducer-result.json#RRCONV-19F-RUST-ROUTE-EVENT-FROM-REDUCER-RESULT replay action=doFillMagicMissileAllocation#step:doFillMagicMissileAllocation` | `src/tests/mod.rs` | `covered` |
| `cleanroom-input/qnt/battle-runtime/battle-runtime-magic-missile.mbt.qnt#step:doFillMagicMissileDamage` | `tasks/target-replay-evidence/RRCONV-19F-route-event-from-reducer-result.json#RRCONV-19F-RUST-ROUTE-EVENT-FROM-REDUCER-RESULT replay action=doFillMagicMissileDamage#step:doFillMagicMissileDamage` | `src/tests/mod.rs` | `covered` |
| `cleanroom-input/qnt/battle-runtime/battle-runtime-save-gated-spell-ordering.mbt.qnt#step:doDiscoverAreaSaveDamage` | `tasks/target-replay-evidence/RRCONV-19F-route-event-from-reducer-result.json#RRCONV-19F-RUST-ROUTE-EVENT-FROM-REDUCER-RESULT replay action=doDiscoverAreaSaveDamage#step:doDiscoverAreaSaveDamage` | `src/tests/mod.rs` | `covered` |
| `cleanroom-input/qnt/battle-runtime/battle-runtime-save-gated-spell-ordering.mbt.qnt#step:doDiscoverTargetListConditionChoice` | `tasks/target-replay-evidence/RRCONV-19F-route-event-from-reducer-result.json#RRCONV-19F-RUST-ROUTE-EVENT-FROM-REDUCER-RESULT replay action=doDiscoverTargetListConditionChoice#step:doDiscoverTargetListConditionChoice` | `src/tests/mod.rs` | `covered` |
| `cleanroom-input/qnt/battle-runtime/battle-runtime-save-gated-spell-ordering.mbt.qnt#step:doFillAreaDamageDice` | `tasks/target-replay-evidence/RRCONV-19F-route-event-from-reducer-result.json#RRCONV-19F-RUST-ROUTE-EVENT-FROM-REDUCER-RESULT replay action=doFillAreaDamageDice#step:doFillAreaDamageDice` | `src/tests/mod.rs` | `covered` |
| `cleanroom-input/qnt/battle-runtime/battle-runtime-save-gated-spell-ordering.mbt.qnt#step:doFillAreaSaveFailed` | `tasks/target-replay-evidence/RRCONV-19F-route-event-from-reducer-result.json#RRCONV-19F-RUST-ROUTE-EVENT-FROM-REDUCER-RESULT replay action=doFillAreaSaveFailed#step:doFillAreaSaveFailed` | `src/tests/mod.rs` | `covered` |
| `cleanroom-input/qnt/battle-runtime/battle-runtime-save-gated-spell-ordering.mbt.qnt#step:doFillConditionChoiceAfterTargetList` | `tasks/target-replay-evidence/RRCONV-19F-route-event-from-reducer-result.json#RRCONV-19F-RUST-ROUTE-EVENT-FROM-REDUCER-RESULT replay action=doFillConditionChoiceAfterTargetList#step:doFillConditionChoiceAfterTargetList` | `src/tests/mod.rs` | `covered` |
| `cleanroom-input/qnt/battle-runtime/battle-runtime-save-gated-spell-ordering.mbt.qnt#step:doFillConditionChoiceBeforeTargetList` | `tasks/target-replay-evidence/RRCONV-19F-route-event-from-reducer-result.json#RRCONV-19F-RUST-ROUTE-EVENT-FROM-REDUCER-RESULT replay action=doFillConditionChoiceBeforeTargetList#step:doFillConditionChoiceBeforeTargetList` | `src/tests/mod.rs` | `covered` |
| `cleanroom-input/qnt/battle-runtime/battle-runtime-save-gated-spell-ordering.mbt.qnt#step:doFillConditionSavingThrow` | `tasks/target-replay-evidence/RRCONV-19F-route-event-from-reducer-result.json#RRCONV-19F-RUST-ROUTE-EVENT-FROM-REDUCER-RESULT replay action=doFillConditionSavingThrow#step:doFillConditionSavingThrow` | `src/tests/mod.rs` | `covered` |
| `cleanroom-input/qnt/battle-runtime/battle-runtime-save-gated-spell-ordering.mbt.qnt#step:doFillTargetListAfterConditionChoice` | `tasks/target-replay-evidence/RRCONV-19F-route-event-from-reducer-result.json#RRCONV-19F-RUST-ROUTE-EVENT-FROM-REDUCER-RESULT replay action=doFillTargetListAfterConditionChoice#step:doFillTargetListAfterConditionChoice` | `src/tests/mod.rs` | `covered` |
| `cleanroom-input/qnt/battle-runtime/battle-runtime-save-gated-spell-ordering.mbt.qnt#step:doFillTargetListBeforeConditionChoice` | `tasks/target-replay-evidence/RRCONV-19F-route-event-from-reducer-result.json#RRCONV-19F-RUST-ROUTE-EVENT-FROM-REDUCER-RESULT replay action=doFillTargetListBeforeConditionChoice#step:doFillTargetListBeforeConditionChoice` | `src/tests/mod.rs` | `covered` |
| `cleanroom-input/qnt/battle-runtime/battle-runtime-save-gated-spell-ordering.mbt.qnt#step:doSubmitDamageBeforeSavingThrow` | `tasks/target-replay-evidence/RRCONV-19F-route-event-from-reducer-result.json#RRCONV-19F-RUST-ROUTE-EVENT-FROM-REDUCER-RESULT replay action=doSubmitDamageBeforeSavingThrow#step:doSubmitDamageBeforeSavingThrow` | `src/tests/mod.rs` | `covered` |

Target replay evidence:

- Evidence file: `tasks/target-replay-evidence/RRCONV-19F-route-event-from-reducer-result.json`
- Target profile: `rust`
- Target profile SHA-256: `6d4cc6c6a4769962798133d57aff01438fb2b661941f71d1aa8a3333f4b7ecc1`
- Reproduction seed or trace id: `RRCONV-19F-RUST-ROUTE-EVENT-FROM-REDUCER-RESULT replay action=<branchAction>`

Harness artifacts:

- Start gate: `tasks/history/RRCONV-19F-RUST-ROUTE-EVENT-FROM-REDUCER-RESULT/START_GATE.json`
- Engine depth: `tasks/history/RRCONV-19F-RUST-ROUTE-EVENT-FROM-REDUCER-RESULT/ENGINE_DEPTH_MANIFEST.json`
- State ownership: `tasks/history/RRCONV-19F-RUST-ROUTE-EVENT-FROM-REDUCER-RESULT/STATE_OWNER_MANIFEST.json`
- Reviewer loop: `tasks/history/RRCONV-19F-RUST-ROUTE-EVENT-FROM-REDUCER-RESULT/REVIEW_LOOP.json`
- Decider decision: `tasks/history/RRCONV-19F-RUST-ROUTE-EVENT-FROM-REDUCER-RESULT/DECIDER_DECISION.json`
- Run ledger: `tasks/RUN_LEDGER.json`

Remaining gaps:

- This is a focused architecture hardening lane; it does not claim every reducer-routed adapter has been converted to result-based route-event construction.

Verification results:

- `cargo fmt --check` passed.
- `cargo test route_event` passed.
- `cargo test magic_missile_adapter_replays_all_branches` passed.
- `cargo test save_gated_spell_ordering_adapter_replays_all_branches` passed.
- `cargo test reducer_spine_contract_adapter_replays_all_branches` passed.
- `cargo test` passed.
- `cargo clippy --all-targets -- -D warnings` passed.
- `node scripts/check-cleanroom-harness.cjs` passed.
- `git diff --check aa3996e4b32e2502b3c3ee5cca051db66157269b...HEAD` passed.

## RRCONV-19G-RUST-SUBJECT-CONTINUATION-LIFECYCLE: Subject Continuation Lifecycle

- Manifest source commit SHA: `564376fd95218a209bb9eae5c9ccb54ca3e04a52`
- Source branch inventory SHA: `4bb2b20a85d94e3b90b7c59cbfe6e1edd5ab3ef40410641e999527861f3d3a32`
- Drivers:
  - `cleanroom-input/qnt/battle-runtime/battle-runtime-save-gated-spell-ordering.mbt.qnt`
  - `cleanroom-input/qnt/battle-runtime/battle-runtime-hit-point-restoration-ordering.mbt.qnt`
- Branch obligations:
  - `step:doDiscoverAreaSaveDamage`
  - `step:doDiscoverTargetListConditionChoice`
  - `step:doFillAreaDamageDice`
  - `step:doFillAreaSaveFailed`
  - `step:doFillConditionChoiceAfterTargetList`
  - `step:doFillConditionChoiceBeforeTargetList`
  - `step:doFillConditionSavingThrow`
  - `step:doFillTargetListAfterConditionChoice`
  - `step:doFillTargetListBeforeConditionChoice`
  - `step:doSubmitDamageBeforeSavingThrow`
  - `step:doDiscoverFeatureHealingPool`
  - `step:doDiscoverSingleTargetSpellHealing`
  - `step:doDiscoverTargetListSpellHealing`
  - `step:doFillFeatureHealingDistribution`
  - `step:doFillSpellHealingRoll`
  - `step:doFillSpellHealingTargetChoice`
  - `step:doFillSpellHealingTargetList`
  - `step:doSubmitHealingRollBeforeTargetChoice`
  - `step:doSubmitHealingRollBeforeTargetList`
- Allowed inputs used:
  - `cleanroom-input/MANIFEST.md`
  - `cleanroom-input/branch-coverage/source-branch-inventory.json`
  - `cleanroom-input/branch-coverage/reducer-route-inventory.json`
  - `cleanroom-input/qnt/battle-runtime/battle-runtime-save-gated-spell-ordering.mbt.qnt`
  - `cleanroom-input/qnt/battle-runtime/battle-runtime-save-gated-spell-ordering.route.mbt.qnt`
  - `cleanroom-input/qnt/battle-runtime/battle-runtime-hit-point-restoration-ordering.mbt.qnt`
  - `cleanroom-input/qnt/battle-runtime/battle-runtime-hit-point-restoration-ordering.route.mbt.qnt`
  - `cleanroom-input/guidance/README.md`
  - `cleanroom-input/guidance/reducer-spine.md`
  - `cleanroom-input/domain/UBIQUITOUS_LANGUAGE.md`
  - `cleanroom-input/domain/CLEANROOM_ASSUMPTIONS.md`
  - `cleanroom-input/raw/srd-5.2.1/Playing-the-Game.md`
  - `cleanroom-input/raw/srd-5.2.1/Rules-Glossary.md`
  - `cleanroom-input/raw/srd-5.2.1/Spells/Gaining-and-Casting.md`

Behavior implemented:

- Added a shared adapter-side continuation helper that consumes `BattleResolutionResult` through `continuing_subject()` and `into_needs_holes()` and returns the next `BattleState` plus `BattleSubject`.
- Reused the helper in Save-Gated Spell Ordering and Hit Point Restoration Ordering continuation paths; no durable `BattleState` fields were introduced.
- Kept QNT branch names quarantined in adapters and evidence; this does not claim all route drivers are converted.

Generated branch coverage:

| Obligation | Target replay evidence | Diagnostic tests | Status |
| --- | --- | --- | --- |
| `cleanroom-input/qnt/battle-runtime/battle-runtime-save-gated-spell-ordering.mbt.qnt#step:doDiscoverAreaSaveDamage` | `tasks/target-replay-evidence/RRCONV-19G-subject-continuation-lifecycle.json#RRCONV-19G-RUST-SUBJECT-CONTINUATION-LIFECYCLE replay action=doDiscoverAreaSaveDamage#step:doDiscoverAreaSaveDamage` | `src/tests/mod.rs` | `covered` |
| `cleanroom-input/qnt/battle-runtime/battle-runtime-save-gated-spell-ordering.mbt.qnt#step:doDiscoverTargetListConditionChoice` | `tasks/target-replay-evidence/RRCONV-19G-subject-continuation-lifecycle.json#RRCONV-19G-RUST-SUBJECT-CONTINUATION-LIFECYCLE replay action=doDiscoverTargetListConditionChoice#step:doDiscoverTargetListConditionChoice` | `src/tests/mod.rs` | `covered` |
| `cleanroom-input/qnt/battle-runtime/battle-runtime-save-gated-spell-ordering.mbt.qnt#step:doFillAreaDamageDice` | `tasks/target-replay-evidence/RRCONV-19G-subject-continuation-lifecycle.json#RRCONV-19G-RUST-SUBJECT-CONTINUATION-LIFECYCLE replay action=doFillAreaDamageDice#step:doFillAreaDamageDice` | `src/tests/mod.rs` | `covered` |
| `cleanroom-input/qnt/battle-runtime/battle-runtime-save-gated-spell-ordering.mbt.qnt#step:doFillAreaSaveFailed` | `tasks/target-replay-evidence/RRCONV-19G-subject-continuation-lifecycle.json#RRCONV-19G-RUST-SUBJECT-CONTINUATION-LIFECYCLE replay action=doFillAreaSaveFailed#step:doFillAreaSaveFailed` | `src/tests/mod.rs` | `covered` |
| `cleanroom-input/qnt/battle-runtime/battle-runtime-save-gated-spell-ordering.mbt.qnt#step:doFillConditionChoiceAfterTargetList` | `tasks/target-replay-evidence/RRCONV-19G-subject-continuation-lifecycle.json#RRCONV-19G-RUST-SUBJECT-CONTINUATION-LIFECYCLE replay action=doFillConditionChoiceAfterTargetList#step:doFillConditionChoiceAfterTargetList` | `src/tests/mod.rs` | `covered` |
| `cleanroom-input/qnt/battle-runtime/battle-runtime-save-gated-spell-ordering.mbt.qnt#step:doFillConditionChoiceBeforeTargetList` | `tasks/target-replay-evidence/RRCONV-19G-subject-continuation-lifecycle.json#RRCONV-19G-RUST-SUBJECT-CONTINUATION-LIFECYCLE replay action=doFillConditionChoiceBeforeTargetList#step:doFillConditionChoiceBeforeTargetList` | `src/tests/mod.rs` | `covered` |
| `cleanroom-input/qnt/battle-runtime/battle-runtime-save-gated-spell-ordering.mbt.qnt#step:doFillConditionSavingThrow` | `tasks/target-replay-evidence/RRCONV-19G-subject-continuation-lifecycle.json#RRCONV-19G-RUST-SUBJECT-CONTINUATION-LIFECYCLE replay action=doFillConditionSavingThrow#step:doFillConditionSavingThrow` | `src/tests/mod.rs` | `covered` |
| `cleanroom-input/qnt/battle-runtime/battle-runtime-save-gated-spell-ordering.mbt.qnt#step:doFillTargetListAfterConditionChoice` | `tasks/target-replay-evidence/RRCONV-19G-subject-continuation-lifecycle.json#RRCONV-19G-RUST-SUBJECT-CONTINUATION-LIFECYCLE replay action=doFillTargetListAfterConditionChoice#step:doFillTargetListAfterConditionChoice` | `src/tests/mod.rs` | `covered` |
| `cleanroom-input/qnt/battle-runtime/battle-runtime-save-gated-spell-ordering.mbt.qnt#step:doFillTargetListBeforeConditionChoice` | `tasks/target-replay-evidence/RRCONV-19G-subject-continuation-lifecycle.json#RRCONV-19G-RUST-SUBJECT-CONTINUATION-LIFECYCLE replay action=doFillTargetListBeforeConditionChoice#step:doFillTargetListBeforeConditionChoice` | `src/tests/mod.rs` | `covered` |
| `cleanroom-input/qnt/battle-runtime/battle-runtime-save-gated-spell-ordering.mbt.qnt#step:doSubmitDamageBeforeSavingThrow` | `tasks/target-replay-evidence/RRCONV-19G-subject-continuation-lifecycle.json#RRCONV-19G-RUST-SUBJECT-CONTINUATION-LIFECYCLE replay action=doSubmitDamageBeforeSavingThrow#step:doSubmitDamageBeforeSavingThrow` | `src/tests/mod.rs` | `covered` |
| `cleanroom-input/qnt/battle-runtime/battle-runtime-hit-point-restoration-ordering.mbt.qnt#step:doDiscoverFeatureHealingPool` | `tasks/target-replay-evidence/RRCONV-19G-subject-continuation-lifecycle.json#RRCONV-19G-RUST-SUBJECT-CONTINUATION-LIFECYCLE replay action=doDiscoverFeatureHealingPool#step:doDiscoverFeatureHealingPool` | `src/tests/mod.rs` | `covered` |
| `cleanroom-input/qnt/battle-runtime/battle-runtime-hit-point-restoration-ordering.mbt.qnt#step:doDiscoverSingleTargetSpellHealing` | `tasks/target-replay-evidence/RRCONV-19G-subject-continuation-lifecycle.json#RRCONV-19G-RUST-SUBJECT-CONTINUATION-LIFECYCLE replay action=doDiscoverSingleTargetSpellHealing#step:doDiscoverSingleTargetSpellHealing` | `src/tests/mod.rs` | `covered` |
| `cleanroom-input/qnt/battle-runtime/battle-runtime-hit-point-restoration-ordering.mbt.qnt#step:doDiscoverTargetListSpellHealing` | `tasks/target-replay-evidence/RRCONV-19G-subject-continuation-lifecycle.json#RRCONV-19G-RUST-SUBJECT-CONTINUATION-LIFECYCLE replay action=doDiscoverTargetListSpellHealing#step:doDiscoverTargetListSpellHealing` | `src/tests/mod.rs` | `covered` |
| `cleanroom-input/qnt/battle-runtime/battle-runtime-hit-point-restoration-ordering.mbt.qnt#step:doFillFeatureHealingDistribution` | `tasks/target-replay-evidence/RRCONV-19G-subject-continuation-lifecycle.json#RRCONV-19G-RUST-SUBJECT-CONTINUATION-LIFECYCLE replay action=doFillFeatureHealingDistribution#step:doFillFeatureHealingDistribution` | `src/tests/mod.rs` | `covered` |
| `cleanroom-input/qnt/battle-runtime/battle-runtime-hit-point-restoration-ordering.mbt.qnt#step:doFillSpellHealingRoll` | `tasks/target-replay-evidence/RRCONV-19G-subject-continuation-lifecycle.json#RRCONV-19G-RUST-SUBJECT-CONTINUATION-LIFECYCLE replay action=doFillSpellHealingRoll#step:doFillSpellHealingRoll` | `src/tests/mod.rs` | `covered` |
| `cleanroom-input/qnt/battle-runtime/battle-runtime-hit-point-restoration-ordering.mbt.qnt#step:doFillSpellHealingTargetChoice` | `tasks/target-replay-evidence/RRCONV-19G-subject-continuation-lifecycle.json#RRCONV-19G-RUST-SUBJECT-CONTINUATION-LIFECYCLE replay action=doFillSpellHealingTargetChoice#step:doFillSpellHealingTargetChoice` | `src/tests/mod.rs` | `covered` |
| `cleanroom-input/qnt/battle-runtime/battle-runtime-hit-point-restoration-ordering.mbt.qnt#step:doFillSpellHealingTargetList` | `tasks/target-replay-evidence/RRCONV-19G-subject-continuation-lifecycle.json#RRCONV-19G-RUST-SUBJECT-CONTINUATION-LIFECYCLE replay action=doFillSpellHealingTargetList#step:doFillSpellHealingTargetList` | `src/tests/mod.rs` | `covered` |
| `cleanroom-input/qnt/battle-runtime/battle-runtime-hit-point-restoration-ordering.mbt.qnt#step:doSubmitHealingRollBeforeTargetChoice` | `tasks/target-replay-evidence/RRCONV-19G-subject-continuation-lifecycle.json#RRCONV-19G-RUST-SUBJECT-CONTINUATION-LIFECYCLE replay action=doSubmitHealingRollBeforeTargetChoice#step:doSubmitHealingRollBeforeTargetChoice` | `src/tests/mod.rs` | `covered` |
| `cleanroom-input/qnt/battle-runtime/battle-runtime-hit-point-restoration-ordering.mbt.qnt#step:doSubmitHealingRollBeforeTargetList` | `tasks/target-replay-evidence/RRCONV-19G-subject-continuation-lifecycle.json#RRCONV-19G-RUST-SUBJECT-CONTINUATION-LIFECYCLE replay action=doSubmitHealingRollBeforeTargetList#step:doSubmitHealingRollBeforeTargetList` | `src/tests/mod.rs` | `covered` |

Target replay evidence:

- Evidence file: `tasks/target-replay-evidence/RRCONV-19G-subject-continuation-lifecycle.json`
- Target profile: `rust`
- Target profile SHA-256: `6d4cc6c6a4769962798133d57aff01438fb2b661941f71d1aa8a3333f4b7ecc1`
- Quint binding: Rust quint-connect harness
- Reproduction seed or trace id: `RRCONV-19G-RUST-SUBJECT-CONTINUATION-LIFECYCLE deterministic replay`

Harness artifacts:

- Start gate: `tasks/history/RRCONV-19G-RUST-SUBJECT-CONTINUATION-LIFECYCLE/START_GATE.json`
- Engine depth: `tasks/history/RRCONV-19G-RUST-SUBJECT-CONTINUATION-LIFECYCLE/ENGINE_DEPTH_MANIFEST.json`
- State ownership: `tasks/history/RRCONV-19G-RUST-SUBJECT-CONTINUATION-LIFECYCLE/STATE_OWNER_MANIFEST.json`
- Reviewer loop: `tasks/history/RRCONV-19G-RUST-SUBJECT-CONTINUATION-LIFECYCLE/REVIEW_LOOP.json`
- Decider decision: `tasks/history/RRCONV-19G-RUST-SUBJECT-CONTINUATION-LIFECYCLE/DECIDER_DECISION.json`
- Run ledger: `tasks/RUN_LEDGER.json`

Remaining gaps:

- `_none_`

Verification results:

- `cargo fmt --check` passed.
- `cargo test continuation` passed.
- `cargo test save_gated_spell_ordering_adapter_replays_all_branches` passed.
- `cargo test hit_point_restoration_ordering_adapter_replays_all_branches` passed.
- `cargo test reducer_spine_contract_adapter_replays_all_branches` passed.
- `cargo test` passed.
- `cargo clippy --all-targets -- -D warnings` passed.
- `node scripts/check-cleanroom-harness.cjs` passed.
- `git diff --check be336582921801cd06995121db38e34ca6f4e275...HEAD` passed.

## L15-RR03-FINISH-CURRENT-DIAGNOSTIC-QUEUE: Integrated Reducer-Route Diagnostic Queue

- Merged from `ralph/rrconv-19-cleanroom` during the RR05 fixer pass.
- Evidence file: `tasks/target-replay-evidence/L15-RR03-reducer-route.json`
- Historical artifacts: `tasks/history/L15-RR03-FINISH-CURRENT-DIAGNOSTIC-QUEUE/`
- Run ledger: `tasks/RUN_LEDGER.json`
- Scope: concentration break teardown and death saving throw reducer-route diagnostics.
- Status: integrated as completed historical coverage; RR05 remains the active latest task entry.


## L15-RR05-BATTLE-ACTION-ATTACK-STATBLOCK-ROUTES: Battle Action Attack / Stat-Block Routes

- Lane drivers: 7
- Branch obligations: 53
- Evidence file: `tasks/target-replay-evidence/L15-RR05-attack-statblock-routes.json`
- Run ledger: `tasks/RUN_LEDGER.json`
- Route assertion hardening: spell attack and creature attack route tests assert exact fill/hole payloads from shared reducer results.

| Obligation | Evidence | Harness test | Status |
|---|---|---|---|
| `cleanroom-input/qnt/battle-runtime/battle-runtime-spell-attack-ordering.mbt.qnt#step:doDiscoverSingleTargetSpellAttack` | `tasks/target-replay-evidence/L15-RR05-attack-statblock-routes.json#L15-RR05-BATTLE-ACTION-ATTACK-STATBLOCK-ROUTES replay action=doDiscoverSingleTargetSpellAttack#step:doDiscoverSingleTargetSpellAttack` | `src/qnt_adapters/battle_runtime_spell_attack_ordering.rs` | `covered` |
| `cleanroom-input/qnt/battle-runtime/battle-runtime-spell-attack-ordering.mbt.qnt#step:doDiscoverTypedSpellAttack` | `tasks/target-replay-evidence/L15-RR05-attack-statblock-routes.json#L15-RR05-BATTLE-ACTION-ATTACK-STATBLOCK-ROUTES replay action=doDiscoverTypedSpellAttack#step:doDiscoverTypedSpellAttack` | `src/qnt_adapters/battle_runtime_spell_attack_ordering.rs` | `covered` |
| `cleanroom-input/qnt/battle-runtime/battle-runtime-spell-attack-ordering.mbt.qnt#step:doFillAttackRollHit` | `tasks/target-replay-evidence/L15-RR05-attack-statblock-routes.json#L15-RR05-BATTLE-ACTION-ATTACK-STATBLOCK-ROUTES replay action=doFillAttackRollHit#step:doFillAttackRollHit` | `src/qnt_adapters/battle_runtime_spell_attack_ordering.rs` | `covered` |
| `cleanroom-input/qnt/battle-runtime/battle-runtime-spell-attack-ordering.mbt.qnt#step:doFillAttackRollMiss` | `tasks/target-replay-evidence/L15-RR05-attack-statblock-routes.json#L15-RR05-BATTLE-ACTION-ATTACK-STATBLOCK-ROUTES replay action=doFillAttackRollMiss#step:doFillAttackRollMiss` | `src/qnt_adapters/battle_runtime_spell_attack_ordering.rs` | `covered` |
| `cleanroom-input/qnt/battle-runtime/battle-runtime-spell-attack-ordering.mbt.qnt#step:doFillDamageDice` | `tasks/target-replay-evidence/L15-RR05-attack-statblock-routes.json#L15-RR05-BATTLE-ACTION-ATTACK-STATBLOCK-ROUTES replay action=doFillDamageDice#step:doFillDamageDice` | `src/qnt_adapters/battle_runtime_spell_attack_ordering.rs` | `covered` |
| `cleanroom-input/qnt/battle-runtime/battle-runtime-spell-attack-ordering.mbt.qnt#step:doFillDamageTypeAfterTargetChoice` | `tasks/target-replay-evidence/L15-RR05-attack-statblock-routes.json#L15-RR05-BATTLE-ACTION-ATTACK-STATBLOCK-ROUTES replay action=doFillDamageTypeAfterTargetChoice#step:doFillDamageTypeAfterTargetChoice` | `src/qnt_adapters/battle_runtime_spell_attack_ordering.rs` | `covered` |
| `cleanroom-input/qnt/battle-runtime/battle-runtime-spell-attack-ordering.mbt.qnt#step:doFillDamageTypeBeforeTargetChoice` | `tasks/target-replay-evidence/L15-RR05-attack-statblock-routes.json#L15-RR05-BATTLE-ACTION-ATTACK-STATBLOCK-ROUTES replay action=doFillDamageTypeBeforeTargetChoice#step:doFillDamageTypeBeforeTargetChoice` | `src/qnt_adapters/battle_runtime_spell_attack_ordering.rs` | `covered` |
| `cleanroom-input/qnt/battle-runtime/battle-runtime-spell-attack-ordering.mbt.qnt#step:doFillTargetChoice` | `tasks/target-replay-evidence/L15-RR05-attack-statblock-routes.json#L15-RR05-BATTLE-ACTION-ATTACK-STATBLOCK-ROUTES replay action=doFillTargetChoice#step:doFillTargetChoice` | `src/qnt_adapters/battle_runtime_spell_attack_ordering.rs` | `covered` |
| `cleanroom-input/qnt/battle-runtime/battle-runtime-spell-attack-ordering.mbt.qnt#step:doFillTargetChoiceAfterDamageType` | `tasks/target-replay-evidence/L15-RR05-attack-statblock-routes.json#L15-RR05-BATTLE-ACTION-ATTACK-STATBLOCK-ROUTES replay action=doFillTargetChoiceAfterDamageType#step:doFillTargetChoiceAfterDamageType` | `src/qnt_adapters/battle_runtime_spell_attack_ordering.rs` | `covered` |
| `cleanroom-input/qnt/battle-runtime/battle-runtime-spell-attack-ordering.mbt.qnt#step:doFillTargetChoiceBeforeDamageType` | `tasks/target-replay-evidence/L15-RR05-attack-statblock-routes.json#L15-RR05-BATTLE-ACTION-ATTACK-STATBLOCK-ROUTES replay action=doFillTargetChoiceBeforeDamageType#step:doFillTargetChoiceBeforeDamageType` | `src/qnt_adapters/battle_runtime_spell_attack_ordering.rs` | `covered` |
| `cleanroom-input/qnt/battle-runtime/battle-runtime-spell-attack-ordering.mbt.qnt#step:doSubmitAttackRollBeforeTargetChoice` | `tasks/target-replay-evidence/L15-RR05-attack-statblock-routes.json#L15-RR05-BATTLE-ACTION-ATTACK-STATBLOCK-ROUTES replay action=doSubmitAttackRollBeforeTargetChoice#step:doSubmitAttackRollBeforeTargetChoice` | `src/qnt_adapters/battle_runtime_spell_attack_ordering.rs` | `covered` |
| `cleanroom-input/qnt/battle-runtime/battle-runtime-spell-attack-ordering.mbt.qnt#step:doSubmitDamageBeforeAttackRoll` | `tasks/target-replay-evidence/L15-RR05-attack-statblock-routes.json#L15-RR05-BATTLE-ACTION-ATTACK-STATBLOCK-ROUTES replay action=doSubmitDamageBeforeAttackRoll#step:doSubmitDamageBeforeAttackRoll` | `src/qnt_adapters/battle_runtime_spell_attack_ordering.rs` | `covered` |
| `cleanroom-input/qnt/battle-runtime/battle-runtime-stat-block-action-ordering.mbt.qnt#step:doDiscoverRolledActionAttackControl` | `tasks/target-replay-evidence/L15-RR05-attack-statblock-routes.json#L15-RR05-BATTLE-ACTION-ATTACK-STATBLOCK-ROUTES replay action=doDiscoverRolledActionAttackControl#step:doDiscoverRolledActionAttackControl` | `src/qnt_adapters/battle_runtime_stat_block_action_ordering.rs` | `covered` |
| `cleanroom-input/qnt/battle-runtime/battle-runtime-stat-block-action-ordering.mbt.qnt#step:doDiscoverStaticActionAttackControl` | `tasks/target-replay-evidence/L15-RR05-attack-statblock-routes.json#L15-RR05-BATTLE-ACTION-ATTACK-STATBLOCK-ROUTES replay action=doDiscoverStaticActionAttackControl#step:doDiscoverStaticActionAttackControl` | `src/qnt_adapters/battle_runtime_stat_block_action_ordering.rs` | `covered` |
| `cleanroom-input/qnt/battle-runtime/battle-runtime-stat-block-action-ordering.mbt.qnt#step:doFillAttackRollMiss` | `tasks/target-replay-evidence/L15-RR05-attack-statblock-routes.json#L15-RR05-BATTLE-ACTION-ATTACK-STATBLOCK-ROUTES replay action=doFillAttackRollMiss#step:doFillAttackRollMiss` | `src/qnt_adapters/battle_runtime_stat_block_action_ordering.rs` | `covered` |
| `cleanroom-input/qnt/battle-runtime/battle-runtime-stat-block-action-ordering.mbt.qnt#step:doFillDamageDice` | `tasks/target-replay-evidence/L15-RR05-attack-statblock-routes.json#L15-RR05-BATTLE-ACTION-ATTACK-STATBLOCK-ROUTES replay action=doFillDamageDice#step:doFillDamageDice` | `src/qnt_adapters/battle_runtime_stat_block_action_ordering.rs` | `covered` |
| `cleanroom-input/qnt/battle-runtime/battle-runtime-stat-block-action-ordering.mbt.qnt#step:doFillRechargeRoll` | `tasks/target-replay-evidence/L15-RR05-attack-statblock-routes.json#L15-RR05-BATTLE-ACTION-ATTACK-STATBLOCK-ROUTES replay action=doFillRechargeRoll#step:doFillRechargeRoll` | `src/qnt_adapters/battle_runtime_stat_block_action_ordering.rs` | `covered` |
| `cleanroom-input/qnt/battle-runtime/battle-runtime-stat-block-action-ordering.mbt.qnt#step:doFillRolledAttackRollHit` | `tasks/target-replay-evidence/L15-RR05-attack-statblock-routes.json#L15-RR05-BATTLE-ACTION-ATTACK-STATBLOCK-ROUTES replay action=doFillRolledAttackRollHit#step:doFillRolledAttackRollHit` | `src/qnt_adapters/battle_runtime_stat_block_action_ordering.rs` | `covered` |
| `cleanroom-input/qnt/battle-runtime/battle-runtime-stat-block-action-ordering.mbt.qnt#step:doFillStaticAttackRollHit` | `tasks/target-replay-evidence/L15-RR05-attack-statblock-routes.json#L15-RR05-BATTLE-ACTION-ATTACK-STATBLOCK-ROUTES replay action=doFillStaticAttackRollHit#step:doFillStaticAttackRollHit` | `src/qnt_adapters/battle_runtime_stat_block_action_ordering.rs` | `covered` |
| `cleanroom-input/qnt/battle-runtime/battle-runtime-stat-block-action-ordering.mbt.qnt#step:doFillTargetChoice` | `tasks/target-replay-evidence/L15-RR05-attack-statblock-routes.json#L15-RR05-BATTLE-ACTION-ATTACK-STATBLOCK-ROUTES replay action=doFillTargetChoice#step:doFillTargetChoice` | `src/qnt_adapters/battle_runtime_stat_block_action_ordering.rs` | `covered` |
| `cleanroom-input/qnt/battle-runtime/battle-runtime-stat-block-action-ordering.mbt.qnt#step:doRejectAttackRollBeforeTargetChoice` | `tasks/target-replay-evidence/L15-RR05-attack-statblock-routes.json#L15-RR05-BATTLE-ACTION-ATTACK-STATBLOCK-ROUTES replay action=doRejectAttackRollBeforeTargetChoice#step:doRejectAttackRollBeforeTargetChoice` | `src/qnt_adapters/battle_runtime_stat_block_action_ordering.rs` | `covered` |
| `cleanroom-input/qnt/battle-runtime/battle-runtime-stat-block-action-ordering.mbt.qnt#step:doRejectDamageBeforeAttackRoll` | `tasks/target-replay-evidence/L15-RR05-attack-statblock-routes.json#L15-RR05-BATTLE-ACTION-ATTACK-STATBLOCK-ROUTES replay action=doRejectDamageBeforeAttackRoll#step:doRejectDamageBeforeAttackRoll` | `src/qnt_adapters/battle_runtime_stat_block_action_ordering.rs` | `covered` |
| `cleanroom-input/qnt/battle-runtime/battle-runtime-stat-block-action-ordering.mbt.qnt#step:doSpendRechargeGatedRolledAttack` | `tasks/target-replay-evidence/L15-RR05-attack-statblock-routes.json#L15-RR05-BATTLE-ACTION-ATTACK-STATBLOCK-ROUTES replay action=doSpendRechargeGatedRolledAttack#step:doSpendRechargeGatedRolledAttack` | `src/qnt_adapters/battle_runtime_stat_block_action_ordering.rs` | `covered` |
| `cleanroom-input/qnt/battle-runtime/battle-runtime-stat-block-action-ordering.mbt.qnt#step:doStartMultiattackControl` | `tasks/target-replay-evidence/L15-RR05-attack-statblock-routes.json#L15-RR05-BATTLE-ACTION-ATTACK-STATBLOCK-ROUTES replay action=doStartMultiattackControl#step:doStartMultiattackControl` | `src/qnt_adapters/battle_runtime_stat_block_action_ordering.rs` | `covered` |
| `cleanroom-input/qnt/battle-runtime/battle-runtime-stat-block-multi-damage.mbt.qnt#step:doFillHitAttackRoll` | `tasks/target-replay-evidence/L15-RR05-attack-statblock-routes.json#L15-RR05-BATTLE-ACTION-ATTACK-STATBLOCK-ROUTES replay action=doFillHitAttackRoll#step:doFillHitAttackRoll` | `src/qnt_adapters/battle_runtime_stat_block_multi_damage.rs` | `covered` |
| `cleanroom-input/qnt/battle-runtime/battle-runtime-stat-block-multi-damage.mbt.qnt#step:doFillTargetChoice` | `tasks/target-replay-evidence/L15-RR05-attack-statblock-routes.json#L15-RR05-BATTLE-ACTION-ATTACK-STATBLOCK-ROUTES replay action=doFillTargetChoice#step:doFillTargetChoice` | `src/qnt_adapters/battle_runtime_stat_block_multi_damage.rs` | `covered` |
| `cleanroom-input/qnt/battle-runtime/battle-runtime-stat-block-multi-damage.mbt.qnt#step:doResolveRolledDamage` | `tasks/target-replay-evidence/L15-RR05-attack-statblock-routes.json#L15-RR05-BATTLE-ACTION-ATTACK-STATBLOCK-ROUTES replay action=doResolveRolledDamage#step:doResolveRolledDamage` | `src/qnt_adapters/battle_runtime_stat_block_multi_damage.rs` | `covered` |
| `cleanroom-input/qnt/battle-runtime/battle-runtime-stat-block-size-gated-condition-rider.mbt.qnt#step:doFillHitAttackRoll` | `tasks/target-replay-evidence/L15-RR05-attack-statblock-routes.json#L15-RR05-BATTLE-ACTION-ATTACK-STATBLOCK-ROUTES replay action=doFillHitAttackRoll#step:doFillHitAttackRoll` | `src/qnt_adapters/battle_runtime_stat_block_size_gated_condition_rider.rs` | `covered` |
| `cleanroom-input/qnt/battle-runtime/battle-runtime-stat-block-size-gated-condition-rider.mbt.qnt#step:doFillTargetChoice` | `tasks/target-replay-evidence/L15-RR05-attack-statblock-routes.json#L15-RR05-BATTLE-ACTION-ATTACK-STATBLOCK-ROUTES replay action=doFillTargetChoice#step:doFillTargetChoice` | `src/qnt_adapters/battle_runtime_stat_block_size_gated_condition_rider.rs` | `covered` |
| `cleanroom-input/qnt/battle-runtime/battle-runtime-stat-block-size-gated-condition-rider.mbt.qnt#step:doResolveDamage` | `tasks/target-replay-evidence/L15-RR05-attack-statblock-routes.json#L15-RR05-BATTLE-ACTION-ATTACK-STATBLOCK-ROUTES replay action=doResolveDamage#step:doResolveDamage` | `src/qnt_adapters/battle_runtime_stat_block_size_gated_condition_rider.rs` | `covered` |
| `cleanroom-input/qnt/battle-runtime/battle-runtime-weapon-attack-ordering.mbt.qnt#step:doDiscoverAttack` | `tasks/target-replay-evidence/L15-RR05-attack-statblock-routes.json#L15-RR05-BATTLE-ACTION-ATTACK-STATBLOCK-ROUTES replay action=doDiscoverAttack#step:doDiscoverAttack` | `src/qnt_adapters/battle_runtime_weapon_attack_ordering.rs` | `covered` |
| `cleanroom-input/qnt/battle-runtime/battle-runtime-weapon-attack-ordering.mbt.qnt#step:doFillAttackRollHit` | `tasks/target-replay-evidence/L15-RR05-attack-statblock-routes.json#L15-RR05-BATTLE-ACTION-ATTACK-STATBLOCK-ROUTES replay action=doFillAttackRollHit#step:doFillAttackRollHit` | `src/qnt_adapters/battle_runtime_weapon_attack_ordering.rs` | `covered` |
| `cleanroom-input/qnt/battle-runtime/battle-runtime-weapon-attack-ordering.mbt.qnt#step:doFillAttackRollMiss` | `tasks/target-replay-evidence/L15-RR05-attack-statblock-routes.json#L15-RR05-BATTLE-ACTION-ATTACK-STATBLOCK-ROUTES replay action=doFillAttackRollMiss#step:doFillAttackRollMiss` | `src/qnt_adapters/battle_runtime_weapon_attack_ordering.rs` | `covered` |
| `cleanroom-input/qnt/battle-runtime/battle-runtime-weapon-attack-ordering.mbt.qnt#step:doFillDamageDice` | `tasks/target-replay-evidence/L15-RR05-attack-statblock-routes.json#L15-RR05-BATTLE-ACTION-ATTACK-STATBLOCK-ROUTES replay action=doFillDamageDice#step:doFillDamageDice` | `src/qnt_adapters/battle_runtime_weapon_attack_ordering.rs` | `covered` |
| `cleanroom-input/qnt/battle-runtime/battle-runtime-weapon-attack-ordering.mbt.qnt#step:doFillTargetChoice` | `tasks/target-replay-evidence/L15-RR05-attack-statblock-routes.json#L15-RR05-BATTLE-ACTION-ATTACK-STATBLOCK-ROUTES replay action=doFillTargetChoice#step:doFillTargetChoice` | `src/qnt_adapters/battle_runtime_weapon_attack_ordering.rs` | `covered` |
| `cleanroom-input/qnt/battle-runtime/battle-runtime-weapon-attack-ordering.mbt.qnt#step:doRejectAttackRollBeforeTargetChoice` | `tasks/target-replay-evidence/L15-RR05-attack-statblock-routes.json#L15-RR05-BATTLE-ACTION-ATTACK-STATBLOCK-ROUTES replay action=doRejectAttackRollBeforeTargetChoice#step:doRejectAttackRollBeforeTargetChoice` | `src/qnt_adapters/battle_runtime_weapon_attack_ordering.rs` | `covered` |
| `cleanroom-input/qnt/battle-runtime/battle-runtime-weapon-attack-ordering.mbt.qnt#step:doRejectDamageBeforeAttackRoll` | `tasks/target-replay-evidence/L15-RR05-attack-statblock-routes.json#L15-RR05-BATTLE-ACTION-ATTACK-STATBLOCK-ROUTES replay action=doRejectDamageBeforeAttackRoll#step:doRejectDamageBeforeAttackRoll` | `src/qnt_adapters/battle_runtime_weapon_attack_ordering.rs` | `covered` |
| `cleanroom-input/qnt/battle-runtime/battle-runtime-weapon-attack-skeleton.mbt.qnt#step:doDiscoverAttack` | `tasks/target-replay-evidence/L15-RR05-attack-statblock-routes.json#L15-RR05-BATTLE-ACTION-ATTACK-STATBLOCK-ROUTES replay action=doDiscoverAttack#step:doDiscoverAttack` | `src/qnt_adapters/battle_runtime_weapon_attack_skeleton.rs` | `covered` |
| `cleanroom-input/qnt/battle-runtime/battle-runtime-weapon-attack-skeleton.mbt.qnt#step:doFillAttackRollHit` | `tasks/target-replay-evidence/L15-RR05-attack-statblock-routes.json#L15-RR05-BATTLE-ACTION-ATTACK-STATBLOCK-ROUTES replay action=doFillAttackRollHit#step:doFillAttackRollHit` | `src/qnt_adapters/battle_runtime_weapon_attack_skeleton.rs` | `covered` |
| `cleanroom-input/qnt/battle-runtime/battle-runtime-weapon-attack-skeleton.mbt.qnt#step:doFillAttackRollMiss` | `tasks/target-replay-evidence/L15-RR05-attack-statblock-routes.json#L15-RR05-BATTLE-ACTION-ATTACK-STATBLOCK-ROUTES replay action=doFillAttackRollMiss#step:doFillAttackRollMiss` | `src/qnt_adapters/battle_runtime_weapon_attack_skeleton.rs` | `covered` |
| `cleanroom-input/qnt/battle-runtime/battle-runtime-weapon-attack-skeleton.mbt.qnt#step:doFillDamageHigh` | `tasks/target-replay-evidence/L15-RR05-attack-statblock-routes.json#L15-RR05-BATTLE-ACTION-ATTACK-STATBLOCK-ROUTES replay action=doFillDamageHigh#step:doFillDamageHigh` | `src/qnt_adapters/battle_runtime_weapon_attack_skeleton.rs` | `covered` |
| `cleanroom-input/qnt/battle-runtime/battle-runtime-weapon-attack-skeleton.mbt.qnt#step:doFillDamageHighSneakAttack` | `tasks/target-replay-evidence/L15-RR05-attack-statblock-routes.json#L15-RR05-BATTLE-ACTION-ATTACK-STATBLOCK-ROUTES replay action=doFillDamageHighSneakAttack#step:doFillDamageHighSneakAttack` | `src/qnt_adapters/battle_runtime_weapon_attack_skeleton.rs` | `covered` |
| `cleanroom-input/qnt/battle-runtime/battle-runtime-weapon-attack-skeleton.mbt.qnt#step:doFillDamageLow` | `tasks/target-replay-evidence/L15-RR05-attack-statblock-routes.json#L15-RR05-BATTLE-ACTION-ATTACK-STATBLOCK-ROUTES replay action=doFillDamageLow#step:doFillDamageLow` | `src/qnt_adapters/battle_runtime_weapon_attack_skeleton.rs` | `covered` |
| `cleanroom-input/qnt/battle-runtime/battle-runtime-weapon-attack-skeleton.mbt.qnt#step:doFillDamageLowSneakAttack` | `tasks/target-replay-evidence/L15-RR05-attack-statblock-routes.json#L15-RR05-BATTLE-ACTION-ATTACK-STATBLOCK-ROUTES replay action=doFillDamageLowSneakAttack#step:doFillDamageLowSneakAttack` | `src/qnt_adapters/battle_runtime_weapon_attack_skeleton.rs` | `covered` |
| `cleanroom-input/qnt/battle-runtime/battle-runtime-weapon-attack-skeleton.mbt.qnt#step:doFillTarget` | `tasks/target-replay-evidence/L15-RR05-attack-statblock-routes.json#L15-RR05-BATTLE-ACTION-ATTACK-STATBLOCK-ROUTES replay action=doFillTarget#step:doFillTarget` | `src/qnt_adapters/battle_runtime_weapon_attack_skeleton.rs` | `covered` |
| `cleanroom-input/qnt/battle-runtime/battle-runtime-weapon-attack-skeleton.mbt.qnt#step:doRejectRecursiveSkeletonMultiattack` | `tasks/target-replay-evidence/L15-RR05-attack-statblock-routes.json#L15-RR05-BATTLE-ACTION-ATTACK-STATBLOCK-ROUTES replay action=doRejectRecursiveSkeletonMultiattack#step:doRejectRecursiveSkeletonMultiattack` | `src/qnt_adapters/battle_runtime_weapon_attack_skeleton.rs` | `covered` |
| `cleanroom-input/qnt/battle-runtime/battle-runtime-weapon-attack-skeleton.mbt.qnt#step:doRejectStaleAfterResolved` | `tasks/target-replay-evidence/L15-RR05-attack-statblock-routes.json#L15-RR05-BATTLE-ACTION-ATTACK-STATBLOCK-ROUTES replay action=doRejectStaleAfterResolved#step:doRejectStaleAfterResolved` | `src/qnt_adapters/battle_runtime_weapon_attack_skeleton.rs` | `covered` |
| `cleanroom-input/qnt/battle-runtime/battle-runtime-weapon-attack-skeleton.mbt.qnt#step:doRejectWrongTarget` | `tasks/target-replay-evidence/L15-RR05-attack-statblock-routes.json#L15-RR05-BATTLE-ACTION-ATTACK-STATBLOCK-ROUTES replay action=doRejectWrongTarget#step:doRejectWrongTarget` | `src/qnt_adapters/battle_runtime_weapon_attack_skeleton.rs` | `covered` |
| `cleanroom-input/qnt/battle-runtime/battle-runtime-weapon-attack-skeleton.mbt.qnt#step:doResolveSkeletonMultiattack` | `tasks/target-replay-evidence/L15-RR05-attack-statblock-routes.json#L15-RR05-BATTLE-ACTION-ATTACK-STATBLOCK-ROUTES replay action=doResolveSkeletonMultiattack#step:doResolveSkeletonMultiattack` | `src/qnt_adapters/battle_runtime_weapon_attack_skeleton.rs` | `covered` |
| `cleanroom-input/qnt/battle-runtime/battle-runtime-weapon-attack-skeleton.mbt.qnt#step:doSpendSkeletonMultiattackDispatch` | `tasks/target-replay-evidence/L15-RR05-attack-statblock-routes.json#L15-RR05-BATTLE-ACTION-ATTACK-STATBLOCK-ROUTES replay action=doSpendSkeletonMultiattackDispatch#step:doSpendSkeletonMultiattackDispatch` | `src/qnt_adapters/battle_runtime_weapon_attack_skeleton.rs` | `covered` |
| `cleanroom-input/qnt/battle-runtime/battle-runtime-weapon-attack-skeleton.mbt.qnt#step:doStartSkeletonTurn` | `tasks/target-replay-evidence/L15-RR05-attack-statblock-routes.json#L15-RR05-BATTLE-ACTION-ATTACK-STATBLOCK-ROUTES replay action=doStartSkeletonTurn#step:doStartSkeletonTurn` | `src/qnt_adapters/battle_runtime_weapon_attack_skeleton.rs` | `covered` |
| `cleanroom-input/qnt/battle-runtime/creature-attack.mbt.qnt#step:doAttackerAAttacks` | `tasks/target-replay-evidence/L15-RR05-attack-statblock-routes.json#L15-RR05-BATTLE-ACTION-ATTACK-STATBLOCK-ROUTES replay action=doAttackerAAttacks#step:doAttackerAAttacks` | `src/qnt_adapters/creature_attack_mbt.rs` | `covered` |
| `cleanroom-input/qnt/battle-runtime/creature-attack.mbt.qnt#step:doAttackerBAttacks` | `tasks/target-replay-evidence/L15-RR05-attack-statblock-routes.json#L15-RR05-BATTLE-ACTION-ATTACK-STATBLOCK-ROUTES replay action=doAttackerBAttacks#step:doAttackerBAttacks` | `src/qnt_adapters/creature_attack_mbt.rs` | `covered` |

- Start gate: `tasks/history/L15-RR05-BATTLE-ACTION-ATTACK-STATBLOCK-ROUTES/START_GATE.json`
- Engine depth: `tasks/history/L15-RR05-BATTLE-ACTION-ATTACK-STATBLOCK-ROUTES/ENGINE_DEPTH_MANIFEST.json`
- State ownership: `tasks/history/L15-RR05-BATTLE-ACTION-ATTACK-STATBLOCK-ROUTES/STATE_OWNER_MANIFEST.json`
- Reviewer loop: `tasks/history/L15-RR05-BATTLE-ACTION-ATTACK-STATBLOCK-ROUTES/REVIEW_LOOP.json`
- Decider decision: `tasks/history/L15-RR05-BATTLE-ACTION-ATTACK-STATBLOCK-ROUTES/DECIDER_DECISION.json`

## L15-RR06-BATTLE-SPELL-EFFECT-ROUTES: Battle Spell Effect Routes

- Manifest source commit SHA: `564376fd95218a209bb9eae5c9ccb54ca3e04a52`
- Source branch inventory SHA: `4bb2b20a85d94e3b90b7c59cbfe6e1edd5ab3ef40410641e999527861f3d3a32`
- Drivers:
  - `cleanroom-input/qnt/battle-runtime/battle-runtime-command-option-next-turn.mbt.qnt`
  - `cleanroom-input/qnt/battle-runtime/battle-runtime-command-ordering.mbt.qnt`
  - `cleanroom-input/qnt/battle-runtime/battle-runtime-scalar-buff.mbt.qnt`

Behavior implemented:

- Command ordering route evidence is checked against `cleanroom-input/qnt/battle-runtime/battle-runtime-command-ordering.route.mbt.qnt`, including `doApproachMovementContinues` as a terminal `discover_battle_acts` route for `CommandApproachMovementStage` under `BattleActiveEffectOwner`.
- Command next-turn remains a literal projection witness driver. Its evidence rows keep the harness-required `qRoute` projection because the reducer-route inventory pairs the driver with `battle-runtime-command-ordering.route.mbt.qnt`, and each row now records that paired connector path; `doFollowGrovel` is checked as a resolved `resolve_battle_subject_without_fill` route under `BattleConditionLifecycleOwner`.
- The Rust reducer now keeps resolved Command subjects with pending next-turn work live for follow resolution and exposes active Command frontiers through `discover_battle_acts`.
- Scalar buff target routing uses `Combatant.speed_feet`, typed scalar target choice, and stale-subject results from the shared reducer surface.

Generated branch coverage:

| Obligation | Target replay evidence | Diagnostic tests | Status |
| --- | --- | --- | --- |
| `cleanroom-input/qnt/battle-runtime/battle-runtime-command-option-next-turn.mbt.qnt#step:doFailedSaveRecordsPending` | `tasks/target-replay-evidence/L15-RR06-spell-effect-routes.json#L15-RR06-BATTLE-SPELL-EFFECT-ROUTES replay action=doFailedSaveRecordsPending#step:doFailedSaveRecordsPending` | `src/qnt_adapters/battle_runtime_command_option_next_turn.rs` | `covered` |
| `cleanroom-input/qnt/battle-runtime/battle-runtime-command-option-next-turn.mbt.qnt#step:doFollowGrovel` | `tasks/target-replay-evidence/L15-RR06-spell-effect-routes.json#L15-RR06-BATTLE-SPELL-EFFECT-ROUTES replay action=doFollowGrovel#step:doFollowGrovel` | `src/qnt_adapters/battle_runtime_command_option_next_turn.rs` | `covered` |
| `cleanroom-input/qnt/battle-runtime/battle-runtime-command-option-next-turn.mbt.qnt#step:doFollowDrop` | `tasks/target-replay-evidence/L15-RR06-spell-effect-routes.json#L15-RR06-BATTLE-SPELL-EFFECT-ROUTES replay action=doFollowDrop#step:doFollowDrop` | `src/qnt_adapters/battle_runtime_command_option_next_turn.rs` | `covered` |
| `cleanroom-input/qnt/battle-runtime/battle-runtime-command-option-next-turn.mbt.qnt#step:doHaltSuppresses` | `tasks/target-replay-evidence/L15-RR06-spell-effect-routes.json#L15-RR06-BATTLE-SPELL-EFFECT-ROUTES replay action=doHaltSuppresses#step:doHaltSuppresses` | `src/qnt_adapters/battle_runtime_command_option_next_turn.rs` | `covered` |
| `cleanroom-input/qnt/battle-runtime/battle-runtime-command-option-next-turn.mbt.qnt#step:doHaltEndTurnCleanup` | `tasks/target-replay-evidence/L15-RR06-spell-effect-routes.json#L15-RR06-BATTLE-SPELL-EFFECT-ROUTES replay action=doHaltEndTurnCleanup#step:doHaltEndTurnCleanup` | `src/qnt_adapters/battle_runtime_command_option_next_turn.rs` | `covered` |
| `cleanroom-input/qnt/battle-runtime/battle-runtime-command-option-next-turn.mbt.qnt#step:doApproachContinues` | `tasks/target-replay-evidence/L15-RR06-spell-effect-routes.json#L15-RR06-BATTLE-SPELL-EFFECT-ROUTES replay action=doApproachContinues#step:doApproachContinues` | `src/qnt_adapters/battle_runtime_command_option_next_turn.rs` | `covered` |
| `cleanroom-input/qnt/battle-runtime/battle-runtime-command-option-next-turn.mbt.qnt#step:doApproachWithinFiveEndsTurn` | `tasks/target-replay-evidence/L15-RR06-spell-effect-routes.json#L15-RR06-BATTLE-SPELL-EFFECT-ROUTES replay action=doApproachWithinFiveEndsTurn#step:doApproachWithinFiveEndsTurn` | `src/qnt_adapters/battle_runtime_command_option_next_turn.rs` | `covered` |
| `cleanroom-input/qnt/battle-runtime/battle-runtime-command-option-next-turn.mbt.qnt#step:doApproachMovementRejected` | `tasks/target-replay-evidence/L15-RR06-spell-effect-routes.json#L15-RR06-BATTLE-SPELL-EFFECT-ROUTES replay action=doApproachMovementRejected#step:doApproachMovementRejected` | `src/qnt_adapters/battle_runtime_command_option_next_turn.rs` | `covered` |
| `cleanroom-input/qnt/battle-runtime/battle-runtime-command-option-next-turn.mbt.qnt#step:doApproachNoMovementCleanup` | `tasks/target-replay-evidence/L15-RR06-spell-effect-routes.json#L15-RR06-BATTLE-SPELL-EFFECT-ROUTES replay action=doApproachNoMovementCleanup#step:doApproachNoMovementCleanup` | `src/qnt_adapters/battle_runtime_command_option_next_turn.rs` | `covered` |
| `cleanroom-input/qnt/battle-runtime/battle-runtime-command-option-next-turn.mbt.qnt#step:doFleeFullMovementEndsTurn` | `tasks/target-replay-evidence/L15-RR06-spell-effect-routes.json#L15-RR06-BATTLE-SPELL-EFFECT-ROUTES replay action=doFleeFullMovementEndsTurn#step:doFleeFullMovementEndsTurn` | `src/qnt_adapters/battle_runtime_command_option_next_turn.rs` | `covered` |
| `cleanroom-input/qnt/battle-runtime/battle-runtime-command-option-next-turn.mbt.qnt#step:doFleePartialMovementRejected` | `tasks/target-replay-evidence/L15-RR06-spell-effect-routes.json#L15-RR06-BATTLE-SPELL-EFFECT-ROUTES replay action=doFleePartialMovementRejected#step:doFleePartialMovementRejected` | `src/qnt_adapters/battle_runtime_command_option_next_turn.rs` | `covered` |
| `cleanroom-input/qnt/battle-runtime/battle-runtime-command-option-next-turn.mbt.qnt#step:doFleeNoMovementCleanup` | `tasks/target-replay-evidence/L15-RR06-spell-effect-routes.json#L15-RR06-BATTLE-SPELL-EFFECT-ROUTES replay action=doFleeNoMovementCleanup#step:doFleeNoMovementCleanup` | `src/qnt_adapters/battle_runtime_command_option_next_turn.rs` | `covered` |
| `cleanroom-input/qnt/battle-runtime/battle-runtime-command-option-next-turn.mbt.qnt#step:doFleeOpportunityAttackWindow` | `tasks/target-replay-evidence/L15-RR06-spell-effect-routes.json#L15-RR06-BATTLE-SPELL-EFFECT-ROUTES replay action=doFleeOpportunityAttackWindow#step:doFleeOpportunityAttackWindow` | `src/qnt_adapters/battle_runtime_command_option_next_turn.rs` | `covered` |
| `cleanroom-input/qnt/battle-runtime/battle-runtime-command-option-next-turn.mbt.qnt#step:doFleeOpportunityAttackDeclinedContinuation` | `tasks/target-replay-evidence/L15-RR06-spell-effect-routes.json#L15-RR06-BATTLE-SPELL-EFFECT-ROUTES replay action=doFleeOpportunityAttackDeclinedContinuation#step:doFleeOpportunityAttackDeclinedContinuation` | `src/qnt_adapters/battle_runtime_command_option_next_turn.rs` | `covered` |
| `cleanroom-input/qnt/battle-runtime/battle-runtime-command-option-next-turn.mbt.qnt#step:doComplete` | `tasks/target-replay-evidence/L15-RR06-spell-effect-routes.json#L15-RR06-BATTLE-SPELL-EFFECT-ROUTES replay action=doComplete#step:doComplete` | `src/qnt_adapters/battle_runtime_command_option_next_turn.rs` | `covered` |
| `cleanroom-input/qnt/battle-runtime/battle-runtime-command-ordering.mbt.qnt#step:doDiscoverCommand` | `tasks/target-replay-evidence/L15-RR06-spell-effect-routes.json#L15-RR06-BATTLE-SPELL-EFFECT-ROUTES replay action=doDiscoverCommand#step:doDiscoverCommand` | `src/qnt_adapters/battle_runtime_command_ordering.rs` | `covered` |
| `cleanroom-input/qnt/battle-runtime/battle-runtime-command-ordering.mbt.qnt#step:doSubmitOptionBeforeTargetList` | `tasks/target-replay-evidence/L15-RR06-spell-effect-routes.json#L15-RR06-BATTLE-SPELL-EFFECT-ROUTES replay action=doSubmitOptionBeforeTargetList#step:doSubmitOptionBeforeTargetList` | `src/qnt_adapters/battle_runtime_command_ordering.rs` | `covered` |
| `cleanroom-input/qnt/battle-runtime/battle-runtime-command-ordering.mbt.qnt#step:doFillTargetList` | `tasks/target-replay-evidence/L15-RR06-spell-effect-routes.json#L15-RR06-BATTLE-SPELL-EFFECT-ROUTES replay action=doFillTargetList#step:doFillTargetList` | `src/qnt_adapters/battle_runtime_command_ordering.rs` | `covered` |
| `cleanroom-input/qnt/battle-runtime/battle-runtime-command-ordering.mbt.qnt#step:doSubmitSavingThrowBeforeOption` | `tasks/target-replay-evidence/L15-RR06-spell-effect-routes.json#L15-RR06-BATTLE-SPELL-EFFECT-ROUTES replay action=doSubmitSavingThrowBeforeOption#step:doSubmitSavingThrowBeforeOption` | `src/qnt_adapters/battle_runtime_command_ordering.rs` | `covered` |
| `cleanroom-input/qnt/battle-runtime/battle-runtime-command-ordering.mbt.qnt#step:doFillGrovelOption` | `tasks/target-replay-evidence/L15-RR06-spell-effect-routes.json#L15-RR06-BATTLE-SPELL-EFFECT-ROUTES replay action=doFillGrovelOption#step:doFillGrovelOption` | `src/qnt_adapters/battle_runtime_command_ordering.rs` | `covered` |
| `cleanroom-input/qnt/battle-runtime/battle-runtime-command-ordering.mbt.qnt#step:doFillFailedGrovelSavingThrow` | `tasks/target-replay-evidence/L15-RR06-spell-effect-routes.json#L15-RR06-BATTLE-SPELL-EFFECT-ROUTES replay action=doFillFailedGrovelSavingThrow#step:doFillFailedGrovelSavingThrow` | `src/qnt_adapters/battle_runtime_command_ordering.rs` | `covered` |
| `cleanroom-input/qnt/battle-runtime/battle-runtime-command-ordering.mbt.qnt#step:doFollowGrovel` | `tasks/target-replay-evidence/L15-RR06-spell-effect-routes.json#L15-RR06-BATTLE-SPELL-EFFECT-ROUTES replay action=doFollowGrovel#step:doFollowGrovel` | `src/qnt_adapters/battle_runtime_command_ordering.rs` | `covered` |
| `cleanroom-input/qnt/battle-runtime/battle-runtime-command-ordering.mbt.qnt#step:doDropNeedsHeldObjectFacts` | `tasks/target-replay-evidence/L15-RR06-spell-effect-routes.json#L15-RR06-BATTLE-SPELL-EFFECT-ROUTES replay action=doDropNeedsHeldObjectFacts#step:doDropNeedsHeldObjectFacts` | `src/qnt_adapters/battle_runtime_command_ordering.rs` | `covered` |
| `cleanroom-input/qnt/battle-runtime/battle-runtime-command-ordering.mbt.qnt#step:doFillDropHeldObjectFacts` | `tasks/target-replay-evidence/L15-RR06-spell-effect-routes.json#L15-RR06-BATTLE-SPELL-EFFECT-ROUTES replay action=doFillDropHeldObjectFacts#step:doFillDropHeldObjectFacts` | `src/qnt_adapters/battle_runtime_command_ordering.rs` | `covered` |
| `cleanroom-input/qnt/battle-runtime/battle-runtime-command-ordering.mbt.qnt#step:doHaltSuppresses` | `tasks/target-replay-evidence/L15-RR06-spell-effect-routes.json#L15-RR06-BATTLE-SPELL-EFFECT-ROUTES replay action=doHaltSuppresses#step:doHaltSuppresses` | `src/qnt_adapters/battle_runtime_command_ordering.rs` | `covered` |
| `cleanroom-input/qnt/battle-runtime/battle-runtime-command-ordering.mbt.qnt#step:doApproachMovementContinues` | `tasks/target-replay-evidence/L15-RR06-spell-effect-routes.json#L15-RR06-BATTLE-SPELL-EFFECT-ROUTES replay action=doApproachMovementContinues#step:doApproachMovementContinues` | `src/qnt_adapters/battle_runtime_command_ordering.rs` | `covered` |
| `cleanroom-input/qnt/battle-runtime/battle-runtime-command-ordering.mbt.qnt#step:doFillApproachMovementContinues` | `tasks/target-replay-evidence/L15-RR06-spell-effect-routes.json#L15-RR06-BATTLE-SPELL-EFFECT-ROUTES replay action=doFillApproachMovementContinues#step:doFillApproachMovementContinues` | `src/qnt_adapters/battle_runtime_command_ordering.rs` | `covered` |
| `cleanroom-input/qnt/battle-runtime/battle-runtime-command-ordering.mbt.qnt#step:doFillApproachMovementWithinFive` | `tasks/target-replay-evidence/L15-RR06-spell-effect-routes.json#L15-RR06-BATTLE-SPELL-EFFECT-ROUTES replay action=doFillApproachMovementWithinFive#step:doFillApproachMovementWithinFive` | `src/qnt_adapters/battle_runtime_command_ordering.rs` | `covered` |
| `cleanroom-input/qnt/battle-runtime/battle-runtime-command-ordering.mbt.qnt#step:doApproachNoMovement` | `tasks/target-replay-evidence/L15-RR06-spell-effect-routes.json#L15-RR06-BATTLE-SPELL-EFFECT-ROUTES replay action=doApproachNoMovement#step:doApproachNoMovement` | `src/qnt_adapters/battle_runtime_command_ordering.rs` | `covered` |
| `cleanroom-input/qnt/battle-runtime/battle-runtime-command-ordering.mbt.qnt#step:doFleeMovement` | `tasks/target-replay-evidence/L15-RR06-spell-effect-routes.json#L15-RR06-BATTLE-SPELL-EFFECT-ROUTES replay action=doFleeMovement#step:doFleeMovement` | `src/qnt_adapters/battle_runtime_command_ordering.rs` | `covered` |
| `cleanroom-input/qnt/battle-runtime/battle-runtime-command-ordering.mbt.qnt#step:doFillFleeMovement` | `tasks/target-replay-evidence/L15-RR06-spell-effect-routes.json#L15-RR06-BATTLE-SPELL-EFFECT-ROUTES replay action=doFillFleeMovement#step:doFillFleeMovement` | `src/qnt_adapters/battle_runtime_command_ordering.rs` | `covered` |
| `cleanroom-input/qnt/battle-runtime/battle-runtime-command-ordering.mbt.qnt#step:doRejectFleePartialMovement` | `tasks/target-replay-evidence/L15-RR06-spell-effect-routes.json#L15-RR06-BATTLE-SPELL-EFFECT-ROUTES replay action=doRejectFleePartialMovement#step:doRejectFleePartialMovement` | `src/qnt_adapters/battle_runtime_command_ordering.rs` | `covered` |
| `cleanroom-input/qnt/battle-runtime/battle-runtime-command-ordering.mbt.qnt#step:doFleeNoMovement` | `tasks/target-replay-evidence/L15-RR06-spell-effect-routes.json#L15-RR06-BATTLE-SPELL-EFFECT-ROUTES replay action=doFleeNoMovement#step:doFleeNoMovement` | `src/qnt_adapters/battle_runtime_command_ordering.rs` | `covered` |
| `cleanroom-input/qnt/battle-runtime/battle-runtime-command-ordering.mbt.qnt#step:doFleeOpportunityAttack` | `tasks/target-replay-evidence/L15-RR06-spell-effect-routes.json#L15-RR06-BATTLE-SPELL-EFFECT-ROUTES replay action=doFleeOpportunityAttack#step:doFleeOpportunityAttack` | `src/qnt_adapters/battle_runtime_command_ordering.rs` | `covered` |
| `cleanroom-input/qnt/battle-runtime/battle-runtime-scalar-buff.mbt.qnt#step:doFillLongstriderTarget` | `tasks/target-replay-evidence/L15-RR06-spell-effect-routes.json#L15-RR06-BATTLE-SPELL-EFFECT-ROUTES replay action=doFillLongstriderTarget#step:doFillLongstriderTarget` | `src/qnt_adapters/battle_runtime_scalar_buff.rs` | `covered` |
| `cleanroom-input/qnt/battle-runtime/battle-runtime-scalar-buff.mbt.qnt#step:doRejectStaleAfterResolved` | `tasks/target-replay-evidence/L15-RR06-spell-effect-routes.json#L15-RR06-BATTLE-SPELL-EFFECT-ROUTES replay action=doRejectStaleAfterResolved#step:doRejectStaleAfterResolved` | `src/qnt_adapters/battle_runtime_scalar_buff.rs` | `covered` |

Target replay evidence:

- Evidence file: `tasks/target-replay-evidence/L15-RR06-spell-effect-routes.json`
- Target profile: `rust`
- Target profile SHA-256: `6d4cc6c6a4769962798133d57aff01438fb2b661941f71d1aa8a3333f4b7ecc1`
- Run ledger: `tasks/RUN_LEDGER.json`

Remaining gaps:

- `_none_` for the two re-review findings. The dirty rehearsal still distinguishes literal command next-turn projection evidence from copied route-connector parity in the evidence metadata.

Verification results:

- `cargo test command_option_next_turn_adapter_replays_all_branches` passed.
- `cargo test command_ordering_adapter_replays_all_branches` passed.
- `cargo test scalar_buff_adapter_replays_all_branches` passed.
- `cargo fmt --check` passed.
- `cargo test` passed.
- `cargo clippy --all-targets -- -D warnings` passed.
- `node scripts/check-cleanroom-harness.cjs` passed.

## L15-RR04A-RULE-CORE-DAMAGE-STATBLOCK-COMPONENTS: Rule-Core Damage, HP, and Stat-Block Components

- Manifest source commit SHA: `564376fd95218a209bb9eae5c9ccb54ca3e04a52`
- Source branch inventory SHA: `4bb2b20a85d94e3b90b7c59cbfe6e1edd5ab3ef40410641e999527861f3d3a32`
- Drivers:
  - `cleanroom-input/qnt/battle-runtime/rule-core-attack-damage-disposition.mbt.qnt`
  - `cleanroom-input/qnt/battle-runtime/rule-core-hit-point-damage.mbt.qnt`
  - `cleanroom-input/qnt/battle-runtime/rule-core-stat-block-controls.mbt.qnt`

Allowed inputs used:

- `cleanroom-input/MANIFEST.md`
- `cleanroom-input/branch-coverage/source-branch-inventory.json`
- `cleanroom-input/branch-coverage/reducer-route-inventory.json`
- `cleanroom-input/qnt/battle-runtime/rule-core-component-route.qnt`
- `cleanroom-input/qnt/battle-runtime/rule-core-attack-damage-disposition.mbt.qnt`
- `cleanroom-input/qnt/battle-runtime/rule-core-hit-point-damage.mbt.qnt`
- `cleanroom-input/qnt/battle-runtime/rule-core-stat-block-controls.mbt.qnt`
- `cleanroom-input/qnt/shared-algebras/proofs/rule-core/attack-damage-composition.qnt`
- `cleanroom-input/qnt/shared-algebras/proofs/rule-core/hit-point-damage.qnt`
- `cleanroom-input/qnt/shared-algebras/proofs/rule-core/hit-point-recovery.qnt`
- `cleanroom-input/qnt/shared-algebras/proofs/rule-core/stat-block-controls.qnt`
- `cleanroom-input/domain/UBIQUITOUS_LANGUAGE.md`
- `cleanroom-input/raw/srd-5.2.1/Playing-the-Game.md`
- `cleanroom-input/raw/srd-5.2.1/Monsters/Overview.md`
- Repo-local `src/**`, `tasks/**`, scripts, and Cargo tooling

Behavior implemented:

- Added derived `qComponentRoute` projections to the three selected rule-core adapters, using the owner-specific route from `rule-core-component-route.qnt`.
- Kept damage, HP, and stat-block facts in reusable rule-core component modules; no production `BattleState` owner was added for witness/protocol state.
- Focused tests assert both component result facts and component route evidence for all selected branch actions.

Generated branch coverage:

| Obligation | Target replay evidence | Diagnostic tests | Status |
| --- | --- | --- | --- |
| `cleanroom-input/qnt/battle-runtime/rule-core-attack-damage-disposition.mbt.qnt#step:doMeleeKnockOut` | `tasks/target-replay-evidence/L15-RR04A-rule-core-damage-statblock-components.json#L15-RR04A-RULE-CORE-DAMAGE-STATBLOCK-COMPONENTS replay action=doMeleeKnockOut#step:doMeleeKnockOut` | `src/qnt_adapters/rule_core_attack_damage_disposition.rs` | `covered` |
| `cleanroom-input/qnt/battle-runtime/rule-core-attack-damage-disposition.mbt.qnt#step:doRejectRangedKnockOut` | `tasks/target-replay-evidence/L15-RR04A-rule-core-damage-statblock-components.json#L15-RR04A-RULE-CORE-DAMAGE-STATBLOCK-COMPONENTS replay action=doRejectRangedKnockOut#step:doRejectRangedKnockOut` | `src/qnt_adapters/rule_core_attack_damage_disposition.rs` | `covered` |
| `cleanroom-input/qnt/battle-runtime/rule-core-hit-point-damage.mbt.qnt#step:doMonsterDiesAtZero` | `tasks/target-replay-evidence/L15-RR04A-rule-core-damage-statblock-components.json#L15-RR04A-RULE-CORE-DAMAGE-STATBLOCK-COMPONENTS replay action=doMonsterDiesAtZero#step:doMonsterDiesAtZero` | `src/qnt_adapters/rule_core_hit_point_damage_mbt.rs` | `covered` |
| `cleanroom-input/qnt/battle-runtime/rule-core-hit-point-damage.mbt.qnt#step:doPlayerCharacterDiesFromMassiveDamage` | `tasks/target-replay-evidence/L15-RR04A-rule-core-damage-statblock-components.json#L15-RR04A-RULE-CORE-DAMAGE-STATBLOCK-COMPONENTS replay action=doPlayerCharacterDiesFromMassiveDamage#step:doPlayerCharacterDiesFromMassiveDamage` | `src/qnt_adapters/rule_core_hit_point_damage_mbt.rs` | `covered` |
| `cleanroom-input/qnt/battle-runtime/rule-core-hit-point-damage.mbt.qnt#step:doPlayerCharacterFallsUnconscious` | `tasks/target-replay-evidence/L15-RR04A-rule-core-damage-statblock-components.json#L15-RR04A-RULE-CORE-DAMAGE-STATBLOCK-COMPONENTS replay action=doPlayerCharacterFallsUnconscious#step:doPlayerCharacterFallsUnconscious` | `src/qnt_adapters/rule_core_hit_point_damage_mbt.rs` | `covered` |
| `cleanroom-input/qnt/battle-runtime/rule-core-hit-point-damage.mbt.qnt#step:doTemporaryHitPointsAbsorbFirst` | `tasks/target-replay-evidence/L15-RR04A-rule-core-damage-statblock-components.json#L15-RR04A-RULE-CORE-DAMAGE-STATBLOCK-COMPONENTS replay action=doTemporaryHitPointsAbsorbFirst#step:doTemporaryHitPointsAbsorbFirst` | `src/qnt_adapters/rule_core_hit_point_damage_mbt.rs` | `covered` |
| `cleanroom-input/qnt/battle-runtime/rule-core-stat-block-controls.mbt.qnt#step:doEndTurnClosesDispatches` | `tasks/target-replay-evidence/L15-RR04A-rule-core-damage-statblock-components.json#L15-RR04A-RULE-CORE-DAMAGE-STATBLOCK-COMPONENTS replay action=doEndTurnClosesDispatches#step:doEndTurnClosesDispatches` | `src/qnt_adapters/rule_core_stat_block_controls_mbt.rs` | `covered` |
| `cleanroom-input/qnt/battle-runtime/rule-core-stat-block-controls.mbt.qnt#step:doMoveDuringDispatch` | `tasks/target-replay-evidence/L15-RR04A-rule-core-damage-statblock-components.json#L15-RR04A-RULE-CORE-DAMAGE-STATBLOCK-COMPONENTS replay action=doMoveDuringDispatch#step:doMoveDuringDispatch` | `src/qnt_adapters/rule_core_stat_block_controls_mbt.rs` | `covered` |
| `cleanroom-input/qnt/battle-runtime/rule-core-stat-block-controls.mbt.qnt#step:doRejectBonusActionDuringDispatch` | `tasks/target-replay-evidence/L15-RR04A-rule-core-damage-statblock-components.json#L15-RR04A-RULE-CORE-DAMAGE-STATBLOCK-COMPONENTS replay action=doRejectBonusActionDuringDispatch#step:doRejectBonusActionDuringDispatch` | `src/qnt_adapters/rule_core_stat_block_controls_mbt.rs` | `covered` |
| `cleanroom-input/qnt/battle-runtime/rule-core-stat-block-controls.mbt.qnt#step:doRejectOrdinaryActionDuringDispatch` | `tasks/target-replay-evidence/L15-RR04A-rule-core-damage-statblock-components.json#L15-RR04A-RULE-CORE-DAMAGE-STATBLOCK-COMPONENTS replay action=doRejectOrdinaryActionDuringDispatch#step:doRejectOrdinaryActionDuringDispatch` | `src/qnt_adapters/rule_core_stat_block_controls_mbt.rs` | `covered` |
| `cleanroom-input/qnt/battle-runtime/rule-core-stat-block-controls.mbt.qnt#step:doResolvePrimaryDispatch` | `tasks/target-replay-evidence/L15-RR04A-rule-core-damage-statblock-components.json#L15-RR04A-RULE-CORE-DAMAGE-STATBLOCK-COMPONENTS replay action=doResolvePrimaryDispatch#step:doResolvePrimaryDispatch` | `src/qnt_adapters/rule_core_stat_block_controls_mbt.rs` | `covered` |
| `cleanroom-input/qnt/battle-runtime/rule-core-stat-block-controls.mbt.qnt#step:doResolveSecondaryDispatch` | `tasks/target-replay-evidence/L15-RR04A-rule-core-damage-statblock-components.json#L15-RR04A-RULE-CORE-DAMAGE-STATBLOCK-COMPONENTS replay action=doResolveSecondaryDispatch#step:doResolveSecondaryDispatch` | `src/qnt_adapters/rule_core_stat_block_controls_mbt.rs` | `covered` |
| `cleanroom-input/qnt/battle-runtime/rule-core-stat-block-controls.mbt.qnt#step:doStartMultiattack` | `tasks/target-replay-evidence/L15-RR04A-rule-core-damage-statblock-components.json#L15-RR04A-RULE-CORE-DAMAGE-STATBLOCK-COMPONENTS replay action=doStartMultiattack#step:doStartMultiattack` | `src/qnt_adapters/rule_core_stat_block_controls_mbt.rs` | `covered` |

Target replay evidence:

- Evidence file: `tasks/target-replay-evidence/L15-RR04A-rule-core-damage-statblock-components.json`
- Target profile: `rust`
- Target profile SHA-256: `6d4cc6c6a4769962798133d57aff01438fb2b661941f71d1aa8a3333f4b7ecc1`
- Run ledger: `tasks/RUN_LEDGER.json`

Harness artifacts:

- Start gate: `tasks/history/L15-RR04A-RULE-CORE-DAMAGE-STATBLOCK-COMPONENTS/START_GATE.json`
- Engine depth: `tasks/history/L15-RR04A-RULE-CORE-DAMAGE-STATBLOCK-COMPONENTS/ENGINE_DEPTH_MANIFEST.json`
- State ownership: `tasks/history/L15-RR04A-RULE-CORE-DAMAGE-STATBLOCK-COMPONENTS/STATE_OWNER_MANIFEST.json`
- Reviewer loop: `tasks/history/L15-RR04A-RULE-CORE-DAMAGE-STATBLOCK-COMPONENTS/REVIEW_LOOP.json`
- Decider decision: `tasks/history/L15-RR04A-RULE-CORE-DAMAGE-STATBLOCK-COMPONENTS/DECIDER_DECISION.json`

Remaining gaps:

- `_none_`

Verification results:

- `cargo test attack_damage_disposition_adapter_replays_all_branches` passed.
- `cargo test hit_point_damage_adapter_replays_all_branches` passed.
- `cargo test stat_block_controls_adapter_replays_all_branches` passed.
- `cargo fmt --check` passed.
- `cargo test` passed.
- `cargo clippy --all-targets -- -D warnings` passed.
- `node scripts/check-cleanroom-harness.cjs` passed.
- `git diff --check a625d3e7190eb33396c17ee5dca7ae73f413b348...HEAD` passed.

## L15-RR04B-RULE-CORE-MOVEMENT-REACTION-SHOVE-COMPONENTS: Movement, Reaction, and Shove Rule-Core Components

- Manifest source commit SHA: `564376fd95218a209bb9eae5c9ccb54ca3e04a52`
- Source branch inventory SHA: `4bb2b20a85d94e3b90b7c59cbfe6e1edd5ab3ef40410641e999527861f3d3a32`
- Campaign lane id: `L15-RR04B-RULE-CORE-MOVEMENT-REACTION-SHOVE-COMPONENTS`
- Parent route task id: `L15-RR04-RULE-CORE-COMPONENT-CONNECTORS`
- Drivers:
  - `cleanroom-input/qnt/battle-runtime/rule-core-movement.mbt.qnt`
  - `cleanroom-input/qnt/battle-runtime/rule-core-reactions.mbt.qnt`
  - `cleanroom-input/qnt/battle-runtime/rule-core-shove-outcome.mbt.qnt`
- Branch obligations:
  - `cleanroom-input/qnt/battle-runtime/rule-core-movement.mbt.qnt#step:doDash`
  - `cleanroom-input/qnt/battle-runtime/rule-core-movement.mbt.qnt#step:doDeclineOpportunityAttack`
  - `cleanroom-input/qnt/battle-runtime/rule-core-movement.mbt.qnt#step:doDiscoverEscapeGrapple`
  - `cleanroom-input/qnt/battle-runtime/rule-core-movement.mbt.qnt#step:doDiscoverGrapple`
  - `cleanroom-input/qnt/battle-runtime/rule-core-movement.mbt.qnt#step:doDiscoverMovement`
  - `cleanroom-input/qnt/battle-runtime/rule-core-movement.mbt.qnt#step:doDisengage`
  - `cleanroom-input/qnt/battle-runtime/rule-core-movement.mbt.qnt#step:doMoveProvokesOpportunityAttack`
  - `cleanroom-input/qnt/battle-runtime/rule-core-movement.mbt.qnt#step:doMoveThreatSuppressedByDisengage`
  - `cleanroom-input/qnt/battle-runtime/rule-core-movement.mbt.qnt#step:doRejectDashAfterActionSpent`
  - `cleanroom-input/qnt/battle-runtime/rule-core-movement.mbt.qnt#step:doRejectMovementOverspend`
  - `cleanroom-input/qnt/battle-runtime/rule-core-movement.mbt.qnt#step:doReleaseGrapple`
  - `cleanroom-input/qnt/battle-runtime/rule-core-movement.mbt.qnt#step:doResolveEscapeFailure`
  - `cleanroom-input/qnt/battle-runtime/rule-core-movement.mbt.qnt#step:doResolveEscapeSuccess`
  - `cleanroom-input/qnt/battle-runtime/rule-core-movement.mbt.qnt#step:doResolveGrappleFailure`
  - `cleanroom-input/qnt/battle-runtime/rule-core-movement.mbt.qnt#step:doResolveGrappleSuccess`
  - `cleanroom-input/qnt/battle-runtime/rule-core-movement.mbt.qnt#step:doSelectGrappleTarget`
  - `cleanroom-input/qnt/battle-runtime/rule-core-movement.mbt.qnt#step:doSpendFullMovement`
  - `cleanroom-input/qnt/battle-runtime/rule-core-movement.mbt.qnt#step:doSpendMovement`
  - `cleanroom-input/qnt/battle-runtime/rule-core-movement.mbt.qnt#step:doSpendShortMovement`
  - `cleanroom-input/qnt/battle-runtime/rule-core-movement.mbt.qnt#step:doStandFromProne`
  - `cleanroom-input/qnt/battle-runtime/rule-core-movement.mbt.qnt#step:doStartGrappledTargetTurn`
  - `cleanroom-input/qnt/battle-runtime/rule-core-reactions.mbt.qnt#step:doBreakReactorConcentrationAfterLargeDamage`
  - `cleanroom-input/qnt/battle-runtime/rule-core-reactions.mbt.qnt#step:doDeclineOpportunityAttack`
  - `cleanroom-input/qnt/battle-runtime/rule-core-reactions.mbt.qnt#step:doDeclineReadiedMovement`
  - `cleanroom-input/qnt/battle-runtime/rule-core-reactions.mbt.qnt#step:doHoldReactorConcentrationAfterSmallDamage`
  - `cleanroom-input/qnt/battle-runtime/rule-core-reactions.mbt.qnt#step:doOfferOpportunityAttack`
  - `cleanroom-input/qnt/battle-runtime/rule-core-reactions.mbt.qnt#step:doOfferReadiedMovement`
  - `cleanroom-input/qnt/battle-runtime/rule-core-reactions.mbt.qnt#step:doReadyMovementFixture`
  - `cleanroom-input/qnt/battle-runtime/rule-core-reactions.mbt.qnt#step:doRejectReadiedMovementZero`
  - `cleanroom-input/qnt/battle-runtime/rule-core-reactions.mbt.qnt#step:doStartReactorConcentrationFixture`
  - `cleanroom-input/qnt/battle-runtime/rule-core-reactions.mbt.qnt#step:doTakeReadiedMovementFill`
  - `cleanroom-input/qnt/battle-runtime/rule-core-reactions.mbt.qnt#step:doTakeReadiedMovementShort`
  - `cleanroom-input/qnt/battle-runtime/rule-core-shove-outcome.mbt.qnt#step:doInvalidPushDistance`
  - `cleanroom-input/qnt/battle-runtime/rule-core-shove-outcome.mbt.qnt#step:doSaveFailsProne`
  - `cleanroom-input/qnt/battle-runtime/rule-core-shove-outcome.mbt.qnt#step:doSaveFailsPush`
  - `cleanroom-input/qnt/battle-runtime/rule-core-shove-outcome.mbt.qnt#step:doSaveFailsPushBlocked`
  - `cleanroom-input/qnt/battle-runtime/rule-core-shove-outcome.mbt.qnt#step:doSaveFailsPushNoLegalDestination`
  - `cleanroom-input/qnt/battle-runtime/rule-core-shove-outcome.mbt.qnt#step:doSaveSucceeds`

Allowed inputs used:

- `cleanroom-input/MANIFEST.md`
- `cleanroom-input/branch-coverage/source-branch-inventory.json`
- `cleanroom-input/branch-coverage/reducer-route-inventory.json`
- `cleanroom-input/qnt/battle-runtime/rule-core-movement.mbt.qnt`
- `cleanroom-input/qnt/battle-runtime/rule-core-reactions.mbt.qnt`
- `cleanroom-input/qnt/battle-runtime/rule-core-shove-outcome.mbt.qnt`
- `cleanroom-input/qnt/shared-algebras/proofs/rule-core/movement-spatial-grapple.qnt`
- `cleanroom-input/qnt/shared-algebras/proofs/rule-core/reactions-continuations-concentration.qnt`
- `cleanroom-input/qnt/shared-algebras/proofs/rule-core/shove-outcome.qnt`
- `cleanroom-input/domain/UBIQUITOUS_LANGUAGE.md`
- `cleanroom-input/domain/CLEANROOM_ASSUMPTIONS.md`
- `cleanroom-input/raw/srd-5.2.1/Playing-the-Game.md`
- `cleanroom-input/raw/srd-5.2.1/Rules-Glossary.md`
- Repo-local `src/**`, `tasks/**`, and Cargo tooling

Behavior implemented:

- Replayed the movement, reaction, and shove selected rule-core drivers through component owner modules: `src/rules/rule_core_movement.rs`, `src/rules/rule_core_reactions.rs`, and `src/rules/rule_core_shove_outcome.rs`.
- Kept QNT action names and projection hashes in `src/qnt_adapters/**` and `tasks/target-replay-evidence/**`; no production battle reducer owner or authored-identity dispatch was added.
- Recorded component projection ownership for `MovementState`, `RuleCoreReactionState`, and `ShoveOutcomeState` in `tasks/STATE_OWNER_MANIFEST.json`.

Generated branch coverage:

| Obligation | Target replay evidence | Diagnostic tests | Status |
| --- | --- | --- | --- |
| `cleanroom-input/qnt/battle-runtime/rule-core-movement.mbt.qnt#step:doDash` | `tasks/target-replay-evidence/L15-RR04B-rule-core-movement-reaction-shove-components.json#L15-RR04B-RULE-CORE-MOVEMENT-REACTION-SHOVE-COMPONENTS driverPath=cleanroom-input/qnt/battle-runtime/rule-core-movement.mbt.qnt seed=1 action=doDash#step:doDash` | `src/tests/mod.rs` | `covered` |
| `cleanroom-input/qnt/battle-runtime/rule-core-movement.mbt.qnt#step:doDeclineOpportunityAttack` | `tasks/target-replay-evidence/L15-RR04B-rule-core-movement-reaction-shove-components.json#L15-RR04B-RULE-CORE-MOVEMENT-REACTION-SHOVE-COMPONENTS driverPath=cleanroom-input/qnt/battle-runtime/rule-core-movement.mbt.qnt seed=1 action=doDeclineOpportunityAttack#step:doDeclineOpportunityAttack` | `src/tests/mod.rs` | `covered` |
| `cleanroom-input/qnt/battle-runtime/rule-core-movement.mbt.qnt#step:doDiscoverEscapeGrapple` | `tasks/target-replay-evidence/L15-RR04B-rule-core-movement-reaction-shove-components.json#L15-RR04B-RULE-CORE-MOVEMENT-REACTION-SHOVE-COMPONENTS driverPath=cleanroom-input/qnt/battle-runtime/rule-core-movement.mbt.qnt seed=1 action=doDiscoverEscapeGrapple#step:doDiscoverEscapeGrapple` | `src/tests/mod.rs` | `covered` |
| `cleanroom-input/qnt/battle-runtime/rule-core-movement.mbt.qnt#step:doDiscoverGrapple` | `tasks/target-replay-evidence/L15-RR04B-rule-core-movement-reaction-shove-components.json#L15-RR04B-RULE-CORE-MOVEMENT-REACTION-SHOVE-COMPONENTS driverPath=cleanroom-input/qnt/battle-runtime/rule-core-movement.mbt.qnt seed=1 action=doDiscoverGrapple#step:doDiscoverGrapple` | `src/tests/mod.rs` | `covered` |
| `cleanroom-input/qnt/battle-runtime/rule-core-movement.mbt.qnt#step:doDiscoverMovement` | `tasks/target-replay-evidence/L15-RR04B-rule-core-movement-reaction-shove-components.json#L15-RR04B-RULE-CORE-MOVEMENT-REACTION-SHOVE-COMPONENTS driverPath=cleanroom-input/qnt/battle-runtime/rule-core-movement.mbt.qnt seed=1 action=doDiscoverMovement#step:doDiscoverMovement` | `src/tests/mod.rs` | `covered` |
| `cleanroom-input/qnt/battle-runtime/rule-core-movement.mbt.qnt#step:doDisengage` | `tasks/target-replay-evidence/L15-RR04B-rule-core-movement-reaction-shove-components.json#L15-RR04B-RULE-CORE-MOVEMENT-REACTION-SHOVE-COMPONENTS driverPath=cleanroom-input/qnt/battle-runtime/rule-core-movement.mbt.qnt seed=1 action=doDisengage#step:doDisengage` | `src/tests/mod.rs` | `covered` |
| `cleanroom-input/qnt/battle-runtime/rule-core-movement.mbt.qnt#step:doMoveProvokesOpportunityAttack` | `tasks/target-replay-evidence/L15-RR04B-rule-core-movement-reaction-shove-components.json#L15-RR04B-RULE-CORE-MOVEMENT-REACTION-SHOVE-COMPONENTS driverPath=cleanroom-input/qnt/battle-runtime/rule-core-movement.mbt.qnt seed=1 action=doMoveProvokesOpportunityAttack#step:doMoveProvokesOpportunityAttack` | `src/tests/mod.rs` | `covered` |
| `cleanroom-input/qnt/battle-runtime/rule-core-movement.mbt.qnt#step:doMoveThreatSuppressedByDisengage` | `tasks/target-replay-evidence/L15-RR04B-rule-core-movement-reaction-shove-components.json#L15-RR04B-RULE-CORE-MOVEMENT-REACTION-SHOVE-COMPONENTS driverPath=cleanroom-input/qnt/battle-runtime/rule-core-movement.mbt.qnt seed=1 action=doMoveThreatSuppressedByDisengage#step:doMoveThreatSuppressedByDisengage` | `src/tests/mod.rs` | `covered` |
| `cleanroom-input/qnt/battle-runtime/rule-core-movement.mbt.qnt#step:doRejectDashAfterActionSpent` | `tasks/target-replay-evidence/L15-RR04B-rule-core-movement-reaction-shove-components.json#L15-RR04B-RULE-CORE-MOVEMENT-REACTION-SHOVE-COMPONENTS driverPath=cleanroom-input/qnt/battle-runtime/rule-core-movement.mbt.qnt seed=1 action=doRejectDashAfterActionSpent#step:doRejectDashAfterActionSpent` | `src/tests/mod.rs` | `covered` |
| `cleanroom-input/qnt/battle-runtime/rule-core-movement.mbt.qnt#step:doRejectMovementOverspend` | `tasks/target-replay-evidence/L15-RR04B-rule-core-movement-reaction-shove-components.json#L15-RR04B-RULE-CORE-MOVEMENT-REACTION-SHOVE-COMPONENTS driverPath=cleanroom-input/qnt/battle-runtime/rule-core-movement.mbt.qnt seed=1 action=doRejectMovementOverspend#step:doRejectMovementOverspend` | `src/tests/mod.rs` | `covered` |
| `cleanroom-input/qnt/battle-runtime/rule-core-movement.mbt.qnt#step:doReleaseGrapple` | `tasks/target-replay-evidence/L15-RR04B-rule-core-movement-reaction-shove-components.json#L15-RR04B-RULE-CORE-MOVEMENT-REACTION-SHOVE-COMPONENTS driverPath=cleanroom-input/qnt/battle-runtime/rule-core-movement.mbt.qnt seed=1 action=doReleaseGrapple#step:doReleaseGrapple` | `src/tests/mod.rs` | `covered` |
| `cleanroom-input/qnt/battle-runtime/rule-core-movement.mbt.qnt#step:doResolveEscapeFailure` | `tasks/target-replay-evidence/L15-RR04B-rule-core-movement-reaction-shove-components.json#L15-RR04B-RULE-CORE-MOVEMENT-REACTION-SHOVE-COMPONENTS driverPath=cleanroom-input/qnt/battle-runtime/rule-core-movement.mbt.qnt seed=1 action=doResolveEscapeFailure#step:doResolveEscapeFailure` | `src/tests/mod.rs` | `covered` |
| `cleanroom-input/qnt/battle-runtime/rule-core-movement.mbt.qnt#step:doResolveEscapeSuccess` | `tasks/target-replay-evidence/L15-RR04B-rule-core-movement-reaction-shove-components.json#L15-RR04B-RULE-CORE-MOVEMENT-REACTION-SHOVE-COMPONENTS driverPath=cleanroom-input/qnt/battle-runtime/rule-core-movement.mbt.qnt seed=1 action=doResolveEscapeSuccess#step:doResolveEscapeSuccess` | `src/tests/mod.rs` | `covered` |
| `cleanroom-input/qnt/battle-runtime/rule-core-movement.mbt.qnt#step:doResolveGrappleFailure` | `tasks/target-replay-evidence/L15-RR04B-rule-core-movement-reaction-shove-components.json#L15-RR04B-RULE-CORE-MOVEMENT-REACTION-SHOVE-COMPONENTS driverPath=cleanroom-input/qnt/battle-runtime/rule-core-movement.mbt.qnt seed=1 action=doResolveGrappleFailure#step:doResolveGrappleFailure` | `src/tests/mod.rs` | `covered` |
| `cleanroom-input/qnt/battle-runtime/rule-core-movement.mbt.qnt#step:doResolveGrappleSuccess` | `tasks/target-replay-evidence/L15-RR04B-rule-core-movement-reaction-shove-components.json#L15-RR04B-RULE-CORE-MOVEMENT-REACTION-SHOVE-COMPONENTS driverPath=cleanroom-input/qnt/battle-runtime/rule-core-movement.mbt.qnt seed=1 action=doResolveGrappleSuccess#step:doResolveGrappleSuccess` | `src/tests/mod.rs` | `covered` |
| `cleanroom-input/qnt/battle-runtime/rule-core-movement.mbt.qnt#step:doSelectGrappleTarget` | `tasks/target-replay-evidence/L15-RR04B-rule-core-movement-reaction-shove-components.json#L15-RR04B-RULE-CORE-MOVEMENT-REACTION-SHOVE-COMPONENTS driverPath=cleanroom-input/qnt/battle-runtime/rule-core-movement.mbt.qnt seed=1 action=doSelectGrappleTarget#step:doSelectGrappleTarget` | `src/tests/mod.rs` | `covered` |
| `cleanroom-input/qnt/battle-runtime/rule-core-movement.mbt.qnt#step:doSpendFullMovement` | `tasks/target-replay-evidence/L15-RR04B-rule-core-movement-reaction-shove-components.json#L15-RR04B-RULE-CORE-MOVEMENT-REACTION-SHOVE-COMPONENTS driverPath=cleanroom-input/qnt/battle-runtime/rule-core-movement.mbt.qnt seed=1 action=doSpendFullMovement#step:doSpendFullMovement` | `src/tests/mod.rs` | `covered` |
| `cleanroom-input/qnt/battle-runtime/rule-core-movement.mbt.qnt#step:doSpendMovement` | `tasks/target-replay-evidence/L15-RR04B-rule-core-movement-reaction-shove-components.json#L15-RR04B-RULE-CORE-MOVEMENT-REACTION-SHOVE-COMPONENTS driverPath=cleanroom-input/qnt/battle-runtime/rule-core-movement.mbt.qnt seed=1 action=doSpendMovement#step:doSpendMovement` | `src/tests/mod.rs` | `covered` |
| `cleanroom-input/qnt/battle-runtime/rule-core-movement.mbt.qnt#step:doSpendShortMovement` | `tasks/target-replay-evidence/L15-RR04B-rule-core-movement-reaction-shove-components.json#L15-RR04B-RULE-CORE-MOVEMENT-REACTION-SHOVE-COMPONENTS driverPath=cleanroom-input/qnt/battle-runtime/rule-core-movement.mbt.qnt seed=1 action=doSpendShortMovement#step:doSpendShortMovement` | `src/tests/mod.rs` | `covered` |
| `cleanroom-input/qnt/battle-runtime/rule-core-movement.mbt.qnt#step:doStandFromProne` | `tasks/target-replay-evidence/L15-RR04B-rule-core-movement-reaction-shove-components.json#L15-RR04B-RULE-CORE-MOVEMENT-REACTION-SHOVE-COMPONENTS driverPath=cleanroom-input/qnt/battle-runtime/rule-core-movement.mbt.qnt seed=1 action=doStandFromProne#step:doStandFromProne` | `src/tests/mod.rs` | `covered` |
| `cleanroom-input/qnt/battle-runtime/rule-core-movement.mbt.qnt#step:doStartGrappledTargetTurn` | `tasks/target-replay-evidence/L15-RR04B-rule-core-movement-reaction-shove-components.json#L15-RR04B-RULE-CORE-MOVEMENT-REACTION-SHOVE-COMPONENTS driverPath=cleanroom-input/qnt/battle-runtime/rule-core-movement.mbt.qnt seed=1 action=doStartGrappledTargetTurn#step:doStartGrappledTargetTurn` | `src/tests/mod.rs` | `covered` |
| `cleanroom-input/qnt/battle-runtime/rule-core-reactions.mbt.qnt#step:doBreakReactorConcentrationAfterLargeDamage` | `tasks/target-replay-evidence/L15-RR04B-rule-core-movement-reaction-shove-components.json#L15-RR04B-RULE-CORE-MOVEMENT-REACTION-SHOVE-COMPONENTS driverPath=cleanroom-input/qnt/battle-runtime/rule-core-reactions.mbt.qnt seed=1 action=doBreakReactorConcentrationAfterLargeDamage#step:doBreakReactorConcentrationAfterLargeDamage` | `src/tests/mod.rs` | `covered` |
| `cleanroom-input/qnt/battle-runtime/rule-core-reactions.mbt.qnt#step:doDeclineOpportunityAttack` | `tasks/target-replay-evidence/L15-RR04B-rule-core-movement-reaction-shove-components.json#L15-RR04B-RULE-CORE-MOVEMENT-REACTION-SHOVE-COMPONENTS driverPath=cleanroom-input/qnt/battle-runtime/rule-core-reactions.mbt.qnt seed=1 action=doDeclineOpportunityAttack#step:doDeclineOpportunityAttack` | `src/tests/mod.rs` | `covered` |
| `cleanroom-input/qnt/battle-runtime/rule-core-reactions.mbt.qnt#step:doDeclineReadiedMovement` | `tasks/target-replay-evidence/L15-RR04B-rule-core-movement-reaction-shove-components.json#L15-RR04B-RULE-CORE-MOVEMENT-REACTION-SHOVE-COMPONENTS driverPath=cleanroom-input/qnt/battle-runtime/rule-core-reactions.mbt.qnt seed=1 action=doDeclineReadiedMovement#step:doDeclineReadiedMovement` | `src/tests/mod.rs` | `covered` |
| `cleanroom-input/qnt/battle-runtime/rule-core-reactions.mbt.qnt#step:doHoldReactorConcentrationAfterSmallDamage` | `tasks/target-replay-evidence/L15-RR04B-rule-core-movement-reaction-shove-components.json#L15-RR04B-RULE-CORE-MOVEMENT-REACTION-SHOVE-COMPONENTS driverPath=cleanroom-input/qnt/battle-runtime/rule-core-reactions.mbt.qnt seed=1 action=doHoldReactorConcentrationAfterSmallDamage#step:doHoldReactorConcentrationAfterSmallDamage` | `src/tests/mod.rs` | `covered` |
| `cleanroom-input/qnt/battle-runtime/rule-core-reactions.mbt.qnt#step:doOfferOpportunityAttack` | `tasks/target-replay-evidence/L15-RR04B-rule-core-movement-reaction-shove-components.json#L15-RR04B-RULE-CORE-MOVEMENT-REACTION-SHOVE-COMPONENTS driverPath=cleanroom-input/qnt/battle-runtime/rule-core-reactions.mbt.qnt seed=1 action=doOfferOpportunityAttack#step:doOfferOpportunityAttack` | `src/tests/mod.rs` | `covered` |
| `cleanroom-input/qnt/battle-runtime/rule-core-reactions.mbt.qnt#step:doOfferReadiedMovement` | `tasks/target-replay-evidence/L15-RR04B-rule-core-movement-reaction-shove-components.json#L15-RR04B-RULE-CORE-MOVEMENT-REACTION-SHOVE-COMPONENTS driverPath=cleanroom-input/qnt/battle-runtime/rule-core-reactions.mbt.qnt seed=1 action=doOfferReadiedMovement#step:doOfferReadiedMovement` | `src/tests/mod.rs` | `covered` |
| `cleanroom-input/qnt/battle-runtime/rule-core-reactions.mbt.qnt#step:doReadyMovementFixture` | `tasks/target-replay-evidence/L15-RR04B-rule-core-movement-reaction-shove-components.json#L15-RR04B-RULE-CORE-MOVEMENT-REACTION-SHOVE-COMPONENTS driverPath=cleanroom-input/qnt/battle-runtime/rule-core-reactions.mbt.qnt seed=1 action=doReadyMovementFixture#step:doReadyMovementFixture` | `src/tests/mod.rs` | `covered` |
| `cleanroom-input/qnt/battle-runtime/rule-core-reactions.mbt.qnt#step:doRejectReadiedMovementZero` | `tasks/target-replay-evidence/L15-RR04B-rule-core-movement-reaction-shove-components.json#L15-RR04B-RULE-CORE-MOVEMENT-REACTION-SHOVE-COMPONENTS driverPath=cleanroom-input/qnt/battle-runtime/rule-core-reactions.mbt.qnt seed=1 action=doRejectReadiedMovementZero#step:doRejectReadiedMovementZero` | `src/tests/mod.rs` | `covered` |
| `cleanroom-input/qnt/battle-runtime/rule-core-reactions.mbt.qnt#step:doStartReactorConcentrationFixture` | `tasks/target-replay-evidence/L15-RR04B-rule-core-movement-reaction-shove-components.json#L15-RR04B-RULE-CORE-MOVEMENT-REACTION-SHOVE-COMPONENTS driverPath=cleanroom-input/qnt/battle-runtime/rule-core-reactions.mbt.qnt seed=1 action=doStartReactorConcentrationFixture#step:doStartReactorConcentrationFixture` | `src/tests/mod.rs` | `covered` |
| `cleanroom-input/qnt/battle-runtime/rule-core-reactions.mbt.qnt#step:doTakeReadiedMovementFill` | `tasks/target-replay-evidence/L15-RR04B-rule-core-movement-reaction-shove-components.json#L15-RR04B-RULE-CORE-MOVEMENT-REACTION-SHOVE-COMPONENTS driverPath=cleanroom-input/qnt/battle-runtime/rule-core-reactions.mbt.qnt seed=1 action=doTakeReadiedMovementFill#step:doTakeReadiedMovementFill` | `src/tests/mod.rs` | `covered` |
| `cleanroom-input/qnt/battle-runtime/rule-core-reactions.mbt.qnt#step:doTakeReadiedMovementShort` | `tasks/target-replay-evidence/L15-RR04B-rule-core-movement-reaction-shove-components.json#L15-RR04B-RULE-CORE-MOVEMENT-REACTION-SHOVE-COMPONENTS driverPath=cleanroom-input/qnt/battle-runtime/rule-core-reactions.mbt.qnt seed=1 action=doTakeReadiedMovementShort#step:doTakeReadiedMovementShort` | `src/tests/mod.rs` | `covered` |
| `cleanroom-input/qnt/battle-runtime/rule-core-shove-outcome.mbt.qnt#step:doInvalidPushDistance` | `tasks/target-replay-evidence/L15-RR04B-rule-core-movement-reaction-shove-components.json#L15-RR04B-RULE-CORE-MOVEMENT-REACTION-SHOVE-COMPONENTS driverPath=cleanroom-input/qnt/battle-runtime/rule-core-shove-outcome.mbt.qnt seed=1 action=doInvalidPushDistance#step:doInvalidPushDistance` | `src/tests/mod.rs` | `covered` |
| `cleanroom-input/qnt/battle-runtime/rule-core-shove-outcome.mbt.qnt#step:doSaveFailsProne` | `tasks/target-replay-evidence/L15-RR04B-rule-core-movement-reaction-shove-components.json#L15-RR04B-RULE-CORE-MOVEMENT-REACTION-SHOVE-COMPONENTS driverPath=cleanroom-input/qnt/battle-runtime/rule-core-shove-outcome.mbt.qnt seed=1 action=doSaveFailsProne#step:doSaveFailsProne` | `src/tests/mod.rs` | `covered` |
| `cleanroom-input/qnt/battle-runtime/rule-core-shove-outcome.mbt.qnt#step:doSaveFailsPush` | `tasks/target-replay-evidence/L15-RR04B-rule-core-movement-reaction-shove-components.json#L15-RR04B-RULE-CORE-MOVEMENT-REACTION-SHOVE-COMPONENTS driverPath=cleanroom-input/qnt/battle-runtime/rule-core-shove-outcome.mbt.qnt seed=1 action=doSaveFailsPush#step:doSaveFailsPush` | `src/tests/mod.rs` | `covered` |
| `cleanroom-input/qnt/battle-runtime/rule-core-shove-outcome.mbt.qnt#step:doSaveFailsPushBlocked` | `tasks/target-replay-evidence/L15-RR04B-rule-core-movement-reaction-shove-components.json#L15-RR04B-RULE-CORE-MOVEMENT-REACTION-SHOVE-COMPONENTS driverPath=cleanroom-input/qnt/battle-runtime/rule-core-shove-outcome.mbt.qnt seed=1 action=doSaveFailsPushBlocked#step:doSaveFailsPushBlocked` | `src/tests/mod.rs` | `covered` |
| `cleanroom-input/qnt/battle-runtime/rule-core-shove-outcome.mbt.qnt#step:doSaveFailsPushNoLegalDestination` | `tasks/target-replay-evidence/L15-RR04B-rule-core-movement-reaction-shove-components.json#L15-RR04B-RULE-CORE-MOVEMENT-REACTION-SHOVE-COMPONENTS driverPath=cleanroom-input/qnt/battle-runtime/rule-core-shove-outcome.mbt.qnt seed=1 action=doSaveFailsPushNoLegalDestination#step:doSaveFailsPushNoLegalDestination` | `src/tests/mod.rs` | `covered` |
| `cleanroom-input/qnt/battle-runtime/rule-core-shove-outcome.mbt.qnt#step:doSaveSucceeds` | `tasks/target-replay-evidence/L15-RR04B-rule-core-movement-reaction-shove-components.json#L15-RR04B-RULE-CORE-MOVEMENT-REACTION-SHOVE-COMPONENTS driverPath=cleanroom-input/qnt/battle-runtime/rule-core-shove-outcome.mbt.qnt seed=1 action=doSaveSucceeds#step:doSaveSucceeds` | `src/tests/mod.rs` | `covered` |

Target replay evidence:

- Evidence file: `tasks/target-replay-evidence/L15-RR04B-rule-core-movement-reaction-shove-components.json`
- Target profile: `rust`
- Target profile SHA-256: `6d4cc6c6a4769962798133d57aff01438fb2b661941f71d1aa8a3333f4b7ecc1`
- Quint binding: Rust quint-connect harness
- Reproduction seed or trace id: `L15-RR04B-RULE-CORE-MOVEMENT-REACTION-SHOVE-COMPONENTS seed=1`

Harness artifacts:

- Start gate: `tasks/history/L15-RR04B-RULE-CORE-MOVEMENT-REACTION-SHOVE-COMPONENTS/START_GATE.json`
- Engine depth: `tasks/history/L15-RR04B-RULE-CORE-MOVEMENT-REACTION-SHOVE-COMPONENTS/ENGINE_DEPTH_MANIFEST.json`
- State ownership: `tasks/history/L15-RR04B-RULE-CORE-MOVEMENT-REACTION-SHOVE-COMPONENTS/STATE_OWNER_MANIFEST.json`
- Reviewer loop: `tasks/history/L15-RR04B-RULE-CORE-MOVEMENT-REACTION-SHOVE-COMPONENTS/REVIEW_LOOP.json`
- Decider decision: `tasks/history/L15-RR04B-RULE-CORE-MOVEMENT-REACTION-SHOVE-COMPONENTS/DECIDER_DECISION.json`
- Run ledger: `tasks/RUN_LEDGER.json`

Remaining gaps:

- `_none_`

Verification results:

- `cargo test adapter_replays_all_branches` passed.
- `cargo fmt --check` passed.
- `cargo test` passed.
- `cargo clippy --all-targets -- -D warnings` passed.
- `node scripts/check-cleanroom-harness.cjs` passed.
- `git diff --check a625d3e7190eb33396c17ee5dca7ae73f413b348...HEAD` passed.

## L15-RR04D-RULE-CORE-FEATURE-PROFILE-COMPONENTS

- Manifest source commit SHA: `564376fd95218a209bb9eae5c9ccb54ca3e04a52`
- Source branch inventory SHA: `4bb2b20a85d94e3b90b7c59cbfe6e1edd5ab3ef40410641e999527861f3d3a32`
- Selected driver: `cleanroom-input/qnt/battle-runtime/rule-core-features.mbt.qnt`
- Allowed inputs used: copied QNT driver, copied rule-core QNT slices, copied SRD 5.2.1 RAW, UBIQUITOUS_LANGUAGE.md, reducer-route inventory, and local Cargo harness.
- Behavior implemented: adapter-only component-first feature profile replay of the copied QNT literal `qComponentRoute` through `RuleCoreFeatureProfileSemanticsOwner`; no battle-state owner, production component state, or authored feature identity production dispatch added.
- Generated branch coverage: 22 selected obligations covered for the `qComponentRoute` projection only; this is dirty rehearsal evidence for the component-owner projection, not a claim that production reducer-shaped feature component semantics were added.
- Target replay evidence: `tasks/target-replay-evidence/L15-RR04D-rule-core-feature-profile-components.json`
- Harness artifacts: `tasks/START_GATE.json`, `tasks/ENGINE_DEPTH_MANIFEST.json`, `tasks/STATE_OWNER_MANIFEST.json`, `tasks/REVIEW_LOOP.json`, `tasks/DECIDER_DECISION.json`, and `tasks/RUN_LEDGER.json`.
- Remaining gaps: Mycelium Step remains source-blocked in the copied corpus; Evasion and Boon of Combat Prowess remain outside this lane denominator; `doUncannyDodge` is out of scope for this lane because Rogue Uncanny Dodge is a level-5 feature assigned to `L15-RR12-LEVEL5-SCOPE-PROMOTION` in the copied inventory.
- Verification results: focused adapter test, cargo fmt, cargo test, cargo clippy, cleanroom harness, and diff whitespace checks recorded in `tasks/RUN_LEDGER.json`.

| Obligation | Evidence | Harness | Status |
|---|---|---|---|
| `cleanroom-input/qnt/battle-runtime/rule-core-features.mbt.qnt#step:doActionSurgeActivate` | `tasks/target-replay-evidence/L15-RR04D-rule-core-feature-profile-components.json#L15-RR04D-RULE-CORE-FEATURE-PROFILE-COMPONENTS replay action=doActionSurgeActivate#step:doActionSurgeActivate` | `src/qnt_adapters/rule_core_features_mbt.rs` | `covered` |
| `cleanroom-input/qnt/battle-runtime/rule-core-features.mbt.qnt#step:doActionSurgeRejectTwice` | `tasks/target-replay-evidence/L15-RR04D-rule-core-feature-profile-components.json#L15-RR04D-RULE-CORE-FEATURE-PROFILE-COMPONENTS replay action=doActionSurgeRejectTwice#step:doActionSurgeRejectTwice` | `src/qnt_adapters/rule_core_features_mbt.rs` | `covered` |
| `cleanroom-input/qnt/battle-runtime/rule-core-features.mbt.qnt#step:doActionSurgeSpendAttack` | `tasks/target-replay-evidence/L15-RR04D-rule-core-feature-profile-components.json#L15-RR04D-RULE-CORE-FEATURE-PROFILE-COMPONENTS replay action=doActionSurgeSpendAttack#step:doActionSurgeSpendAttack` | `src/qnt_adapters/rule_core_features_mbt.rs` | `covered` |
| `cleanroom-input/qnt/battle-runtime/rule-core-features.mbt.qnt#step:doArcheryAttackRollBonus` | `tasks/target-replay-evidence/L15-RR04D-rule-core-feature-profile-components.json#L15-RR04D-RULE-CORE-FEATURE-PROFILE-COMPONENTS replay action=doArcheryAttackRollBonus#step:doArcheryAttackRollBonus` | `src/qnt_adapters/rule_core_features_mbt.rs` | `covered` |
| `cleanroom-input/qnt/battle-runtime/rule-core-features.mbt.qnt#step:doCunningDash` | `tasks/target-replay-evidence/L15-RR04D-rule-core-feature-profile-components.json#L15-RR04D-RULE-CORE-FEATURE-PROFILE-COMPONENTS replay action=doCunningDash#step:doCunningDash` | `src/qnt_adapters/rule_core_features_mbt.rs` | `covered` |
| `cleanroom-input/qnt/battle-runtime/rule-core-features.mbt.qnt#step:doCunningDisengage` | `tasks/target-replay-evidence/L15-RR04D-rule-core-feature-profile-components.json#L15-RR04D-RULE-CORE-FEATURE-PROFILE-COMPONENTS replay action=doCunningDisengage#step:doCunningDisengage` | `src/qnt_adapters/rule_core_features_mbt.rs` | `covered` |
| `cleanroom-input/qnt/battle-runtime/rule-core-features.mbt.qnt#step:doCunningHide` | `tasks/target-replay-evidence/L15-RR04D-rule-core-feature-profile-components.json#L15-RR04D-RULE-CORE-FEATURE-PROFILE-COMPONENTS replay action=doCunningHide#step:doCunningHide` | `src/qnt_adapters/rule_core_features_mbt.rs` | `covered` |
| `cleanroom-input/qnt/battle-runtime/rule-core-features.mbt.qnt#step:doCuttingWordsDamage` | `tasks/target-replay-evidence/L15-RR04D-rule-core-feature-profile-components.json#L15-RR04D-RULE-CORE-FEATURE-PROFILE-COMPONENTS replay action=doCuttingWordsDamage#step:doCuttingWordsDamage` | `src/qnt_adapters/rule_core_features_mbt.rs` | `covered` |
| `cleanroom-input/qnt/battle-runtime/rule-core-features.mbt.qnt#step:doDefenseArmorClass` | `tasks/target-replay-evidence/L15-RR04D-rule-core-feature-profile-components.json#L15-RR04D-RULE-CORE-FEATURE-PROFILE-COMPONENTS replay action=doDefenseArmorClass#step:doDefenseArmorClass` | `src/qnt_adapters/rule_core_features_mbt.rs` | `covered` |
| `cleanroom-input/qnt/battle-runtime/rule-core-features.mbt.qnt#step:doDeflectAttacksDamageReduction` | `tasks/target-replay-evidence/L15-RR04D-rule-core-feature-profile-components.json#L15-RR04D-RULE-CORE-FEATURE-PROFILE-COMPONENTS replay action=doDeflectAttacksDamageReduction#step:doDeflectAttacksDamageReduction` | `src/qnt_adapters/rule_core_features_mbt.rs` | `covered` |
| `cleanroom-input/qnt/battle-runtime/rule-core-features.mbt.qnt#step:doDiscoverSecondWind` | `tasks/target-replay-evidence/L15-RR04D-rule-core-feature-profile-components.json#L15-RR04D-RULE-CORE-FEATURE-PROFILE-COMPONENTS replay action=doDiscoverSecondWind#step:doDiscoverSecondWind` | `src/qnt_adapters/rule_core_features_mbt.rs` | `covered` |
| `cleanroom-input/qnt/battle-runtime/rule-core-features.mbt.qnt#step:doFrenzy` | `tasks/target-replay-evidence/L15-RR04D-rule-core-feature-profile-components.json#L15-RR04D-RULE-CORE-FEATURE-PROFILE-COMPONENTS replay action=doFrenzy#step:doFrenzy` | `src/qnt_adapters/rule_core_features_mbt.rs` | `covered` |
| `cleanroom-input/qnt/battle-runtime/rule-core-features.mbt.qnt#step:doImprovedCritical` | `tasks/target-replay-evidence/L15-RR04D-rule-core-feature-profile-components.json#L15-RR04D-RULE-CORE-FEATURE-PROFILE-COMPONENTS replay action=doImprovedCritical#step:doImprovedCritical` | `src/qnt_adapters/rule_core_features_mbt.rs` | `covered` |
| `cleanroom-input/qnt/battle-runtime/rule-core-features.mbt.qnt#step:doRageActivateAndDamage` | `tasks/target-replay-evidence/L15-RR04D-rule-core-feature-profile-components.json#L15-RR04D-RULE-CORE-FEATURE-PROFILE-COMPONENTS replay action=doRageActivateAndDamage#step:doRageActivateAndDamage` | `src/qnt_adapters/rule_core_features_mbt.rs` | `covered` |
| `cleanroom-input/qnt/battle-runtime/rule-core-features.mbt.qnt#step:doRecklessAttack` | `tasks/target-replay-evidence/L15-RR04D-rule-core-feature-profile-components.json#L15-RR04D-RULE-CORE-FEATURE-PROFILE-COMPONENTS replay action=doRecklessAttack#step:doRecklessAttack` | `src/qnt_adapters/rule_core_features_mbt.rs` | `covered` |
| `cleanroom-input/qnt/battle-runtime/rule-core-features.mbt.qnt#step:doResolveSecondWindHigh` | `tasks/target-replay-evidence/L15-RR04D-rule-core-feature-profile-components.json#L15-RR04D-RULE-CORE-FEATURE-PROFILE-COMPONENTS replay action=doResolveSecondWindHigh#step:doResolveSecondWindHigh` | `src/qnt_adapters/rule_core_features_mbt.rs` | `covered` |
| `cleanroom-input/qnt/battle-runtime/rule-core-features.mbt.qnt#step:doResolveSecondWindLow` | `tasks/target-replay-evidence/L15-RR04D-rule-core-feature-profile-components.json#L15-RR04D-RULE-CORE-FEATURE-PROFILE-COMPONENTS replay action=doResolveSecondWindLow#step:doResolveSecondWindLow` | `src/qnt_adapters/rule_core_features_mbt.rs` | `covered` |
| `cleanroom-input/qnt/battle-runtime/rule-core-features.mbt.qnt#step:doSavageAttackerDamage` | `tasks/target-replay-evidence/L15-RR04D-rule-core-feature-profile-components.json#L15-RR04D-RULE-CORE-FEATURE-PROFILE-COMPONENTS replay action=doSavageAttackerDamage#step:doSavageAttackerDamage` | `src/qnt_adapters/rule_core_features_mbt.rs` | `covered` |
| `cleanroom-input/qnt/battle-runtime/rule-core-features.mbt.qnt#step:doSneakAttack` | `tasks/target-replay-evidence/L15-RR04D-rule-core-feature-profile-components.json#L15-RR04D-RULE-CORE-FEATURE-PROFILE-COMPONENTS replay action=doSneakAttack#step:doSneakAttack` | `src/qnt_adapters/rule_core_features_mbt.rs` | `covered` |
| `cleanroom-input/qnt/battle-runtime/rule-core-features.mbt.qnt#step:doTacticalMindConvertedSuccess` | `tasks/target-replay-evidence/L15-RR04D-rule-core-feature-profile-components.json#L15-RR04D-RULE-CORE-FEATURE-PROFILE-COMPONENTS replay action=doTacticalMindConvertedSuccess#step:doTacticalMindConvertedSuccess` | `src/qnt_adapters/rule_core_features_mbt.rs` | `covered` |
| `cleanroom-input/qnt/battle-runtime/rule-core-features.mbt.qnt#step:doTacticalMindStillFailed` | `tasks/target-replay-evidence/L15-RR04D-rule-core-feature-profile-components.json#L15-RR04D-RULE-CORE-FEATURE-PROFILE-COMPONENTS replay action=doTacticalMindStillFailed#step:doTacticalMindStillFailed` | `src/qnt_adapters/rule_core_features_mbt.rs` | `covered` |
| `cleanroom-input/qnt/battle-runtime/rule-core-features.mbt.qnt#step:doZeroHitPointReplacement` | `tasks/target-replay-evidence/L15-RR04D-rule-core-feature-profile-components.json#L15-RR04D-RULE-CORE-FEATURE-PROFILE-COMPONENTS replay action=doZeroHitPointReplacement#step:doZeroHitPointReplacement` | `src/qnt_adapters/rule_core_features_mbt.rs` | `covered` |

## L15-RR04C-RULE-CORE-SPELL-ABILITY-COMPONENTS: Cumulative Ability and Spell Component Evidence

# Cleanroom Branch Coverage

Machine-readable run ledger: `tasks/RUN_LEDGER.json`

- Manifest source commit SHA: `564376fd95218a209bb9eae5c9ccb54ca3e04a52`
- Source branch inventory SHA: `4bb2b20a85d94e3b90b7c59cbfe6e1edd5ab3ef40410641e999527861f3d3a32`

Allowed inputs used:

- Cleanroom worktree files under `cleanroom-input/**`, `tasks/**`, `src/**`, repo-local scripts, and Cargo tooling.
- Selected drivers and accepted replay refs are taken from `tasks/RUN_LEDGER.json`.

Behavior implemented:

- Prior accepted ledger tasks remain recorded as cumulative branch coverage.
- L15-RR04C-RULE-CORE-SPELL-ABILITY-COMPONENTS: ability, skill, Search, Enhance Ability, and Command component replay checks `RuleCoreAbilitySkillCommandOwner` via shared route facts plus full adapter projection payload hashes.
- L15-RR04C-RULE-CORE-SPELL-ABILITY-COMPONENTS: accepted in-scope spell invocation, direct damage, attack/save damage, healing, Mage Armor, and readied-spell component replay checks `RuleCoreSpellProcedureProfileOwner` via shared route facts plus full adapter projection payload hashes; mass spell rows are non-acceptance regression coverage.
- L15-RR04C-RULE-CORE-SPELL-ABILITY-COMPONENTS: no durable production battle-state fields were added for this component-first lane.

Selected drivers by ledger task:

- T001: cleanroom-input/qnt/battle-runtime/battle-runtime-magic-missile.mbt.qnt
- T002: cleanroom-input/qnt/battle-runtime/battle-runtime-save-gated-spell-ordering.mbt.qnt
- T003: cleanroom-input/qnt/battle-runtime/battle-runtime-hit-point-restoration-ordering.mbt.qnt
- T004: cleanroom-input/qnt/battle-runtime/battle-runtime-death-saving-throw.mbt.qnt
- T005: cleanroom-input/qnt/battle-runtime/battle-runtime-concentration-break-teardown.mbt.qnt
- RRCONV-19A-RUST-BATTLE-SETUP-ENTRYPOINT: cleanroom-input/qnt/battle-runtime/battle-runtime-weapon-attack-skeleton.mbt.qnt
- RRCONV-19B-RUST-ACT-DISCOVERY-CONTRACT: cleanroom-input/qnt/battle-runtime/battle-runtime-weapon-attack-skeleton.mbt.qnt
- RRCONV-19D-RUST-TURN-ADVANCE-RESULT: cleanroom-input/qnt/battle-runtime/battle-runtime-reducer-spine-contract.mbt.qnt
- RRCONV-19C-RUST-RESOLUTION-RESULT-CONTRACT: cleanroom-input/qnt/battle-runtime/battle-runtime-reducer-spine-contract.mbt.qnt
- RRCONV-19E-RUST-END-TURN-SUBJECT-RESOLUTION: cleanroom-input/qnt/battle-runtime/battle-runtime-reducer-spine-contract.mbt.qnt
- RRCONV-19F-RUST-ROUTE-EVENT-FROM-REDUCER-RESULT: cleanroom-input/qnt/battle-runtime/battle-runtime-magic-missile.mbt.qnt, cleanroom-input/qnt/battle-runtime/battle-runtime-save-gated-spell-ordering.mbt.qnt
- RRCONV-19G-RUST-SUBJECT-CONTINUATION-LIFECYCLE: cleanroom-input/qnt/battle-runtime/battle-runtime-save-gated-spell-ordering.mbt.qnt, cleanroom-input/qnt/battle-runtime/battle-runtime-hit-point-restoration-ordering.mbt.qnt
- L15-RR03-FINISH-CURRENT-DIAGNOSTIC-QUEUE: cleanroom-input/qnt/battle-runtime/battle-runtime-concentration-break-teardown.mbt.qnt, cleanroom-input/qnt/battle-runtime/battle-runtime-death-saving-throw.mbt.qnt
- L15-RR05-BATTLE-ACTION-ATTACK-STATBLOCK-ROUTES: cleanroom-input/qnt/battle-runtime/battle-runtime-spell-attack-ordering.mbt.qnt, cleanroom-input/qnt/battle-runtime/battle-runtime-stat-block-action-ordering.mbt.qnt, cleanroom-input/qnt/battle-runtime/battle-runtime-stat-block-multi-damage.mbt.qnt, cleanroom-input/qnt/battle-runtime/battle-runtime-stat-block-size-gated-condition-rider.mbt.qnt, cleanroom-input/qnt/battle-runtime/battle-runtime-weapon-attack-ordering.mbt.qnt, cleanroom-input/qnt/battle-runtime/battle-runtime-weapon-attack-skeleton.mbt.qnt, cleanroom-input/qnt/battle-runtime/creature-attack.mbt.qnt
- L15-RR06-BATTLE-SPELL-EFFECT-ROUTES: cleanroom-input/qnt/battle-runtime/battle-runtime-command-option-next-turn.mbt.qnt, cleanroom-input/qnt/battle-runtime/battle-runtime-command-ordering.mbt.qnt, cleanroom-input/qnt/battle-runtime/battle-runtime-scalar-buff.mbt.qnt
- L15-RR04C-RULE-CORE-SPELL-ABILITY-COMPONENTS: cleanroom-input/qnt/battle-runtime/rule-core-ability-skill-command.mbt.qnt, cleanroom-input/qnt/battle-runtime/rule-core-spells.mbt.qnt

Generated branch coverage:

| Obligation | Target replay evidence | Diagnostic tests | Status |
| --- | --- | --- | --- |
| `cleanroom-input/qnt/battle-runtime/battle-runtime-magic-missile.mbt.qnt#step:doFillMagicMissileAllocation` | `tasks/target-replay-evidence/T001-battle-runtime-magic-missile.json#T001 seed=1 action=doFillMagicMissileAllocation#step:doFillMagicMissileAllocation` | `src/qnt_adapters/battle_runtime_magic_missile.rs` | `covered` |
| `cleanroom-input/qnt/battle-runtime/battle-runtime-magic-missile.mbt.qnt#step:doFillMagicMissileDamage` | `tasks/target-replay-evidence/T001-battle-runtime-magic-missile.json#T001 seed=1 action=doFillMagicMissileDamage#step:doFillMagicMissileDamage` | `src/qnt_adapters/battle_runtime_magic_missile.rs` | `covered` |
| `cleanroom-input/qnt/battle-runtime/battle-runtime-save-gated-spell-ordering.mbt.qnt#step:doDiscoverAreaSaveDamage` | `tasks/target-replay-evidence/T002-battle-runtime-save-gated-spell-ordering.json#T002 seed=1 action=doDiscoverAreaSaveDamage#step:doDiscoverAreaSaveDamage` | `src/qnt_adapters/battle_runtime_save_gated_spell_ordering.rs` | `covered` |
| `cleanroom-input/qnt/battle-runtime/battle-runtime-save-gated-spell-ordering.mbt.qnt#step:doDiscoverTargetListConditionChoice` | `tasks/target-replay-evidence/T002-battle-runtime-save-gated-spell-ordering.json#T002 seed=1 action=doDiscoverTargetListConditionChoice#step:doDiscoverTargetListConditionChoice` | `src/qnt_adapters/battle_runtime_save_gated_spell_ordering.rs` | `covered` |
| `cleanroom-input/qnt/battle-runtime/battle-runtime-save-gated-spell-ordering.mbt.qnt#step:doFillAreaDamageDice` | `tasks/target-replay-evidence/T002-battle-runtime-save-gated-spell-ordering.json#T002 seed=1 action=doFillAreaDamageDice#step:doFillAreaDamageDice` | `src/qnt_adapters/battle_runtime_save_gated_spell_ordering.rs` | `covered` |
| `cleanroom-input/qnt/battle-runtime/battle-runtime-save-gated-spell-ordering.mbt.qnt#step:doFillAreaSaveFailed` | `tasks/target-replay-evidence/T002-battle-runtime-save-gated-spell-ordering.json#T002 seed=1 action=doFillAreaSaveFailed#step:doFillAreaSaveFailed` | `src/qnt_adapters/battle_runtime_save_gated_spell_ordering.rs` | `covered` |
| `cleanroom-input/qnt/battle-runtime/battle-runtime-save-gated-spell-ordering.mbt.qnt#step:doFillConditionChoiceAfterTargetList` | `tasks/target-replay-evidence/T002-battle-runtime-save-gated-spell-ordering.json#T002 seed=1 action=doFillConditionChoiceAfterTargetList#step:doFillConditionChoiceAfterTargetList` | `src/qnt_adapters/battle_runtime_save_gated_spell_ordering.rs` | `covered` |
| `cleanroom-input/qnt/battle-runtime/battle-runtime-save-gated-spell-ordering.mbt.qnt#step:doFillConditionChoiceBeforeTargetList` | `tasks/target-replay-evidence/T002-battle-runtime-save-gated-spell-ordering.json#T002 seed=1 action=doFillConditionChoiceBeforeTargetList#step:doFillConditionChoiceBeforeTargetList` | `src/qnt_adapters/battle_runtime_save_gated_spell_ordering.rs` | `covered` |
| `cleanroom-input/qnt/battle-runtime/battle-runtime-save-gated-spell-ordering.mbt.qnt#step:doFillConditionSavingThrow` | `tasks/target-replay-evidence/T002-battle-runtime-save-gated-spell-ordering.json#T002 seed=1 action=doFillConditionSavingThrow#step:doFillConditionSavingThrow` | `src/qnt_adapters/battle_runtime_save_gated_spell_ordering.rs` | `covered` |
| `cleanroom-input/qnt/battle-runtime/battle-runtime-save-gated-spell-ordering.mbt.qnt#step:doFillTargetListAfterConditionChoice` | `tasks/target-replay-evidence/T002-battle-runtime-save-gated-spell-ordering.json#T002 seed=1 action=doFillTargetListAfterConditionChoice#step:doFillTargetListAfterConditionChoice` | `src/qnt_adapters/battle_runtime_save_gated_spell_ordering.rs` | `covered` |
| `cleanroom-input/qnt/battle-runtime/battle-runtime-save-gated-spell-ordering.mbt.qnt#step:doFillTargetListBeforeConditionChoice` | `tasks/target-replay-evidence/T002-battle-runtime-save-gated-spell-ordering.json#T002 seed=1 action=doFillTargetListBeforeConditionChoice#step:doFillTargetListBeforeConditionChoice` | `src/qnt_adapters/battle_runtime_save_gated_spell_ordering.rs` | `covered` |
| `cleanroom-input/qnt/battle-runtime/battle-runtime-save-gated-spell-ordering.mbt.qnt#step:doSubmitDamageBeforeSavingThrow` | `tasks/target-replay-evidence/T002-battle-runtime-save-gated-spell-ordering.json#T002 seed=1 action=doSubmitDamageBeforeSavingThrow#step:doSubmitDamageBeforeSavingThrow` | `src/qnt_adapters/battle_runtime_save_gated_spell_ordering.rs` | `covered` |
| `cleanroom-input/qnt/battle-runtime/battle-runtime-hit-point-restoration-ordering.mbt.qnt#step:doDiscoverFeatureHealingPool` | `tasks/target-replay-evidence/T003-battle-runtime-hit-point-restoration-ordering.json#T003 seed=1 action=doDiscoverFeatureHealingPool#step:doDiscoverFeatureHealingPool` | `src/qnt_adapters/battle_runtime_hit_point_restoration_ordering.rs` | `covered` |
| `cleanroom-input/qnt/battle-runtime/battle-runtime-hit-point-restoration-ordering.mbt.qnt#step:doDiscoverSingleTargetSpellHealing` | `tasks/target-replay-evidence/T003-battle-runtime-hit-point-restoration-ordering.json#T003 seed=1 action=doDiscoverSingleTargetSpellHealing#step:doDiscoverSingleTargetSpellHealing` | `src/qnt_adapters/battle_runtime_hit_point_restoration_ordering.rs` | `covered` |
| `cleanroom-input/qnt/battle-runtime/battle-runtime-hit-point-restoration-ordering.mbt.qnt#step:doDiscoverTargetListSpellHealing` | `tasks/target-replay-evidence/T003-battle-runtime-hit-point-restoration-ordering.json#T003 seed=1 action=doDiscoverTargetListSpellHealing#step:doDiscoverTargetListSpellHealing` | `src/qnt_adapters/battle_runtime_hit_point_restoration_ordering.rs` | `covered` |
| `cleanroom-input/qnt/battle-runtime/battle-runtime-hit-point-restoration-ordering.mbt.qnt#step:doFillFeatureHealingDistribution` | `tasks/target-replay-evidence/T003-battle-runtime-hit-point-restoration-ordering.json#T003 seed=1 action=doFillFeatureHealingDistribution#step:doFillFeatureHealingDistribution` | `src/qnt_adapters/battle_runtime_hit_point_restoration_ordering.rs` | `covered` |
| `cleanroom-input/qnt/battle-runtime/battle-runtime-hit-point-restoration-ordering.mbt.qnt#step:doFillSpellHealingRoll` | `tasks/target-replay-evidence/T003-battle-runtime-hit-point-restoration-ordering.json#T003 seed=1 action=doFillSpellHealingRoll#step:doFillSpellHealingRoll` | `src/qnt_adapters/battle_runtime_hit_point_restoration_ordering.rs` | `covered` |
| `cleanroom-input/qnt/battle-runtime/battle-runtime-hit-point-restoration-ordering.mbt.qnt#step:doFillSpellHealingTargetChoice` | `tasks/target-replay-evidence/T003-battle-runtime-hit-point-restoration-ordering.json#T003 seed=1 action=doFillSpellHealingTargetChoice#step:doFillSpellHealingTargetChoice` | `src/qnt_adapters/battle_runtime_hit_point_restoration_ordering.rs` | `covered` |
| `cleanroom-input/qnt/battle-runtime/battle-runtime-hit-point-restoration-ordering.mbt.qnt#step:doFillSpellHealingTargetList` | `tasks/target-replay-evidence/T003-battle-runtime-hit-point-restoration-ordering.json#T003 seed=1 action=doFillSpellHealingTargetList#step:doFillSpellHealingTargetList` | `src/qnt_adapters/battle_runtime_hit_point_restoration_ordering.rs` | `covered` |
| `cleanroom-input/qnt/battle-runtime/battle-runtime-hit-point-restoration-ordering.mbt.qnt#step:doSubmitHealingRollBeforeTargetChoice` | `tasks/target-replay-evidence/T003-battle-runtime-hit-point-restoration-ordering.json#T003 seed=1 action=doSubmitHealingRollBeforeTargetChoice#step:doSubmitHealingRollBeforeTargetChoice` | `src/qnt_adapters/battle_runtime_hit_point_restoration_ordering.rs` | `covered` |
| `cleanroom-input/qnt/battle-runtime/battle-runtime-hit-point-restoration-ordering.mbt.qnt#step:doSubmitHealingRollBeforeTargetList` | `tasks/target-replay-evidence/T003-battle-runtime-hit-point-restoration-ordering.json#T003 seed=1 action=doSubmitHealingRollBeforeTargetList#step:doSubmitHealingRollBeforeTargetList` | `src/qnt_adapters/battle_runtime_hit_point_restoration_ordering.rs` | `covered` |
| `cleanroom-input/qnt/battle-runtime/battle-runtime-death-saving-throw.mbt.qnt#step:doDiscoverEndTurnDeathSavingThrow` | `tasks/target-replay-evidence/T004-battle-runtime-death-saving-throw.json#T004 seed=1 action=doDiscoverEndTurnDeathSavingThrow#step:doDiscoverEndTurnDeathSavingThrow` | `src/qnt_adapters/battle_runtime_death_saving_throw.rs` | `covered` |
| `cleanroom-input/qnt/battle-runtime/battle-runtime-death-saving-throw.mbt.qnt#step:doFillDeathSavingThrow` | `tasks/target-replay-evidence/T004-battle-runtime-death-saving-throw.json#T004 seed=1 action=doFillDeathSavingThrow#step:doFillDeathSavingThrow` | `src/qnt_adapters/battle_runtime_death_saving_throw.rs` | `covered` |
| `cleanroom-input/qnt/battle-runtime/battle-runtime-death-saving-throw.mbt.qnt#step:doRejectWrongActorEndTurnAfterResolved` | `tasks/target-replay-evidence/T004-battle-runtime-death-saving-throw.json#T004 seed=1 action=doRejectWrongActorEndTurnAfterResolved#step:doRejectWrongActorEndTurnAfterResolved` | `src/qnt_adapters/battle_runtime_death_saving_throw.rs` | `covered` |
| `cleanroom-input/qnt/battle-runtime/battle-runtime-concentration-break-teardown.mbt.qnt#step:doCastConcentrationSpell` | `tasks/target-replay-evidence/T005-battle-runtime-concentration-break-teardown.json#T005 seed=1 action=doCastConcentrationSpell#step:doCastConcentrationSpell` | `src/qnt_adapters/battle_runtime_concentration_break_teardown.rs` | `covered` |
| `cleanroom-input/qnt/battle-runtime/battle-runtime-concentration-break-teardown.mbt.qnt#step:doCastReplacementConcentrationSpell` | `tasks/target-replay-evidence/T005-battle-runtime-concentration-break-teardown.json#T005 seed=1 action=doCastReplacementConcentrationSpell#step:doCastReplacementConcentrationSpell` | `src/qnt_adapters/battle_runtime_concentration_break_teardown.rs` | `covered` |
| `cleanroom-input/qnt/battle-runtime/battle-runtime-concentration-break-teardown.mbt.qnt#step:doDamageRequestsConcentrationSave` | `tasks/target-replay-evidence/T005-battle-runtime-concentration-break-teardown.json#T005 seed=1 action=doDamageRequestsConcentrationSave#step:doDamageRequestsConcentrationSave` | `src/qnt_adapters/battle_runtime_concentration_break_teardown.rs` | `covered` |
| `cleanroom-input/qnt/battle-runtime/battle-runtime-concentration-break-teardown.mbt.qnt#step:doFailConcentrationSave` | `tasks/target-replay-evidence/T005-battle-runtime-concentration-break-teardown.json#T005 seed=1 action=doFailConcentrationSave#step:doFailConcentrationSave` | `src/qnt_adapters/battle_runtime_concentration_break_teardown.rs` | `covered` |
| `cleanroom-input/qnt/battle-runtime/battle-runtime-concentration-break-teardown.mbt.qnt#step:doVoluntaryEndConcentration` | `tasks/target-replay-evidence/T005-battle-runtime-concentration-break-teardown.json#T005 seed=1 action=doVoluntaryEndConcentration#step:doVoluntaryEndConcentration` | `src/qnt_adapters/battle_runtime_concentration_break_teardown.rs` | `covered` |
| `cleanroom-input/qnt/battle-runtime/battle-runtime-weapon-attack-skeleton.mbt.qnt#step:doDiscoverAttack` | `tasks/target-replay-evidence/RRCONV-19A-battle-setup-entrypoint.json#RRCONV-19A dirty replay action=doDiscoverAttack#step:doDiscoverAttack` | `src/qnt_adapters/battle_runtime_weapon_attack_skeleton.rs` | `covered` |
| `cleanroom-input/qnt/battle-runtime/battle-runtime-weapon-attack-skeleton.mbt.qnt#step:doFillAttackRollHit` | `tasks/target-replay-evidence/RRCONV-19A-battle-setup-entrypoint.json#RRCONV-19A dirty replay action=doFillAttackRollHit#step:doFillAttackRollHit` | `src/qnt_adapters/battle_runtime_weapon_attack_skeleton.rs` | `covered` |
| `cleanroom-input/qnt/battle-runtime/battle-runtime-weapon-attack-skeleton.mbt.qnt#step:doFillAttackRollMiss` | `tasks/target-replay-evidence/RRCONV-19A-battle-setup-entrypoint.json#RRCONV-19A dirty replay action=doFillAttackRollMiss#step:doFillAttackRollMiss` | `src/qnt_adapters/battle_runtime_weapon_attack_skeleton.rs` | `covered` |
| `cleanroom-input/qnt/battle-runtime/battle-runtime-weapon-attack-skeleton.mbt.qnt#step:doFillDamageHigh` | `tasks/target-replay-evidence/RRCONV-19A-battle-setup-entrypoint.json#RRCONV-19A dirty replay action=doFillDamageHigh#step:doFillDamageHigh` | `src/qnt_adapters/battle_runtime_weapon_attack_skeleton.rs` | `covered` |
| `cleanroom-input/qnt/battle-runtime/battle-runtime-weapon-attack-skeleton.mbt.qnt#step:doFillDamageHighSneakAttack` | `tasks/target-replay-evidence/RRCONV-19A-battle-setup-entrypoint.json#RRCONV-19A dirty replay action=doFillDamageHighSneakAttack#step:doFillDamageHighSneakAttack` | `src/qnt_adapters/battle_runtime_weapon_attack_skeleton.rs` | `covered` |
| `cleanroom-input/qnt/battle-runtime/battle-runtime-weapon-attack-skeleton.mbt.qnt#step:doFillDamageLow` | `tasks/target-replay-evidence/RRCONV-19A-battle-setup-entrypoint.json#RRCONV-19A dirty replay action=doFillDamageLow#step:doFillDamageLow` | `src/qnt_adapters/battle_runtime_weapon_attack_skeleton.rs` | `covered` |
| `cleanroom-input/qnt/battle-runtime/battle-runtime-weapon-attack-skeleton.mbt.qnt#step:doFillDamageLowSneakAttack` | `tasks/target-replay-evidence/RRCONV-19A-battle-setup-entrypoint.json#RRCONV-19A dirty replay action=doFillDamageLowSneakAttack#step:doFillDamageLowSneakAttack` | `src/qnt_adapters/battle_runtime_weapon_attack_skeleton.rs` | `covered` |
| `cleanroom-input/qnt/battle-runtime/battle-runtime-weapon-attack-skeleton.mbt.qnt#step:doFillTarget` | `tasks/target-replay-evidence/RRCONV-19A-battle-setup-entrypoint.json#RRCONV-19A dirty replay action=doFillTarget#step:doFillTarget` | `src/qnt_adapters/battle_runtime_weapon_attack_skeleton.rs` | `covered` |
| `cleanroom-input/qnt/battle-runtime/battle-runtime-weapon-attack-skeleton.mbt.qnt#step:doRejectRecursiveSkeletonMultiattack` | `tasks/target-replay-evidence/RRCONV-19A-battle-setup-entrypoint.json#RRCONV-19A dirty replay action=doRejectRecursiveSkeletonMultiattack#step:doRejectRecursiveSkeletonMultiattack` | `src/qnt_adapters/battle_runtime_weapon_attack_skeleton.rs` | `covered` |
| `cleanroom-input/qnt/battle-runtime/battle-runtime-weapon-attack-skeleton.mbt.qnt#step:doRejectStaleAfterResolved` | `tasks/target-replay-evidence/RRCONV-19A-battle-setup-entrypoint.json#RRCONV-19A dirty replay action=doRejectStaleAfterResolved#step:doRejectStaleAfterResolved` | `src/qnt_adapters/battle_runtime_weapon_attack_skeleton.rs` | `covered` |
| `cleanroom-input/qnt/battle-runtime/battle-runtime-weapon-attack-skeleton.mbt.qnt#step:doRejectWrongTarget` | `tasks/target-replay-evidence/RRCONV-19A-battle-setup-entrypoint.json#RRCONV-19A dirty replay action=doRejectWrongTarget#step:doRejectWrongTarget` | `src/qnt_adapters/battle_runtime_weapon_attack_skeleton.rs` | `covered` |
| `cleanroom-input/qnt/battle-runtime/battle-runtime-weapon-attack-skeleton.mbt.qnt#step:doResolveSkeletonMultiattack` | `tasks/target-replay-evidence/RRCONV-19A-battle-setup-entrypoint.json#RRCONV-19A dirty replay action=doResolveSkeletonMultiattack#step:doResolveSkeletonMultiattack` | `src/qnt_adapters/battle_runtime_weapon_attack_skeleton.rs` | `covered` |
| `cleanroom-input/qnt/battle-runtime/battle-runtime-weapon-attack-skeleton.mbt.qnt#step:doSpendSkeletonMultiattackDispatch` | `tasks/target-replay-evidence/RRCONV-19A-battle-setup-entrypoint.json#RRCONV-19A dirty replay action=doSpendSkeletonMultiattackDispatch#step:doSpendSkeletonMultiattackDispatch` | `src/qnt_adapters/battle_runtime_weapon_attack_skeleton.rs` | `covered` |
| `cleanroom-input/qnt/battle-runtime/battle-runtime-weapon-attack-skeleton.mbt.qnt#step:doStartSkeletonTurn` | `tasks/target-replay-evidence/RRCONV-19A-battle-setup-entrypoint.json#RRCONV-19A dirty replay action=doStartSkeletonTurn#step:doStartSkeletonTurn` | `src/qnt_adapters/battle_runtime_weapon_attack_skeleton.rs` | `covered` |
| `cleanroom-input/qnt/battle-runtime/battle-runtime-weapon-attack-skeleton.mbt.qnt#step:doDiscoverAttack` | `tasks/target-replay-evidence/RRCONV-19B-act-discovery-contract.json#RRCONV-19B dirty replay action=doDiscoverAttack#step:doDiscoverAttack` | `src/qnt_adapters/battle_runtime_weapon_attack_skeleton.rs` | `covered` |
| `cleanroom-input/qnt/battle-runtime/battle-runtime-weapon-attack-skeleton.mbt.qnt#step:doFillAttackRollHit` | `tasks/target-replay-evidence/RRCONV-19B-act-discovery-contract.json#RRCONV-19B dirty replay action=doFillAttackRollHit#step:doFillAttackRollHit` | `src/qnt_adapters/battle_runtime_weapon_attack_skeleton.rs` | `covered` |
| `cleanroom-input/qnt/battle-runtime/battle-runtime-weapon-attack-skeleton.mbt.qnt#step:doFillAttackRollMiss` | `tasks/target-replay-evidence/RRCONV-19B-act-discovery-contract.json#RRCONV-19B dirty replay action=doFillAttackRollMiss#step:doFillAttackRollMiss` | `src/qnt_adapters/battle_runtime_weapon_attack_skeleton.rs` | `covered` |
| `cleanroom-input/qnt/battle-runtime/battle-runtime-weapon-attack-skeleton.mbt.qnt#step:doFillDamageHigh` | `tasks/target-replay-evidence/RRCONV-19B-act-discovery-contract.json#RRCONV-19B dirty replay action=doFillDamageHigh#step:doFillDamageHigh` | `src/qnt_adapters/battle_runtime_weapon_attack_skeleton.rs` | `covered` |
| `cleanroom-input/qnt/battle-runtime/battle-runtime-weapon-attack-skeleton.mbt.qnt#step:doFillDamageHighSneakAttack` | `tasks/target-replay-evidence/RRCONV-19B-act-discovery-contract.json#RRCONV-19B dirty replay action=doFillDamageHighSneakAttack#step:doFillDamageHighSneakAttack` | `src/qnt_adapters/battle_runtime_weapon_attack_skeleton.rs` | `covered` |
| `cleanroom-input/qnt/battle-runtime/battle-runtime-weapon-attack-skeleton.mbt.qnt#step:doFillDamageLow` | `tasks/target-replay-evidence/RRCONV-19B-act-discovery-contract.json#RRCONV-19B dirty replay action=doFillDamageLow#step:doFillDamageLow` | `src/qnt_adapters/battle_runtime_weapon_attack_skeleton.rs` | `covered` |
| `cleanroom-input/qnt/battle-runtime/battle-runtime-weapon-attack-skeleton.mbt.qnt#step:doFillDamageLowSneakAttack` | `tasks/target-replay-evidence/RRCONV-19B-act-discovery-contract.json#RRCONV-19B dirty replay action=doFillDamageLowSneakAttack#step:doFillDamageLowSneakAttack` | `src/qnt_adapters/battle_runtime_weapon_attack_skeleton.rs` | `covered` |
| `cleanroom-input/qnt/battle-runtime/battle-runtime-weapon-attack-skeleton.mbt.qnt#step:doFillTarget` | `tasks/target-replay-evidence/RRCONV-19B-act-discovery-contract.json#RRCONV-19B dirty replay action=doFillTarget#step:doFillTarget` | `src/qnt_adapters/battle_runtime_weapon_attack_skeleton.rs` | `covered` |
| `cleanroom-input/qnt/battle-runtime/battle-runtime-weapon-attack-skeleton.mbt.qnt#step:doRejectRecursiveSkeletonMultiattack` | `tasks/target-replay-evidence/RRCONV-19B-act-discovery-contract.json#RRCONV-19B dirty replay action=doRejectRecursiveSkeletonMultiattack#step:doRejectRecursiveSkeletonMultiattack` | `src/qnt_adapters/battle_runtime_weapon_attack_skeleton.rs` | `covered` |
| `cleanroom-input/qnt/battle-runtime/battle-runtime-weapon-attack-skeleton.mbt.qnt#step:doRejectStaleAfterResolved` | `tasks/target-replay-evidence/RRCONV-19B-act-discovery-contract.json#RRCONV-19B dirty replay action=doRejectStaleAfterResolved#step:doRejectStaleAfterResolved` | `src/qnt_adapters/battle_runtime_weapon_attack_skeleton.rs` | `covered` |
| `cleanroom-input/qnt/battle-runtime/battle-runtime-weapon-attack-skeleton.mbt.qnt#step:doRejectWrongTarget` | `tasks/target-replay-evidence/RRCONV-19B-act-discovery-contract.json#RRCONV-19B dirty replay action=doRejectWrongTarget#step:doRejectWrongTarget` | `src/qnt_adapters/battle_runtime_weapon_attack_skeleton.rs` | `covered` |
| `cleanroom-input/qnt/battle-runtime/battle-runtime-weapon-attack-skeleton.mbt.qnt#step:doResolveSkeletonMultiattack` | `tasks/target-replay-evidence/RRCONV-19B-act-discovery-contract.json#RRCONV-19B dirty replay action=doResolveSkeletonMultiattack#step:doResolveSkeletonMultiattack` | `src/qnt_adapters/battle_runtime_weapon_attack_skeleton.rs` | `covered` |
| `cleanroom-input/qnt/battle-runtime/battle-runtime-weapon-attack-skeleton.mbt.qnt#step:doSpendSkeletonMultiattackDispatch` | `tasks/target-replay-evidence/RRCONV-19B-act-discovery-contract.json#RRCONV-19B dirty replay action=doSpendSkeletonMultiattackDispatch#step:doSpendSkeletonMultiattackDispatch` | `src/qnt_adapters/battle_runtime_weapon_attack_skeleton.rs` | `covered` |
| `cleanroom-input/qnt/battle-runtime/battle-runtime-weapon-attack-skeleton.mbt.qnt#step:doStartSkeletonTurn` | `tasks/target-replay-evidence/RRCONV-19B-act-discovery-contract.json#RRCONV-19B dirty replay action=doStartSkeletonTurn#step:doStartSkeletonTurn` | `src/qnt_adapters/battle_runtime_weapon_attack_skeleton.rs` | `covered` |
| `cleanroom-input/qnt/battle-runtime/battle-runtime-reducer-spine-contract.mbt.qnt#step:doStartBattle` | `tasks/target-replay-evidence/RRCONV-19D-turn-advance-result.json#RRCONV-19D dirty replay action=doStartBattle#step:doStartBattle` | `src/qnt_adapters/battle_runtime_reducer_spine_contract.rs` | `covered` |
| `cleanroom-input/qnt/battle-runtime/battle-runtime-reducer-spine-contract.mbt.qnt#step:doDiscoverSlotSpell` | `tasks/target-replay-evidence/RRCONV-19D-turn-advance-result.json#RRCONV-19D dirty replay action=doDiscoverSlotSpell#step:doDiscoverSlotSpell` | `src/qnt_adapters/battle_runtime_reducer_spine_contract.rs` | `covered` |
| `cleanroom-input/qnt/battle-runtime/battle-runtime-reducer-spine-contract.mbt.qnt#step:doResolveSlotSpellTargets` | `tasks/target-replay-evidence/RRCONV-19D-turn-advance-result.json#RRCONV-19D dirty replay action=doResolveSlotSpellTargets#step:doResolveSlotSpellTargets` | `src/qnt_adapters/battle_runtime_reducer_spine_contract.rs` | `covered` |
| `cleanroom-input/qnt/battle-runtime/battle-runtime-reducer-spine-contract.mbt.qnt#step:doResolveSlotSpellDamage` | `tasks/target-replay-evidence/RRCONV-19D-turn-advance-result.json#RRCONV-19D dirty replay action=doResolveSlotSpellDamage#step:doResolveSlotSpellDamage` | `src/qnt_adapters/battle_runtime_reducer_spine_contract.rs` | `covered` |
| `cleanroom-input/qnt/battle-runtime/battle-runtime-reducer-spine-contract.mbt.qnt#step:doEndTurnToTarget` | `tasks/target-replay-evidence/RRCONV-19D-turn-advance-result.json#RRCONV-19D dirty replay action=doEndTurnToTarget#step:doEndTurnToTarget` | `src/qnt_adapters/battle_runtime_reducer_spine_contract.rs` | `covered` |
| `cleanroom-input/qnt/battle-runtime/battle-runtime-reducer-spine-contract.mbt.qnt#step:doDiscoverWeaponAttack` | `tasks/target-replay-evidence/RRCONV-19D-turn-advance-result.json#RRCONV-19D dirty replay action=doDiscoverWeaponAttack#step:doDiscoverWeaponAttack` | `src/qnt_adapters/battle_runtime_reducer_spine_contract.rs` | `covered` |
| `cleanroom-input/qnt/battle-runtime/battle-runtime-reducer-spine-contract.mbt.qnt#step:doResolveWeaponTarget` | `tasks/target-replay-evidence/RRCONV-19D-turn-advance-result.json#RRCONV-19D dirty replay action=doResolveWeaponTarget#step:doResolveWeaponTarget` | `src/qnt_adapters/battle_runtime_reducer_spine_contract.rs` | `covered` |
| `cleanroom-input/qnt/battle-runtime/battle-runtime-reducer-spine-contract.mbt.qnt#step:doResolveWeaponAttackHit` | `tasks/target-replay-evidence/RRCONV-19D-turn-advance-result.json#RRCONV-19D dirty replay action=doResolveWeaponAttackHit#step:doResolveWeaponAttackHit` | `src/qnt_adapters/battle_runtime_reducer_spine_contract.rs` | `covered` |
| `cleanroom-input/qnt/battle-runtime/battle-runtime-reducer-spine-contract.mbt.qnt#step:doResolveWeaponDamage` | `tasks/target-replay-evidence/RRCONV-19D-turn-advance-result.json#RRCONV-19D dirty replay action=doResolveWeaponDamage#step:doResolveWeaponDamage` | `src/qnt_adapters/battle_runtime_reducer_spine_contract.rs` | `covered` |
| `cleanroom-input/qnt/battle-runtime/battle-runtime-reducer-spine-contract.mbt.qnt#step:doDiscoverSlotSpell` | `tasks/target-replay-evidence/RRCONV-19C-resolution-result-contract.json#RRCONV-19C-RUST-RESOLUTION-RESULT-CONTRACT fresh replay action=doDiscoverSlotSpell#step:doDiscoverSlotSpell` | `src/qnt_adapters/battle_runtime_reducer_spine_contract.rs` | `covered` |
| `cleanroom-input/qnt/battle-runtime/battle-runtime-reducer-spine-contract.mbt.qnt#step:doDiscoverWeaponAttack` | `tasks/target-replay-evidence/RRCONV-19C-resolution-result-contract.json#RRCONV-19C-RUST-RESOLUTION-RESULT-CONTRACT fresh replay action=doDiscoverWeaponAttack#step:doDiscoverWeaponAttack` | `src/qnt_adapters/battle_runtime_reducer_spine_contract.rs` | `covered` |
| `cleanroom-input/qnt/battle-runtime/battle-runtime-reducer-spine-contract.mbt.qnt#step:doEndTurnToTarget` | `tasks/target-replay-evidence/RRCONV-19C-resolution-result-contract.json#RRCONV-19C-RUST-RESOLUTION-RESULT-CONTRACT fresh replay action=doEndTurnToTarget#step:doEndTurnToTarget` | `src/qnt_adapters/battle_runtime_reducer_spine_contract.rs` | `covered` |
| `cleanroom-input/qnt/battle-runtime/battle-runtime-reducer-spine-contract.mbt.qnt#step:doResolveSlotSpellDamage` | `tasks/target-replay-evidence/RRCONV-19C-resolution-result-contract.json#RRCONV-19C-RUST-RESOLUTION-RESULT-CONTRACT fresh replay action=doResolveSlotSpellDamage#step:doResolveSlotSpellDamage` | `src/qnt_adapters/battle_runtime_reducer_spine_contract.rs` | `covered` |
| `cleanroom-input/qnt/battle-runtime/battle-runtime-reducer-spine-contract.mbt.qnt#step:doResolveSlotSpellTargets` | `tasks/target-replay-evidence/RRCONV-19C-resolution-result-contract.json#RRCONV-19C-RUST-RESOLUTION-RESULT-CONTRACT fresh replay action=doResolveSlotSpellTargets#step:doResolveSlotSpellTargets` | `src/qnt_adapters/battle_runtime_reducer_spine_contract.rs` | `covered` |
| `cleanroom-input/qnt/battle-runtime/battle-runtime-reducer-spine-contract.mbt.qnt#step:doResolveWeaponAttackHit` | `tasks/target-replay-evidence/RRCONV-19C-resolution-result-contract.json#RRCONV-19C-RUST-RESOLUTION-RESULT-CONTRACT fresh replay action=doResolveWeaponAttackHit#step:doResolveWeaponAttackHit` | `src/qnt_adapters/battle_runtime_reducer_spine_contract.rs` | `covered` |
| `cleanroom-input/qnt/battle-runtime/battle-runtime-reducer-spine-contract.mbt.qnt#step:doResolveWeaponDamage` | `tasks/target-replay-evidence/RRCONV-19C-resolution-result-contract.json#RRCONV-19C-RUST-RESOLUTION-RESULT-CONTRACT fresh replay action=doResolveWeaponDamage#step:doResolveWeaponDamage` | `src/qnt_adapters/battle_runtime_reducer_spine_contract.rs` | `covered` |
| `cleanroom-input/qnt/battle-runtime/battle-runtime-reducer-spine-contract.mbt.qnt#step:doResolveWeaponTarget` | `tasks/target-replay-evidence/RRCONV-19C-resolution-result-contract.json#RRCONV-19C-RUST-RESOLUTION-RESULT-CONTRACT fresh replay action=doResolveWeaponTarget#step:doResolveWeaponTarget` | `src/qnt_adapters/battle_runtime_reducer_spine_contract.rs` | `covered` |
| `cleanroom-input/qnt/battle-runtime/battle-runtime-reducer-spine-contract.mbt.qnt#step:doStartBattle` | `tasks/target-replay-evidence/RRCONV-19C-resolution-result-contract.json#RRCONV-19C-RUST-RESOLUTION-RESULT-CONTRACT fresh replay action=doStartBattle#step:doStartBattle` | `src/qnt_adapters/battle_runtime_reducer_spine_contract.rs` | `covered` |
| `cleanroom-input/qnt/battle-runtime/battle-runtime-reducer-spine-contract.mbt.qnt#step:doDiscoverSlotSpell` | `tasks/target-replay-evidence/RRCONV-19E-end-turn-subject-resolution.json#RRCONV-19E-RUST-END-TURN-SUBJECT-RESOLUTION fresh replay action=doDiscoverSlotSpell#step:doDiscoverSlotSpell` | `src/qnt_adapters/battle_runtime_reducer_spine_contract.rs` | `covered` |
| `cleanroom-input/qnt/battle-runtime/battle-runtime-reducer-spine-contract.mbt.qnt#step:doDiscoverWeaponAttack` | `tasks/target-replay-evidence/RRCONV-19E-end-turn-subject-resolution.json#RRCONV-19E-RUST-END-TURN-SUBJECT-RESOLUTION fresh replay action=doDiscoverWeaponAttack#step:doDiscoverWeaponAttack` | `src/qnt_adapters/battle_runtime_reducer_spine_contract.rs` | `covered` |
| `cleanroom-input/qnt/battle-runtime/battle-runtime-reducer-spine-contract.mbt.qnt#step:doEndTurnToTarget` | `tasks/target-replay-evidence/RRCONV-19E-end-turn-subject-resolution.json#RRCONV-19E-RUST-END-TURN-SUBJECT-RESOLUTION fresh replay action=doEndTurnToTarget#step:doEndTurnToTarget` | `src/qnt_adapters/battle_runtime_reducer_spine_contract.rs` | `covered` |
| `cleanroom-input/qnt/battle-runtime/battle-runtime-reducer-spine-contract.mbt.qnt#step:doResolveSlotSpellDamage` | `tasks/target-replay-evidence/RRCONV-19E-end-turn-subject-resolution.json#RRCONV-19E-RUST-END-TURN-SUBJECT-RESOLUTION fresh replay action=doResolveSlotSpellDamage#step:doResolveSlotSpellDamage` | `src/qnt_adapters/battle_runtime_reducer_spine_contract.rs` | `covered` |
| `cleanroom-input/qnt/battle-runtime/battle-runtime-reducer-spine-contract.mbt.qnt#step:doResolveSlotSpellTargets` | `tasks/target-replay-evidence/RRCONV-19E-end-turn-subject-resolution.json#RRCONV-19E-RUST-END-TURN-SUBJECT-RESOLUTION fresh replay action=doResolveSlotSpellTargets#step:doResolveSlotSpellTargets` | `src/qnt_adapters/battle_runtime_reducer_spine_contract.rs` | `covered` |
| `cleanroom-input/qnt/battle-runtime/battle-runtime-reducer-spine-contract.mbt.qnt#step:doResolveWeaponAttackHit` | `tasks/target-replay-evidence/RRCONV-19E-end-turn-subject-resolution.json#RRCONV-19E-RUST-END-TURN-SUBJECT-RESOLUTION fresh replay action=doResolveWeaponAttackHit#step:doResolveWeaponAttackHit` | `src/qnt_adapters/battle_runtime_reducer_spine_contract.rs` | `covered` |
| `cleanroom-input/qnt/battle-runtime/battle-runtime-reducer-spine-contract.mbt.qnt#step:doResolveWeaponDamage` | `tasks/target-replay-evidence/RRCONV-19E-end-turn-subject-resolution.json#RRCONV-19E-RUST-END-TURN-SUBJECT-RESOLUTION fresh replay action=doResolveWeaponDamage#step:doResolveWeaponDamage` | `src/qnt_adapters/battle_runtime_reducer_spine_contract.rs` | `covered` |
| `cleanroom-input/qnt/battle-runtime/battle-runtime-reducer-spine-contract.mbt.qnt#step:doResolveWeaponTarget` | `tasks/target-replay-evidence/RRCONV-19E-end-turn-subject-resolution.json#RRCONV-19E-RUST-END-TURN-SUBJECT-RESOLUTION fresh replay action=doResolveWeaponTarget#step:doResolveWeaponTarget` | `src/qnt_adapters/battle_runtime_reducer_spine_contract.rs` | `covered` |
| `cleanroom-input/qnt/battle-runtime/battle-runtime-reducer-spine-contract.mbt.qnt#step:doStartBattle` | `tasks/target-replay-evidence/RRCONV-19E-end-turn-subject-resolution.json#RRCONV-19E-RUST-END-TURN-SUBJECT-RESOLUTION fresh replay action=doStartBattle#step:doStartBattle` | `src/qnt_adapters/battle_runtime_reducer_spine_contract.rs` | `covered` |
| `cleanroom-input/qnt/battle-runtime/battle-runtime-magic-missile.mbt.qnt#step:doFillMagicMissileAllocation` | `tasks/target-replay-evidence/RRCONV-19F-route-event-from-reducer-result.json#RRCONV-19F-RUST-ROUTE-EVENT-FROM-REDUCER-RESULT replay action=doFillMagicMissileAllocation#step:doFillMagicMissileAllocation` | `src/qnt_adapters/battle_runtime_magic_missile.rs` | `covered` |
| `cleanroom-input/qnt/battle-runtime/battle-runtime-magic-missile.mbt.qnt#step:doFillMagicMissileDamage` | `tasks/target-replay-evidence/RRCONV-19F-route-event-from-reducer-result.json#RRCONV-19F-RUST-ROUTE-EVENT-FROM-REDUCER-RESULT replay action=doFillMagicMissileDamage#step:doFillMagicMissileDamage` | `src/qnt_adapters/battle_runtime_magic_missile.rs` | `covered` |
| `cleanroom-input/qnt/battle-runtime/battle-runtime-save-gated-spell-ordering.mbt.qnt#step:doDiscoverAreaSaveDamage` | `tasks/target-replay-evidence/RRCONV-19F-route-event-from-reducer-result.json#RRCONV-19F-RUST-ROUTE-EVENT-FROM-REDUCER-RESULT replay action=doDiscoverAreaSaveDamage#step:doDiscoverAreaSaveDamage` | `src/qnt_adapters/battle_runtime_save_gated_spell_ordering.rs` | `covered` |
| `cleanroom-input/qnt/battle-runtime/battle-runtime-save-gated-spell-ordering.mbt.qnt#step:doDiscoverTargetListConditionChoice` | `tasks/target-replay-evidence/RRCONV-19F-route-event-from-reducer-result.json#RRCONV-19F-RUST-ROUTE-EVENT-FROM-REDUCER-RESULT replay action=doDiscoverTargetListConditionChoice#step:doDiscoverTargetListConditionChoice` | `src/qnt_adapters/battle_runtime_save_gated_spell_ordering.rs` | `covered` |
| `cleanroom-input/qnt/battle-runtime/battle-runtime-save-gated-spell-ordering.mbt.qnt#step:doFillAreaDamageDice` | `tasks/target-replay-evidence/RRCONV-19F-route-event-from-reducer-result.json#RRCONV-19F-RUST-ROUTE-EVENT-FROM-REDUCER-RESULT replay action=doFillAreaDamageDice#step:doFillAreaDamageDice` | `src/qnt_adapters/battle_runtime_save_gated_spell_ordering.rs` | `covered` |
| `cleanroom-input/qnt/battle-runtime/battle-runtime-save-gated-spell-ordering.mbt.qnt#step:doFillAreaSaveFailed` | `tasks/target-replay-evidence/RRCONV-19F-route-event-from-reducer-result.json#RRCONV-19F-RUST-ROUTE-EVENT-FROM-REDUCER-RESULT replay action=doFillAreaSaveFailed#step:doFillAreaSaveFailed` | `src/qnt_adapters/battle_runtime_save_gated_spell_ordering.rs` | `covered` |
| `cleanroom-input/qnt/battle-runtime/battle-runtime-save-gated-spell-ordering.mbt.qnt#step:doFillConditionChoiceAfterTargetList` | `tasks/target-replay-evidence/RRCONV-19F-route-event-from-reducer-result.json#RRCONV-19F-RUST-ROUTE-EVENT-FROM-REDUCER-RESULT replay action=doFillConditionChoiceAfterTargetList#step:doFillConditionChoiceAfterTargetList` | `src/qnt_adapters/battle_runtime_save_gated_spell_ordering.rs` | `covered` |
| `cleanroom-input/qnt/battle-runtime/battle-runtime-save-gated-spell-ordering.mbt.qnt#step:doFillConditionChoiceBeforeTargetList` | `tasks/target-replay-evidence/RRCONV-19F-route-event-from-reducer-result.json#RRCONV-19F-RUST-ROUTE-EVENT-FROM-REDUCER-RESULT replay action=doFillConditionChoiceBeforeTargetList#step:doFillConditionChoiceBeforeTargetList` | `src/qnt_adapters/battle_runtime_save_gated_spell_ordering.rs` | `covered` |
| `cleanroom-input/qnt/battle-runtime/battle-runtime-save-gated-spell-ordering.mbt.qnt#step:doFillConditionSavingThrow` | `tasks/target-replay-evidence/RRCONV-19F-route-event-from-reducer-result.json#RRCONV-19F-RUST-ROUTE-EVENT-FROM-REDUCER-RESULT replay action=doFillConditionSavingThrow#step:doFillConditionSavingThrow` | `src/qnt_adapters/battle_runtime_save_gated_spell_ordering.rs` | `covered` |
| `cleanroom-input/qnt/battle-runtime/battle-runtime-save-gated-spell-ordering.mbt.qnt#step:doFillTargetListAfterConditionChoice` | `tasks/target-replay-evidence/RRCONV-19F-route-event-from-reducer-result.json#RRCONV-19F-RUST-ROUTE-EVENT-FROM-REDUCER-RESULT replay action=doFillTargetListAfterConditionChoice#step:doFillTargetListAfterConditionChoice` | `src/qnt_adapters/battle_runtime_save_gated_spell_ordering.rs` | `covered` |
| `cleanroom-input/qnt/battle-runtime/battle-runtime-save-gated-spell-ordering.mbt.qnt#step:doFillTargetListBeforeConditionChoice` | `tasks/target-replay-evidence/RRCONV-19F-route-event-from-reducer-result.json#RRCONV-19F-RUST-ROUTE-EVENT-FROM-REDUCER-RESULT replay action=doFillTargetListBeforeConditionChoice#step:doFillTargetListBeforeConditionChoice` | `src/qnt_adapters/battle_runtime_save_gated_spell_ordering.rs` | `covered` |
| `cleanroom-input/qnt/battle-runtime/battle-runtime-save-gated-spell-ordering.mbt.qnt#step:doSubmitDamageBeforeSavingThrow` | `tasks/target-replay-evidence/RRCONV-19F-route-event-from-reducer-result.json#RRCONV-19F-RUST-ROUTE-EVENT-FROM-REDUCER-RESULT replay action=doSubmitDamageBeforeSavingThrow#step:doSubmitDamageBeforeSavingThrow` | `src/qnt_adapters/battle_runtime_save_gated_spell_ordering.rs` | `covered` |
| `cleanroom-input/qnt/battle-runtime/battle-runtime-save-gated-spell-ordering.mbt.qnt#step:doDiscoverAreaSaveDamage` | `tasks/target-replay-evidence/RRCONV-19G-subject-continuation-lifecycle.json#RRCONV-19G-RUST-SUBJECT-CONTINUATION-LIFECYCLE replay action=doDiscoverAreaSaveDamage#step:doDiscoverAreaSaveDamage` | `src/qnt_adapters/battle_runtime_save_gated_spell_ordering.rs` | `covered` |
| `cleanroom-input/qnt/battle-runtime/battle-runtime-save-gated-spell-ordering.mbt.qnt#step:doDiscoverTargetListConditionChoice` | `tasks/target-replay-evidence/RRCONV-19G-subject-continuation-lifecycle.json#RRCONV-19G-RUST-SUBJECT-CONTINUATION-LIFECYCLE replay action=doDiscoverTargetListConditionChoice#step:doDiscoverTargetListConditionChoice` | `src/qnt_adapters/battle_runtime_save_gated_spell_ordering.rs` | `covered` |
| `cleanroom-input/qnt/battle-runtime/battle-runtime-save-gated-spell-ordering.mbt.qnt#step:doFillAreaDamageDice` | `tasks/target-replay-evidence/RRCONV-19G-subject-continuation-lifecycle.json#RRCONV-19G-RUST-SUBJECT-CONTINUATION-LIFECYCLE replay action=doFillAreaDamageDice#step:doFillAreaDamageDice` | `src/qnt_adapters/battle_runtime_save_gated_spell_ordering.rs` | `covered` |
| `cleanroom-input/qnt/battle-runtime/battle-runtime-save-gated-spell-ordering.mbt.qnt#step:doFillAreaSaveFailed` | `tasks/target-replay-evidence/RRCONV-19G-subject-continuation-lifecycle.json#RRCONV-19G-RUST-SUBJECT-CONTINUATION-LIFECYCLE replay action=doFillAreaSaveFailed#step:doFillAreaSaveFailed` | `src/qnt_adapters/battle_runtime_save_gated_spell_ordering.rs` | `covered` |
| `cleanroom-input/qnt/battle-runtime/battle-runtime-save-gated-spell-ordering.mbt.qnt#step:doFillConditionChoiceAfterTargetList` | `tasks/target-replay-evidence/RRCONV-19G-subject-continuation-lifecycle.json#RRCONV-19G-RUST-SUBJECT-CONTINUATION-LIFECYCLE replay action=doFillConditionChoiceAfterTargetList#step:doFillConditionChoiceAfterTargetList` | `src/qnt_adapters/battle_runtime_save_gated_spell_ordering.rs` | `covered` |
| `cleanroom-input/qnt/battle-runtime/battle-runtime-save-gated-spell-ordering.mbt.qnt#step:doFillConditionChoiceBeforeTargetList` | `tasks/target-replay-evidence/RRCONV-19G-subject-continuation-lifecycle.json#RRCONV-19G-RUST-SUBJECT-CONTINUATION-LIFECYCLE replay action=doFillConditionChoiceBeforeTargetList#step:doFillConditionChoiceBeforeTargetList` | `src/qnt_adapters/battle_runtime_save_gated_spell_ordering.rs` | `covered` |
| `cleanroom-input/qnt/battle-runtime/battle-runtime-save-gated-spell-ordering.mbt.qnt#step:doFillConditionSavingThrow` | `tasks/target-replay-evidence/RRCONV-19G-subject-continuation-lifecycle.json#RRCONV-19G-RUST-SUBJECT-CONTINUATION-LIFECYCLE replay action=doFillConditionSavingThrow#step:doFillConditionSavingThrow` | `src/qnt_adapters/battle_runtime_save_gated_spell_ordering.rs` | `covered` |
| `cleanroom-input/qnt/battle-runtime/battle-runtime-save-gated-spell-ordering.mbt.qnt#step:doFillTargetListAfterConditionChoice` | `tasks/target-replay-evidence/RRCONV-19G-subject-continuation-lifecycle.json#RRCONV-19G-RUST-SUBJECT-CONTINUATION-LIFECYCLE replay action=doFillTargetListAfterConditionChoice#step:doFillTargetListAfterConditionChoice` | `src/qnt_adapters/battle_runtime_save_gated_spell_ordering.rs` | `covered` |
| `cleanroom-input/qnt/battle-runtime/battle-runtime-save-gated-spell-ordering.mbt.qnt#step:doFillTargetListBeforeConditionChoice` | `tasks/target-replay-evidence/RRCONV-19G-subject-continuation-lifecycle.json#RRCONV-19G-RUST-SUBJECT-CONTINUATION-LIFECYCLE replay action=doFillTargetListBeforeConditionChoice#step:doFillTargetListBeforeConditionChoice` | `src/qnt_adapters/battle_runtime_save_gated_spell_ordering.rs` | `covered` |
| `cleanroom-input/qnt/battle-runtime/battle-runtime-save-gated-spell-ordering.mbt.qnt#step:doSubmitDamageBeforeSavingThrow` | `tasks/target-replay-evidence/RRCONV-19G-subject-continuation-lifecycle.json#RRCONV-19G-RUST-SUBJECT-CONTINUATION-LIFECYCLE replay action=doSubmitDamageBeforeSavingThrow#step:doSubmitDamageBeforeSavingThrow` | `src/qnt_adapters/battle_runtime_save_gated_spell_ordering.rs` | `covered` |
| `cleanroom-input/qnt/battle-runtime/battle-runtime-hit-point-restoration-ordering.mbt.qnt#step:doDiscoverFeatureHealingPool` | `tasks/target-replay-evidence/RRCONV-19G-subject-continuation-lifecycle.json#RRCONV-19G-RUST-SUBJECT-CONTINUATION-LIFECYCLE replay action=doDiscoverFeatureHealingPool#step:doDiscoverFeatureHealingPool` | `src/qnt_adapters/battle_runtime_hit_point_restoration_ordering.rs` | `covered` |
| `cleanroom-input/qnt/battle-runtime/battle-runtime-hit-point-restoration-ordering.mbt.qnt#step:doDiscoverSingleTargetSpellHealing` | `tasks/target-replay-evidence/RRCONV-19G-subject-continuation-lifecycle.json#RRCONV-19G-RUST-SUBJECT-CONTINUATION-LIFECYCLE replay action=doDiscoverSingleTargetSpellHealing#step:doDiscoverSingleTargetSpellHealing` | `src/qnt_adapters/battle_runtime_hit_point_restoration_ordering.rs` | `covered` |
| `cleanroom-input/qnt/battle-runtime/battle-runtime-hit-point-restoration-ordering.mbt.qnt#step:doDiscoverTargetListSpellHealing` | `tasks/target-replay-evidence/RRCONV-19G-subject-continuation-lifecycle.json#RRCONV-19G-RUST-SUBJECT-CONTINUATION-LIFECYCLE replay action=doDiscoverTargetListSpellHealing#step:doDiscoverTargetListSpellHealing` | `src/qnt_adapters/battle_runtime_hit_point_restoration_ordering.rs` | `covered` |
| `cleanroom-input/qnt/battle-runtime/battle-runtime-hit-point-restoration-ordering.mbt.qnt#step:doFillFeatureHealingDistribution` | `tasks/target-replay-evidence/RRCONV-19G-subject-continuation-lifecycle.json#RRCONV-19G-RUST-SUBJECT-CONTINUATION-LIFECYCLE replay action=doFillFeatureHealingDistribution#step:doFillFeatureHealingDistribution` | `src/qnt_adapters/battle_runtime_hit_point_restoration_ordering.rs` | `covered` |
| `cleanroom-input/qnt/battle-runtime/battle-runtime-hit-point-restoration-ordering.mbt.qnt#step:doFillSpellHealingRoll` | `tasks/target-replay-evidence/RRCONV-19G-subject-continuation-lifecycle.json#RRCONV-19G-RUST-SUBJECT-CONTINUATION-LIFECYCLE replay action=doFillSpellHealingRoll#step:doFillSpellHealingRoll` | `src/qnt_adapters/battle_runtime_hit_point_restoration_ordering.rs` | `covered` |
| `cleanroom-input/qnt/battle-runtime/battle-runtime-hit-point-restoration-ordering.mbt.qnt#step:doFillSpellHealingTargetChoice` | `tasks/target-replay-evidence/RRCONV-19G-subject-continuation-lifecycle.json#RRCONV-19G-RUST-SUBJECT-CONTINUATION-LIFECYCLE replay action=doFillSpellHealingTargetChoice#step:doFillSpellHealingTargetChoice` | `src/qnt_adapters/battle_runtime_hit_point_restoration_ordering.rs` | `covered` |
| `cleanroom-input/qnt/battle-runtime/battle-runtime-hit-point-restoration-ordering.mbt.qnt#step:doFillSpellHealingTargetList` | `tasks/target-replay-evidence/RRCONV-19G-subject-continuation-lifecycle.json#RRCONV-19G-RUST-SUBJECT-CONTINUATION-LIFECYCLE replay action=doFillSpellHealingTargetList#step:doFillSpellHealingTargetList` | `src/qnt_adapters/battle_runtime_hit_point_restoration_ordering.rs` | `covered` |
| `cleanroom-input/qnt/battle-runtime/battle-runtime-hit-point-restoration-ordering.mbt.qnt#step:doSubmitHealingRollBeforeTargetChoice` | `tasks/target-replay-evidence/RRCONV-19G-subject-continuation-lifecycle.json#RRCONV-19G-RUST-SUBJECT-CONTINUATION-LIFECYCLE replay action=doSubmitHealingRollBeforeTargetChoice#step:doSubmitHealingRollBeforeTargetChoice` | `src/qnt_adapters/battle_runtime_hit_point_restoration_ordering.rs` | `covered` |
| `cleanroom-input/qnt/battle-runtime/battle-runtime-hit-point-restoration-ordering.mbt.qnt#step:doSubmitHealingRollBeforeTargetList` | `tasks/target-replay-evidence/RRCONV-19G-subject-continuation-lifecycle.json#RRCONV-19G-RUST-SUBJECT-CONTINUATION-LIFECYCLE replay action=doSubmitHealingRollBeforeTargetList#step:doSubmitHealingRollBeforeTargetList` | `src/qnt_adapters/battle_runtime_hit_point_restoration_ordering.rs` | `covered` |
| `cleanroom-input/qnt/battle-runtime/battle-runtime-concentration-break-teardown.mbt.qnt#step:doCastConcentrationSpell` | `tasks/target-replay-evidence/L15-RR03-reducer-route.json#L15-RR03 route action=doCastConcentrationSpell#step:doCastConcentrationSpell` | `src/qnt_adapters/battle_runtime_concentration_break_teardown.rs` | `covered` |
| `cleanroom-input/qnt/battle-runtime/battle-runtime-concentration-break-teardown.mbt.qnt#step:doCastReplacementConcentrationSpell` | `tasks/target-replay-evidence/L15-RR03-reducer-route.json#L15-RR03 route action=doCastReplacementConcentrationSpell#step:doCastReplacementConcentrationSpell` | `src/qnt_adapters/battle_runtime_concentration_break_teardown.rs` | `covered` |
| `cleanroom-input/qnt/battle-runtime/battle-runtime-concentration-break-teardown.mbt.qnt#step:doDamageRequestsConcentrationSave` | `tasks/target-replay-evidence/L15-RR03-reducer-route.json#L15-RR03 route action=doDamageRequestsConcentrationSave#step:doDamageRequestsConcentrationSave` | `src/qnt_adapters/battle_runtime_concentration_break_teardown.rs` | `covered` |
| `cleanroom-input/qnt/battle-runtime/battle-runtime-concentration-break-teardown.mbt.qnt#step:doFailConcentrationSave` | `tasks/target-replay-evidence/L15-RR03-reducer-route.json#L15-RR03 route action=doFailConcentrationSave#step:doFailConcentrationSave` | `src/qnt_adapters/battle_runtime_concentration_break_teardown.rs` | `covered` |
| `cleanroom-input/qnt/battle-runtime/battle-runtime-concentration-break-teardown.mbt.qnt#step:doVoluntaryEndConcentration` | `tasks/target-replay-evidence/L15-RR03-reducer-route.json#L15-RR03 route action=doVoluntaryEndConcentration#step:doVoluntaryEndConcentration` | `src/qnt_adapters/battle_runtime_concentration_break_teardown.rs` | `covered` |
| `cleanroom-input/qnt/battle-runtime/battle-runtime-death-saving-throw.mbt.qnt#step:doDiscoverEndTurnDeathSavingThrow` | `tasks/target-replay-evidence/L15-RR03-reducer-route.json#L15-RR03 route action=doDiscoverEndTurnDeathSavingThrow#step:doDiscoverEndTurnDeathSavingThrow` | `src/qnt_adapters/battle_runtime_death_saving_throw.rs` | `covered` |
| `cleanroom-input/qnt/battle-runtime/battle-runtime-death-saving-throw.mbt.qnt#step:doFillDeathSavingThrow` | `tasks/target-replay-evidence/L15-RR03-reducer-route.json#L15-RR03 route action=doFillDeathSavingThrow#step:doFillDeathSavingThrow` | `src/qnt_adapters/battle_runtime_death_saving_throw.rs` | `covered` |
| `cleanroom-input/qnt/battle-runtime/battle-runtime-death-saving-throw.mbt.qnt#step:doRejectWrongActorEndTurnAfterResolved` | `tasks/target-replay-evidence/L15-RR03-reducer-route.json#L15-RR03 route action=doRejectWrongActorEndTurnAfterResolved#step:doRejectWrongActorEndTurnAfterResolved` | `src/qnt_adapters/battle_runtime_death_saving_throw.rs` | `covered` |
| `cleanroom-input/qnt/battle-runtime/battle-runtime-spell-attack-ordering.mbt.qnt#step:doDiscoverSingleTargetSpellAttack` | `tasks/target-replay-evidence/L15-RR05-attack-statblock-routes.json#L15-RR05-BATTLE-ACTION-ATTACK-STATBLOCK-ROUTES replay action=doDiscoverSingleTargetSpellAttack#step:doDiscoverSingleTargetSpellAttack` | `src/qnt_adapters/battle_runtime_spell_attack_ordering.rs` | `covered` |
| `cleanroom-input/qnt/battle-runtime/battle-runtime-spell-attack-ordering.mbt.qnt#step:doDiscoverTypedSpellAttack` | `tasks/target-replay-evidence/L15-RR05-attack-statblock-routes.json#L15-RR05-BATTLE-ACTION-ATTACK-STATBLOCK-ROUTES replay action=doDiscoverTypedSpellAttack#step:doDiscoverTypedSpellAttack` | `src/qnt_adapters/battle_runtime_spell_attack_ordering.rs` | `covered` |
| `cleanroom-input/qnt/battle-runtime/battle-runtime-spell-attack-ordering.mbt.qnt#step:doFillAttackRollHit` | `tasks/target-replay-evidence/L15-RR05-attack-statblock-routes.json#L15-RR05-BATTLE-ACTION-ATTACK-STATBLOCK-ROUTES replay action=doFillAttackRollHit#step:doFillAttackRollHit` | `src/qnt_adapters/battle_runtime_spell_attack_ordering.rs` | `covered` |
| `cleanroom-input/qnt/battle-runtime/battle-runtime-spell-attack-ordering.mbt.qnt#step:doFillAttackRollMiss` | `tasks/target-replay-evidence/L15-RR05-attack-statblock-routes.json#L15-RR05-BATTLE-ACTION-ATTACK-STATBLOCK-ROUTES replay action=doFillAttackRollMiss#step:doFillAttackRollMiss` | `src/qnt_adapters/battle_runtime_spell_attack_ordering.rs` | `covered` |
| `cleanroom-input/qnt/battle-runtime/battle-runtime-spell-attack-ordering.mbt.qnt#step:doFillDamageDice` | `tasks/target-replay-evidence/L15-RR05-attack-statblock-routes.json#L15-RR05-BATTLE-ACTION-ATTACK-STATBLOCK-ROUTES replay action=doFillDamageDice#step:doFillDamageDice` | `src/qnt_adapters/battle_runtime_spell_attack_ordering.rs` | `covered` |
| `cleanroom-input/qnt/battle-runtime/battle-runtime-spell-attack-ordering.mbt.qnt#step:doFillDamageTypeAfterTargetChoice` | `tasks/target-replay-evidence/L15-RR05-attack-statblock-routes.json#L15-RR05-BATTLE-ACTION-ATTACK-STATBLOCK-ROUTES replay action=doFillDamageTypeAfterTargetChoice#step:doFillDamageTypeAfterTargetChoice` | `src/qnt_adapters/battle_runtime_spell_attack_ordering.rs` | `covered` |
| `cleanroom-input/qnt/battle-runtime/battle-runtime-spell-attack-ordering.mbt.qnt#step:doFillDamageTypeBeforeTargetChoice` | `tasks/target-replay-evidence/L15-RR05-attack-statblock-routes.json#L15-RR05-BATTLE-ACTION-ATTACK-STATBLOCK-ROUTES replay action=doFillDamageTypeBeforeTargetChoice#step:doFillDamageTypeBeforeTargetChoice` | `src/qnt_adapters/battle_runtime_spell_attack_ordering.rs` | `covered` |
| `cleanroom-input/qnt/battle-runtime/battle-runtime-spell-attack-ordering.mbt.qnt#step:doFillTargetChoice` | `tasks/target-replay-evidence/L15-RR05-attack-statblock-routes.json#L15-RR05-BATTLE-ACTION-ATTACK-STATBLOCK-ROUTES replay action=doFillTargetChoice#step:doFillTargetChoice` | `src/qnt_adapters/battle_runtime_spell_attack_ordering.rs` | `covered` |
| `cleanroom-input/qnt/battle-runtime/battle-runtime-spell-attack-ordering.mbt.qnt#step:doFillTargetChoiceAfterDamageType` | `tasks/target-replay-evidence/L15-RR05-attack-statblock-routes.json#L15-RR05-BATTLE-ACTION-ATTACK-STATBLOCK-ROUTES replay action=doFillTargetChoiceAfterDamageType#step:doFillTargetChoiceAfterDamageType` | `src/qnt_adapters/battle_runtime_spell_attack_ordering.rs` | `covered` |
| `cleanroom-input/qnt/battle-runtime/battle-runtime-spell-attack-ordering.mbt.qnt#step:doFillTargetChoiceBeforeDamageType` | `tasks/target-replay-evidence/L15-RR05-attack-statblock-routes.json#L15-RR05-BATTLE-ACTION-ATTACK-STATBLOCK-ROUTES replay action=doFillTargetChoiceBeforeDamageType#step:doFillTargetChoiceBeforeDamageType` | `src/qnt_adapters/battle_runtime_spell_attack_ordering.rs` | `covered` |
| `cleanroom-input/qnt/battle-runtime/battle-runtime-spell-attack-ordering.mbt.qnt#step:doSubmitAttackRollBeforeTargetChoice` | `tasks/target-replay-evidence/L15-RR05-attack-statblock-routes.json#L15-RR05-BATTLE-ACTION-ATTACK-STATBLOCK-ROUTES replay action=doSubmitAttackRollBeforeTargetChoice#step:doSubmitAttackRollBeforeTargetChoice` | `src/qnt_adapters/battle_runtime_spell_attack_ordering.rs` | `covered` |
| `cleanroom-input/qnt/battle-runtime/battle-runtime-spell-attack-ordering.mbt.qnt#step:doSubmitDamageBeforeAttackRoll` | `tasks/target-replay-evidence/L15-RR05-attack-statblock-routes.json#L15-RR05-BATTLE-ACTION-ATTACK-STATBLOCK-ROUTES replay action=doSubmitDamageBeforeAttackRoll#step:doSubmitDamageBeforeAttackRoll` | `src/qnt_adapters/battle_runtime_spell_attack_ordering.rs` | `covered` |
| `cleanroom-input/qnt/battle-runtime/battle-runtime-stat-block-action-ordering.mbt.qnt#step:doDiscoverRolledActionAttackControl` | `tasks/target-replay-evidence/L15-RR05-attack-statblock-routes.json#L15-RR05-BATTLE-ACTION-ATTACK-STATBLOCK-ROUTES replay action=doDiscoverRolledActionAttackControl#step:doDiscoverRolledActionAttackControl` | `src/qnt_adapters/battle_runtime_stat_block_action_ordering.rs` | `covered` |
| `cleanroom-input/qnt/battle-runtime/battle-runtime-stat-block-action-ordering.mbt.qnt#step:doDiscoverStaticActionAttackControl` | `tasks/target-replay-evidence/L15-RR05-attack-statblock-routes.json#L15-RR05-BATTLE-ACTION-ATTACK-STATBLOCK-ROUTES replay action=doDiscoverStaticActionAttackControl#step:doDiscoverStaticActionAttackControl` | `src/qnt_adapters/battle_runtime_stat_block_action_ordering.rs` | `covered` |
| `cleanroom-input/qnt/battle-runtime/battle-runtime-stat-block-action-ordering.mbt.qnt#step:doFillAttackRollMiss` | `tasks/target-replay-evidence/L15-RR05-attack-statblock-routes.json#L15-RR05-BATTLE-ACTION-ATTACK-STATBLOCK-ROUTES replay action=doFillAttackRollMiss#step:doFillAttackRollMiss` | `src/qnt_adapters/battle_runtime_stat_block_action_ordering.rs` | `covered` |
| `cleanroom-input/qnt/battle-runtime/battle-runtime-stat-block-action-ordering.mbt.qnt#step:doFillDamageDice` | `tasks/target-replay-evidence/L15-RR05-attack-statblock-routes.json#L15-RR05-BATTLE-ACTION-ATTACK-STATBLOCK-ROUTES replay action=doFillDamageDice#step:doFillDamageDice` | `src/qnt_adapters/battle_runtime_stat_block_action_ordering.rs` | `covered` |
| `cleanroom-input/qnt/battle-runtime/battle-runtime-stat-block-action-ordering.mbt.qnt#step:doFillRechargeRoll` | `tasks/target-replay-evidence/L15-RR05-attack-statblock-routes.json#L15-RR05-BATTLE-ACTION-ATTACK-STATBLOCK-ROUTES replay action=doFillRechargeRoll#step:doFillRechargeRoll` | `src/qnt_adapters/battle_runtime_stat_block_action_ordering.rs` | `covered` |
| `cleanroom-input/qnt/battle-runtime/battle-runtime-stat-block-action-ordering.mbt.qnt#step:doFillRolledAttackRollHit` | `tasks/target-replay-evidence/L15-RR05-attack-statblock-routes.json#L15-RR05-BATTLE-ACTION-ATTACK-STATBLOCK-ROUTES replay action=doFillRolledAttackRollHit#step:doFillRolledAttackRollHit` | `src/qnt_adapters/battle_runtime_stat_block_action_ordering.rs` | `covered` |
| `cleanroom-input/qnt/battle-runtime/battle-runtime-stat-block-action-ordering.mbt.qnt#step:doFillStaticAttackRollHit` | `tasks/target-replay-evidence/L15-RR05-attack-statblock-routes.json#L15-RR05-BATTLE-ACTION-ATTACK-STATBLOCK-ROUTES replay action=doFillStaticAttackRollHit#step:doFillStaticAttackRollHit` | `src/qnt_adapters/battle_runtime_stat_block_action_ordering.rs` | `covered` |
| `cleanroom-input/qnt/battle-runtime/battle-runtime-stat-block-action-ordering.mbt.qnt#step:doFillTargetChoice` | `tasks/target-replay-evidence/L15-RR05-attack-statblock-routes.json#L15-RR05-BATTLE-ACTION-ATTACK-STATBLOCK-ROUTES replay action=doFillTargetChoice#step:doFillTargetChoice` | `src/qnt_adapters/battle_runtime_stat_block_action_ordering.rs` | `covered` |
| `cleanroom-input/qnt/battle-runtime/battle-runtime-stat-block-action-ordering.mbt.qnt#step:doRejectAttackRollBeforeTargetChoice` | `tasks/target-replay-evidence/L15-RR05-attack-statblock-routes.json#L15-RR05-BATTLE-ACTION-ATTACK-STATBLOCK-ROUTES replay action=doRejectAttackRollBeforeTargetChoice#step:doRejectAttackRollBeforeTargetChoice` | `src/qnt_adapters/battle_runtime_stat_block_action_ordering.rs` | `covered` |
| `cleanroom-input/qnt/battle-runtime/battle-runtime-stat-block-action-ordering.mbt.qnt#step:doRejectDamageBeforeAttackRoll` | `tasks/target-replay-evidence/L15-RR05-attack-statblock-routes.json#L15-RR05-BATTLE-ACTION-ATTACK-STATBLOCK-ROUTES replay action=doRejectDamageBeforeAttackRoll#step:doRejectDamageBeforeAttackRoll` | `src/qnt_adapters/battle_runtime_stat_block_action_ordering.rs` | `covered` |
| `cleanroom-input/qnt/battle-runtime/battle-runtime-stat-block-action-ordering.mbt.qnt#step:doSpendRechargeGatedRolledAttack` | `tasks/target-replay-evidence/L15-RR05-attack-statblock-routes.json#L15-RR05-BATTLE-ACTION-ATTACK-STATBLOCK-ROUTES replay action=doSpendRechargeGatedRolledAttack#step:doSpendRechargeGatedRolledAttack` | `src/qnt_adapters/battle_runtime_stat_block_action_ordering.rs` | `covered` |
| `cleanroom-input/qnt/battle-runtime/battle-runtime-stat-block-action-ordering.mbt.qnt#step:doStartMultiattackControl` | `tasks/target-replay-evidence/L15-RR05-attack-statblock-routes.json#L15-RR05-BATTLE-ACTION-ATTACK-STATBLOCK-ROUTES replay action=doStartMultiattackControl#step:doStartMultiattackControl` | `src/qnt_adapters/battle_runtime_stat_block_action_ordering.rs` | `covered` |
| `cleanroom-input/qnt/battle-runtime/battle-runtime-stat-block-multi-damage.mbt.qnt#step:doFillHitAttackRoll` | `tasks/target-replay-evidence/L15-RR05-attack-statblock-routes.json#L15-RR05-BATTLE-ACTION-ATTACK-STATBLOCK-ROUTES replay action=doFillHitAttackRoll#step:doFillHitAttackRoll` | `src/qnt_adapters/battle_runtime_stat_block_multi_damage.rs` | `covered` |
| `cleanroom-input/qnt/battle-runtime/battle-runtime-stat-block-multi-damage.mbt.qnt#step:doFillTargetChoice` | `tasks/target-replay-evidence/L15-RR05-attack-statblock-routes.json#L15-RR05-BATTLE-ACTION-ATTACK-STATBLOCK-ROUTES replay action=doFillTargetChoice#step:doFillTargetChoice` | `src/qnt_adapters/battle_runtime_stat_block_multi_damage.rs` | `covered` |
| `cleanroom-input/qnt/battle-runtime/battle-runtime-stat-block-multi-damage.mbt.qnt#step:doResolveRolledDamage` | `tasks/target-replay-evidence/L15-RR05-attack-statblock-routes.json#L15-RR05-BATTLE-ACTION-ATTACK-STATBLOCK-ROUTES replay action=doResolveRolledDamage#step:doResolveRolledDamage` | `src/qnt_adapters/battle_runtime_stat_block_multi_damage.rs` | `covered` |
| `cleanroom-input/qnt/battle-runtime/battle-runtime-stat-block-size-gated-condition-rider.mbt.qnt#step:doFillHitAttackRoll` | `tasks/target-replay-evidence/L15-RR05-attack-statblock-routes.json#L15-RR05-BATTLE-ACTION-ATTACK-STATBLOCK-ROUTES replay action=doFillHitAttackRoll#step:doFillHitAttackRoll` | `src/qnt_adapters/battle_runtime_stat_block_size_gated_condition_rider.rs` | `covered` |
| `cleanroom-input/qnt/battle-runtime/battle-runtime-stat-block-size-gated-condition-rider.mbt.qnt#step:doFillTargetChoice` | `tasks/target-replay-evidence/L15-RR05-attack-statblock-routes.json#L15-RR05-BATTLE-ACTION-ATTACK-STATBLOCK-ROUTES replay action=doFillTargetChoice#step:doFillTargetChoice` | `src/qnt_adapters/battle_runtime_stat_block_size_gated_condition_rider.rs` | `covered` |
| `cleanroom-input/qnt/battle-runtime/battle-runtime-stat-block-size-gated-condition-rider.mbt.qnt#step:doResolveDamage` | `tasks/target-replay-evidence/L15-RR05-attack-statblock-routes.json#L15-RR05-BATTLE-ACTION-ATTACK-STATBLOCK-ROUTES replay action=doResolveDamage#step:doResolveDamage` | `src/qnt_adapters/battle_runtime_stat_block_size_gated_condition_rider.rs` | `covered` |
| `cleanroom-input/qnt/battle-runtime/battle-runtime-weapon-attack-ordering.mbt.qnt#step:doDiscoverAttack` | `tasks/target-replay-evidence/L15-RR05-attack-statblock-routes.json#L15-RR05-BATTLE-ACTION-ATTACK-STATBLOCK-ROUTES replay action=doDiscoverAttack#step:doDiscoverAttack` | `src/qnt_adapters/battle_runtime_weapon_attack_ordering.rs` | `covered` |
| `cleanroom-input/qnt/battle-runtime/battle-runtime-weapon-attack-ordering.mbt.qnt#step:doFillAttackRollHit` | `tasks/target-replay-evidence/L15-RR05-attack-statblock-routes.json#L15-RR05-BATTLE-ACTION-ATTACK-STATBLOCK-ROUTES replay action=doFillAttackRollHit#step:doFillAttackRollHit` | `src/qnt_adapters/battle_runtime_weapon_attack_ordering.rs` | `covered` |
| `cleanroom-input/qnt/battle-runtime/battle-runtime-weapon-attack-ordering.mbt.qnt#step:doFillAttackRollMiss` | `tasks/target-replay-evidence/L15-RR05-attack-statblock-routes.json#L15-RR05-BATTLE-ACTION-ATTACK-STATBLOCK-ROUTES replay action=doFillAttackRollMiss#step:doFillAttackRollMiss` | `src/qnt_adapters/battle_runtime_weapon_attack_ordering.rs` | `covered` |
| `cleanroom-input/qnt/battle-runtime/battle-runtime-weapon-attack-ordering.mbt.qnt#step:doFillDamageDice` | `tasks/target-replay-evidence/L15-RR05-attack-statblock-routes.json#L15-RR05-BATTLE-ACTION-ATTACK-STATBLOCK-ROUTES replay action=doFillDamageDice#step:doFillDamageDice` | `src/qnt_adapters/battle_runtime_weapon_attack_ordering.rs` | `covered` |
| `cleanroom-input/qnt/battle-runtime/battle-runtime-weapon-attack-ordering.mbt.qnt#step:doFillTargetChoice` | `tasks/target-replay-evidence/L15-RR05-attack-statblock-routes.json#L15-RR05-BATTLE-ACTION-ATTACK-STATBLOCK-ROUTES replay action=doFillTargetChoice#step:doFillTargetChoice` | `src/qnt_adapters/battle_runtime_weapon_attack_ordering.rs` | `covered` |
| `cleanroom-input/qnt/battle-runtime/battle-runtime-weapon-attack-ordering.mbt.qnt#step:doRejectAttackRollBeforeTargetChoice` | `tasks/target-replay-evidence/L15-RR05-attack-statblock-routes.json#L15-RR05-BATTLE-ACTION-ATTACK-STATBLOCK-ROUTES replay action=doRejectAttackRollBeforeTargetChoice#step:doRejectAttackRollBeforeTargetChoice` | `src/qnt_adapters/battle_runtime_weapon_attack_ordering.rs` | `covered` |
| `cleanroom-input/qnt/battle-runtime/battle-runtime-weapon-attack-ordering.mbt.qnt#step:doRejectDamageBeforeAttackRoll` | `tasks/target-replay-evidence/L15-RR05-attack-statblock-routes.json#L15-RR05-BATTLE-ACTION-ATTACK-STATBLOCK-ROUTES replay action=doRejectDamageBeforeAttackRoll#step:doRejectDamageBeforeAttackRoll` | `src/qnt_adapters/battle_runtime_weapon_attack_ordering.rs` | `covered` |
| `cleanroom-input/qnt/battle-runtime/battle-runtime-weapon-attack-skeleton.mbt.qnt#step:doDiscoverAttack` | `tasks/target-replay-evidence/L15-RR05-attack-statblock-routes.json#L15-RR05-BATTLE-ACTION-ATTACK-STATBLOCK-ROUTES replay action=doDiscoverAttack#step:doDiscoverAttack` | `src/qnt_adapters/battle_runtime_weapon_attack_skeleton.rs` | `covered` |
| `cleanroom-input/qnt/battle-runtime/battle-runtime-weapon-attack-skeleton.mbt.qnt#step:doFillAttackRollHit` | `tasks/target-replay-evidence/L15-RR05-attack-statblock-routes.json#L15-RR05-BATTLE-ACTION-ATTACK-STATBLOCK-ROUTES replay action=doFillAttackRollHit#step:doFillAttackRollHit` | `src/qnt_adapters/battle_runtime_weapon_attack_skeleton.rs` | `covered` |
| `cleanroom-input/qnt/battle-runtime/battle-runtime-weapon-attack-skeleton.mbt.qnt#step:doFillAttackRollMiss` | `tasks/target-replay-evidence/L15-RR05-attack-statblock-routes.json#L15-RR05-BATTLE-ACTION-ATTACK-STATBLOCK-ROUTES replay action=doFillAttackRollMiss#step:doFillAttackRollMiss` | `src/qnt_adapters/battle_runtime_weapon_attack_skeleton.rs` | `covered` |
| `cleanroom-input/qnt/battle-runtime/battle-runtime-weapon-attack-skeleton.mbt.qnt#step:doFillDamageHigh` | `tasks/target-replay-evidence/L15-RR05-attack-statblock-routes.json#L15-RR05-BATTLE-ACTION-ATTACK-STATBLOCK-ROUTES replay action=doFillDamageHigh#step:doFillDamageHigh` | `src/qnt_adapters/battle_runtime_weapon_attack_skeleton.rs` | `covered` |
| `cleanroom-input/qnt/battle-runtime/battle-runtime-weapon-attack-skeleton.mbt.qnt#step:doFillDamageHighSneakAttack` | `tasks/target-replay-evidence/L15-RR05-attack-statblock-routes.json#L15-RR05-BATTLE-ACTION-ATTACK-STATBLOCK-ROUTES replay action=doFillDamageHighSneakAttack#step:doFillDamageHighSneakAttack` | `src/qnt_adapters/battle_runtime_weapon_attack_skeleton.rs` | `covered` |
| `cleanroom-input/qnt/battle-runtime/battle-runtime-weapon-attack-skeleton.mbt.qnt#step:doFillDamageLow` | `tasks/target-replay-evidence/L15-RR05-attack-statblock-routes.json#L15-RR05-BATTLE-ACTION-ATTACK-STATBLOCK-ROUTES replay action=doFillDamageLow#step:doFillDamageLow` | `src/qnt_adapters/battle_runtime_weapon_attack_skeleton.rs` | `covered` |
| `cleanroom-input/qnt/battle-runtime/battle-runtime-weapon-attack-skeleton.mbt.qnt#step:doFillDamageLowSneakAttack` | `tasks/target-replay-evidence/L15-RR05-attack-statblock-routes.json#L15-RR05-BATTLE-ACTION-ATTACK-STATBLOCK-ROUTES replay action=doFillDamageLowSneakAttack#step:doFillDamageLowSneakAttack` | `src/qnt_adapters/battle_runtime_weapon_attack_skeleton.rs` | `covered` |
| `cleanroom-input/qnt/battle-runtime/battle-runtime-weapon-attack-skeleton.mbt.qnt#step:doFillTarget` | `tasks/target-replay-evidence/L15-RR05-attack-statblock-routes.json#L15-RR05-BATTLE-ACTION-ATTACK-STATBLOCK-ROUTES replay action=doFillTarget#step:doFillTarget` | `src/qnt_adapters/battle_runtime_weapon_attack_skeleton.rs` | `covered` |
| `cleanroom-input/qnt/battle-runtime/battle-runtime-weapon-attack-skeleton.mbt.qnt#step:doRejectRecursiveSkeletonMultiattack` | `tasks/target-replay-evidence/L15-RR05-attack-statblock-routes.json#L15-RR05-BATTLE-ACTION-ATTACK-STATBLOCK-ROUTES replay action=doRejectRecursiveSkeletonMultiattack#step:doRejectRecursiveSkeletonMultiattack` | `src/qnt_adapters/battle_runtime_weapon_attack_skeleton.rs` | `covered` |
| `cleanroom-input/qnt/battle-runtime/battle-runtime-weapon-attack-skeleton.mbt.qnt#step:doRejectStaleAfterResolved` | `tasks/target-replay-evidence/L15-RR05-attack-statblock-routes.json#L15-RR05-BATTLE-ACTION-ATTACK-STATBLOCK-ROUTES replay action=doRejectStaleAfterResolved#step:doRejectStaleAfterResolved` | `src/qnt_adapters/battle_runtime_weapon_attack_skeleton.rs` | `covered` |
| `cleanroom-input/qnt/battle-runtime/battle-runtime-weapon-attack-skeleton.mbt.qnt#step:doRejectWrongTarget` | `tasks/target-replay-evidence/L15-RR05-attack-statblock-routes.json#L15-RR05-BATTLE-ACTION-ATTACK-STATBLOCK-ROUTES replay action=doRejectWrongTarget#step:doRejectWrongTarget` | `src/qnt_adapters/battle_runtime_weapon_attack_skeleton.rs` | `covered` |
| `cleanroom-input/qnt/battle-runtime/battle-runtime-weapon-attack-skeleton.mbt.qnt#step:doResolveSkeletonMultiattack` | `tasks/target-replay-evidence/L15-RR05-attack-statblock-routes.json#L15-RR05-BATTLE-ACTION-ATTACK-STATBLOCK-ROUTES replay action=doResolveSkeletonMultiattack#step:doResolveSkeletonMultiattack` | `src/qnt_adapters/battle_runtime_weapon_attack_skeleton.rs` | `covered` |
| `cleanroom-input/qnt/battle-runtime/battle-runtime-weapon-attack-skeleton.mbt.qnt#step:doSpendSkeletonMultiattackDispatch` | `tasks/target-replay-evidence/L15-RR05-attack-statblock-routes.json#L15-RR05-BATTLE-ACTION-ATTACK-STATBLOCK-ROUTES replay action=doSpendSkeletonMultiattackDispatch#step:doSpendSkeletonMultiattackDispatch` | `src/qnt_adapters/battle_runtime_weapon_attack_skeleton.rs` | `covered` |
| `cleanroom-input/qnt/battle-runtime/battle-runtime-weapon-attack-skeleton.mbt.qnt#step:doStartSkeletonTurn` | `tasks/target-replay-evidence/L15-RR05-attack-statblock-routes.json#L15-RR05-BATTLE-ACTION-ATTACK-STATBLOCK-ROUTES replay action=doStartSkeletonTurn#step:doStartSkeletonTurn` | `src/qnt_adapters/battle_runtime_weapon_attack_skeleton.rs` | `covered` |
| `cleanroom-input/qnt/battle-runtime/creature-attack.mbt.qnt#step:doAttackerAAttacks` | `tasks/target-replay-evidence/L15-RR05-attack-statblock-routes.json#L15-RR05-BATTLE-ACTION-ATTACK-STATBLOCK-ROUTES replay action=doAttackerAAttacks#step:doAttackerAAttacks` | `src/qnt_adapters/creature_attack_mbt.rs` | `covered` |
| `cleanroom-input/qnt/battle-runtime/creature-attack.mbt.qnt#step:doAttackerBAttacks` | `tasks/target-replay-evidence/L15-RR05-attack-statblock-routes.json#L15-RR05-BATTLE-ACTION-ATTACK-STATBLOCK-ROUTES replay action=doAttackerBAttacks#step:doAttackerBAttacks` | `src/qnt_adapters/creature_attack_mbt.rs` | `covered` |
| `cleanroom-input/qnt/battle-runtime/battle-runtime-command-option-next-turn.mbt.qnt#step:doFailedSaveRecordsPending` | `tasks/target-replay-evidence/L15-RR06-spell-effect-routes.json#L15-RR06-BATTLE-SPELL-EFFECT-ROUTES replay action=doFailedSaveRecordsPending#step:doFailedSaveRecordsPending` | `src/qnt_adapters/battle_runtime_command_option_next_turn.rs` | `covered` |
| `cleanroom-input/qnt/battle-runtime/battle-runtime-command-option-next-turn.mbt.qnt#step:doFollowGrovel` | `tasks/target-replay-evidence/L15-RR06-spell-effect-routes.json#L15-RR06-BATTLE-SPELL-EFFECT-ROUTES replay action=doFollowGrovel#step:doFollowGrovel` | `src/qnt_adapters/battle_runtime_command_option_next_turn.rs` | `covered` |
| `cleanroom-input/qnt/battle-runtime/battle-runtime-command-option-next-turn.mbt.qnt#step:doFollowDrop` | `tasks/target-replay-evidence/L15-RR06-spell-effect-routes.json#L15-RR06-BATTLE-SPELL-EFFECT-ROUTES replay action=doFollowDrop#step:doFollowDrop` | `src/qnt_adapters/battle_runtime_command_option_next_turn.rs` | `covered` |
| `cleanroom-input/qnt/battle-runtime/battle-runtime-command-option-next-turn.mbt.qnt#step:doHaltSuppresses` | `tasks/target-replay-evidence/L15-RR06-spell-effect-routes.json#L15-RR06-BATTLE-SPELL-EFFECT-ROUTES replay action=doHaltSuppresses#step:doHaltSuppresses` | `src/qnt_adapters/battle_runtime_command_option_next_turn.rs` | `covered` |
| `cleanroom-input/qnt/battle-runtime/battle-runtime-command-option-next-turn.mbt.qnt#step:doHaltEndTurnCleanup` | `tasks/target-replay-evidence/L15-RR06-spell-effect-routes.json#L15-RR06-BATTLE-SPELL-EFFECT-ROUTES replay action=doHaltEndTurnCleanup#step:doHaltEndTurnCleanup` | `src/qnt_adapters/battle_runtime_command_option_next_turn.rs` | `covered` |
| `cleanroom-input/qnt/battle-runtime/battle-runtime-command-option-next-turn.mbt.qnt#step:doApproachContinues` | `tasks/target-replay-evidence/L15-RR06-spell-effect-routes.json#L15-RR06-BATTLE-SPELL-EFFECT-ROUTES replay action=doApproachContinues#step:doApproachContinues` | `src/qnt_adapters/battle_runtime_command_option_next_turn.rs` | `covered` |
| `cleanroom-input/qnt/battle-runtime/battle-runtime-command-option-next-turn.mbt.qnt#step:doApproachWithinFiveEndsTurn` | `tasks/target-replay-evidence/L15-RR06-spell-effect-routes.json#L15-RR06-BATTLE-SPELL-EFFECT-ROUTES replay action=doApproachWithinFiveEndsTurn#step:doApproachWithinFiveEndsTurn` | `src/qnt_adapters/battle_runtime_command_option_next_turn.rs` | `covered` |
| `cleanroom-input/qnt/battle-runtime/battle-runtime-command-option-next-turn.mbt.qnt#step:doApproachMovementRejected` | `tasks/target-replay-evidence/L15-RR06-spell-effect-routes.json#L15-RR06-BATTLE-SPELL-EFFECT-ROUTES replay action=doApproachMovementRejected#step:doApproachMovementRejected` | `src/qnt_adapters/battle_runtime_command_option_next_turn.rs` | `covered` |
| `cleanroom-input/qnt/battle-runtime/battle-runtime-command-option-next-turn.mbt.qnt#step:doApproachNoMovementCleanup` | `tasks/target-replay-evidence/L15-RR06-spell-effect-routes.json#L15-RR06-BATTLE-SPELL-EFFECT-ROUTES replay action=doApproachNoMovementCleanup#step:doApproachNoMovementCleanup` | `src/qnt_adapters/battle_runtime_command_option_next_turn.rs` | `covered` |
| `cleanroom-input/qnt/battle-runtime/battle-runtime-command-option-next-turn.mbt.qnt#step:doFleeFullMovementEndsTurn` | `tasks/target-replay-evidence/L15-RR06-spell-effect-routes.json#L15-RR06-BATTLE-SPELL-EFFECT-ROUTES replay action=doFleeFullMovementEndsTurn#step:doFleeFullMovementEndsTurn` | `src/qnt_adapters/battle_runtime_command_option_next_turn.rs` | `covered` |
| `cleanroom-input/qnt/battle-runtime/battle-runtime-command-option-next-turn.mbt.qnt#step:doFleePartialMovementRejected` | `tasks/target-replay-evidence/L15-RR06-spell-effect-routes.json#L15-RR06-BATTLE-SPELL-EFFECT-ROUTES replay action=doFleePartialMovementRejected#step:doFleePartialMovementRejected` | `src/qnt_adapters/battle_runtime_command_option_next_turn.rs` | `covered` |
| `cleanroom-input/qnt/battle-runtime/battle-runtime-command-option-next-turn.mbt.qnt#step:doFleeNoMovementCleanup` | `tasks/target-replay-evidence/L15-RR06-spell-effect-routes.json#L15-RR06-BATTLE-SPELL-EFFECT-ROUTES replay action=doFleeNoMovementCleanup#step:doFleeNoMovementCleanup` | `src/qnt_adapters/battle_runtime_command_option_next_turn.rs` | `covered` |
| `cleanroom-input/qnt/battle-runtime/battle-runtime-command-option-next-turn.mbt.qnt#step:doFleeOpportunityAttackWindow` | `tasks/target-replay-evidence/L15-RR06-spell-effect-routes.json#L15-RR06-BATTLE-SPELL-EFFECT-ROUTES replay action=doFleeOpportunityAttackWindow#step:doFleeOpportunityAttackWindow` | `src/qnt_adapters/battle_runtime_command_option_next_turn.rs` | `covered` |
| `cleanroom-input/qnt/battle-runtime/battle-runtime-command-option-next-turn.mbt.qnt#step:doFleeOpportunityAttackDeclinedContinuation` | `tasks/target-replay-evidence/L15-RR06-spell-effect-routes.json#L15-RR06-BATTLE-SPELL-EFFECT-ROUTES replay action=doFleeOpportunityAttackDeclinedContinuation#step:doFleeOpportunityAttackDeclinedContinuation` | `src/qnt_adapters/battle_runtime_command_option_next_turn.rs` | `covered` |
| `cleanroom-input/qnt/battle-runtime/battle-runtime-command-option-next-turn.mbt.qnt#step:doComplete` | `tasks/target-replay-evidence/L15-RR06-spell-effect-routes.json#L15-RR06-BATTLE-SPELL-EFFECT-ROUTES replay action=doComplete#step:doComplete` | `src/qnt_adapters/battle_runtime_command_option_next_turn.rs` | `covered` |
| `cleanroom-input/qnt/battle-runtime/battle-runtime-command-ordering.mbt.qnt#step:doDiscoverCommand` | `tasks/target-replay-evidence/L15-RR06-spell-effect-routes.json#L15-RR06-BATTLE-SPELL-EFFECT-ROUTES replay action=doDiscoverCommand#step:doDiscoverCommand` | `src/qnt_adapters/battle_runtime_command_ordering.rs` | `covered` |
| `cleanroom-input/qnt/battle-runtime/battle-runtime-command-ordering.mbt.qnt#step:doSubmitOptionBeforeTargetList` | `tasks/target-replay-evidence/L15-RR06-spell-effect-routes.json#L15-RR06-BATTLE-SPELL-EFFECT-ROUTES replay action=doSubmitOptionBeforeTargetList#step:doSubmitOptionBeforeTargetList` | `src/qnt_adapters/battle_runtime_command_ordering.rs` | `covered` |
| `cleanroom-input/qnt/battle-runtime/battle-runtime-command-ordering.mbt.qnt#step:doFillTargetList` | `tasks/target-replay-evidence/L15-RR06-spell-effect-routes.json#L15-RR06-BATTLE-SPELL-EFFECT-ROUTES replay action=doFillTargetList#step:doFillTargetList` | `src/qnt_adapters/battle_runtime_command_ordering.rs` | `covered` |
| `cleanroom-input/qnt/battle-runtime/battle-runtime-command-ordering.mbt.qnt#step:doSubmitSavingThrowBeforeOption` | `tasks/target-replay-evidence/L15-RR06-spell-effect-routes.json#L15-RR06-BATTLE-SPELL-EFFECT-ROUTES replay action=doSubmitSavingThrowBeforeOption#step:doSubmitSavingThrowBeforeOption` | `src/qnt_adapters/battle_runtime_command_ordering.rs` | `covered` |
| `cleanroom-input/qnt/battle-runtime/battle-runtime-command-ordering.mbt.qnt#step:doFillGrovelOption` | `tasks/target-replay-evidence/L15-RR06-spell-effect-routes.json#L15-RR06-BATTLE-SPELL-EFFECT-ROUTES replay action=doFillGrovelOption#step:doFillGrovelOption` | `src/qnt_adapters/battle_runtime_command_ordering.rs` | `covered` |
| `cleanroom-input/qnt/battle-runtime/battle-runtime-command-ordering.mbt.qnt#step:doFillFailedGrovelSavingThrow` | `tasks/target-replay-evidence/L15-RR06-spell-effect-routes.json#L15-RR06-BATTLE-SPELL-EFFECT-ROUTES replay action=doFillFailedGrovelSavingThrow#step:doFillFailedGrovelSavingThrow` | `src/qnt_adapters/battle_runtime_command_ordering.rs` | `covered` |
| `cleanroom-input/qnt/battle-runtime/battle-runtime-command-ordering.mbt.qnt#step:doFollowGrovel` | `tasks/target-replay-evidence/L15-RR06-spell-effect-routes.json#L15-RR06-BATTLE-SPELL-EFFECT-ROUTES replay action=doFollowGrovel#step:doFollowGrovel` | `src/qnt_adapters/battle_runtime_command_ordering.rs` | `covered` |
| `cleanroom-input/qnt/battle-runtime/battle-runtime-command-ordering.mbt.qnt#step:doDropNeedsHeldObjectFacts` | `tasks/target-replay-evidence/L15-RR06-spell-effect-routes.json#L15-RR06-BATTLE-SPELL-EFFECT-ROUTES replay action=doDropNeedsHeldObjectFacts#step:doDropNeedsHeldObjectFacts` | `src/qnt_adapters/battle_runtime_command_ordering.rs` | `covered` |
| `cleanroom-input/qnt/battle-runtime/battle-runtime-command-ordering.mbt.qnt#step:doFillDropHeldObjectFacts` | `tasks/target-replay-evidence/L15-RR06-spell-effect-routes.json#L15-RR06-BATTLE-SPELL-EFFECT-ROUTES replay action=doFillDropHeldObjectFacts#step:doFillDropHeldObjectFacts` | `src/qnt_adapters/battle_runtime_command_ordering.rs` | `covered` |
| `cleanroom-input/qnt/battle-runtime/battle-runtime-command-ordering.mbt.qnt#step:doHaltSuppresses` | `tasks/target-replay-evidence/L15-RR06-spell-effect-routes.json#L15-RR06-BATTLE-SPELL-EFFECT-ROUTES replay action=doHaltSuppresses#step:doHaltSuppresses` | `src/qnt_adapters/battle_runtime_command_ordering.rs` | `covered` |
| `cleanroom-input/qnt/battle-runtime/battle-runtime-command-ordering.mbt.qnt#step:doApproachMovementContinues` | `tasks/target-replay-evidence/L15-RR06-spell-effect-routes.json#L15-RR06-BATTLE-SPELL-EFFECT-ROUTES replay action=doApproachMovementContinues#step:doApproachMovementContinues` | `src/qnt_adapters/battle_runtime_command_ordering.rs` | `covered` |
| `cleanroom-input/qnt/battle-runtime/battle-runtime-command-ordering.mbt.qnt#step:doFillApproachMovementContinues` | `tasks/target-replay-evidence/L15-RR06-spell-effect-routes.json#L15-RR06-BATTLE-SPELL-EFFECT-ROUTES replay action=doFillApproachMovementContinues#step:doFillApproachMovementContinues` | `src/qnt_adapters/battle_runtime_command_ordering.rs` | `covered` |
| `cleanroom-input/qnt/battle-runtime/battle-runtime-command-ordering.mbt.qnt#step:doFillApproachMovementWithinFive` | `tasks/target-replay-evidence/L15-RR06-spell-effect-routes.json#L15-RR06-BATTLE-SPELL-EFFECT-ROUTES replay action=doFillApproachMovementWithinFive#step:doFillApproachMovementWithinFive` | `src/qnt_adapters/battle_runtime_command_ordering.rs` | `covered` |
| `cleanroom-input/qnt/battle-runtime/battle-runtime-command-ordering.mbt.qnt#step:doApproachNoMovement` | `tasks/target-replay-evidence/L15-RR06-spell-effect-routes.json#L15-RR06-BATTLE-SPELL-EFFECT-ROUTES replay action=doApproachNoMovement#step:doApproachNoMovement` | `src/qnt_adapters/battle_runtime_command_ordering.rs` | `covered` |
| `cleanroom-input/qnt/battle-runtime/battle-runtime-command-ordering.mbt.qnt#step:doFleeMovement` | `tasks/target-replay-evidence/L15-RR06-spell-effect-routes.json#L15-RR06-BATTLE-SPELL-EFFECT-ROUTES replay action=doFleeMovement#step:doFleeMovement` | `src/qnt_adapters/battle_runtime_command_ordering.rs` | `covered` |
| `cleanroom-input/qnt/battle-runtime/battle-runtime-command-ordering.mbt.qnt#step:doFillFleeMovement` | `tasks/target-replay-evidence/L15-RR06-spell-effect-routes.json#L15-RR06-BATTLE-SPELL-EFFECT-ROUTES replay action=doFillFleeMovement#step:doFillFleeMovement` | `src/qnt_adapters/battle_runtime_command_ordering.rs` | `covered` |
| `cleanroom-input/qnt/battle-runtime/battle-runtime-command-ordering.mbt.qnt#step:doRejectFleePartialMovement` | `tasks/target-replay-evidence/L15-RR06-spell-effect-routes.json#L15-RR06-BATTLE-SPELL-EFFECT-ROUTES replay action=doRejectFleePartialMovement#step:doRejectFleePartialMovement` | `src/qnt_adapters/battle_runtime_command_ordering.rs` | `covered` |
| `cleanroom-input/qnt/battle-runtime/battle-runtime-command-ordering.mbt.qnt#step:doFleeNoMovement` | `tasks/target-replay-evidence/L15-RR06-spell-effect-routes.json#L15-RR06-BATTLE-SPELL-EFFECT-ROUTES replay action=doFleeNoMovement#step:doFleeNoMovement` | `src/qnt_adapters/battle_runtime_command_ordering.rs` | `covered` |
| `cleanroom-input/qnt/battle-runtime/battle-runtime-command-ordering.mbt.qnt#step:doFleeOpportunityAttack` | `tasks/target-replay-evidence/L15-RR06-spell-effect-routes.json#L15-RR06-BATTLE-SPELL-EFFECT-ROUTES replay action=doFleeOpportunityAttack#step:doFleeOpportunityAttack` | `src/qnt_adapters/battle_runtime_command_ordering.rs` | `covered` |
| `cleanroom-input/qnt/battle-runtime/battle-runtime-scalar-buff.mbt.qnt#step:doFillLongstriderTarget` | `tasks/target-replay-evidence/L15-RR06-spell-effect-routes.json#L15-RR06-BATTLE-SPELL-EFFECT-ROUTES replay action=doFillLongstriderTarget#step:doFillLongstriderTarget` | `src/qnt_adapters/battle_runtime_scalar_buff.rs` | `covered` |
| `cleanroom-input/qnt/battle-runtime/battle-runtime-scalar-buff.mbt.qnt#step:doRejectStaleAfterResolved` | `tasks/target-replay-evidence/L15-RR06-spell-effect-routes.json#L15-RR06-BATTLE-SPELL-EFFECT-ROUTES replay action=doRejectStaleAfterResolved#step:doRejectStaleAfterResolved` | `src/qnt_adapters/battle_runtime_scalar_buff.rs` | `covered` |
| `cleanroom-input/qnt/battle-runtime/rule-core-ability-skill-command.mbt.qnt#step:doCommandCastGrovel` | `tasks/target-replay-evidence/L15-RR04C-RULE-CORE-SPELL-ABILITY-COMPONENTS-ability-skill-command.json#L15-RR04C-RULE-CORE-SPELL-ABILITY-COMPONENTS replay action=doCommandCastGrovel#step:doCommandCastGrovel` | `src/qnt_adapters/rule_core_ability_skill_command_mbt.rs` | `covered` |
| `cleanroom-input/qnt/battle-runtime/rule-core-ability-skill-command.mbt.qnt#step:doCommandFollowApproachContinues` | `tasks/target-replay-evidence/L15-RR04C-RULE-CORE-SPELL-ABILITY-COMPONENTS-ability-skill-command.json#L15-RR04C-RULE-CORE-SPELL-ABILITY-COMPONENTS replay action=doCommandFollowApproachContinues#step:doCommandFollowApproachContinues` | `src/qnt_adapters/rule_core_ability_skill_command_mbt.rs` | `covered` |
| `cleanroom-input/qnt/battle-runtime/rule-core-ability-skill-command.mbt.qnt#step:doCommandFollowApproachNoMovement` | `tasks/target-replay-evidence/L15-RR04C-RULE-CORE-SPELL-ABILITY-COMPONENTS-ability-skill-command.json#L15-RR04C-RULE-CORE-SPELL-ABILITY-COMPONENTS replay action=doCommandFollowApproachNoMovement#step:doCommandFollowApproachNoMovement` | `src/qnt_adapters/rule_core_ability_skill_command_mbt.rs` | `covered` |
| `cleanroom-input/qnt/battle-runtime/rule-core-ability-skill-command.mbt.qnt#step:doCommandFollowApproachWithinFive` | `tasks/target-replay-evidence/L15-RR04C-RULE-CORE-SPELL-ABILITY-COMPONENTS-ability-skill-command.json#L15-RR04C-RULE-CORE-SPELL-ABILITY-COMPONENTS replay action=doCommandFollowApproachWithinFive#step:doCommandFollowApproachWithinFive` | `src/qnt_adapters/rule_core_ability_skill_command_mbt.rs` | `covered` |
| `cleanroom-input/qnt/battle-runtime/rule-core-ability-skill-command.mbt.qnt#step:doCommandFollowDrop` | `tasks/target-replay-evidence/L15-RR04C-RULE-CORE-SPELL-ABILITY-COMPONENTS-ability-skill-command.json#L15-RR04C-RULE-CORE-SPELL-ABILITY-COMPONENTS replay action=doCommandFollowDrop#step:doCommandFollowDrop` | `src/qnt_adapters/rule_core_ability_skill_command_mbt.rs` | `covered` |
| `cleanroom-input/qnt/battle-runtime/rule-core-ability-skill-command.mbt.qnt#step:doCommandFollowFlee` | `tasks/target-replay-evidence/L15-RR04C-RULE-CORE-SPELL-ABILITY-COMPONENTS-ability-skill-command.json#L15-RR04C-RULE-CORE-SPELL-ABILITY-COMPONENTS replay action=doCommandFollowFlee#step:doCommandFollowFlee` | `src/qnt_adapters/rule_core_ability_skill_command_mbt.rs` | `covered` |
| `cleanroom-input/qnt/battle-runtime/rule-core-ability-skill-command.mbt.qnt#step:doCommandFollowFleeNoMovement` | `tasks/target-replay-evidence/L15-RR04C-RULE-CORE-SPELL-ABILITY-COMPONENTS-ability-skill-command.json#L15-RR04C-RULE-CORE-SPELL-ABILITY-COMPONENTS replay action=doCommandFollowFleeNoMovement#step:doCommandFollowFleeNoMovement` | `src/qnt_adapters/rule_core_ability_skill_command_mbt.rs` | `covered` |
| `cleanroom-input/qnt/battle-runtime/rule-core-ability-skill-command.mbt.qnt#step:doCommandFollowFleeOpportunityAttack` | `tasks/target-replay-evidence/L15-RR04C-RULE-CORE-SPELL-ABILITY-COMPONENTS-ability-skill-command.json#L15-RR04C-RULE-CORE-SPELL-ABILITY-COMPONENTS replay action=doCommandFollowFleeOpportunityAttack#step:doCommandFollowFleeOpportunityAttack` | `src/qnt_adapters/rule_core_ability_skill_command_mbt.rs` | `covered` |
| `cleanroom-input/qnt/battle-runtime/rule-core-ability-skill-command.mbt.qnt#step:doCommandFollowFleePartialRejected` | `tasks/target-replay-evidence/L15-RR04C-RULE-CORE-SPELL-ABILITY-COMPONENTS-ability-skill-command.json#L15-RR04C-RULE-CORE-SPELL-ABILITY-COMPONENTS replay action=doCommandFollowFleePartialRejected#step:doCommandFollowFleePartialRejected` | `src/qnt_adapters/rule_core_ability_skill_command_mbt.rs` | `covered` |
| `cleanroom-input/qnt/battle-runtime/rule-core-ability-skill-command.mbt.qnt#step:doCommandFollowGrovel` | `tasks/target-replay-evidence/L15-RR04C-RULE-CORE-SPELL-ABILITY-COMPONENTS-ability-skill-command.json#L15-RR04C-RULE-CORE-SPELL-ABILITY-COMPONENTS replay action=doCommandFollowGrovel#step:doCommandFollowGrovel` | `src/qnt_adapters/rule_core_ability_skill_command_mbt.rs` | `covered` |
| `cleanroom-input/qnt/battle-runtime/rule-core-ability-skill-command.mbt.qnt#step:doCommandHaltSuppresses` | `tasks/target-replay-evidence/L15-RR04C-RULE-CORE-SPELL-ABILITY-COMPONENTS-ability-skill-command.json#L15-RR04C-RULE-CORE-SPELL-ABILITY-COMPONENTS replay action=doCommandHaltSuppresses#step:doCommandHaltSuppresses` | `src/qnt_adapters/rule_core_ability_skill_command_mbt.rs` | `covered` |
| `cleanroom-input/qnt/battle-runtime/rule-core-ability-skill-command.mbt.qnt#step:doEnhanceAbilityChoice` | `tasks/target-replay-evidence/L15-RR04C-RULE-CORE-SPELL-ABILITY-COMPONENTS-ability-skill-command.json#L15-RR04C-RULE-CORE-SPELL-ABILITY-COMPONENTS replay action=doEnhanceAbilityChoice#step:doEnhanceAbilityChoice` | `src/qnt_adapters/rule_core_ability_skill_command_mbt.rs` | `covered` |
| `cleanroom-input/qnt/battle-runtime/rule-core-ability-skill-command.mbt.qnt#step:doGuidanceSkillAcrobatics` | `tasks/target-replay-evidence/L15-RR04C-RULE-CORE-SPELL-ABILITY-COMPONENTS-ability-skill-command.json#L15-RR04C-RULE-CORE-SPELL-ABILITY-COMPONENTS replay action=doGuidanceSkillAcrobatics#step:doGuidanceSkillAcrobatics` | `src/qnt_adapters/rule_core_ability_skill_command_mbt.rs` | `covered` |
| `cleanroom-input/qnt/battle-runtime/rule-core-ability-skill-command.mbt.qnt#step:doGuidanceSkillAnimalHandling` | `tasks/target-replay-evidence/L15-RR04C-RULE-CORE-SPELL-ABILITY-COMPONENTS-ability-skill-command.json#L15-RR04C-RULE-CORE-SPELL-ABILITY-COMPONENTS replay action=doGuidanceSkillAnimalHandling#step:doGuidanceSkillAnimalHandling` | `src/qnt_adapters/rule_core_ability_skill_command_mbt.rs` | `covered` |
| `cleanroom-input/qnt/battle-runtime/rule-core-ability-skill-command.mbt.qnt#step:doGuidanceSkillArcana` | `tasks/target-replay-evidence/L15-RR04C-RULE-CORE-SPELL-ABILITY-COMPONENTS-ability-skill-command.json#L15-RR04C-RULE-CORE-SPELL-ABILITY-COMPONENTS replay action=doGuidanceSkillArcana#step:doGuidanceSkillArcana` | `src/qnt_adapters/rule_core_ability_skill_command_mbt.rs` | `covered` |
| `cleanroom-input/qnt/battle-runtime/rule-core-ability-skill-command.mbt.qnt#step:doGuidanceSkillAthletics` | `tasks/target-replay-evidence/L15-RR04C-RULE-CORE-SPELL-ABILITY-COMPONENTS-ability-skill-command.json#L15-RR04C-RULE-CORE-SPELL-ABILITY-COMPONENTS replay action=doGuidanceSkillAthletics#step:doGuidanceSkillAthletics` | `src/qnt_adapters/rule_core_ability_skill_command_mbt.rs` | `covered` |
| `cleanroom-input/qnt/battle-runtime/rule-core-ability-skill-command.mbt.qnt#step:doGuidanceSkillDeception` | `tasks/target-replay-evidence/L15-RR04C-RULE-CORE-SPELL-ABILITY-COMPONENTS-ability-skill-command.json#L15-RR04C-RULE-CORE-SPELL-ABILITY-COMPONENTS replay action=doGuidanceSkillDeception#step:doGuidanceSkillDeception` | `src/qnt_adapters/rule_core_ability_skill_command_mbt.rs` | `covered` |
| `cleanroom-input/qnt/battle-runtime/rule-core-ability-skill-command.mbt.qnt#step:doGuidanceSkillHistory` | `tasks/target-replay-evidence/L15-RR04C-RULE-CORE-SPELL-ABILITY-COMPONENTS-ability-skill-command.json#L15-RR04C-RULE-CORE-SPELL-ABILITY-COMPONENTS replay action=doGuidanceSkillHistory#step:doGuidanceSkillHistory` | `src/qnt_adapters/rule_core_ability_skill_command_mbt.rs` | `covered` |
| `cleanroom-input/qnt/battle-runtime/rule-core-ability-skill-command.mbt.qnt#step:doGuidanceSkillInsight` | `tasks/target-replay-evidence/L15-RR04C-RULE-CORE-SPELL-ABILITY-COMPONENTS-ability-skill-command.json#L15-RR04C-RULE-CORE-SPELL-ABILITY-COMPONENTS replay action=doGuidanceSkillInsight#step:doGuidanceSkillInsight` | `src/qnt_adapters/rule_core_ability_skill_command_mbt.rs` | `covered` |
| `cleanroom-input/qnt/battle-runtime/rule-core-ability-skill-command.mbt.qnt#step:doGuidanceSkillIntimidation` | `tasks/target-replay-evidence/L15-RR04C-RULE-CORE-SPELL-ABILITY-COMPONENTS-ability-skill-command.json#L15-RR04C-RULE-CORE-SPELL-ABILITY-COMPONENTS replay action=doGuidanceSkillIntimidation#step:doGuidanceSkillIntimidation` | `src/qnt_adapters/rule_core_ability_skill_command_mbt.rs` | `covered` |
| `cleanroom-input/qnt/battle-runtime/rule-core-ability-skill-command.mbt.qnt#step:doGuidanceSkillInvestigation` | `tasks/target-replay-evidence/L15-RR04C-RULE-CORE-SPELL-ABILITY-COMPONENTS-ability-skill-command.json#L15-RR04C-RULE-CORE-SPELL-ABILITY-COMPONENTS replay action=doGuidanceSkillInvestigation#step:doGuidanceSkillInvestigation` | `src/qnt_adapters/rule_core_ability_skill_command_mbt.rs` | `covered` |
| `cleanroom-input/qnt/battle-runtime/rule-core-ability-skill-command.mbt.qnt#step:doGuidanceSkillMedicine` | `tasks/target-replay-evidence/L15-RR04C-RULE-CORE-SPELL-ABILITY-COMPONENTS-ability-skill-command.json#L15-RR04C-RULE-CORE-SPELL-ABILITY-COMPONENTS replay action=doGuidanceSkillMedicine#step:doGuidanceSkillMedicine` | `src/qnt_adapters/rule_core_ability_skill_command_mbt.rs` | `covered` |
| `cleanroom-input/qnt/battle-runtime/rule-core-ability-skill-command.mbt.qnt#step:doGuidanceSkillNature` | `tasks/target-replay-evidence/L15-RR04C-RULE-CORE-SPELL-ABILITY-COMPONENTS-ability-skill-command.json#L15-RR04C-RULE-CORE-SPELL-ABILITY-COMPONENTS replay action=doGuidanceSkillNature#step:doGuidanceSkillNature` | `src/qnt_adapters/rule_core_ability_skill_command_mbt.rs` | `covered` |
| `cleanroom-input/qnt/battle-runtime/rule-core-ability-skill-command.mbt.qnt#step:doGuidanceSkillPerception` | `tasks/target-replay-evidence/L15-RR04C-RULE-CORE-SPELL-ABILITY-COMPONENTS-ability-skill-command.json#L15-RR04C-RULE-CORE-SPELL-ABILITY-COMPONENTS replay action=doGuidanceSkillPerception#step:doGuidanceSkillPerception` | `src/qnt_adapters/rule_core_ability_skill_command_mbt.rs` | `covered` |
| `cleanroom-input/qnt/battle-runtime/rule-core-ability-skill-command.mbt.qnt#step:doGuidanceSkillPerformance` | `tasks/target-replay-evidence/L15-RR04C-RULE-CORE-SPELL-ABILITY-COMPONENTS-ability-skill-command.json#L15-RR04C-RULE-CORE-SPELL-ABILITY-COMPONENTS replay action=doGuidanceSkillPerformance#step:doGuidanceSkillPerformance` | `src/qnt_adapters/rule_core_ability_skill_command_mbt.rs` | `covered` |
| `cleanroom-input/qnt/battle-runtime/rule-core-ability-skill-command.mbt.qnt#step:doGuidanceSkillPersuasion` | `tasks/target-replay-evidence/L15-RR04C-RULE-CORE-SPELL-ABILITY-COMPONENTS-ability-skill-command.json#L15-RR04C-RULE-CORE-SPELL-ABILITY-COMPONENTS replay action=doGuidanceSkillPersuasion#step:doGuidanceSkillPersuasion` | `src/qnt_adapters/rule_core_ability_skill_command_mbt.rs` | `covered` |
| `cleanroom-input/qnt/battle-runtime/rule-core-ability-skill-command.mbt.qnt#step:doGuidanceSkillReligion` | `tasks/target-replay-evidence/L15-RR04C-RULE-CORE-SPELL-ABILITY-COMPONENTS-ability-skill-command.json#L15-RR04C-RULE-CORE-SPELL-ABILITY-COMPONENTS replay action=doGuidanceSkillReligion#step:doGuidanceSkillReligion` | `src/qnt_adapters/rule_core_ability_skill_command_mbt.rs` | `covered` |
| `cleanroom-input/qnt/battle-runtime/rule-core-ability-skill-command.mbt.qnt#step:doGuidanceSkillSleightOfHand` | `tasks/target-replay-evidence/L15-RR04C-RULE-CORE-SPELL-ABILITY-COMPONENTS-ability-skill-command.json#L15-RR04C-RULE-CORE-SPELL-ABILITY-COMPONENTS replay action=doGuidanceSkillSleightOfHand#step:doGuidanceSkillSleightOfHand` | `src/qnt_adapters/rule_core_ability_skill_command_mbt.rs` | `covered` |
| `cleanroom-input/qnt/battle-runtime/rule-core-ability-skill-command.mbt.qnt#step:doGuidanceSkillStealth` | `tasks/target-replay-evidence/L15-RR04C-RULE-CORE-SPELL-ABILITY-COMPONENTS-ability-skill-command.json#L15-RR04C-RULE-CORE-SPELL-ABILITY-COMPONENTS replay action=doGuidanceSkillStealth#step:doGuidanceSkillStealth` | `src/qnt_adapters/rule_core_ability_skill_command_mbt.rs` | `covered` |
| `cleanroom-input/qnt/battle-runtime/rule-core-ability-skill-command.mbt.qnt#step:doGuidanceSkillSurvival` | `tasks/target-replay-evidence/L15-RR04C-RULE-CORE-SPELL-ABILITY-COMPONENTS-ability-skill-command.json#L15-RR04C-RULE-CORE-SPELL-ABILITY-COMPONENTS replay action=doGuidanceSkillSurvival#step:doGuidanceSkillSurvival` | `src/qnt_adapters/rule_core_ability_skill_command_mbt.rs` | `covered` |
| `cleanroom-input/qnt/battle-runtime/rule-core-ability-skill-command.mbt.qnt#step:doSearchFails` | `tasks/target-replay-evidence/L15-RR04C-RULE-CORE-SPELL-ABILITY-COMPONENTS-ability-skill-command.json#L15-RR04C-RULE-CORE-SPELL-ABILITY-COMPONENTS replay action=doSearchFails#step:doSearchFails` | `src/qnt_adapters/rule_core_ability_skill_command_mbt.rs` | `covered` |
| `cleanroom-input/qnt/battle-runtime/rule-core-ability-skill-command.mbt.qnt#step:doSearchSucceeds` | `tasks/target-replay-evidence/L15-RR04C-RULE-CORE-SPELL-ABILITY-COMPONENTS-ability-skill-command.json#L15-RR04C-RULE-CORE-SPELL-ABILITY-COMPONENTS replay action=doSearchSucceeds#step:doSearchSucceeds` | `src/qnt_adapters/rule_core_ability_skill_command_mbt.rs` | `covered` |
| `cleanroom-input/qnt/battle-runtime/rule-core-spells.mbt.qnt#step:doAcidSplashAllSuccess` | `tasks/target-replay-evidence/L15-RR04C-RULE-CORE-SPELL-ABILITY-COMPONENTS-spells.json#L15-RR04C-RULE-CORE-SPELL-ABILITY-COMPONENTS replay action=doAcidSplashAllSuccess#step:doAcidSplashAllSuccess` | `src/qnt_adapters/rule_core_spells_mbt.rs` | `covered` |
| `cleanroom-input/qnt/battle-runtime/rule-core-spells.mbt.qnt#step:doAcidSplashNeedsDamageRoll` | `tasks/target-replay-evidence/L15-RR04C-RULE-CORE-SPELL-ABILITY-COMPONENTS-spells.json#L15-RR04C-RULE-CORE-SPELL-ABILITY-COMPONENTS replay action=doAcidSplashNeedsDamageRoll#step:doAcidSplashNeedsDamageRoll` | `src/qnt_adapters/rule_core_spells_mbt.rs` | `covered` |
| `cleanroom-input/qnt/battle-runtime/rule-core-spells.mbt.qnt#step:doAcidSplashNeedsSavingThrow` | `tasks/target-replay-evidence/L15-RR04C-RULE-CORE-SPELL-ABILITY-COMPONENTS-spells.json#L15-RR04C-RULE-CORE-SPELL-ABILITY-COMPONENTS replay action=doAcidSplashNeedsSavingThrow#step:doAcidSplashNeedsSavingThrow` | `src/qnt_adapters/rule_core_spells_mbt.rs` | `covered` |
| `cleanroom-input/qnt/battle-runtime/rule-core-spells.mbt.qnt#step:doAcidSplashOneFail` | `tasks/target-replay-evidence/L15-RR04C-RULE-CORE-SPELL-ABILITY-COMPONENTS-spells.json#L15-RR04C-RULE-CORE-SPELL-ABILITY-COMPONENTS replay action=doAcidSplashOneFail#step:doAcidSplashOneFail` | `src/qnt_adapters/rule_core_spells_mbt.rs` | `covered` |
| `cleanroom-input/qnt/battle-runtime/rule-core-spells.mbt.qnt#step:doCureWoundsNeedsHealingRoll` | `tasks/target-replay-evidence/L15-RR04C-RULE-CORE-SPELL-ABILITY-COMPONENTS-spells.json#L15-RR04C-RULE-CORE-SPELL-ABILITY-COMPONENTS replay action=doCureWoundsNeedsHealingRoll#step:doCureWoundsNeedsHealingRoll` | `src/qnt_adapters/rule_core_spells_mbt.rs` | `covered` |
| `cleanroom-input/qnt/battle-runtime/rule-core-spells.mbt.qnt#step:doCureWoundsNeedsTarget` | `tasks/target-replay-evidence/L15-RR04C-RULE-CORE-SPELL-ABILITY-COMPONENTS-spells.json#L15-RR04C-RULE-CORE-SPELL-ABILITY-COMPONENTS replay action=doCureWoundsNeedsTarget#step:doCureWoundsNeedsTarget` | `src/qnt_adapters/rule_core_spells_mbt.rs` | `covered` |
| `cleanroom-input/qnt/battle-runtime/rule-core-spells.mbt.qnt#step:doCureWoundsWounded` | `tasks/target-replay-evidence/L15-RR04C-RULE-CORE-SPELL-ABILITY-COMPONENTS-spells.json#L15-RR04C-RULE-CORE-SPELL-ABILITY-COMPONENTS replay action=doCureWoundsWounded#step:doCureWoundsWounded` | `src/qnt_adapters/rule_core_spells_mbt.rs` | `covered` |
| `cleanroom-input/qnt/battle-runtime/rule-core-spells.mbt.qnt#step:doHealingWordNeedsHealingRoll` | `tasks/target-replay-evidence/L15-RR04C-RULE-CORE-SPELL-ABILITY-COMPONENTS-spells.json#L15-RR04C-RULE-CORE-SPELL-ABILITY-COMPONENTS replay action=doHealingWordNeedsHealingRoll#step:doHealingWordNeedsHealingRoll` | `src/qnt_adapters/rule_core_spells_mbt.rs` | `covered` |
| `cleanroom-input/qnt/battle-runtime/rule-core-spells.mbt.qnt#step:doHealingWordNeedsTarget` | `tasks/target-replay-evidence/L15-RR04C-RULE-CORE-SPELL-ABILITY-COMPONENTS-spells.json#L15-RR04C-RULE-CORE-SPELL-ABILITY-COMPONENTS replay action=doHealingWordNeedsTarget#step:doHealingWordNeedsTarget` | `src/qnt_adapters/rule_core_spells_mbt.rs` | `covered` |
| `cleanroom-input/qnt/battle-runtime/rule-core-spells.mbt.qnt#step:doHealingWordWounded` | `tasks/target-replay-evidence/L15-RR04C-RULE-CORE-SPELL-ABILITY-COMPONENTS-spells.json#L15-RR04C-RULE-CORE-SPELL-ABILITY-COMPONENTS replay action=doHealingWordWounded#step:doHealingWordWounded` | `src/qnt_adapters/rule_core_spells_mbt.rs` | `covered` |
| `cleanroom-input/qnt/battle-runtime/rule-core-spells.mbt.qnt#step:doHealingWordZeroHp` | `tasks/target-replay-evidence/L15-RR04C-RULE-CORE-SPELL-ABILITY-COMPONENTS-spells.json#L15-RR04C-RULE-CORE-SPELL-ABILITY-COMPONENTS replay action=doHealingWordZeroHp#step:doHealingWordZeroHp` | `src/qnt_adapters/rule_core_spells_mbt.rs` | `covered` |
| `cleanroom-input/qnt/battle-runtime/rule-core-spells.mbt.qnt#step:doMageArmor` | `tasks/target-replay-evidence/L15-RR04C-RULE-CORE-SPELL-ABILITY-COMPONENTS-spells.json#L15-RR04C-RULE-CORE-SPELL-ABILITY-COMPONENTS replay action=doMageArmor#step:doMageArmor` | `src/qnt_adapters/rule_core_spells_mbt.rs` | `covered` |
| `cleanroom-input/qnt/battle-runtime/rule-core-spells.mbt.qnt#step:doMageArmorNeedsTarget` | `tasks/target-replay-evidence/L15-RR04C-RULE-CORE-SPELL-ABILITY-COMPONENTS-spells.json#L15-RR04C-RULE-CORE-SPELL-ABILITY-COMPONENTS replay action=doMageArmorNeedsTarget#step:doMageArmorNeedsTarget` | `src/qnt_adapters/rule_core_spells_mbt.rs` | `covered` |
| `cleanroom-input/qnt/battle-runtime/rule-core-spells.mbt.qnt#step:doMagicMissileLow` | `tasks/target-replay-evidence/L15-RR04C-RULE-CORE-SPELL-ABILITY-COMPONENTS-spells.json#L15-RR04C-RULE-CORE-SPELL-ABILITY-COMPONENTS replay action=doMagicMissileLow#step:doMagicMissileLow` | `src/qnt_adapters/rule_core_spells_mbt.rs` | `covered` |
| `cleanroom-input/qnt/battle-runtime/rule-core-spells.mbt.qnt#step:doMagicMissileNeedsAllocation` | `tasks/target-replay-evidence/L15-RR04C-RULE-CORE-SPELL-ABILITY-COMPONENTS-spells.json#L15-RR04C-RULE-CORE-SPELL-ABILITY-COMPONENTS replay action=doMagicMissileNeedsAllocation#step:doMagicMissileNeedsAllocation` | `src/qnt_adapters/rule_core_spells_mbt.rs` | `covered` |
| `cleanroom-input/qnt/battle-runtime/rule-core-spells.mbt.qnt#step:doRayOfFrostCritical` | `tasks/target-replay-evidence/L15-RR04C-RULE-CORE-SPELL-ABILITY-COMPONENTS-spells.json#L15-RR04C-RULE-CORE-SPELL-ABILITY-COMPONENTS replay action=doRayOfFrostCritical#step:doRayOfFrostCritical` | `src/qnt_adapters/rule_core_spells_mbt.rs` | `covered` |
| `cleanroom-input/qnt/battle-runtime/rule-core-spells.mbt.qnt#step:doRayOfFrostHit` | `tasks/target-replay-evidence/L15-RR04C-RULE-CORE-SPELL-ABILITY-COMPONENTS-spells.json#L15-RR04C-RULE-CORE-SPELL-ABILITY-COMPONENTS replay action=doRayOfFrostHit#step:doRayOfFrostHit` | `src/qnt_adapters/rule_core_spells_mbt.rs` | `covered` |
| `cleanroom-input/qnt/battle-runtime/rule-core-spells.mbt.qnt#step:doRayOfFrostMiss` | `tasks/target-replay-evidence/L15-RR04C-RULE-CORE-SPELL-ABILITY-COMPONENTS-spells.json#L15-RR04C-RULE-CORE-SPELL-ABILITY-COMPONENTS replay action=doRayOfFrostMiss#step:doRayOfFrostMiss` | `src/qnt_adapters/rule_core_spells_mbt.rs` | `covered` |
| `cleanroom-input/qnt/battle-runtime/rule-core-spells.mbt.qnt#step:doRayOfFrostNeedsAttackRoll` | `tasks/target-replay-evidence/L15-RR04C-RULE-CORE-SPELL-ABILITY-COMPONENTS-spells.json#L15-RR04C-RULE-CORE-SPELL-ABILITY-COMPONENTS replay action=doRayOfFrostNeedsAttackRoll#step:doRayOfFrostNeedsAttackRoll` | `src/qnt_adapters/rule_core_spells_mbt.rs` | `covered` |
| `cleanroom-input/qnt/battle-runtime/rule-core-spells.mbt.qnt#step:doRayOfFrostNeedsDamageRoll` | `tasks/target-replay-evidence/L15-RR04C-RULE-CORE-SPELL-ABILITY-COMPONENTS-spells.json#L15-RR04C-RULE-CORE-SPELL-ABILITY-COMPONENTS replay action=doRayOfFrostNeedsDamageRoll#step:doRayOfFrostNeedsDamageRoll` | `src/qnt_adapters/rule_core_spells_mbt.rs` | `covered` |
| `cleanroom-input/qnt/battle-runtime/rule-core-spells.mbt.qnt#step:doRayOfFrostNeedsTarget` | `tasks/target-replay-evidence/L15-RR04C-RULE-CORE-SPELL-ABILITY-COMPONENTS-spells.json#L15-RR04C-RULE-CORE-SPELL-ABILITY-COMPONENTS replay action=doRayOfFrostNeedsTarget#step:doRayOfFrostNeedsTarget` | `src/qnt_adapters/rule_core_spells_mbt.rs` | `covered` |
| `cleanroom-input/qnt/battle-runtime/rule-core-spells.mbt.qnt#step:doReadySpellHold` | `tasks/target-replay-evidence/L15-RR04C-RULE-CORE-SPELL-ABILITY-COMPONENTS-spells.json#L15-RR04C-RULE-CORE-SPELL-ABILITY-COMPONENTS replay action=doReadySpellHold#step:doReadySpellHold` | `src/qnt_adapters/rule_core_spells_mbt.rs` | `covered` |
| `cleanroom-input/qnt/battle-runtime/rule-core-spells.mbt.qnt#step:doRejectSecondSlotSpell` | `tasks/target-replay-evidence/L15-RR04C-RULE-CORE-SPELL-ABILITY-COMPONENTS-spells.json#L15-RR04C-RULE-CORE-SPELL-ABILITY-COMPONENTS replay action=doRejectSecondSlotSpell#step:doRejectSecondSlotSpell` | `src/qnt_adapters/rule_core_spells_mbt.rs` | `covered` |
| `cleanroom-input/qnt/battle-runtime/rule-core-spells.mbt.qnt#step:doReleaseReadiedSpell` | `tasks/target-replay-evidence/L15-RR04C-RULE-CORE-SPELL-ABILITY-COMPONENTS-spells.json#L15-RR04C-RULE-CORE-SPELL-ABILITY-COMPONENTS replay action=doReleaseReadiedSpell#step:doReleaseReadiedSpell` | `src/qnt_adapters/rule_core_spells_mbt.rs` | `covered` |

Target replay evidence:

- T001: `tasks/target-replay-evidence/T001-battle-runtime-magic-missile.json` (2 accepted refs, sha256 `0a77cdcaef21d43d60b8d11582bad04661dc5f3fa60e21854847c0ce7884818c`)
- T002: `tasks/target-replay-evidence/T002-battle-runtime-save-gated-spell-ordering.json` (10 accepted refs, sha256 `71f2eed1ef989bbf51d4f182c644a6970d60103b937dce176fd46b0bd7ce8dba`)
- T003: `tasks/target-replay-evidence/T003-battle-runtime-hit-point-restoration-ordering.json` (9 accepted refs, sha256 `ed5edc22b8fe08fd5dce2331848ee0dea3d29405a9060c7629a59f75155eb162`)
- T004: `tasks/target-replay-evidence/T004-battle-runtime-death-saving-throw.json` (3 accepted refs, sha256 `ae3d1b14dccbc63d2c9b18543a0c0b1911088bca5e2ebfbee47e5a3c48821f60`)
- T005: `tasks/target-replay-evidence/T005-battle-runtime-concentration-break-teardown.json` (5 accepted refs, sha256 `ab636fe7b0ff575f60f5693311c4c2bba0a96d0a9d8f6273ba0be0bd6f4edfef`)
- RRCONV-19A-RUST-BATTLE-SETUP-ENTRYPOINT: `tasks/target-replay-evidence/RRCONV-19A-battle-setup-entrypoint.json` (14 accepted refs, sha256 `b4cc845f9313a967e07375be4b6739f69295966aa98ebeb67f8832e1ca55856a`)
- RRCONV-19B-RUST-ACT-DISCOVERY-CONTRACT: `tasks/target-replay-evidence/RRCONV-19B-act-discovery-contract.json` (14 accepted refs, sha256 `b55669c41913df311137a48883eacdc1281e34d0524f82c58f46873526c4dbeb`)
- RRCONV-19D-RUST-TURN-ADVANCE-RESULT: `tasks/target-replay-evidence/RRCONV-19D-turn-advance-result.json` (9 accepted refs, sha256 `8e49deae4fc74dc80ddbfe4a2561b28d3aa0379915e11e24de93efaa34c76a2b`)
- RRCONV-19C-RUST-RESOLUTION-RESULT-CONTRACT: `tasks/target-replay-evidence/RRCONV-19C-resolution-result-contract.json` (9 accepted refs, sha256 `9724ea3f33400555280188cfb5365777d997bad933da7453d9f2ab1bc121ed11`)
- RRCONV-19E-RUST-END-TURN-SUBJECT-RESOLUTION: `tasks/target-replay-evidence/RRCONV-19E-end-turn-subject-resolution.json` (9 accepted refs, sha256 `58c33c9c6292a1d55b55e2c415a7fd30dd5c3580832c6a56ad579253288728a0`)
- RRCONV-19F-RUST-ROUTE-EVENT-FROM-REDUCER-RESULT: `tasks/target-replay-evidence/RRCONV-19F-route-event-from-reducer-result.json` (12 accepted refs, sha256 `b672f928594babb75fe6fa752e18d17d7841d84e11268a5eaee7193c127300c5`)
- RRCONV-19G-RUST-SUBJECT-CONTINUATION-LIFECYCLE: `tasks/target-replay-evidence/RRCONV-19G-subject-continuation-lifecycle.json` (19 accepted refs, sha256 `4df2e02ca93e388fe6cd4d95c34f8262f1a0e327d05e126e34e327058b364181`)
- L15-RR03-FINISH-CURRENT-DIAGNOSTIC-QUEUE: `tasks/target-replay-evidence/L15-RR03-reducer-route.json` (8 accepted refs, sha256 `dab4e9c67f4cd1fdfa486c9dfc4aeec0c3e9c0c6ca61905bb20f9bd2d922bec0`)
- L15-RR05-BATTLE-ACTION-ATTACK-STATBLOCK-ROUTES: `tasks/target-replay-evidence/L15-RR05-attack-statblock-routes.json` (53 accepted refs, sha256 `d9b846e463d07cc131e3daafd5320f30eb4a3ce37a77eb2ebac4ad667c525759`)
- L15-RR06-BATTLE-SPELL-EFFECT-ROUTES: `tasks/target-replay-evidence/L15-RR06-spell-effect-routes.json` (36 accepted refs, sha256 `6349cedd7dd73748fe3e31e98953c602a4f17e59107733d35d4f48676dc2468d`)
- L15-RR04C-RULE-CORE-SPELL-ABILITY-COMPONENTS: `tasks/target-replay-evidence/L15-RR04C-RULE-CORE-SPELL-ABILITY-COMPONENTS-ability-skill-command.json` (32 accepted refs, sha256 `52cc18dc94fd282ace837b4073bd2e636968f16857a56ab5bec3c48fc59602ba`)
- L15-RR04C-RULE-CORE-SPELL-ABILITY-COMPONENTS: `tasks/target-replay-evidence/L15-RR04C-RULE-CORE-SPELL-ABILITY-COMPONENTS-spells.json` (24 accepted refs, sha256 `371a98f64b53a35e69c13992342cbc77cda94406dac2448726d2810ff271ea88`)

Harness artifacts:

- deciderDecision: `tasks/history/L15-RR04C-RULE-CORE-SPELL-ABILITY-COMPONENTS/DECIDER_DECISION.json` (sha256 `5311c8cbd29af2a649606ddff2f2400be08adb43fe096da1841576f2d32b4e44`)
- engineDepth: `tasks/history/L15-RR04C-RULE-CORE-SPELL-ABILITY-COMPONENTS/ENGINE_DEPTH_MANIFEST.json` (sha256 `151c771427a312805565739b6937d106c4915b7be2ed3e53fe01ff5179adb0d0`)
- reviewLoop: `tasks/history/L15-RR04C-RULE-CORE-SPELL-ABILITY-COMPONENTS/REVIEW_LOOP.json` (sha256 `a909c690742cb557d80efe48a568552889066ec9f30f426e5e810f0e51dac79b`)
- startGate: `tasks/history/L15-RR04C-RULE-CORE-SPELL-ABILITY-COMPONENTS/START_GATE.json` (sha256 `420c51d37f78a798c447117a040d249364d34c9df04d108e0197beedcc829874`)
- stateOwnerManifest: `tasks/history/L15-RR04C-RULE-CORE-SPELL-ABILITY-COMPONENTS/STATE_OWNER_MANIFEST.json` (sha256 `d9829fe6458a57eeef59205a809f47cfac8470d62f21c0cf4b332e4651360bd0`)

Remaining gaps:

- L15-RR04C-RULE-CORE-SPELL-ABILITY-COMPONENTS: none. Six source-inventory spell rows are out-of-scope for accepted replay evidence and are listed in `tasks/target-replay-evidence/L15-RR04C-RULE-CORE-SPELL-ABILITY-COMPONENTS-spells.json` source metadata; the focused Rust adapter has a separate non-acceptance regression test for them.

Verification results:

- `cargo test ability_skill_command -- --nocapture`: pass.
- `cargo test rule_core_spells -- --nocapture`: pass.
- `node scripts/check-target-replay-evidence-file.cjs --driver cleanroom-input/qnt/battle-runtime/rule-core-ability-skill-command.mbt.qnt --evidence tasks/target-replay-evidence/L15-RR04C-RULE-CORE-SPELL-ABILITY-COMPONENTS-ability-skill-command.json`: pass, 32 obligations covered.
- `node scripts/check-target-replay-evidence-file.cjs --driver cleanroom-input/qnt/battle-runtime/rule-core-spells.mbt.qnt --evidence tasks/target-replay-evidence/L15-RR04C-RULE-CORE-SPELL-ABILITY-COMPONENTS-spells.json`: pass, 24 obligations covered.
- `cargo fmt --check`: pass.
- `cargo test`: pass, 190 tests.
- `cargo clippy --all-targets -- -D warnings`: pass.
- `node scripts/check-cleanroom-harness.cjs`: pass.
- `git diff --check a625d3e7190eb33396c17ee5dca7ae73f413b348...HEAD`: pass.

## L15-RR09-CHARACTER-SHEET-ROUTES: Character Sheet Routes

- Manifest source commit SHA: `564376fd95218a209bb9eae5c9ccb54ca3e04a52`
- Source branch inventory SHA: `4bb2b20a85d94e3b90b7c59cbfe6e1edd5ab3ef40410641e999527861f3d3a32`
- Selected driver: `cleanroom-input/qnt/character-sheet-runtime/character-sheet-ability-check-proficiency-bonus.mbt.qnt`
- Selected driver: `cleanroom-input/qnt/character-sheet-runtime/character-sheet-arcane-recovery-selected-identity.mbt.qnt`
- Selected driver: `cleanroom-input/qnt/character-sheet-runtime/character-sheet-armor-class-base-selected-identity.mbt.qnt`
- Selected driver: `cleanroom-input/qnt/character-sheet-runtime/character-sheet-class-feature-selected-identity.mbt.qnt`
- Selected driver: `cleanroom-input/qnt/character-sheet-runtime/character-sheet-healing-resource-selected-identity.mbt.qnt`
- Selected driver: `cleanroom-input/qnt/character-sheet-runtime/character-sheet-hit-point-maximum.mbt.qnt`
- Selected driver: `cleanroom-input/qnt/character-sheet-runtime/character-sheet-hp-rest-hit-dice.mbt.qnt`
- Selected driver: `cleanroom-input/qnt/character-sheet-runtime/character-sheet-spell-slots-pact-slots.mbt.qnt`
- Selected driver: `cleanroom-input/qnt/character-sheet-runtime/character-sheet-spellbook-ritual-selected-identity.mbt.qnt`
- Selected driver: `cleanroom-input/qnt/character-sheet-runtime/character-sheet-weapon-mastery-containers-selected-identity.mbt.qnt`

Allowed inputs used:

- `cleanroom-input/MANIFEST.md`
- `cleanroom-input/branch-coverage/source-branch-inventory.json`
- `cleanroom-input/branch-coverage/reducer-route-inventory.json`
- `cleanroom-input/qnt/character-sheet-runtime/character-sheet-reducer-route.qnt`
- `cleanroom-input/domain/UBIQUITOUS_LANGUAGE.md`
- `cleanroom-input/domain/CLEANROOM_ASSUMPTIONS.md`
- `cleanroom-input/raw/srd-5.2.1/Character-Creation.md`
- `cleanroom-input/raw/srd-5.2.1/Classes/Wizard.md`
- `cleanroom-input/raw/srd-5.2.1/Classes/Warlock.md`
- `cleanroom-input/raw/srd-5.2.1/Spells/Gaining-and-Casting.md`
- Repo-local `src/**`, `tasks/**`, and Cargo tooling

Behavior implemented:

- Routed selected level-1/2 Character Sheet projections through reusable sheet-owned slot/resource facts and QNT-shaped route events.
- `src/tests/mod.rs` now replays and compares route values for the previously projection-only Ability Check, Healing Resource, Spellbook Ritual, and Weapon Mastery selected drivers, using expected routes transcribed from the copied `.route.mbt.qnt` shapes.
- Kept authored ids in adapters, tests, and evidence payloads; production sheet functions consume shape facts and state projections.

Generated branch coverage:

| Obligation | Target replay evidence | Diagnostic tests | Status |
| --- | --- | --- | --- |
| `cleanroom-input/qnt/character-sheet-runtime/character-sheet-ability-check-proficiency-bonus.mbt.qnt#step:doProjectExpertise` | `tasks/target-replay-evidence/L15-RR09-character-sheet-routes.json#L15-RR09-CHARACTER-SHEET-ROUTES route action=doProjectExpertise#step:doProjectExpertise` | `src/tests/mod.rs` | `covered` |
| `cleanroom-input/qnt/character-sheet-runtime/character-sheet-ability-check-proficiency-bonus.mbt.qnt#step:doProjectJackOfAllTradesLevelTwo` | `tasks/target-replay-evidence/L15-RR09-character-sheet-routes.json#L15-RR09-CHARACTER-SHEET-ROUTES route action=doProjectJackOfAllTradesLevelTwo#step:doProjectJackOfAllTradesLevelTwo` | `src/tests/mod.rs` | `covered` |
| `cleanroom-input/qnt/character-sheet-runtime/character-sheet-ability-check-proficiency-bonus.mbt.qnt#step:doProjectJackOfAllTradesRoundedDown` | `tasks/target-replay-evidence/L15-RR09-character-sheet-routes.json#L15-RR09-CHARACTER-SHEET-ROUTES route action=doProjectJackOfAllTradesRoundedDown#step:doProjectJackOfAllTradesRoundedDown` | `src/tests/mod.rs` | `covered` |
| `cleanroom-input/qnt/character-sheet-runtime/character-sheet-ability-check-proficiency-bonus.mbt.qnt#step:doProjectSkillProficiency` | `tasks/target-replay-evidence/L15-RR09-character-sheet-routes.json#L15-RR09-CHARACTER-SHEET-ROUTES route action=doProjectSkillProficiency#step:doProjectSkillProficiency` | `src/tests/mod.rs` | `covered` |
| `cleanroom-input/qnt/character-sheet-runtime/character-sheet-ability-check-proficiency-bonus.mbt.qnt#step:doRejectMissingBardLevelTwo` | `tasks/target-replay-evidence/L15-RR09-character-sheet-routes.json#L15-RR09-CHARACTER-SHEET-ROUTES route action=doRejectMissingBardLevelTwo#step:doRejectMissingBardLevelTwo` | `src/tests/mod.rs` | `covered` |
| `cleanroom-input/qnt/character-sheet-runtime/character-sheet-ability-check-proficiency-bonus.mbt.qnt#step:doRejectOtherProficiencyBonus` | `tasks/target-replay-evidence/L15-RR09-character-sheet-routes.json#L15-RR09-CHARACTER-SHEET-ROUTES route action=doRejectOtherProficiencyBonus#step:doRejectOtherProficiencyBonus` | `src/tests/mod.rs` | `covered` |
| `cleanroom-input/qnt/character-sheet-runtime/character-sheet-arcane-recovery-selected-identity.mbt.qnt#step:doRecoverSecondLevelSpellSlot` | `tasks/target-replay-evidence/L15-RR09-character-sheet-routes.json#L15-RR09-CHARACTER-SHEET-ROUTES route action=doRecoverSecondLevelSpellSlot#step:doRecoverSecondLevelSpellSlot` | `src/tests/mod.rs` | `covered` |
| `cleanroom-input/qnt/character-sheet-runtime/character-sheet-arcane-recovery-selected-identity.mbt.qnt#step:doRejectPactSlotArcaneRecovery` | `tasks/target-replay-evidence/L15-RR09-character-sheet-routes.json#L15-RR09-CHARACTER-SHEET-ROUTES route action=doRejectPactSlotArcaneRecovery#step:doRejectPactSlotArcaneRecovery` | `src/tests/mod.rs` | `covered` |
| `cleanroom-input/qnt/character-sheet-runtime/character-sheet-arcane-recovery-selected-identity.mbt.qnt#step:doResetArcaneRecoveryOnLongRest` | `tasks/target-replay-evidence/L15-RR09-character-sheet-routes.json#L15-RR09-CHARACTER-SHEET-ROUTES route action=doResetArcaneRecoveryOnLongRest#step:doResetArcaneRecoveryOnLongRest` | `src/tests/mod.rs` | `covered` |
| `cleanroom-input/qnt/character-sheet-runtime/character-sheet-armor-class-base-selected-identity.mbt.qnt#step:doProjectHeavyArmorWithShield` | `tasks/target-replay-evidence/L15-RR09-character-sheet-routes.json#L15-RR09-CHARACTER-SHEET-ROUTES route action=doProjectHeavyArmorWithShield#step:doProjectHeavyArmorWithShield` | `src/tests/mod.rs` | `covered` |
| `cleanroom-input/qnt/character-sheet-runtime/character-sheet-armor-class-base-selected-identity.mbt.qnt#step:doProjectLightArmor` | `tasks/target-replay-evidence/L15-RR09-character-sheet-routes.json#L15-RR09-CHARACTER-SHEET-ROUTES route action=doProjectLightArmor#step:doProjectLightArmor` | `src/tests/mod.rs` | `covered` |
| `cleanroom-input/qnt/character-sheet-runtime/character-sheet-armor-class-base-selected-identity.mbt.qnt#step:doProjectMediumArmorDexCap` | `tasks/target-replay-evidence/L15-RR09-character-sheet-routes.json#L15-RR09-CHARACTER-SHEET-ROUTES route action=doProjectMediumArmorDexCap#step:doProjectMediumArmorDexCap` | `src/tests/mod.rs` | `covered` |
| `cleanroom-input/qnt/character-sheet-runtime/character-sheet-armor-class-base-selected-identity.mbt.qnt#step:doSelectBarbarianUnarmoredDefense` | `tasks/target-replay-evidence/L15-RR09-character-sheet-routes.json#L15-RR09-CHARACTER-SHEET-ROUTES route action=doSelectBarbarianUnarmoredDefense#step:doSelectBarbarianUnarmoredDefense` | `src/tests/mod.rs` | `covered` |
| `cleanroom-input/qnt/character-sheet-runtime/character-sheet-armor-class-base-selected-identity.mbt.qnt#step:doSelectBarbarianUnarmoredDefenseWithShield` | `tasks/target-replay-evidence/L15-RR09-character-sheet-routes.json#L15-RR09-CHARACTER-SHEET-ROUTES route action=doSelectBarbarianUnarmoredDefenseWithShield#step:doSelectBarbarianUnarmoredDefenseWithShield` | `src/tests/mod.rs` | `covered` |
| `cleanroom-input/qnt/character-sheet-runtime/character-sheet-armor-class-base-selected-identity.mbt.qnt#step:doSelectMonkUnarmoredDefense` | `tasks/target-replay-evidence/L15-RR09-character-sheet-routes.json#L15-RR09-CHARACTER-SHEET-ROUTES route action=doSelectMonkUnarmoredDefense#step:doSelectMonkUnarmoredDefense` | `src/tests/mod.rs` | `covered` |
| `cleanroom-input/qnt/character-sheet-runtime/character-sheet-class-feature-selected-identity.mbt.qnt#step:doProjectBardJackOfAllTrades` | `tasks/target-replay-evidence/L15-RR09-character-sheet-routes.json#L15-RR09-CHARACTER-SHEET-ROUTES route action=doProjectBardJackOfAllTrades#step:doProjectBardJackOfAllTrades` | `src/tests/mod.rs` | `covered` |
| `cleanroom-input/qnt/character-sheet-runtime/character-sheet-class-feature-selected-identity.mbt.qnt#step:doProjectClericLifeDomainSpells` | `tasks/target-replay-evidence/L15-RR09-character-sheet-routes.json#L15-RR09-CHARACTER-SHEET-ROUTES route action=doProjectClericLifeDomainSpells#step:doProjectClericLifeDomainSpells` | `src/tests/mod.rs` | `covered` |
| `cleanroom-input/qnt/character-sheet-runtime/character-sheet-class-feature-selected-identity.mbt.qnt#step:doProjectDruidCircleLandSpells` | `tasks/target-replay-evidence/L15-RR09-character-sheet-routes.json#L15-RR09-CHARACTER-SHEET-ROUTES route action=doProjectDruidCircleLandSpells#step:doProjectDruidCircleLandSpells` | `src/tests/mod.rs` | `covered` |
| `cleanroom-input/qnt/character-sheet-runtime/character-sheet-class-feature-selected-identity.mbt.qnt#step:doProjectPaladinOathDevotionSpells` | `tasks/target-replay-evidence/L15-RR09-character-sheet-routes.json#L15-RR09-CHARACTER-SHEET-ROUTES route action=doProjectPaladinOathDevotionSpells#step:doProjectPaladinOathDevotionSpells` | `src/tests/mod.rs` | `covered` |
| `cleanroom-input/qnt/character-sheet-runtime/character-sheet-class-feature-selected-identity.mbt.qnt#step:doProjectPaladinsSmite` | `tasks/target-replay-evidence/L15-RR09-character-sheet-routes.json#L15-RR09-CHARACTER-SHEET-ROUTES route action=doProjectPaladinsSmite#step:doProjectPaladinsSmite` | `src/tests/mod.rs` | `covered` |
| `cleanroom-input/qnt/character-sheet-runtime/character-sheet-class-feature-selected-identity.mbt.qnt#step:doProjectRangerFavoredEnemy` | `tasks/target-replay-evidence/L15-RR09-character-sheet-routes.json#L15-RR09-CHARACTER-SHEET-ROUTES route action=doProjectRangerFavoredEnemy#step:doProjectRangerFavoredEnemy` | `src/tests/mod.rs` | `covered` |
| `cleanroom-input/qnt/character-sheet-runtime/character-sheet-class-feature-selected-identity.mbt.qnt#step:doProjectSorcererDraconicSpells` | `tasks/target-replay-evidence/L15-RR09-character-sheet-routes.json#L15-RR09-CHARACTER-SHEET-ROUTES route action=doProjectSorcererDraconicSpells#step:doProjectSorcererDraconicSpells` | `src/tests/mod.rs` | `covered` |
| `cleanroom-input/qnt/character-sheet-runtime/character-sheet-class-feature-selected-identity.mbt.qnt#step:doProjectWarlockFiendSpells` | `tasks/target-replay-evidence/L15-RR09-character-sheet-routes.json#L15-RR09-CHARACTER-SHEET-ROUTES route action=doProjectWarlockFiendSpells#step:doProjectWarlockFiendSpells` | `src/tests/mod.rs` | `covered` |
| `cleanroom-input/qnt/character-sheet-runtime/character-sheet-healing-resource-selected-identity.mbt.qnt#step:doLayOnHandsRestoreHpAndRemovePoisoned` | `tasks/target-replay-evidence/L15-RR09-character-sheet-routes.json#L15-RR09-CHARACTER-SHEET-ROUTES route action=doLayOnHandsRestoreHpAndRemovePoisoned#step:doLayOnHandsRestoreHpAndRemovePoisoned` | `src/tests/mod.rs` | `covered` |
| `cleanroom-input/qnt/character-sheet-runtime/character-sheet-hit-point-maximum.mbt.qnt#step:doProjectFighterLevelOne` | `tasks/target-replay-evidence/L15-RR09-character-sheet-routes.json#L15-RR09-CHARACTER-SHEET-ROUTES route action=doProjectFighterLevelOne#step:doProjectFighterLevelOne` | `src/tests/mod.rs` | `covered` |
| `cleanroom-input/qnt/character-sheet-runtime/character-sheet-hit-point-maximum.mbt.qnt#step:doProjectFighterLevelTwo` | `tasks/target-replay-evidence/L15-RR09-character-sheet-routes.json#L15-RR09-CHARACTER-SHEET-ROUTES route action=doProjectFighterLevelTwo#step:doProjectFighterLevelTwo` | `src/tests/mod.rs` | `covered` |
| `cleanroom-input/qnt/character-sheet-runtime/character-sheet-hit-point-maximum.mbt.qnt#step:doProjectMinimumHigherLevelGain` | `tasks/target-replay-evidence/L15-RR09-character-sheet-routes.json#L15-RR09-CHARACTER-SHEET-ROUTES route action=doProjectMinimumHigherLevelGain#step:doProjectMinimumHigherLevelGain` | `src/tests/mod.rs` | `covered` |
| `cleanroom-input/qnt/character-sheet-runtime/character-sheet-hit-point-maximum.mbt.qnt#step:doProjectReducedEffectiveMaximum` | `tasks/target-replay-evidence/L15-RR09-character-sheet-routes.json#L15-RR09-CHARACTER-SHEET-ROUTES route action=doProjectReducedEffectiveMaximum#step:doProjectReducedEffectiveMaximum` | `src/tests/mod.rs` | `covered` |
| `cleanroom-input/qnt/character-sheet-runtime/character-sheet-hit-point-maximum.mbt.qnt#step:doProjectSorcererDraconicResilience` | `tasks/target-replay-evidence/L15-RR09-character-sheet-routes.json#L15-RR09-CHARACTER-SHEET-ROUTES route action=doProjectSorcererDraconicResilience#step:doProjectSorcererDraconicResilience` | `src/tests/mod.rs` | `covered` |
| `cleanroom-input/qnt/character-sheet-runtime/character-sheet-hit-point-maximum.mbt.qnt#step:doProjectWizardFighterMulticlass` | `tasks/target-replay-evidence/L15-RR09-character-sheet-routes.json#L15-RR09-CHARACTER-SHEET-ROUTES route action=doProjectWizardFighterMulticlass#step:doProjectWizardFighterMulticlass` | `src/tests/mod.rs` | `covered` |
| `cleanroom-input/qnt/character-sheet-runtime/character-sheet-hp-rest-hit-dice.mbt.qnt#step:doCompleteLongRestRestoresHpHitPointDiceAndMaximum` | `tasks/target-replay-evidence/L15-RR09-character-sheet-routes.json#L15-RR09-CHARACTER-SHEET-ROUTES route action=doCompleteLongRestRestoresHpHitPointDiceAndMaximum#step:doCompleteLongRestRestoresHpHitPointDiceAndMaximum` | `src/tests/mod.rs` | `covered` |
| `cleanroom-input/qnt/character-sheet-runtime/character-sheet-hp-rest-hit-dice.mbt.qnt#step:doInterruptLongRestBeforeOneHourNoBenefit` | `tasks/target-replay-evidence/L15-RR09-character-sheet-routes.json#L15-RR09-CHARACTER-SHEET-ROUTES route action=doInterruptLongRestBeforeOneHourNoBenefit#step:doInterruptLongRestBeforeOneHourNoBenefit` | `src/tests/mod.rs` | `covered` |
| `cleanroom-input/qnt/character-sheet-runtime/character-sheet-hp-rest-hit-dice.mbt.qnt#step:doInterruptLongRestWithShortRestBenefits` | `tasks/target-replay-evidence/L15-RR09-character-sheet-routes.json#L15-RR09-CHARACTER-SHEET-ROUTES route action=doInterruptLongRestWithShortRestBenefits#step:doInterruptLongRestWithShortRestBenefits` | `src/tests/mod.rs` | `covered` |
| `cleanroom-input/qnt/character-sheet-runtime/character-sheet-hp-rest-hit-dice.mbt.qnt#step:doInterruptShortRestNoBenefit` | `tasks/target-replay-evidence/L15-RR09-character-sheet-routes.json#L15-RR09-CHARACTER-SHEET-ROUTES route action=doInterruptShortRestNoBenefit#step:doInterruptShortRestNoBenefit` | `src/tests/mod.rs` | `covered` |
| `cleanroom-input/qnt/character-sheet-runtime/character-sheet-hp-rest-hit-dice.mbt.qnt#step:doRejectLongRestBeforeSixteenHourWait` | `tasks/target-replay-evidence/L15-RR09-character-sheet-routes.json#L15-RR09-CHARACTER-SHEET-ROUTES route action=doRejectLongRestBeforeSixteenHourWait#step:doRejectLongRestBeforeSixteenHourWait` | `src/tests/mod.rs` | `covered` |
| `cleanroom-input/qnt/character-sheet-runtime/character-sheet-hp-rest-hit-dice.mbt.qnt#step:doRejectLongRestDurationTooShort` | `tasks/target-replay-evidence/L15-RR09-character-sheet-routes.json#L15-RR09-CHARACTER-SHEET-ROUTES route action=doRejectLongRestDurationTooShort#step:doRejectLongRestDurationTooShort` | `src/tests/mod.rs` | `covered` |
| `cleanroom-input/qnt/character-sheet-runtime/character-sheet-hp-rest-hit-dice.mbt.qnt#step:doRejectLongRestInterruptionAtRequiredDuration` | `tasks/target-replay-evidence/L15-RR09-character-sheet-routes.json#L15-RR09-CHARACTER-SHEET-ROUTES route action=doRejectLongRestInterruptionAtRequiredDuration#step:doRejectLongRestInterruptionAtRequiredDuration` | `src/tests/mod.rs` | `covered` |
| `cleanroom-input/qnt/character-sheet-runtime/character-sheet-hp-rest-hit-dice.mbt.qnt#step:doRejectLongRestPhysicalExertionTooShort` | `tasks/target-replay-evidence/L15-RR09-character-sheet-routes.json#L15-RR09-CHARACTER-SHEET-ROUTES route action=doRejectLongRestPhysicalExertionTooShort#step:doRejectLongRestPhysicalExertionTooShort` | `src/tests/mod.rs` | `covered` |
| `cleanroom-input/qnt/character-sheet-runtime/character-sheet-hp-rest-hit-dice.mbt.qnt#step:doRejectLongRestStartAtZeroHp` | `tasks/target-replay-evidence/L15-RR09-character-sheet-routes.json#L15-RR09-CHARACTER-SHEET-ROUTES route action=doRejectLongRestStartAtZeroHp#step:doRejectLongRestStartAtZeroHp` | `src/tests/mod.rs` | `covered` |
| `cleanroom-input/qnt/character-sheet-runtime/character-sheet-hp-rest-hit-dice.mbt.qnt#step:doRejectShortRestDurationTooShort` | `tasks/target-replay-evidence/L15-RR09-character-sheet-routes.json#L15-RR09-CHARACTER-SHEET-ROUTES route action=doRejectShortRestDurationTooShort#step:doRejectShortRestDurationTooShort` | `src/tests/mod.rs` | `covered` |
| `cleanroom-input/qnt/character-sheet-runtime/character-sheet-hp-rest-hit-dice.mbt.qnt#step:doRejectShortRestStartAtZeroHp` | `tasks/target-replay-evidence/L15-RR09-character-sheet-routes.json#L15-RR09-CHARACTER-SHEET-ROUTES route action=doRejectShortRestStartAtZeroHp#step:doRejectShortRestStartAtZeroHp` | `src/tests/mod.rs` | `covered` |
| `cleanroom-input/qnt/character-sheet-runtime/character-sheet-hp-rest-hit-dice.mbt.qnt#step:doSpendShortRestHitPointDiceSequentially` | `tasks/target-replay-evidence/L15-RR09-character-sheet-routes.json#L15-RR09-CHARACTER-SHEET-ROUTES route action=doSpendShortRestHitPointDiceSequentially#step:doSpendShortRestHitPointDiceSequentially` | `src/tests/mod.rs` | `covered` |
| `cleanroom-input/qnt/character-sheet-runtime/character-sheet-hp-rest-hit-dice.mbt.qnt#step:doSpendShortRestHitPointDie` | `tasks/target-replay-evidence/L15-RR09-character-sheet-routes.json#L15-RR09-CHARACTER-SHEET-ROUTES route action=doSpendShortRestHitPointDie#step:doSpendShortRestHitPointDie` | `src/tests/mod.rs` | `covered` |
| `cleanroom-input/qnt/character-sheet-runtime/character-sheet-spell-slots-pact-slots.mbt.qnt#step:doCompleteLongRestRestoresOrdinaryPactAndClearsCreatedSlots` | `tasks/target-replay-evidence/L15-RR09-character-sheet-routes.json#L15-RR09-CHARACTER-SHEET-ROUTES route action=doCompleteLongRestRestoresOrdinaryPactAndClearsCreatedSlots#step:doCompleteLongRestRestoresOrdinaryPactAndClearsCreatedSlots` | `src/tests/mod.rs` | `covered` |
| `cleanroom-input/qnt/character-sheet-runtime/character-sheet-spell-slots-pact-slots.mbt.qnt#step:doInterruptLongRestBeforeOneHourNoSlotBenefit` | `tasks/target-replay-evidence/L15-RR09-character-sheet-routes.json#L15-RR09-CHARACTER-SHEET-ROUTES route action=doInterruptLongRestBeforeOneHourNoSlotBenefit#step:doInterruptLongRestBeforeOneHourNoSlotBenefit` | `src/tests/mod.rs` | `covered` |
| `cleanroom-input/qnt/character-sheet-runtime/character-sheet-spell-slots-pact-slots.mbt.qnt#step:doInterruptLongRestWithShortRestSlotBenefits` | `tasks/target-replay-evidence/L15-RR09-character-sheet-routes.json#L15-RR09-CHARACTER-SHEET-ROUTES route action=doInterruptLongRestWithShortRestSlotBenefits#step:doInterruptLongRestWithShortRestSlotBenefits` | `src/tests/mod.rs` | `covered` |
| `cleanroom-input/qnt/character-sheet-runtime/character-sheet-spell-slots-pact-slots.mbt.qnt#step:doInterruptShortRestNoSlotBenefit` | `tasks/target-replay-evidence/L15-RR09-character-sheet-routes.json#L15-RR09-CHARACTER-SHEET-ROUTES route action=doInterruptShortRestNoSlotBenefit#step:doInterruptShortRestNoSlotBenefit` | `src/tests/mod.rs` | `covered` |
| `cleanroom-input/qnt/character-sheet-runtime/character-sheet-spell-slots-pact-slots.mbt.qnt#step:doMagicalCunningRecoversPactSlots` | `tasks/target-replay-evidence/L15-RR09-character-sheet-routes.json#L15-RR09-CHARACTER-SHEET-ROUTES route action=doMagicalCunningRecoversPactSlots#step:doMagicalCunningRecoversPactSlots` | `src/tests/mod.rs` | `covered` |
| `cleanroom-input/qnt/character-sheet-runtime/character-sheet-spell-slots-pact-slots.mbt.qnt#step:doRejectArcaneRecoveryPactSlotRefund` | `tasks/target-replay-evidence/L15-RR09-character-sheet-routes.json#L15-RR09-CHARACTER-SHEET-ROUTES route action=doRejectArcaneRecoveryPactSlotRefund#step:doRejectArcaneRecoveryPactSlotRefund` | `src/tests/mod.rs` | `covered` |
| `cleanroom-input/qnt/character-sheet-runtime/character-sheet-spell-slots-pact-slots.mbt.qnt#step:doRejectMagicalCunningWithoutExpendedPactSlots` | `tasks/target-replay-evidence/L15-RR09-character-sheet-routes.json#L15-RR09-CHARACTER-SHEET-ROUTES route action=doRejectMagicalCunningWithoutExpendedPactSlots#step:doRejectMagicalCunningWithoutExpendedPactSlots` | `src/tests/mod.rs` | `covered` |
| `cleanroom-input/qnt/character-sheet-runtime/character-sheet-spell-slots-pact-slots.mbt.qnt#step:doRejectMismatchedOrdinarySpellSlotCapacity` | `tasks/target-replay-evidence/L15-RR09-character-sheet-routes.json#L15-RR09-CHARACTER-SHEET-ROUTES route action=doRejectMismatchedOrdinarySpellSlotCapacity#step:doRejectMismatchedOrdinarySpellSlotCapacity` | `src/tests/mod.rs` | `covered` |
| `cleanroom-input/qnt/character-sheet-runtime/character-sheet-spell-slots-pact-slots.mbt.qnt#step:doRejectPactSlotExpenditureOverCapacity` | `tasks/target-replay-evidence/L15-RR09-character-sheet-routes.json#L15-RR09-CHARACTER-SHEET-ROUTES route action=doRejectPactSlotExpenditureOverCapacity#step:doRejectPactSlotExpenditureOverCapacity` | `src/tests/mod.rs` | `covered` |
| `cleanroom-input/qnt/character-sheet-runtime/character-sheet-spell-slots-pact-slots.mbt.qnt#step:doShortRestArcaneRecoveryRefundsOrdinarySpellSlot` | `tasks/target-replay-evidence/L15-RR09-character-sheet-routes.json#L15-RR09-CHARACTER-SHEET-ROUTES route action=doShortRestArcaneRecoveryRefundsOrdinarySpellSlot#step:doShortRestArcaneRecoveryRefundsOrdinarySpellSlot` | `src/tests/mod.rs` | `covered` |
| `cleanroom-input/qnt/character-sheet-runtime/character-sheet-spell-slots-pact-slots.mbt.qnt#step:doShortRestRestoresPactSlotsOnly` | `tasks/target-replay-evidence/L15-RR09-character-sheet-routes.json#L15-RR09-CHARACTER-SHEET-ROUTES route action=doShortRestRestoresPactSlotsOnly#step:doShortRestRestoresPactSlotsOnly` | `src/tests/mod.rs` | `covered` |
| `cleanroom-input/qnt/character-sheet-runtime/character-sheet-spellbook-ritual-selected-identity.mbt.qnt#step:doInvokeSpellbookRitual` | `tasks/target-replay-evidence/L15-RR09-character-sheet-routes.json#L15-RR09-CHARACTER-SHEET-ROUTES route action=doInvokeSpellbookRitual#step:doInvokeSpellbookRitual` | `src/tests/mod.rs` | `covered` |
| `cleanroom-input/qnt/character-sheet-runtime/character-sheet-spellbook-ritual-selected-identity.mbt.qnt#step:doRejectMissingRitualAccessFeature` | `tasks/target-replay-evidence/L15-RR09-character-sheet-routes.json#L15-RR09-CHARACTER-SHEET-ROUTES route action=doRejectMissingRitualAccessFeature#step:doRejectMissingRitualAccessFeature` | `src/tests/mod.rs` | `covered` |
| `cleanroom-input/qnt/character-sheet-runtime/character-sheet-spellbook-ritual-selected-identity.mbt.qnt#step:doRejectNonLeveledRitualSpellbookSpell` | `tasks/target-replay-evidence/L15-RR09-character-sheet-routes.json#L15-RR09-CHARACTER-SHEET-ROUTES route action=doRejectNonLeveledRitualSpellbookSpell#step:doRejectNonLeveledRitualSpellbookSpell` | `src/tests/mod.rs` | `covered` |
| `cleanroom-input/qnt/character-sheet-runtime/character-sheet-spellbook-ritual-selected-identity.mbt.qnt#step:doRejectNonRitualSpellbookSpell` | `tasks/target-replay-evidence/L15-RR09-character-sheet-routes.json#L15-RR09-CHARACTER-SHEET-ROUTES route action=doRejectNonRitualSpellbookSpell#step:doRejectNonRitualSpellbookSpell` | `src/tests/mod.rs` | `covered` |
| `cleanroom-input/qnt/character-sheet-runtime/character-sheet-spellbook-ritual-selected-identity.mbt.qnt#step:doRejectPreparedOnlyRitual` | `tasks/target-replay-evidence/L15-RR09-character-sheet-routes.json#L15-RR09-CHARACTER-SHEET-ROUTES route action=doRejectPreparedOnlyRitual#step:doRejectPreparedOnlyRitual` | `src/tests/mod.rs` | `covered` |
| `cleanroom-input/qnt/character-sheet-runtime/character-sheet-weapon-mastery-containers-selected-identity.mbt.qnt#step:doAcceptOneChangeWeaponMasteryReselection` | `tasks/target-replay-evidence/L15-RR09-character-sheet-routes.json#L15-RR09-CHARACTER-SHEET-ROUTES route action=doAcceptOneChangeWeaponMasteryReselection#step:doAcceptOneChangeWeaponMasteryReselection` | `src/tests/mod.rs` | `covered` |
| `cleanroom-input/qnt/character-sheet-runtime/character-sheet-weapon-mastery-containers-selected-identity.mbt.qnt#step:doRejectTooManyChangesWeaponMasteryReselection` | `tasks/target-replay-evidence/L15-RR09-character-sheet-routes.json#L15-RR09-CHARACTER-SHEET-ROUTES route action=doRejectTooManyChangesWeaponMasteryReselection#step:doRejectTooManyChangesWeaponMasteryReselection` | `src/tests/mod.rs` | `covered` |
| `cleanroom-input/qnt/character-sheet-runtime/character-sheet-weapon-mastery-containers-selected-identity.mbt.qnt#step:doReselectPaladinWeaponMasteryOnLongRest` | `tasks/target-replay-evidence/L15-RR09-character-sheet-routes.json#L15-RR09-CHARACTER-SHEET-ROUTES route action=doReselectPaladinWeaponMasteryOnLongRest#step:doReselectPaladinWeaponMasteryOnLongRest` | `src/tests/mod.rs` | `covered` |
| `cleanroom-input/qnt/character-sheet-runtime/character-sheet-weapon-mastery-containers-selected-identity.mbt.qnt#step:doReselectRangerWeaponMasteryOnLongRest` | `tasks/target-replay-evidence/L15-RR09-character-sheet-routes.json#L15-RR09-CHARACTER-SHEET-ROUTES route action=doReselectRangerWeaponMasteryOnLongRest#step:doReselectRangerWeaponMasteryOnLongRest` | `src/tests/mod.rs` | `covered` |
| `cleanroom-input/qnt/character-sheet-runtime/character-sheet-weapon-mastery-containers-selected-identity.mbt.qnt#step:doReselectRogueWeaponMasteryOnLongRest` | `tasks/target-replay-evidence/L15-RR09-character-sheet-routes.json#L15-RR09-CHARACTER-SHEET-ROUTES route action=doReselectRogueWeaponMasteryOnLongRest#step:doReselectRogueWeaponMasteryOnLongRest` | `src/tests/mod.rs` | `covered` |
| `cleanroom-input/qnt/character-sheet-runtime/character-sheet-weapon-mastery-containers-selected-identity.mbt.qnt#step:doSelectPaladinWeaponMastery` | `tasks/target-replay-evidence/L15-RR09-character-sheet-routes.json#L15-RR09-CHARACTER-SHEET-ROUTES route action=doSelectPaladinWeaponMastery#step:doSelectPaladinWeaponMastery` | `src/tests/mod.rs` | `covered` |
| `cleanroom-input/qnt/character-sheet-runtime/character-sheet-weapon-mastery-containers-selected-identity.mbt.qnt#step:doSelectRangerWeaponMastery` | `tasks/target-replay-evidence/L15-RR09-character-sheet-routes.json#L15-RR09-CHARACTER-SHEET-ROUTES route action=doSelectRangerWeaponMastery#step:doSelectRangerWeaponMastery` | `src/tests/mod.rs` | `covered` |
| `cleanroom-input/qnt/character-sheet-runtime/character-sheet-weapon-mastery-containers-selected-identity.mbt.qnt#step:doSelectRogueWeaponMastery` | `tasks/target-replay-evidence/L15-RR09-character-sheet-routes.json#L15-RR09-CHARACTER-SHEET-ROUTES route action=doSelectRogueWeaponMastery#step:doSelectRogueWeaponMastery` | `src/tests/mod.rs` | `covered` |

Target replay evidence:

- Evidence file: `tasks/target-replay-evidence/L15-RR09-character-sheet-routes.json`
- Target profile: `rust`
- Reproduction seed or trace id: `L15-RR09-CHARACTER-SHEET-ROUTES route action=<branchAction>`

Harness artifacts:

- Start gate: `tasks/history/L15-RR09-CHARACTER-SHEET-ROUTES/START_GATE.json`
- Engine depth: `tasks/history/L15-RR09-CHARACTER-SHEET-ROUTES/ENGINE_DEPTH_MANIFEST.json`
- State ownership: `tasks/history/L15-RR09-CHARACTER-SHEET-ROUTES/STATE_OWNER_MANIFEST.json`
- Reviewer loop: `tasks/history/L15-RR09-CHARACTER-SHEET-ROUTES/REVIEW_LOOP.json`
- Decider decision: `tasks/history/L15-RR09-CHARACTER-SHEET-ROUTES/DECIDER_DECISION.json`
- Run ledger: `tasks/RUN_LEDGER.json`

Remaining gaps:

- `_none_`

Verification results:

- `cargo fmt --check` passed.
- `cargo test` passed.
- `cargo clippy --all-targets -- -D warnings` passed.
- `node scripts/check-cleanroom-harness.cjs` passed.

## L15-RR08-CHARACTER-CREATION-ROUTES: Character Creation Routes

- Manifest source commit SHA: `564376fd95218a209bb9eae5c9ccb54ca3e04a52`
- Source branch inventory SHA: `4bb2b20a85d94e3b90b7c59cbfe6e1edd5ab3ef40410641e999527861f3d3a32`
- Selected drivers: `cleanroom-input/qnt/character-creation-runtime/character-creation-class-feature-projections.mbt.qnt`, `cleanroom-input/qnt/character-creation-runtime/character-creation-class-feature-selected-identity.mbt.qnt`, `cleanroom-input/qnt/character-creation-runtime/character-creation-cleric-druid-order-selected-identity.mbt.qnt`, `cleanroom-input/qnt/character-creation-runtime/character-creation-fighter-fighting-style-selected-identity.mbt.qnt`, `cleanroom-input/qnt/character-creation-runtime/character-creation-rogue-expertise-selected-identity.mbt.qnt`, `cleanroom-input/qnt/character-creation-runtime/character-creation-runtime.mbt.qnt`, `cleanroom-input/qnt/character-creation-runtime/character-creation-warlock-eldritch-invocations-selected-identity.mbt.qnt`, `cleanroom-input/qnt/character-creation-runtime/character-creation-weapon-mastery-containers-selected-identity.mbt.qnt`
- Machine-readable run ledger: `tasks/RUN_LEDGER.json`

Allowed inputs used:

- `cleanroom-input/MANIFEST.md`
- `cleanroom-input/branch-coverage/source-branch-inventory.json`
- `cleanroom-input/branch-coverage/reducer-route-inventory.json`
- Selected `cleanroom-input/qnt/character-creation-runtime/*.mbt.qnt` and route connectors
- `cleanroom-input/raw/srd-5.2.1/Character-Creation.md`, `Classes/*.md`, `Rules-Glossary.md`, `Equipment.md`
- `cleanroom-input/domain/UBIQUITOUS_LANGUAGE.md` and `CLEANROOM_ASSUMPTIONS.md`
- Repo-local `src/**`, `tasks/**`, and Cargo tooling

Behavior implemented:

- Added shared character-owned creation state and qRoute events in `src/rules/character_creation.rs`.
- Added `ProjectBuildFacts` routing plus adapter-local QNT literal expected-route witnesses in `src/qnt_adapters/character_creation_expected_routes.rs`; character-creation expected routes no longer call observed replay helpers.
- Replaced production class-feature identity dispatch for level-2 feature projections, cleric/druid order projections, weapon mastery class counts, and metamagic option cost/effect facts with admitted support fact shapes in `src/rules/class_features.rs`.
- Added missing selected-identity adapters and route evidence for the selected character-creation drivers; authored QNT action names and official selected labels remain quarantined in adapters/tests/evidence or selection-boundary fact mapping.

Reviewer finding fixes:

- Fixed `fixer-1-independent-route-evidence`: expected qRoute witnesses are independent adapter-local literals derived from the copied QNT route fixtures, not `replay_observed_route`.
- Fixed `fixer-2-production-authored-identity-dispatch`: production rules now consume `ClassFeatureProjectionFacts`, `ClassOrderFacts`, `WeaponMasteryFacts`, and opaque metamagic choice keys instead of branching on selected official identity.
- Fixed `fixer-3-report-honesty`: review, state-owner, engine-depth, decider, and validation-report artifacts now describe the repaired evidence boundary and no longer claim the pre-fix implementation had converged.

Generated branch coverage:

| Obligation | Target replay evidence | Diagnostic tests | Status |
| --- | --- | --- | --- |
| `cleanroom-input/qnt/character-creation-runtime/character-creation-class-feature-projections.mbt.qnt#step:doProjectMonkFocusAndUncannyMetabolism` | `tasks/target-replay-evidence/L15-RR08-character-creation-routes.json#L15-RR08-CHARACTER-CREATION-ROUTES replay action=doProjectMonkFocusAndUncannyMetabolism#step:doProjectMonkFocusAndUncannyMetabolism` | `src/tests/mod.rs` | `covered` |
| `cleanroom-input/qnt/character-creation-runtime/character-creation-class-feature-projections.mbt.qnt#step:doProjectSorcererFontAndMetamagic` | `tasks/target-replay-evidence/L15-RR08-character-creation-routes.json#L15-RR08-CHARACTER-CREATION-ROUTES replay action=doProjectSorcererFontAndMetamagic#step:doProjectSorcererFontAndMetamagic` | `src/tests/mod.rs` | `covered` |
| `cleanroom-input/qnt/character-creation-runtime/character-creation-class-feature-selected-identity.mbt.qnt#step:doProjectClericChannelDivinity` | `tasks/target-replay-evidence/L15-RR08-character-creation-routes.json#L15-RR08-CHARACTER-CREATION-ROUTES replay action=doProjectClericChannelDivinity#step:doProjectClericChannelDivinity` | `src/tests/mod.rs` | `covered` |
| `cleanroom-input/qnt/character-creation-runtime/character-creation-class-feature-selected-identity.mbt.qnt#step:doProjectDruidWildCompanion` | `tasks/target-replay-evidence/L15-RR08-character-creation-routes.json#L15-RR08-CHARACTER-CREATION-ROUTES replay action=doProjectDruidWildCompanion#step:doProjectDruidWildCompanion` | `src/tests/mod.rs` | `covered` |
| `cleanroom-input/qnt/character-creation-runtime/character-creation-class-feature-selected-identity.mbt.qnt#step:doProjectDruidWildShape` | `tasks/target-replay-evidence/L15-RR08-character-creation-routes.json#L15-RR08-CHARACTER-CREATION-ROUTES replay action=doProjectDruidWildShape#step:doProjectDruidWildShape` | `src/tests/mod.rs` | `covered` |
| `cleanroom-input/qnt/character-creation-runtime/character-creation-class-feature-selected-identity.mbt.qnt#step:doProjectMonksFocus` | `tasks/target-replay-evidence/L15-RR08-character-creation-routes.json#L15-RR08-CHARACTER-CREATION-ROUTES replay action=doProjectMonksFocus#step:doProjectMonksFocus` | `src/tests/mod.rs` | `covered` |
| `cleanroom-input/qnt/character-creation-runtime/character-creation-class-feature-selected-identity.mbt.qnt#step:doProjectMonkUncannyMetabolism` | `tasks/target-replay-evidence/L15-RR08-character-creation-routes.json#L15-RR08-CHARACTER-CREATION-ROUTES replay action=doProjectMonkUncannyMetabolism#step:doProjectMonkUncannyMetabolism` | `src/tests/mod.rs` | `covered` |
| `cleanroom-input/qnt/character-creation-runtime/character-creation-class-feature-selected-identity.mbt.qnt#step:doProjectWarlockPactMagic` | `tasks/target-replay-evidence/L15-RR08-character-creation-routes.json#L15-RR08-CHARACTER-CREATION-ROUTES replay action=doProjectWarlockPactMagic#step:doProjectWarlockPactMagic` | `src/tests/mod.rs` | `covered` |
| `cleanroom-input/qnt/character-creation-runtime/character-creation-class-feature-selected-identity.mbt.qnt#step:doSelectBardExpertise` | `tasks/target-replay-evidence/L15-RR08-character-creation-routes.json#L15-RR08-CHARACTER-CREATION-ROUTES replay action=doSelectBardExpertise#step:doSelectBardExpertise` | `src/tests/mod.rs` | `covered` |
| `cleanroom-input/qnt/character-creation-runtime/character-creation-class-feature-selected-identity.mbt.qnt#step:doSelectPaladinFightingStyle` | `tasks/target-replay-evidence/L15-RR08-character-creation-routes.json#L15-RR08-CHARACTER-CREATION-ROUTES replay action=doSelectPaladinFightingStyle#step:doSelectPaladinFightingStyle` | `src/tests/mod.rs` | `covered` |
| `cleanroom-input/qnt/character-creation-runtime/character-creation-class-feature-selected-identity.mbt.qnt#step:doSelectRangerDeftExplorer` | `tasks/target-replay-evidence/L15-RR08-character-creation-routes.json#L15-RR08-CHARACTER-CREATION-ROUTES replay action=doSelectRangerDeftExplorer#step:doSelectRangerDeftExplorer` | `src/tests/mod.rs` | `covered` |
| `cleanroom-input/qnt/character-creation-runtime/character-creation-class-feature-selected-identity.mbt.qnt#step:doSelectRangerFightingStyle` | `tasks/target-replay-evidence/L15-RR08-character-creation-routes.json#L15-RR08-CHARACTER-CREATION-ROUTES replay action=doSelectRangerFightingStyle#step:doSelectRangerFightingStyle` | `src/tests/mod.rs` | `covered` |
| `cleanroom-input/qnt/character-creation-runtime/character-creation-class-feature-selected-identity.mbt.qnt#step:doSelectWizardEvocationSavant` | `tasks/target-replay-evidence/L15-RR08-character-creation-routes.json#L15-RR08-CHARACTER-CREATION-ROUTES replay action=doSelectWizardEvocationSavant#step:doSelectWizardEvocationSavant` | `src/tests/mod.rs` | `covered` |
| `cleanroom-input/qnt/character-creation-runtime/character-creation-class-feature-selected-identity.mbt.qnt#step:doSelectWizardScholar` | `tasks/target-replay-evidence/L15-RR08-character-creation-routes.json#L15-RR08-CHARACTER-CREATION-ROUTES replay action=doSelectWizardScholar#step:doSelectWizardScholar` | `src/tests/mod.rs` | `covered` |
| `cleanroom-input/qnt/character-creation-runtime/character-creation-cleric-druid-order-selected-identity.mbt.qnt#step:doSelectClericProtectorOrder` | `tasks/target-replay-evidence/L15-RR08-character-creation-routes.json#L15-RR08-CHARACTER-CREATION-ROUTES replay action=doSelectClericProtectorOrder#step:doSelectClericProtectorOrder` | `src/tests/mod.rs` | `covered` |
| `cleanroom-input/qnt/character-creation-runtime/character-creation-cleric-druid-order-selected-identity.mbt.qnt#step:doSelectClericThaumaturgeOrder` | `tasks/target-replay-evidence/L15-RR08-character-creation-routes.json#L15-RR08-CHARACTER-CREATION-ROUTES replay action=doSelectClericThaumaturgeOrder#step:doSelectClericThaumaturgeOrder` | `src/tests/mod.rs` | `covered` |
| `cleanroom-input/qnt/character-creation-runtime/character-creation-cleric-druid-order-selected-identity.mbt.qnt#step:doSelectDruidMagicianOrder` | `tasks/target-replay-evidence/L15-RR08-character-creation-routes.json#L15-RR08-CHARACTER-CREATION-ROUTES replay action=doSelectDruidMagicianOrder#step:doSelectDruidMagicianOrder` | `src/tests/mod.rs` | `covered` |
| `cleanroom-input/qnt/character-creation-runtime/character-creation-cleric-druid-order-selected-identity.mbt.qnt#step:doSelectDruidWardenOrder` | `tasks/target-replay-evidence/L15-RR08-character-creation-routes.json#L15-RR08-CHARACTER-CREATION-ROUTES replay action=doSelectDruidWardenOrder#step:doSelectDruidWardenOrder` | `src/tests/mod.rs` | `covered` |
| `cleanroom-input/qnt/character-creation-runtime/character-creation-fighter-fighting-style-selected-identity.mbt.qnt#step:doReplaceArcheryWithDefenseOnFighterLevelGain` | `tasks/target-replay-evidence/L15-RR08-character-creation-routes.json#L15-RR08-CHARACTER-CREATION-ROUTES replay action=doReplaceArcheryWithDefenseOnFighterLevelGain#step:doReplaceArcheryWithDefenseOnFighterLevelGain` | `src/tests/mod.rs` | `covered` |
| `cleanroom-input/qnt/character-creation-runtime/character-creation-fighter-fighting-style-selected-identity.mbt.qnt#step:doReplaceDefenseWithArcheryOnFighterLevelGain` | `tasks/target-replay-evidence/L15-RR08-character-creation-routes.json#L15-RR08-CHARACTER-CREATION-ROUTES replay action=doReplaceDefenseWithArcheryOnFighterLevelGain#step:doReplaceDefenseWithArcheryOnFighterLevelGain` | `src/tests/mod.rs` | `covered` |
| `cleanroom-input/qnt/character-creation-runtime/character-creation-fighter-fighting-style-selected-identity.mbt.qnt#step:doReplaceDefenseWithGreatWeaponFightingOnFighterLevelGain` | `tasks/target-replay-evidence/L15-RR08-character-creation-routes.json#L15-RR08-CHARACTER-CREATION-ROUTES replay action=doReplaceDefenseWithGreatWeaponFightingOnFighterLevelGain#step:doReplaceDefenseWithGreatWeaponFightingOnFighterLevelGain` | `src/tests/mod.rs` | `covered` |
| `cleanroom-input/qnt/character-creation-runtime/character-creation-fighter-fighting-style-selected-identity.mbt.qnt#step:doReplaceDefenseWithTwoWeaponFightingOnFighterLevelGain` | `tasks/target-replay-evidence/L15-RR08-character-creation-routes.json#L15-RR08-CHARACTER-CREATION-ROUTES replay action=doReplaceDefenseWithTwoWeaponFightingOnFighterLevelGain#step:doReplaceDefenseWithTwoWeaponFightingOnFighterLevelGain` | `src/tests/mod.rs` | `covered` |
| `cleanroom-input/qnt/character-creation-runtime/character-creation-fighter-fighting-style-selected-identity.mbt.qnt#step:doSelectArcheryFightingStyle` | `tasks/target-replay-evidence/L15-RR08-character-creation-routes.json#L15-RR08-CHARACTER-CREATION-ROUTES replay action=doSelectArcheryFightingStyle#step:doSelectArcheryFightingStyle` | `src/tests/mod.rs` | `covered` |
| `cleanroom-input/qnt/character-creation-runtime/character-creation-fighter-fighting-style-selected-identity.mbt.qnt#step:doSelectDefenseFightingStyle` | `tasks/target-replay-evidence/L15-RR08-character-creation-routes.json#L15-RR08-CHARACTER-CREATION-ROUTES replay action=doSelectDefenseFightingStyle#step:doSelectDefenseFightingStyle` | `src/tests/mod.rs` | `covered` |
| `cleanroom-input/qnt/character-creation-runtime/character-creation-fighter-fighting-style-selected-identity.mbt.qnt#step:doSelectGreatWeaponFightingStyle` | `tasks/target-replay-evidence/L15-RR08-character-creation-routes.json#L15-RR08-CHARACTER-CREATION-ROUTES replay action=doSelectGreatWeaponFightingStyle#step:doSelectGreatWeaponFightingStyle` | `src/tests/mod.rs` | `covered` |
| `cleanroom-input/qnt/character-creation-runtime/character-creation-fighter-fighting-style-selected-identity.mbt.qnt#step:doSelectTwoWeaponFightingStyle` | `tasks/target-replay-evidence/L15-RR08-character-creation-routes.json#L15-RR08-CHARACTER-CREATION-ROUTES replay action=doSelectTwoWeaponFightingStyle#step:doSelectTwoWeaponFightingStyle` | `src/tests/mod.rs` | `covered` |
| `cleanroom-input/qnt/character-creation-runtime/character-creation-rogue-expertise-selected-identity.mbt.qnt#step:doSelectLevelOneOwnedSkillExpertise` | `tasks/target-replay-evidence/L15-RR08-character-creation-routes.json#L15-RR08-CHARACTER-CREATION-ROUTES replay action=doSelectLevelOneOwnedSkillExpertise#step:doSelectLevelOneOwnedSkillExpertise` | `src/tests/mod.rs` | `covered` |
| `cleanroom-input/qnt/character-creation-runtime/character-creation-runtime.mbt.qnt#step:doFillAbilityScoresOnly` | `tasks/target-replay-evidence/L15-RR08-character-creation-routes.json#L15-RR08-CHARACTER-CREATION-ROUTES replay action=doFillAbilityScoresOnly#step:doFillAbilityScoresOnly` | `src/tests/mod.rs` | `covered` |
| `cleanroom-input/qnt/character-creation-runtime/character-creation-runtime.mbt.qnt#step:doFillInitialChoicesOnly` | `tasks/target-replay-evidence/L15-RR08-character-creation-routes.json#L15-RR08-CHARACTER-CREATION-ROUTES replay action=doFillInitialChoicesOnly#step:doFillInitialChoicesOnly` | `src/tests/mod.rs` | `covered` |
| `cleanroom-input/qnt/character-creation-runtime/character-creation-runtime.mbt.qnt#step:doFillInitialManifest` | `tasks/target-replay-evidence/L15-RR08-character-creation-routes.json#L15-RR08-CHARACTER-CREATION-ROUTES replay action=doFillInitialManifest#step:doFillInitialManifest` | `src/tests/mod.rs` | `covered` |
| `cleanroom-input/qnt/character-creation-runtime/character-creation-runtime.mbt.qnt#step:doFillManifestChoices` | `tasks/target-replay-evidence/L15-RR08-character-creation-routes.json#L15-RR08-CHARACTER-CREATION-ROUTES replay action=doFillManifestChoices#step:doFillManifestChoices` | `src/tests/mod.rs` | `covered` |
| `cleanroom-input/qnt/character-creation-runtime/character-creation-runtime.mbt.qnt#step:doFillManifestLoadout` | `tasks/target-replay-evidence/L15-RR08-character-creation-routes.json#L15-RR08-CHARACTER-CREATION-ROUTES replay action=doFillManifestLoadout#step:doFillManifestLoadout` | `src/tests/mod.rs` | `covered` |
| `cleanroom-input/qnt/character-creation-runtime/character-creation-runtime.mbt.qnt#step:doFillManifestPurchase` | `tasks/target-replay-evidence/L15-RR08-character-creation-routes.json#L15-RR08-CHARACTER-CREATION-ROUTES replay action=doFillManifestPurchase#step:doFillManifestPurchase` | `src/tests/mod.rs` | `covered` |
| `cleanroom-input/qnt/character-creation-runtime/character-creation-runtime.mbt.qnt#step:doRejectClosedInitialProgressionHole` | `tasks/target-replay-evidence/L15-RR08-character-creation-routes.json#L15-RR08-CHARACTER-CREATION-ROUTES replay action=doRejectClosedInitialProgressionHole#step:doRejectClosedInitialProgressionHole` | `src/tests/mod.rs` | `covered` |
| `cleanroom-input/qnt/character-creation-runtime/character-creation-runtime.mbt.qnt#step:doRejectDuplicateFill` | `tasks/target-replay-evidence/L15-RR08-character-creation-routes.json#L15-RR08-CHARACTER-CREATION-ROUTES replay action=doRejectDuplicateFill#step:doRejectDuplicateFill` | `src/tests/mod.rs` | `covered` |
| `cleanroom-input/qnt/character-creation-runtime/character-creation-runtime.mbt.qnt#step:doRejectDuplicateLanguage` | `tasks/target-replay-evidence/L15-RR08-character-creation-routes.json#L15-RR08-CHARACTER-CREATION-ROUTES replay action=doRejectDuplicateLanguage#step:doRejectDuplicateLanguage` | `src/tests/mod.rs` | `covered` |
| `cleanroom-input/qnt/character-creation-runtime/character-creation-runtime.mbt.qnt#step:doRejectStaleInitialManifest` | `tasks/target-replay-evidence/L15-RR08-character-creation-routes.json#L15-RR08-CHARACTER-CREATION-ROUTES replay action=doRejectStaleInitialManifest#step:doRejectStaleInitialManifest` | `src/tests/mod.rs` | `covered` |
| `cleanroom-input/qnt/character-creation-runtime/character-creation-runtime.mbt.qnt#step:doRejectTooFewLanguages` | `tasks/target-replay-evidence/L15-RR08-character-creation-routes.json#L15-RR08-CHARACTER-CREATION-ROUTES replay action=doRejectTooFewLanguages#step:doRejectTooFewLanguages` | `src/tests/mod.rs` | `covered` |
| `cleanroom-input/qnt/character-creation-runtime/character-creation-runtime.mbt.qnt#step:doRejectTooManyLanguages` | `tasks/target-replay-evidence/L15-RR08-character-creation-routes.json#L15-RR08-CHARACTER-CREATION-ROUTES replay action=doRejectTooManyLanguages#step:doRejectTooManyLanguages` | `src/tests/mod.rs` | `covered` |
| `cleanroom-input/qnt/character-creation-runtime/character-creation-runtime.mbt.qnt#step:doRejectUnknownLoadoutArmor` | `tasks/target-replay-evidence/L15-RR08-character-creation-routes.json#L15-RR08-CHARACTER-CREATION-ROUTES replay action=doRejectUnknownLoadoutArmor#step:doRejectUnknownLoadoutArmor` | `src/tests/mod.rs` | `covered` |
| `cleanroom-input/qnt/character-creation-runtime/character-creation-runtime.mbt.qnt#step:doRejectUnsupportedClassEquipment` | `tasks/target-replay-evidence/L15-RR08-character-creation-routes.json#L15-RR08-CHARACTER-CREATION-ROUTES replay action=doRejectUnsupportedClassEquipment#step:doRejectUnsupportedClassEquipment` | `src/tests/mod.rs` | `covered` |
| `cleanroom-input/qnt/character-creation-runtime/character-creation-runtime.mbt.qnt#step:doRejectUnsupportedLanguage` | `tasks/target-replay-evidence/L15-RR08-character-creation-routes.json#L15-RR08-CHARACTER-CREATION-ROUTES replay action=doRejectUnsupportedLanguage#step:doRejectUnsupportedLanguage` | `src/tests/mod.rs` | `covered` |
| `cleanroom-input/qnt/character-creation-runtime/character-creation-runtime.mbt.qnt#step:doRejectWrongKindPrimaryClass` | `tasks/target-replay-evidence/L15-RR08-character-creation-routes.json#L15-RR08-CHARACTER-CREATION-ROUTES replay action=doRejectWrongKindPrimaryClass#step:doRejectWrongKindPrimaryClass` | `src/tests/mod.rs` | `covered` |
| `cleanroom-input/qnt/character-creation-runtime/character-creation-warlock-eldritch-invocations-selected-identity.mbt.qnt#step:doGainLevelTwoInvocations` | `tasks/target-replay-evidence/L15-RR08-character-creation-routes.json#L15-RR08-CHARACTER-CREATION-ROUTES replay action=doGainLevelTwoInvocations#step:doGainLevelTwoInvocations` | `src/tests/mod.rs` | `covered` |
| `cleanroom-input/qnt/character-creation-runtime/character-creation-warlock-eldritch-invocations-selected-identity.mbt.qnt#step:doRejectDuplicateInvocationSelections` | `tasks/target-replay-evidence/L15-RR08-character-creation-routes.json#L15-RR08-CHARACTER-CREATION-ROUTES replay action=doRejectDuplicateInvocationSelections#step:doRejectDuplicateInvocationSelections` | `src/tests/mod.rs` | `covered` |
| `cleanroom-input/qnt/character-creation-runtime/character-creation-warlock-eldritch-invocations-selected-identity.mbt.qnt#step:doReplaceArmorWithEldritchMindOnWarlockLevelGain` | `tasks/target-replay-evidence/L15-RR08-character-creation-routes.json#L15-RR08-CHARACTER-CREATION-ROUTES replay action=doReplaceArmorWithEldritchMindOnWarlockLevelGain#step:doReplaceArmorWithEldritchMindOnWarlockLevelGain` | `src/tests/mod.rs` | `covered` |
| `cleanroom-input/qnt/character-creation-runtime/character-creation-warlock-eldritch-invocations-selected-identity.mbt.qnt#step:doReplaceRepeatableInvocationByChoice` | `tasks/target-replay-evidence/L15-RR08-character-creation-routes.json#L15-RR08-CHARACTER-CREATION-ROUTES replay action=doReplaceRepeatableInvocationByChoice#step:doReplaceRepeatableInvocationByChoice` | `src/tests/mod.rs` | `covered` |
| `cleanroom-input/qnt/character-creation-runtime/character-creation-warlock-eldritch-invocations-selected-identity.mbt.qnt#step:doSelectLevelOneArmorOfShadows` | `tasks/target-replay-evidence/L15-RR08-character-creation-routes.json#L15-RR08-CHARACTER-CREATION-ROUTES replay action=doSelectLevelOneArmorOfShadows#step:doSelectLevelOneArmorOfShadows` | `src/tests/mod.rs` | `covered` |
| `cleanroom-input/qnt/character-creation-runtime/character-creation-weapon-mastery-containers-selected-identity.mbt.qnt#step:doFinalizeBarbarianWeaponMastery` | `tasks/target-replay-evidence/L15-RR08-character-creation-routes.json#L15-RR08-CHARACTER-CREATION-ROUTES replay action=doFinalizeBarbarianWeaponMastery#step:doFinalizeBarbarianWeaponMastery` | `src/tests/mod.rs` | `covered` |
| `cleanroom-input/qnt/character-creation-runtime/character-creation-weapon-mastery-containers-selected-identity.mbt.qnt#step:doFinalizeFighterWeaponMastery` | `tasks/target-replay-evidence/L15-RR08-character-creation-routes.json#L15-RR08-CHARACTER-CREATION-ROUTES replay action=doFinalizeFighterWeaponMastery#step:doFinalizeFighterWeaponMastery` | `src/tests/mod.rs` | `covered` |
| `cleanroom-input/qnt/character-creation-runtime/character-creation-weapon-mastery-containers-selected-identity.mbt.qnt#step:doFinalizePaladinWeaponMastery` | `tasks/target-replay-evidence/L15-RR08-character-creation-routes.json#L15-RR08-CHARACTER-CREATION-ROUTES replay action=doFinalizePaladinWeaponMastery#step:doFinalizePaladinWeaponMastery` | `src/tests/mod.rs` | `covered` |
| `cleanroom-input/qnt/character-creation-runtime/character-creation-weapon-mastery-containers-selected-identity.mbt.qnt#step:doFinalizeRangerWeaponMastery` | `tasks/target-replay-evidence/L15-RR08-character-creation-routes.json#L15-RR08-CHARACTER-CREATION-ROUTES replay action=doFinalizeRangerWeaponMastery#step:doFinalizeRangerWeaponMastery` | `src/tests/mod.rs` | `covered` |
| `cleanroom-input/qnt/character-creation-runtime/character-creation-weapon-mastery-containers-selected-identity.mbt.qnt#step:doFinalizeRogueWeaponMastery` | `tasks/target-replay-evidence/L15-RR08-character-creation-routes.json#L15-RR08-CHARACTER-CREATION-ROUTES replay action=doFinalizeRogueWeaponMastery#step:doFinalizeRogueWeaponMastery` | `src/tests/mod.rs` | `covered` |

Target replay evidence:

- Evidence file: `tasks/target-replay-evidence/L15-RR08-character-creation-routes.json`
- Target profile: `rust`
- Target profile SHA-256: `6d4cc6c6a4769962798133d57aff01438fb2b661941f71d1aa8a3333f4b7ecc1`
- Reproduction trace id prefix: `L15-RR08-CHARACTER-CREATION-ROUTES replay action=<branchAction>`

Harness artifacts:

- Start gate: `tasks/history/L15-RR08-CHARACTER-CREATION-ROUTES/START_GATE.json`
- Engine depth: `tasks/history/L15-RR08-CHARACTER-CREATION-ROUTES/ENGINE_DEPTH_MANIFEST.json`
- State ownership: `tasks/history/L15-RR08-CHARACTER-CREATION-ROUTES/STATE_OWNER_MANIFEST.json`
- Reviewer loop: `tasks/history/L15-RR08-CHARACTER-CREATION-ROUTES/REVIEW_LOOP.json`
- Decider decision: `tasks/history/L15-RR08-CHARACTER-CREATION-ROUTES/DECIDER_DECISION.json`
- Run ledger: `tasks/RUN_LEDGER.json`

Remaining gaps:

- `_none_` for in-scope L15-RR08 obligations. Source marks Rogue level 6 expertise and Warlock level 5 retained-prerequisite rejection outside this lane's level-1/2 acceptance denominator.

Verification results:

- `cargo test character_creation` passed.
- `cargo test class_feature` passed.
- `cargo test cleric_druid_order` passed.
- `cargo test fighting_style` passed.
- `cargo test rogue_expertise` passed.
- `cargo test warlock_invocation` passed.
- `cargo test weapon_mastery` passed.
- `cargo fmt --check` passed.
- `cargo test` passed.
- `cargo clippy --all-targets -- -D warnings` passed.
- `node scripts/check-cleanroom-harness.cjs` passed.

## L15-RR10-CHARACTER-BATTLE-HANDOFF-ROUTES: character-battle handoff routes

- Manifest source commit SHA: `564376fd95218a209bb9eae5c9ccb54ca3e04a52`
- Source branch inventory SHA: `4bb2b20a85d94e3b90b7c59cbfe6e1edd5ab3ef40410641e999527861f3d3a32`
- Selected drivers: `cleanroom-input/qnt/character-battle-runtime/character-battle-init-projection.mbt.qnt`, `cleanroom-input/qnt/character-battle-runtime/character-battle-origin-feat-selected-identity.mbt.qnt`, `cleanroom-input/qnt/character-battle-runtime/character-battle-settlement.mbt.qnt`, `cleanroom-input/qnt/character-battle-runtime/character-layer-projection-lifecycle.mbt.qnt`, `cleanroom-input/qnt/character-battle-runtime/character-sheet-feature-resources.mbt.qnt`
- Machine-readable run ledger: `tasks/RUN_LEDGER.json`

Allowed inputs used:

- `cleanroom-input/MANIFEST.md`
- `cleanroom-input/branch-coverage/source-branch-inventory.json`
- `cleanroom-input/branch-coverage/reducer-route-inventory.json`
- Selected `cleanroom-input/qnt/character-battle-runtime/*.mbt.qnt` drivers and route connectors
- `cleanroom-input/raw/srd-5.2.1/Playing-the-Game.md`, `Rules-Glossary.md`, `Classes/Paladin.md`, `Classes/Monk.md`, `Classes/Sorcerer.md`, `Classes/Warlock.md`, `Character-Creation.md`, `Character-Origins.md`, and `Feats.md`
- `cleanroom-input/domain/UBIQUITOUS_LANGUAGE.md` and `CLEANROOM_ASSUMPTIONS.md`
- Repo-local `src/**`, `tasks/**`, and Cargo tooling

Behavior implemented:

- Added shared `CharacterBattleRouteEvent` route-owner vocabulary for sheet-to-battle projection, battle runtime entry, settlement, and rejection holes.
- Added accepted pure Pact Slot, mixed Spell/Pact rejection, and active battle-state settlement branch handling.
- Updated handoff adapters to compare observed replay against independent expected projection records and adapter-local literal qRoute witnesses; expected routes no longer share observed append/build helpers.
- Kept selected authored identity in adapter/test/evidence boundaries; production route/fact code consumes typed subjects, owners, fills, holes, and state-owner facts.

Generated branch coverage:

| Obligation | Target replay evidence | Diagnostic tests | Status |
| --- | --- | --- | --- |
| `cleanroom-input/qnt/character-battle-runtime/character-battle-origin-feat-selected-identity.mbt.qnt#step:doFinalizeCriminalAlertOriginFeat` | `tasks/target-replay-evidence/L15-RR10-character-battle-handoff-routes.json#L15-RR10-CHARACTER-BATTLE-HANDOFF-ROUTES route action=doFinalizeCriminalAlertOriginFeat#step:doFinalizeCriminalAlertOriginFeat` | `src/tests/mod.rs` | `covered` |
| `cleanroom-input/qnt/character-battle-runtime/character-battle-origin-feat-selected-identity.mbt.qnt#step:doProjectAlertInitiativeHandoff` | `tasks/target-replay-evidence/L15-RR10-character-battle-handoff-routes.json#L15-RR10-CHARACTER-BATTLE-HANDOFF-ROUTES route action=doProjectAlertInitiativeHandoff#step:doProjectAlertInitiativeHandoff` | `src/tests/mod.rs` | `covered` |
| `cleanroom-input/qnt/character-battle-runtime/character-battle-init-projection.mbt.qnt#step:doProjectSheetHitPointsArmorClassConditionsAndProfiles` | `tasks/target-replay-evidence/L15-RR10-character-battle-handoff-routes.json#L15-RR10-CHARACTER-BATTLE-HANDOFF-ROUTES route action=doProjectSheetHitPointsArmorClassConditionsAndProfiles#step:doProjectSheetHitPointsArmorClassConditionsAndProfiles` | `src/tests/mod.rs` | `covered` |
| `cleanroom-input/qnt/character-battle-runtime/character-battle-init-projection.mbt.qnt#step:doProjectPurePactMagicSlot` | `tasks/target-replay-evidence/L15-RR10-character-battle-handoff-routes.json#L15-RR10-CHARACTER-BATTLE-HANDOFF-ROUTES route action=doProjectPurePactMagicSlot#step:doProjectPurePactMagicSlot` | `src/tests/mod.rs` | `covered` |
| `cleanroom-input/qnt/character-battle-runtime/character-battle-init-projection.mbt.qnt#step:doRejectMixedSpellAndPactSlotInit` | `tasks/target-replay-evidence/L15-RR10-character-battle-handoff-routes.json#L15-RR10-CHARACTER-BATTLE-HANDOFF-ROUTES route action=doRejectMixedSpellAndPactSlotInit#step:doRejectMixedSpellAndPactSlotInit` | `src/tests/mod.rs` | `covered` |
| `cleanroom-input/qnt/character-battle-runtime/character-battle-init-projection.mbt.qnt#step:doRejectBuildMaximumAboveBuildMaximum` | `tasks/target-replay-evidence/L15-RR10-character-battle-handoff-routes.json#L15-RR10-CHARACTER-BATTLE-HANDOFF-ROUTES route action=doRejectBuildMaximumAboveBuildMaximum#step:doRejectBuildMaximumAboveBuildMaximum` | `src/tests/mod.rs` | `covered` |
| `cleanroom-input/qnt/character-battle-runtime/character-battle-init-projection.mbt.qnt#step:doRejectStableRecoveryProgressDuringInit` | `tasks/target-replay-evidence/L15-RR10-character-battle-handoff-routes.json#L15-RR10-CHARACTER-BATTLE-HANDOFF-ROUTES route action=doRejectStableRecoveryProgressDuringInit#step:doRejectStableRecoveryProgressDuringInit` | `src/tests/mod.rs` | `covered` |
| `cleanroom-input/qnt/character-battle-runtime/character-battle-settlement.mbt.qnt#step:doSettleHitPointsConditionsSlotsAndPreservedSheetState` | `tasks/target-replay-evidence/L15-RR10-character-battle-handoff-routes.json#L15-RR10-CHARACTER-BATTLE-HANDOFF-ROUTES route action=doSettleHitPointsConditionsSlotsAndPreservedSheetState#step:doSettleHitPointsConditionsSlotsAndPreservedSheetState` | `src/tests/mod.rs` | `covered` |
| `cleanroom-input/qnt/character-battle-runtime/character-battle-settlement.mbt.qnt#step:doSettlePurePactMagicSlotExpenditure` | `tasks/target-replay-evidence/L15-RR10-character-battle-handoff-routes.json#L15-RR10-CHARACTER-BATTLE-HANDOFF-ROUTES route action=doSettlePurePactMagicSlotExpenditure#step:doSettlePurePactMagicSlotExpenditure` | `src/tests/mod.rs` | `covered` |
| `cleanroom-input/qnt/character-battle-runtime/character-battle-settlement.mbt.qnt#step:doRejectMixedSpellAndPactSlotSettlement` | `tasks/target-replay-evidence/L15-RR10-character-battle-handoff-routes.json#L15-RR10-CHARACTER-BATTLE-HANDOFF-ROUTES route action=doRejectMixedSpellAndPactSlotSettlement#step:doRejectMixedSpellAndPactSlotSettlement` | `src/tests/mod.rs` | `covered` |
| `cleanroom-input/qnt/character-battle-runtime/character-battle-settlement.mbt.qnt#step:doSettleFeatureResourceExpenditure` | `tasks/target-replay-evidence/L15-RR10-character-battle-handoff-routes.json#L15-RR10-CHARACTER-BATTLE-HANDOFF-ROUTES route action=doSettleFeatureResourceExpenditure#step:doSettleFeatureResourceExpenditure` | `src/tests/mod.rs` | `covered` |
| `cleanroom-input/qnt/character-battle-runtime/character-battle-settlement.mbt.qnt#step:doRejectMismatchedCharacterIdentity` | `tasks/target-replay-evidence/L15-RR10-character-battle-handoff-routes.json#L15-RR10-CHARACTER-BATTLE-HANDOFF-ROUTES route action=doRejectMismatchedCharacterIdentity#step:doRejectMismatchedCharacterIdentity` | `src/tests/mod.rs` | `covered` |
| `cleanroom-input/qnt/character-battle-runtime/character-battle-settlement.mbt.qnt#step:doRejectMaximumHpDrift` | `tasks/target-replay-evidence/L15-RR10-character-battle-handoff-routes.json#L15-RR10-CHARACTER-BATTLE-HANDOFF-ROUTES route action=doRejectMaximumHpDrift#step:doRejectMaximumHpDrift` | `src/tests/mod.rs` | `covered` |
| `cleanroom-input/qnt/character-battle-runtime/character-battle-settlement.mbt.qnt#step:doRejectActiveWildShapeHandoff` | `tasks/target-replay-evidence/L15-RR10-character-battle-handoff-routes.json#L15-RR10-CHARACTER-BATTLE-HANDOFF-ROUTES route action=doRejectActiveWildShapeHandoff#step:doRejectActiveWildShapeHandoff` | `src/tests/mod.rs` | `covered` |
| `cleanroom-input/qnt/character-battle-runtime/character-battle-settlement.mbt.qnt#step:doRejectActiveBattleStateHandoff` | `tasks/target-replay-evidence/L15-RR10-character-battle-handoff-routes.json#L15-RR10-CHARACTER-BATTLE-HANDOFF-ROUTES route action=doRejectActiveBattleStateHandoff#step:doRejectActiveBattleStateHandoff` | `src/tests/mod.rs` | `covered` |
| `cleanroom-input/qnt/character-battle-runtime/character-battle-settlement.mbt.qnt#step:doRejectStableRecoveryProgressHandoff` | `tasks/target-replay-evidence/L15-RR10-character-battle-handoff-routes.json#L15-RR10-CHARACTER-BATTLE-HANDOFF-ROUTES route action=doRejectStableRecoveryProgressHandoff#step:doRejectStableRecoveryProgressHandoff` | `src/tests/mod.rs` | `covered` |
| `cleanroom-input/qnt/character-battle-runtime/character-battle-settlement.mbt.qnt#step:doSettleZeroHpStableLifecycle` | `tasks/target-replay-evidence/L15-RR10-character-battle-handoff-routes.json#L15-RR10-CHARACTER-BATTLE-HANDOFF-ROUTES route action=doSettleZeroHpStableLifecycle#step:doSettleZeroHpStableLifecycle` | `src/tests/mod.rs` | `covered` |
| `cleanroom-input/qnt/character-battle-runtime/character-layer-projection-lifecycle.mbt.qnt#step:doFinalizeDraftToBuild` | `tasks/target-replay-evidence/L15-RR10-character-battle-handoff-routes.json#L15-RR10-CHARACTER-BATTLE-HANDOFF-ROUTES route action=doFinalizeDraftToBuild#step:doFinalizeDraftToBuild` | `src/tests/mod.rs` | `covered` |
| `cleanroom-input/qnt/character-battle-runtime/character-layer-projection-lifecycle.mbt.qnt#step:doCreateSheetFromBuild` | `tasks/target-replay-evidence/L15-RR10-character-battle-handoff-routes.json#L15-RR10-CHARACTER-BATTLE-HANDOFF-ROUTES route action=doCreateSheetFromBuild#step:doCreateSheetFromBuild` | `src/tests/mod.rs` | `covered` |
| `cleanroom-input/qnt/character-battle-runtime/character-layer-projection-lifecycle.mbt.qnt#step:doProjectSheetToBattleInit` | `tasks/target-replay-evidence/L15-RR10-character-battle-handoff-routes.json#L15-RR10-CHARACTER-BATTLE-HANDOFF-ROUTES route action=doProjectSheetToBattleInit#step:doProjectSheetToBattleInit` | `src/tests/mod.rs` | `covered` |
| `cleanroom-input/qnt/character-battle-runtime/character-layer-projection-lifecycle.mbt.qnt#step:doResolveSkeletonShortswordAttack` | `tasks/target-replay-evidence/L15-RR10-character-battle-handoff-routes.json#L15-RR10-CHARACTER-BATTLE-HANDOFF-ROUTES route action=doResolveSkeletonShortswordAttack#step:doResolveSkeletonShortswordAttack` | `src/tests/mod.rs` | `covered` |
| `cleanroom-input/qnt/character-battle-runtime/character-layer-projection-lifecycle.mbt.qnt#step:doSettleBattleToSheet` | `tasks/target-replay-evidence/L15-RR10-character-battle-handoff-routes.json#L15-RR10-CHARACTER-BATTLE-HANDOFF-ROUTES route action=doSettleBattleToSheet#step:doSettleBattleToSheet` | `src/tests/mod.rs` | `covered` |
| `cleanroom-input/qnt/character-battle-runtime/character-sheet-feature-resources.mbt.qnt#step:doLayOnHandsRestoresHpAndRemovesPoisoned` | `tasks/target-replay-evidence/L15-RR10-character-battle-handoff-routes.json#L15-RR10-CHARACTER-BATTLE-HANDOFF-ROUTES route action=doLayOnHandsRestoresHpAndRemovesPoisoned#step:doLayOnHandsRestoresHpAndRemovesPoisoned` | `src/tests/mod.rs` | `covered` |
| `cleanroom-input/qnt/character-battle-runtime/character-sheet-feature-resources.mbt.qnt#step:doRejectLayOnHandsOverspend` | `tasks/target-replay-evidence/L15-RR10-character-battle-handoff-routes.json#L15-RR10-CHARACTER-BATTLE-HANDOFF-ROUTES route action=doRejectLayOnHandsOverspend#step:doRejectLayOnHandsOverspend` | `src/tests/mod.rs` | `covered` |
| `cleanroom-input/qnt/character-battle-runtime/character-sheet-feature-resources.mbt.qnt#step:doLongRestClearsLayOnHandsPool` | `tasks/target-replay-evidence/L15-RR10-character-battle-handoff-routes.json#L15-RR10-CHARACTER-BATTLE-HANDOFF-ROUTES route action=doLongRestClearsLayOnHandsPool#step:doLongRestClearsLayOnHandsPool` | `src/tests/mod.rs` | `covered` |
| `cleanroom-input/qnt/character-battle-runtime/character-sheet-feature-resources.mbt.qnt#step:doShortRestRecoversUseCountPools` | `tasks/target-replay-evidence/L15-RR10-character-battle-handoff-routes.json#L15-RR10-CHARACTER-BATTLE-HANDOFF-ROUTES route action=doShortRestRecoversUseCountPools#step:doShortRestRecoversUseCountPools` | `src/tests/mod.rs` | `covered` |
| `cleanroom-input/qnt/character-battle-runtime/character-sheet-feature-resources.mbt.qnt#step:doLongRestClearsPointPoolAndUseState` | `tasks/target-replay-evidence/L15-RR10-character-battle-handoff-routes.json#L15-RR10-CHARACTER-BATTLE-HANDOFF-ROUTES route action=doLongRestClearsPointPoolAndUseState#step:doLongRestClearsPointPoolAndUseState` | `src/tests/mod.rs` | `covered` |
| `cleanroom-input/qnt/character-battle-runtime/character-sheet-feature-resources.mbt.qnt#step:doFontOfMagicSlotToPoints` | `tasks/target-replay-evidence/L15-RR10-character-battle-handoff-routes.json#L15-RR10-CHARACTER-BATTLE-HANDOFF-ROUTES route action=doFontOfMagicSlotToPoints#step:doFontOfMagicSlotToPoints` | `src/tests/mod.rs` | `covered` |
| `cleanroom-input/qnt/character-battle-runtime/character-sheet-feature-resources.mbt.qnt#step:doShortRestPreservesUncannyUseState` | `tasks/target-replay-evidence/L15-RR10-character-battle-handoff-routes.json#L15-RR10-CHARACTER-BATTLE-HANDOFF-ROUTES route action=doShortRestPreservesUncannyUseState#step:doShortRestPreservesUncannyUseState` | `src/tests/mod.rs` | `covered` |
| `cleanroom-input/qnt/character-battle-runtime/character-sheet-feature-resources.mbt.qnt#step:doLongRestClearsUncannyUseState` | `tasks/target-replay-evidence/L15-RR10-character-battle-handoff-routes.json#L15-RR10-CHARACTER-BATTLE-HANDOFF-ROUTES route action=doLongRestClearsUncannyUseState#step:doLongRestClearsUncannyUseState` | `src/tests/mod.rs` | `covered` |
| `cleanroom-input/qnt/character-battle-runtime/character-sheet-feature-resources.mbt.qnt#step:doUncannyMetabolismRecoversFocusAndHeals` | `tasks/target-replay-evidence/L15-RR10-character-battle-handoff-routes.json#L15-RR10-CHARACTER-BATTLE-HANDOFF-ROUTES route action=doUncannyMetabolismRecoversFocusAndHeals#step:doUncannyMetabolismRecoversFocusAndHeals` | `src/tests/mod.rs` | `covered` |
| `cleanroom-input/qnt/character-battle-runtime/character-sheet-feature-resources.mbt.qnt#step:doRejectUncannyMetabolismRepeatUse` | `tasks/target-replay-evidence/L15-RR10-character-battle-handoff-routes.json#L15-RR10-CHARACTER-BATTLE-HANDOFF-ROUTES route action=doRejectUncannyMetabolismRepeatUse#step:doRejectUncannyMetabolismRepeatUse` | `src/tests/mod.rs` | `covered` |

Target replay evidence:

- Evidence file: `tasks/target-replay-evidence/L15-RR10-character-battle-handoff-routes.json`
- Route evidence basis: independent adapter-local literal route witnesses derived from the selected `.mbt.qnt` route connectors and cleanroom facts.
- Target profile: `rust`
- Target profile SHA-256: `6d4cc6c6a4769962798133d57aff01438fb2b661941f71d1aa8a3333f4b7ecc1`
- Reproduction trace id prefix: `L15-RR10-CHARACTER-BATTLE-HANDOFF-ROUTES route action=<branchAction>`

Harness artifacts:

- Start gate: `tasks/history/L15-RR10-CHARACTER-BATTLE-HANDOFF-ROUTES/START_GATE.json`
- Engine depth: `tasks/history/L15-RR10-CHARACTER-BATTLE-HANDOFF-ROUTES/ENGINE_DEPTH_MANIFEST.json`
- State ownership: `tasks/history/L15-RR10-CHARACTER-BATTLE-HANDOFF-ROUTES/STATE_OWNER_MANIFEST.json`
- Reviewer loop: `tasks/history/L15-RR10-CHARACTER-BATTLE-HANDOFF-ROUTES/REVIEW_LOOP.json`
- Decider decision: `tasks/history/L15-RR10-CHARACTER-BATTLE-HANDOFF-ROUTES/DECIDER_DECISION.json`
- Run ledger: `tasks/RUN_LEDGER.json`

Remaining gaps:

- No blockers for harness-accepted L15-RR10 CP3 obligations. Transit/higher-scope rows are present in adapter tests where useful but are not cited as accepted coverage.

Verification results:

- `cargo fmt --check` passed.
- `cargo test` passed.
- `cargo clippy --all-targets -- -D warnings` passed.
- `node scripts/check-cleanroom-harness.cjs` passed after artifact update.
- `git diff --check 6af492188311839dd4839b464b2e7049e3330568...HEAD` passed.

## L15-RR07S-B-ZERO-HP-FORM-MOVEMENT-SUBSTRATES

Scope inputs:

- `cleanroom-input/qnt/battle-runtime/battle-runtime-healing-stabilization-selected-identity.mbt.qnt`
- `cleanroom-input/qnt/battle-runtime/battle-runtime-druid-wild-shape-form-lifecycle.mbt.qnt`
- `cleanroom-input/qnt/battle-runtime/battle-runtime-movement-forced-movement-selected-identity.mbt.qnt`
- Route connectors: `battle-runtime-zero-hit-point-stabilization.route.mbt.qnt`, `battle-runtime-druid-wild-shape-form-lifecycle.route.mbt.qnt`, and `battle-runtime-movement-forced-movement-selected-identity.route.mbt.qnt`

Behavior implemented:

- Added reducer route evidence for zero-HP stabilization through `ZeroHitPointStabilizationRouteSubject`, `BattleActionEconomyOwner`, and `BattleHitPointAndZeroHpLifecycleOwner`.
- Added active-form lifecycle evidence through action-economy, feature-resource, Temporary Hit Point, active-effect, creature-state, movement-resource, condition-lifecycle, HP/zero-HP, and turn-boundary owners.
- Added in-scope movement/forced-movement evidence for Dissonant Whispers, Command Flee, Expeditious Retreat, and Monk unarmored movement through forced-movement and movement-resource route subjects.
- Kept selected identity names at adapter/test/evidence boundaries; runtime route evidence uses typed route subjects, owner groups, holes, and fills.

Generated branch coverage:

| Obligation | Target replay evidence | Diagnostic tests | Status |
| --- | --- | --- | --- |
| `cleanroom-input/qnt/battle-runtime/battle-runtime-healing-stabilization-selected-identity.mbt.qnt#step:doResolveSpareTheDyingStable` | `tasks/target-replay-evidence/L15-RR07S-B-zero-hp-form-movement-substrates.json#L15-RR07S-B-ZERO-HP-FORM-MOVEMENT-SUBSTRATES route action=doResolveSpareTheDyingStable#step:doResolveSpareTheDyingStable` | `src/tests/mod.rs` | `covered` |
| `cleanroom-input/qnt/battle-runtime/battle-runtime-druid-wild-shape-form-lifecycle.mbt.qnt#step:doAssumeRidingHorse` | `tasks/target-replay-evidence/L15-RR07S-B-zero-hp-form-movement-substrates.json#L15-RR07S-B-ZERO-HP-FORM-MOVEMENT-SUBSTRATES route action=doAssumeRidingHorse#step:doAssumeRidingHorse` | `src/tests/mod.rs` | `covered` |
| `cleanroom-input/qnt/battle-runtime/battle-runtime-druid-wild-shape-form-lifecycle.mbt.qnt#step:doBeginNextTurn` | `tasks/target-replay-evidence/L15-RR07S-B-zero-hp-form-movement-substrates.json#L15-RR07S-B-ZERO-HP-FORM-MOVEMENT-SUBSTRATES route action=doBeginNextTurn#step:doBeginNextTurn` | `src/tests/mod.rs` | `covered` |
| `cleanroom-input/qnt/battle-runtime/battle-runtime-druid-wild-shape-form-lifecycle.mbt.qnt#step:doDeathReversion` | `tasks/target-replay-evidence/L15-RR07S-B-zero-hp-form-movement-substrates.json#L15-RR07S-B-ZERO-HP-FORM-MOVEMENT-SUBSTRATES route action=doDeathReversion#step:doDeathReversion` | `src/tests/mod.rs` | `covered` |
| `cleanroom-input/qnt/battle-runtime/battle-runtime-druid-wild-shape-form-lifecycle.mbt.qnt#step:doDismissForm` | `tasks/target-replay-evidence/L15-RR07S-B-zero-hp-form-movement-substrates.json#L15-RR07S-B-ZERO-HP-FORM-MOVEMENT-SUBSTRATES route action=doDismissForm#step:doDismissForm` | `src/tests/mod.rs` | `covered` |
| `cleanroom-input/qnt/battle-runtime/battle-runtime-druid-wild-shape-form-lifecycle.mbt.qnt#step:doIncapacitatedReversion` | `tasks/target-replay-evidence/L15-RR07S-B-zero-hp-form-movement-substrates.json#L15-RR07S-B-ZERO-HP-FORM-MOVEMENT-SUBSTRATES route action=doIncapacitatedReversion#step:doIncapacitatedReversion` | `src/tests/mod.rs` | `covered` |
| `cleanroom-input/qnt/battle-runtime/battle-runtime-druid-wild-shape-form-lifecycle.mbt.qnt#step:doReuseAsCat` | `tasks/target-replay-evidence/L15-RR07S-B-zero-hp-form-movement-substrates.json#L15-RR07S-B-ZERO-HP-FORM-MOVEMENT-SUBSTRATES route action=doReuseAsCat#step:doReuseAsCat` | `src/tests/mod.rs` | `covered` |
| `cleanroom-input/qnt/battle-runtime/battle-runtime-druid-wild-shape-form-lifecycle.mbt.qnt#step:doStutter` | `tasks/target-replay-evidence/L15-RR07S-B-zero-hp-form-movement-substrates.json#L15-RR07S-B-ZERO-HP-FORM-MOVEMENT-SUBSTRATES route action=doStutter#step:doStutter` | `src/tests/mod.rs` | `covered` |
| `cleanroom-input/qnt/battle-runtime/battle-runtime-movement-forced-movement-selected-identity.mbt.qnt#step:doCommandFleeTargetTurn` | `tasks/target-replay-evidence/L15-RR07S-B-zero-hp-form-movement-substrates.json#L15-RR07S-B-ZERO-HP-FORM-MOVEMENT-SUBSTRATES route action=doCommandFleeTargetTurn#step:doCommandFleeTargetTurn` | `src/tests/mod.rs` | `covered` |
| `cleanroom-input/qnt/battle-runtime/battle-runtime-movement-forced-movement-selected-identity.mbt.qnt#step:doDissonantWhispersForcedReactionMovement` | `tasks/target-replay-evidence/L15-RR07S-B-zero-hp-form-movement-substrates.json#L15-RR07S-B-ZERO-HP-FORM-MOVEMENT-SUBSTRATES route action=doDissonantWhispersForcedReactionMovement#step:doDissonantWhispersForcedReactionMovement` | `src/tests/mod.rs` | `covered` |
| `cleanroom-input/qnt/battle-runtime/battle-runtime-movement-forced-movement-selected-identity.mbt.qnt#step:doExpeditiousRetreatImmediateDash` | `tasks/target-replay-evidence/L15-RR07S-B-zero-hp-form-movement-substrates.json#L15-RR07S-B-ZERO-HP-FORM-MOVEMENT-SUBSTRATES route action=doExpeditiousRetreatImmediateDash#step:doExpeditiousRetreatImmediateDash` | `src/tests/mod.rs` | `covered` |
| `cleanroom-input/qnt/battle-runtime/battle-runtime-movement-forced-movement-selected-identity.mbt.qnt#step:doMonkUnarmoredMovementDash` | `tasks/target-replay-evidence/L15-RR07S-B-zero-hp-form-movement-substrates.json#L15-RR07S-B-ZERO-HP-FORM-MOVEMENT-SUBSTRATES route action=doMonkUnarmoredMovementDash#step:doMonkUnarmoredMovementDash` | `src/tests/mod.rs` | `covered` |

Blocked/out-of-lane obligations:

- `doBarbarianFastMovementDash` remains assigned to the later level-5 promotion lane.
- `doRangerRovingClimbSwimMovement` remains outside the level 1-5 denominator for this lane.

Verification results:

- Focused adapter tests passed for stabilization, Wild Shape form lifecycle, and movement/forced movement.
- Full verification commands are recorded in `tasks/RUN_LEDGER.json` for this task.

## L15-RR07S-C-WEAPON-BREATH-FEATURE-SUBSTRATES: weapon, breath, feature substrates

- Manifest source commit SHA: `564376fd95218a209bb9eae5c9ccb54ca3e04a52`
- Source branch inventory SHA: `4bb2b20a85d94e3b90b7c59cbfe6e1edd5ab3ef40410641e999527861f3d3a32`
- Selected drivers: `cleanroom-input/qnt/battle-runtime/battle-runtime-weapon-mastery-selected-identity.mbt.qnt`, `cleanroom-input/qnt/battle-runtime/battle-runtime-dragonborn-breath-weapon.mbt.qnt`, `cleanroom-input/qnt/battle-runtime/battle-runtime-feature-selected-identity.mbt.qnt`
- Machine-readable run ledger: `tasks/RUN_LEDGER.json`

Behavior implemented:

- Weapon mastery selected-identity replay routes Sap, Topple, and Cleave by typed `WeaponMasteryProperty` facts through reducer-observed start/discover/resolve entrypoint events.
- Dragonborn Breath Weapon replay routes attack-action area-save damage replacement by `DragonbornBreathWeaponFacts`, including reducer-observed resource, area-shape, and damage-roll rejection outcomes. Extra Attack continuation remains validated in state projection rather than claimed as a separate observed route event.
- Active feature spell-benefit replay routes Innate Sorcery by typed spell-benefit eligibility facts, not authored feature identity; route evidence is limited to reducer-observed feature-substrate entrypoint events.
- Focused adapter route checks compare `BattleEntrypointTrace` route output emitted by production reducer observer entrypoints against independent literal `ReducerRouteEvent` expected records.
- Selected authored identity remains confined to adapter branch labels, tests, and evidence payloads.

Generated branch coverage:

| Obligation | Target replay evidence | Diagnostic tests | Status |
| --- | --- | --- | --- |
| `cleanroom-input/qnt/battle-runtime/battle-runtime-weapon-mastery-selected-identity.mbt.qnt#step:doResolveSapMasteryPropertyHit` | `tasks/target-replay-evidence/L15-RR07S-C-weapon-breath-feature-substrates.json#L15-RR07S-C replay action=doResolveSapMasteryPropertyHit#step:doResolveSapMasteryPropertyHit` | `src/tests/mod.rs` | `covered` |
| `cleanroom-input/qnt/battle-runtime/battle-runtime-weapon-mastery-selected-identity.mbt.qnt#step:doResolveToppleMasteryPropertyFailedSavingThrow` | `tasks/target-replay-evidence/L15-RR07S-C-weapon-breath-feature-substrates.json#L15-RR07S-C replay action=doResolveToppleMasteryPropertyFailedSavingThrow#step:doResolveToppleMasteryPropertyFailedSavingThrow` | `src/tests/mod.rs` | `covered` |
| `cleanroom-input/qnt/battle-runtime/battle-runtime-weapon-mastery-selected-identity.mbt.qnt#step:doResolveCleaveMasteryPropertySecondTargetHit` | `tasks/target-replay-evidence/L15-RR07S-C-weapon-breath-feature-substrates.json#L15-RR07S-C replay action=doResolveCleaveMasteryPropertySecondTargetHit#step:doResolveCleaveMasteryPropertySecondTargetHit` | `src/tests/mod.rs` | `covered` |
| `cleanroom-input/qnt/battle-runtime/battle-runtime-dragonborn-breath-weapon.mbt.qnt#step:doResolveBreathWeapon` | `tasks/target-replay-evidence/L15-RR07S-C-weapon-breath-feature-substrates.json#L15-RR07S-C replay action=doResolveBreathWeapon#step:doResolveBreathWeapon` | `src/tests/mod.rs` | `covered` |
| `cleanroom-input/qnt/battle-runtime/battle-runtime-dragonborn-breath-weapon.mbt.qnt#step:doOpenExtraAttackSlot` | `tasks/target-replay-evidence/L15-RR07S-C-weapon-breath-feature-substrates.json#L15-RR07S-C replay action=doOpenExtraAttackSlot#step:doOpenExtraAttackSlot` | `src/tests/mod.rs` | `covered` |
| `cleanroom-input/qnt/battle-runtime/battle-runtime-dragonborn-breath-weapon.mbt.qnt#step:doRejectMissingResource` | `tasks/target-replay-evidence/L15-RR07S-C-weapon-breath-feature-substrates.json#L15-RR07S-C replay action=doRejectMissingResource#step:doRejectMissingResource` | `src/tests/mod.rs` | `covered` |
| `cleanroom-input/qnt/battle-runtime/battle-runtime-dragonborn-breath-weapon.mbt.qnt#step:doRejectMismatchedArea` | `tasks/target-replay-evidence/L15-RR07S-C-weapon-breath-feature-substrates.json#L15-RR07S-C replay action=doRejectMismatchedArea#step:doRejectMismatchedArea` | `src/tests/mod.rs` | `covered` |
| `cleanroom-input/qnt/battle-runtime/battle-runtime-dragonborn-breath-weapon.mbt.qnt#step:doRejectInvalidDamageRoll` | `tasks/target-replay-evidence/L15-RR07S-C-weapon-breath-feature-substrates.json#L15-RR07S-C replay action=doRejectInvalidDamageRoll#step:doRejectInvalidDamageRoll` | `src/tests/mod.rs` | `covered` |
| `cleanroom-input/qnt/battle-runtime/battle-runtime-feature-selected-identity.mbt.qnt#step:doActivateInnateSorcery` | `tasks/target-replay-evidence/L15-RR07S-C-weapon-breath-feature-substrates.json#L15-RR07S-C replay action=doActivateInnateSorcery#step:doActivateInnateSorcery` | `src/tests/mod.rs` | `covered` |
| `cleanroom-input/qnt/battle-runtime/battle-runtime-feature-selected-identity.mbt.qnt#step:doProjectInnateSorcerySpellBenefits` | `tasks/target-replay-evidence/L15-RR07S-C-weapon-breath-feature-substrates.json#L15-RR07S-C replay action=doProjectInnateSorcerySpellBenefits#step:doProjectInnateSorcerySpellBenefits` | `src/tests/mod.rs` | `covered` |
| `cleanroom-input/qnt/battle-runtime/battle-runtime-feature-selected-identity.mbt.qnt#step:doExcludeInnateSorceryNonSorcererSpellBenefits` | `tasks/target-replay-evidence/L15-RR07S-C-weapon-breath-feature-substrates.json#L15-RR07S-C replay action=doExcludeInnateSorceryNonSorcererSpellBenefits#step:doExcludeInnateSorceryNonSorcererSpellBenefits` | `src/tests/mod.rs` | `covered` |

Harness artifacts:

- Start gate: `tasks/history/L15-RR07S-C-WEAPON-BREATH-FEATURE-SUBSTRATES/START_GATE.json`
- Engine depth: `tasks/history/L15-RR07S-C-WEAPON-BREATH-FEATURE-SUBSTRATES/ENGINE_DEPTH_MANIFEST.json`
- State ownership: `tasks/history/L15-RR07S-C-WEAPON-BREATH-FEATURE-SUBSTRATES/STATE_OWNER_MANIFEST.json`
- Reviewer loop: `tasks/history/L15-RR07S-C-WEAPON-BREATH-FEATURE-SUBSTRATES/REVIEW_LOOP.json`
- Decider decision: `tasks/history/L15-RR07S-C-WEAPON-BREATH-FEATURE-SUBSTRATES/DECIDER_DECISION.json`

Verification results:

- `cargo fmt --check` passed.
- Focused route evidence tests passed: `cargo test weapon_mastery_selected_identity_adapter_replays_all_branches`, `cargo test dragonborn_breath_weapon_adapter_replays_all_branches`, and `cargo test feature_selected_identity_adapter_replays_all_branches`.
- `cargo test` passed.
- `cargo clippy --all-targets -- -D warnings` passed.
- `node scripts/check-cleanroom-harness.cjs` passed after artifact schema convergence.
- `git diff --check 8d8576315773c721128fabaf79319bdbf2921eaa...HEAD` passed.

## L15-RR07S-A-PASSIVE-ROLL-RESOURCE-SUBSTRATES

- Ledger task: L15-RR07S-A-PASSIVE-ROLL-RESOURCE-SUBSTRATES
- Machine-readable run ledger: tasks/RUN_LEDGER.json
- Current manifest source commit SHA: 564376fd95218a209bb9eae5c9ccb54ca3e04a52
- Source branch inventory SHA: 4bb2b20a85d94e3b90b7c59cbfe6e1edd5ab3ef40410641e999527861f3d3a32
- Target profile: `rust`
- Target profile SHA-256: `6d4cc6c6a4769962798133d57aff01438fb2b661941f71d1aa8a3333f4b7ecc1`

### Allowed inputs used:
- cleanroom-input/qnt/battle-runtime/battle-runtime-adrenaline-rush.mbt.qnt
- cleanroom-input/qnt/battle-runtime/battle-runtime-halfling-nimbleness-selected-identity.mbt.qnt
- cleanroom-input/qnt/battle-runtime/battle-runtime-species-passive-trait-selected-identity.mbt.qnt
- cleanroom-input/qnt/battle-runtime/battle-runtime-danger-sense-selected-identity.mbt.qnt
- cleanroom-input/qnt/battle-runtime/battle-runtime-roll-modifier-buff-selected-identity.mbt.qnt

### Behavior implemented:
Passive trait, passive movement, passive roll-mode, roll-modifier buff, Danger Sense, and feature-resource-backed Adrenaline behavior route through typed substrate owners. Selected species, feature, and spell identity is retained only in adapter/test/evidence boundaries.

The accepted target replay rows are `qRoute` / `route-event-list` evidence: each focused adapter test now compares traces emitted by `src/rules/battle_reducer_spine.rs` and `src/rules/battle_features.rs` production observer entrypoints against an independent `expected_route(action)` record before accepting the branch. Literal projection witnesses remain as component state checks, but they are not the basis for the route comparator claim.

### Generated branch coverage:
| Obligation | Evidence | Harness | Status |
| --- | --- | --- | --- |
| `cleanroom-input/qnt/battle-runtime/battle-runtime-adrenaline-rush.mbt.qnt#step:doAdrenalineRushDash` | `tasks/target-replay-evidence/L15-RR07S-A-PASSIVE-ROLL-RESOURCE-SUBSTRATES.json#L15-RR07S-A-PASSIVE-ROLL-RESOURCE-SUBSTRATES replay action=doAdrenalineRushDash#step:doAdrenalineRushDash` | `src/qnt_adapters/battle_runtime_adrenaline_rush.rs` | `covered` |
| `cleanroom-input/qnt/battle-runtime/battle-runtime-adrenaline-rush.mbt.qnt#step:doRejectSecondDash` | `tasks/target-replay-evidence/L15-RR07S-A-PASSIVE-ROLL-RESOURCE-SUBSTRATES.json#L15-RR07S-A-PASSIVE-ROLL-RESOURCE-SUBSTRATES replay action=doRejectSecondDash#step:doRejectSecondDash` | `src/qnt_adapters/battle_runtime_adrenaline_rush.rs` | `covered` |
| `cleanroom-input/qnt/battle-runtime/battle-runtime-halfling-nimbleness-selected-identity.mbt.qnt#step:doMoveThroughLargerCreatureSpace` | `tasks/target-replay-evidence/L15-RR07S-A-PASSIVE-ROLL-RESOURCE-SUBSTRATES.json#L15-RR07S-A-PASSIVE-ROLL-RESOURCE-SUBSTRATES replay action=doMoveThroughLargerCreatureSpace#step:doMoveThroughLargerCreatureSpace` | `src/qnt_adapters/battle_runtime_halfling_nimbleness_selected_identity.rs` | `covered` |
| `cleanroom-input/qnt/battle-runtime/battle-runtime-halfling-nimbleness-selected-identity.mbt.qnt#step:doRejectOccupiedStop` | `tasks/target-replay-evidence/L15-RR07S-A-PASSIVE-ROLL-RESOURCE-SUBSTRATES.json#L15-RR07S-A-PASSIVE-ROLL-RESOURCE-SUBSTRATES replay action=doRejectOccupiedStop#step:doRejectOccupiedStop` | `src/qnt_adapters/battle_runtime_halfling_nimbleness_selected_identity.rs` | `covered` |
| `cleanroom-input/qnt/battle-runtime/battle-runtime-halfling-nimbleness-selected-identity.mbt.qnt#step:doRejectMissingProfile` | `tasks/target-replay-evidence/L15-RR07S-A-PASSIVE-ROLL-RESOURCE-SUBSTRATES.json#L15-RR07S-A-PASSIVE-ROLL-RESOURCE-SUBSTRATES replay action=doRejectMissingProfile#step:doRejectMissingProfile` | `src/qnt_adapters/battle_runtime_halfling_nimbleness_selected_identity.rs` | `covered` |
| `cleanroom-input/qnt/battle-runtime/battle-runtime-halfling-nimbleness-selected-identity.mbt.qnt#step:doRejectSameSizeTraversal` | `tasks/target-replay-evidence/L15-RR07S-A-PASSIVE-ROLL-RESOURCE-SUBSTRATES.json#L15-RR07S-A-PASSIVE-ROLL-RESOURCE-SUBSTRATES replay action=doRejectSameSizeTraversal#step:doRejectSameSizeTraversal` | `src/qnt_adapters/battle_runtime_halfling_nimbleness_selected_identity.rs` | `covered` |
| `cleanroom-input/qnt/battle-runtime/battle-runtime-species-passive-trait-selected-identity.mbt.qnt#step:doDragonbornDamageResistance` | `tasks/target-replay-evidence/L15-RR07S-A-PASSIVE-ROLL-RESOURCE-SUBSTRATES.json#L15-RR07S-A-PASSIVE-ROLL-RESOURCE-SUBSTRATES replay action=doDragonbornDamageResistance#step:doDragonbornDamageResistance` | `src/qnt_adapters/battle_runtime_species_passive_trait_selected_identity.rs` | `covered` |
| `cleanroom-input/qnt/battle-runtime/battle-runtime-species-passive-trait-selected-identity.mbt.qnt#step:doDwarvenResilience` | `tasks/target-replay-evidence/L15-RR07S-A-PASSIVE-ROLL-RESOURCE-SUBSTRATES.json#L15-RR07S-A-PASSIVE-ROLL-RESOURCE-SUBSTRATES replay action=doDwarvenResilience#step:doDwarvenResilience` | `src/qnt_adapters/battle_runtime_species_passive_trait_selected_identity.rs` | `covered` |
| `cleanroom-input/qnt/battle-runtime/battle-runtime-species-passive-trait-selected-identity.mbt.qnt#step:doHalflingBrave` | `tasks/target-replay-evidence/L15-RR07S-A-PASSIVE-ROLL-RESOURCE-SUBSTRATES.json#L15-RR07S-A-PASSIVE-ROLL-RESOURCE-SUBSTRATES replay action=doHalflingBrave#step:doHalflingBrave` | `src/qnt_adapters/battle_runtime_species_passive_trait_selected_identity.rs` | `covered` |
| `cleanroom-input/qnt/battle-runtime/battle-runtime-species-passive-trait-selected-identity.mbt.qnt#step:doGoliathPowerfulBuild` | `tasks/target-replay-evidence/L15-RR07S-A-PASSIVE-ROLL-RESOURCE-SUBSTRATES.json#L15-RR07S-A-PASSIVE-ROLL-RESOURCE-SUBSTRATES replay action=doGoliathPowerfulBuild#step:doGoliathPowerfulBuild` | `src/qnt_adapters/battle_runtime_species_passive_trait_selected_identity.rs` | `covered` |
| `cleanroom-input/qnt/battle-runtime/battle-runtime-danger-sense-selected-identity.mbt.qnt#step:doProjectDangerSenseDexterityAdvantage` | `tasks/target-replay-evidence/L15-RR07S-A-PASSIVE-ROLL-RESOURCE-SUBSTRATES.json#L15-RR07S-A-PASSIVE-ROLL-RESOURCE-SUBSTRATES replay action=doProjectDangerSenseDexterityAdvantage#step:doProjectDangerSenseDexterityAdvantage` | `src/qnt_adapters/battle_runtime_danger_sense_selected_identity.rs` | `covered` |
| `cleanroom-input/qnt/battle-runtime/battle-runtime-danger-sense-selected-identity.mbt.qnt#step:doSuppressDangerSenseWhileIncapacitated` | `tasks/target-replay-evidence/L15-RR07S-A-PASSIVE-ROLL-RESOURCE-SUBSTRATES.json#L15-RR07S-A-PASSIVE-ROLL-RESOURCE-SUBSTRATES replay action=doSuppressDangerSenseWhileIncapacitated#step:doSuppressDangerSenseWhileIncapacitated` | `src/qnt_adapters/battle_runtime_danger_sense_selected_identity.rs` | `covered` |
| `cleanroom-input/qnt/battle-runtime/battle-runtime-roll-modifier-buff-selected-identity.mbt.qnt#step:doBlessAttackAndSaveModifier` | `tasks/target-replay-evidence/L15-RR07S-A-PASSIVE-ROLL-RESOURCE-SUBSTRATES.json#L15-RR07S-A-PASSIVE-ROLL-RESOURCE-SUBSTRATES replay action=doBlessAttackAndSaveModifier#step:doBlessAttackAndSaveModifier` | `src/qnt_adapters/battle_runtime_roll_modifier_buff_selected_identity.rs` | `covered` |
| `cleanroom-input/qnt/battle-runtime/battle-runtime-roll-modifier-buff-selected-identity.mbt.qnt#step:doBaneFailedSavePenalty` | `tasks/target-replay-evidence/L15-RR07S-A-PASSIVE-ROLL-RESOURCE-SUBSTRATES.json#L15-RR07S-A-PASSIVE-ROLL-RESOURCE-SUBSTRATES replay action=doBaneFailedSavePenalty#step:doBaneFailedSavePenalty` | `src/qnt_adapters/battle_runtime_roll_modifier_buff_selected_identity.rs` | `covered` |
| `cleanroom-input/qnt/battle-runtime/battle-runtime-roll-modifier-buff-selected-identity.mbt.qnt#step:doGuidanceSkillAbilityCheckModifier` | `tasks/target-replay-evidence/L15-RR07S-A-PASSIVE-ROLL-RESOURCE-SUBSTRATES.json#L15-RR07S-A-PASSIVE-ROLL-RESOURCE-SUBSTRATES replay action=doGuidanceSkillAbilityCheckModifier#step:doGuidanceSkillAbilityCheckModifier` | `src/qnt_adapters/battle_runtime_roll_modifier_buff_selected_identity.rs` | `covered` |
| `cleanroom-input/qnt/battle-runtime/battle-runtime-roll-modifier-buff-selected-identity.mbt.qnt#step:doResistanceReducesMatchingDamage` | `tasks/target-replay-evidence/L15-RR07S-A-PASSIVE-ROLL-RESOURCE-SUBSTRATES.json#L15-RR07S-A-PASSIVE-ROLL-RESOURCE-SUBSTRATES replay action=doResistanceReducesMatchingDamage#step:doResistanceReducesMatchingDamage` | `src/qnt_adapters/battle_runtime_roll_modifier_buff_selected_identity.rs` | `covered` |
| `cleanroom-input/qnt/battle-runtime/battle-runtime-roll-modifier-buff-selected-identity.mbt.qnt#step:doShieldOfFaithArmorClassBonus` | `tasks/target-replay-evidence/L15-RR07S-A-PASSIVE-ROLL-RESOURCE-SUBSTRATES.json#L15-RR07S-A-PASSIVE-ROLL-RESOURCE-SUBSTRATES replay action=doShieldOfFaithArmorClassBonus#step:doShieldOfFaithArmorClassBonus` | `src/qnt_adapters/battle_runtime_roll_modifier_buff_selected_identity.rs` | `covered` |

### Harness artifacts:
- tasks/START_GATE.json
- tasks/ENGINE_DEPTH_MANIFEST.json
- tasks/STATE_OWNER_MANIFEST.json
- tasks/REVIEW_LOOP.json
- tasks/DECIDER_DECISION.json
- tasks/target-replay-evidence/L15-RR07S-A-PASSIVE-ROLL-RESOURCE-SUBSTRATES.json

### Remaining gaps:
- None for the selected lane obligations.

### Verification results:
- cargo test adapter_replays_all_branches: pass
- cargo fmt --check: pass
- cargo test: pass
- cargo clippy --all-targets -- -D warnings: pass
- node scripts/check-cleanroom-harness.cjs: pass

## L15-RR07-FU08A-METAMAGIC-GOVERNOR-QUICKENED-SUBSTRATE: Quickened Metamagic Governor Substrate

- Manifest source commit SHA: `564376fd95218a209bb9eae5c9ccb54ca3e04a52`
- Source branch inventory SHA: `4bb2b20a85d94e3b90b7c59cbfe6e1edd5ab3ef40410641e999527861f3d3a32`
- Machine-readable run ledger: `tasks/RUN_LEDGER.json`

Allowed inputs used:

- `cleanroom-input/qnt/battle-runtime/battle-runtime-quickened-spell-governor.mbt.qnt`
- `cleanroom-input/qnt/battle-runtime/battle-runtime-sorcerer-metamagic-selected-identity.mbt.qnt`
- `cleanroom-input/qnt/battle-runtime/battle-runtime-sorcerer-metamagic-spell-attack-selected-identity.mbt.qnt`
- `cleanroom-input/qnt/battle-runtime/battle-runtime-sorcerer-metamagic-spell-attack-sequence-selected-identity.mbt.qnt`
- `cleanroom-input/raw/srd-5.2.1/Classes/Sorcerer.md`
- `cleanroom-input/raw/srd-5.2.1/Spells/Gaining-and-Casting.md`
- `cleanroom-input/domain/UBIQUITOUS_LANGUAGE.md`
- Repo-local `src/**`, `tasks/**`, scripts, and Cargo tooling

Behavior implemented:

- Replayed Quickened and generic metamagic selected-identity drivers through `BattleState`, `start_battle`, `discover_battle_acts`, typed `MetamagicOptionSpell` subject/fill variants, `resolve_battle_subject`, and `BattleResolutionResult`.
- Added battle-owned Sorcery Point spend and Quickened substrate projection without production authored-name dispatch.

Generated branch coverage:

| Obligation | Target replay evidence | Diagnostic tests | Status |
| --- | --- | --- | --- |
| `cleanroom-input/qnt/battle-runtime/battle-runtime-quickened-spell-governor.mbt.qnt#step:doResolveQuickenedRestoration` | `tasks/target-replay-evidence/L15-RR07-FU08A-METAMAGIC-GOVERNOR-QUICKENED-SUBSTRATE.json#L15-RR07-FU08A-METAMAGIC-GOVERNOR-QUICKENED-SUBSTRATE seed=1 action=doResolveQuickenedRestoration#step:doResolveQuickenedRestoration` | `src/tests/mod.rs` | `covered` |
| `cleanroom-input/qnt/battle-runtime/battle-runtime-quickened-spell-governor.mbt.qnt#step:doResolveQuickenedSaveGatedCondition` | `tasks/target-replay-evidence/L15-RR07-FU08A-METAMAGIC-GOVERNOR-QUICKENED-SUBSTRATE.json#L15-RR07-FU08A-METAMAGIC-GOVERNOR-QUICKENED-SUBSTRATE seed=1 action=doResolveQuickenedSaveGatedCondition#step:doResolveQuickenedSaveGatedCondition` | `src/tests/mod.rs` | `covered` |
| `cleanroom-input/qnt/battle-runtime/battle-runtime-quickened-spell-governor.mbt.qnt#step:doResolveQuickenedSaveGatedConditionImmunity` | `tasks/target-replay-evidence/L15-RR07-FU08A-METAMAGIC-GOVERNOR-QUICKENED-SUBSTRATE.json#L15-RR07-FU08A-METAMAGIC-GOVERNOR-QUICKENED-SUBSTRATE seed=1 action=doResolveQuickenedSaveGatedConditionImmunity#step:doResolveQuickenedSaveGatedConditionImmunity` | `src/tests/mod.rs` | `covered` |
| `cleanroom-input/qnt/battle-runtime/battle-runtime-quickened-spell-governor.mbt.qnt#step:doResolveQuickenedDirectCondition` | `tasks/target-replay-evidence/L15-RR07-FU08A-METAMAGIC-GOVERNOR-QUICKENED-SUBSTRATE.json#L15-RR07-FU08A-METAMAGIC-GOVERNOR-QUICKENED-SUBSTRATE seed=1 action=doResolveQuickenedDirectCondition#step:doResolveQuickenedDirectCondition` | `src/tests/mod.rs` | `covered` |
| `cleanroom-input/qnt/battle-runtime/battle-runtime-quickened-spell-governor.mbt.qnt#step:doResolveQuickenedRollModifier` | `tasks/target-replay-evidence/L15-RR07-FU08A-METAMAGIC-GOVERNOR-QUICKENED-SUBSTRATE.json#L15-RR07-FU08A-METAMAGIC-GOVERNOR-QUICKENED-SUBSTRATE seed=1 action=doResolveQuickenedRollModifier#step:doResolveQuickenedRollModifier` | `src/tests/mod.rs` | `covered` |
| `cleanroom-input/qnt/battle-runtime/battle-runtime-quickened-spell-governor.mbt.qnt#step:doResolveQuickenedAfterMagicActionSpent` | `tasks/target-replay-evidence/L15-RR07-FU08A-METAMAGIC-GOVERNOR-QUICKENED-SUBSTRATE.json#L15-RR07-FU08A-METAMAGIC-GOVERNOR-QUICKENED-SUBSTRATE seed=1 action=doResolveQuickenedAfterMagicActionSpent#step:doResolveQuickenedAfterMagicActionSpent` | `src/tests/mod.rs` | `covered` |
| `cleanroom-input/qnt/battle-runtime/battle-runtime-quickened-spell-governor.mbt.qnt#step:doRejectUnaffordable` | `tasks/target-replay-evidence/L15-RR07-FU08A-METAMAGIC-GOVERNOR-QUICKENED-SUBSTRATE.json#L15-RR07-FU08A-METAMAGIC-GOVERNOR-QUICKENED-SUBSTRATE seed=1 action=doRejectUnaffordable#step:doRejectUnaffordable` | `src/tests/mod.rs` | `covered` |
| `cleanroom-input/qnt/battle-runtime/battle-runtime-quickened-spell-governor.mbt.qnt#step:doRejectUnknownOption` | `tasks/target-replay-evidence/L15-RR07-FU08A-METAMAGIC-GOVERNOR-QUICKENED-SUBSTRATE.json#L15-RR07-FU08A-METAMAGIC-GOVERNOR-QUICKENED-SUBSTRATE seed=1 action=doRejectUnknownOption#step:doRejectUnknownOption` | `src/tests/mod.rs` | `covered` |
| `cleanroom-input/qnt/battle-runtime/battle-runtime-quickened-spell-governor.mbt.qnt#step:doRejectUnsupportedSecondOption` | `tasks/target-replay-evidence/L15-RR07-FU08A-METAMAGIC-GOVERNOR-QUICKENED-SUBSTRATE.json#L15-RR07-FU08A-METAMAGIC-GOVERNOR-QUICKENED-SUBSTRATE seed=1 action=doRejectUnsupportedSecondOption#step:doRejectUnsupportedSecondOption` | `src/tests/mod.rs` | `covered` |
| `cleanroom-input/qnt/battle-runtime/battle-runtime-quickened-spell-governor.mbt.qnt#step:doRejectOnePerSpell` | `tasks/target-replay-evidence/L15-RR07-FU08A-METAMAGIC-GOVERNOR-QUICKENED-SUBSTRATE.json#L15-RR07-FU08A-METAMAGIC-GOVERNOR-QUICKENED-SUBSTRATE seed=1 action=doRejectOnePerSpell#step:doRejectOnePerSpell` | `src/tests/mod.rs` | `covered` |
| `cleanroom-input/qnt/battle-runtime/battle-runtime-quickened-spell-governor.mbt.qnt#step:doRejectPriorLevelOnePlusSpell` | `tasks/target-replay-evidence/L15-RR07-FU08A-METAMAGIC-GOVERNOR-QUICKENED-SUBSTRATE.json#L15-RR07-FU08A-METAMAGIC-GOVERNOR-QUICKENED-SUBSTRATE seed=1 action=doRejectPriorLevelOnePlusSpell#step:doRejectPriorLevelOnePlusSpell` | `src/tests/mod.rs` | `covered` |
| `cleanroom-input/qnt/battle-runtime/battle-runtime-sorcerer-metamagic-selected-identity.mbt.qnt#step:doResolveQuickenedSaveGatedDamage` | `tasks/target-replay-evidence/L15-RR07-FU08A-METAMAGIC-GOVERNOR-QUICKENED-SUBSTRATE.json#L15-RR07-FU08A-METAMAGIC-GOVERNOR-QUICKENED-SUBSTRATE seed=1 action=doResolveQuickenedSaveGatedDamage#step:doResolveQuickenedSaveGatedDamage` | `src/tests/mod.rs` | `covered` |
| `cleanroom-input/qnt/battle-runtime/battle-runtime-sorcerer-metamagic-spell-attack-selected-identity.mbt.qnt#step:doResolveQuickenedSpellAttack` | `tasks/target-replay-evidence/L15-RR07-FU08A-METAMAGIC-GOVERNOR-QUICKENED-SUBSTRATE.json#L15-RR07-FU08A-METAMAGIC-GOVERNOR-QUICKENED-SUBSTRATE seed=1 action=doResolveQuickenedSpellAttack#step:doResolveQuickenedSpellAttack` | `src/tests/mod.rs` | `covered` |
| `cleanroom-input/qnt/battle-runtime/battle-runtime-sorcerer-metamagic-spell-attack-sequence-selected-identity.mbt.qnt#step:doResolveQuickenedSpellAttackSequence` | `tasks/target-replay-evidence/L15-RR07-FU08A-METAMAGIC-GOVERNOR-QUICKENED-SUBSTRATE.json#L15-RR07-FU08A-METAMAGIC-GOVERNOR-QUICKENED-SUBSTRATE seed=1 action=doResolveQuickenedSpellAttackSequence#step:doResolveQuickenedSpellAttackSequence` | `src/tests/mod.rs` | `covered` |

Target replay evidence:

- Evidence file: `tasks/target-replay-evidence/L15-RR07-FU08A-METAMAGIC-GOVERNOR-QUICKENED-SUBSTRATE.json`
- Target profile: `rust`
- Target profile SHA-256: `6d4cc6c6a4769962798133d57aff01438fb2b661941f71d1aa8a3333f4b7ecc1`
- Reproduction seed or trace id: `L15-RR07-FU08A-METAMAGIC-GOVERNOR-QUICKENED-SUBSTRATE seed=1`

Harness artifacts:

- Start gate: `tasks/history/L15-RR07-FU08A-METAMAGIC-GOVERNOR-QUICKENED-SUBSTRATE/START_GATE.json`
- Engine depth: `tasks/history/L15-RR07-FU08A-METAMAGIC-GOVERNOR-QUICKENED-SUBSTRATE/ENGINE_DEPTH_MANIFEST.json`
- State ownership: `tasks/history/L15-RR07-FU08A-METAMAGIC-GOVERNOR-QUICKENED-SUBSTRATE/STATE_OWNER_MANIFEST.json`
- Reviewer loop: `tasks/history/L15-RR07-FU08A-METAMAGIC-GOVERNOR-QUICKENED-SUBSTRATE/REVIEW_LOOP.json`
- Decider decision: `tasks/history/L15-RR07-FU08A-METAMAGIC-GOVERNOR-QUICKENED-SUBSTRATE/DECIDER_DECISION.json`
- Run ledger: `tasks/RUN_LEDGER.json`

Remaining gaps:

- _none_

Verification results:

- `cargo test quickened -- --nocapture` passed.
- `cargo fmt --check` passed.
- `cargo test` passed.
- `cargo clippy --all-targets -- -D warnings` passed.
- `node scripts/check-cleanroom-harness.cjs` passed.
- `git diff --check 4b24c074161a5bbe2b52ef1125c5f7044e3172e0...HEAD` passed.

## L15-RR07-FU01A-CATALOG-READY-SPELL-SUBSTRATES: Catalog-ready spell substrates

Selected drivers:

- `cleanroom-input/qnt/battle-runtime/battle-runtime-condition-saving-throw-selected-identity.mbt.qnt`
- `cleanroom-input/qnt/battle-runtime/battle-runtime-find-familiar-selected-identity.mbt.qnt`
- `cleanroom-input/qnt/battle-runtime/battle-runtime-thaumaturgy-selected-identity.mbt.qnt`

Behavior implemented:

- Accepted condition-save catalog witnesses route through generic `SaveGatedSpellRouteSubject` and `RepeatSaveConditionEffectRouteSubject` substrate evidence.
- Accepted Find Familiar catalog witnesses route through production-observed `BattleReducerRouteTrace` events emitted by `find_familiar_companion_route_observed`; expected route rows remain literal adapter witnesses for generic companion lifecycle and touch-delivery substrates.
- Accepted Thaumaturgy catalog witness routes through generic `RollModifierEffectRouteSubject` substrate evidence.
- Selected spell identity remains confined to adapter, test, and evidence boundaries.

Evidence disposition:

- Accepted Find Familiar rows retained: `doCastFindFamiliar`, `doDeliverTouchSpellThroughFindFamiliar`, `doDismissAndReappearFindFamiliar`, and `doRecastFindFamiliarReplacement`.
- Demoted Find Familiar rows: `_none_`; production reducer ownership is now represented by `BattleReducerRouteTrace.events`.

Generated branch coverage:

| Obligation | Target replay evidence | Diagnostic tests | Status |
| --- | --- | --- | --- |
| `cleanroom-input/qnt/battle-runtime/battle-runtime-condition-saving-throw-selected-identity.mbt.qnt#step:doResolveColorSprayFailedSavingThrow` | `tasks/target-replay-evidence/L15-RR07-FU01A-catalog-ready-spell-substrates.json#L15-RR07-FU01A-CATALOG-READY-SPELL-SUBSTRATES route action=doResolveColorSprayFailedSavingThrow#step:doResolveColorSprayFailedSavingThrow` | `src/qnt_adapters/battle_runtime_condition_saving_throw_selected_identity.rs` | `covered` |
| `cleanroom-input/qnt/battle-runtime/battle-runtime-condition-saving-throw-selected-identity.mbt.qnt#step:doResolveEntangleFailedSavingThrow` | `tasks/target-replay-evidence/L15-RR07-FU01A-catalog-ready-spell-substrates.json#L15-RR07-FU01A-CATALOG-READY-SPELL-SUBSTRATES route action=doResolveEntangleFailedSavingThrow#step:doResolveEntangleFailedSavingThrow` | `src/qnt_adapters/battle_runtime_condition_saving_throw_selected_identity.rs` | `covered` |
| `cleanroom-input/qnt/battle-runtime/battle-runtime-condition-saving-throw-selected-identity.mbt.qnt#step:doResolveHideousLaughterRepeatSavingThrowSuccess` | `tasks/target-replay-evidence/L15-RR07-FU01A-catalog-ready-spell-substrates.json#L15-RR07-FU01A-CATALOG-READY-SPELL-SUBSTRATES route action=doResolveHideousLaughterRepeatSavingThrowSuccess#step:doResolveHideousLaughterRepeatSavingThrowSuccess` | `src/qnt_adapters/battle_runtime_condition_saving_throw_selected_identity.rs` | `covered` |
| `cleanroom-input/qnt/battle-runtime/battle-runtime-condition-saving-throw-selected-identity.mbt.qnt#step:doResolveSleepRepeatSavingThrowFailure` | `tasks/target-replay-evidence/L15-RR07-FU01A-catalog-ready-spell-substrates.json#L15-RR07-FU01A-CATALOG-READY-SPELL-SUBSTRATES route action=doResolveSleepRepeatSavingThrowFailure#step:doResolveSleepRepeatSavingThrowFailure` | `src/qnt_adapters/battle_runtime_condition_saving_throw_selected_identity.rs` | `covered` |
| `cleanroom-input/qnt/battle-runtime/battle-runtime-find-familiar-selected-identity.mbt.qnt#step:doCastFindFamiliar` | `tasks/target-replay-evidence/L15-RR07-FU01A-catalog-ready-spell-substrates.json#L15-RR07-FU01A-CATALOG-READY-SPELL-SUBSTRATES route action=doCastFindFamiliar#step:doCastFindFamiliar` | `src/qnt_adapters/battle_runtime_find_familiar_selected_identity.rs` | `covered` |
| `cleanroom-input/qnt/battle-runtime/battle-runtime-find-familiar-selected-identity.mbt.qnt#step:doDeliverTouchSpellThroughFindFamiliar` | `tasks/target-replay-evidence/L15-RR07-FU01A-catalog-ready-spell-substrates.json#L15-RR07-FU01A-CATALOG-READY-SPELL-SUBSTRATES route action=doDeliverTouchSpellThroughFindFamiliar#step:doDeliverTouchSpellThroughFindFamiliar` | `src/qnt_adapters/battle_runtime_find_familiar_selected_identity.rs` | `covered` |
| `cleanroom-input/qnt/battle-runtime/battle-runtime-find-familiar-selected-identity.mbt.qnt#step:doDismissAndReappearFindFamiliar` | `tasks/target-replay-evidence/L15-RR07-FU01A-catalog-ready-spell-substrates.json#L15-RR07-FU01A-CATALOG-READY-SPELL-SUBSTRATES route action=doDismissAndReappearFindFamiliar#step:doDismissAndReappearFindFamiliar` | `src/qnt_adapters/battle_runtime_find_familiar_selected_identity.rs` | `covered` |
| `cleanroom-input/qnt/battle-runtime/battle-runtime-find-familiar-selected-identity.mbt.qnt#step:doRecastFindFamiliarReplacement` | `tasks/target-replay-evidence/L15-RR07-FU01A-catalog-ready-spell-substrates.json#L15-RR07-FU01A-CATALOG-READY-SPELL-SUBSTRATES route action=doRecastFindFamiliarReplacement#step:doRecastFindFamiliarReplacement` | `src/qnt_adapters/battle_runtime_find_familiar_selected_identity.rs` | `covered` |
| `cleanroom-input/qnt/battle-runtime/battle-runtime-thaumaturgy-selected-identity.mbt.qnt#step:doResolveThaumaturgyBoomingVoice` | `tasks/target-replay-evidence/L15-RR07-FU01A-catalog-ready-spell-substrates.json#L15-RR07-FU01A-CATALOG-READY-SPELL-SUBSTRATES route action=doResolveThaumaturgyBoomingVoice#step:doResolveThaumaturgyBoomingVoice` | `src/qnt_adapters/battle_runtime_thaumaturgy_selected_identity.rs` | `covered` |

Out-of-scope branches recorded for this lane:

- `doResolveBlindnessDeafnessBlindedSavingThrow` (level-2 condition-save branch)
- `doResolveBlindnessDeafnessDeafenedSavingThrow` (level-2 condition-save branch)
- `doResolveHoldPersonFailedSavingThrow` (level-2 condition-save branch)
- `doResolveHoldPersonRepeatSavingThrowSuccess` (level-2 repeat-save branch)
- `doResolveHypnoticPatternFailedSavingThrow` (level-3 condition-save branch)

Harness artifacts:

- Start gate: `tasks/START_GATE.json`
- Engine depth: `tasks/ENGINE_DEPTH_MANIFEST.json`
- State ownership: `tasks/STATE_OWNER_MANIFEST.json`
- Reviewer loop: `tasks/REVIEW_LOOP.json`
- Decider decision: `tasks/DECIDER_DECISION.json`
- Run ledger: `tasks/RUN_LEDGER.json`

Verification results:

- `cargo test condition_saving_throw_selected_identity_routes_in_scope_substrate_branches` passed.
- `cargo test find_familiar_selected_identity_adapter_replays_all_branches` passed.
- `cargo test selected_identity_adapter_replays_all_branches` passed.
- `cargo fmt --check` passed.
- `cargo test` passed.
- `cargo clippy --all-targets -- -D warnings` passed.
- `node scripts/check-cleanroom-harness.cjs` passed after artifact convergence.
- `git diff --check 4b24c074161a5bbe2b52ef1125c5f7044e3172e0...HEAD` passed.
## L15-RR07-FU08B-METAMAGIC-SAVE-RANGE-TARGET-SUBSTRATES: Metamagic Save, Range, and Target Substrates

- Manifest source commit SHA: `564376fd95218a209bb9eae5c9ccb54ca3e04a52`
- Source branch inventory SHA: `4bb2b20a85d94e3b90b7c59cbfe6e1edd5ab3ef40410641e999527861f3d3a32`
- Drivers:
  - `cleanroom-input/qnt/battle-runtime/battle-runtime-sorcerer-metamagic-careful-selected-identity.mbt.qnt`
  - `cleanroom-input/qnt/battle-runtime/battle-runtime-sorcerer-metamagic-heightened-selected-identity.mbt.qnt`
  - `cleanroom-input/qnt/battle-runtime/battle-runtime-sorcerer-metamagic-distant-selected-identity.mbt.qnt`
  - `cleanroom-input/qnt/battle-runtime/battle-runtime-sorcerer-metamagic-twinned-selected-identity.mbt.qnt`
- Machine-readable run ledger: `tasks/RUN_LEDGER.json`

Allowed inputs used:

- `cleanroom-input/MANIFEST.md`
- `cleanroom-input/branch-coverage/source-branch-inventory.json`
- `cleanroom-input/qnt/battle-runtime/battle-runtime-sorcerer-metamagic-careful-selected-identity.mbt.qnt`
- `cleanroom-input/qnt/battle-runtime/battle-runtime-sorcerer-metamagic-heightened-selected-identity.mbt.qnt`
- `cleanroom-input/qnt/battle-runtime/battle-runtime-sorcerer-metamagic-distant-selected-identity.mbt.qnt`
- `cleanroom-input/qnt/battle-runtime/battle-runtime-sorcerer-metamagic-twinned-selected-identity.mbt.qnt`
- `cleanroom-input/raw/srd-5.2.1/Classes/Sorcerer.md`
- `cleanroom-input/raw/srd-5.2.1/Spells/Descriptions-E-L.md`
- `cleanroom-input/domain/UBIQUITOUS_LANGUAGE.md`
- Repo-local `src/**`, `tasks/**`, and Rust/Cargo tooling

Behavior implemented:

- Replayed Careful, Heightened, Distant, and Twinned selected-identity branches through production `BattleState` reducer entrypoints: `start_battle_observed`, `discover_battle_acts_observed`, and `resolve_battle_subject_observed`.
- Reused the FU08A Sorcery Point resource and one-option metamagic governor facts; branch adapters no longer synthesize observed state directly.
- Routed observed evidence through typed save, damage-adjustment, condition, object-boundary, target-selection, action-economy, and feature-resource owner facts without production dispatch on authored spell or option identity.

Generated branch coverage:

| Obligation | Target replay evidence | Diagnostic tests | Status |
| --- | --- | --- | --- |
| `cleanroom-input/qnt/battle-runtime/battle-runtime-sorcerer-metamagic-careful-selected-identity.mbt.qnt#step:doResolveCarefulSaveGatedDamage` | `tasks/target-replay-evidence/L15-RR07-FU08B-METAMAGIC-SAVE-RANGE-TARGET-SUBSTRATES.json#L15-RR07-FU08B-METAMAGIC-SAVE-RANGE-TARGET-SUBSTRATES seed=1 action=doResolveCarefulSaveGatedDamage#step:doResolveCarefulSaveGatedDamage` | `src/qnt_adapters/battle_runtime_sorcerer_metamagic_careful_selected_identity.rs` | `covered` |
| `cleanroom-input/qnt/battle-runtime/battle-runtime-sorcerer-metamagic-careful-selected-identity.mbt.qnt#step:doResolveCarefulSaveGatedNoEffect` | `tasks/target-replay-evidence/L15-RR07-FU08B-METAMAGIC-SAVE-RANGE-TARGET-SUBSTRATES.json#L15-RR07-FU08B-METAMAGIC-SAVE-RANGE-TARGET-SUBSTRATES seed=1 action=doResolveCarefulSaveGatedNoEffect#step:doResolveCarefulSaveGatedNoEffect` | `src/qnt_adapters/battle_runtime_sorcerer_metamagic_careful_selected_identity.rs` | `covered` |
| `cleanroom-input/qnt/battle-runtime/battle-runtime-sorcerer-metamagic-heightened-selected-identity.mbt.qnt#step:doResolveHeightenedSaveGatedDamage` | `tasks/target-replay-evidence/L15-RR07-FU08B-METAMAGIC-SAVE-RANGE-TARGET-SUBSTRATES.json#L15-RR07-FU08B-METAMAGIC-SAVE-RANGE-TARGET-SUBSTRATES seed=1 action=doResolveHeightenedSaveGatedDamage#step:doResolveHeightenedSaveGatedDamage` | `src/qnt_adapters/battle_runtime_sorcerer_metamagic_heightened_selected_identity.rs` | `covered` |
| `cleanroom-input/qnt/battle-runtime/battle-runtime-sorcerer-metamagic-heightened-selected-identity.mbt.qnt#step:doResolveHeightenedHideousLaughter` | `tasks/target-replay-evidence/L15-RR07-FU08B-METAMAGIC-SAVE-RANGE-TARGET-SUBSTRATES.json#L15-RR07-FU08B-METAMAGIC-SAVE-RANGE-TARGET-SUBSTRATES seed=1 action=doResolveHeightenedHideousLaughter#step:doResolveHeightenedHideousLaughter` | `src/qnt_adapters/battle_runtime_sorcerer_metamagic_heightened_selected_identity.rs` | `covered` |
| `cleanroom-input/qnt/battle-runtime/battle-runtime-sorcerer-metamagic-heightened-selected-identity.mbt.qnt#step:doResolveHeightenedGreaseEntrySave` | `tasks/target-replay-evidence/L15-RR07-FU08B-METAMAGIC-SAVE-RANGE-TARGET-SUBSTRATES.json#L15-RR07-FU08B-METAMAGIC-SAVE-RANGE-TARGET-SUBSTRATES seed=1 action=doResolveHeightenedGreaseEntrySave#step:doResolveHeightenedGreaseEntrySave` | `src/qnt_adapters/battle_runtime_sorcerer_metamagic_heightened_selected_identity.rs` | `covered` |
| `cleanroom-input/qnt/battle-runtime/battle-runtime-sorcerer-metamagic-heightened-selected-identity.mbt.qnt#step:doResolveHeightenedGustOfWindEndTurnSave` | `tasks/target-replay-evidence/L15-RR07-FU08B-METAMAGIC-SAVE-RANGE-TARGET-SUBSTRATES.json#L15-RR07-FU08B-METAMAGIC-SAVE-RANGE-TARGET-SUBSTRATES seed=1 action=doResolveHeightenedGustOfWindEndTurnSave#step:doResolveHeightenedGustOfWindEndTurnSave` | `src/qnt_adapters/battle_runtime_sorcerer_metamagic_heightened_selected_identity.rs` | `covered` |
| `cleanroom-input/qnt/battle-runtime/battle-runtime-sorcerer-metamagic-heightened-selected-identity.mbt.qnt#step:doResolveHeightenedSaveGatedConditionEndTurnSave` | `tasks/target-replay-evidence/L15-RR07-FU08B-METAMAGIC-SAVE-RANGE-TARGET-SUBSTRATES.json#L15-RR07-FU08B-METAMAGIC-SAVE-RANGE-TARGET-SUBSTRATES seed=1 action=doResolveHeightenedSaveGatedConditionEndTurnSave#step:doResolveHeightenedSaveGatedConditionEndTurnSave` | `src/qnt_adapters/battle_runtime_sorcerer_metamagic_heightened_selected_identity.rs` | `covered` |
| `cleanroom-input/qnt/battle-runtime/battle-runtime-sorcerer-metamagic-distant-selected-identity.mbt.qnt#step:doResolveDistantObjectLight` | `tasks/target-replay-evidence/L15-RR07-FU08B-METAMAGIC-SAVE-RANGE-TARGET-SUBSTRATES.json#L15-RR07-FU08B-METAMAGIC-SAVE-RANGE-TARGET-SUBSTRATES seed=1 action=doResolveDistantObjectLight#step:doResolveDistantObjectLight` | `src/qnt_adapters/battle_runtime_sorcerer_metamagic_distant_selected_identity.rs` | `covered` |
| `cleanroom-input/qnt/battle-runtime/battle-runtime-sorcerer-metamagic-twinned-selected-identity.mbt.qnt#step:doResolveTwinnedTargetCount` | `tasks/target-replay-evidence/L15-RR07-FU08B-METAMAGIC-SAVE-RANGE-TARGET-SUBSTRATES.json#L15-RR07-FU08B-METAMAGIC-SAVE-RANGE-TARGET-SUBSTRATES seed=1 action=doResolveTwinnedTargetCount#step:doResolveTwinnedTargetCount` | `src/qnt_adapters/battle_runtime_sorcerer_metamagic_twinned_selected_identity.rs` | `covered` |

Accepted/demoted/blocker rows:

| Row type | Count | Notes |
| --- | ---: | --- |
| Accepted | 9 | All in-scope FU08B branch actions route through production reducer entrypoints. |
| Demoted | 0 | None. |
| Blocker | 0 | None. |

Target replay evidence:

- Evidence file: `tasks/target-replay-evidence/L15-RR07-FU08B-METAMAGIC-SAVE-RANGE-TARGET-SUBSTRATES.json`
- Target profile: `rust`
- Target profile SHA-256: `6d4cc6c6a4769962798133d57aff01438fb2b661941f71d1aa8a3333f4b7ecc1`
- Reproduction seed or trace id: `L15-RR07-FU08B-METAMAGIC-SAVE-RANGE-TARGET-SUBSTRATES seed=1`

Harness artifacts:

- Start gate: `tasks/history/L15-RR07-FU08B-METAMAGIC-SAVE-RANGE-TARGET-SUBSTRATES/START_GATE.json`
- Engine depth: `tasks/history/L15-RR07-FU08B-METAMAGIC-SAVE-RANGE-TARGET-SUBSTRATES/ENGINE_DEPTH_MANIFEST.json`
- State ownership: `tasks/history/L15-RR07-FU08B-METAMAGIC-SAVE-RANGE-TARGET-SUBSTRATES/STATE_OWNER_MANIFEST.json`
- Reviewer loop: `tasks/history/L15-RR07-FU08B-METAMAGIC-SAVE-RANGE-TARGET-SUBSTRATES/REVIEW_LOOP.json`
- Decider decision: `tasks/history/L15-RR07-FU08B-METAMAGIC-SAVE-RANGE-TARGET-SUBSTRATES/DECIDER_DECISION.json`
- Run ledger: `tasks/RUN_LEDGER.json`

Remaining gaps:

- `_none_`

Verification results:

- `cargo fmt --check` passed.
- `cargo test` passed.
- `cargo clippy --all-targets -- -D warnings` passed.
- `node scripts/check-cleanroom-harness.cjs` passed.
- `git diff --check 6bad2c3509996b8cafab2a1d0258e3062e8ff60b...HEAD` passed.

## L15-RR07-FU01D-PROTECTION-CHARM-WARD-SUBSTRATES: Protection, charm, and ward substrates

Selected drivers:

- `cleanroom-input/qnt/battle-runtime/battle-runtime-creature-type-protection-and-charm-selected-identity.mbt.qnt`
- `cleanroom-input/qnt/battle-runtime/battle-runtime-sanctuary-selected-identity.mbt.qnt`

Behavior implemented:

- Rust reducer and adapter replay remain as diagnostic substrate through `BattleState.protection_charm_ward` subjects.
- No FU01D row is counted as accepted coverage because copied cleanroom inputs do not include an executable generic protection/charm/ward QNT connector substrate.
- The copied reducer-route inventory already classifies both selected drivers as `source-qnt-corpus-blocker`; selected spell identity remains quarantined to adapters/tests/artifacts.

Generated branch coverage:

| Source-QNT blocker row | Reason | Status |
| --- | --- | --- |
| `doDiscoverAnimalFriendshipBeastTargetAdmission` | copied source-QNT route inventory still treats generic protection/charm/ward route rows as source-QNT corpus blockers; the cleanroom inputs provide route vocabulary and Rust diagnostic replay, but no executable copied-QNT generic connector substrate | `source-qnt-corpus-blocker` |
| `doResolveAnimalFriendshipFailedSaveCharmed` | copied source-QNT route inventory still treats generic protection/charm/ward route rows as source-QNT corpus blockers; the cleanroom inputs provide route vocabulary and Rust diagnostic replay, but no executable copied-QNT generic connector substrate | `source-qnt-corpus-blocker` |
| `doResolveAnimalFriendshipCasterDamageBreak` | copied source-QNT route inventory still treats generic protection/charm/ward route rows as source-QNT corpus blockers; the cleanroom inputs provide route vocabulary and Rust diagnostic replay, but no executable copied-QNT generic connector substrate | `source-qnt-corpus-blocker` |
| `doResolveProtectionFromEvilAndGoodKnownWillingTargetProtection` | copied source-QNT route inventory still treats generic protection/charm/ward route rows as source-QNT corpus blockers; the cleanroom inputs provide route vocabulary and Rust diagnostic replay, but no executable copied-QNT generic connector substrate | `source-qnt-corpus-blocker` |
| `doProjectProtectionFromEvilAndGoodScopedAttackDisadvantage` | copied source-QNT route inventory still treats generic protection/charm/ward route rows as source-QNT corpus blockers; the cleanroom inputs provide route vocabulary and Rust diagnostic replay, but no executable copied-QNT generic connector substrate | `source-qnt-corpus-blocker` |
| `doPreventProtectionFromEvilAndGoodScopedCharmAndPossession` | copied source-QNT route inventory still treats generic protection/charm/ward route rows as source-QNT corpus blockers; the cleanroom inputs provide route vocabulary and Rust diagnostic replay, but no executable copied-QNT generic connector substrate | `source-qnt-corpus-blocker` |
| `doResolveProtectionFromEvilAndGoodRelevantCharmSaveAdvantage` | copied source-QNT route inventory still treats generic protection/charm/ward route rows as source-QNT corpus blockers; the cleanroom inputs provide route vocabulary and Rust diagnostic replay, but no executable copied-QNT generic connector substrate | `source-qnt-corpus-blocker` |
| `doCastSanctuaryWardCreation` | copied source-QNT route inventory still treats generic protection/charm/ward route rows as source-QNT corpus blockers; the cleanroom inputs provide route vocabulary and Rust diagnostic replay, but no executable copied-QNT generic connector substrate | `source-qnt-corpus-blocker` |
| `doInterdictDirectAttackFailedSaveLoss` | copied source-QNT route inventory still treats generic protection/charm/ward route rows as source-QNT corpus blockers; the cleanroom inputs provide route vocabulary and Rust diagnostic replay, but no executable copied-QNT generic connector substrate | `source-qnt-corpus-blocker` |
| `doInterdictDirectSpellSuccessfulSavePassThrough` | copied source-QNT route inventory still treats generic protection/charm/ward route rows as source-QNT corpus blockers; the cleanroom inputs provide route vocabulary and Rust diagnostic replay, but no executable copied-QNT generic connector substrate | `source-qnt-corpus-blocker` |
| `doRetargetDirectAttackToLegalReplacement` | copied source-QNT route inventory still treats generic protection/charm/ward route rows as source-QNT corpus blockers; the cleanroom inputs provide route vocabulary and Rust diagnostic replay, but no executable copied-QNT generic connector substrate | `source-qnt-corpus-blocker` |
| `doRejectIllegalReplacementTarget` | copied source-QNT route inventory still treats generic protection/charm/ward route rows as source-QNT corpus blockers; the cleanroom inputs provide route vocabulary and Rust diagnostic replay, but no executable copied-QNT generic connector substrate | `source-qnt-corpus-blocker` |
| `doExcludeAreaEffectFromInterdiction` | copied source-QNT route inventory still treats generic protection/charm/ward route rows as source-QNT corpus blockers; the cleanroom inputs provide route vocabulary and Rust diagnostic replay, but no executable copied-QNT generic connector substrate | `source-qnt-corpus-blocker` |
| `doEndWardOnWardedAttackRoll` | copied source-QNT route inventory still treats generic protection/charm/ward route rows as source-QNT corpus blockers; the cleanroom inputs provide route vocabulary and Rust diagnostic replay, but no executable copied-QNT generic connector substrate | `source-qnt-corpus-blocker` |
| `doEndWardOnWardedSpellCast` | copied source-QNT route inventory still treats generic protection/charm/ward route rows as source-QNT corpus blockers; the cleanroom inputs provide route vocabulary and Rust diagnostic replay, but no executable copied-QNT generic connector substrate | `source-qnt-corpus-blocker` |
| `doEndWardOnWardedDamageDealt` | copied source-QNT route inventory still treats generic protection/charm/ward route rows as source-QNT corpus blockers; the cleanroom inputs provide route vocabulary and Rust diagnostic replay, but no executable copied-QNT generic connector substrate | `source-qnt-corpus-blocker` |

Target replay evidence:

- _none_; the previous FU01D target replay evidence file was removed because it represented diagnostic Rust replay, not accepted copied-QNT connector evidence.

Remaining gaps:

- Add a copied executable generic protection/charm/ward QNT connector substrate before these rows may count for target replay coverage.

Verification results:

- `cargo test creature_type_protection_adapter_replays_all_branches` passed.
- `cargo test sanctuary_selected_identity_adapter_replays_all_branches` passed.
- `cargo fmt --check` passed.
- `node scripts/check-cleanroom-harness.cjs` passed.
- `git diff --check 410a784738fba3b80566eae292140327d4e30877...HEAD` passed.
- `cargo test` passed.
- `cargo clippy --all-targets -- -D warnings` passed.

Harness artifacts:

- History: `tasks/history/L15-RR07-FU01D-PROTECTION-CHARM-WARD-SUBSTRATES/`
- Run ledger: `tasks/RUN_LEDGER.json` records accepted coverage only; FU01D is intentionally absent after demotion.

## L15-RR07-FU08C-METAMAGIC-REROLL-DAMAGE-PROJECTION-SUBSTRATES: Metamagic Reroll, Damage, Projection Substrates

- Manifest source commit SHA: `564376fd95218a209bb9eae5c9ccb54ca3e04a52`
- Source branch inventory SHA: `4bb2b20a85d94e3b90b7c59cbfe6e1edd5ab3ef40410641e999527861f3d3a32`
- Drivers: `cleanroom-input/qnt/battle-runtime/battle-runtime-sorcerer-metamagic-empowered-selected-identity.mbt.qnt`, `cleanroom-input/qnt/battle-runtime/battle-runtime-sorcerer-metamagic-extended-selected-identity.mbt.qnt`, `cleanroom-input/qnt/battle-runtime/battle-runtime-sorcerer-metamagic-seeking-selected-identity.mbt.qnt`, `cleanroom-input/qnt/battle-runtime/battle-runtime-sorcerer-metamagic-subtle-selected-identity.mbt.qnt`, `cleanroom-input/qnt/battle-runtime/battle-runtime-sorcerer-metamagic-transmuted-selected-identity.mbt.qnt`
- Machine-readable run ledger: `tasks/RUN_LEDGER.json`

Behavior implemented:

- Routed Empowered, Seeking, Subtle, Transmuted, and Extended selected-identity witnesses through production `BattleState` start/discover/resolve entrypoints.
- Reused `BattleState.feature_resources.sorcery_points` for metamagic resource spend and route-gated option application by typed spell modification facts rather than authored identities.
- Kept expected route witnesses literal and independent from observed reducer traces.

Generated branch coverage:

| Obligation | Target replay evidence | Diagnostic tests | Status |
| --- | --- | --- | --- |
| `cleanroom-input/qnt/battle-runtime/battle-runtime-sorcerer-metamagic-empowered-selected-identity.mbt.qnt#step:doResolveEmpoweredSpellDamageReroll` | `tasks/target-replay-evidence/L15-RR07-FU08C-metamagic-reroll-damage-projection-substrates.json#L15-RR07-FU08C-METAMAGIC-REROLL-DAMAGE-PROJECTION-SUBSTRATES seed=1 action=doResolveEmpoweredSpellDamageReroll#step:doResolveEmpoweredSpellDamageReroll` | `src/tests/mod.rs` | `covered` |
| `cleanroom-input/qnt/battle-runtime/battle-runtime-sorcerer-metamagic-extended-selected-identity.mbt.qnt#step:doResolveExtendedCreatureSizeIncrease` | `tasks/target-replay-evidence/L15-RR07-FU08C-metamagic-reroll-damage-projection-substrates.json#L15-RR07-FU08C-METAMAGIC-REROLL-DAMAGE-PROJECTION-SUBSTRATES seed=1 action=doResolveExtendedCreatureSizeIncrease#step:doResolveExtendedCreatureSizeIncrease` | `src/tests/mod.rs` | `covered` |
| `cleanroom-input/qnt/battle-runtime/battle-runtime-sorcerer-metamagic-seeking-selected-identity.mbt.qnt#step:doResolveSeekingSpellAttackReroll` | `tasks/target-replay-evidence/L15-RR07-FU08C-metamagic-reroll-damage-projection-substrates.json#L15-RR07-FU08C-METAMAGIC-REROLL-DAMAGE-PROJECTION-SUBSTRATES seed=1 action=doResolveSeekingSpellAttackReroll#step:doResolveSeekingSpellAttackReroll` | `src/tests/mod.rs` | `covered` |
| `cleanroom-input/qnt/battle-runtime/battle-runtime-sorcerer-metamagic-subtle-selected-identity.mbt.qnt#step:doRejectSubtleFalseLifeWithoutSorceryPoints` | `tasks/target-replay-evidence/L15-RR07-FU08C-metamagic-reroll-damage-projection-substrates.json#L15-RR07-FU08C-METAMAGIC-REROLL-DAMAGE-PROJECTION-SUBSTRATES seed=1 action=doRejectSubtleFalseLifeWithoutSorceryPoints#step:doRejectSubtleFalseLifeWithoutSorceryPoints` | `src/tests/mod.rs` | `covered` |
| `cleanroom-input/qnt/battle-runtime/battle-runtime-sorcerer-metamagic-subtle-selected-identity.mbt.qnt#step:doResolveSubtleFalseLife` | `tasks/target-replay-evidence/L15-RR07-FU08C-metamagic-reroll-damage-projection-substrates.json#L15-RR07-FU08C-METAMAGIC-REROLL-DAMAGE-PROJECTION-SUBSTRATES seed=1 action=doResolveSubtleFalseLife#step:doResolveSubtleFalseLife` | `src/tests/mod.rs` | `covered` |
| `cleanroom-input/qnt/battle-runtime/battle-runtime-sorcerer-metamagic-transmuted-selected-identity.mbt.qnt#step:doResolveTransmutedSaveGatedDamage` | `tasks/target-replay-evidence/L15-RR07-FU08C-metamagic-reroll-damage-projection-substrates.json#L15-RR07-FU08C-METAMAGIC-REROLL-DAMAGE-PROJECTION-SUBSTRATES seed=1 action=doResolveTransmutedSaveGatedDamage#step:doResolveTransmutedSaveGatedDamage` | `src/tests/mod.rs` | `covered` |
| `cleanroom-input/qnt/battle-runtime/battle-runtime-sorcerer-metamagic-transmuted-selected-identity.mbt.qnt#step:doResolveTransmutedSpellAttack` | `tasks/target-replay-evidence/L15-RR07-FU08C-metamagic-reroll-damage-projection-substrates.json#L15-RR07-FU08C-METAMAGIC-REROLL-DAMAGE-PROJECTION-SUBSTRATES seed=1 action=doResolveTransmutedSpellAttack#step:doResolveTransmutedSpellAttack` | `src/tests/mod.rs` | `covered` |

Target replay evidence:

- Evidence file: `tasks/target-replay-evidence/L15-RR07-FU08C-metamagic-reroll-damage-projection-substrates.json`
- Target profile: `rust`
- Target profile SHA-256: `6d4cc6c6a4769962798133d57aff01438fb2b661941f71d1aa8a3333f4b7ecc1`
- Reproduction seed or trace id: `L15-RR07-FU08C-METAMAGIC-REROLL-DAMAGE-PROJECTION-SUBSTRATES seed=1`

Harness artifacts:

- Start gate: `tasks/history/L15-RR07-FU08C-METAMAGIC-REROLL-DAMAGE-PROJECTION-SUBSTRATES/START_GATE.json`
- Engine depth: `tasks/history/L15-RR07-FU08C-METAMAGIC-REROLL-DAMAGE-PROJECTION-SUBSTRATES/ENGINE_DEPTH_MANIFEST.json`
- State ownership: `tasks/history/L15-RR07-FU08C-METAMAGIC-REROLL-DAMAGE-PROJECTION-SUBSTRATES/STATE_OWNER_MANIFEST.json`
- Reviewer loop: `tasks/history/L15-RR07-FU08C-METAMAGIC-REROLL-DAMAGE-PROJECTION-SUBSTRATES/REVIEW_LOOP.json`
- Decider decision: `tasks/history/L15-RR07-FU08C-METAMAGIC-REROLL-DAMAGE-PROJECTION-SUBSTRATES/DECIDER_DECISION.json`
- Run ledger: `tasks/RUN_LEDGER.json`

Remaining gaps:

- `_none_`

Verification results:

- Focused adapter tests passed.
- `cargo fmt --check` passed.
- `cargo test` passed.
- `cargo clippy --all-targets -- -D warnings` passed.
- `node scripts/check-cleanroom-harness.cjs` passed.
- `git diff --check 6bad2c3509996b8cafab2a1d0258e3062e8ff60b...HEAD` passed.

## L15-RR07-FU01B-SPELL-ATTACK-SAVE-DAMAGE-SUBSTRATES: Spell attack/save damage substrates

Selected drivers:

- `cleanroom-input/qnt/battle-runtime/battle-runtime-attack-spell-shape-selected-identity.mbt.qnt`
- `cleanroom-input/qnt/battle-runtime/battle-runtime-level1-damage-spell-selected-identity.mbt.qnt`

Behavior implemented:

- Accepted attack spell shape rows replay through production `BattleState` spell attack or save-gated reducer entrypoints and observed `ReducerRouteEvent.qRoute` evidence.
- Accepted level-1 damage spell rows replay through production `BattleState` spell attack or save-gated reducer entrypoints and observed `ReducerRouteEvent.qRoute` evidence.
- `doResolveChromaticOrbDuplicateDamageLeap` and `doResolveStarryWispObjectSpellAttackDamageAndDimLight` are not accepted as resolved witnesses. They are target blockers because the implemented Rust facts live in `ChromaticOrbSequenceState` and `StarryWispObjectState`, not production `BattleState` route subjects.

Accepted rows:

- Attack spell shape: `doFireBoltHit`, `doChillTouchHitPointRegainPrevention`, `doGuidingBoltNextAttackAdvantage`, `doInflictWoundsFailedSave`, `doInflictWoundsSuccessfulSave`, `doShockingGraspOpportunityAttackDenied`
- Level 1 damage: `doResolveBurningHandsMixedConeSavingThrows`, `doResolveIceKnifeHitAttackDamageAndBurstSavingThrows`, `doResolveIceKnifeMissBurstSavingThrows`, `doResolvePoisonSpraySpellAttackDamage`, `doResolveRayOfSicknessSpellAttackDamageAndPoisoned`, `doResolveSacredFlameDexteritySavingThrowRadiantDamage`, `doResolveSorcerousBurstSpellAttackDamage`, `doResolveViciousMockeryWisdomSavingThrowPsychicDamageAndNextAttackDisadvantage`

Target blockers:

- `doResolveChromaticOrbDuplicateDamageLeap`: `target-blocker`, no production `BattleState` chained spell-attack route.
- `doResolveStarryWispObjectSpellAttackDamageAndDimLight`: `target-blocker`, no production `BattleState` object-target damage/light route.

Allowed inputs used:

- `cleanroom-input/MANIFEST.md` source commit `564376fd95218a209bb9eae5c9ccb54ca3e04a52`.
- Source branch inventory SHA: `4bb2b20a85d94e3b90b7c59cbfe6e1edd5ab3ef40410641e999527861f3d3a32`.
- SRD 5.2.1 spell attack, saving throw, damage, and hit point passages under `cleanroom-input/raw/srd-5.2.1/`.
- Ubiquitous language: `cleanroom-input/domain/UBIQUITOUS_LANGUAGE.md`.

Generated branch coverage:

| Obligation | Target replay evidence | Diagnostic tests | Status |
| --- | --- | --- | --- |
| `cleanroom-input/qnt/battle-runtime/battle-runtime-attack-spell-shape-selected-identity.mbt.qnt#step:doChillTouchHitPointRegainPrevention` | `tasks/target-replay-evidence/L15-RR07-FU01B-SPELL-ATTACK-SAVE-DAMAGE-SUBSTRATES.json#L15-RR07-FU01B-SPELL-ATTACK-SAVE-DAMAGE-SUBSTRATES route action=doChillTouchHitPointRegainPrevention#step:doChillTouchHitPointRegainPrevention` | `src/qnt_adapters/battle_runtime_attack_spell_shape_selected_identity.rs` | `covered` |
| `cleanroom-input/qnt/battle-runtime/battle-runtime-attack-spell-shape-selected-identity.mbt.qnt#step:doFireBoltHit` | `tasks/target-replay-evidence/L15-RR07-FU01B-SPELL-ATTACK-SAVE-DAMAGE-SUBSTRATES.json#L15-RR07-FU01B-SPELL-ATTACK-SAVE-DAMAGE-SUBSTRATES route action=doFireBoltHit#step:doFireBoltHit` | `src/qnt_adapters/battle_runtime_attack_spell_shape_selected_identity.rs` | `covered` |
| `cleanroom-input/qnt/battle-runtime/battle-runtime-attack-spell-shape-selected-identity.mbt.qnt#step:doGuidingBoltNextAttackAdvantage` | `tasks/target-replay-evidence/L15-RR07-FU01B-SPELL-ATTACK-SAVE-DAMAGE-SUBSTRATES.json#L15-RR07-FU01B-SPELL-ATTACK-SAVE-DAMAGE-SUBSTRATES route action=doGuidingBoltNextAttackAdvantage#step:doGuidingBoltNextAttackAdvantage` | `src/qnt_adapters/battle_runtime_attack_spell_shape_selected_identity.rs` | `covered` |
| `cleanroom-input/qnt/battle-runtime/battle-runtime-attack-spell-shape-selected-identity.mbt.qnt#step:doInflictWoundsFailedSave` | `tasks/target-replay-evidence/L15-RR07-FU01B-SPELL-ATTACK-SAVE-DAMAGE-SUBSTRATES.json#L15-RR07-FU01B-SPELL-ATTACK-SAVE-DAMAGE-SUBSTRATES route action=doInflictWoundsFailedSave#step:doInflictWoundsFailedSave` | `src/qnt_adapters/battle_runtime_attack_spell_shape_selected_identity.rs` | `covered` |
| `cleanroom-input/qnt/battle-runtime/battle-runtime-attack-spell-shape-selected-identity.mbt.qnt#step:doInflictWoundsSuccessfulSave` | `tasks/target-replay-evidence/L15-RR07-FU01B-SPELL-ATTACK-SAVE-DAMAGE-SUBSTRATES.json#L15-RR07-FU01B-SPELL-ATTACK-SAVE-DAMAGE-SUBSTRATES route action=doInflictWoundsSuccessfulSave#step:doInflictWoundsSuccessfulSave` | `src/qnt_adapters/battle_runtime_attack_spell_shape_selected_identity.rs` | `covered` |
| `cleanroom-input/qnt/battle-runtime/battle-runtime-attack-spell-shape-selected-identity.mbt.qnt#step:doShockingGraspOpportunityAttackDenied` | `tasks/target-replay-evidence/L15-RR07-FU01B-SPELL-ATTACK-SAVE-DAMAGE-SUBSTRATES.json#L15-RR07-FU01B-SPELL-ATTACK-SAVE-DAMAGE-SUBSTRATES route action=doShockingGraspOpportunityAttackDenied#step:doShockingGraspOpportunityAttackDenied` | `src/qnt_adapters/battle_runtime_attack_spell_shape_selected_identity.rs` | `covered` |
| `cleanroom-input/qnt/battle-runtime/battle-runtime-level1-damage-spell-selected-identity.mbt.qnt#step:doResolveBurningHandsMixedConeSavingThrows` | `tasks/target-replay-evidence/L15-RR07-FU01B-SPELL-ATTACK-SAVE-DAMAGE-SUBSTRATES.json#L15-RR07-FU01B-SPELL-ATTACK-SAVE-DAMAGE-SUBSTRATES route action=doResolveBurningHandsMixedConeSavingThrows#step:doResolveBurningHandsMixedConeSavingThrows` | `src/qnt_adapters/battle_runtime_level1_damage_spell_selected_identity.rs` | `covered` |
| `cleanroom-input/qnt/battle-runtime/battle-runtime-level1-damage-spell-selected-identity.mbt.qnt#step:doResolveIceKnifeHitAttackDamageAndBurstSavingThrows` | `tasks/target-replay-evidence/L15-RR07-FU01B-SPELL-ATTACK-SAVE-DAMAGE-SUBSTRATES.json#L15-RR07-FU01B-SPELL-ATTACK-SAVE-DAMAGE-SUBSTRATES route action=doResolveIceKnifeHitAttackDamageAndBurstSavingThrows#step:doResolveIceKnifeHitAttackDamageAndBurstSavingThrows` | `src/qnt_adapters/battle_runtime_level1_damage_spell_selected_identity.rs` | `covered` |
| `cleanroom-input/qnt/battle-runtime/battle-runtime-level1-damage-spell-selected-identity.mbt.qnt#step:doResolveIceKnifeMissBurstSavingThrows` | `tasks/target-replay-evidence/L15-RR07-FU01B-SPELL-ATTACK-SAVE-DAMAGE-SUBSTRATES.json#L15-RR07-FU01B-SPELL-ATTACK-SAVE-DAMAGE-SUBSTRATES route action=doResolveIceKnifeMissBurstSavingThrows#step:doResolveIceKnifeMissBurstSavingThrows` | `src/qnt_adapters/battle_runtime_level1_damage_spell_selected_identity.rs` | `covered` |
| `cleanroom-input/qnt/battle-runtime/battle-runtime-level1-damage-spell-selected-identity.mbt.qnt#step:doResolvePoisonSpraySpellAttackDamage` | `tasks/target-replay-evidence/L15-RR07-FU01B-SPELL-ATTACK-SAVE-DAMAGE-SUBSTRATES.json#L15-RR07-FU01B-SPELL-ATTACK-SAVE-DAMAGE-SUBSTRATES route action=doResolvePoisonSpraySpellAttackDamage#step:doResolvePoisonSpraySpellAttackDamage` | `src/qnt_adapters/battle_runtime_level1_damage_spell_selected_identity.rs` | `covered` |
| `cleanroom-input/qnt/battle-runtime/battle-runtime-level1-damage-spell-selected-identity.mbt.qnt#step:doResolveRayOfSicknessSpellAttackDamageAndPoisoned` | `tasks/target-replay-evidence/L15-RR07-FU01B-SPELL-ATTACK-SAVE-DAMAGE-SUBSTRATES.json#L15-RR07-FU01B-SPELL-ATTACK-SAVE-DAMAGE-SUBSTRATES route action=doResolveRayOfSicknessSpellAttackDamageAndPoisoned#step:doResolveRayOfSicknessSpellAttackDamageAndPoisoned` | `src/qnt_adapters/battle_runtime_level1_damage_spell_selected_identity.rs` | `covered` |
| `cleanroom-input/qnt/battle-runtime/battle-runtime-level1-damage-spell-selected-identity.mbt.qnt#step:doResolveSacredFlameDexteritySavingThrowRadiantDamage` | `tasks/target-replay-evidence/L15-RR07-FU01B-SPELL-ATTACK-SAVE-DAMAGE-SUBSTRATES.json#L15-RR07-FU01B-SPELL-ATTACK-SAVE-DAMAGE-SUBSTRATES route action=doResolveSacredFlameDexteritySavingThrowRadiantDamage#step:doResolveSacredFlameDexteritySavingThrowRadiantDamage` | `src/qnt_adapters/battle_runtime_level1_damage_spell_selected_identity.rs` | `covered` |
| `cleanroom-input/qnt/battle-runtime/battle-runtime-level1-damage-spell-selected-identity.mbt.qnt#step:doResolveSorcerousBurstSpellAttackDamage` | `tasks/target-replay-evidence/L15-RR07-FU01B-SPELL-ATTACK-SAVE-DAMAGE-SUBSTRATES.json#L15-RR07-FU01B-SPELL-ATTACK-SAVE-DAMAGE-SUBSTRATES route action=doResolveSorcerousBurstSpellAttackDamage#step:doResolveSorcerousBurstSpellAttackDamage` | `src/qnt_adapters/battle_runtime_level1_damage_spell_selected_identity.rs` | `covered` |
| `cleanroom-input/qnt/battle-runtime/battle-runtime-level1-damage-spell-selected-identity.mbt.qnt#step:doResolveViciousMockeryWisdomSavingThrowPsychicDamageAndNextAttackDisadvantage` | `tasks/target-replay-evidence/L15-RR07-FU01B-SPELL-ATTACK-SAVE-DAMAGE-SUBSTRATES.json#L15-RR07-FU01B-SPELL-ATTACK-SAVE-DAMAGE-SUBSTRATES route action=doResolveViciousMockeryWisdomSavingThrowPsychicDamageAndNextAttackDisadvantage#step:doResolveViciousMockeryWisdomSavingThrowPsychicDamageAndNextAttackDisadvantage` | `src/qnt_adapters/battle_runtime_level1_damage_spell_selected_identity.rs` | `covered` |
| `cleanroom-input/qnt/battle-runtime/battle-runtime-level1-damage-spell-selected-identity.mbt.qnt#step:doResolveChromaticOrbDuplicateDamageLeap` | `tasks/target-replay-evidence/L15-RR07-FU01B-SPELL-ATTACK-SAVE-DAMAGE-SUBSTRATES.json#L15-RR07-FU01B-SPELL-ATTACK-SAVE-DAMAGE-SUBSTRATES blocker action=doResolveChromaticOrbDuplicateDamageLeap#step:doResolveChromaticOrbDuplicateDamageLeap` | `src/qnt_adapters/battle_runtime_level1_damage_spell_selected_identity.rs` | `blocked` |
| `cleanroom-input/qnt/battle-runtime/battle-runtime-level1-damage-spell-selected-identity.mbt.qnt#step:doResolveStarryWispObjectSpellAttackDamageAndDimLight` | `tasks/target-replay-evidence/L15-RR07-FU01B-SPELL-ATTACK-SAVE-DAMAGE-SUBSTRATES.json#L15-RR07-FU01B-SPELL-ATTACK-SAVE-DAMAGE-SUBSTRATES blocker action=doResolveStarryWispObjectSpellAttackDamageAndDimLight#step:doResolveStarryWispObjectSpellAttackDamageAndDimLight` | `src/qnt_adapters/battle_runtime_level1_damage_spell_selected_identity.rs` | `blocked` |

Harness artifacts:

- Start gate: `tasks/START_GATE.json`
- Engine depth: `tasks/ENGINE_DEPTH_MANIFEST.json`
- State ownership: `tasks/STATE_OWNER_MANIFEST.json`
- Reviewer loop: `tasks/REVIEW_LOOP.json`
- Decider decision: `tasks/DECIDER_DECISION.json`
- History: `tasks/history/L15-RR07-FU01B-SPELL-ATTACK-SAVE-DAMAGE-SUBSTRATES/`
- Target replay evidence: `tasks/target-replay-evidence/L15-RR07-FU01B-SPELL-ATTACK-SAVE-DAMAGE-SUBSTRATES.json`
- Run ledger: `tasks/RUN_LEDGER.json`

Verification results:

- `cargo test attack_spell_shape_adapter_replays_all_branches` passed.
- `cargo test level1_damage_spell_adapter_replays_all_branches` passed.
- `cargo fmt --check` passed.
- `cargo test` passed.
- `cargo clippy --all-targets -- -D warnings` passed.
- `node scripts/check-cleanroom-harness.cjs` passed.
- `git diff --check 6bad2c3509996b8cafab2a1d0258e3062e8ff60b...HEAD` passed.


## L15-RR07-FU01E-ARMOR-CLASS-REACTION-SUBSTRATES: Armor-Class Reaction Substrates

- Manifest source commit SHA: `564376fd95218a209bb9eae5c9ccb54ca3e04a52`
- Source branch inventory SHA: `4bb2b20a85d94e3b90b7c59cbfe6e1edd5ab3ef40410641e999527861f3d3a32`
- Driver: `cleanroom-input/qnt/battle-runtime/battle-runtime-mage-armor-selected-identity.mbt.qnt`
- Driver: `cleanroom-input/qnt/battle-runtime/battle-runtime-reaction-spell-selected-identity.mbt.qnt`
- Machine-readable run ledger: `tasks/RUN_LEDGER.json`

Allowed inputs used:

- `cleanroom-input/qnt/battle-runtime/battle-runtime-mage-armor-selected-identity.mbt.qnt`
- `cleanroom-input/qnt/battle-runtime/battle-runtime-reaction-spell-selected-identity.mbt.qnt`
- `cleanroom-input/qnt/battle-runtime/battle-runtime-reducer-route.qnt`
- `cleanroom-input/raw/srd-5.2.1/Playing-the-Game.md`
- `cleanroom-input/raw/srd-5.2.1/Spells/Gaining-and-Casting.md`
- `cleanroom-input/raw/srd-5.2.1/Spells/Descriptions-M-P.md`
- `src/**`, `tasks/**`, and Rust/Cargo tooling

Behavior implemented:

- Accepted Mage Armor base AC projection through generic `ArmorClassSpellEffect` route using `armor_class_projection` and active-effect state.
- Accepted Shield and Hellish Rebuke selected rows through generic `ReactionSpell` fills for armor-class interruption and failed-save damage.
- Counterspell remains out of scope because it is level 3.

Generated branch coverage:

| Obligation | Target replay evidence | Diagnostic tests | Status |
| --- | --- | --- | --- |
| `cleanroom-input/qnt/battle-runtime/battle-runtime-mage-armor-selected-identity.mbt.qnt#step:doDiscoverMageArmorUnarmoredSelfTarget` | `tasks/target-replay-evidence/L15-RR07-FU01E-ARMOR-CLASS-REACTION-SUBSTRATES.json#L15-RR07-FU01E-ARMOR-CLASS-REACTION-SUBSTRATES target-blocker action=doDiscoverMageArmorUnarmoredSelfTarget#step:doDiscoverMageArmorUnarmoredSelfTarget` | `src/tests/mod.rs` | `blocked` |
| `cleanroom-input/qnt/battle-runtime/battle-runtime-mage-armor-selected-identity.mbt.qnt#step:doExpireMageArmorDuration` | `tasks/target-replay-evidence/L15-RR07-FU01E-ARMOR-CLASS-REACTION-SUBSTRATES.json#L15-RR07-FU01E-ARMOR-CLASS-REACTION-SUBSTRATES target-blocker action=doExpireMageArmorDuration#step:doExpireMageArmorDuration` | `src/tests/mod.rs` | `blocked` |
| `cleanroom-input/qnt/battle-runtime/battle-runtime-mage-armor-selected-identity.mbt.qnt#step:doRejectMageArmorArmoredTarget` | `tasks/target-replay-evidence/L15-RR07-FU01E-ARMOR-CLASS-REACTION-SUBSTRATES.json#L15-RR07-FU01E-ARMOR-CLASS-REACTION-SUBSTRATES target-blocker action=doRejectMageArmorArmoredTarget#step:doRejectMageArmorArmoredTarget` | `src/tests/mod.rs` | `blocked` |
| `cleanroom-input/qnt/battle-runtime/battle-runtime-mage-armor-selected-identity.mbt.qnt#step:doResolveMageArmorBaseArmorClassProjection` | `tasks/target-replay-evidence/L15-RR07-FU01E-ARMOR-CLASS-REACTION-SUBSTRATES.json#L15-RR07-FU01E-ARMOR-CLASS-REACTION-SUBSTRATES route action=doResolveMageArmorBaseArmorClassProjection#step:doResolveMageArmorBaseArmorClassProjection` | `src/tests/mod.rs` | `covered` |
| `cleanroom-input/qnt/battle-runtime/battle-runtime-reaction-spell-selected-identity.mbt.qnt#step:doResolveHellishRebukeFailedSavingThrow` | `tasks/target-replay-evidence/L15-RR07-FU01E-ARMOR-CLASS-REACTION-SUBSTRATES.json#L15-RR07-FU01E-ARMOR-CLASS-REACTION-SUBSTRATES route action=doResolveHellishRebukeFailedSavingThrow#step:doResolveHellishRebukeFailedSavingThrow` | `src/tests/mod.rs` | `covered` |
| `cleanroom-input/qnt/battle-runtime/battle-runtime-reaction-spell-selected-identity.mbt.qnt#step:doResolveShieldReactionSpellHit` | `tasks/target-replay-evidence/L15-RR07-FU01E-ARMOR-CLASS-REACTION-SUBSTRATES.json#L15-RR07-FU01E-ARMOR-CLASS-REACTION-SUBSTRATES route action=doResolveShieldReactionSpellHit#step:doResolveShieldReactionSpellHit` | `src/tests/mod.rs` | `covered` |

Target replay evidence:

- Evidence file: `tasks/target-replay-evidence/L15-RR07-FU01E-ARMOR-CLASS-REACTION-SUBSTRATES.json`
- Target profile: `rust`
- Reproduction seed or trace id: `L15-RR07-FU01E-ARMOR-CLASS-REACTION-SUBSTRATES`

Harness artifacts:

- Start gate: `tasks/history/L15-RR07-FU01E-ARMOR-CLASS-REACTION-SUBSTRATES/START_GATE.json`
- Engine depth: `tasks/history/L15-RR07-FU01E-ARMOR-CLASS-REACTION-SUBSTRATES/ENGINE_DEPTH_MANIFEST.json`
- State ownership: `tasks/history/L15-RR07-FU01E-ARMOR-CLASS-REACTION-SUBSTRATES/STATE_OWNER_MANIFEST.json`
- Reviewer loop: `tasks/history/L15-RR07-FU01E-ARMOR-CLASS-REACTION-SUBSTRATES/REVIEW_LOOP.json`
- Decider decision: `tasks/history/L15-RR07-FU01E-ARMOR-CLASS-REACTION-SUBSTRATES/DECIDER_DECISION.json`
- Run ledger: `tasks/RUN_LEDGER.json`

Remaining gaps:

- `doDiscoverMageArmorUnarmoredSelfTarget`, `doRejectMageArmorArmoredTarget`, and `doExpireMageArmorDuration` are target-blocked for this lane.
- `doResolveCounterspellMagicMissileCast` is out of scope.

Verification results:

- `cargo test mage_armor_adapter_replays_all_branches && cargo test reaction_spell_selected_identity_adapter_replays_in_scope_branches` passed.
- `cargo fmt --check` passed.
- `cargo test` passed.
- `cargo clippy --all-targets -- -D warnings` passed.
- `node scripts/check-cleanroom-harness.cjs` passed.
- `git diff --check 410a784738fba3b80566eae292140327d4e30877...HEAD` passed.

## FU01C Integration Addendum

# Validation Report

tasks/RUN_LEDGER.json is the machine-readable run ledger.

Current manifest source commit SHA: 564376fd95218a209bb9eae5c9ccb54ca3e04a52
Source branch inventory SHA: 4bb2b20a85d94e3b90b7c59cbfe6e1edd5ab3ef40410641e999527861f3d3a32

Allowed inputs used: cleanroom-input/**, local task artifacts, and Rust/Cargo tooling in this cleanroom worktree.

Behavior implemented: accepted FU01C rows route through production BattleSubjectKind diagnostic subjects emitted as BattleEntrypointTrace route events and compared through adapter qRoute projection; rows without generic route subjects are recorded as target blockers.

Generated branch coverage: see the obligation table below for every ledger obligation.

Target replay evidence: tasks/target-replay-evidence/*.json files referenced by tasks/RUN_LEDGER.json.

Harness artifacts: START_GATE.json, ENGINE_DEPTH_MANIFEST.json, STATE_OWNER_MANIFEST.json, REVIEW_LOOP.json, DECIDER_DECISION.json, and matching tasks/history entries.

Remaining gaps: FU01C mark-transfer and condition-immunity rows remain blocked until generic marked-effect and condition-immunity active-effect route subjects exist.

Verification results: recorded in RUN_LEDGER commandResults; this lane requires focused adapter tests, cargo fmt --check, cargo test, cargo clippy --all-targets -- -D warnings, node scripts/check-cleanroom-harness.cjs, and git diff --check 410a784738fba3b80566eae292140327d4e30877...HEAD.

## Ledger Tasks
- T001
  - cleanroom-input/qnt/battle-runtime/battle-runtime-magic-missile.mbt.qnt
- T002
  - cleanroom-input/qnt/battle-runtime/battle-runtime-save-gated-spell-ordering.mbt.qnt
- T003
  - cleanroom-input/qnt/battle-runtime/battle-runtime-hit-point-restoration-ordering.mbt.qnt
- T004
  - cleanroom-input/qnt/battle-runtime/battle-runtime-death-saving-throw.mbt.qnt
- T005
  - cleanroom-input/qnt/battle-runtime/battle-runtime-concentration-break-teardown.mbt.qnt
- RRCONV-19A-RUST-BATTLE-SETUP-ENTRYPOINT
  - cleanroom-input/qnt/battle-runtime/battle-runtime-weapon-attack-skeleton.mbt.qnt
- RRCONV-19B-RUST-ACT-DISCOVERY-CONTRACT
  - cleanroom-input/qnt/battle-runtime/battle-runtime-weapon-attack-skeleton.mbt.qnt
- RRCONV-19D-RUST-TURN-ADVANCE-RESULT
  - cleanroom-input/qnt/battle-runtime/battle-runtime-reducer-spine-contract.mbt.qnt
- RRCONV-19C-RUST-RESOLUTION-RESULT-CONTRACT
  - cleanroom-input/qnt/battle-runtime/battle-runtime-reducer-spine-contract.mbt.qnt
- RRCONV-19E-RUST-END-TURN-SUBJECT-RESOLUTION
  - cleanroom-input/qnt/battle-runtime/battle-runtime-reducer-spine-contract.mbt.qnt
- RRCONV-19F-RUST-ROUTE-EVENT-FROM-REDUCER-RESULT
  - cleanroom-input/qnt/battle-runtime/battle-runtime-magic-missile.mbt.qnt
  - cleanroom-input/qnt/battle-runtime/battle-runtime-save-gated-spell-ordering.mbt.qnt
- RRCONV-19G-RUST-SUBJECT-CONTINUATION-LIFECYCLE
  - cleanroom-input/qnt/battle-runtime/battle-runtime-save-gated-spell-ordering.mbt.qnt
  - cleanroom-input/qnt/battle-runtime/battle-runtime-hit-point-restoration-ordering.mbt.qnt
- L15-RR03-FINISH-CURRENT-DIAGNOSTIC-QUEUE
  - cleanroom-input/qnt/battle-runtime/battle-runtime-concentration-break-teardown.mbt.qnt
  - cleanroom-input/qnt/battle-runtime/battle-runtime-death-saving-throw.mbt.qnt
- L15-RR05-BATTLE-ACTION-ATTACK-STATBLOCK-ROUTES
  - cleanroom-input/qnt/battle-runtime/battle-runtime-spell-attack-ordering.mbt.qnt
  - cleanroom-input/qnt/battle-runtime/battle-runtime-stat-block-action-ordering.mbt.qnt
  - cleanroom-input/qnt/battle-runtime/battle-runtime-stat-block-multi-damage.mbt.qnt
  - cleanroom-input/qnt/battle-runtime/battle-runtime-stat-block-size-gated-condition-rider.mbt.qnt
  - cleanroom-input/qnt/battle-runtime/battle-runtime-weapon-attack-ordering.mbt.qnt
  - cleanroom-input/qnt/battle-runtime/battle-runtime-weapon-attack-skeleton.mbt.qnt
  - cleanroom-input/qnt/battle-runtime/creature-attack.mbt.qnt
- L15-RR06-BATTLE-SPELL-EFFECT-ROUTES
  - cleanroom-input/qnt/battle-runtime/battle-runtime-command-option-next-turn.mbt.qnt
  - cleanroom-input/qnt/battle-runtime/battle-runtime-command-ordering.mbt.qnt
  - cleanroom-input/qnt/battle-runtime/battle-runtime-scalar-buff.mbt.qnt
- L15-RR04A-RULE-CORE-DAMAGE-STATBLOCK-COMPONENTS
  - cleanroom-input/qnt/battle-runtime/rule-core-attack-damage-disposition.mbt.qnt
  - cleanroom-input/qnt/battle-runtime/rule-core-hit-point-damage.mbt.qnt
  - cleanroom-input/qnt/battle-runtime/rule-core-stat-block-controls.mbt.qnt
- L15-RR04B-RULE-CORE-MOVEMENT-REACTION-SHOVE-COMPONENTS
  - cleanroom-input/qnt/battle-runtime/rule-core-movement.mbt.qnt
  - cleanroom-input/qnt/battle-runtime/rule-core-reactions.mbt.qnt
  - cleanroom-input/qnt/battle-runtime/rule-core-shove-outcome.mbt.qnt
- L15-RR04D-RULE-CORE-FEATURE-PROFILE-COMPONENTS
  - cleanroom-input/qnt/battle-runtime/rule-core-features.mbt.qnt
- L15-RR04C-RULE-CORE-SPELL-ABILITY-COMPONENTS
  - cleanroom-input/qnt/battle-runtime/rule-core-ability-skill-command.mbt.qnt
  - cleanroom-input/qnt/battle-runtime/rule-core-spells.mbt.qnt
- L15-RR08-CHARACTER-CREATION-ROUTES
  - cleanroom-input/qnt/character-creation-runtime/character-creation-class-feature-projections.mbt.qnt
  - cleanroom-input/qnt/character-creation-runtime/character-creation-class-feature-selected-identity.mbt.qnt
  - cleanroom-input/qnt/character-creation-runtime/character-creation-cleric-druid-order-selected-identity.mbt.qnt
  - cleanroom-input/qnt/character-creation-runtime/character-creation-fighter-fighting-style-selected-identity.mbt.qnt
  - cleanroom-input/qnt/character-creation-runtime/character-creation-rogue-expertise-selected-identity.mbt.qnt
  - cleanroom-input/qnt/character-creation-runtime/character-creation-runtime.mbt.qnt
  - cleanroom-input/qnt/character-creation-runtime/character-creation-warlock-eldritch-invocations-selected-identity.mbt.qnt
  - cleanroom-input/qnt/character-creation-runtime/character-creation-weapon-mastery-containers-selected-identity.mbt.qnt
- L15-RR09-CHARACTER-SHEET-ROUTES
  - cleanroom-input/qnt/character-sheet-runtime/character-sheet-ability-check-proficiency-bonus.mbt.qnt
  - cleanroom-input/qnt/character-sheet-runtime/character-sheet-arcane-recovery-selected-identity.mbt.qnt
  - cleanroom-input/qnt/character-sheet-runtime/character-sheet-armor-class-base-selected-identity.mbt.qnt
  - cleanroom-input/qnt/character-sheet-runtime/character-sheet-class-feature-selected-identity.mbt.qnt
  - cleanroom-input/qnt/character-sheet-runtime/character-sheet-healing-resource-selected-identity.mbt.qnt
  - cleanroom-input/qnt/character-sheet-runtime/character-sheet-hit-point-maximum.mbt.qnt
  - cleanroom-input/qnt/character-sheet-runtime/character-sheet-hp-rest-hit-dice.mbt.qnt
  - cleanroom-input/qnt/character-sheet-runtime/character-sheet-spell-slots-pact-slots.mbt.qnt
  - cleanroom-input/qnt/character-sheet-runtime/character-sheet-spellbook-ritual-selected-identity.mbt.qnt
  - cleanroom-input/qnt/character-sheet-runtime/character-sheet-weapon-mastery-containers-selected-identity.mbt.qnt
- L15-RR10-CHARACTER-BATTLE-HANDOFF-ROUTES
  - cleanroom-input/qnt/character-battle-runtime/character-battle-init-projection.mbt.qnt
  - cleanroom-input/qnt/character-battle-runtime/character-battle-origin-feat-selected-identity.mbt.qnt
  - cleanroom-input/qnt/character-battle-runtime/character-battle-settlement.mbt.qnt
  - cleanroom-input/qnt/character-battle-runtime/character-layer-projection-lifecycle.mbt.qnt
  - cleanroom-input/qnt/character-battle-runtime/character-sheet-feature-resources.mbt.qnt
- L15-RR07S-B-ZERO-HP-FORM-MOVEMENT-SUBSTRATES
  - cleanroom-input/qnt/battle-runtime/battle-runtime-healing-stabilization-selected-identity.mbt.qnt
  - cleanroom-input/qnt/battle-runtime/battle-runtime-druid-wild-shape-form-lifecycle.mbt.qnt
  - cleanroom-input/qnt/battle-runtime/battle-runtime-movement-forced-movement-selected-identity.mbt.qnt
- L15-RR07S-C-WEAPON-BREATH-FEATURE-SUBSTRATES
  - cleanroom-input/qnt/battle-runtime/battle-runtime-weapon-mastery-selected-identity.mbt.qnt
  - cleanroom-input/qnt/battle-runtime/battle-runtime-dragonborn-breath-weapon.mbt.qnt
  - cleanroom-input/qnt/battle-runtime/battle-runtime-feature-selected-identity.mbt.qnt
- L15-RR07S-A-PASSIVE-ROLL-RESOURCE-SUBSTRATES
  - cleanroom-input/qnt/battle-runtime/battle-runtime-adrenaline-rush.mbt.qnt
  - cleanroom-input/qnt/battle-runtime/battle-runtime-halfling-nimbleness-selected-identity.mbt.qnt
  - cleanroom-input/qnt/battle-runtime/battle-runtime-species-passive-trait-selected-identity.mbt.qnt
  - cleanroom-input/qnt/battle-runtime/battle-runtime-danger-sense-selected-identity.mbt.qnt
  - cleanroom-input/qnt/battle-runtime/battle-runtime-roll-modifier-buff-selected-identity.mbt.qnt
- L15-RR07-FU08A-METAMAGIC-GOVERNOR-QUICKENED-SUBSTRATE
  - cleanroom-input/qnt/battle-runtime/battle-runtime-quickened-spell-governor.mbt.qnt
  - cleanroom-input/qnt/battle-runtime/battle-runtime-sorcerer-metamagic-selected-identity.mbt.qnt
  - cleanroom-input/qnt/battle-runtime/battle-runtime-sorcerer-metamagic-spell-attack-selected-identity.mbt.qnt
  - cleanroom-input/qnt/battle-runtime/battle-runtime-sorcerer-metamagic-spell-attack-sequence-selected-identity.mbt.qnt
- L15-RR07-FU01A-CATALOG-READY-SPELL-SUBSTRATES
  - cleanroom-input/qnt/battle-runtime/battle-runtime-condition-saving-throw-selected-identity.mbt.qnt
  - cleanroom-input/qnt/battle-runtime/battle-runtime-find-familiar-selected-identity.mbt.qnt
  - cleanroom-input/qnt/battle-runtime/battle-runtime-thaumaturgy-selected-identity.mbt.qnt
- L15-RR07-FU08B-METAMAGIC-SAVE-RANGE-TARGET-SUBSTRATES
  - cleanroom-input/qnt/battle-runtime/battle-runtime-sorcerer-metamagic-careful-selected-identity.mbt.qnt
  - cleanroom-input/qnt/battle-runtime/battle-runtime-sorcerer-metamagic-heightened-selected-identity.mbt.qnt
  - cleanroom-input/qnt/battle-runtime/battle-runtime-sorcerer-metamagic-distant-selected-identity.mbt.qnt
  - cleanroom-input/qnt/battle-runtime/battle-runtime-sorcerer-metamagic-twinned-selected-identity.mbt.qnt
- L15-RR07-FU08C-METAMAGIC-REROLL-DAMAGE-PROJECTION-SUBSTRATES
  - cleanroom-input/qnt/battle-runtime/battle-runtime-sorcerer-metamagic-empowered-selected-identity.mbt.qnt
  - cleanroom-input/qnt/battle-runtime/battle-runtime-sorcerer-metamagic-extended-selected-identity.mbt.qnt
  - cleanroom-input/qnt/battle-runtime/battle-runtime-sorcerer-metamagic-seeking-selected-identity.mbt.qnt
  - cleanroom-input/qnt/battle-runtime/battle-runtime-sorcerer-metamagic-subtle-selected-identity.mbt.qnt
  - cleanroom-input/qnt/battle-runtime/battle-runtime-sorcerer-metamagic-transmuted-selected-identity.mbt.qnt
- L15-RR07-FU01B-SPELL-ATTACK-SAVE-DAMAGE-SUBSTRATES
  - cleanroom-input/qnt/battle-runtime/battle-runtime-attack-spell-shape-selected-identity.mbt.qnt
  - cleanroom-input/qnt/battle-runtime/battle-runtime-level1-damage-spell-selected-identity.mbt.qnt
- L15-RR07-FU01C-WEAPON-BUFF-MARK-SMITE-SUBSTRATES
  - cleanroom-input/qnt/battle-runtime/battle-runtime-level1-buff-mark-smite-selected-identity.mbt.qnt

## Obligation Coverage
| Obligation | Target replay evidence | Notes | Status |
| --- | --- | --- | --- |
| cleanroom-input/qnt/battle-runtime/battle-runtime-magic-missile.mbt.qnt#step:doFillMagicMissileAllocation | tasks/target-replay-evidence/T001-battle-runtime-magic-missile.json#T001 seed=1 action=doFillMagicMissileAllocation#step:doFillMagicMissileAllocation | pass | covered |
| cleanroom-input/qnt/battle-runtime/battle-runtime-magic-missile.mbt.qnt#step:doFillMagicMissileDamage | tasks/target-replay-evidence/T001-battle-runtime-magic-missile.json#T001 seed=1 action=doFillMagicMissileDamage#step:doFillMagicMissileDamage | pass | covered |
| cleanroom-input/qnt/battle-runtime/battle-runtime-save-gated-spell-ordering.mbt.qnt#step:doDiscoverAreaSaveDamage | tasks/target-replay-evidence/T002-battle-runtime-save-gated-spell-ordering.json#T002 seed=1 action=doDiscoverAreaSaveDamage#step:doDiscoverAreaSaveDamage | pass | covered |
| cleanroom-input/qnt/battle-runtime/battle-runtime-save-gated-spell-ordering.mbt.qnt#step:doDiscoverTargetListConditionChoice | tasks/target-replay-evidence/T002-battle-runtime-save-gated-spell-ordering.json#T002 seed=1 action=doDiscoverTargetListConditionChoice#step:doDiscoverTargetListConditionChoice | pass | covered |
| cleanroom-input/qnt/battle-runtime/battle-runtime-save-gated-spell-ordering.mbt.qnt#step:doFillAreaDamageDice | tasks/target-replay-evidence/T002-battle-runtime-save-gated-spell-ordering.json#T002 seed=1 action=doFillAreaDamageDice#step:doFillAreaDamageDice | pass | covered |
| cleanroom-input/qnt/battle-runtime/battle-runtime-save-gated-spell-ordering.mbt.qnt#step:doFillAreaSaveFailed | tasks/target-replay-evidence/T002-battle-runtime-save-gated-spell-ordering.json#T002 seed=1 action=doFillAreaSaveFailed#step:doFillAreaSaveFailed | pass | covered |
| cleanroom-input/qnt/battle-runtime/battle-runtime-save-gated-spell-ordering.mbt.qnt#step:doFillConditionChoiceAfterTargetList | tasks/target-replay-evidence/T002-battle-runtime-save-gated-spell-ordering.json#T002 seed=1 action=doFillConditionChoiceAfterTargetList#step:doFillConditionChoiceAfterTargetList | pass | covered |
| cleanroom-input/qnt/battle-runtime/battle-runtime-save-gated-spell-ordering.mbt.qnt#step:doFillConditionChoiceBeforeTargetList | tasks/target-replay-evidence/T002-battle-runtime-save-gated-spell-ordering.json#T002 seed=1 action=doFillConditionChoiceBeforeTargetList#step:doFillConditionChoiceBeforeTargetList | pass | covered |
| cleanroom-input/qnt/battle-runtime/battle-runtime-save-gated-spell-ordering.mbt.qnt#step:doFillConditionSavingThrow | tasks/target-replay-evidence/T002-battle-runtime-save-gated-spell-ordering.json#T002 seed=1 action=doFillConditionSavingThrow#step:doFillConditionSavingThrow | pass | covered |
| cleanroom-input/qnt/battle-runtime/battle-runtime-save-gated-spell-ordering.mbt.qnt#step:doFillTargetListAfterConditionChoice | tasks/target-replay-evidence/T002-battle-runtime-save-gated-spell-ordering.json#T002 seed=1 action=doFillTargetListAfterConditionChoice#step:doFillTargetListAfterConditionChoice | pass | covered |
| cleanroom-input/qnt/battle-runtime/battle-runtime-save-gated-spell-ordering.mbt.qnt#step:doFillTargetListBeforeConditionChoice | tasks/target-replay-evidence/T002-battle-runtime-save-gated-spell-ordering.json#T002 seed=1 action=doFillTargetListBeforeConditionChoice#step:doFillTargetListBeforeConditionChoice | pass | covered |
| cleanroom-input/qnt/battle-runtime/battle-runtime-save-gated-spell-ordering.mbt.qnt#step:doSubmitDamageBeforeSavingThrow | tasks/target-replay-evidence/T002-battle-runtime-save-gated-spell-ordering.json#T002 seed=1 action=doSubmitDamageBeforeSavingThrow#step:doSubmitDamageBeforeSavingThrow | pass | covered |
| cleanroom-input/qnt/battle-runtime/battle-runtime-hit-point-restoration-ordering.mbt.qnt#step:doDiscoverFeatureHealingPool | tasks/target-replay-evidence/T003-battle-runtime-hit-point-restoration-ordering.json#T003 seed=1 action=doDiscoverFeatureHealingPool#step:doDiscoverFeatureHealingPool | pass | covered |
| cleanroom-input/qnt/battle-runtime/battle-runtime-hit-point-restoration-ordering.mbt.qnt#step:doDiscoverSingleTargetSpellHealing | tasks/target-replay-evidence/T003-battle-runtime-hit-point-restoration-ordering.json#T003 seed=1 action=doDiscoverSingleTargetSpellHealing#step:doDiscoverSingleTargetSpellHealing | pass | covered |
| cleanroom-input/qnt/battle-runtime/battle-runtime-hit-point-restoration-ordering.mbt.qnt#step:doDiscoverTargetListSpellHealing | tasks/target-replay-evidence/T003-battle-runtime-hit-point-restoration-ordering.json#T003 seed=1 action=doDiscoverTargetListSpellHealing#step:doDiscoverTargetListSpellHealing | pass | covered |
| cleanroom-input/qnt/battle-runtime/battle-runtime-hit-point-restoration-ordering.mbt.qnt#step:doFillFeatureHealingDistribution | tasks/target-replay-evidence/T003-battle-runtime-hit-point-restoration-ordering.json#T003 seed=1 action=doFillFeatureHealingDistribution#step:doFillFeatureHealingDistribution | pass | covered |
| cleanroom-input/qnt/battle-runtime/battle-runtime-hit-point-restoration-ordering.mbt.qnt#step:doFillSpellHealingRoll | tasks/target-replay-evidence/T003-battle-runtime-hit-point-restoration-ordering.json#T003 seed=1 action=doFillSpellHealingRoll#step:doFillSpellHealingRoll | pass | covered |
| cleanroom-input/qnt/battle-runtime/battle-runtime-hit-point-restoration-ordering.mbt.qnt#step:doFillSpellHealingTargetChoice | tasks/target-replay-evidence/T003-battle-runtime-hit-point-restoration-ordering.json#T003 seed=1 action=doFillSpellHealingTargetChoice#step:doFillSpellHealingTargetChoice | pass | covered |
| cleanroom-input/qnt/battle-runtime/battle-runtime-hit-point-restoration-ordering.mbt.qnt#step:doFillSpellHealingTargetList | tasks/target-replay-evidence/T003-battle-runtime-hit-point-restoration-ordering.json#T003 seed=1 action=doFillSpellHealingTargetList#step:doFillSpellHealingTargetList | pass | covered |
| cleanroom-input/qnt/battle-runtime/battle-runtime-hit-point-restoration-ordering.mbt.qnt#step:doSubmitHealingRollBeforeTargetChoice | tasks/target-replay-evidence/T003-battle-runtime-hit-point-restoration-ordering.json#T003 seed=1 action=doSubmitHealingRollBeforeTargetChoice#step:doSubmitHealingRollBeforeTargetChoice | pass | covered |
| cleanroom-input/qnt/battle-runtime/battle-runtime-hit-point-restoration-ordering.mbt.qnt#step:doSubmitHealingRollBeforeTargetList | tasks/target-replay-evidence/T003-battle-runtime-hit-point-restoration-ordering.json#T003 seed=1 action=doSubmitHealingRollBeforeTargetList#step:doSubmitHealingRollBeforeTargetList | pass | covered |
| cleanroom-input/qnt/battle-runtime/battle-runtime-death-saving-throw.mbt.qnt#step:doDiscoverEndTurnDeathSavingThrow | tasks/target-replay-evidence/T004-battle-runtime-death-saving-throw.json#T004 seed=1 action=doDiscoverEndTurnDeathSavingThrow#step:doDiscoverEndTurnDeathSavingThrow | pass | covered |
| cleanroom-input/qnt/battle-runtime/battle-runtime-death-saving-throw.mbt.qnt#step:doFillDeathSavingThrow | tasks/target-replay-evidence/T004-battle-runtime-death-saving-throw.json#T004 seed=1 action=doFillDeathSavingThrow#step:doFillDeathSavingThrow | pass | covered |
| cleanroom-input/qnt/battle-runtime/battle-runtime-death-saving-throw.mbt.qnt#step:doRejectWrongActorEndTurnAfterResolved | tasks/target-replay-evidence/T004-battle-runtime-death-saving-throw.json#T004 seed=1 action=doRejectWrongActorEndTurnAfterResolved#step:doRejectWrongActorEndTurnAfterResolved | pass | covered |
| cleanroom-input/qnt/battle-runtime/battle-runtime-concentration-break-teardown.mbt.qnt#step:doCastConcentrationSpell | tasks/target-replay-evidence/T005-battle-runtime-concentration-break-teardown.json#T005 seed=1 action=doCastConcentrationSpell#step:doCastConcentrationSpell | pass | covered |
| cleanroom-input/qnt/battle-runtime/battle-runtime-concentration-break-teardown.mbt.qnt#step:doCastReplacementConcentrationSpell | tasks/target-replay-evidence/T005-battle-runtime-concentration-break-teardown.json#T005 seed=1 action=doCastReplacementConcentrationSpell#step:doCastReplacementConcentrationSpell | pass | covered |
| cleanroom-input/qnt/battle-runtime/battle-runtime-concentration-break-teardown.mbt.qnt#step:doDamageRequestsConcentrationSave | tasks/target-replay-evidence/T005-battle-runtime-concentration-break-teardown.json#T005 seed=1 action=doDamageRequestsConcentrationSave#step:doDamageRequestsConcentrationSave | pass | covered |
| cleanroom-input/qnt/battle-runtime/battle-runtime-concentration-break-teardown.mbt.qnt#step:doFailConcentrationSave | tasks/target-replay-evidence/T005-battle-runtime-concentration-break-teardown.json#T005 seed=1 action=doFailConcentrationSave#step:doFailConcentrationSave | pass | covered |
| cleanroom-input/qnt/battle-runtime/battle-runtime-concentration-break-teardown.mbt.qnt#step:doVoluntaryEndConcentration | tasks/target-replay-evidence/T005-battle-runtime-concentration-break-teardown.json#T005 seed=1 action=doVoluntaryEndConcentration#step:doVoluntaryEndConcentration | pass | covered |
| cleanroom-input/qnt/battle-runtime/battle-runtime-weapon-attack-skeleton.mbt.qnt#step:doDiscoverAttack | tasks/target-replay-evidence/RRCONV-19A-battle-setup-entrypoint.json#RRCONV-19A dirty replay action=doDiscoverAttack#step:doDiscoverAttack | pass | covered |
| cleanroom-input/qnt/battle-runtime/battle-runtime-weapon-attack-skeleton.mbt.qnt#step:doFillAttackRollHit | tasks/target-replay-evidence/RRCONV-19A-battle-setup-entrypoint.json#RRCONV-19A dirty replay action=doFillAttackRollHit#step:doFillAttackRollHit | pass | covered |
| cleanroom-input/qnt/battle-runtime/battle-runtime-weapon-attack-skeleton.mbt.qnt#step:doFillAttackRollMiss | tasks/target-replay-evidence/RRCONV-19A-battle-setup-entrypoint.json#RRCONV-19A dirty replay action=doFillAttackRollMiss#step:doFillAttackRollMiss | pass | covered |
| cleanroom-input/qnt/battle-runtime/battle-runtime-weapon-attack-skeleton.mbt.qnt#step:doFillDamageHigh | tasks/target-replay-evidence/RRCONV-19A-battle-setup-entrypoint.json#RRCONV-19A dirty replay action=doFillDamageHigh#step:doFillDamageHigh | pass | covered |
| cleanroom-input/qnt/battle-runtime/battle-runtime-weapon-attack-skeleton.mbt.qnt#step:doFillDamageHighSneakAttack | tasks/target-replay-evidence/RRCONV-19A-battle-setup-entrypoint.json#RRCONV-19A dirty replay action=doFillDamageHighSneakAttack#step:doFillDamageHighSneakAttack | pass | covered |
| cleanroom-input/qnt/battle-runtime/battle-runtime-weapon-attack-skeleton.mbt.qnt#step:doFillDamageLow | tasks/target-replay-evidence/RRCONV-19A-battle-setup-entrypoint.json#RRCONV-19A dirty replay action=doFillDamageLow#step:doFillDamageLow | pass | covered |
| cleanroom-input/qnt/battle-runtime/battle-runtime-weapon-attack-skeleton.mbt.qnt#step:doFillDamageLowSneakAttack | tasks/target-replay-evidence/RRCONV-19A-battle-setup-entrypoint.json#RRCONV-19A dirty replay action=doFillDamageLowSneakAttack#step:doFillDamageLowSneakAttack | pass | covered |
| cleanroom-input/qnt/battle-runtime/battle-runtime-weapon-attack-skeleton.mbt.qnt#step:doFillTarget | tasks/target-replay-evidence/RRCONV-19A-battle-setup-entrypoint.json#RRCONV-19A dirty replay action=doFillTarget#step:doFillTarget | pass | covered |
| cleanroom-input/qnt/battle-runtime/battle-runtime-weapon-attack-skeleton.mbt.qnt#step:doRejectRecursiveSkeletonMultiattack | tasks/target-replay-evidence/RRCONV-19A-battle-setup-entrypoint.json#RRCONV-19A dirty replay action=doRejectRecursiveSkeletonMultiattack#step:doRejectRecursiveSkeletonMultiattack | pass | covered |
| cleanroom-input/qnt/battle-runtime/battle-runtime-weapon-attack-skeleton.mbt.qnt#step:doRejectStaleAfterResolved | tasks/target-replay-evidence/RRCONV-19A-battle-setup-entrypoint.json#RRCONV-19A dirty replay action=doRejectStaleAfterResolved#step:doRejectStaleAfterResolved | pass | covered |
| cleanroom-input/qnt/battle-runtime/battle-runtime-weapon-attack-skeleton.mbt.qnt#step:doRejectWrongTarget | tasks/target-replay-evidence/RRCONV-19A-battle-setup-entrypoint.json#RRCONV-19A dirty replay action=doRejectWrongTarget#step:doRejectWrongTarget | pass | covered |
| cleanroom-input/qnt/battle-runtime/battle-runtime-weapon-attack-skeleton.mbt.qnt#step:doResolveSkeletonMultiattack | tasks/target-replay-evidence/RRCONV-19A-battle-setup-entrypoint.json#RRCONV-19A dirty replay action=doResolveSkeletonMultiattack#step:doResolveSkeletonMultiattack | pass | covered |
| cleanroom-input/qnt/battle-runtime/battle-runtime-weapon-attack-skeleton.mbt.qnt#step:doSpendSkeletonMultiattackDispatch | tasks/target-replay-evidence/RRCONV-19A-battle-setup-entrypoint.json#RRCONV-19A dirty replay action=doSpendSkeletonMultiattackDispatch#step:doSpendSkeletonMultiattackDispatch | pass | covered |
| cleanroom-input/qnt/battle-runtime/battle-runtime-weapon-attack-skeleton.mbt.qnt#step:doStartSkeletonTurn | tasks/target-replay-evidence/RRCONV-19A-battle-setup-entrypoint.json#RRCONV-19A dirty replay action=doStartSkeletonTurn#step:doStartSkeletonTurn | pass | covered |
| cleanroom-input/qnt/battle-runtime/battle-runtime-weapon-attack-skeleton.mbt.qnt#step:doDiscoverAttack | tasks/target-replay-evidence/RRCONV-19B-act-discovery-contract.json#RRCONV-19B dirty replay action=doDiscoverAttack#step:doDiscoverAttack | pass | covered |
| cleanroom-input/qnt/battle-runtime/battle-runtime-weapon-attack-skeleton.mbt.qnt#step:doFillAttackRollHit | tasks/target-replay-evidence/RRCONV-19B-act-discovery-contract.json#RRCONV-19B dirty replay action=doFillAttackRollHit#step:doFillAttackRollHit | pass | covered |
| cleanroom-input/qnt/battle-runtime/battle-runtime-weapon-attack-skeleton.mbt.qnt#step:doFillAttackRollMiss | tasks/target-replay-evidence/RRCONV-19B-act-discovery-contract.json#RRCONV-19B dirty replay action=doFillAttackRollMiss#step:doFillAttackRollMiss | pass | covered |
| cleanroom-input/qnt/battle-runtime/battle-runtime-weapon-attack-skeleton.mbt.qnt#step:doFillDamageHigh | tasks/target-replay-evidence/RRCONV-19B-act-discovery-contract.json#RRCONV-19B dirty replay action=doFillDamageHigh#step:doFillDamageHigh | pass | covered |
| cleanroom-input/qnt/battle-runtime/battle-runtime-weapon-attack-skeleton.mbt.qnt#step:doFillDamageHighSneakAttack | tasks/target-replay-evidence/RRCONV-19B-act-discovery-contract.json#RRCONV-19B dirty replay action=doFillDamageHighSneakAttack#step:doFillDamageHighSneakAttack | pass | covered |
| cleanroom-input/qnt/battle-runtime/battle-runtime-weapon-attack-skeleton.mbt.qnt#step:doFillDamageLow | tasks/target-replay-evidence/RRCONV-19B-act-discovery-contract.json#RRCONV-19B dirty replay action=doFillDamageLow#step:doFillDamageLow | pass | covered |
| cleanroom-input/qnt/battle-runtime/battle-runtime-weapon-attack-skeleton.mbt.qnt#step:doFillDamageLowSneakAttack | tasks/target-replay-evidence/RRCONV-19B-act-discovery-contract.json#RRCONV-19B dirty replay action=doFillDamageLowSneakAttack#step:doFillDamageLowSneakAttack | pass | covered |
| cleanroom-input/qnt/battle-runtime/battle-runtime-weapon-attack-skeleton.mbt.qnt#step:doFillTarget | tasks/target-replay-evidence/RRCONV-19B-act-discovery-contract.json#RRCONV-19B dirty replay action=doFillTarget#step:doFillTarget | pass | covered |
| cleanroom-input/qnt/battle-runtime/battle-runtime-weapon-attack-skeleton.mbt.qnt#step:doRejectRecursiveSkeletonMultiattack | tasks/target-replay-evidence/RRCONV-19B-act-discovery-contract.json#RRCONV-19B dirty replay action=doRejectRecursiveSkeletonMultiattack#step:doRejectRecursiveSkeletonMultiattack | pass | covered |
| cleanroom-input/qnt/battle-runtime/battle-runtime-weapon-attack-skeleton.mbt.qnt#step:doRejectStaleAfterResolved | tasks/target-replay-evidence/RRCONV-19B-act-discovery-contract.json#RRCONV-19B dirty replay action=doRejectStaleAfterResolved#step:doRejectStaleAfterResolved | pass | covered |
| cleanroom-input/qnt/battle-runtime/battle-runtime-weapon-attack-skeleton.mbt.qnt#step:doRejectWrongTarget | tasks/target-replay-evidence/RRCONV-19B-act-discovery-contract.json#RRCONV-19B dirty replay action=doRejectWrongTarget#step:doRejectWrongTarget | pass | covered |
| cleanroom-input/qnt/battle-runtime/battle-runtime-weapon-attack-skeleton.mbt.qnt#step:doResolveSkeletonMultiattack | tasks/target-replay-evidence/RRCONV-19B-act-discovery-contract.json#RRCONV-19B dirty replay action=doResolveSkeletonMultiattack#step:doResolveSkeletonMultiattack | pass | covered |
| cleanroom-input/qnt/battle-runtime/battle-runtime-weapon-attack-skeleton.mbt.qnt#step:doSpendSkeletonMultiattackDispatch | tasks/target-replay-evidence/RRCONV-19B-act-discovery-contract.json#RRCONV-19B dirty replay action=doSpendSkeletonMultiattackDispatch#step:doSpendSkeletonMultiattackDispatch | pass | covered |
| cleanroom-input/qnt/battle-runtime/battle-runtime-weapon-attack-skeleton.mbt.qnt#step:doStartSkeletonTurn | tasks/target-replay-evidence/RRCONV-19B-act-discovery-contract.json#RRCONV-19B dirty replay action=doStartSkeletonTurn#step:doStartSkeletonTurn | pass | covered |
| cleanroom-input/qnt/battle-runtime/battle-runtime-reducer-spine-contract.mbt.qnt#step:doStartBattle | tasks/target-replay-evidence/RRCONV-19D-turn-advance-result.json#RRCONV-19D dirty replay action=doStartBattle#step:doStartBattle | pass | covered |
| cleanroom-input/qnt/battle-runtime/battle-runtime-reducer-spine-contract.mbt.qnt#step:doDiscoverSlotSpell | tasks/target-replay-evidence/RRCONV-19D-turn-advance-result.json#RRCONV-19D dirty replay action=doDiscoverSlotSpell#step:doDiscoverSlotSpell | pass | covered |
| cleanroom-input/qnt/battle-runtime/battle-runtime-reducer-spine-contract.mbt.qnt#step:doResolveSlotSpellTargets | tasks/target-replay-evidence/RRCONV-19D-turn-advance-result.json#RRCONV-19D dirty replay action=doResolveSlotSpellTargets#step:doResolveSlotSpellTargets | pass | covered |
| cleanroom-input/qnt/battle-runtime/battle-runtime-reducer-spine-contract.mbt.qnt#step:doResolveSlotSpellDamage | tasks/target-replay-evidence/RRCONV-19D-turn-advance-result.json#RRCONV-19D dirty replay action=doResolveSlotSpellDamage#step:doResolveSlotSpellDamage | pass | covered |
| cleanroom-input/qnt/battle-runtime/battle-runtime-reducer-spine-contract.mbt.qnt#step:doEndTurnToTarget | tasks/target-replay-evidence/RRCONV-19D-turn-advance-result.json#RRCONV-19D dirty replay action=doEndTurnToTarget#step:doEndTurnToTarget | pass | covered |
| cleanroom-input/qnt/battle-runtime/battle-runtime-reducer-spine-contract.mbt.qnt#step:doDiscoverWeaponAttack | tasks/target-replay-evidence/RRCONV-19D-turn-advance-result.json#RRCONV-19D dirty replay action=doDiscoverWeaponAttack#step:doDiscoverWeaponAttack | pass | covered |
| cleanroom-input/qnt/battle-runtime/battle-runtime-reducer-spine-contract.mbt.qnt#step:doResolveWeaponTarget | tasks/target-replay-evidence/RRCONV-19D-turn-advance-result.json#RRCONV-19D dirty replay action=doResolveWeaponTarget#step:doResolveWeaponTarget | pass | covered |
| cleanroom-input/qnt/battle-runtime/battle-runtime-reducer-spine-contract.mbt.qnt#step:doResolveWeaponAttackHit | tasks/target-replay-evidence/RRCONV-19D-turn-advance-result.json#RRCONV-19D dirty replay action=doResolveWeaponAttackHit#step:doResolveWeaponAttackHit | pass | covered |
| cleanroom-input/qnt/battle-runtime/battle-runtime-reducer-spine-contract.mbt.qnt#step:doResolveWeaponDamage | tasks/target-replay-evidence/RRCONV-19D-turn-advance-result.json#RRCONV-19D dirty replay action=doResolveWeaponDamage#step:doResolveWeaponDamage | pass | covered |
| cleanroom-input/qnt/battle-runtime/battle-runtime-reducer-spine-contract.mbt.qnt#step:doDiscoverSlotSpell | tasks/target-replay-evidence/RRCONV-19C-resolution-result-contract.json#RRCONV-19C-RUST-RESOLUTION-RESULT-CONTRACT fresh replay action=doDiscoverSlotSpell#step:doDiscoverSlotSpell | pass | covered |
| cleanroom-input/qnt/battle-runtime/battle-runtime-reducer-spine-contract.mbt.qnt#step:doDiscoverWeaponAttack | tasks/target-replay-evidence/RRCONV-19C-resolution-result-contract.json#RRCONV-19C-RUST-RESOLUTION-RESULT-CONTRACT fresh replay action=doDiscoverWeaponAttack#step:doDiscoverWeaponAttack | pass | covered |
| cleanroom-input/qnt/battle-runtime/battle-runtime-reducer-spine-contract.mbt.qnt#step:doEndTurnToTarget | tasks/target-replay-evidence/RRCONV-19C-resolution-result-contract.json#RRCONV-19C-RUST-RESOLUTION-RESULT-CONTRACT fresh replay action=doEndTurnToTarget#step:doEndTurnToTarget | pass | covered |
| cleanroom-input/qnt/battle-runtime/battle-runtime-reducer-spine-contract.mbt.qnt#step:doResolveSlotSpellDamage | tasks/target-replay-evidence/RRCONV-19C-resolution-result-contract.json#RRCONV-19C-RUST-RESOLUTION-RESULT-CONTRACT fresh replay action=doResolveSlotSpellDamage#step:doResolveSlotSpellDamage | pass | covered |
| cleanroom-input/qnt/battle-runtime/battle-runtime-reducer-spine-contract.mbt.qnt#step:doResolveSlotSpellTargets | tasks/target-replay-evidence/RRCONV-19C-resolution-result-contract.json#RRCONV-19C-RUST-RESOLUTION-RESULT-CONTRACT fresh replay action=doResolveSlotSpellTargets#step:doResolveSlotSpellTargets | pass | covered |
| cleanroom-input/qnt/battle-runtime/battle-runtime-reducer-spine-contract.mbt.qnt#step:doResolveWeaponAttackHit | tasks/target-replay-evidence/RRCONV-19C-resolution-result-contract.json#RRCONV-19C-RUST-RESOLUTION-RESULT-CONTRACT fresh replay action=doResolveWeaponAttackHit#step:doResolveWeaponAttackHit | pass | covered |
| cleanroom-input/qnt/battle-runtime/battle-runtime-reducer-spine-contract.mbt.qnt#step:doResolveWeaponDamage | tasks/target-replay-evidence/RRCONV-19C-resolution-result-contract.json#RRCONV-19C-RUST-RESOLUTION-RESULT-CONTRACT fresh replay action=doResolveWeaponDamage#step:doResolveWeaponDamage | pass | covered |
| cleanroom-input/qnt/battle-runtime/battle-runtime-reducer-spine-contract.mbt.qnt#step:doResolveWeaponTarget | tasks/target-replay-evidence/RRCONV-19C-resolution-result-contract.json#RRCONV-19C-RUST-RESOLUTION-RESULT-CONTRACT fresh replay action=doResolveWeaponTarget#step:doResolveWeaponTarget | pass | covered |
| cleanroom-input/qnt/battle-runtime/battle-runtime-reducer-spine-contract.mbt.qnt#step:doStartBattle | tasks/target-replay-evidence/RRCONV-19C-resolution-result-contract.json#RRCONV-19C-RUST-RESOLUTION-RESULT-CONTRACT fresh replay action=doStartBattle#step:doStartBattle | pass | covered |
| cleanroom-input/qnt/battle-runtime/battle-runtime-reducer-spine-contract.mbt.qnt#step:doDiscoverSlotSpell | tasks/target-replay-evidence/RRCONV-19E-end-turn-subject-resolution.json#RRCONV-19E-RUST-END-TURN-SUBJECT-RESOLUTION fresh replay action=doDiscoverSlotSpell#step:doDiscoverSlotSpell | pass | covered |
| cleanroom-input/qnt/battle-runtime/battle-runtime-reducer-spine-contract.mbt.qnt#step:doDiscoverWeaponAttack | tasks/target-replay-evidence/RRCONV-19E-end-turn-subject-resolution.json#RRCONV-19E-RUST-END-TURN-SUBJECT-RESOLUTION fresh replay action=doDiscoverWeaponAttack#step:doDiscoverWeaponAttack | pass | covered |
| cleanroom-input/qnt/battle-runtime/battle-runtime-reducer-spine-contract.mbt.qnt#step:doEndTurnToTarget | tasks/target-replay-evidence/RRCONV-19E-end-turn-subject-resolution.json#RRCONV-19E-RUST-END-TURN-SUBJECT-RESOLUTION fresh replay action=doEndTurnToTarget#step:doEndTurnToTarget | pass | covered |
| cleanroom-input/qnt/battle-runtime/battle-runtime-reducer-spine-contract.mbt.qnt#step:doResolveSlotSpellDamage | tasks/target-replay-evidence/RRCONV-19E-end-turn-subject-resolution.json#RRCONV-19E-RUST-END-TURN-SUBJECT-RESOLUTION fresh replay action=doResolveSlotSpellDamage#step:doResolveSlotSpellDamage | pass | covered |
| cleanroom-input/qnt/battle-runtime/battle-runtime-reducer-spine-contract.mbt.qnt#step:doResolveSlotSpellTargets | tasks/target-replay-evidence/RRCONV-19E-end-turn-subject-resolution.json#RRCONV-19E-RUST-END-TURN-SUBJECT-RESOLUTION fresh replay action=doResolveSlotSpellTargets#step:doResolveSlotSpellTargets | pass | covered |
| cleanroom-input/qnt/battle-runtime/battle-runtime-reducer-spine-contract.mbt.qnt#step:doResolveWeaponAttackHit | tasks/target-replay-evidence/RRCONV-19E-end-turn-subject-resolution.json#RRCONV-19E-RUST-END-TURN-SUBJECT-RESOLUTION fresh replay action=doResolveWeaponAttackHit#step:doResolveWeaponAttackHit | pass | covered |
| cleanroom-input/qnt/battle-runtime/battle-runtime-reducer-spine-contract.mbt.qnt#step:doResolveWeaponDamage | tasks/target-replay-evidence/RRCONV-19E-end-turn-subject-resolution.json#RRCONV-19E-RUST-END-TURN-SUBJECT-RESOLUTION fresh replay action=doResolveWeaponDamage#step:doResolveWeaponDamage | pass | covered |
| cleanroom-input/qnt/battle-runtime/battle-runtime-reducer-spine-contract.mbt.qnt#step:doResolveWeaponTarget | tasks/target-replay-evidence/RRCONV-19E-end-turn-subject-resolution.json#RRCONV-19E-RUST-END-TURN-SUBJECT-RESOLUTION fresh replay action=doResolveWeaponTarget#step:doResolveWeaponTarget | pass | covered |
| cleanroom-input/qnt/battle-runtime/battle-runtime-reducer-spine-contract.mbt.qnt#step:doStartBattle | tasks/target-replay-evidence/RRCONV-19E-end-turn-subject-resolution.json#RRCONV-19E-RUST-END-TURN-SUBJECT-RESOLUTION fresh replay action=doStartBattle#step:doStartBattle | pass | covered |
| cleanroom-input/qnt/battle-runtime/battle-runtime-magic-missile.mbt.qnt#step:doFillMagicMissileAllocation | tasks/target-replay-evidence/RRCONV-19F-route-event-from-reducer-result.json#RRCONV-19F-RUST-ROUTE-EVENT-FROM-REDUCER-RESULT replay action=doFillMagicMissileAllocation#step:doFillMagicMissileAllocation | pass | covered |
| cleanroom-input/qnt/battle-runtime/battle-runtime-magic-missile.mbt.qnt#step:doFillMagicMissileDamage | tasks/target-replay-evidence/RRCONV-19F-route-event-from-reducer-result.json#RRCONV-19F-RUST-ROUTE-EVENT-FROM-REDUCER-RESULT replay action=doFillMagicMissileDamage#step:doFillMagicMissileDamage | pass | covered |
| cleanroom-input/qnt/battle-runtime/battle-runtime-save-gated-spell-ordering.mbt.qnt#step:doDiscoverAreaSaveDamage | tasks/target-replay-evidence/RRCONV-19F-route-event-from-reducer-result.json#RRCONV-19F-RUST-ROUTE-EVENT-FROM-REDUCER-RESULT replay action=doDiscoverAreaSaveDamage#step:doDiscoverAreaSaveDamage | pass | covered |
| cleanroom-input/qnt/battle-runtime/battle-runtime-save-gated-spell-ordering.mbt.qnt#step:doDiscoverTargetListConditionChoice | tasks/target-replay-evidence/RRCONV-19F-route-event-from-reducer-result.json#RRCONV-19F-RUST-ROUTE-EVENT-FROM-REDUCER-RESULT replay action=doDiscoverTargetListConditionChoice#step:doDiscoverTargetListConditionChoice | pass | covered |
| cleanroom-input/qnt/battle-runtime/battle-runtime-save-gated-spell-ordering.mbt.qnt#step:doFillAreaDamageDice | tasks/target-replay-evidence/RRCONV-19F-route-event-from-reducer-result.json#RRCONV-19F-RUST-ROUTE-EVENT-FROM-REDUCER-RESULT replay action=doFillAreaDamageDice#step:doFillAreaDamageDice | pass | covered |
| cleanroom-input/qnt/battle-runtime/battle-runtime-save-gated-spell-ordering.mbt.qnt#step:doFillAreaSaveFailed | tasks/target-replay-evidence/RRCONV-19F-route-event-from-reducer-result.json#RRCONV-19F-RUST-ROUTE-EVENT-FROM-REDUCER-RESULT replay action=doFillAreaSaveFailed#step:doFillAreaSaveFailed | pass | covered |
| cleanroom-input/qnt/battle-runtime/battle-runtime-save-gated-spell-ordering.mbt.qnt#step:doFillConditionChoiceAfterTargetList | tasks/target-replay-evidence/RRCONV-19F-route-event-from-reducer-result.json#RRCONV-19F-RUST-ROUTE-EVENT-FROM-REDUCER-RESULT replay action=doFillConditionChoiceAfterTargetList#step:doFillConditionChoiceAfterTargetList | pass | covered |
| cleanroom-input/qnt/battle-runtime/battle-runtime-save-gated-spell-ordering.mbt.qnt#step:doFillConditionChoiceBeforeTargetList | tasks/target-replay-evidence/RRCONV-19F-route-event-from-reducer-result.json#RRCONV-19F-RUST-ROUTE-EVENT-FROM-REDUCER-RESULT replay action=doFillConditionChoiceBeforeTargetList#step:doFillConditionChoiceBeforeTargetList | pass | covered |
| cleanroom-input/qnt/battle-runtime/battle-runtime-save-gated-spell-ordering.mbt.qnt#step:doFillConditionSavingThrow | tasks/target-replay-evidence/RRCONV-19F-route-event-from-reducer-result.json#RRCONV-19F-RUST-ROUTE-EVENT-FROM-REDUCER-RESULT replay action=doFillConditionSavingThrow#step:doFillConditionSavingThrow | pass | covered |
| cleanroom-input/qnt/battle-runtime/battle-runtime-save-gated-spell-ordering.mbt.qnt#step:doFillTargetListAfterConditionChoice | tasks/target-replay-evidence/RRCONV-19F-route-event-from-reducer-result.json#RRCONV-19F-RUST-ROUTE-EVENT-FROM-REDUCER-RESULT replay action=doFillTargetListAfterConditionChoice#step:doFillTargetListAfterConditionChoice | pass | covered |
| cleanroom-input/qnt/battle-runtime/battle-runtime-save-gated-spell-ordering.mbt.qnt#step:doFillTargetListBeforeConditionChoice | tasks/target-replay-evidence/RRCONV-19F-route-event-from-reducer-result.json#RRCONV-19F-RUST-ROUTE-EVENT-FROM-REDUCER-RESULT replay action=doFillTargetListBeforeConditionChoice#step:doFillTargetListBeforeConditionChoice | pass | covered |
| cleanroom-input/qnt/battle-runtime/battle-runtime-save-gated-spell-ordering.mbt.qnt#step:doSubmitDamageBeforeSavingThrow | tasks/target-replay-evidence/RRCONV-19F-route-event-from-reducer-result.json#RRCONV-19F-RUST-ROUTE-EVENT-FROM-REDUCER-RESULT replay action=doSubmitDamageBeforeSavingThrow#step:doSubmitDamageBeforeSavingThrow | pass | covered |
| cleanroom-input/qnt/battle-runtime/battle-runtime-save-gated-spell-ordering.mbt.qnt#step:doDiscoverAreaSaveDamage | tasks/target-replay-evidence/RRCONV-19G-subject-continuation-lifecycle.json#RRCONV-19G-RUST-SUBJECT-CONTINUATION-LIFECYCLE replay action=doDiscoverAreaSaveDamage#step:doDiscoverAreaSaveDamage | pass | covered |
| cleanroom-input/qnt/battle-runtime/battle-runtime-save-gated-spell-ordering.mbt.qnt#step:doDiscoverTargetListConditionChoice | tasks/target-replay-evidence/RRCONV-19G-subject-continuation-lifecycle.json#RRCONV-19G-RUST-SUBJECT-CONTINUATION-LIFECYCLE replay action=doDiscoverTargetListConditionChoice#step:doDiscoverTargetListConditionChoice | pass | covered |
| cleanroom-input/qnt/battle-runtime/battle-runtime-save-gated-spell-ordering.mbt.qnt#step:doFillAreaDamageDice | tasks/target-replay-evidence/RRCONV-19G-subject-continuation-lifecycle.json#RRCONV-19G-RUST-SUBJECT-CONTINUATION-LIFECYCLE replay action=doFillAreaDamageDice#step:doFillAreaDamageDice | pass | covered |
| cleanroom-input/qnt/battle-runtime/battle-runtime-save-gated-spell-ordering.mbt.qnt#step:doFillAreaSaveFailed | tasks/target-replay-evidence/RRCONV-19G-subject-continuation-lifecycle.json#RRCONV-19G-RUST-SUBJECT-CONTINUATION-LIFECYCLE replay action=doFillAreaSaveFailed#step:doFillAreaSaveFailed | pass | covered |
| cleanroom-input/qnt/battle-runtime/battle-runtime-save-gated-spell-ordering.mbt.qnt#step:doFillConditionChoiceAfterTargetList | tasks/target-replay-evidence/RRCONV-19G-subject-continuation-lifecycle.json#RRCONV-19G-RUST-SUBJECT-CONTINUATION-LIFECYCLE replay action=doFillConditionChoiceAfterTargetList#step:doFillConditionChoiceAfterTargetList | pass | covered |
| cleanroom-input/qnt/battle-runtime/battle-runtime-save-gated-spell-ordering.mbt.qnt#step:doFillConditionChoiceBeforeTargetList | tasks/target-replay-evidence/RRCONV-19G-subject-continuation-lifecycle.json#RRCONV-19G-RUST-SUBJECT-CONTINUATION-LIFECYCLE replay action=doFillConditionChoiceBeforeTargetList#step:doFillConditionChoiceBeforeTargetList | pass | covered |
| cleanroom-input/qnt/battle-runtime/battle-runtime-save-gated-spell-ordering.mbt.qnt#step:doFillConditionSavingThrow | tasks/target-replay-evidence/RRCONV-19G-subject-continuation-lifecycle.json#RRCONV-19G-RUST-SUBJECT-CONTINUATION-LIFECYCLE replay action=doFillConditionSavingThrow#step:doFillConditionSavingThrow | pass | covered |
| cleanroom-input/qnt/battle-runtime/battle-runtime-save-gated-spell-ordering.mbt.qnt#step:doFillTargetListAfterConditionChoice | tasks/target-replay-evidence/RRCONV-19G-subject-continuation-lifecycle.json#RRCONV-19G-RUST-SUBJECT-CONTINUATION-LIFECYCLE replay action=doFillTargetListAfterConditionChoice#step:doFillTargetListAfterConditionChoice | pass | covered |
| cleanroom-input/qnt/battle-runtime/battle-runtime-save-gated-spell-ordering.mbt.qnt#step:doFillTargetListBeforeConditionChoice | tasks/target-replay-evidence/RRCONV-19G-subject-continuation-lifecycle.json#RRCONV-19G-RUST-SUBJECT-CONTINUATION-LIFECYCLE replay action=doFillTargetListBeforeConditionChoice#step:doFillTargetListBeforeConditionChoice | pass | covered |
| cleanroom-input/qnt/battle-runtime/battle-runtime-save-gated-spell-ordering.mbt.qnt#step:doSubmitDamageBeforeSavingThrow | tasks/target-replay-evidence/RRCONV-19G-subject-continuation-lifecycle.json#RRCONV-19G-RUST-SUBJECT-CONTINUATION-LIFECYCLE replay action=doSubmitDamageBeforeSavingThrow#step:doSubmitDamageBeforeSavingThrow | pass | covered |
| cleanroom-input/qnt/battle-runtime/battle-runtime-hit-point-restoration-ordering.mbt.qnt#step:doDiscoverFeatureHealingPool | tasks/target-replay-evidence/RRCONV-19G-subject-continuation-lifecycle.json#RRCONV-19G-RUST-SUBJECT-CONTINUATION-LIFECYCLE replay action=doDiscoverFeatureHealingPool#step:doDiscoverFeatureHealingPool | pass | covered |
| cleanroom-input/qnt/battle-runtime/battle-runtime-hit-point-restoration-ordering.mbt.qnt#step:doDiscoverSingleTargetSpellHealing | tasks/target-replay-evidence/RRCONV-19G-subject-continuation-lifecycle.json#RRCONV-19G-RUST-SUBJECT-CONTINUATION-LIFECYCLE replay action=doDiscoverSingleTargetSpellHealing#step:doDiscoverSingleTargetSpellHealing | pass | covered |
| cleanroom-input/qnt/battle-runtime/battle-runtime-hit-point-restoration-ordering.mbt.qnt#step:doDiscoverTargetListSpellHealing | tasks/target-replay-evidence/RRCONV-19G-subject-continuation-lifecycle.json#RRCONV-19G-RUST-SUBJECT-CONTINUATION-LIFECYCLE replay action=doDiscoverTargetListSpellHealing#step:doDiscoverTargetListSpellHealing | pass | covered |
| cleanroom-input/qnt/battle-runtime/battle-runtime-hit-point-restoration-ordering.mbt.qnt#step:doFillFeatureHealingDistribution | tasks/target-replay-evidence/RRCONV-19G-subject-continuation-lifecycle.json#RRCONV-19G-RUST-SUBJECT-CONTINUATION-LIFECYCLE replay action=doFillFeatureHealingDistribution#step:doFillFeatureHealingDistribution | pass | covered |
| cleanroom-input/qnt/battle-runtime/battle-runtime-hit-point-restoration-ordering.mbt.qnt#step:doFillSpellHealingRoll | tasks/target-replay-evidence/RRCONV-19G-subject-continuation-lifecycle.json#RRCONV-19G-RUST-SUBJECT-CONTINUATION-LIFECYCLE replay action=doFillSpellHealingRoll#step:doFillSpellHealingRoll | pass | covered |
| cleanroom-input/qnt/battle-runtime/battle-runtime-hit-point-restoration-ordering.mbt.qnt#step:doFillSpellHealingTargetChoice | tasks/target-replay-evidence/RRCONV-19G-subject-continuation-lifecycle.json#RRCONV-19G-RUST-SUBJECT-CONTINUATION-LIFECYCLE replay action=doFillSpellHealingTargetChoice#step:doFillSpellHealingTargetChoice | pass | covered |
| cleanroom-input/qnt/battle-runtime/battle-runtime-hit-point-restoration-ordering.mbt.qnt#step:doFillSpellHealingTargetList | tasks/target-replay-evidence/RRCONV-19G-subject-continuation-lifecycle.json#RRCONV-19G-RUST-SUBJECT-CONTINUATION-LIFECYCLE replay action=doFillSpellHealingTargetList#step:doFillSpellHealingTargetList | pass | covered |
| cleanroom-input/qnt/battle-runtime/battle-runtime-hit-point-restoration-ordering.mbt.qnt#step:doSubmitHealingRollBeforeTargetChoice | tasks/target-replay-evidence/RRCONV-19G-subject-continuation-lifecycle.json#RRCONV-19G-RUST-SUBJECT-CONTINUATION-LIFECYCLE replay action=doSubmitHealingRollBeforeTargetChoice#step:doSubmitHealingRollBeforeTargetChoice | pass | covered |
| cleanroom-input/qnt/battle-runtime/battle-runtime-hit-point-restoration-ordering.mbt.qnt#step:doSubmitHealingRollBeforeTargetList | tasks/target-replay-evidence/RRCONV-19G-subject-continuation-lifecycle.json#RRCONV-19G-RUST-SUBJECT-CONTINUATION-LIFECYCLE replay action=doSubmitHealingRollBeforeTargetList#step:doSubmitHealingRollBeforeTargetList | pass | covered |
| cleanroom-input/qnt/battle-runtime/battle-runtime-concentration-break-teardown.mbt.qnt#step:doCastConcentrationSpell | tasks/target-replay-evidence/L15-RR03-reducer-route.json#L15-RR03 route action=doCastConcentrationSpell#step:doCastConcentrationSpell | pass | covered |
| cleanroom-input/qnt/battle-runtime/battle-runtime-concentration-break-teardown.mbt.qnt#step:doCastReplacementConcentrationSpell | tasks/target-replay-evidence/L15-RR03-reducer-route.json#L15-RR03 route action=doCastReplacementConcentrationSpell#step:doCastReplacementConcentrationSpell | pass | covered |
| cleanroom-input/qnt/battle-runtime/battle-runtime-concentration-break-teardown.mbt.qnt#step:doDamageRequestsConcentrationSave | tasks/target-replay-evidence/L15-RR03-reducer-route.json#L15-RR03 route action=doDamageRequestsConcentrationSave#step:doDamageRequestsConcentrationSave | pass | covered |
| cleanroom-input/qnt/battle-runtime/battle-runtime-concentration-break-teardown.mbt.qnt#step:doFailConcentrationSave | tasks/target-replay-evidence/L15-RR03-reducer-route.json#L15-RR03 route action=doFailConcentrationSave#step:doFailConcentrationSave | pass | covered |
| cleanroom-input/qnt/battle-runtime/battle-runtime-concentration-break-teardown.mbt.qnt#step:doVoluntaryEndConcentration | tasks/target-replay-evidence/L15-RR03-reducer-route.json#L15-RR03 route action=doVoluntaryEndConcentration#step:doVoluntaryEndConcentration | pass | covered |
| cleanroom-input/qnt/battle-runtime/battle-runtime-death-saving-throw.mbt.qnt#step:doDiscoverEndTurnDeathSavingThrow | tasks/target-replay-evidence/L15-RR03-reducer-route.json#L15-RR03 route action=doDiscoverEndTurnDeathSavingThrow#step:doDiscoverEndTurnDeathSavingThrow | pass | covered |
| cleanroom-input/qnt/battle-runtime/battle-runtime-death-saving-throw.mbt.qnt#step:doFillDeathSavingThrow | tasks/target-replay-evidence/L15-RR03-reducer-route.json#L15-RR03 route action=doFillDeathSavingThrow#step:doFillDeathSavingThrow | pass | covered |
| cleanroom-input/qnt/battle-runtime/battle-runtime-death-saving-throw.mbt.qnt#step:doRejectWrongActorEndTurnAfterResolved | tasks/target-replay-evidence/L15-RR03-reducer-route.json#L15-RR03 route action=doRejectWrongActorEndTurnAfterResolved#step:doRejectWrongActorEndTurnAfterResolved | pass | covered |
| cleanroom-input/qnt/battle-runtime/battle-runtime-spell-attack-ordering.mbt.qnt#step:doDiscoverSingleTargetSpellAttack | tasks/target-replay-evidence/L15-RR05-attack-statblock-routes.json#L15-RR05-BATTLE-ACTION-ATTACK-STATBLOCK-ROUTES replay action=doDiscoverSingleTargetSpellAttack#step:doDiscoverSingleTargetSpellAttack | pass | covered |
| cleanroom-input/qnt/battle-runtime/battle-runtime-spell-attack-ordering.mbt.qnt#step:doDiscoverTypedSpellAttack | tasks/target-replay-evidence/L15-RR05-attack-statblock-routes.json#L15-RR05-BATTLE-ACTION-ATTACK-STATBLOCK-ROUTES replay action=doDiscoverTypedSpellAttack#step:doDiscoverTypedSpellAttack | pass | covered |
| cleanroom-input/qnt/battle-runtime/battle-runtime-spell-attack-ordering.mbt.qnt#step:doFillAttackRollHit | tasks/target-replay-evidence/L15-RR05-attack-statblock-routes.json#L15-RR05-BATTLE-ACTION-ATTACK-STATBLOCK-ROUTES replay action=doFillAttackRollHit#step:doFillAttackRollHit | pass | covered |
| cleanroom-input/qnt/battle-runtime/battle-runtime-spell-attack-ordering.mbt.qnt#step:doFillAttackRollMiss | tasks/target-replay-evidence/L15-RR05-attack-statblock-routes.json#L15-RR05-BATTLE-ACTION-ATTACK-STATBLOCK-ROUTES replay action=doFillAttackRollMiss#step:doFillAttackRollMiss | pass | covered |
| cleanroom-input/qnt/battle-runtime/battle-runtime-spell-attack-ordering.mbt.qnt#step:doFillDamageDice | tasks/target-replay-evidence/L15-RR05-attack-statblock-routes.json#L15-RR05-BATTLE-ACTION-ATTACK-STATBLOCK-ROUTES replay action=doFillDamageDice#step:doFillDamageDice | pass | covered |
| cleanroom-input/qnt/battle-runtime/battle-runtime-spell-attack-ordering.mbt.qnt#step:doFillDamageTypeAfterTargetChoice | tasks/target-replay-evidence/L15-RR05-attack-statblock-routes.json#L15-RR05-BATTLE-ACTION-ATTACK-STATBLOCK-ROUTES replay action=doFillDamageTypeAfterTargetChoice#step:doFillDamageTypeAfterTargetChoice | pass | covered |
| cleanroom-input/qnt/battle-runtime/battle-runtime-spell-attack-ordering.mbt.qnt#step:doFillDamageTypeBeforeTargetChoice | tasks/target-replay-evidence/L15-RR05-attack-statblock-routes.json#L15-RR05-BATTLE-ACTION-ATTACK-STATBLOCK-ROUTES replay action=doFillDamageTypeBeforeTargetChoice#step:doFillDamageTypeBeforeTargetChoice | pass | covered |
| cleanroom-input/qnt/battle-runtime/battle-runtime-spell-attack-ordering.mbt.qnt#step:doFillTargetChoice | tasks/target-replay-evidence/L15-RR05-attack-statblock-routes.json#L15-RR05-BATTLE-ACTION-ATTACK-STATBLOCK-ROUTES replay action=doFillTargetChoice#step:doFillTargetChoice | pass | covered |
| cleanroom-input/qnt/battle-runtime/battle-runtime-spell-attack-ordering.mbt.qnt#step:doFillTargetChoiceAfterDamageType | tasks/target-replay-evidence/L15-RR05-attack-statblock-routes.json#L15-RR05-BATTLE-ACTION-ATTACK-STATBLOCK-ROUTES replay action=doFillTargetChoiceAfterDamageType#step:doFillTargetChoiceAfterDamageType | pass | covered |
| cleanroom-input/qnt/battle-runtime/battle-runtime-spell-attack-ordering.mbt.qnt#step:doFillTargetChoiceBeforeDamageType | tasks/target-replay-evidence/L15-RR05-attack-statblock-routes.json#L15-RR05-BATTLE-ACTION-ATTACK-STATBLOCK-ROUTES replay action=doFillTargetChoiceBeforeDamageType#step:doFillTargetChoiceBeforeDamageType | pass | covered |
| cleanroom-input/qnt/battle-runtime/battle-runtime-spell-attack-ordering.mbt.qnt#step:doSubmitAttackRollBeforeTargetChoice | tasks/target-replay-evidence/L15-RR05-attack-statblock-routes.json#L15-RR05-BATTLE-ACTION-ATTACK-STATBLOCK-ROUTES replay action=doSubmitAttackRollBeforeTargetChoice#step:doSubmitAttackRollBeforeTargetChoice | pass | covered |
| cleanroom-input/qnt/battle-runtime/battle-runtime-spell-attack-ordering.mbt.qnt#step:doSubmitDamageBeforeAttackRoll | tasks/target-replay-evidence/L15-RR05-attack-statblock-routes.json#L15-RR05-BATTLE-ACTION-ATTACK-STATBLOCK-ROUTES replay action=doSubmitDamageBeforeAttackRoll#step:doSubmitDamageBeforeAttackRoll | pass | covered |
| cleanroom-input/qnt/battle-runtime/battle-runtime-stat-block-action-ordering.mbt.qnt#step:doDiscoverRolledActionAttackControl | tasks/target-replay-evidence/L15-RR05-attack-statblock-routes.json#L15-RR05-BATTLE-ACTION-ATTACK-STATBLOCK-ROUTES replay action=doDiscoverRolledActionAttackControl#step:doDiscoverRolledActionAttackControl | pass | covered |
| cleanroom-input/qnt/battle-runtime/battle-runtime-stat-block-action-ordering.mbt.qnt#step:doDiscoverStaticActionAttackControl | tasks/target-replay-evidence/L15-RR05-attack-statblock-routes.json#L15-RR05-BATTLE-ACTION-ATTACK-STATBLOCK-ROUTES replay action=doDiscoverStaticActionAttackControl#step:doDiscoverStaticActionAttackControl | pass | covered |
| cleanroom-input/qnt/battle-runtime/battle-runtime-stat-block-action-ordering.mbt.qnt#step:doFillAttackRollMiss | tasks/target-replay-evidence/L15-RR05-attack-statblock-routes.json#L15-RR05-BATTLE-ACTION-ATTACK-STATBLOCK-ROUTES replay action=doFillAttackRollMiss#step:doFillAttackRollMiss | pass | covered |
| cleanroom-input/qnt/battle-runtime/battle-runtime-stat-block-action-ordering.mbt.qnt#step:doFillDamageDice | tasks/target-replay-evidence/L15-RR05-attack-statblock-routes.json#L15-RR05-BATTLE-ACTION-ATTACK-STATBLOCK-ROUTES replay action=doFillDamageDice#step:doFillDamageDice | pass | covered |
| cleanroom-input/qnt/battle-runtime/battle-runtime-stat-block-action-ordering.mbt.qnt#step:doFillRechargeRoll | tasks/target-replay-evidence/L15-RR05-attack-statblock-routes.json#L15-RR05-BATTLE-ACTION-ATTACK-STATBLOCK-ROUTES replay action=doFillRechargeRoll#step:doFillRechargeRoll | pass | covered |
| cleanroom-input/qnt/battle-runtime/battle-runtime-stat-block-action-ordering.mbt.qnt#step:doFillRolledAttackRollHit | tasks/target-replay-evidence/L15-RR05-attack-statblock-routes.json#L15-RR05-BATTLE-ACTION-ATTACK-STATBLOCK-ROUTES replay action=doFillRolledAttackRollHit#step:doFillRolledAttackRollHit | pass | covered |
| cleanroom-input/qnt/battle-runtime/battle-runtime-stat-block-action-ordering.mbt.qnt#step:doFillStaticAttackRollHit | tasks/target-replay-evidence/L15-RR05-attack-statblock-routes.json#L15-RR05-BATTLE-ACTION-ATTACK-STATBLOCK-ROUTES replay action=doFillStaticAttackRollHit#step:doFillStaticAttackRollHit | pass | covered |
| cleanroom-input/qnt/battle-runtime/battle-runtime-stat-block-action-ordering.mbt.qnt#step:doFillTargetChoice | tasks/target-replay-evidence/L15-RR05-attack-statblock-routes.json#L15-RR05-BATTLE-ACTION-ATTACK-STATBLOCK-ROUTES replay action=doFillTargetChoice#step:doFillTargetChoice | pass | covered |
| cleanroom-input/qnt/battle-runtime/battle-runtime-stat-block-action-ordering.mbt.qnt#step:doRejectAttackRollBeforeTargetChoice | tasks/target-replay-evidence/L15-RR05-attack-statblock-routes.json#L15-RR05-BATTLE-ACTION-ATTACK-STATBLOCK-ROUTES replay action=doRejectAttackRollBeforeTargetChoice#step:doRejectAttackRollBeforeTargetChoice | pass | covered |
| cleanroom-input/qnt/battle-runtime/battle-runtime-stat-block-action-ordering.mbt.qnt#step:doRejectDamageBeforeAttackRoll | tasks/target-replay-evidence/L15-RR05-attack-statblock-routes.json#L15-RR05-BATTLE-ACTION-ATTACK-STATBLOCK-ROUTES replay action=doRejectDamageBeforeAttackRoll#step:doRejectDamageBeforeAttackRoll | pass | covered |
| cleanroom-input/qnt/battle-runtime/battle-runtime-stat-block-action-ordering.mbt.qnt#step:doSpendRechargeGatedRolledAttack | tasks/target-replay-evidence/L15-RR05-attack-statblock-routes.json#L15-RR05-BATTLE-ACTION-ATTACK-STATBLOCK-ROUTES replay action=doSpendRechargeGatedRolledAttack#step:doSpendRechargeGatedRolledAttack | pass | covered |
| cleanroom-input/qnt/battle-runtime/battle-runtime-stat-block-action-ordering.mbt.qnt#step:doStartMultiattackControl | tasks/target-replay-evidence/L15-RR05-attack-statblock-routes.json#L15-RR05-BATTLE-ACTION-ATTACK-STATBLOCK-ROUTES replay action=doStartMultiattackControl#step:doStartMultiattackControl | pass | covered |
| cleanroom-input/qnt/battle-runtime/battle-runtime-stat-block-multi-damage.mbt.qnt#step:doFillHitAttackRoll | tasks/target-replay-evidence/L15-RR05-attack-statblock-routes.json#L15-RR05-BATTLE-ACTION-ATTACK-STATBLOCK-ROUTES replay action=doFillHitAttackRoll#step:doFillHitAttackRoll | pass | covered |
| cleanroom-input/qnt/battle-runtime/battle-runtime-stat-block-multi-damage.mbt.qnt#step:doFillTargetChoice | tasks/target-replay-evidence/L15-RR05-attack-statblock-routes.json#L15-RR05-BATTLE-ACTION-ATTACK-STATBLOCK-ROUTES replay action=doFillTargetChoice#step:doFillTargetChoice | pass | covered |
| cleanroom-input/qnt/battle-runtime/battle-runtime-stat-block-multi-damage.mbt.qnt#step:doResolveRolledDamage | tasks/target-replay-evidence/L15-RR05-attack-statblock-routes.json#L15-RR05-BATTLE-ACTION-ATTACK-STATBLOCK-ROUTES replay action=doResolveRolledDamage#step:doResolveRolledDamage | pass | covered |
| cleanroom-input/qnt/battle-runtime/battle-runtime-stat-block-size-gated-condition-rider.mbt.qnt#step:doFillHitAttackRoll | tasks/target-replay-evidence/L15-RR05-attack-statblock-routes.json#L15-RR05-BATTLE-ACTION-ATTACK-STATBLOCK-ROUTES replay action=doFillHitAttackRoll#step:doFillHitAttackRoll | pass | covered |
| cleanroom-input/qnt/battle-runtime/battle-runtime-stat-block-size-gated-condition-rider.mbt.qnt#step:doFillTargetChoice | tasks/target-replay-evidence/L15-RR05-attack-statblock-routes.json#L15-RR05-BATTLE-ACTION-ATTACK-STATBLOCK-ROUTES replay action=doFillTargetChoice#step:doFillTargetChoice | pass | covered |
| cleanroom-input/qnt/battle-runtime/battle-runtime-stat-block-size-gated-condition-rider.mbt.qnt#step:doResolveDamage | tasks/target-replay-evidence/L15-RR05-attack-statblock-routes.json#L15-RR05-BATTLE-ACTION-ATTACK-STATBLOCK-ROUTES replay action=doResolveDamage#step:doResolveDamage | pass | covered |
| cleanroom-input/qnt/battle-runtime/battle-runtime-weapon-attack-ordering.mbt.qnt#step:doDiscoverAttack | tasks/target-replay-evidence/L15-RR05-attack-statblock-routes.json#L15-RR05-BATTLE-ACTION-ATTACK-STATBLOCK-ROUTES replay action=doDiscoverAttack#step:doDiscoverAttack | pass | covered |
| cleanroom-input/qnt/battle-runtime/battle-runtime-weapon-attack-ordering.mbt.qnt#step:doFillAttackRollHit | tasks/target-replay-evidence/L15-RR05-attack-statblock-routes.json#L15-RR05-BATTLE-ACTION-ATTACK-STATBLOCK-ROUTES replay action=doFillAttackRollHit#step:doFillAttackRollHit | pass | covered |
| cleanroom-input/qnt/battle-runtime/battle-runtime-weapon-attack-ordering.mbt.qnt#step:doFillAttackRollMiss | tasks/target-replay-evidence/L15-RR05-attack-statblock-routes.json#L15-RR05-BATTLE-ACTION-ATTACK-STATBLOCK-ROUTES replay action=doFillAttackRollMiss#step:doFillAttackRollMiss | pass | covered |
| cleanroom-input/qnt/battle-runtime/battle-runtime-weapon-attack-ordering.mbt.qnt#step:doFillDamageDice | tasks/target-replay-evidence/L15-RR05-attack-statblock-routes.json#L15-RR05-BATTLE-ACTION-ATTACK-STATBLOCK-ROUTES replay action=doFillDamageDice#step:doFillDamageDice | pass | covered |
| cleanroom-input/qnt/battle-runtime/battle-runtime-weapon-attack-ordering.mbt.qnt#step:doFillTargetChoice | tasks/target-replay-evidence/L15-RR05-attack-statblock-routes.json#L15-RR05-BATTLE-ACTION-ATTACK-STATBLOCK-ROUTES replay action=doFillTargetChoice#step:doFillTargetChoice | pass | covered |
| cleanroom-input/qnt/battle-runtime/battle-runtime-weapon-attack-ordering.mbt.qnt#step:doRejectAttackRollBeforeTargetChoice | tasks/target-replay-evidence/L15-RR05-attack-statblock-routes.json#L15-RR05-BATTLE-ACTION-ATTACK-STATBLOCK-ROUTES replay action=doRejectAttackRollBeforeTargetChoice#step:doRejectAttackRollBeforeTargetChoice | pass | covered |
| cleanroom-input/qnt/battle-runtime/battle-runtime-weapon-attack-ordering.mbt.qnt#step:doRejectDamageBeforeAttackRoll | tasks/target-replay-evidence/L15-RR05-attack-statblock-routes.json#L15-RR05-BATTLE-ACTION-ATTACK-STATBLOCK-ROUTES replay action=doRejectDamageBeforeAttackRoll#step:doRejectDamageBeforeAttackRoll | pass | covered |
| cleanroom-input/qnt/battle-runtime/battle-runtime-weapon-attack-skeleton.mbt.qnt#step:doDiscoverAttack | tasks/target-replay-evidence/L15-RR05-attack-statblock-routes.json#L15-RR05-BATTLE-ACTION-ATTACK-STATBLOCK-ROUTES replay action=doDiscoverAttack#step:doDiscoverAttack | pass | covered |
| cleanroom-input/qnt/battle-runtime/battle-runtime-weapon-attack-skeleton.mbt.qnt#step:doFillAttackRollHit | tasks/target-replay-evidence/L15-RR05-attack-statblock-routes.json#L15-RR05-BATTLE-ACTION-ATTACK-STATBLOCK-ROUTES replay action=doFillAttackRollHit#step:doFillAttackRollHit | pass | covered |
| cleanroom-input/qnt/battle-runtime/battle-runtime-weapon-attack-skeleton.mbt.qnt#step:doFillAttackRollMiss | tasks/target-replay-evidence/L15-RR05-attack-statblock-routes.json#L15-RR05-BATTLE-ACTION-ATTACK-STATBLOCK-ROUTES replay action=doFillAttackRollMiss#step:doFillAttackRollMiss | pass | covered |
| cleanroom-input/qnt/battle-runtime/battle-runtime-weapon-attack-skeleton.mbt.qnt#step:doFillDamageHigh | tasks/target-replay-evidence/L15-RR05-attack-statblock-routes.json#L15-RR05-BATTLE-ACTION-ATTACK-STATBLOCK-ROUTES replay action=doFillDamageHigh#step:doFillDamageHigh | pass | covered |
| cleanroom-input/qnt/battle-runtime/battle-runtime-weapon-attack-skeleton.mbt.qnt#step:doFillDamageHighSneakAttack | tasks/target-replay-evidence/L15-RR05-attack-statblock-routes.json#L15-RR05-BATTLE-ACTION-ATTACK-STATBLOCK-ROUTES replay action=doFillDamageHighSneakAttack#step:doFillDamageHighSneakAttack | pass | covered |
| cleanroom-input/qnt/battle-runtime/battle-runtime-weapon-attack-skeleton.mbt.qnt#step:doFillDamageLow | tasks/target-replay-evidence/L15-RR05-attack-statblock-routes.json#L15-RR05-BATTLE-ACTION-ATTACK-STATBLOCK-ROUTES replay action=doFillDamageLow#step:doFillDamageLow | pass | covered |
| cleanroom-input/qnt/battle-runtime/battle-runtime-weapon-attack-skeleton.mbt.qnt#step:doFillDamageLowSneakAttack | tasks/target-replay-evidence/L15-RR05-attack-statblock-routes.json#L15-RR05-BATTLE-ACTION-ATTACK-STATBLOCK-ROUTES replay action=doFillDamageLowSneakAttack#step:doFillDamageLowSneakAttack | pass | covered |
| cleanroom-input/qnt/battle-runtime/battle-runtime-weapon-attack-skeleton.mbt.qnt#step:doFillTarget | tasks/target-replay-evidence/L15-RR05-attack-statblock-routes.json#L15-RR05-BATTLE-ACTION-ATTACK-STATBLOCK-ROUTES replay action=doFillTarget#step:doFillTarget | pass | covered |
| cleanroom-input/qnt/battle-runtime/battle-runtime-weapon-attack-skeleton.mbt.qnt#step:doRejectRecursiveSkeletonMultiattack | tasks/target-replay-evidence/L15-RR05-attack-statblock-routes.json#L15-RR05-BATTLE-ACTION-ATTACK-STATBLOCK-ROUTES replay action=doRejectRecursiveSkeletonMultiattack#step:doRejectRecursiveSkeletonMultiattack | pass | covered |
| cleanroom-input/qnt/battle-runtime/battle-runtime-weapon-attack-skeleton.mbt.qnt#step:doRejectStaleAfterResolved | tasks/target-replay-evidence/L15-RR05-attack-statblock-routes.json#L15-RR05-BATTLE-ACTION-ATTACK-STATBLOCK-ROUTES replay action=doRejectStaleAfterResolved#step:doRejectStaleAfterResolved | pass | covered |
| cleanroom-input/qnt/battle-runtime/battle-runtime-weapon-attack-skeleton.mbt.qnt#step:doRejectWrongTarget | tasks/target-replay-evidence/L15-RR05-attack-statblock-routes.json#L15-RR05-BATTLE-ACTION-ATTACK-STATBLOCK-ROUTES replay action=doRejectWrongTarget#step:doRejectWrongTarget | pass | covered |
| cleanroom-input/qnt/battle-runtime/battle-runtime-weapon-attack-skeleton.mbt.qnt#step:doResolveSkeletonMultiattack | tasks/target-replay-evidence/L15-RR05-attack-statblock-routes.json#L15-RR05-BATTLE-ACTION-ATTACK-STATBLOCK-ROUTES replay action=doResolveSkeletonMultiattack#step:doResolveSkeletonMultiattack | pass | covered |
| cleanroom-input/qnt/battle-runtime/battle-runtime-weapon-attack-skeleton.mbt.qnt#step:doSpendSkeletonMultiattackDispatch | tasks/target-replay-evidence/L15-RR05-attack-statblock-routes.json#L15-RR05-BATTLE-ACTION-ATTACK-STATBLOCK-ROUTES replay action=doSpendSkeletonMultiattackDispatch#step:doSpendSkeletonMultiattackDispatch | pass | covered |
| cleanroom-input/qnt/battle-runtime/battle-runtime-weapon-attack-skeleton.mbt.qnt#step:doStartSkeletonTurn | tasks/target-replay-evidence/L15-RR05-attack-statblock-routes.json#L15-RR05-BATTLE-ACTION-ATTACK-STATBLOCK-ROUTES replay action=doStartSkeletonTurn#step:doStartSkeletonTurn | pass | covered |
| cleanroom-input/qnt/battle-runtime/creature-attack.mbt.qnt#step:doAttackerAAttacks | tasks/target-replay-evidence/L15-RR05-attack-statblock-routes.json#L15-RR05-BATTLE-ACTION-ATTACK-STATBLOCK-ROUTES replay action=doAttackerAAttacks#step:doAttackerAAttacks | pass | covered |
| cleanroom-input/qnt/battle-runtime/creature-attack.mbt.qnt#step:doAttackerBAttacks | tasks/target-replay-evidence/L15-RR05-attack-statblock-routes.json#L15-RR05-BATTLE-ACTION-ATTACK-STATBLOCK-ROUTES replay action=doAttackerBAttacks#step:doAttackerBAttacks | pass | covered |
| cleanroom-input/qnt/battle-runtime/battle-runtime-command-option-next-turn.mbt.qnt#step:doFailedSaveRecordsPending | tasks/target-replay-evidence/L15-RR06-spell-effect-routes.json#L15-RR06-BATTLE-SPELL-EFFECT-ROUTES replay action=doFailedSaveRecordsPending#step:doFailedSaveRecordsPending | pass | covered |
| cleanroom-input/qnt/battle-runtime/battle-runtime-command-option-next-turn.mbt.qnt#step:doFollowGrovel | tasks/target-replay-evidence/L15-RR06-spell-effect-routes.json#L15-RR06-BATTLE-SPELL-EFFECT-ROUTES replay action=doFollowGrovel#step:doFollowGrovel | pass | covered |
| cleanroom-input/qnt/battle-runtime/battle-runtime-command-option-next-turn.mbt.qnt#step:doFollowDrop | tasks/target-replay-evidence/L15-RR06-spell-effect-routes.json#L15-RR06-BATTLE-SPELL-EFFECT-ROUTES replay action=doFollowDrop#step:doFollowDrop | pass | covered |
| cleanroom-input/qnt/battle-runtime/battle-runtime-command-option-next-turn.mbt.qnt#step:doHaltSuppresses | tasks/target-replay-evidence/L15-RR06-spell-effect-routes.json#L15-RR06-BATTLE-SPELL-EFFECT-ROUTES replay action=doHaltSuppresses#step:doHaltSuppresses | pass | covered |
| cleanroom-input/qnt/battle-runtime/battle-runtime-command-option-next-turn.mbt.qnt#step:doHaltEndTurnCleanup | tasks/target-replay-evidence/L15-RR06-spell-effect-routes.json#L15-RR06-BATTLE-SPELL-EFFECT-ROUTES replay action=doHaltEndTurnCleanup#step:doHaltEndTurnCleanup | pass | covered |
| cleanroom-input/qnt/battle-runtime/battle-runtime-command-option-next-turn.mbt.qnt#step:doApproachContinues | tasks/target-replay-evidence/L15-RR06-spell-effect-routes.json#L15-RR06-BATTLE-SPELL-EFFECT-ROUTES replay action=doApproachContinues#step:doApproachContinues | pass | covered |
| cleanroom-input/qnt/battle-runtime/battle-runtime-command-option-next-turn.mbt.qnt#step:doApproachWithinFiveEndsTurn | tasks/target-replay-evidence/L15-RR06-spell-effect-routes.json#L15-RR06-BATTLE-SPELL-EFFECT-ROUTES replay action=doApproachWithinFiveEndsTurn#step:doApproachWithinFiveEndsTurn | pass | covered |
| cleanroom-input/qnt/battle-runtime/battle-runtime-command-option-next-turn.mbt.qnt#step:doApproachMovementRejected | tasks/target-replay-evidence/L15-RR06-spell-effect-routes.json#L15-RR06-BATTLE-SPELL-EFFECT-ROUTES replay action=doApproachMovementRejected#step:doApproachMovementRejected | pass | covered |
| cleanroom-input/qnt/battle-runtime/battle-runtime-command-option-next-turn.mbt.qnt#step:doApproachNoMovementCleanup | tasks/target-replay-evidence/L15-RR06-spell-effect-routes.json#L15-RR06-BATTLE-SPELL-EFFECT-ROUTES replay action=doApproachNoMovementCleanup#step:doApproachNoMovementCleanup | pass | covered |
| cleanroom-input/qnt/battle-runtime/battle-runtime-command-option-next-turn.mbt.qnt#step:doFleeFullMovementEndsTurn | tasks/target-replay-evidence/L15-RR06-spell-effect-routes.json#L15-RR06-BATTLE-SPELL-EFFECT-ROUTES replay action=doFleeFullMovementEndsTurn#step:doFleeFullMovementEndsTurn | pass | covered |
| cleanroom-input/qnt/battle-runtime/battle-runtime-command-option-next-turn.mbt.qnt#step:doFleePartialMovementRejected | tasks/target-replay-evidence/L15-RR06-spell-effect-routes.json#L15-RR06-BATTLE-SPELL-EFFECT-ROUTES replay action=doFleePartialMovementRejected#step:doFleePartialMovementRejected | pass | covered |
| cleanroom-input/qnt/battle-runtime/battle-runtime-command-option-next-turn.mbt.qnt#step:doFleeNoMovementCleanup | tasks/target-replay-evidence/L15-RR06-spell-effect-routes.json#L15-RR06-BATTLE-SPELL-EFFECT-ROUTES replay action=doFleeNoMovementCleanup#step:doFleeNoMovementCleanup | pass | covered |
| cleanroom-input/qnt/battle-runtime/battle-runtime-command-option-next-turn.mbt.qnt#step:doFleeOpportunityAttackWindow | tasks/target-replay-evidence/L15-RR06-spell-effect-routes.json#L15-RR06-BATTLE-SPELL-EFFECT-ROUTES replay action=doFleeOpportunityAttackWindow#step:doFleeOpportunityAttackWindow | pass | covered |
| cleanroom-input/qnt/battle-runtime/battle-runtime-command-option-next-turn.mbt.qnt#step:doFleeOpportunityAttackDeclinedContinuation | tasks/target-replay-evidence/L15-RR06-spell-effect-routes.json#L15-RR06-BATTLE-SPELL-EFFECT-ROUTES replay action=doFleeOpportunityAttackDeclinedContinuation#step:doFleeOpportunityAttackDeclinedContinuation | pass | covered |
| cleanroom-input/qnt/battle-runtime/battle-runtime-command-option-next-turn.mbt.qnt#step:doComplete | tasks/target-replay-evidence/L15-RR06-spell-effect-routes.json#L15-RR06-BATTLE-SPELL-EFFECT-ROUTES replay action=doComplete#step:doComplete | pass | covered |
| cleanroom-input/qnt/battle-runtime/battle-runtime-command-ordering.mbt.qnt#step:doDiscoverCommand | tasks/target-replay-evidence/L15-RR06-spell-effect-routes.json#L15-RR06-BATTLE-SPELL-EFFECT-ROUTES replay action=doDiscoverCommand#step:doDiscoverCommand | pass | covered |
| cleanroom-input/qnt/battle-runtime/battle-runtime-command-ordering.mbt.qnt#step:doSubmitOptionBeforeTargetList | tasks/target-replay-evidence/L15-RR06-spell-effect-routes.json#L15-RR06-BATTLE-SPELL-EFFECT-ROUTES replay action=doSubmitOptionBeforeTargetList#step:doSubmitOptionBeforeTargetList | pass | covered |
| cleanroom-input/qnt/battle-runtime/battle-runtime-command-ordering.mbt.qnt#step:doFillTargetList | tasks/target-replay-evidence/L15-RR06-spell-effect-routes.json#L15-RR06-BATTLE-SPELL-EFFECT-ROUTES replay action=doFillTargetList#step:doFillTargetList | pass | covered |
| cleanroom-input/qnt/battle-runtime/battle-runtime-command-ordering.mbt.qnt#step:doSubmitSavingThrowBeforeOption | tasks/target-replay-evidence/L15-RR06-spell-effect-routes.json#L15-RR06-BATTLE-SPELL-EFFECT-ROUTES replay action=doSubmitSavingThrowBeforeOption#step:doSubmitSavingThrowBeforeOption | pass | covered |
| cleanroom-input/qnt/battle-runtime/battle-runtime-command-ordering.mbt.qnt#step:doFillGrovelOption | tasks/target-replay-evidence/L15-RR06-spell-effect-routes.json#L15-RR06-BATTLE-SPELL-EFFECT-ROUTES replay action=doFillGrovelOption#step:doFillGrovelOption | pass | covered |
| cleanroom-input/qnt/battle-runtime/battle-runtime-command-ordering.mbt.qnt#step:doFillFailedGrovelSavingThrow | tasks/target-replay-evidence/L15-RR06-spell-effect-routes.json#L15-RR06-BATTLE-SPELL-EFFECT-ROUTES replay action=doFillFailedGrovelSavingThrow#step:doFillFailedGrovelSavingThrow | pass | covered |
| cleanroom-input/qnt/battle-runtime/battle-runtime-command-ordering.mbt.qnt#step:doFollowGrovel | tasks/target-replay-evidence/L15-RR06-spell-effect-routes.json#L15-RR06-BATTLE-SPELL-EFFECT-ROUTES replay action=doFollowGrovel#step:doFollowGrovel | pass | covered |
| cleanroom-input/qnt/battle-runtime/battle-runtime-command-ordering.mbt.qnt#step:doDropNeedsHeldObjectFacts | tasks/target-replay-evidence/L15-RR06-spell-effect-routes.json#L15-RR06-BATTLE-SPELL-EFFECT-ROUTES replay action=doDropNeedsHeldObjectFacts#step:doDropNeedsHeldObjectFacts | pass | covered |
| cleanroom-input/qnt/battle-runtime/battle-runtime-command-ordering.mbt.qnt#step:doFillDropHeldObjectFacts | tasks/target-replay-evidence/L15-RR06-spell-effect-routes.json#L15-RR06-BATTLE-SPELL-EFFECT-ROUTES replay action=doFillDropHeldObjectFacts#step:doFillDropHeldObjectFacts | pass | covered |
| cleanroom-input/qnt/battle-runtime/battle-runtime-command-ordering.mbt.qnt#step:doHaltSuppresses | tasks/target-replay-evidence/L15-RR06-spell-effect-routes.json#L15-RR06-BATTLE-SPELL-EFFECT-ROUTES replay action=doHaltSuppresses#step:doHaltSuppresses | pass | covered |
| cleanroom-input/qnt/battle-runtime/battle-runtime-command-ordering.mbt.qnt#step:doApproachMovementContinues | tasks/target-replay-evidence/L15-RR06-spell-effect-routes.json#L15-RR06-BATTLE-SPELL-EFFECT-ROUTES replay action=doApproachMovementContinues#step:doApproachMovementContinues | pass | covered |
| cleanroom-input/qnt/battle-runtime/battle-runtime-command-ordering.mbt.qnt#step:doFillApproachMovementContinues | tasks/target-replay-evidence/L15-RR06-spell-effect-routes.json#L15-RR06-BATTLE-SPELL-EFFECT-ROUTES replay action=doFillApproachMovementContinues#step:doFillApproachMovementContinues | pass | covered |
| cleanroom-input/qnt/battle-runtime/battle-runtime-command-ordering.mbt.qnt#step:doFillApproachMovementWithinFive | tasks/target-replay-evidence/L15-RR06-spell-effect-routes.json#L15-RR06-BATTLE-SPELL-EFFECT-ROUTES replay action=doFillApproachMovementWithinFive#step:doFillApproachMovementWithinFive | pass | covered |
| cleanroom-input/qnt/battle-runtime/battle-runtime-command-ordering.mbt.qnt#step:doApproachNoMovement | tasks/target-replay-evidence/L15-RR06-spell-effect-routes.json#L15-RR06-BATTLE-SPELL-EFFECT-ROUTES replay action=doApproachNoMovement#step:doApproachNoMovement | pass | covered |
| cleanroom-input/qnt/battle-runtime/battle-runtime-command-ordering.mbt.qnt#step:doFleeMovement | tasks/target-replay-evidence/L15-RR06-spell-effect-routes.json#L15-RR06-BATTLE-SPELL-EFFECT-ROUTES replay action=doFleeMovement#step:doFleeMovement | pass | covered |
| cleanroom-input/qnt/battle-runtime/battle-runtime-command-ordering.mbt.qnt#step:doFillFleeMovement | tasks/target-replay-evidence/L15-RR06-spell-effect-routes.json#L15-RR06-BATTLE-SPELL-EFFECT-ROUTES replay action=doFillFleeMovement#step:doFillFleeMovement | pass | covered |
| cleanroom-input/qnt/battle-runtime/battle-runtime-command-ordering.mbt.qnt#step:doRejectFleePartialMovement | tasks/target-replay-evidence/L15-RR06-spell-effect-routes.json#L15-RR06-BATTLE-SPELL-EFFECT-ROUTES replay action=doRejectFleePartialMovement#step:doRejectFleePartialMovement | pass | covered |
| cleanroom-input/qnt/battle-runtime/battle-runtime-command-ordering.mbt.qnt#step:doFleeNoMovement | tasks/target-replay-evidence/L15-RR06-spell-effect-routes.json#L15-RR06-BATTLE-SPELL-EFFECT-ROUTES replay action=doFleeNoMovement#step:doFleeNoMovement | pass | covered |
| cleanroom-input/qnt/battle-runtime/battle-runtime-command-ordering.mbt.qnt#step:doFleeOpportunityAttack | tasks/target-replay-evidence/L15-RR06-spell-effect-routes.json#L15-RR06-BATTLE-SPELL-EFFECT-ROUTES replay action=doFleeOpportunityAttack#step:doFleeOpportunityAttack | pass | covered |
| cleanroom-input/qnt/battle-runtime/battle-runtime-scalar-buff.mbt.qnt#step:doFillLongstriderTarget | tasks/target-replay-evidence/L15-RR06-spell-effect-routes.json#L15-RR06-BATTLE-SPELL-EFFECT-ROUTES replay action=doFillLongstriderTarget#step:doFillLongstriderTarget | pass | covered |
| cleanroom-input/qnt/battle-runtime/battle-runtime-scalar-buff.mbt.qnt#step:doRejectStaleAfterResolved | tasks/target-replay-evidence/L15-RR06-spell-effect-routes.json#L15-RR06-BATTLE-SPELL-EFFECT-ROUTES replay action=doRejectStaleAfterResolved#step:doRejectStaleAfterResolved | pass | covered |
| cleanroom-input/qnt/battle-runtime/rule-core-attack-damage-disposition.mbt.qnt#step:doMeleeKnockOut | tasks/target-replay-evidence/L15-RR04A-rule-core-damage-statblock-components.json#L15-RR04A-RULE-CORE-DAMAGE-STATBLOCK-COMPONENTS replay action=doMeleeKnockOut#step:doMeleeKnockOut | pass | covered |
| cleanroom-input/qnt/battle-runtime/rule-core-attack-damage-disposition.mbt.qnt#step:doRejectRangedKnockOut | tasks/target-replay-evidence/L15-RR04A-rule-core-damage-statblock-components.json#L15-RR04A-RULE-CORE-DAMAGE-STATBLOCK-COMPONENTS replay action=doRejectRangedKnockOut#step:doRejectRangedKnockOut | pass | covered |
| cleanroom-input/qnt/battle-runtime/rule-core-hit-point-damage.mbt.qnt#step:doMonsterDiesAtZero | tasks/target-replay-evidence/L15-RR04A-rule-core-damage-statblock-components.json#L15-RR04A-RULE-CORE-DAMAGE-STATBLOCK-COMPONENTS replay action=doMonsterDiesAtZero#step:doMonsterDiesAtZero | pass | covered |
| cleanroom-input/qnt/battle-runtime/rule-core-hit-point-damage.mbt.qnt#step:doPlayerCharacterDiesFromMassiveDamage | tasks/target-replay-evidence/L15-RR04A-rule-core-damage-statblock-components.json#L15-RR04A-RULE-CORE-DAMAGE-STATBLOCK-COMPONENTS replay action=doPlayerCharacterDiesFromMassiveDamage#step:doPlayerCharacterDiesFromMassiveDamage | pass | covered |
| cleanroom-input/qnt/battle-runtime/rule-core-hit-point-damage.mbt.qnt#step:doPlayerCharacterFallsUnconscious | tasks/target-replay-evidence/L15-RR04A-rule-core-damage-statblock-components.json#L15-RR04A-RULE-CORE-DAMAGE-STATBLOCK-COMPONENTS replay action=doPlayerCharacterFallsUnconscious#step:doPlayerCharacterFallsUnconscious | pass | covered |
| cleanroom-input/qnt/battle-runtime/rule-core-hit-point-damage.mbt.qnt#step:doTemporaryHitPointsAbsorbFirst | tasks/target-replay-evidence/L15-RR04A-rule-core-damage-statblock-components.json#L15-RR04A-RULE-CORE-DAMAGE-STATBLOCK-COMPONENTS replay action=doTemporaryHitPointsAbsorbFirst#step:doTemporaryHitPointsAbsorbFirst | pass | covered |
| cleanroom-input/qnt/battle-runtime/rule-core-stat-block-controls.mbt.qnt#step:doEndTurnClosesDispatches | tasks/target-replay-evidence/L15-RR04A-rule-core-damage-statblock-components.json#L15-RR04A-RULE-CORE-DAMAGE-STATBLOCK-COMPONENTS replay action=doEndTurnClosesDispatches#step:doEndTurnClosesDispatches | pass | covered |
| cleanroom-input/qnt/battle-runtime/rule-core-stat-block-controls.mbt.qnt#step:doMoveDuringDispatch | tasks/target-replay-evidence/L15-RR04A-rule-core-damage-statblock-components.json#L15-RR04A-RULE-CORE-DAMAGE-STATBLOCK-COMPONENTS replay action=doMoveDuringDispatch#step:doMoveDuringDispatch | pass | covered |
| cleanroom-input/qnt/battle-runtime/rule-core-stat-block-controls.mbt.qnt#step:doRejectBonusActionDuringDispatch | tasks/target-replay-evidence/L15-RR04A-rule-core-damage-statblock-components.json#L15-RR04A-RULE-CORE-DAMAGE-STATBLOCK-COMPONENTS replay action=doRejectBonusActionDuringDispatch#step:doRejectBonusActionDuringDispatch | pass | covered |
| cleanroom-input/qnt/battle-runtime/rule-core-stat-block-controls.mbt.qnt#step:doRejectOrdinaryActionDuringDispatch | tasks/target-replay-evidence/L15-RR04A-rule-core-damage-statblock-components.json#L15-RR04A-RULE-CORE-DAMAGE-STATBLOCK-COMPONENTS replay action=doRejectOrdinaryActionDuringDispatch#step:doRejectOrdinaryActionDuringDispatch | pass | covered |
| cleanroom-input/qnt/battle-runtime/rule-core-stat-block-controls.mbt.qnt#step:doResolvePrimaryDispatch | tasks/target-replay-evidence/L15-RR04A-rule-core-damage-statblock-components.json#L15-RR04A-RULE-CORE-DAMAGE-STATBLOCK-COMPONENTS replay action=doResolvePrimaryDispatch#step:doResolvePrimaryDispatch | pass | covered |
| cleanroom-input/qnt/battle-runtime/rule-core-stat-block-controls.mbt.qnt#step:doResolveSecondaryDispatch | tasks/target-replay-evidence/L15-RR04A-rule-core-damage-statblock-components.json#L15-RR04A-RULE-CORE-DAMAGE-STATBLOCK-COMPONENTS replay action=doResolveSecondaryDispatch#step:doResolveSecondaryDispatch | pass | covered |
| cleanroom-input/qnt/battle-runtime/rule-core-stat-block-controls.mbt.qnt#step:doStartMultiattack | tasks/target-replay-evidence/L15-RR04A-rule-core-damage-statblock-components.json#L15-RR04A-RULE-CORE-DAMAGE-STATBLOCK-COMPONENTS replay action=doStartMultiattack#step:doStartMultiattack | pass | covered |
| cleanroom-input/qnt/battle-runtime/rule-core-movement.mbt.qnt#step:doDash | tasks/target-replay-evidence/L15-RR04B-rule-core-movement-reaction-shove-components.json#L15-RR04B-RULE-CORE-MOVEMENT-REACTION-SHOVE-COMPONENTS driverPath=cleanroom-input/qnt/battle-runtime/rule-core-movement.mbt.qnt seed=1 action=doDash#step:doDash | pass | covered |
| cleanroom-input/qnt/battle-runtime/rule-core-movement.mbt.qnt#step:doDeclineOpportunityAttack | tasks/target-replay-evidence/L15-RR04B-rule-core-movement-reaction-shove-components.json#L15-RR04B-RULE-CORE-MOVEMENT-REACTION-SHOVE-COMPONENTS driverPath=cleanroom-input/qnt/battle-runtime/rule-core-movement.mbt.qnt seed=1 action=doDeclineOpportunityAttack#step:doDeclineOpportunityAttack | pass | covered |
| cleanroom-input/qnt/battle-runtime/rule-core-movement.mbt.qnt#step:doDiscoverEscapeGrapple | tasks/target-replay-evidence/L15-RR04B-rule-core-movement-reaction-shove-components.json#L15-RR04B-RULE-CORE-MOVEMENT-REACTION-SHOVE-COMPONENTS driverPath=cleanroom-input/qnt/battle-runtime/rule-core-movement.mbt.qnt seed=1 action=doDiscoverEscapeGrapple#step:doDiscoverEscapeGrapple | pass | covered |
| cleanroom-input/qnt/battle-runtime/rule-core-movement.mbt.qnt#step:doDiscoverGrapple | tasks/target-replay-evidence/L15-RR04B-rule-core-movement-reaction-shove-components.json#L15-RR04B-RULE-CORE-MOVEMENT-REACTION-SHOVE-COMPONENTS driverPath=cleanroom-input/qnt/battle-runtime/rule-core-movement.mbt.qnt seed=1 action=doDiscoverGrapple#step:doDiscoverGrapple | pass | covered |
| cleanroom-input/qnt/battle-runtime/rule-core-movement.mbt.qnt#step:doDiscoverMovement | tasks/target-replay-evidence/L15-RR04B-rule-core-movement-reaction-shove-components.json#L15-RR04B-RULE-CORE-MOVEMENT-REACTION-SHOVE-COMPONENTS driverPath=cleanroom-input/qnt/battle-runtime/rule-core-movement.mbt.qnt seed=1 action=doDiscoverMovement#step:doDiscoverMovement | pass | covered |
| cleanroom-input/qnt/battle-runtime/rule-core-movement.mbt.qnt#step:doDisengage | tasks/target-replay-evidence/L15-RR04B-rule-core-movement-reaction-shove-components.json#L15-RR04B-RULE-CORE-MOVEMENT-REACTION-SHOVE-COMPONENTS driverPath=cleanroom-input/qnt/battle-runtime/rule-core-movement.mbt.qnt seed=1 action=doDisengage#step:doDisengage | pass | covered |
| cleanroom-input/qnt/battle-runtime/rule-core-movement.mbt.qnt#step:doMoveProvokesOpportunityAttack | tasks/target-replay-evidence/L15-RR04B-rule-core-movement-reaction-shove-components.json#L15-RR04B-RULE-CORE-MOVEMENT-REACTION-SHOVE-COMPONENTS driverPath=cleanroom-input/qnt/battle-runtime/rule-core-movement.mbt.qnt seed=1 action=doMoveProvokesOpportunityAttack#step:doMoveProvokesOpportunityAttack | pass | covered |
| cleanroom-input/qnt/battle-runtime/rule-core-movement.mbt.qnt#step:doMoveThreatSuppressedByDisengage | tasks/target-replay-evidence/L15-RR04B-rule-core-movement-reaction-shove-components.json#L15-RR04B-RULE-CORE-MOVEMENT-REACTION-SHOVE-COMPONENTS driverPath=cleanroom-input/qnt/battle-runtime/rule-core-movement.mbt.qnt seed=1 action=doMoveThreatSuppressedByDisengage#step:doMoveThreatSuppressedByDisengage | pass | covered |
| cleanroom-input/qnt/battle-runtime/rule-core-movement.mbt.qnt#step:doRejectDashAfterActionSpent | tasks/target-replay-evidence/L15-RR04B-rule-core-movement-reaction-shove-components.json#L15-RR04B-RULE-CORE-MOVEMENT-REACTION-SHOVE-COMPONENTS driverPath=cleanroom-input/qnt/battle-runtime/rule-core-movement.mbt.qnt seed=1 action=doRejectDashAfterActionSpent#step:doRejectDashAfterActionSpent | pass | covered |
| cleanroom-input/qnt/battle-runtime/rule-core-movement.mbt.qnt#step:doRejectMovementOverspend | tasks/target-replay-evidence/L15-RR04B-rule-core-movement-reaction-shove-components.json#L15-RR04B-RULE-CORE-MOVEMENT-REACTION-SHOVE-COMPONENTS driverPath=cleanroom-input/qnt/battle-runtime/rule-core-movement.mbt.qnt seed=1 action=doRejectMovementOverspend#step:doRejectMovementOverspend | pass | covered |
| cleanroom-input/qnt/battle-runtime/rule-core-movement.mbt.qnt#step:doReleaseGrapple | tasks/target-replay-evidence/L15-RR04B-rule-core-movement-reaction-shove-components.json#L15-RR04B-RULE-CORE-MOVEMENT-REACTION-SHOVE-COMPONENTS driverPath=cleanroom-input/qnt/battle-runtime/rule-core-movement.mbt.qnt seed=1 action=doReleaseGrapple#step:doReleaseGrapple | pass | covered |
| cleanroom-input/qnt/battle-runtime/rule-core-movement.mbt.qnt#step:doResolveEscapeFailure | tasks/target-replay-evidence/L15-RR04B-rule-core-movement-reaction-shove-components.json#L15-RR04B-RULE-CORE-MOVEMENT-REACTION-SHOVE-COMPONENTS driverPath=cleanroom-input/qnt/battle-runtime/rule-core-movement.mbt.qnt seed=1 action=doResolveEscapeFailure#step:doResolveEscapeFailure | pass | covered |
| cleanroom-input/qnt/battle-runtime/rule-core-movement.mbt.qnt#step:doResolveEscapeSuccess | tasks/target-replay-evidence/L15-RR04B-rule-core-movement-reaction-shove-components.json#L15-RR04B-RULE-CORE-MOVEMENT-REACTION-SHOVE-COMPONENTS driverPath=cleanroom-input/qnt/battle-runtime/rule-core-movement.mbt.qnt seed=1 action=doResolveEscapeSuccess#step:doResolveEscapeSuccess | pass | covered |
| cleanroom-input/qnt/battle-runtime/rule-core-movement.mbt.qnt#step:doResolveGrappleFailure | tasks/target-replay-evidence/L15-RR04B-rule-core-movement-reaction-shove-components.json#L15-RR04B-RULE-CORE-MOVEMENT-REACTION-SHOVE-COMPONENTS driverPath=cleanroom-input/qnt/battle-runtime/rule-core-movement.mbt.qnt seed=1 action=doResolveGrappleFailure#step:doResolveGrappleFailure | pass | covered |
| cleanroom-input/qnt/battle-runtime/rule-core-movement.mbt.qnt#step:doResolveGrappleSuccess | tasks/target-replay-evidence/L15-RR04B-rule-core-movement-reaction-shove-components.json#L15-RR04B-RULE-CORE-MOVEMENT-REACTION-SHOVE-COMPONENTS driverPath=cleanroom-input/qnt/battle-runtime/rule-core-movement.mbt.qnt seed=1 action=doResolveGrappleSuccess#step:doResolveGrappleSuccess | pass | covered |
| cleanroom-input/qnt/battle-runtime/rule-core-movement.mbt.qnt#step:doSelectGrappleTarget | tasks/target-replay-evidence/L15-RR04B-rule-core-movement-reaction-shove-components.json#L15-RR04B-RULE-CORE-MOVEMENT-REACTION-SHOVE-COMPONENTS driverPath=cleanroom-input/qnt/battle-runtime/rule-core-movement.mbt.qnt seed=1 action=doSelectGrappleTarget#step:doSelectGrappleTarget | pass | covered |
| cleanroom-input/qnt/battle-runtime/rule-core-movement.mbt.qnt#step:doSpendFullMovement | tasks/target-replay-evidence/L15-RR04B-rule-core-movement-reaction-shove-components.json#L15-RR04B-RULE-CORE-MOVEMENT-REACTION-SHOVE-COMPONENTS driverPath=cleanroom-input/qnt/battle-runtime/rule-core-movement.mbt.qnt seed=1 action=doSpendFullMovement#step:doSpendFullMovement | pass | covered |
| cleanroom-input/qnt/battle-runtime/rule-core-movement.mbt.qnt#step:doSpendMovement | tasks/target-replay-evidence/L15-RR04B-rule-core-movement-reaction-shove-components.json#L15-RR04B-RULE-CORE-MOVEMENT-REACTION-SHOVE-COMPONENTS driverPath=cleanroom-input/qnt/battle-runtime/rule-core-movement.mbt.qnt seed=1 action=doSpendMovement#step:doSpendMovement | pass | covered |
| cleanroom-input/qnt/battle-runtime/rule-core-movement.mbt.qnt#step:doSpendShortMovement | tasks/target-replay-evidence/L15-RR04B-rule-core-movement-reaction-shove-components.json#L15-RR04B-RULE-CORE-MOVEMENT-REACTION-SHOVE-COMPONENTS driverPath=cleanroom-input/qnt/battle-runtime/rule-core-movement.mbt.qnt seed=1 action=doSpendShortMovement#step:doSpendShortMovement | pass | covered |
| cleanroom-input/qnt/battle-runtime/rule-core-movement.mbt.qnt#step:doStandFromProne | tasks/target-replay-evidence/L15-RR04B-rule-core-movement-reaction-shove-components.json#L15-RR04B-RULE-CORE-MOVEMENT-REACTION-SHOVE-COMPONENTS driverPath=cleanroom-input/qnt/battle-runtime/rule-core-movement.mbt.qnt seed=1 action=doStandFromProne#step:doStandFromProne | pass | covered |
| cleanroom-input/qnt/battle-runtime/rule-core-movement.mbt.qnt#step:doStartGrappledTargetTurn | tasks/target-replay-evidence/L15-RR04B-rule-core-movement-reaction-shove-components.json#L15-RR04B-RULE-CORE-MOVEMENT-REACTION-SHOVE-COMPONENTS driverPath=cleanroom-input/qnt/battle-runtime/rule-core-movement.mbt.qnt seed=1 action=doStartGrappledTargetTurn#step:doStartGrappledTargetTurn | pass | covered |
| cleanroom-input/qnt/battle-runtime/rule-core-reactions.mbt.qnt#step:doBreakReactorConcentrationAfterLargeDamage | tasks/target-replay-evidence/L15-RR04B-rule-core-movement-reaction-shove-components.json#L15-RR04B-RULE-CORE-MOVEMENT-REACTION-SHOVE-COMPONENTS driverPath=cleanroom-input/qnt/battle-runtime/rule-core-reactions.mbt.qnt seed=1 action=doBreakReactorConcentrationAfterLargeDamage#step:doBreakReactorConcentrationAfterLargeDamage | pass | covered |
| cleanroom-input/qnt/battle-runtime/rule-core-reactions.mbt.qnt#step:doDeclineOpportunityAttack | tasks/target-replay-evidence/L15-RR04B-rule-core-movement-reaction-shove-components.json#L15-RR04B-RULE-CORE-MOVEMENT-REACTION-SHOVE-COMPONENTS driverPath=cleanroom-input/qnt/battle-runtime/rule-core-reactions.mbt.qnt seed=1 action=doDeclineOpportunityAttack#step:doDeclineOpportunityAttack | pass | covered |
| cleanroom-input/qnt/battle-runtime/rule-core-reactions.mbt.qnt#step:doDeclineReadiedMovement | tasks/target-replay-evidence/L15-RR04B-rule-core-movement-reaction-shove-components.json#L15-RR04B-RULE-CORE-MOVEMENT-REACTION-SHOVE-COMPONENTS driverPath=cleanroom-input/qnt/battle-runtime/rule-core-reactions.mbt.qnt seed=1 action=doDeclineReadiedMovement#step:doDeclineReadiedMovement | pass | covered |
| cleanroom-input/qnt/battle-runtime/rule-core-reactions.mbt.qnt#step:doHoldReactorConcentrationAfterSmallDamage | tasks/target-replay-evidence/L15-RR04B-rule-core-movement-reaction-shove-components.json#L15-RR04B-RULE-CORE-MOVEMENT-REACTION-SHOVE-COMPONENTS driverPath=cleanroom-input/qnt/battle-runtime/rule-core-reactions.mbt.qnt seed=1 action=doHoldReactorConcentrationAfterSmallDamage#step:doHoldReactorConcentrationAfterSmallDamage | pass | covered |
| cleanroom-input/qnt/battle-runtime/rule-core-reactions.mbt.qnt#step:doOfferOpportunityAttack | tasks/target-replay-evidence/L15-RR04B-rule-core-movement-reaction-shove-components.json#L15-RR04B-RULE-CORE-MOVEMENT-REACTION-SHOVE-COMPONENTS driverPath=cleanroom-input/qnt/battle-runtime/rule-core-reactions.mbt.qnt seed=1 action=doOfferOpportunityAttack#step:doOfferOpportunityAttack | pass | covered |
| cleanroom-input/qnt/battle-runtime/rule-core-reactions.mbt.qnt#step:doOfferReadiedMovement | tasks/target-replay-evidence/L15-RR04B-rule-core-movement-reaction-shove-components.json#L15-RR04B-RULE-CORE-MOVEMENT-REACTION-SHOVE-COMPONENTS driverPath=cleanroom-input/qnt/battle-runtime/rule-core-reactions.mbt.qnt seed=1 action=doOfferReadiedMovement#step:doOfferReadiedMovement | pass | covered |
| cleanroom-input/qnt/battle-runtime/rule-core-reactions.mbt.qnt#step:doReadyMovementFixture | tasks/target-replay-evidence/L15-RR04B-rule-core-movement-reaction-shove-components.json#L15-RR04B-RULE-CORE-MOVEMENT-REACTION-SHOVE-COMPONENTS driverPath=cleanroom-input/qnt/battle-runtime/rule-core-reactions.mbt.qnt seed=1 action=doReadyMovementFixture#step:doReadyMovementFixture | pass | covered |
| cleanroom-input/qnt/battle-runtime/rule-core-reactions.mbt.qnt#step:doRejectReadiedMovementZero | tasks/target-replay-evidence/L15-RR04B-rule-core-movement-reaction-shove-components.json#L15-RR04B-RULE-CORE-MOVEMENT-REACTION-SHOVE-COMPONENTS driverPath=cleanroom-input/qnt/battle-runtime/rule-core-reactions.mbt.qnt seed=1 action=doRejectReadiedMovementZero#step:doRejectReadiedMovementZero | pass | covered |
| cleanroom-input/qnt/battle-runtime/rule-core-reactions.mbt.qnt#step:doStartReactorConcentrationFixture | tasks/target-replay-evidence/L15-RR04B-rule-core-movement-reaction-shove-components.json#L15-RR04B-RULE-CORE-MOVEMENT-REACTION-SHOVE-COMPONENTS driverPath=cleanroom-input/qnt/battle-runtime/rule-core-reactions.mbt.qnt seed=1 action=doStartReactorConcentrationFixture#step:doStartReactorConcentrationFixture | pass | covered |
| cleanroom-input/qnt/battle-runtime/rule-core-reactions.mbt.qnt#step:doTakeReadiedMovementFill | tasks/target-replay-evidence/L15-RR04B-rule-core-movement-reaction-shove-components.json#L15-RR04B-RULE-CORE-MOVEMENT-REACTION-SHOVE-COMPONENTS driverPath=cleanroom-input/qnt/battle-runtime/rule-core-reactions.mbt.qnt seed=1 action=doTakeReadiedMovementFill#step:doTakeReadiedMovementFill | pass | covered |
| cleanroom-input/qnt/battle-runtime/rule-core-reactions.mbt.qnt#step:doTakeReadiedMovementShort | tasks/target-replay-evidence/L15-RR04B-rule-core-movement-reaction-shove-components.json#L15-RR04B-RULE-CORE-MOVEMENT-REACTION-SHOVE-COMPONENTS driverPath=cleanroom-input/qnt/battle-runtime/rule-core-reactions.mbt.qnt seed=1 action=doTakeReadiedMovementShort#step:doTakeReadiedMovementShort | pass | covered |
| cleanroom-input/qnt/battle-runtime/rule-core-shove-outcome.mbt.qnt#step:doInvalidPushDistance | tasks/target-replay-evidence/L15-RR04B-rule-core-movement-reaction-shove-components.json#L15-RR04B-RULE-CORE-MOVEMENT-REACTION-SHOVE-COMPONENTS driverPath=cleanroom-input/qnt/battle-runtime/rule-core-shove-outcome.mbt.qnt seed=1 action=doInvalidPushDistance#step:doInvalidPushDistance | pass | covered |
| cleanroom-input/qnt/battle-runtime/rule-core-shove-outcome.mbt.qnt#step:doSaveFailsProne | tasks/target-replay-evidence/L15-RR04B-rule-core-movement-reaction-shove-components.json#L15-RR04B-RULE-CORE-MOVEMENT-REACTION-SHOVE-COMPONENTS driverPath=cleanroom-input/qnt/battle-runtime/rule-core-shove-outcome.mbt.qnt seed=1 action=doSaveFailsProne#step:doSaveFailsProne | pass | covered |
| cleanroom-input/qnt/battle-runtime/rule-core-shove-outcome.mbt.qnt#step:doSaveFailsPush | tasks/target-replay-evidence/L15-RR04B-rule-core-movement-reaction-shove-components.json#L15-RR04B-RULE-CORE-MOVEMENT-REACTION-SHOVE-COMPONENTS driverPath=cleanroom-input/qnt/battle-runtime/rule-core-shove-outcome.mbt.qnt seed=1 action=doSaveFailsPush#step:doSaveFailsPush | pass | covered |
| cleanroom-input/qnt/battle-runtime/rule-core-shove-outcome.mbt.qnt#step:doSaveFailsPushBlocked | tasks/target-replay-evidence/L15-RR04B-rule-core-movement-reaction-shove-components.json#L15-RR04B-RULE-CORE-MOVEMENT-REACTION-SHOVE-COMPONENTS driverPath=cleanroom-input/qnt/battle-runtime/rule-core-shove-outcome.mbt.qnt seed=1 action=doSaveFailsPushBlocked#step:doSaveFailsPushBlocked | pass | covered |
| cleanroom-input/qnt/battle-runtime/rule-core-shove-outcome.mbt.qnt#step:doSaveFailsPushNoLegalDestination | tasks/target-replay-evidence/L15-RR04B-rule-core-movement-reaction-shove-components.json#L15-RR04B-RULE-CORE-MOVEMENT-REACTION-SHOVE-COMPONENTS driverPath=cleanroom-input/qnt/battle-runtime/rule-core-shove-outcome.mbt.qnt seed=1 action=doSaveFailsPushNoLegalDestination#step:doSaveFailsPushNoLegalDestination | pass | covered |
| cleanroom-input/qnt/battle-runtime/rule-core-shove-outcome.mbt.qnt#step:doSaveSucceeds | tasks/target-replay-evidence/L15-RR04B-rule-core-movement-reaction-shove-components.json#L15-RR04B-RULE-CORE-MOVEMENT-REACTION-SHOVE-COMPONENTS driverPath=cleanroom-input/qnt/battle-runtime/rule-core-shove-outcome.mbt.qnt seed=1 action=doSaveSucceeds#step:doSaveSucceeds | pass | covered |
| cleanroom-input/qnt/battle-runtime/rule-core-features.mbt.qnt#step:doActionSurgeActivate | tasks/target-replay-evidence/L15-RR04D-rule-core-feature-profile-components.json#L15-RR04D-RULE-CORE-FEATURE-PROFILE-COMPONENTS replay action=doActionSurgeActivate#step:doActionSurgeActivate | pass | covered |
| cleanroom-input/qnt/battle-runtime/rule-core-features.mbt.qnt#step:doActionSurgeRejectTwice | tasks/target-replay-evidence/L15-RR04D-rule-core-feature-profile-components.json#L15-RR04D-RULE-CORE-FEATURE-PROFILE-COMPONENTS replay action=doActionSurgeRejectTwice#step:doActionSurgeRejectTwice | pass | covered |
| cleanroom-input/qnt/battle-runtime/rule-core-features.mbt.qnt#step:doActionSurgeSpendAttack | tasks/target-replay-evidence/L15-RR04D-rule-core-feature-profile-components.json#L15-RR04D-RULE-CORE-FEATURE-PROFILE-COMPONENTS replay action=doActionSurgeSpendAttack#step:doActionSurgeSpendAttack | pass | covered |
| cleanroom-input/qnt/battle-runtime/rule-core-features.mbt.qnt#step:doArcheryAttackRollBonus | tasks/target-replay-evidence/L15-RR04D-rule-core-feature-profile-components.json#L15-RR04D-RULE-CORE-FEATURE-PROFILE-COMPONENTS replay action=doArcheryAttackRollBonus#step:doArcheryAttackRollBonus | pass | covered |
| cleanroom-input/qnt/battle-runtime/rule-core-features.mbt.qnt#step:doCunningDash | tasks/target-replay-evidence/L15-RR04D-rule-core-feature-profile-components.json#L15-RR04D-RULE-CORE-FEATURE-PROFILE-COMPONENTS replay action=doCunningDash#step:doCunningDash | pass | covered |
| cleanroom-input/qnt/battle-runtime/rule-core-features.mbt.qnt#step:doCunningDisengage | tasks/target-replay-evidence/L15-RR04D-rule-core-feature-profile-components.json#L15-RR04D-RULE-CORE-FEATURE-PROFILE-COMPONENTS replay action=doCunningDisengage#step:doCunningDisengage | pass | covered |
| cleanroom-input/qnt/battle-runtime/rule-core-features.mbt.qnt#step:doCunningHide | tasks/target-replay-evidence/L15-RR04D-rule-core-feature-profile-components.json#L15-RR04D-RULE-CORE-FEATURE-PROFILE-COMPONENTS replay action=doCunningHide#step:doCunningHide | pass | covered |
| cleanroom-input/qnt/battle-runtime/rule-core-features.mbt.qnt#step:doCuttingWordsDamage | tasks/target-replay-evidence/L15-RR04D-rule-core-feature-profile-components.json#L15-RR04D-RULE-CORE-FEATURE-PROFILE-COMPONENTS replay action=doCuttingWordsDamage#step:doCuttingWordsDamage | pass | covered |
| cleanroom-input/qnt/battle-runtime/rule-core-features.mbt.qnt#step:doDefenseArmorClass | tasks/target-replay-evidence/L15-RR04D-rule-core-feature-profile-components.json#L15-RR04D-RULE-CORE-FEATURE-PROFILE-COMPONENTS replay action=doDefenseArmorClass#step:doDefenseArmorClass | pass | covered |
| cleanroom-input/qnt/battle-runtime/rule-core-features.mbt.qnt#step:doDeflectAttacksDamageReduction | tasks/target-replay-evidence/L15-RR04D-rule-core-feature-profile-components.json#L15-RR04D-RULE-CORE-FEATURE-PROFILE-COMPONENTS replay action=doDeflectAttacksDamageReduction#step:doDeflectAttacksDamageReduction | pass | covered |
| cleanroom-input/qnt/battle-runtime/rule-core-features.mbt.qnt#step:doDiscoverSecondWind | tasks/target-replay-evidence/L15-RR04D-rule-core-feature-profile-components.json#L15-RR04D-RULE-CORE-FEATURE-PROFILE-COMPONENTS replay action=doDiscoverSecondWind#step:doDiscoverSecondWind | pass | covered |
| cleanroom-input/qnt/battle-runtime/rule-core-features.mbt.qnt#step:doFrenzy | tasks/target-replay-evidence/L15-RR04D-rule-core-feature-profile-components.json#L15-RR04D-RULE-CORE-FEATURE-PROFILE-COMPONENTS replay action=doFrenzy#step:doFrenzy | pass | covered |
| cleanroom-input/qnt/battle-runtime/rule-core-features.mbt.qnt#step:doImprovedCritical | tasks/target-replay-evidence/L15-RR04D-rule-core-feature-profile-components.json#L15-RR04D-RULE-CORE-FEATURE-PROFILE-COMPONENTS replay action=doImprovedCritical#step:doImprovedCritical | pass | covered |
| cleanroom-input/qnt/battle-runtime/rule-core-features.mbt.qnt#step:doRageActivateAndDamage | tasks/target-replay-evidence/L15-RR04D-rule-core-feature-profile-components.json#L15-RR04D-RULE-CORE-FEATURE-PROFILE-COMPONENTS replay action=doRageActivateAndDamage#step:doRageActivateAndDamage | pass | covered |
| cleanroom-input/qnt/battle-runtime/rule-core-features.mbt.qnt#step:doRecklessAttack | tasks/target-replay-evidence/L15-RR04D-rule-core-feature-profile-components.json#L15-RR04D-RULE-CORE-FEATURE-PROFILE-COMPONENTS replay action=doRecklessAttack#step:doRecklessAttack | pass | covered |
| cleanroom-input/qnt/battle-runtime/rule-core-features.mbt.qnt#step:doResolveSecondWindHigh | tasks/target-replay-evidence/L15-RR04D-rule-core-feature-profile-components.json#L15-RR04D-RULE-CORE-FEATURE-PROFILE-COMPONENTS replay action=doResolveSecondWindHigh#step:doResolveSecondWindHigh | pass | covered |
| cleanroom-input/qnt/battle-runtime/rule-core-features.mbt.qnt#step:doResolveSecondWindLow | tasks/target-replay-evidence/L15-RR04D-rule-core-feature-profile-components.json#L15-RR04D-RULE-CORE-FEATURE-PROFILE-COMPONENTS replay action=doResolveSecondWindLow#step:doResolveSecondWindLow | pass | covered |
| cleanroom-input/qnt/battle-runtime/rule-core-features.mbt.qnt#step:doSavageAttackerDamage | tasks/target-replay-evidence/L15-RR04D-rule-core-feature-profile-components.json#L15-RR04D-RULE-CORE-FEATURE-PROFILE-COMPONENTS replay action=doSavageAttackerDamage#step:doSavageAttackerDamage | pass | covered |
| cleanroom-input/qnt/battle-runtime/rule-core-features.mbt.qnt#step:doSneakAttack | tasks/target-replay-evidence/L15-RR04D-rule-core-feature-profile-components.json#L15-RR04D-RULE-CORE-FEATURE-PROFILE-COMPONENTS replay action=doSneakAttack#step:doSneakAttack | pass | covered |
| cleanroom-input/qnt/battle-runtime/rule-core-features.mbt.qnt#step:doTacticalMindConvertedSuccess | tasks/target-replay-evidence/L15-RR04D-rule-core-feature-profile-components.json#L15-RR04D-RULE-CORE-FEATURE-PROFILE-COMPONENTS replay action=doTacticalMindConvertedSuccess#step:doTacticalMindConvertedSuccess | pass | covered |
| cleanroom-input/qnt/battle-runtime/rule-core-features.mbt.qnt#step:doTacticalMindStillFailed | tasks/target-replay-evidence/L15-RR04D-rule-core-feature-profile-components.json#L15-RR04D-RULE-CORE-FEATURE-PROFILE-COMPONENTS replay action=doTacticalMindStillFailed#step:doTacticalMindStillFailed | pass | covered |
| cleanroom-input/qnt/battle-runtime/rule-core-features.mbt.qnt#step:doZeroHitPointReplacement | tasks/target-replay-evidence/L15-RR04D-rule-core-feature-profile-components.json#L15-RR04D-RULE-CORE-FEATURE-PROFILE-COMPONENTS replay action=doZeroHitPointReplacement#step:doZeroHitPointReplacement | pass | covered |
| cleanroom-input/qnt/battle-runtime/rule-core-ability-skill-command.mbt.qnt#step:doCommandCastGrovel | tasks/target-replay-evidence/L15-RR04C-RULE-CORE-SPELL-ABILITY-COMPONENTS-ability-skill-command.json#L15-RR04C-RULE-CORE-SPELL-ABILITY-COMPONENTS replay action=doCommandCastGrovel#step:doCommandCastGrovel | pass | covered |
| cleanroom-input/qnt/battle-runtime/rule-core-ability-skill-command.mbt.qnt#step:doCommandFollowApproachContinues | tasks/target-replay-evidence/L15-RR04C-RULE-CORE-SPELL-ABILITY-COMPONENTS-ability-skill-command.json#L15-RR04C-RULE-CORE-SPELL-ABILITY-COMPONENTS replay action=doCommandFollowApproachContinues#step:doCommandFollowApproachContinues | pass | covered |
| cleanroom-input/qnt/battle-runtime/rule-core-ability-skill-command.mbt.qnt#step:doCommandFollowApproachNoMovement | tasks/target-replay-evidence/L15-RR04C-RULE-CORE-SPELL-ABILITY-COMPONENTS-ability-skill-command.json#L15-RR04C-RULE-CORE-SPELL-ABILITY-COMPONENTS replay action=doCommandFollowApproachNoMovement#step:doCommandFollowApproachNoMovement | pass | covered |
| cleanroom-input/qnt/battle-runtime/rule-core-ability-skill-command.mbt.qnt#step:doCommandFollowApproachWithinFive | tasks/target-replay-evidence/L15-RR04C-RULE-CORE-SPELL-ABILITY-COMPONENTS-ability-skill-command.json#L15-RR04C-RULE-CORE-SPELL-ABILITY-COMPONENTS replay action=doCommandFollowApproachWithinFive#step:doCommandFollowApproachWithinFive | pass | covered |
| cleanroom-input/qnt/battle-runtime/rule-core-ability-skill-command.mbt.qnt#step:doCommandFollowDrop | tasks/target-replay-evidence/L15-RR04C-RULE-CORE-SPELL-ABILITY-COMPONENTS-ability-skill-command.json#L15-RR04C-RULE-CORE-SPELL-ABILITY-COMPONENTS replay action=doCommandFollowDrop#step:doCommandFollowDrop | pass | covered |
| cleanroom-input/qnt/battle-runtime/rule-core-ability-skill-command.mbt.qnt#step:doCommandFollowFlee | tasks/target-replay-evidence/L15-RR04C-RULE-CORE-SPELL-ABILITY-COMPONENTS-ability-skill-command.json#L15-RR04C-RULE-CORE-SPELL-ABILITY-COMPONENTS replay action=doCommandFollowFlee#step:doCommandFollowFlee | pass | covered |
| cleanroom-input/qnt/battle-runtime/rule-core-ability-skill-command.mbt.qnt#step:doCommandFollowFleeNoMovement | tasks/target-replay-evidence/L15-RR04C-RULE-CORE-SPELL-ABILITY-COMPONENTS-ability-skill-command.json#L15-RR04C-RULE-CORE-SPELL-ABILITY-COMPONENTS replay action=doCommandFollowFleeNoMovement#step:doCommandFollowFleeNoMovement | pass | covered |
| cleanroom-input/qnt/battle-runtime/rule-core-ability-skill-command.mbt.qnt#step:doCommandFollowFleeOpportunityAttack | tasks/target-replay-evidence/L15-RR04C-RULE-CORE-SPELL-ABILITY-COMPONENTS-ability-skill-command.json#L15-RR04C-RULE-CORE-SPELL-ABILITY-COMPONENTS replay action=doCommandFollowFleeOpportunityAttack#step:doCommandFollowFleeOpportunityAttack | pass | covered |
| cleanroom-input/qnt/battle-runtime/rule-core-ability-skill-command.mbt.qnt#step:doCommandFollowFleePartialRejected | tasks/target-replay-evidence/L15-RR04C-RULE-CORE-SPELL-ABILITY-COMPONENTS-ability-skill-command.json#L15-RR04C-RULE-CORE-SPELL-ABILITY-COMPONENTS replay action=doCommandFollowFleePartialRejected#step:doCommandFollowFleePartialRejected | pass | covered |
| cleanroom-input/qnt/battle-runtime/rule-core-ability-skill-command.mbt.qnt#step:doCommandFollowGrovel | tasks/target-replay-evidence/L15-RR04C-RULE-CORE-SPELL-ABILITY-COMPONENTS-ability-skill-command.json#L15-RR04C-RULE-CORE-SPELL-ABILITY-COMPONENTS replay action=doCommandFollowGrovel#step:doCommandFollowGrovel | pass | covered |
| cleanroom-input/qnt/battle-runtime/rule-core-ability-skill-command.mbt.qnt#step:doCommandHaltSuppresses | tasks/target-replay-evidence/L15-RR04C-RULE-CORE-SPELL-ABILITY-COMPONENTS-ability-skill-command.json#L15-RR04C-RULE-CORE-SPELL-ABILITY-COMPONENTS replay action=doCommandHaltSuppresses#step:doCommandHaltSuppresses | pass | covered |
| cleanroom-input/qnt/battle-runtime/rule-core-ability-skill-command.mbt.qnt#step:doEnhanceAbilityChoice | tasks/target-replay-evidence/L15-RR04C-RULE-CORE-SPELL-ABILITY-COMPONENTS-ability-skill-command.json#L15-RR04C-RULE-CORE-SPELL-ABILITY-COMPONENTS replay action=doEnhanceAbilityChoice#step:doEnhanceAbilityChoice | pass | covered |
| cleanroom-input/qnt/battle-runtime/rule-core-ability-skill-command.mbt.qnt#step:doGuidanceSkillAcrobatics | tasks/target-replay-evidence/L15-RR04C-RULE-CORE-SPELL-ABILITY-COMPONENTS-ability-skill-command.json#L15-RR04C-RULE-CORE-SPELL-ABILITY-COMPONENTS replay action=doGuidanceSkillAcrobatics#step:doGuidanceSkillAcrobatics | pass | covered |
| cleanroom-input/qnt/battle-runtime/rule-core-ability-skill-command.mbt.qnt#step:doGuidanceSkillAnimalHandling | tasks/target-replay-evidence/L15-RR04C-RULE-CORE-SPELL-ABILITY-COMPONENTS-ability-skill-command.json#L15-RR04C-RULE-CORE-SPELL-ABILITY-COMPONENTS replay action=doGuidanceSkillAnimalHandling#step:doGuidanceSkillAnimalHandling | pass | covered |
| cleanroom-input/qnt/battle-runtime/rule-core-ability-skill-command.mbt.qnt#step:doGuidanceSkillArcana | tasks/target-replay-evidence/L15-RR04C-RULE-CORE-SPELL-ABILITY-COMPONENTS-ability-skill-command.json#L15-RR04C-RULE-CORE-SPELL-ABILITY-COMPONENTS replay action=doGuidanceSkillArcana#step:doGuidanceSkillArcana | pass | covered |
| cleanroom-input/qnt/battle-runtime/rule-core-ability-skill-command.mbt.qnt#step:doGuidanceSkillAthletics | tasks/target-replay-evidence/L15-RR04C-RULE-CORE-SPELL-ABILITY-COMPONENTS-ability-skill-command.json#L15-RR04C-RULE-CORE-SPELL-ABILITY-COMPONENTS replay action=doGuidanceSkillAthletics#step:doGuidanceSkillAthletics | pass | covered |
| cleanroom-input/qnt/battle-runtime/rule-core-ability-skill-command.mbt.qnt#step:doGuidanceSkillDeception | tasks/target-replay-evidence/L15-RR04C-RULE-CORE-SPELL-ABILITY-COMPONENTS-ability-skill-command.json#L15-RR04C-RULE-CORE-SPELL-ABILITY-COMPONENTS replay action=doGuidanceSkillDeception#step:doGuidanceSkillDeception | pass | covered |
| cleanroom-input/qnt/battle-runtime/rule-core-ability-skill-command.mbt.qnt#step:doGuidanceSkillHistory | tasks/target-replay-evidence/L15-RR04C-RULE-CORE-SPELL-ABILITY-COMPONENTS-ability-skill-command.json#L15-RR04C-RULE-CORE-SPELL-ABILITY-COMPONENTS replay action=doGuidanceSkillHistory#step:doGuidanceSkillHistory | pass | covered |
| cleanroom-input/qnt/battle-runtime/rule-core-ability-skill-command.mbt.qnt#step:doGuidanceSkillInsight | tasks/target-replay-evidence/L15-RR04C-RULE-CORE-SPELL-ABILITY-COMPONENTS-ability-skill-command.json#L15-RR04C-RULE-CORE-SPELL-ABILITY-COMPONENTS replay action=doGuidanceSkillInsight#step:doGuidanceSkillInsight | pass | covered |
| cleanroom-input/qnt/battle-runtime/rule-core-ability-skill-command.mbt.qnt#step:doGuidanceSkillIntimidation | tasks/target-replay-evidence/L15-RR04C-RULE-CORE-SPELL-ABILITY-COMPONENTS-ability-skill-command.json#L15-RR04C-RULE-CORE-SPELL-ABILITY-COMPONENTS replay action=doGuidanceSkillIntimidation#step:doGuidanceSkillIntimidation | pass | covered |
| cleanroom-input/qnt/battle-runtime/rule-core-ability-skill-command.mbt.qnt#step:doGuidanceSkillInvestigation | tasks/target-replay-evidence/L15-RR04C-RULE-CORE-SPELL-ABILITY-COMPONENTS-ability-skill-command.json#L15-RR04C-RULE-CORE-SPELL-ABILITY-COMPONENTS replay action=doGuidanceSkillInvestigation#step:doGuidanceSkillInvestigation | pass | covered |
| cleanroom-input/qnt/battle-runtime/rule-core-ability-skill-command.mbt.qnt#step:doGuidanceSkillMedicine | tasks/target-replay-evidence/L15-RR04C-RULE-CORE-SPELL-ABILITY-COMPONENTS-ability-skill-command.json#L15-RR04C-RULE-CORE-SPELL-ABILITY-COMPONENTS replay action=doGuidanceSkillMedicine#step:doGuidanceSkillMedicine | pass | covered |
| cleanroom-input/qnt/battle-runtime/rule-core-ability-skill-command.mbt.qnt#step:doGuidanceSkillNature | tasks/target-replay-evidence/L15-RR04C-RULE-CORE-SPELL-ABILITY-COMPONENTS-ability-skill-command.json#L15-RR04C-RULE-CORE-SPELL-ABILITY-COMPONENTS replay action=doGuidanceSkillNature#step:doGuidanceSkillNature | pass | covered |
| cleanroom-input/qnt/battle-runtime/rule-core-ability-skill-command.mbt.qnt#step:doGuidanceSkillPerception | tasks/target-replay-evidence/L15-RR04C-RULE-CORE-SPELL-ABILITY-COMPONENTS-ability-skill-command.json#L15-RR04C-RULE-CORE-SPELL-ABILITY-COMPONENTS replay action=doGuidanceSkillPerception#step:doGuidanceSkillPerception | pass | covered |
| cleanroom-input/qnt/battle-runtime/rule-core-ability-skill-command.mbt.qnt#step:doGuidanceSkillPerformance | tasks/target-replay-evidence/L15-RR04C-RULE-CORE-SPELL-ABILITY-COMPONENTS-ability-skill-command.json#L15-RR04C-RULE-CORE-SPELL-ABILITY-COMPONENTS replay action=doGuidanceSkillPerformance#step:doGuidanceSkillPerformance | pass | covered |
| cleanroom-input/qnt/battle-runtime/rule-core-ability-skill-command.mbt.qnt#step:doGuidanceSkillPersuasion | tasks/target-replay-evidence/L15-RR04C-RULE-CORE-SPELL-ABILITY-COMPONENTS-ability-skill-command.json#L15-RR04C-RULE-CORE-SPELL-ABILITY-COMPONENTS replay action=doGuidanceSkillPersuasion#step:doGuidanceSkillPersuasion | pass | covered |
| cleanroom-input/qnt/battle-runtime/rule-core-ability-skill-command.mbt.qnt#step:doGuidanceSkillReligion | tasks/target-replay-evidence/L15-RR04C-RULE-CORE-SPELL-ABILITY-COMPONENTS-ability-skill-command.json#L15-RR04C-RULE-CORE-SPELL-ABILITY-COMPONENTS replay action=doGuidanceSkillReligion#step:doGuidanceSkillReligion | pass | covered |
| cleanroom-input/qnt/battle-runtime/rule-core-ability-skill-command.mbt.qnt#step:doGuidanceSkillSleightOfHand | tasks/target-replay-evidence/L15-RR04C-RULE-CORE-SPELL-ABILITY-COMPONENTS-ability-skill-command.json#L15-RR04C-RULE-CORE-SPELL-ABILITY-COMPONENTS replay action=doGuidanceSkillSleightOfHand#step:doGuidanceSkillSleightOfHand | pass | covered |
| cleanroom-input/qnt/battle-runtime/rule-core-ability-skill-command.mbt.qnt#step:doGuidanceSkillStealth | tasks/target-replay-evidence/L15-RR04C-RULE-CORE-SPELL-ABILITY-COMPONENTS-ability-skill-command.json#L15-RR04C-RULE-CORE-SPELL-ABILITY-COMPONENTS replay action=doGuidanceSkillStealth#step:doGuidanceSkillStealth | pass | covered |
| cleanroom-input/qnt/battle-runtime/rule-core-ability-skill-command.mbt.qnt#step:doGuidanceSkillSurvival | tasks/target-replay-evidence/L15-RR04C-RULE-CORE-SPELL-ABILITY-COMPONENTS-ability-skill-command.json#L15-RR04C-RULE-CORE-SPELL-ABILITY-COMPONENTS replay action=doGuidanceSkillSurvival#step:doGuidanceSkillSurvival | pass | covered |
| cleanroom-input/qnt/battle-runtime/rule-core-ability-skill-command.mbt.qnt#step:doSearchFails | tasks/target-replay-evidence/L15-RR04C-RULE-CORE-SPELL-ABILITY-COMPONENTS-ability-skill-command.json#L15-RR04C-RULE-CORE-SPELL-ABILITY-COMPONENTS replay action=doSearchFails#step:doSearchFails | pass | covered |
| cleanroom-input/qnt/battle-runtime/rule-core-ability-skill-command.mbt.qnt#step:doSearchSucceeds | tasks/target-replay-evidence/L15-RR04C-RULE-CORE-SPELL-ABILITY-COMPONENTS-ability-skill-command.json#L15-RR04C-RULE-CORE-SPELL-ABILITY-COMPONENTS replay action=doSearchSucceeds#step:doSearchSucceeds | pass | covered |
| cleanroom-input/qnt/battle-runtime/rule-core-spells.mbt.qnt#step:doAcidSplashAllSuccess | tasks/target-replay-evidence/L15-RR04C-RULE-CORE-SPELL-ABILITY-COMPONENTS-spells.json#L15-RR04C-RULE-CORE-SPELL-ABILITY-COMPONENTS replay action=doAcidSplashAllSuccess#step:doAcidSplashAllSuccess | pass | covered |
| cleanroom-input/qnt/battle-runtime/rule-core-spells.mbt.qnt#step:doAcidSplashNeedsDamageRoll | tasks/target-replay-evidence/L15-RR04C-RULE-CORE-SPELL-ABILITY-COMPONENTS-spells.json#L15-RR04C-RULE-CORE-SPELL-ABILITY-COMPONENTS replay action=doAcidSplashNeedsDamageRoll#step:doAcidSplashNeedsDamageRoll | pass | covered |
| cleanroom-input/qnt/battle-runtime/rule-core-spells.mbt.qnt#step:doAcidSplashNeedsSavingThrow | tasks/target-replay-evidence/L15-RR04C-RULE-CORE-SPELL-ABILITY-COMPONENTS-spells.json#L15-RR04C-RULE-CORE-SPELL-ABILITY-COMPONENTS replay action=doAcidSplashNeedsSavingThrow#step:doAcidSplashNeedsSavingThrow | pass | covered |
| cleanroom-input/qnt/battle-runtime/rule-core-spells.mbt.qnt#step:doAcidSplashOneFail | tasks/target-replay-evidence/L15-RR04C-RULE-CORE-SPELL-ABILITY-COMPONENTS-spells.json#L15-RR04C-RULE-CORE-SPELL-ABILITY-COMPONENTS replay action=doAcidSplashOneFail#step:doAcidSplashOneFail | pass | covered |
| cleanroom-input/qnt/battle-runtime/rule-core-spells.mbt.qnt#step:doCureWoundsNeedsHealingRoll | tasks/target-replay-evidence/L15-RR04C-RULE-CORE-SPELL-ABILITY-COMPONENTS-spells.json#L15-RR04C-RULE-CORE-SPELL-ABILITY-COMPONENTS replay action=doCureWoundsNeedsHealingRoll#step:doCureWoundsNeedsHealingRoll | pass | covered |
| cleanroom-input/qnt/battle-runtime/rule-core-spells.mbt.qnt#step:doCureWoundsNeedsTarget | tasks/target-replay-evidence/L15-RR04C-RULE-CORE-SPELL-ABILITY-COMPONENTS-spells.json#L15-RR04C-RULE-CORE-SPELL-ABILITY-COMPONENTS replay action=doCureWoundsNeedsTarget#step:doCureWoundsNeedsTarget | pass | covered |
| cleanroom-input/qnt/battle-runtime/rule-core-spells.mbt.qnt#step:doCureWoundsWounded | tasks/target-replay-evidence/L15-RR04C-RULE-CORE-SPELL-ABILITY-COMPONENTS-spells.json#L15-RR04C-RULE-CORE-SPELL-ABILITY-COMPONENTS replay action=doCureWoundsWounded#step:doCureWoundsWounded | pass | covered |
| cleanroom-input/qnt/battle-runtime/rule-core-spells.mbt.qnt#step:doHealingWordNeedsHealingRoll | tasks/target-replay-evidence/L15-RR04C-RULE-CORE-SPELL-ABILITY-COMPONENTS-spells.json#L15-RR04C-RULE-CORE-SPELL-ABILITY-COMPONENTS replay action=doHealingWordNeedsHealingRoll#step:doHealingWordNeedsHealingRoll | pass | covered |
| cleanroom-input/qnt/battle-runtime/rule-core-spells.mbt.qnt#step:doHealingWordNeedsTarget | tasks/target-replay-evidence/L15-RR04C-RULE-CORE-SPELL-ABILITY-COMPONENTS-spells.json#L15-RR04C-RULE-CORE-SPELL-ABILITY-COMPONENTS replay action=doHealingWordNeedsTarget#step:doHealingWordNeedsTarget | pass | covered |
| cleanroom-input/qnt/battle-runtime/rule-core-spells.mbt.qnt#step:doHealingWordWounded | tasks/target-replay-evidence/L15-RR04C-RULE-CORE-SPELL-ABILITY-COMPONENTS-spells.json#L15-RR04C-RULE-CORE-SPELL-ABILITY-COMPONENTS replay action=doHealingWordWounded#step:doHealingWordWounded | pass | covered |
| cleanroom-input/qnt/battle-runtime/rule-core-spells.mbt.qnt#step:doHealingWordZeroHp | tasks/target-replay-evidence/L15-RR04C-RULE-CORE-SPELL-ABILITY-COMPONENTS-spells.json#L15-RR04C-RULE-CORE-SPELL-ABILITY-COMPONENTS replay action=doHealingWordZeroHp#step:doHealingWordZeroHp | pass | covered |
| cleanroom-input/qnt/battle-runtime/rule-core-spells.mbt.qnt#step:doMageArmor | tasks/target-replay-evidence/L15-RR04C-RULE-CORE-SPELL-ABILITY-COMPONENTS-spells.json#L15-RR04C-RULE-CORE-SPELL-ABILITY-COMPONENTS replay action=doMageArmor#step:doMageArmor | pass | covered |
| cleanroom-input/qnt/battle-runtime/rule-core-spells.mbt.qnt#step:doMageArmorNeedsTarget | tasks/target-replay-evidence/L15-RR04C-RULE-CORE-SPELL-ABILITY-COMPONENTS-spells.json#L15-RR04C-RULE-CORE-SPELL-ABILITY-COMPONENTS replay action=doMageArmorNeedsTarget#step:doMageArmorNeedsTarget | pass | covered |
| cleanroom-input/qnt/battle-runtime/rule-core-spells.mbt.qnt#step:doMagicMissileLow | tasks/target-replay-evidence/L15-RR04C-RULE-CORE-SPELL-ABILITY-COMPONENTS-spells.json#L15-RR04C-RULE-CORE-SPELL-ABILITY-COMPONENTS replay action=doMagicMissileLow#step:doMagicMissileLow | pass | covered |
| cleanroom-input/qnt/battle-runtime/rule-core-spells.mbt.qnt#step:doMagicMissileNeedsAllocation | tasks/target-replay-evidence/L15-RR04C-RULE-CORE-SPELL-ABILITY-COMPONENTS-spells.json#L15-RR04C-RULE-CORE-SPELL-ABILITY-COMPONENTS replay action=doMagicMissileNeedsAllocation#step:doMagicMissileNeedsAllocation | pass | covered |
| cleanroom-input/qnt/battle-runtime/rule-core-spells.mbt.qnt#step:doRayOfFrostCritical | tasks/target-replay-evidence/L15-RR04C-RULE-CORE-SPELL-ABILITY-COMPONENTS-spells.json#L15-RR04C-RULE-CORE-SPELL-ABILITY-COMPONENTS replay action=doRayOfFrostCritical#step:doRayOfFrostCritical | pass | covered |
| cleanroom-input/qnt/battle-runtime/rule-core-spells.mbt.qnt#step:doRayOfFrostHit | tasks/target-replay-evidence/L15-RR04C-RULE-CORE-SPELL-ABILITY-COMPONENTS-spells.json#L15-RR04C-RULE-CORE-SPELL-ABILITY-COMPONENTS replay action=doRayOfFrostHit#step:doRayOfFrostHit | pass | covered |
| cleanroom-input/qnt/battle-runtime/rule-core-spells.mbt.qnt#step:doRayOfFrostMiss | tasks/target-replay-evidence/L15-RR04C-RULE-CORE-SPELL-ABILITY-COMPONENTS-spells.json#L15-RR04C-RULE-CORE-SPELL-ABILITY-COMPONENTS replay action=doRayOfFrostMiss#step:doRayOfFrostMiss | pass | covered |
| cleanroom-input/qnt/battle-runtime/rule-core-spells.mbt.qnt#step:doRayOfFrostNeedsAttackRoll | tasks/target-replay-evidence/L15-RR04C-RULE-CORE-SPELL-ABILITY-COMPONENTS-spells.json#L15-RR04C-RULE-CORE-SPELL-ABILITY-COMPONENTS replay action=doRayOfFrostNeedsAttackRoll#step:doRayOfFrostNeedsAttackRoll | pass | covered |
| cleanroom-input/qnt/battle-runtime/rule-core-spells.mbt.qnt#step:doRayOfFrostNeedsDamageRoll | tasks/target-replay-evidence/L15-RR04C-RULE-CORE-SPELL-ABILITY-COMPONENTS-spells.json#L15-RR04C-RULE-CORE-SPELL-ABILITY-COMPONENTS replay action=doRayOfFrostNeedsDamageRoll#step:doRayOfFrostNeedsDamageRoll | pass | covered |
| cleanroom-input/qnt/battle-runtime/rule-core-spells.mbt.qnt#step:doRayOfFrostNeedsTarget | tasks/target-replay-evidence/L15-RR04C-RULE-CORE-SPELL-ABILITY-COMPONENTS-spells.json#L15-RR04C-RULE-CORE-SPELL-ABILITY-COMPONENTS replay action=doRayOfFrostNeedsTarget#step:doRayOfFrostNeedsTarget | pass | covered |
| cleanroom-input/qnt/battle-runtime/rule-core-spells.mbt.qnt#step:doReadySpellHold | tasks/target-replay-evidence/L15-RR04C-RULE-CORE-SPELL-ABILITY-COMPONENTS-spells.json#L15-RR04C-RULE-CORE-SPELL-ABILITY-COMPONENTS replay action=doReadySpellHold#step:doReadySpellHold | pass | covered |
| cleanroom-input/qnt/battle-runtime/rule-core-spells.mbt.qnt#step:doRejectSecondSlotSpell | tasks/target-replay-evidence/L15-RR04C-RULE-CORE-SPELL-ABILITY-COMPONENTS-spells.json#L15-RR04C-RULE-CORE-SPELL-ABILITY-COMPONENTS replay action=doRejectSecondSlotSpell#step:doRejectSecondSlotSpell | pass | covered |
| cleanroom-input/qnt/battle-runtime/rule-core-spells.mbt.qnt#step:doReleaseReadiedSpell | tasks/target-replay-evidence/L15-RR04C-RULE-CORE-SPELL-ABILITY-COMPONENTS-spells.json#L15-RR04C-RULE-CORE-SPELL-ABILITY-COMPONENTS replay action=doReleaseReadiedSpell#step:doReleaseReadiedSpell | pass | covered |
| cleanroom-input/qnt/character-creation-runtime/character-creation-class-feature-projections.mbt.qnt#step:doProjectMonkFocusAndUncannyMetabolism | tasks/target-replay-evidence/L15-RR08-character-creation-routes.json#L15-RR08-CHARACTER-CREATION-ROUTES replay action=doProjectMonkFocusAndUncannyMetabolism#step:doProjectMonkFocusAndUncannyMetabolism | pass | covered |
| cleanroom-input/qnt/character-creation-runtime/character-creation-class-feature-projections.mbt.qnt#step:doProjectSorcererFontAndMetamagic | tasks/target-replay-evidence/L15-RR08-character-creation-routes.json#L15-RR08-CHARACTER-CREATION-ROUTES replay action=doProjectSorcererFontAndMetamagic#step:doProjectSorcererFontAndMetamagic | pass | covered |
| cleanroom-input/qnt/character-creation-runtime/character-creation-class-feature-selected-identity.mbt.qnt#step:doProjectClericChannelDivinity | tasks/target-replay-evidence/L15-RR08-character-creation-routes.json#L15-RR08-CHARACTER-CREATION-ROUTES replay action=doProjectClericChannelDivinity#step:doProjectClericChannelDivinity | pass | covered |
| cleanroom-input/qnt/character-creation-runtime/character-creation-class-feature-selected-identity.mbt.qnt#step:doProjectDruidWildCompanion | tasks/target-replay-evidence/L15-RR08-character-creation-routes.json#L15-RR08-CHARACTER-CREATION-ROUTES replay action=doProjectDruidWildCompanion#step:doProjectDruidWildCompanion | pass | covered |
| cleanroom-input/qnt/character-creation-runtime/character-creation-class-feature-selected-identity.mbt.qnt#step:doProjectDruidWildShape | tasks/target-replay-evidence/L15-RR08-character-creation-routes.json#L15-RR08-CHARACTER-CREATION-ROUTES replay action=doProjectDruidWildShape#step:doProjectDruidWildShape | pass | covered |
| cleanroom-input/qnt/character-creation-runtime/character-creation-class-feature-selected-identity.mbt.qnt#step:doProjectMonksFocus | tasks/target-replay-evidence/L15-RR08-character-creation-routes.json#L15-RR08-CHARACTER-CREATION-ROUTES replay action=doProjectMonksFocus#step:doProjectMonksFocus | pass | covered |
| cleanroom-input/qnt/character-creation-runtime/character-creation-class-feature-selected-identity.mbt.qnt#step:doProjectMonkUncannyMetabolism | tasks/target-replay-evidence/L15-RR08-character-creation-routes.json#L15-RR08-CHARACTER-CREATION-ROUTES replay action=doProjectMonkUncannyMetabolism#step:doProjectMonkUncannyMetabolism | pass | covered |
| cleanroom-input/qnt/character-creation-runtime/character-creation-class-feature-selected-identity.mbt.qnt#step:doProjectWarlockPactMagic | tasks/target-replay-evidence/L15-RR08-character-creation-routes.json#L15-RR08-CHARACTER-CREATION-ROUTES replay action=doProjectWarlockPactMagic#step:doProjectWarlockPactMagic | pass | covered |
| cleanroom-input/qnt/character-creation-runtime/character-creation-class-feature-selected-identity.mbt.qnt#step:doSelectBardExpertise | tasks/target-replay-evidence/L15-RR08-character-creation-routes.json#L15-RR08-CHARACTER-CREATION-ROUTES replay action=doSelectBardExpertise#step:doSelectBardExpertise | pass | covered |
| cleanroom-input/qnt/character-creation-runtime/character-creation-class-feature-selected-identity.mbt.qnt#step:doSelectPaladinFightingStyle | tasks/target-replay-evidence/L15-RR08-character-creation-routes.json#L15-RR08-CHARACTER-CREATION-ROUTES replay action=doSelectPaladinFightingStyle#step:doSelectPaladinFightingStyle | pass | covered |
| cleanroom-input/qnt/character-creation-runtime/character-creation-class-feature-selected-identity.mbt.qnt#step:doSelectRangerDeftExplorer | tasks/target-replay-evidence/L15-RR08-character-creation-routes.json#L15-RR08-CHARACTER-CREATION-ROUTES replay action=doSelectRangerDeftExplorer#step:doSelectRangerDeftExplorer | pass | covered |
| cleanroom-input/qnt/character-creation-runtime/character-creation-class-feature-selected-identity.mbt.qnt#step:doSelectRangerFightingStyle | tasks/target-replay-evidence/L15-RR08-character-creation-routes.json#L15-RR08-CHARACTER-CREATION-ROUTES replay action=doSelectRangerFightingStyle#step:doSelectRangerFightingStyle | pass | covered |
| cleanroom-input/qnt/character-creation-runtime/character-creation-class-feature-selected-identity.mbt.qnt#step:doSelectWizardEvocationSavant | tasks/target-replay-evidence/L15-RR08-character-creation-routes.json#L15-RR08-CHARACTER-CREATION-ROUTES replay action=doSelectWizardEvocationSavant#step:doSelectWizardEvocationSavant | pass | covered |
| cleanroom-input/qnt/character-creation-runtime/character-creation-class-feature-selected-identity.mbt.qnt#step:doSelectWizardScholar | tasks/target-replay-evidence/L15-RR08-character-creation-routes.json#L15-RR08-CHARACTER-CREATION-ROUTES replay action=doSelectWizardScholar#step:doSelectWizardScholar | pass | covered |
| cleanroom-input/qnt/character-creation-runtime/character-creation-cleric-druid-order-selected-identity.mbt.qnt#step:doSelectClericProtectorOrder | tasks/target-replay-evidence/L15-RR08-character-creation-routes.json#L15-RR08-CHARACTER-CREATION-ROUTES replay action=doSelectClericProtectorOrder#step:doSelectClericProtectorOrder | pass | covered |
| cleanroom-input/qnt/character-creation-runtime/character-creation-cleric-druid-order-selected-identity.mbt.qnt#step:doSelectClericThaumaturgeOrder | tasks/target-replay-evidence/L15-RR08-character-creation-routes.json#L15-RR08-CHARACTER-CREATION-ROUTES replay action=doSelectClericThaumaturgeOrder#step:doSelectClericThaumaturgeOrder | pass | covered |
| cleanroom-input/qnt/character-creation-runtime/character-creation-cleric-druid-order-selected-identity.mbt.qnt#step:doSelectDruidMagicianOrder | tasks/target-replay-evidence/L15-RR08-character-creation-routes.json#L15-RR08-CHARACTER-CREATION-ROUTES replay action=doSelectDruidMagicianOrder#step:doSelectDruidMagicianOrder | pass | covered |
| cleanroom-input/qnt/character-creation-runtime/character-creation-cleric-druid-order-selected-identity.mbt.qnt#step:doSelectDruidWardenOrder | tasks/target-replay-evidence/L15-RR08-character-creation-routes.json#L15-RR08-CHARACTER-CREATION-ROUTES replay action=doSelectDruidWardenOrder#step:doSelectDruidWardenOrder | pass | covered |
| cleanroom-input/qnt/character-creation-runtime/character-creation-fighter-fighting-style-selected-identity.mbt.qnt#step:doReplaceArcheryWithDefenseOnFighterLevelGain | tasks/target-replay-evidence/L15-RR08-character-creation-routes.json#L15-RR08-CHARACTER-CREATION-ROUTES replay action=doReplaceArcheryWithDefenseOnFighterLevelGain#step:doReplaceArcheryWithDefenseOnFighterLevelGain | pass | covered |
| cleanroom-input/qnt/character-creation-runtime/character-creation-fighter-fighting-style-selected-identity.mbt.qnt#step:doReplaceDefenseWithArcheryOnFighterLevelGain | tasks/target-replay-evidence/L15-RR08-character-creation-routes.json#L15-RR08-CHARACTER-CREATION-ROUTES replay action=doReplaceDefenseWithArcheryOnFighterLevelGain#step:doReplaceDefenseWithArcheryOnFighterLevelGain | pass | covered |
| cleanroom-input/qnt/character-creation-runtime/character-creation-fighter-fighting-style-selected-identity.mbt.qnt#step:doReplaceDefenseWithGreatWeaponFightingOnFighterLevelGain | tasks/target-replay-evidence/L15-RR08-character-creation-routes.json#L15-RR08-CHARACTER-CREATION-ROUTES replay action=doReplaceDefenseWithGreatWeaponFightingOnFighterLevelGain#step:doReplaceDefenseWithGreatWeaponFightingOnFighterLevelGain | pass | covered |
| cleanroom-input/qnt/character-creation-runtime/character-creation-fighter-fighting-style-selected-identity.mbt.qnt#step:doReplaceDefenseWithTwoWeaponFightingOnFighterLevelGain | tasks/target-replay-evidence/L15-RR08-character-creation-routes.json#L15-RR08-CHARACTER-CREATION-ROUTES replay action=doReplaceDefenseWithTwoWeaponFightingOnFighterLevelGain#step:doReplaceDefenseWithTwoWeaponFightingOnFighterLevelGain | pass | covered |
| cleanroom-input/qnt/character-creation-runtime/character-creation-fighter-fighting-style-selected-identity.mbt.qnt#step:doSelectArcheryFightingStyle | tasks/target-replay-evidence/L15-RR08-character-creation-routes.json#L15-RR08-CHARACTER-CREATION-ROUTES replay action=doSelectArcheryFightingStyle#step:doSelectArcheryFightingStyle | pass | covered |
| cleanroom-input/qnt/character-creation-runtime/character-creation-fighter-fighting-style-selected-identity.mbt.qnt#step:doSelectDefenseFightingStyle | tasks/target-replay-evidence/L15-RR08-character-creation-routes.json#L15-RR08-CHARACTER-CREATION-ROUTES replay action=doSelectDefenseFightingStyle#step:doSelectDefenseFightingStyle | pass | covered |
| cleanroom-input/qnt/character-creation-runtime/character-creation-fighter-fighting-style-selected-identity.mbt.qnt#step:doSelectGreatWeaponFightingStyle | tasks/target-replay-evidence/L15-RR08-character-creation-routes.json#L15-RR08-CHARACTER-CREATION-ROUTES replay action=doSelectGreatWeaponFightingStyle#step:doSelectGreatWeaponFightingStyle | pass | covered |
| cleanroom-input/qnt/character-creation-runtime/character-creation-fighter-fighting-style-selected-identity.mbt.qnt#step:doSelectTwoWeaponFightingStyle | tasks/target-replay-evidence/L15-RR08-character-creation-routes.json#L15-RR08-CHARACTER-CREATION-ROUTES replay action=doSelectTwoWeaponFightingStyle#step:doSelectTwoWeaponFightingStyle | pass | covered |
| cleanroom-input/qnt/character-creation-runtime/character-creation-rogue-expertise-selected-identity.mbt.qnt#step:doSelectLevelOneOwnedSkillExpertise | tasks/target-replay-evidence/L15-RR08-character-creation-routes.json#L15-RR08-CHARACTER-CREATION-ROUTES replay action=doSelectLevelOneOwnedSkillExpertise#step:doSelectLevelOneOwnedSkillExpertise | pass | covered |
| cleanroom-input/qnt/character-creation-runtime/character-creation-runtime.mbt.qnt#step:doFillAbilityScoresOnly | tasks/target-replay-evidence/L15-RR08-character-creation-routes.json#L15-RR08-CHARACTER-CREATION-ROUTES replay action=doFillAbilityScoresOnly#step:doFillAbilityScoresOnly | pass | covered |
| cleanroom-input/qnt/character-creation-runtime/character-creation-runtime.mbt.qnt#step:doFillInitialChoicesOnly | tasks/target-replay-evidence/L15-RR08-character-creation-routes.json#L15-RR08-CHARACTER-CREATION-ROUTES replay action=doFillInitialChoicesOnly#step:doFillInitialChoicesOnly | pass | covered |
| cleanroom-input/qnt/character-creation-runtime/character-creation-runtime.mbt.qnt#step:doFillInitialManifest | tasks/target-replay-evidence/L15-RR08-character-creation-routes.json#L15-RR08-CHARACTER-CREATION-ROUTES replay action=doFillInitialManifest#step:doFillInitialManifest | pass | covered |
| cleanroom-input/qnt/character-creation-runtime/character-creation-runtime.mbt.qnt#step:doFillManifestChoices | tasks/target-replay-evidence/L15-RR08-character-creation-routes.json#L15-RR08-CHARACTER-CREATION-ROUTES replay action=doFillManifestChoices#step:doFillManifestChoices | pass | covered |
| cleanroom-input/qnt/character-creation-runtime/character-creation-runtime.mbt.qnt#step:doFillManifestLoadout | tasks/target-replay-evidence/L15-RR08-character-creation-routes.json#L15-RR08-CHARACTER-CREATION-ROUTES replay action=doFillManifestLoadout#step:doFillManifestLoadout | pass | covered |
| cleanroom-input/qnt/character-creation-runtime/character-creation-runtime.mbt.qnt#step:doFillManifestPurchase | tasks/target-replay-evidence/L15-RR08-character-creation-routes.json#L15-RR08-CHARACTER-CREATION-ROUTES replay action=doFillManifestPurchase#step:doFillManifestPurchase | pass | covered |
| cleanroom-input/qnt/character-creation-runtime/character-creation-runtime.mbt.qnt#step:doRejectClosedInitialProgressionHole | tasks/target-replay-evidence/L15-RR08-character-creation-routes.json#L15-RR08-CHARACTER-CREATION-ROUTES replay action=doRejectClosedInitialProgressionHole#step:doRejectClosedInitialProgressionHole | pass | covered |
| cleanroom-input/qnt/character-creation-runtime/character-creation-runtime.mbt.qnt#step:doRejectDuplicateFill | tasks/target-replay-evidence/L15-RR08-character-creation-routes.json#L15-RR08-CHARACTER-CREATION-ROUTES replay action=doRejectDuplicateFill#step:doRejectDuplicateFill | pass | covered |
| cleanroom-input/qnt/character-creation-runtime/character-creation-runtime.mbt.qnt#step:doRejectDuplicateLanguage | tasks/target-replay-evidence/L15-RR08-character-creation-routes.json#L15-RR08-CHARACTER-CREATION-ROUTES replay action=doRejectDuplicateLanguage#step:doRejectDuplicateLanguage | pass | covered |
| cleanroom-input/qnt/character-creation-runtime/character-creation-runtime.mbt.qnt#step:doRejectStaleInitialManifest | tasks/target-replay-evidence/L15-RR08-character-creation-routes.json#L15-RR08-CHARACTER-CREATION-ROUTES replay action=doRejectStaleInitialManifest#step:doRejectStaleInitialManifest | pass | covered |
| cleanroom-input/qnt/character-creation-runtime/character-creation-runtime.mbt.qnt#step:doRejectTooFewLanguages | tasks/target-replay-evidence/L15-RR08-character-creation-routes.json#L15-RR08-CHARACTER-CREATION-ROUTES replay action=doRejectTooFewLanguages#step:doRejectTooFewLanguages | pass | covered |
| cleanroom-input/qnt/character-creation-runtime/character-creation-runtime.mbt.qnt#step:doRejectTooManyLanguages | tasks/target-replay-evidence/L15-RR08-character-creation-routes.json#L15-RR08-CHARACTER-CREATION-ROUTES replay action=doRejectTooManyLanguages#step:doRejectTooManyLanguages | pass | covered |
| cleanroom-input/qnt/character-creation-runtime/character-creation-runtime.mbt.qnt#step:doRejectUnknownLoadoutArmor | tasks/target-replay-evidence/L15-RR08-character-creation-routes.json#L15-RR08-CHARACTER-CREATION-ROUTES replay action=doRejectUnknownLoadoutArmor#step:doRejectUnknownLoadoutArmor | pass | covered |
| cleanroom-input/qnt/character-creation-runtime/character-creation-runtime.mbt.qnt#step:doRejectUnsupportedClassEquipment | tasks/target-replay-evidence/L15-RR08-character-creation-routes.json#L15-RR08-CHARACTER-CREATION-ROUTES replay action=doRejectUnsupportedClassEquipment#step:doRejectUnsupportedClassEquipment | pass | covered |
| cleanroom-input/qnt/character-creation-runtime/character-creation-runtime.mbt.qnt#step:doRejectUnsupportedLanguage | tasks/target-replay-evidence/L15-RR08-character-creation-routes.json#L15-RR08-CHARACTER-CREATION-ROUTES replay action=doRejectUnsupportedLanguage#step:doRejectUnsupportedLanguage | pass | covered |
| cleanroom-input/qnt/character-creation-runtime/character-creation-runtime.mbt.qnt#step:doRejectWrongKindPrimaryClass | tasks/target-replay-evidence/L15-RR08-character-creation-routes.json#L15-RR08-CHARACTER-CREATION-ROUTES replay action=doRejectWrongKindPrimaryClass#step:doRejectWrongKindPrimaryClass | pass | covered |
| cleanroom-input/qnt/character-creation-runtime/character-creation-warlock-eldritch-invocations-selected-identity.mbt.qnt#step:doGainLevelTwoInvocations | tasks/target-replay-evidence/L15-RR08-character-creation-routes.json#L15-RR08-CHARACTER-CREATION-ROUTES replay action=doGainLevelTwoInvocations#step:doGainLevelTwoInvocations | pass | covered |
| cleanroom-input/qnt/character-creation-runtime/character-creation-warlock-eldritch-invocations-selected-identity.mbt.qnt#step:doRejectDuplicateInvocationSelections | tasks/target-replay-evidence/L15-RR08-character-creation-routes.json#L15-RR08-CHARACTER-CREATION-ROUTES replay action=doRejectDuplicateInvocationSelections#step:doRejectDuplicateInvocationSelections | pass | covered |
| cleanroom-input/qnt/character-creation-runtime/character-creation-warlock-eldritch-invocations-selected-identity.mbt.qnt#step:doReplaceArmorWithEldritchMindOnWarlockLevelGain | tasks/target-replay-evidence/L15-RR08-character-creation-routes.json#L15-RR08-CHARACTER-CREATION-ROUTES replay action=doReplaceArmorWithEldritchMindOnWarlockLevelGain#step:doReplaceArmorWithEldritchMindOnWarlockLevelGain | pass | covered |
| cleanroom-input/qnt/character-creation-runtime/character-creation-warlock-eldritch-invocations-selected-identity.mbt.qnt#step:doReplaceRepeatableInvocationByChoice | tasks/target-replay-evidence/L15-RR08-character-creation-routes.json#L15-RR08-CHARACTER-CREATION-ROUTES replay action=doReplaceRepeatableInvocationByChoice#step:doReplaceRepeatableInvocationByChoice | pass | covered |
| cleanroom-input/qnt/character-creation-runtime/character-creation-warlock-eldritch-invocations-selected-identity.mbt.qnt#step:doSelectLevelOneArmorOfShadows | tasks/target-replay-evidence/L15-RR08-character-creation-routes.json#L15-RR08-CHARACTER-CREATION-ROUTES replay action=doSelectLevelOneArmorOfShadows#step:doSelectLevelOneArmorOfShadows | pass | covered |
| cleanroom-input/qnt/character-creation-runtime/character-creation-weapon-mastery-containers-selected-identity.mbt.qnt#step:doFinalizeBarbarianWeaponMastery | tasks/target-replay-evidence/L15-RR08-character-creation-routes.json#L15-RR08-CHARACTER-CREATION-ROUTES replay action=doFinalizeBarbarianWeaponMastery#step:doFinalizeBarbarianWeaponMastery | pass | covered |
| cleanroom-input/qnt/character-creation-runtime/character-creation-weapon-mastery-containers-selected-identity.mbt.qnt#step:doFinalizeFighterWeaponMastery | tasks/target-replay-evidence/L15-RR08-character-creation-routes.json#L15-RR08-CHARACTER-CREATION-ROUTES replay action=doFinalizeFighterWeaponMastery#step:doFinalizeFighterWeaponMastery | pass | covered |
| cleanroom-input/qnt/character-creation-runtime/character-creation-weapon-mastery-containers-selected-identity.mbt.qnt#step:doFinalizePaladinWeaponMastery | tasks/target-replay-evidence/L15-RR08-character-creation-routes.json#L15-RR08-CHARACTER-CREATION-ROUTES replay action=doFinalizePaladinWeaponMastery#step:doFinalizePaladinWeaponMastery | pass | covered |
| cleanroom-input/qnt/character-creation-runtime/character-creation-weapon-mastery-containers-selected-identity.mbt.qnt#step:doFinalizeRangerWeaponMastery | tasks/target-replay-evidence/L15-RR08-character-creation-routes.json#L15-RR08-CHARACTER-CREATION-ROUTES replay action=doFinalizeRangerWeaponMastery#step:doFinalizeRangerWeaponMastery | pass | covered |
| cleanroom-input/qnt/character-creation-runtime/character-creation-weapon-mastery-containers-selected-identity.mbt.qnt#step:doFinalizeRogueWeaponMastery | tasks/target-replay-evidence/L15-RR08-character-creation-routes.json#L15-RR08-CHARACTER-CREATION-ROUTES replay action=doFinalizeRogueWeaponMastery#step:doFinalizeRogueWeaponMastery | pass | covered |
| cleanroom-input/qnt/character-sheet-runtime/character-sheet-ability-check-proficiency-bonus.mbt.qnt#step:doProjectExpertise | tasks/target-replay-evidence/L15-RR09-character-sheet-routes.json#L15-RR09-CHARACTER-SHEET-ROUTES route action=doProjectExpertise#step:doProjectExpertise | pass | covered |
| cleanroom-input/qnt/character-sheet-runtime/character-sheet-ability-check-proficiency-bonus.mbt.qnt#step:doProjectJackOfAllTradesLevelTwo | tasks/target-replay-evidence/L15-RR09-character-sheet-routes.json#L15-RR09-CHARACTER-SHEET-ROUTES route action=doProjectJackOfAllTradesLevelTwo#step:doProjectJackOfAllTradesLevelTwo | pass | covered |
| cleanroom-input/qnt/character-sheet-runtime/character-sheet-ability-check-proficiency-bonus.mbt.qnt#step:doProjectJackOfAllTradesRoundedDown | tasks/target-replay-evidence/L15-RR09-character-sheet-routes.json#L15-RR09-CHARACTER-SHEET-ROUTES route action=doProjectJackOfAllTradesRoundedDown#step:doProjectJackOfAllTradesRoundedDown | pass | covered |
| cleanroom-input/qnt/character-sheet-runtime/character-sheet-ability-check-proficiency-bonus.mbt.qnt#step:doProjectSkillProficiency | tasks/target-replay-evidence/L15-RR09-character-sheet-routes.json#L15-RR09-CHARACTER-SHEET-ROUTES route action=doProjectSkillProficiency#step:doProjectSkillProficiency | pass | covered |
| cleanroom-input/qnt/character-sheet-runtime/character-sheet-ability-check-proficiency-bonus.mbt.qnt#step:doRejectMissingBardLevelTwo | tasks/target-replay-evidence/L15-RR09-character-sheet-routes.json#L15-RR09-CHARACTER-SHEET-ROUTES route action=doRejectMissingBardLevelTwo#step:doRejectMissingBardLevelTwo | pass | covered |
| cleanroom-input/qnt/character-sheet-runtime/character-sheet-ability-check-proficiency-bonus.mbt.qnt#step:doRejectOtherProficiencyBonus | tasks/target-replay-evidence/L15-RR09-character-sheet-routes.json#L15-RR09-CHARACTER-SHEET-ROUTES route action=doRejectOtherProficiencyBonus#step:doRejectOtherProficiencyBonus | pass | covered |
| cleanroom-input/qnt/character-sheet-runtime/character-sheet-arcane-recovery-selected-identity.mbt.qnt#step:doRecoverSecondLevelSpellSlot | tasks/target-replay-evidence/L15-RR09-character-sheet-routes.json#L15-RR09-CHARACTER-SHEET-ROUTES route action=doRecoverSecondLevelSpellSlot#step:doRecoverSecondLevelSpellSlot | pass | covered |
| cleanroom-input/qnt/character-sheet-runtime/character-sheet-arcane-recovery-selected-identity.mbt.qnt#step:doRejectPactSlotArcaneRecovery | tasks/target-replay-evidence/L15-RR09-character-sheet-routes.json#L15-RR09-CHARACTER-SHEET-ROUTES route action=doRejectPactSlotArcaneRecovery#step:doRejectPactSlotArcaneRecovery | pass | covered |
| cleanroom-input/qnt/character-sheet-runtime/character-sheet-arcane-recovery-selected-identity.mbt.qnt#step:doResetArcaneRecoveryOnLongRest | tasks/target-replay-evidence/L15-RR09-character-sheet-routes.json#L15-RR09-CHARACTER-SHEET-ROUTES route action=doResetArcaneRecoveryOnLongRest#step:doResetArcaneRecoveryOnLongRest | pass | covered |
| cleanroom-input/qnt/character-sheet-runtime/character-sheet-armor-class-base-selected-identity.mbt.qnt#step:doProjectHeavyArmorWithShield | tasks/target-replay-evidence/L15-RR09-character-sheet-routes.json#L15-RR09-CHARACTER-SHEET-ROUTES route action=doProjectHeavyArmorWithShield#step:doProjectHeavyArmorWithShield | pass | covered |
| cleanroom-input/qnt/character-sheet-runtime/character-sheet-armor-class-base-selected-identity.mbt.qnt#step:doProjectLightArmor | tasks/target-replay-evidence/L15-RR09-character-sheet-routes.json#L15-RR09-CHARACTER-SHEET-ROUTES route action=doProjectLightArmor#step:doProjectLightArmor | pass | covered |
| cleanroom-input/qnt/character-sheet-runtime/character-sheet-armor-class-base-selected-identity.mbt.qnt#step:doProjectMediumArmorDexCap | tasks/target-replay-evidence/L15-RR09-character-sheet-routes.json#L15-RR09-CHARACTER-SHEET-ROUTES route action=doProjectMediumArmorDexCap#step:doProjectMediumArmorDexCap | pass | covered |
| cleanroom-input/qnt/character-sheet-runtime/character-sheet-armor-class-base-selected-identity.mbt.qnt#step:doSelectBarbarianUnarmoredDefense | tasks/target-replay-evidence/L15-RR09-character-sheet-routes.json#L15-RR09-CHARACTER-SHEET-ROUTES route action=doSelectBarbarianUnarmoredDefense#step:doSelectBarbarianUnarmoredDefense | pass | covered |
| cleanroom-input/qnt/character-sheet-runtime/character-sheet-armor-class-base-selected-identity.mbt.qnt#step:doSelectBarbarianUnarmoredDefenseWithShield | tasks/target-replay-evidence/L15-RR09-character-sheet-routes.json#L15-RR09-CHARACTER-SHEET-ROUTES route action=doSelectBarbarianUnarmoredDefenseWithShield#step:doSelectBarbarianUnarmoredDefenseWithShield | pass | covered |
| cleanroom-input/qnt/character-sheet-runtime/character-sheet-armor-class-base-selected-identity.mbt.qnt#step:doSelectMonkUnarmoredDefense | tasks/target-replay-evidence/L15-RR09-character-sheet-routes.json#L15-RR09-CHARACTER-SHEET-ROUTES route action=doSelectMonkUnarmoredDefense#step:doSelectMonkUnarmoredDefense | pass | covered |
| cleanroom-input/qnt/character-sheet-runtime/character-sheet-class-feature-selected-identity.mbt.qnt#step:doProjectBardJackOfAllTrades | tasks/target-replay-evidence/L15-RR09-character-sheet-routes.json#L15-RR09-CHARACTER-SHEET-ROUTES route action=doProjectBardJackOfAllTrades#step:doProjectBardJackOfAllTrades | pass | covered |
| cleanroom-input/qnt/character-sheet-runtime/character-sheet-class-feature-selected-identity.mbt.qnt#step:doProjectClericLifeDomainSpells | tasks/target-replay-evidence/L15-RR09-character-sheet-routes.json#L15-RR09-CHARACTER-SHEET-ROUTES route action=doProjectClericLifeDomainSpells#step:doProjectClericLifeDomainSpells | pass | covered |
| cleanroom-input/qnt/character-sheet-runtime/character-sheet-class-feature-selected-identity.mbt.qnt#step:doProjectDruidCircleLandSpells | tasks/target-replay-evidence/L15-RR09-character-sheet-routes.json#L15-RR09-CHARACTER-SHEET-ROUTES route action=doProjectDruidCircleLandSpells#step:doProjectDruidCircleLandSpells | pass | covered |
| cleanroom-input/qnt/character-sheet-runtime/character-sheet-class-feature-selected-identity.mbt.qnt#step:doProjectPaladinOathDevotionSpells | tasks/target-replay-evidence/L15-RR09-character-sheet-routes.json#L15-RR09-CHARACTER-SHEET-ROUTES route action=doProjectPaladinOathDevotionSpells#step:doProjectPaladinOathDevotionSpells | pass | covered |
| cleanroom-input/qnt/character-sheet-runtime/character-sheet-class-feature-selected-identity.mbt.qnt#step:doProjectPaladinsSmite | tasks/target-replay-evidence/L15-RR09-character-sheet-routes.json#L15-RR09-CHARACTER-SHEET-ROUTES route action=doProjectPaladinsSmite#step:doProjectPaladinsSmite | pass | covered |
| cleanroom-input/qnt/character-sheet-runtime/character-sheet-class-feature-selected-identity.mbt.qnt#step:doProjectRangerFavoredEnemy | tasks/target-replay-evidence/L15-RR09-character-sheet-routes.json#L15-RR09-CHARACTER-SHEET-ROUTES route action=doProjectRangerFavoredEnemy#step:doProjectRangerFavoredEnemy | pass | covered |
| cleanroom-input/qnt/character-sheet-runtime/character-sheet-class-feature-selected-identity.mbt.qnt#step:doProjectSorcererDraconicSpells | tasks/target-replay-evidence/L15-RR09-character-sheet-routes.json#L15-RR09-CHARACTER-SHEET-ROUTES route action=doProjectSorcererDraconicSpells#step:doProjectSorcererDraconicSpells | pass | covered |
| cleanroom-input/qnt/character-sheet-runtime/character-sheet-class-feature-selected-identity.mbt.qnt#step:doProjectWarlockFiendSpells | tasks/target-replay-evidence/L15-RR09-character-sheet-routes.json#L15-RR09-CHARACTER-SHEET-ROUTES route action=doProjectWarlockFiendSpells#step:doProjectWarlockFiendSpells | pass | covered |
| cleanroom-input/qnt/character-sheet-runtime/character-sheet-healing-resource-selected-identity.mbt.qnt#step:doLayOnHandsRestoreHpAndRemovePoisoned | tasks/target-replay-evidence/L15-RR09-character-sheet-routes.json#L15-RR09-CHARACTER-SHEET-ROUTES route action=doLayOnHandsRestoreHpAndRemovePoisoned#step:doLayOnHandsRestoreHpAndRemovePoisoned | pass | covered |
| cleanroom-input/qnt/character-sheet-runtime/character-sheet-hit-point-maximum.mbt.qnt#step:doProjectFighterLevelOne | tasks/target-replay-evidence/L15-RR09-character-sheet-routes.json#L15-RR09-CHARACTER-SHEET-ROUTES route action=doProjectFighterLevelOne#step:doProjectFighterLevelOne | pass | covered |
| cleanroom-input/qnt/character-sheet-runtime/character-sheet-hit-point-maximum.mbt.qnt#step:doProjectFighterLevelTwo | tasks/target-replay-evidence/L15-RR09-character-sheet-routes.json#L15-RR09-CHARACTER-SHEET-ROUTES route action=doProjectFighterLevelTwo#step:doProjectFighterLevelTwo | pass | covered |
| cleanroom-input/qnt/character-sheet-runtime/character-sheet-hit-point-maximum.mbt.qnt#step:doProjectMinimumHigherLevelGain | tasks/target-replay-evidence/L15-RR09-character-sheet-routes.json#L15-RR09-CHARACTER-SHEET-ROUTES route action=doProjectMinimumHigherLevelGain#step:doProjectMinimumHigherLevelGain | pass | covered |
| cleanroom-input/qnt/character-sheet-runtime/character-sheet-hit-point-maximum.mbt.qnt#step:doProjectReducedEffectiveMaximum | tasks/target-replay-evidence/L15-RR09-character-sheet-routes.json#L15-RR09-CHARACTER-SHEET-ROUTES route action=doProjectReducedEffectiveMaximum#step:doProjectReducedEffectiveMaximum | pass | covered |
| cleanroom-input/qnt/character-sheet-runtime/character-sheet-hit-point-maximum.mbt.qnt#step:doProjectSorcererDraconicResilience | tasks/target-replay-evidence/L15-RR09-character-sheet-routes.json#L15-RR09-CHARACTER-SHEET-ROUTES route action=doProjectSorcererDraconicResilience#step:doProjectSorcererDraconicResilience | pass | covered |
| cleanroom-input/qnt/character-sheet-runtime/character-sheet-hit-point-maximum.mbt.qnt#step:doProjectWizardFighterMulticlass | tasks/target-replay-evidence/L15-RR09-character-sheet-routes.json#L15-RR09-CHARACTER-SHEET-ROUTES route action=doProjectWizardFighterMulticlass#step:doProjectWizardFighterMulticlass | pass | covered |
| cleanroom-input/qnt/character-sheet-runtime/character-sheet-hp-rest-hit-dice.mbt.qnt#step:doCompleteLongRestRestoresHpHitPointDiceAndMaximum | tasks/target-replay-evidence/L15-RR09-character-sheet-routes.json#L15-RR09-CHARACTER-SHEET-ROUTES route action=doCompleteLongRestRestoresHpHitPointDiceAndMaximum#step:doCompleteLongRestRestoresHpHitPointDiceAndMaximum | pass | covered |
| cleanroom-input/qnt/character-sheet-runtime/character-sheet-hp-rest-hit-dice.mbt.qnt#step:doInterruptLongRestBeforeOneHourNoBenefit | tasks/target-replay-evidence/L15-RR09-character-sheet-routes.json#L15-RR09-CHARACTER-SHEET-ROUTES route action=doInterruptLongRestBeforeOneHourNoBenefit#step:doInterruptLongRestBeforeOneHourNoBenefit | pass | covered |
| cleanroom-input/qnt/character-sheet-runtime/character-sheet-hp-rest-hit-dice.mbt.qnt#step:doInterruptLongRestWithShortRestBenefits | tasks/target-replay-evidence/L15-RR09-character-sheet-routes.json#L15-RR09-CHARACTER-SHEET-ROUTES route action=doInterruptLongRestWithShortRestBenefits#step:doInterruptLongRestWithShortRestBenefits | pass | covered |
| cleanroom-input/qnt/character-sheet-runtime/character-sheet-hp-rest-hit-dice.mbt.qnt#step:doInterruptShortRestNoBenefit | tasks/target-replay-evidence/L15-RR09-character-sheet-routes.json#L15-RR09-CHARACTER-SHEET-ROUTES route action=doInterruptShortRestNoBenefit#step:doInterruptShortRestNoBenefit | pass | covered |
| cleanroom-input/qnt/character-sheet-runtime/character-sheet-hp-rest-hit-dice.mbt.qnt#step:doRejectLongRestBeforeSixteenHourWait | tasks/target-replay-evidence/L15-RR09-character-sheet-routes.json#L15-RR09-CHARACTER-SHEET-ROUTES route action=doRejectLongRestBeforeSixteenHourWait#step:doRejectLongRestBeforeSixteenHourWait | pass | covered |
| cleanroom-input/qnt/character-sheet-runtime/character-sheet-hp-rest-hit-dice.mbt.qnt#step:doRejectLongRestDurationTooShort | tasks/target-replay-evidence/L15-RR09-character-sheet-routes.json#L15-RR09-CHARACTER-SHEET-ROUTES route action=doRejectLongRestDurationTooShort#step:doRejectLongRestDurationTooShort | pass | covered |
| cleanroom-input/qnt/character-sheet-runtime/character-sheet-hp-rest-hit-dice.mbt.qnt#step:doRejectLongRestInterruptionAtRequiredDuration | tasks/target-replay-evidence/L15-RR09-character-sheet-routes.json#L15-RR09-CHARACTER-SHEET-ROUTES route action=doRejectLongRestInterruptionAtRequiredDuration#step:doRejectLongRestInterruptionAtRequiredDuration | pass | covered |
| cleanroom-input/qnt/character-sheet-runtime/character-sheet-hp-rest-hit-dice.mbt.qnt#step:doRejectLongRestPhysicalExertionTooShort | tasks/target-replay-evidence/L15-RR09-character-sheet-routes.json#L15-RR09-CHARACTER-SHEET-ROUTES route action=doRejectLongRestPhysicalExertionTooShort#step:doRejectLongRestPhysicalExertionTooShort | pass | covered |
| cleanroom-input/qnt/character-sheet-runtime/character-sheet-hp-rest-hit-dice.mbt.qnt#step:doRejectLongRestStartAtZeroHp | tasks/target-replay-evidence/L15-RR09-character-sheet-routes.json#L15-RR09-CHARACTER-SHEET-ROUTES route action=doRejectLongRestStartAtZeroHp#step:doRejectLongRestStartAtZeroHp | pass | covered |
| cleanroom-input/qnt/character-sheet-runtime/character-sheet-hp-rest-hit-dice.mbt.qnt#step:doRejectShortRestDurationTooShort | tasks/target-replay-evidence/L15-RR09-character-sheet-routes.json#L15-RR09-CHARACTER-SHEET-ROUTES route action=doRejectShortRestDurationTooShort#step:doRejectShortRestDurationTooShort | pass | covered |
| cleanroom-input/qnt/character-sheet-runtime/character-sheet-hp-rest-hit-dice.mbt.qnt#step:doRejectShortRestStartAtZeroHp | tasks/target-replay-evidence/L15-RR09-character-sheet-routes.json#L15-RR09-CHARACTER-SHEET-ROUTES route action=doRejectShortRestStartAtZeroHp#step:doRejectShortRestStartAtZeroHp | pass | covered |
| cleanroom-input/qnt/character-sheet-runtime/character-sheet-hp-rest-hit-dice.mbt.qnt#step:doSpendShortRestHitPointDiceSequentially | tasks/target-replay-evidence/L15-RR09-character-sheet-routes.json#L15-RR09-CHARACTER-SHEET-ROUTES route action=doSpendShortRestHitPointDiceSequentially#step:doSpendShortRestHitPointDiceSequentially | pass | covered |
| cleanroom-input/qnt/character-sheet-runtime/character-sheet-hp-rest-hit-dice.mbt.qnt#step:doSpendShortRestHitPointDie | tasks/target-replay-evidence/L15-RR09-character-sheet-routes.json#L15-RR09-CHARACTER-SHEET-ROUTES route action=doSpendShortRestHitPointDie#step:doSpendShortRestHitPointDie | pass | covered |
| cleanroom-input/qnt/character-sheet-runtime/character-sheet-spell-slots-pact-slots.mbt.qnt#step:doCompleteLongRestRestoresOrdinaryPactAndClearsCreatedSlots | tasks/target-replay-evidence/L15-RR09-character-sheet-routes.json#L15-RR09-CHARACTER-SHEET-ROUTES route action=doCompleteLongRestRestoresOrdinaryPactAndClearsCreatedSlots#step:doCompleteLongRestRestoresOrdinaryPactAndClearsCreatedSlots | pass | covered |
| cleanroom-input/qnt/character-sheet-runtime/character-sheet-spell-slots-pact-slots.mbt.qnt#step:doInterruptLongRestBeforeOneHourNoSlotBenefit | tasks/target-replay-evidence/L15-RR09-character-sheet-routes.json#L15-RR09-CHARACTER-SHEET-ROUTES route action=doInterruptLongRestBeforeOneHourNoSlotBenefit#step:doInterruptLongRestBeforeOneHourNoSlotBenefit | pass | covered |
| cleanroom-input/qnt/character-sheet-runtime/character-sheet-spell-slots-pact-slots.mbt.qnt#step:doInterruptLongRestWithShortRestSlotBenefits | tasks/target-replay-evidence/L15-RR09-character-sheet-routes.json#L15-RR09-CHARACTER-SHEET-ROUTES route action=doInterruptLongRestWithShortRestSlotBenefits#step:doInterruptLongRestWithShortRestSlotBenefits | pass | covered |
| cleanroom-input/qnt/character-sheet-runtime/character-sheet-spell-slots-pact-slots.mbt.qnt#step:doInterruptShortRestNoSlotBenefit | tasks/target-replay-evidence/L15-RR09-character-sheet-routes.json#L15-RR09-CHARACTER-SHEET-ROUTES route action=doInterruptShortRestNoSlotBenefit#step:doInterruptShortRestNoSlotBenefit | pass | covered |
| cleanroom-input/qnt/character-sheet-runtime/character-sheet-spell-slots-pact-slots.mbt.qnt#step:doMagicalCunningRecoversPactSlots | tasks/target-replay-evidence/L15-RR09-character-sheet-routes.json#L15-RR09-CHARACTER-SHEET-ROUTES route action=doMagicalCunningRecoversPactSlots#step:doMagicalCunningRecoversPactSlots | pass | covered |
| cleanroom-input/qnt/character-sheet-runtime/character-sheet-spell-slots-pact-slots.mbt.qnt#step:doRejectArcaneRecoveryPactSlotRefund | tasks/target-replay-evidence/L15-RR09-character-sheet-routes.json#L15-RR09-CHARACTER-SHEET-ROUTES route action=doRejectArcaneRecoveryPactSlotRefund#step:doRejectArcaneRecoveryPactSlotRefund | pass | covered |
| cleanroom-input/qnt/character-sheet-runtime/character-sheet-spell-slots-pact-slots.mbt.qnt#step:doRejectMagicalCunningWithoutExpendedPactSlots | tasks/target-replay-evidence/L15-RR09-character-sheet-routes.json#L15-RR09-CHARACTER-SHEET-ROUTES route action=doRejectMagicalCunningWithoutExpendedPactSlots#step:doRejectMagicalCunningWithoutExpendedPactSlots | pass | covered |
| cleanroom-input/qnt/character-sheet-runtime/character-sheet-spell-slots-pact-slots.mbt.qnt#step:doRejectMismatchedOrdinarySpellSlotCapacity | tasks/target-replay-evidence/L15-RR09-character-sheet-routes.json#L15-RR09-CHARACTER-SHEET-ROUTES route action=doRejectMismatchedOrdinarySpellSlotCapacity#step:doRejectMismatchedOrdinarySpellSlotCapacity | pass | covered |
| cleanroom-input/qnt/character-sheet-runtime/character-sheet-spell-slots-pact-slots.mbt.qnt#step:doRejectPactSlotExpenditureOverCapacity | tasks/target-replay-evidence/L15-RR09-character-sheet-routes.json#L15-RR09-CHARACTER-SHEET-ROUTES route action=doRejectPactSlotExpenditureOverCapacity#step:doRejectPactSlotExpenditureOverCapacity | pass | covered |
| cleanroom-input/qnt/character-sheet-runtime/character-sheet-spell-slots-pact-slots.mbt.qnt#step:doShortRestArcaneRecoveryRefundsOrdinarySpellSlot | tasks/target-replay-evidence/L15-RR09-character-sheet-routes.json#L15-RR09-CHARACTER-SHEET-ROUTES route action=doShortRestArcaneRecoveryRefundsOrdinarySpellSlot#step:doShortRestArcaneRecoveryRefundsOrdinarySpellSlot | pass | covered |
| cleanroom-input/qnt/character-sheet-runtime/character-sheet-spell-slots-pact-slots.mbt.qnt#step:doShortRestRestoresPactSlotsOnly | tasks/target-replay-evidence/L15-RR09-character-sheet-routes.json#L15-RR09-CHARACTER-SHEET-ROUTES route action=doShortRestRestoresPactSlotsOnly#step:doShortRestRestoresPactSlotsOnly | pass | covered |
| cleanroom-input/qnt/character-sheet-runtime/character-sheet-spellbook-ritual-selected-identity.mbt.qnt#step:doInvokeSpellbookRitual | tasks/target-replay-evidence/L15-RR09-character-sheet-routes.json#L15-RR09-CHARACTER-SHEET-ROUTES route action=doInvokeSpellbookRitual#step:doInvokeSpellbookRitual | pass | covered |
| cleanroom-input/qnt/character-sheet-runtime/character-sheet-spellbook-ritual-selected-identity.mbt.qnt#step:doRejectMissingRitualAccessFeature | tasks/target-replay-evidence/L15-RR09-character-sheet-routes.json#L15-RR09-CHARACTER-SHEET-ROUTES route action=doRejectMissingRitualAccessFeature#step:doRejectMissingRitualAccessFeature | pass | covered |
| cleanroom-input/qnt/character-sheet-runtime/character-sheet-spellbook-ritual-selected-identity.mbt.qnt#step:doRejectNonLeveledRitualSpellbookSpell | tasks/target-replay-evidence/L15-RR09-character-sheet-routes.json#L15-RR09-CHARACTER-SHEET-ROUTES route action=doRejectNonLeveledRitualSpellbookSpell#step:doRejectNonLeveledRitualSpellbookSpell | pass | covered |
| cleanroom-input/qnt/character-sheet-runtime/character-sheet-spellbook-ritual-selected-identity.mbt.qnt#step:doRejectNonRitualSpellbookSpell | tasks/target-replay-evidence/L15-RR09-character-sheet-routes.json#L15-RR09-CHARACTER-SHEET-ROUTES route action=doRejectNonRitualSpellbookSpell#step:doRejectNonRitualSpellbookSpell | pass | covered |
| cleanroom-input/qnt/character-sheet-runtime/character-sheet-spellbook-ritual-selected-identity.mbt.qnt#step:doRejectPreparedOnlyRitual | tasks/target-replay-evidence/L15-RR09-character-sheet-routes.json#L15-RR09-CHARACTER-SHEET-ROUTES route action=doRejectPreparedOnlyRitual#step:doRejectPreparedOnlyRitual | pass | covered |
| cleanroom-input/qnt/character-sheet-runtime/character-sheet-weapon-mastery-containers-selected-identity.mbt.qnt#step:doAcceptOneChangeWeaponMasteryReselection | tasks/target-replay-evidence/L15-RR09-character-sheet-routes.json#L15-RR09-CHARACTER-SHEET-ROUTES route action=doAcceptOneChangeWeaponMasteryReselection#step:doAcceptOneChangeWeaponMasteryReselection | pass | covered |
| cleanroom-input/qnt/character-sheet-runtime/character-sheet-weapon-mastery-containers-selected-identity.mbt.qnt#step:doRejectTooManyChangesWeaponMasteryReselection | tasks/target-replay-evidence/L15-RR09-character-sheet-routes.json#L15-RR09-CHARACTER-SHEET-ROUTES route action=doRejectTooManyChangesWeaponMasteryReselection#step:doRejectTooManyChangesWeaponMasteryReselection | pass | covered |
| cleanroom-input/qnt/character-sheet-runtime/character-sheet-weapon-mastery-containers-selected-identity.mbt.qnt#step:doReselectPaladinWeaponMasteryOnLongRest | tasks/target-replay-evidence/L15-RR09-character-sheet-routes.json#L15-RR09-CHARACTER-SHEET-ROUTES route action=doReselectPaladinWeaponMasteryOnLongRest#step:doReselectPaladinWeaponMasteryOnLongRest | pass | covered |
| cleanroom-input/qnt/character-sheet-runtime/character-sheet-weapon-mastery-containers-selected-identity.mbt.qnt#step:doReselectRangerWeaponMasteryOnLongRest | tasks/target-replay-evidence/L15-RR09-character-sheet-routes.json#L15-RR09-CHARACTER-SHEET-ROUTES route action=doReselectRangerWeaponMasteryOnLongRest#step:doReselectRangerWeaponMasteryOnLongRest | pass | covered |
| cleanroom-input/qnt/character-sheet-runtime/character-sheet-weapon-mastery-containers-selected-identity.mbt.qnt#step:doReselectRogueWeaponMasteryOnLongRest | tasks/target-replay-evidence/L15-RR09-character-sheet-routes.json#L15-RR09-CHARACTER-SHEET-ROUTES route action=doReselectRogueWeaponMasteryOnLongRest#step:doReselectRogueWeaponMasteryOnLongRest | pass | covered |
| cleanroom-input/qnt/character-sheet-runtime/character-sheet-weapon-mastery-containers-selected-identity.mbt.qnt#step:doSelectPaladinWeaponMastery | tasks/target-replay-evidence/L15-RR09-character-sheet-routes.json#L15-RR09-CHARACTER-SHEET-ROUTES route action=doSelectPaladinWeaponMastery#step:doSelectPaladinWeaponMastery | pass | covered |
| cleanroom-input/qnt/character-sheet-runtime/character-sheet-weapon-mastery-containers-selected-identity.mbt.qnt#step:doSelectRangerWeaponMastery | tasks/target-replay-evidence/L15-RR09-character-sheet-routes.json#L15-RR09-CHARACTER-SHEET-ROUTES route action=doSelectRangerWeaponMastery#step:doSelectRangerWeaponMastery | pass | covered |
| cleanroom-input/qnt/character-sheet-runtime/character-sheet-weapon-mastery-containers-selected-identity.mbt.qnt#step:doSelectRogueWeaponMastery | tasks/target-replay-evidence/L15-RR09-character-sheet-routes.json#L15-RR09-CHARACTER-SHEET-ROUTES route action=doSelectRogueWeaponMastery#step:doSelectRogueWeaponMastery | pass | covered |
| cleanroom-input/qnt/character-battle-runtime/character-battle-origin-feat-selected-identity.mbt.qnt#step:doFinalizeCriminalAlertOriginFeat | tasks/target-replay-evidence/L15-RR10-character-battle-handoff-routes.json#L15-RR10-CHARACTER-BATTLE-HANDOFF-ROUTES route action=doFinalizeCriminalAlertOriginFeat#step:doFinalizeCriminalAlertOriginFeat | pass | covered |
| cleanroom-input/qnt/character-battle-runtime/character-battle-origin-feat-selected-identity.mbt.qnt#step:doProjectAlertInitiativeHandoff | tasks/target-replay-evidence/L15-RR10-character-battle-handoff-routes.json#L15-RR10-CHARACTER-BATTLE-HANDOFF-ROUTES route action=doProjectAlertInitiativeHandoff#step:doProjectAlertInitiativeHandoff | pass | covered |
| cleanroom-input/qnt/character-battle-runtime/character-battle-init-projection.mbt.qnt#step:doProjectSheetHitPointsArmorClassConditionsAndProfiles | tasks/target-replay-evidence/L15-RR10-character-battle-handoff-routes.json#L15-RR10-CHARACTER-BATTLE-HANDOFF-ROUTES route action=doProjectSheetHitPointsArmorClassConditionsAndProfiles#step:doProjectSheetHitPointsArmorClassConditionsAndProfiles | pass | covered |
| cleanroom-input/qnt/character-battle-runtime/character-battle-init-projection.mbt.qnt#step:doProjectPurePactMagicSlot | tasks/target-replay-evidence/L15-RR10-character-battle-handoff-routes.json#L15-RR10-CHARACTER-BATTLE-HANDOFF-ROUTES route action=doProjectPurePactMagicSlot#step:doProjectPurePactMagicSlot | pass | covered |
| cleanroom-input/qnt/character-battle-runtime/character-battle-init-projection.mbt.qnt#step:doRejectMixedSpellAndPactSlotInit | tasks/target-replay-evidence/L15-RR10-character-battle-handoff-routes.json#L15-RR10-CHARACTER-BATTLE-HANDOFF-ROUTES route action=doRejectMixedSpellAndPactSlotInit#step:doRejectMixedSpellAndPactSlotInit | pass | covered |
| cleanroom-input/qnt/character-battle-runtime/character-battle-init-projection.mbt.qnt#step:doRejectBuildMaximumAboveBuildMaximum | tasks/target-replay-evidence/L15-RR10-character-battle-handoff-routes.json#L15-RR10-CHARACTER-BATTLE-HANDOFF-ROUTES route action=doRejectBuildMaximumAboveBuildMaximum#step:doRejectBuildMaximumAboveBuildMaximum | pass | covered |
| cleanroom-input/qnt/character-battle-runtime/character-battle-init-projection.mbt.qnt#step:doRejectStableRecoveryProgressDuringInit | tasks/target-replay-evidence/L15-RR10-character-battle-handoff-routes.json#L15-RR10-CHARACTER-BATTLE-HANDOFF-ROUTES route action=doRejectStableRecoveryProgressDuringInit#step:doRejectStableRecoveryProgressDuringInit | pass | covered |
| cleanroom-input/qnt/character-battle-runtime/character-battle-settlement.mbt.qnt#step:doSettleHitPointsConditionsSlotsAndPreservedSheetState | tasks/target-replay-evidence/L15-RR10-character-battle-handoff-routes.json#L15-RR10-CHARACTER-BATTLE-HANDOFF-ROUTES route action=doSettleHitPointsConditionsSlotsAndPreservedSheetState#step:doSettleHitPointsConditionsSlotsAndPreservedSheetState | pass | covered |
| cleanroom-input/qnt/character-battle-runtime/character-battle-settlement.mbt.qnt#step:doSettlePurePactMagicSlotExpenditure | tasks/target-replay-evidence/L15-RR10-character-battle-handoff-routes.json#L15-RR10-CHARACTER-BATTLE-HANDOFF-ROUTES route action=doSettlePurePactMagicSlotExpenditure#step:doSettlePurePactMagicSlotExpenditure | pass | covered |
| cleanroom-input/qnt/character-battle-runtime/character-battle-settlement.mbt.qnt#step:doRejectMixedSpellAndPactSlotSettlement | tasks/target-replay-evidence/L15-RR10-character-battle-handoff-routes.json#L15-RR10-CHARACTER-BATTLE-HANDOFF-ROUTES route action=doRejectMixedSpellAndPactSlotSettlement#step:doRejectMixedSpellAndPactSlotSettlement | pass | covered |
| cleanroom-input/qnt/character-battle-runtime/character-battle-settlement.mbt.qnt#step:doSettleFeatureResourceExpenditure | tasks/target-replay-evidence/L15-RR10-character-battle-handoff-routes.json#L15-RR10-CHARACTER-BATTLE-HANDOFF-ROUTES route action=doSettleFeatureResourceExpenditure#step:doSettleFeatureResourceExpenditure | pass | covered |
| cleanroom-input/qnt/character-battle-runtime/character-battle-settlement.mbt.qnt#step:doRejectMismatchedCharacterIdentity | tasks/target-replay-evidence/L15-RR10-character-battle-handoff-routes.json#L15-RR10-CHARACTER-BATTLE-HANDOFF-ROUTES route action=doRejectMismatchedCharacterIdentity#step:doRejectMismatchedCharacterIdentity | pass | covered |
| cleanroom-input/qnt/character-battle-runtime/character-battle-settlement.mbt.qnt#step:doRejectMaximumHpDrift | tasks/target-replay-evidence/L15-RR10-character-battle-handoff-routes.json#L15-RR10-CHARACTER-BATTLE-HANDOFF-ROUTES route action=doRejectMaximumHpDrift#step:doRejectMaximumHpDrift | pass | covered |
| cleanroom-input/qnt/character-battle-runtime/character-battle-settlement.mbt.qnt#step:doRejectActiveWildShapeHandoff | tasks/target-replay-evidence/L15-RR10-character-battle-handoff-routes.json#L15-RR10-CHARACTER-BATTLE-HANDOFF-ROUTES route action=doRejectActiveWildShapeHandoff#step:doRejectActiveWildShapeHandoff | pass | covered |
| cleanroom-input/qnt/character-battle-runtime/character-battle-settlement.mbt.qnt#step:doRejectActiveBattleStateHandoff | tasks/target-replay-evidence/L15-RR10-character-battle-handoff-routes.json#L15-RR10-CHARACTER-BATTLE-HANDOFF-ROUTES route action=doRejectActiveBattleStateHandoff#step:doRejectActiveBattleStateHandoff | pass | covered |
| cleanroom-input/qnt/character-battle-runtime/character-battle-settlement.mbt.qnt#step:doRejectStableRecoveryProgressHandoff | tasks/target-replay-evidence/L15-RR10-character-battle-handoff-routes.json#L15-RR10-CHARACTER-BATTLE-HANDOFF-ROUTES route action=doRejectStableRecoveryProgressHandoff#step:doRejectStableRecoveryProgressHandoff | pass | covered |
| cleanroom-input/qnt/character-battle-runtime/character-battle-settlement.mbt.qnt#step:doSettleZeroHpStableLifecycle | tasks/target-replay-evidence/L15-RR10-character-battle-handoff-routes.json#L15-RR10-CHARACTER-BATTLE-HANDOFF-ROUTES route action=doSettleZeroHpStableLifecycle#step:doSettleZeroHpStableLifecycle | pass | covered |
| cleanroom-input/qnt/character-battle-runtime/character-layer-projection-lifecycle.mbt.qnt#step:doFinalizeDraftToBuild | tasks/target-replay-evidence/L15-RR10-character-battle-handoff-routes.json#L15-RR10-CHARACTER-BATTLE-HANDOFF-ROUTES route action=doFinalizeDraftToBuild#step:doFinalizeDraftToBuild | pass | covered |
| cleanroom-input/qnt/character-battle-runtime/character-layer-projection-lifecycle.mbt.qnt#step:doCreateSheetFromBuild | tasks/target-replay-evidence/L15-RR10-character-battle-handoff-routes.json#L15-RR10-CHARACTER-BATTLE-HANDOFF-ROUTES route action=doCreateSheetFromBuild#step:doCreateSheetFromBuild | pass | covered |
| cleanroom-input/qnt/character-battle-runtime/character-layer-projection-lifecycle.mbt.qnt#step:doProjectSheetToBattleInit | tasks/target-replay-evidence/L15-RR10-character-battle-handoff-routes.json#L15-RR10-CHARACTER-BATTLE-HANDOFF-ROUTES route action=doProjectSheetToBattleInit#step:doProjectSheetToBattleInit | pass | covered |
| cleanroom-input/qnt/character-battle-runtime/character-layer-projection-lifecycle.mbt.qnt#step:doResolveSkeletonShortswordAttack | tasks/target-replay-evidence/L15-RR10-character-battle-handoff-routes.json#L15-RR10-CHARACTER-BATTLE-HANDOFF-ROUTES route action=doResolveSkeletonShortswordAttack#step:doResolveSkeletonShortswordAttack | pass | covered |
| cleanroom-input/qnt/character-battle-runtime/character-layer-projection-lifecycle.mbt.qnt#step:doSettleBattleToSheet | tasks/target-replay-evidence/L15-RR10-character-battle-handoff-routes.json#L15-RR10-CHARACTER-BATTLE-HANDOFF-ROUTES route action=doSettleBattleToSheet#step:doSettleBattleToSheet | pass | covered |
| cleanroom-input/qnt/character-battle-runtime/character-sheet-feature-resources.mbt.qnt#step:doLayOnHandsRestoresHpAndRemovesPoisoned | tasks/target-replay-evidence/L15-RR10-character-battle-handoff-routes.json#L15-RR10-CHARACTER-BATTLE-HANDOFF-ROUTES route action=doLayOnHandsRestoresHpAndRemovesPoisoned#step:doLayOnHandsRestoresHpAndRemovesPoisoned | pass | covered |
| cleanroom-input/qnt/character-battle-runtime/character-sheet-feature-resources.mbt.qnt#step:doRejectLayOnHandsOverspend | tasks/target-replay-evidence/L15-RR10-character-battle-handoff-routes.json#L15-RR10-CHARACTER-BATTLE-HANDOFF-ROUTES route action=doRejectLayOnHandsOverspend#step:doRejectLayOnHandsOverspend | pass | covered |
| cleanroom-input/qnt/character-battle-runtime/character-sheet-feature-resources.mbt.qnt#step:doLongRestClearsLayOnHandsPool | tasks/target-replay-evidence/L15-RR10-character-battle-handoff-routes.json#L15-RR10-CHARACTER-BATTLE-HANDOFF-ROUTES route action=doLongRestClearsLayOnHandsPool#step:doLongRestClearsLayOnHandsPool | pass | covered |
| cleanroom-input/qnt/character-battle-runtime/character-sheet-feature-resources.mbt.qnt#step:doShortRestRecoversUseCountPools | tasks/target-replay-evidence/L15-RR10-character-battle-handoff-routes.json#L15-RR10-CHARACTER-BATTLE-HANDOFF-ROUTES route action=doShortRestRecoversUseCountPools#step:doShortRestRecoversUseCountPools | pass | covered |
| cleanroom-input/qnt/character-battle-runtime/character-sheet-feature-resources.mbt.qnt#step:doLongRestClearsPointPoolAndUseState | tasks/target-replay-evidence/L15-RR10-character-battle-handoff-routes.json#L15-RR10-CHARACTER-BATTLE-HANDOFF-ROUTES route action=doLongRestClearsPointPoolAndUseState#step:doLongRestClearsPointPoolAndUseState | pass | covered |
| cleanroom-input/qnt/character-battle-runtime/character-sheet-feature-resources.mbt.qnt#step:doFontOfMagicSlotToPoints | tasks/target-replay-evidence/L15-RR10-character-battle-handoff-routes.json#L15-RR10-CHARACTER-BATTLE-HANDOFF-ROUTES route action=doFontOfMagicSlotToPoints#step:doFontOfMagicSlotToPoints | pass | covered |
| cleanroom-input/qnt/character-battle-runtime/character-sheet-feature-resources.mbt.qnt#step:doShortRestPreservesUncannyUseState | tasks/target-replay-evidence/L15-RR10-character-battle-handoff-routes.json#L15-RR10-CHARACTER-BATTLE-HANDOFF-ROUTES route action=doShortRestPreservesUncannyUseState#step:doShortRestPreservesUncannyUseState | pass | covered |
| cleanroom-input/qnt/character-battle-runtime/character-sheet-feature-resources.mbt.qnt#step:doLongRestClearsUncannyUseState | tasks/target-replay-evidence/L15-RR10-character-battle-handoff-routes.json#L15-RR10-CHARACTER-BATTLE-HANDOFF-ROUTES route action=doLongRestClearsUncannyUseState#step:doLongRestClearsUncannyUseState | pass | covered |
| cleanroom-input/qnt/character-battle-runtime/character-sheet-feature-resources.mbt.qnt#step:doUncannyMetabolismRecoversFocusAndHeals | tasks/target-replay-evidence/L15-RR10-character-battle-handoff-routes.json#L15-RR10-CHARACTER-BATTLE-HANDOFF-ROUTES route action=doUncannyMetabolismRecoversFocusAndHeals#step:doUncannyMetabolismRecoversFocusAndHeals | pass | covered |
| cleanroom-input/qnt/character-battle-runtime/character-sheet-feature-resources.mbt.qnt#step:doRejectUncannyMetabolismRepeatUse | tasks/target-replay-evidence/L15-RR10-character-battle-handoff-routes.json#L15-RR10-CHARACTER-BATTLE-HANDOFF-ROUTES route action=doRejectUncannyMetabolismRepeatUse#step:doRejectUncannyMetabolismRepeatUse | pass | covered |
| cleanroom-input/qnt/battle-runtime/battle-runtime-healing-stabilization-selected-identity.mbt.qnt#step:doResolveSpareTheDyingStable | tasks/target-replay-evidence/L15-RR07S-B-zero-hp-form-movement-substrates.json#L15-RR07S-B-ZERO-HP-FORM-MOVEMENT-SUBSTRATES route action=doResolveSpareTheDyingStable#step:doResolveSpareTheDyingStable | pass | covered |
| cleanroom-input/qnt/battle-runtime/battle-runtime-druid-wild-shape-form-lifecycle.mbt.qnt#step:doAssumeRidingHorse | tasks/target-replay-evidence/L15-RR07S-B-zero-hp-form-movement-substrates.json#L15-RR07S-B-ZERO-HP-FORM-MOVEMENT-SUBSTRATES route action=doAssumeRidingHorse#step:doAssumeRidingHorse | pass | covered |
| cleanroom-input/qnt/battle-runtime/battle-runtime-druid-wild-shape-form-lifecycle.mbt.qnt#step:doBeginNextTurn | tasks/target-replay-evidence/L15-RR07S-B-zero-hp-form-movement-substrates.json#L15-RR07S-B-ZERO-HP-FORM-MOVEMENT-SUBSTRATES route action=doBeginNextTurn#step:doBeginNextTurn | pass | covered |
| cleanroom-input/qnt/battle-runtime/battle-runtime-druid-wild-shape-form-lifecycle.mbt.qnt#step:doDeathReversion | tasks/target-replay-evidence/L15-RR07S-B-zero-hp-form-movement-substrates.json#L15-RR07S-B-ZERO-HP-FORM-MOVEMENT-SUBSTRATES route action=doDeathReversion#step:doDeathReversion | pass | covered |
| cleanroom-input/qnt/battle-runtime/battle-runtime-druid-wild-shape-form-lifecycle.mbt.qnt#step:doDismissForm | tasks/target-replay-evidence/L15-RR07S-B-zero-hp-form-movement-substrates.json#L15-RR07S-B-ZERO-HP-FORM-MOVEMENT-SUBSTRATES route action=doDismissForm#step:doDismissForm | pass | covered |
| cleanroom-input/qnt/battle-runtime/battle-runtime-druid-wild-shape-form-lifecycle.mbt.qnt#step:doIncapacitatedReversion | tasks/target-replay-evidence/L15-RR07S-B-zero-hp-form-movement-substrates.json#L15-RR07S-B-ZERO-HP-FORM-MOVEMENT-SUBSTRATES route action=doIncapacitatedReversion#step:doIncapacitatedReversion | pass | covered |
| cleanroom-input/qnt/battle-runtime/battle-runtime-druid-wild-shape-form-lifecycle.mbt.qnt#step:doReuseAsCat | tasks/target-replay-evidence/L15-RR07S-B-zero-hp-form-movement-substrates.json#L15-RR07S-B-ZERO-HP-FORM-MOVEMENT-SUBSTRATES route action=doReuseAsCat#step:doReuseAsCat | pass | covered |
| cleanroom-input/qnt/battle-runtime/battle-runtime-druid-wild-shape-form-lifecycle.mbt.qnt#step:doStutter | tasks/target-replay-evidence/L15-RR07S-B-zero-hp-form-movement-substrates.json#L15-RR07S-B-ZERO-HP-FORM-MOVEMENT-SUBSTRATES route action=doStutter#step:doStutter | pass | covered |
| cleanroom-input/qnt/battle-runtime/battle-runtime-movement-forced-movement-selected-identity.mbt.qnt#step:doCommandFleeTargetTurn | tasks/target-replay-evidence/L15-RR07S-B-zero-hp-form-movement-substrates.json#L15-RR07S-B-ZERO-HP-FORM-MOVEMENT-SUBSTRATES route action=doCommandFleeTargetTurn#step:doCommandFleeTargetTurn | pass | covered |
| cleanroom-input/qnt/battle-runtime/battle-runtime-movement-forced-movement-selected-identity.mbt.qnt#step:doDissonantWhispersForcedReactionMovement | tasks/target-replay-evidence/L15-RR07S-B-zero-hp-form-movement-substrates.json#L15-RR07S-B-ZERO-HP-FORM-MOVEMENT-SUBSTRATES route action=doDissonantWhispersForcedReactionMovement#step:doDissonantWhispersForcedReactionMovement | pass | covered |
| cleanroom-input/qnt/battle-runtime/battle-runtime-movement-forced-movement-selected-identity.mbt.qnt#step:doExpeditiousRetreatImmediateDash | tasks/target-replay-evidence/L15-RR07S-B-zero-hp-form-movement-substrates.json#L15-RR07S-B-ZERO-HP-FORM-MOVEMENT-SUBSTRATES route action=doExpeditiousRetreatImmediateDash#step:doExpeditiousRetreatImmediateDash | pass | covered |
| cleanroom-input/qnt/battle-runtime/battle-runtime-movement-forced-movement-selected-identity.mbt.qnt#step:doMonkUnarmoredMovementDash | tasks/target-replay-evidence/L15-RR07S-B-zero-hp-form-movement-substrates.json#L15-RR07S-B-ZERO-HP-FORM-MOVEMENT-SUBSTRATES route action=doMonkUnarmoredMovementDash#step:doMonkUnarmoredMovementDash | pass | covered |
| cleanroom-input/qnt/battle-runtime/battle-runtime-weapon-mastery-selected-identity.mbt.qnt#step:doResolveSapMasteryPropertyHit | tasks/target-replay-evidence/L15-RR07S-C-weapon-breath-feature-substrates.json#L15-RR07S-C replay action=doResolveSapMasteryPropertyHit#step:doResolveSapMasteryPropertyHit | pass | covered |
| cleanroom-input/qnt/battle-runtime/battle-runtime-weapon-mastery-selected-identity.mbt.qnt#step:doResolveToppleMasteryPropertyFailedSavingThrow | tasks/target-replay-evidence/L15-RR07S-C-weapon-breath-feature-substrates.json#L15-RR07S-C replay action=doResolveToppleMasteryPropertyFailedSavingThrow#step:doResolveToppleMasteryPropertyFailedSavingThrow | pass | covered |
| cleanroom-input/qnt/battle-runtime/battle-runtime-weapon-mastery-selected-identity.mbt.qnt#step:doResolveCleaveMasteryPropertySecondTargetHit | tasks/target-replay-evidence/L15-RR07S-C-weapon-breath-feature-substrates.json#L15-RR07S-C replay action=doResolveCleaveMasteryPropertySecondTargetHit#step:doResolveCleaveMasteryPropertySecondTargetHit | pass | covered |
| cleanroom-input/qnt/battle-runtime/battle-runtime-dragonborn-breath-weapon.mbt.qnt#step:doResolveBreathWeapon | tasks/target-replay-evidence/L15-RR07S-C-weapon-breath-feature-substrates.json#L15-RR07S-C replay action=doResolveBreathWeapon#step:doResolveBreathWeapon | pass | covered |
| cleanroom-input/qnt/battle-runtime/battle-runtime-dragonborn-breath-weapon.mbt.qnt#step:doOpenExtraAttackSlot | tasks/target-replay-evidence/L15-RR07S-C-weapon-breath-feature-substrates.json#L15-RR07S-C replay action=doOpenExtraAttackSlot#step:doOpenExtraAttackSlot | pass | covered |
| cleanroom-input/qnt/battle-runtime/battle-runtime-dragonborn-breath-weapon.mbt.qnt#step:doRejectMissingResource | tasks/target-replay-evidence/L15-RR07S-C-weapon-breath-feature-substrates.json#L15-RR07S-C replay action=doRejectMissingResource#step:doRejectMissingResource | pass | covered |
| cleanroom-input/qnt/battle-runtime/battle-runtime-dragonborn-breath-weapon.mbt.qnt#step:doRejectMismatchedArea | tasks/target-replay-evidence/L15-RR07S-C-weapon-breath-feature-substrates.json#L15-RR07S-C replay action=doRejectMismatchedArea#step:doRejectMismatchedArea | pass | covered |
| cleanroom-input/qnt/battle-runtime/battle-runtime-dragonborn-breath-weapon.mbt.qnt#step:doRejectInvalidDamageRoll | tasks/target-replay-evidence/L15-RR07S-C-weapon-breath-feature-substrates.json#L15-RR07S-C replay action=doRejectInvalidDamageRoll#step:doRejectInvalidDamageRoll | pass | covered |
| cleanroom-input/qnt/battle-runtime/battle-runtime-feature-selected-identity.mbt.qnt#step:doActivateInnateSorcery | tasks/target-replay-evidence/L15-RR07S-C-weapon-breath-feature-substrates.json#L15-RR07S-C replay action=doActivateInnateSorcery#step:doActivateInnateSorcery | pass | covered |
| cleanroom-input/qnt/battle-runtime/battle-runtime-feature-selected-identity.mbt.qnt#step:doProjectInnateSorcerySpellBenefits | tasks/target-replay-evidence/L15-RR07S-C-weapon-breath-feature-substrates.json#L15-RR07S-C replay action=doProjectInnateSorcerySpellBenefits#step:doProjectInnateSorcerySpellBenefits | pass | covered |
| cleanroom-input/qnt/battle-runtime/battle-runtime-feature-selected-identity.mbt.qnt#step:doExcludeInnateSorceryNonSorcererSpellBenefits | tasks/target-replay-evidence/L15-RR07S-C-weapon-breath-feature-substrates.json#L15-RR07S-C replay action=doExcludeInnateSorceryNonSorcererSpellBenefits#step:doExcludeInnateSorceryNonSorcererSpellBenefits | pass | covered |
| cleanroom-input/qnt/battle-runtime/battle-runtime-adrenaline-rush.mbt.qnt#step:doAdrenalineRushDash | tasks/target-replay-evidence/L15-RR07S-A-PASSIVE-ROLL-RESOURCE-SUBSTRATES.json#L15-RR07S-A-PASSIVE-ROLL-RESOURCE-SUBSTRATES replay action=doAdrenalineRushDash#step:doAdrenalineRushDash | pass | covered |
| cleanroom-input/qnt/battle-runtime/battle-runtime-adrenaline-rush.mbt.qnt#step:doRejectSecondDash | tasks/target-replay-evidence/L15-RR07S-A-PASSIVE-ROLL-RESOURCE-SUBSTRATES.json#L15-RR07S-A-PASSIVE-ROLL-RESOURCE-SUBSTRATES replay action=doRejectSecondDash#step:doRejectSecondDash | pass | covered |
| cleanroom-input/qnt/battle-runtime/battle-runtime-halfling-nimbleness-selected-identity.mbt.qnt#step:doMoveThroughLargerCreatureSpace | tasks/target-replay-evidence/L15-RR07S-A-PASSIVE-ROLL-RESOURCE-SUBSTRATES.json#L15-RR07S-A-PASSIVE-ROLL-RESOURCE-SUBSTRATES replay action=doMoveThroughLargerCreatureSpace#step:doMoveThroughLargerCreatureSpace | pass | covered |
| cleanroom-input/qnt/battle-runtime/battle-runtime-halfling-nimbleness-selected-identity.mbt.qnt#step:doRejectOccupiedStop | tasks/target-replay-evidence/L15-RR07S-A-PASSIVE-ROLL-RESOURCE-SUBSTRATES.json#L15-RR07S-A-PASSIVE-ROLL-RESOURCE-SUBSTRATES replay action=doRejectOccupiedStop#step:doRejectOccupiedStop | pass | covered |
| cleanroom-input/qnt/battle-runtime/battle-runtime-halfling-nimbleness-selected-identity.mbt.qnt#step:doRejectMissingProfile | tasks/target-replay-evidence/L15-RR07S-A-PASSIVE-ROLL-RESOURCE-SUBSTRATES.json#L15-RR07S-A-PASSIVE-ROLL-RESOURCE-SUBSTRATES replay action=doRejectMissingProfile#step:doRejectMissingProfile | pass | covered |
| cleanroom-input/qnt/battle-runtime/battle-runtime-halfling-nimbleness-selected-identity.mbt.qnt#step:doRejectSameSizeTraversal | tasks/target-replay-evidence/L15-RR07S-A-PASSIVE-ROLL-RESOURCE-SUBSTRATES.json#L15-RR07S-A-PASSIVE-ROLL-RESOURCE-SUBSTRATES replay action=doRejectSameSizeTraversal#step:doRejectSameSizeTraversal | pass | covered |
| cleanroom-input/qnt/battle-runtime/battle-runtime-species-passive-trait-selected-identity.mbt.qnt#step:doDragonbornDamageResistance | tasks/target-replay-evidence/L15-RR07S-A-PASSIVE-ROLL-RESOURCE-SUBSTRATES.json#L15-RR07S-A-PASSIVE-ROLL-RESOURCE-SUBSTRATES replay action=doDragonbornDamageResistance#step:doDragonbornDamageResistance | pass | covered |
| cleanroom-input/qnt/battle-runtime/battle-runtime-species-passive-trait-selected-identity.mbt.qnt#step:doDwarvenResilience | tasks/target-replay-evidence/L15-RR07S-A-PASSIVE-ROLL-RESOURCE-SUBSTRATES.json#L15-RR07S-A-PASSIVE-ROLL-RESOURCE-SUBSTRATES replay action=doDwarvenResilience#step:doDwarvenResilience | pass | covered |
| cleanroom-input/qnt/battle-runtime/battle-runtime-species-passive-trait-selected-identity.mbt.qnt#step:doHalflingBrave | tasks/target-replay-evidence/L15-RR07S-A-PASSIVE-ROLL-RESOURCE-SUBSTRATES.json#L15-RR07S-A-PASSIVE-ROLL-RESOURCE-SUBSTRATES replay action=doHalflingBrave#step:doHalflingBrave | pass | covered |
| cleanroom-input/qnt/battle-runtime/battle-runtime-species-passive-trait-selected-identity.mbt.qnt#step:doGoliathPowerfulBuild | tasks/target-replay-evidence/L15-RR07S-A-PASSIVE-ROLL-RESOURCE-SUBSTRATES.json#L15-RR07S-A-PASSIVE-ROLL-RESOURCE-SUBSTRATES replay action=doGoliathPowerfulBuild#step:doGoliathPowerfulBuild | pass | covered |
| cleanroom-input/qnt/battle-runtime/battle-runtime-danger-sense-selected-identity.mbt.qnt#step:doProjectDangerSenseDexterityAdvantage | tasks/target-replay-evidence/L15-RR07S-A-PASSIVE-ROLL-RESOURCE-SUBSTRATES.json#L15-RR07S-A-PASSIVE-ROLL-RESOURCE-SUBSTRATES replay action=doProjectDangerSenseDexterityAdvantage#step:doProjectDangerSenseDexterityAdvantage | pass | covered |
| cleanroom-input/qnt/battle-runtime/battle-runtime-danger-sense-selected-identity.mbt.qnt#step:doSuppressDangerSenseWhileIncapacitated | tasks/target-replay-evidence/L15-RR07S-A-PASSIVE-ROLL-RESOURCE-SUBSTRATES.json#L15-RR07S-A-PASSIVE-ROLL-RESOURCE-SUBSTRATES replay action=doSuppressDangerSenseWhileIncapacitated#step:doSuppressDangerSenseWhileIncapacitated | pass | covered |
| cleanroom-input/qnt/battle-runtime/battle-runtime-roll-modifier-buff-selected-identity.mbt.qnt#step:doBlessAttackAndSaveModifier | tasks/target-replay-evidence/L15-RR07S-A-PASSIVE-ROLL-RESOURCE-SUBSTRATES.json#L15-RR07S-A-PASSIVE-ROLL-RESOURCE-SUBSTRATES replay action=doBlessAttackAndSaveModifier#step:doBlessAttackAndSaveModifier | pass | covered |
| cleanroom-input/qnt/battle-runtime/battle-runtime-roll-modifier-buff-selected-identity.mbt.qnt#step:doBaneFailedSavePenalty | tasks/target-replay-evidence/L15-RR07S-A-PASSIVE-ROLL-RESOURCE-SUBSTRATES.json#L15-RR07S-A-PASSIVE-ROLL-RESOURCE-SUBSTRATES replay action=doBaneFailedSavePenalty#step:doBaneFailedSavePenalty | pass | covered |
| cleanroom-input/qnt/battle-runtime/battle-runtime-roll-modifier-buff-selected-identity.mbt.qnt#step:doGuidanceSkillAbilityCheckModifier | tasks/target-replay-evidence/L15-RR07S-A-PASSIVE-ROLL-RESOURCE-SUBSTRATES.json#L15-RR07S-A-PASSIVE-ROLL-RESOURCE-SUBSTRATES replay action=doGuidanceSkillAbilityCheckModifier#step:doGuidanceSkillAbilityCheckModifier | pass | covered |
| cleanroom-input/qnt/battle-runtime/battle-runtime-roll-modifier-buff-selected-identity.mbt.qnt#step:doResistanceReducesMatchingDamage | tasks/target-replay-evidence/L15-RR07S-A-PASSIVE-ROLL-RESOURCE-SUBSTRATES.json#L15-RR07S-A-PASSIVE-ROLL-RESOURCE-SUBSTRATES replay action=doResistanceReducesMatchingDamage#step:doResistanceReducesMatchingDamage | pass | covered |
| cleanroom-input/qnt/battle-runtime/battle-runtime-roll-modifier-buff-selected-identity.mbt.qnt#step:doShieldOfFaithArmorClassBonus | tasks/target-replay-evidence/L15-RR07S-A-PASSIVE-ROLL-RESOURCE-SUBSTRATES.json#L15-RR07S-A-PASSIVE-ROLL-RESOURCE-SUBSTRATES replay action=doShieldOfFaithArmorClassBonus#step:doShieldOfFaithArmorClassBonus | pass | covered |
| cleanroom-input/qnt/battle-runtime/battle-runtime-quickened-spell-governor.mbt.qnt#step:doResolveQuickenedRestoration | tasks/target-replay-evidence/L15-RR07-FU08A-METAMAGIC-GOVERNOR-QUICKENED-SUBSTRATE.json#L15-RR07-FU08A-METAMAGIC-GOVERNOR-QUICKENED-SUBSTRATE seed=1 action=doResolveQuickenedRestoration#step:doResolveQuickenedRestoration | pass | covered |
| cleanroom-input/qnt/battle-runtime/battle-runtime-quickened-spell-governor.mbt.qnt#step:doResolveQuickenedSaveGatedCondition | tasks/target-replay-evidence/L15-RR07-FU08A-METAMAGIC-GOVERNOR-QUICKENED-SUBSTRATE.json#L15-RR07-FU08A-METAMAGIC-GOVERNOR-QUICKENED-SUBSTRATE seed=1 action=doResolveQuickenedSaveGatedCondition#step:doResolveQuickenedSaveGatedCondition | pass | covered |
| cleanroom-input/qnt/battle-runtime/battle-runtime-quickened-spell-governor.mbt.qnt#step:doResolveQuickenedSaveGatedConditionImmunity | tasks/target-replay-evidence/L15-RR07-FU08A-METAMAGIC-GOVERNOR-QUICKENED-SUBSTRATE.json#L15-RR07-FU08A-METAMAGIC-GOVERNOR-QUICKENED-SUBSTRATE seed=1 action=doResolveQuickenedSaveGatedConditionImmunity#step:doResolveQuickenedSaveGatedConditionImmunity | pass | covered |
| cleanroom-input/qnt/battle-runtime/battle-runtime-quickened-spell-governor.mbt.qnt#step:doResolveQuickenedDirectCondition | tasks/target-replay-evidence/L15-RR07-FU08A-METAMAGIC-GOVERNOR-QUICKENED-SUBSTRATE.json#L15-RR07-FU08A-METAMAGIC-GOVERNOR-QUICKENED-SUBSTRATE seed=1 action=doResolveQuickenedDirectCondition#step:doResolveQuickenedDirectCondition | pass | covered |
| cleanroom-input/qnt/battle-runtime/battle-runtime-quickened-spell-governor.mbt.qnt#step:doResolveQuickenedRollModifier | tasks/target-replay-evidence/L15-RR07-FU08A-METAMAGIC-GOVERNOR-QUICKENED-SUBSTRATE.json#L15-RR07-FU08A-METAMAGIC-GOVERNOR-QUICKENED-SUBSTRATE seed=1 action=doResolveQuickenedRollModifier#step:doResolveQuickenedRollModifier | pass | covered |
| cleanroom-input/qnt/battle-runtime/battle-runtime-quickened-spell-governor.mbt.qnt#step:doResolveQuickenedAfterMagicActionSpent | tasks/target-replay-evidence/L15-RR07-FU08A-METAMAGIC-GOVERNOR-QUICKENED-SUBSTRATE.json#L15-RR07-FU08A-METAMAGIC-GOVERNOR-QUICKENED-SUBSTRATE seed=1 action=doResolveQuickenedAfterMagicActionSpent#step:doResolveQuickenedAfterMagicActionSpent | pass | covered |
| cleanroom-input/qnt/battle-runtime/battle-runtime-quickened-spell-governor.mbt.qnt#step:doRejectUnaffordable | tasks/target-replay-evidence/L15-RR07-FU08A-METAMAGIC-GOVERNOR-QUICKENED-SUBSTRATE.json#L15-RR07-FU08A-METAMAGIC-GOVERNOR-QUICKENED-SUBSTRATE seed=1 action=doRejectUnaffordable#step:doRejectUnaffordable | pass | covered |
| cleanroom-input/qnt/battle-runtime/battle-runtime-quickened-spell-governor.mbt.qnt#step:doRejectUnknownOption | tasks/target-replay-evidence/L15-RR07-FU08A-METAMAGIC-GOVERNOR-QUICKENED-SUBSTRATE.json#L15-RR07-FU08A-METAMAGIC-GOVERNOR-QUICKENED-SUBSTRATE seed=1 action=doRejectUnknownOption#step:doRejectUnknownOption | pass | covered |
| cleanroom-input/qnt/battle-runtime/battle-runtime-quickened-spell-governor.mbt.qnt#step:doRejectUnsupportedSecondOption | tasks/target-replay-evidence/L15-RR07-FU08A-METAMAGIC-GOVERNOR-QUICKENED-SUBSTRATE.json#L15-RR07-FU08A-METAMAGIC-GOVERNOR-QUICKENED-SUBSTRATE seed=1 action=doRejectUnsupportedSecondOption#step:doRejectUnsupportedSecondOption | pass | covered |
| cleanroom-input/qnt/battle-runtime/battle-runtime-quickened-spell-governor.mbt.qnt#step:doRejectOnePerSpell | tasks/target-replay-evidence/L15-RR07-FU08A-METAMAGIC-GOVERNOR-QUICKENED-SUBSTRATE.json#L15-RR07-FU08A-METAMAGIC-GOVERNOR-QUICKENED-SUBSTRATE seed=1 action=doRejectOnePerSpell#step:doRejectOnePerSpell | pass | covered |
| cleanroom-input/qnt/battle-runtime/battle-runtime-quickened-spell-governor.mbt.qnt#step:doRejectPriorLevelOnePlusSpell | tasks/target-replay-evidence/L15-RR07-FU08A-METAMAGIC-GOVERNOR-QUICKENED-SUBSTRATE.json#L15-RR07-FU08A-METAMAGIC-GOVERNOR-QUICKENED-SUBSTRATE seed=1 action=doRejectPriorLevelOnePlusSpell#step:doRejectPriorLevelOnePlusSpell | pass | covered |
| cleanroom-input/qnt/battle-runtime/battle-runtime-sorcerer-metamagic-selected-identity.mbt.qnt#step:doResolveQuickenedSaveGatedDamage | tasks/target-replay-evidence/L15-RR07-FU08A-METAMAGIC-GOVERNOR-QUICKENED-SUBSTRATE.json#L15-RR07-FU08A-METAMAGIC-GOVERNOR-QUICKENED-SUBSTRATE seed=1 action=doResolveQuickenedSaveGatedDamage#step:doResolveQuickenedSaveGatedDamage | pass | covered |
| cleanroom-input/qnt/battle-runtime/battle-runtime-sorcerer-metamagic-spell-attack-selected-identity.mbt.qnt#step:doResolveQuickenedSpellAttack | tasks/target-replay-evidence/L15-RR07-FU08A-METAMAGIC-GOVERNOR-QUICKENED-SUBSTRATE.json#L15-RR07-FU08A-METAMAGIC-GOVERNOR-QUICKENED-SUBSTRATE seed=1 action=doResolveQuickenedSpellAttack#step:doResolveQuickenedSpellAttack | pass | covered |
| cleanroom-input/qnt/battle-runtime/battle-runtime-sorcerer-metamagic-spell-attack-sequence-selected-identity.mbt.qnt#step:doResolveQuickenedSpellAttackSequence | tasks/target-replay-evidence/L15-RR07-FU08A-METAMAGIC-GOVERNOR-QUICKENED-SUBSTRATE.json#L15-RR07-FU08A-METAMAGIC-GOVERNOR-QUICKENED-SUBSTRATE seed=1 action=doResolveQuickenedSpellAttackSequence#step:doResolveQuickenedSpellAttackSequence | pass | covered |
| cleanroom-input/qnt/battle-runtime/battle-runtime-condition-saving-throw-selected-identity.mbt.qnt#step:doResolveColorSprayFailedSavingThrow | tasks/target-replay-evidence/L15-RR07-FU01A-catalog-ready-spell-substrates.json#L15-RR07-FU01A-CATALOG-READY-SPELL-SUBSTRATES route action=doResolveColorSprayFailedSavingThrow#step:doResolveColorSprayFailedSavingThrow | pass | covered |
| cleanroom-input/qnt/battle-runtime/battle-runtime-condition-saving-throw-selected-identity.mbt.qnt#step:doResolveEntangleFailedSavingThrow | tasks/target-replay-evidence/L15-RR07-FU01A-catalog-ready-spell-substrates.json#L15-RR07-FU01A-CATALOG-READY-SPELL-SUBSTRATES route action=doResolveEntangleFailedSavingThrow#step:doResolveEntangleFailedSavingThrow | pass | covered |
| cleanroom-input/qnt/battle-runtime/battle-runtime-condition-saving-throw-selected-identity.mbt.qnt#step:doResolveHideousLaughterRepeatSavingThrowSuccess | tasks/target-replay-evidence/L15-RR07-FU01A-catalog-ready-spell-substrates.json#L15-RR07-FU01A-CATALOG-READY-SPELL-SUBSTRATES route action=doResolveHideousLaughterRepeatSavingThrowSuccess#step:doResolveHideousLaughterRepeatSavingThrowSuccess | pass | covered |
| cleanroom-input/qnt/battle-runtime/battle-runtime-condition-saving-throw-selected-identity.mbt.qnt#step:doResolveSleepRepeatSavingThrowFailure | tasks/target-replay-evidence/L15-RR07-FU01A-catalog-ready-spell-substrates.json#L15-RR07-FU01A-CATALOG-READY-SPELL-SUBSTRATES route action=doResolveSleepRepeatSavingThrowFailure#step:doResolveSleepRepeatSavingThrowFailure | pass | covered |
| cleanroom-input/qnt/battle-runtime/battle-runtime-find-familiar-selected-identity.mbt.qnt#step:doCastFindFamiliar | tasks/target-replay-evidence/L15-RR07-FU01A-catalog-ready-spell-substrates.json#L15-RR07-FU01A-CATALOG-READY-SPELL-SUBSTRATES route action=doCastFindFamiliar#step:doCastFindFamiliar | pass | covered |
| cleanroom-input/qnt/battle-runtime/battle-runtime-find-familiar-selected-identity.mbt.qnt#step:doDeliverTouchSpellThroughFindFamiliar | tasks/target-replay-evidence/L15-RR07-FU01A-catalog-ready-spell-substrates.json#L15-RR07-FU01A-CATALOG-READY-SPELL-SUBSTRATES route action=doDeliverTouchSpellThroughFindFamiliar#step:doDeliverTouchSpellThroughFindFamiliar | pass | covered |
| cleanroom-input/qnt/battle-runtime/battle-runtime-find-familiar-selected-identity.mbt.qnt#step:doDismissAndReappearFindFamiliar | tasks/target-replay-evidence/L15-RR07-FU01A-catalog-ready-spell-substrates.json#L15-RR07-FU01A-CATALOG-READY-SPELL-SUBSTRATES route action=doDismissAndReappearFindFamiliar#step:doDismissAndReappearFindFamiliar | pass | covered |
| cleanroom-input/qnt/battle-runtime/battle-runtime-find-familiar-selected-identity.mbt.qnt#step:doRecastFindFamiliarReplacement | tasks/target-replay-evidence/L15-RR07-FU01A-catalog-ready-spell-substrates.json#L15-RR07-FU01A-CATALOG-READY-SPELL-SUBSTRATES route action=doRecastFindFamiliarReplacement#step:doRecastFindFamiliarReplacement | pass | covered |
| cleanroom-input/qnt/battle-runtime/battle-runtime-thaumaturgy-selected-identity.mbt.qnt#step:doResolveThaumaturgyBoomingVoice | tasks/target-replay-evidence/L15-RR07-FU01A-catalog-ready-spell-substrates.json#L15-RR07-FU01A-CATALOG-READY-SPELL-SUBSTRATES route action=doResolveThaumaturgyBoomingVoice#step:doResolveThaumaturgyBoomingVoice | pass | covered |
| cleanroom-input/qnt/battle-runtime/battle-runtime-sorcerer-metamagic-careful-selected-identity.mbt.qnt#step:doResolveCarefulSaveGatedDamage | tasks/target-replay-evidence/L15-RR07-FU08B-METAMAGIC-SAVE-RANGE-TARGET-SUBSTRATES.json#L15-RR07-FU08B-METAMAGIC-SAVE-RANGE-TARGET-SUBSTRATES seed=1 action=doResolveCarefulSaveGatedDamage#step:doResolveCarefulSaveGatedDamage | pass | covered |
| cleanroom-input/qnt/battle-runtime/battle-runtime-sorcerer-metamagic-careful-selected-identity.mbt.qnt#step:doResolveCarefulSaveGatedNoEffect | tasks/target-replay-evidence/L15-RR07-FU08B-METAMAGIC-SAVE-RANGE-TARGET-SUBSTRATES.json#L15-RR07-FU08B-METAMAGIC-SAVE-RANGE-TARGET-SUBSTRATES seed=1 action=doResolveCarefulSaveGatedNoEffect#step:doResolveCarefulSaveGatedNoEffect | pass | covered |
| cleanroom-input/qnt/battle-runtime/battle-runtime-sorcerer-metamagic-heightened-selected-identity.mbt.qnt#step:doResolveHeightenedSaveGatedDamage | tasks/target-replay-evidence/L15-RR07-FU08B-METAMAGIC-SAVE-RANGE-TARGET-SUBSTRATES.json#L15-RR07-FU08B-METAMAGIC-SAVE-RANGE-TARGET-SUBSTRATES seed=1 action=doResolveHeightenedSaveGatedDamage#step:doResolveHeightenedSaveGatedDamage | pass | covered |
| cleanroom-input/qnt/battle-runtime/battle-runtime-sorcerer-metamagic-heightened-selected-identity.mbt.qnt#step:doResolveHeightenedHideousLaughter | tasks/target-replay-evidence/L15-RR07-FU08B-METAMAGIC-SAVE-RANGE-TARGET-SUBSTRATES.json#L15-RR07-FU08B-METAMAGIC-SAVE-RANGE-TARGET-SUBSTRATES seed=1 action=doResolveHeightenedHideousLaughter#step:doResolveHeightenedHideousLaughter | pass | covered |
| cleanroom-input/qnt/battle-runtime/battle-runtime-sorcerer-metamagic-heightened-selected-identity.mbt.qnt#step:doResolveHeightenedGreaseEntrySave | tasks/target-replay-evidence/L15-RR07-FU08B-METAMAGIC-SAVE-RANGE-TARGET-SUBSTRATES.json#L15-RR07-FU08B-METAMAGIC-SAVE-RANGE-TARGET-SUBSTRATES seed=1 action=doResolveHeightenedGreaseEntrySave#step:doResolveHeightenedGreaseEntrySave | pass | covered |
| cleanroom-input/qnt/battle-runtime/battle-runtime-sorcerer-metamagic-heightened-selected-identity.mbt.qnt#step:doResolveHeightenedGustOfWindEndTurnSave | tasks/target-replay-evidence/L15-RR07-FU08B-METAMAGIC-SAVE-RANGE-TARGET-SUBSTRATES.json#L15-RR07-FU08B-METAMAGIC-SAVE-RANGE-TARGET-SUBSTRATES seed=1 action=doResolveHeightenedGustOfWindEndTurnSave#step:doResolveHeightenedGustOfWindEndTurnSave | pass | covered |
| cleanroom-input/qnt/battle-runtime/battle-runtime-sorcerer-metamagic-heightened-selected-identity.mbt.qnt#step:doResolveHeightenedSaveGatedConditionEndTurnSave | tasks/target-replay-evidence/L15-RR07-FU08B-METAMAGIC-SAVE-RANGE-TARGET-SUBSTRATES.json#L15-RR07-FU08B-METAMAGIC-SAVE-RANGE-TARGET-SUBSTRATES seed=1 action=doResolveHeightenedSaveGatedConditionEndTurnSave#step:doResolveHeightenedSaveGatedConditionEndTurnSave | pass | covered |
| cleanroom-input/qnt/battle-runtime/battle-runtime-sorcerer-metamagic-distant-selected-identity.mbt.qnt#step:doResolveDistantObjectLight | tasks/target-replay-evidence/L15-RR07-FU08B-METAMAGIC-SAVE-RANGE-TARGET-SUBSTRATES.json#L15-RR07-FU08B-METAMAGIC-SAVE-RANGE-TARGET-SUBSTRATES seed=1 action=doResolveDistantObjectLight#step:doResolveDistantObjectLight | pass | covered |
| cleanroom-input/qnt/battle-runtime/battle-runtime-sorcerer-metamagic-twinned-selected-identity.mbt.qnt#step:doResolveTwinnedTargetCount | tasks/target-replay-evidence/L15-RR07-FU08B-METAMAGIC-SAVE-RANGE-TARGET-SUBSTRATES.json#L15-RR07-FU08B-METAMAGIC-SAVE-RANGE-TARGET-SUBSTRATES seed=1 action=doResolveTwinnedTargetCount#step:doResolveTwinnedTargetCount | pass | covered |
| cleanroom-input/qnt/battle-runtime/battle-runtime-sorcerer-metamagic-empowered-selected-identity.mbt.qnt#step:doResolveEmpoweredSpellDamageReroll | tasks/target-replay-evidence/L15-RR07-FU08C-metamagic-reroll-damage-projection-substrates.json#L15-RR07-FU08C-METAMAGIC-REROLL-DAMAGE-PROJECTION-SUBSTRATES seed=1 action=doResolveEmpoweredSpellDamageReroll#step:doResolveEmpoweredSpellDamageReroll | pass | covered |
| cleanroom-input/qnt/battle-runtime/battle-runtime-sorcerer-metamagic-extended-selected-identity.mbt.qnt#step:doResolveExtendedCreatureSizeIncrease | tasks/target-replay-evidence/L15-RR07-FU08C-metamagic-reroll-damage-projection-substrates.json#L15-RR07-FU08C-METAMAGIC-REROLL-DAMAGE-PROJECTION-SUBSTRATES seed=1 action=doResolveExtendedCreatureSizeIncrease#step:doResolveExtendedCreatureSizeIncrease | pass | covered |
| cleanroom-input/qnt/battle-runtime/battle-runtime-sorcerer-metamagic-seeking-selected-identity.mbt.qnt#step:doResolveSeekingSpellAttackReroll | tasks/target-replay-evidence/L15-RR07-FU08C-metamagic-reroll-damage-projection-substrates.json#L15-RR07-FU08C-METAMAGIC-REROLL-DAMAGE-PROJECTION-SUBSTRATES seed=1 action=doResolveSeekingSpellAttackReroll#step:doResolveSeekingSpellAttackReroll | pass | covered |
| cleanroom-input/qnt/battle-runtime/battle-runtime-sorcerer-metamagic-subtle-selected-identity.mbt.qnt#step:doRejectSubtleFalseLifeWithoutSorceryPoints | tasks/target-replay-evidence/L15-RR07-FU08C-metamagic-reroll-damage-projection-substrates.json#L15-RR07-FU08C-METAMAGIC-REROLL-DAMAGE-PROJECTION-SUBSTRATES seed=1 action=doRejectSubtleFalseLifeWithoutSorceryPoints#step:doRejectSubtleFalseLifeWithoutSorceryPoints | pass | covered |
| cleanroom-input/qnt/battle-runtime/battle-runtime-sorcerer-metamagic-subtle-selected-identity.mbt.qnt#step:doResolveSubtleFalseLife | tasks/target-replay-evidence/L15-RR07-FU08C-metamagic-reroll-damage-projection-substrates.json#L15-RR07-FU08C-METAMAGIC-REROLL-DAMAGE-PROJECTION-SUBSTRATES seed=1 action=doResolveSubtleFalseLife#step:doResolveSubtleFalseLife | pass | covered |
| cleanroom-input/qnt/battle-runtime/battle-runtime-sorcerer-metamagic-transmuted-selected-identity.mbt.qnt#step:doResolveTransmutedSaveGatedDamage | tasks/target-replay-evidence/L15-RR07-FU08C-metamagic-reroll-damage-projection-substrates.json#L15-RR07-FU08C-METAMAGIC-REROLL-DAMAGE-PROJECTION-SUBSTRATES seed=1 action=doResolveTransmutedSaveGatedDamage#step:doResolveTransmutedSaveGatedDamage | pass | covered |
| cleanroom-input/qnt/battle-runtime/battle-runtime-sorcerer-metamagic-transmuted-selected-identity.mbt.qnt#step:doResolveTransmutedSpellAttack | tasks/target-replay-evidence/L15-RR07-FU08C-metamagic-reroll-damage-projection-substrates.json#L15-RR07-FU08C-METAMAGIC-REROLL-DAMAGE-PROJECTION-SUBSTRATES seed=1 action=doResolveTransmutedSpellAttack#step:doResolveTransmutedSpellAttack | pass | covered |
| cleanroom-input/qnt/battle-runtime/battle-runtime-attack-spell-shape-selected-identity.mbt.qnt#step:doChillTouchHitPointRegainPrevention | tasks/target-replay-evidence/L15-RR07-FU01B-SPELL-ATTACK-SAVE-DAMAGE-SUBSTRATES.json#L15-RR07-FU01B-SPELL-ATTACK-SAVE-DAMAGE-SUBSTRATES route action=doChillTouchHitPointRegainPrevention#step:doChillTouchHitPointRegainPrevention | pass | covered |
| cleanroom-input/qnt/battle-runtime/battle-runtime-attack-spell-shape-selected-identity.mbt.qnt#step:doFireBoltHit | tasks/target-replay-evidence/L15-RR07-FU01B-SPELL-ATTACK-SAVE-DAMAGE-SUBSTRATES.json#L15-RR07-FU01B-SPELL-ATTACK-SAVE-DAMAGE-SUBSTRATES route action=doFireBoltHit#step:doFireBoltHit | pass | covered |
| cleanroom-input/qnt/battle-runtime/battle-runtime-attack-spell-shape-selected-identity.mbt.qnt#step:doGuidingBoltNextAttackAdvantage | tasks/target-replay-evidence/L15-RR07-FU01B-SPELL-ATTACK-SAVE-DAMAGE-SUBSTRATES.json#L15-RR07-FU01B-SPELL-ATTACK-SAVE-DAMAGE-SUBSTRATES route action=doGuidingBoltNextAttackAdvantage#step:doGuidingBoltNextAttackAdvantage | pass | covered |
| cleanroom-input/qnt/battle-runtime/battle-runtime-attack-spell-shape-selected-identity.mbt.qnt#step:doInflictWoundsFailedSave | tasks/target-replay-evidence/L15-RR07-FU01B-SPELL-ATTACK-SAVE-DAMAGE-SUBSTRATES.json#L15-RR07-FU01B-SPELL-ATTACK-SAVE-DAMAGE-SUBSTRATES route action=doInflictWoundsFailedSave#step:doInflictWoundsFailedSave | pass | covered |
| cleanroom-input/qnt/battle-runtime/battle-runtime-attack-spell-shape-selected-identity.mbt.qnt#step:doInflictWoundsSuccessfulSave | tasks/target-replay-evidence/L15-RR07-FU01B-SPELL-ATTACK-SAVE-DAMAGE-SUBSTRATES.json#L15-RR07-FU01B-SPELL-ATTACK-SAVE-DAMAGE-SUBSTRATES route action=doInflictWoundsSuccessfulSave#step:doInflictWoundsSuccessfulSave | pass | covered |
| cleanroom-input/qnt/battle-runtime/battle-runtime-attack-spell-shape-selected-identity.mbt.qnt#step:doShockingGraspOpportunityAttackDenied | tasks/target-replay-evidence/L15-RR07-FU01B-SPELL-ATTACK-SAVE-DAMAGE-SUBSTRATES.json#L15-RR07-FU01B-SPELL-ATTACK-SAVE-DAMAGE-SUBSTRATES route action=doShockingGraspOpportunityAttackDenied#step:doShockingGraspOpportunityAttackDenied | pass | covered |
| cleanroom-input/qnt/battle-runtime/battle-runtime-level1-damage-spell-selected-identity.mbt.qnt#step:doResolveBurningHandsMixedConeSavingThrows | tasks/target-replay-evidence/L15-RR07-FU01B-SPELL-ATTACK-SAVE-DAMAGE-SUBSTRATES.json#L15-RR07-FU01B-SPELL-ATTACK-SAVE-DAMAGE-SUBSTRATES route action=doResolveBurningHandsMixedConeSavingThrows#step:doResolveBurningHandsMixedConeSavingThrows | pass | covered |
| cleanroom-input/qnt/battle-runtime/battle-runtime-level1-damage-spell-selected-identity.mbt.qnt#step:doResolveIceKnifeHitAttackDamageAndBurstSavingThrows | tasks/target-replay-evidence/L15-RR07-FU01B-SPELL-ATTACK-SAVE-DAMAGE-SUBSTRATES.json#L15-RR07-FU01B-SPELL-ATTACK-SAVE-DAMAGE-SUBSTRATES route action=doResolveIceKnifeHitAttackDamageAndBurstSavingThrows#step:doResolveIceKnifeHitAttackDamageAndBurstSavingThrows | pass | covered |
| cleanroom-input/qnt/battle-runtime/battle-runtime-level1-damage-spell-selected-identity.mbt.qnt#step:doResolveIceKnifeMissBurstSavingThrows | tasks/target-replay-evidence/L15-RR07-FU01B-SPELL-ATTACK-SAVE-DAMAGE-SUBSTRATES.json#L15-RR07-FU01B-SPELL-ATTACK-SAVE-DAMAGE-SUBSTRATES route action=doResolveIceKnifeMissBurstSavingThrows#step:doResolveIceKnifeMissBurstSavingThrows | pass | covered |
| cleanroom-input/qnt/battle-runtime/battle-runtime-level1-damage-spell-selected-identity.mbt.qnt#step:doResolvePoisonSpraySpellAttackDamage | tasks/target-replay-evidence/L15-RR07-FU01B-SPELL-ATTACK-SAVE-DAMAGE-SUBSTRATES.json#L15-RR07-FU01B-SPELL-ATTACK-SAVE-DAMAGE-SUBSTRATES route action=doResolvePoisonSpraySpellAttackDamage#step:doResolvePoisonSpraySpellAttackDamage | pass | covered |
| cleanroom-input/qnt/battle-runtime/battle-runtime-level1-damage-spell-selected-identity.mbt.qnt#step:doResolveRayOfSicknessSpellAttackDamageAndPoisoned | tasks/target-replay-evidence/L15-RR07-FU01B-SPELL-ATTACK-SAVE-DAMAGE-SUBSTRATES.json#L15-RR07-FU01B-SPELL-ATTACK-SAVE-DAMAGE-SUBSTRATES route action=doResolveRayOfSicknessSpellAttackDamageAndPoisoned#step:doResolveRayOfSicknessSpellAttackDamageAndPoisoned | pass | covered |
| cleanroom-input/qnt/battle-runtime/battle-runtime-level1-damage-spell-selected-identity.mbt.qnt#step:doResolveSacredFlameDexteritySavingThrowRadiantDamage | tasks/target-replay-evidence/L15-RR07-FU01B-SPELL-ATTACK-SAVE-DAMAGE-SUBSTRATES.json#L15-RR07-FU01B-SPELL-ATTACK-SAVE-DAMAGE-SUBSTRATES route action=doResolveSacredFlameDexteritySavingThrowRadiantDamage#step:doResolveSacredFlameDexteritySavingThrowRadiantDamage | pass | covered |
| cleanroom-input/qnt/battle-runtime/battle-runtime-level1-damage-spell-selected-identity.mbt.qnt#step:doResolveSorcerousBurstSpellAttackDamage | tasks/target-replay-evidence/L15-RR07-FU01B-SPELL-ATTACK-SAVE-DAMAGE-SUBSTRATES.json#L15-RR07-FU01B-SPELL-ATTACK-SAVE-DAMAGE-SUBSTRATES route action=doResolveSorcerousBurstSpellAttackDamage#step:doResolveSorcerousBurstSpellAttackDamage | pass | covered |
| cleanroom-input/qnt/battle-runtime/battle-runtime-level1-damage-spell-selected-identity.mbt.qnt#step:doResolveViciousMockeryWisdomSavingThrowPsychicDamageAndNextAttackDisadvantage | tasks/target-replay-evidence/L15-RR07-FU01B-SPELL-ATTACK-SAVE-DAMAGE-SUBSTRATES.json#L15-RR07-FU01B-SPELL-ATTACK-SAVE-DAMAGE-SUBSTRATES route action=doResolveViciousMockeryWisdomSavingThrowPsychicDamageAndNextAttackDisadvantage#step:doResolveViciousMockeryWisdomSavingThrowPsychicDamageAndNextAttackDisadvantage | pass | covered |
| cleanroom-input/qnt/battle-runtime/battle-runtime-level1-damage-spell-selected-identity.mbt.qnt#step:doResolveChromaticOrbDuplicateDamageLeap | tasks/target-replay-evidence/L15-RR07-FU01B-SPELL-ATTACK-SAVE-DAMAGE-SUBSTRATES.json#L15-RR07-FU01B-SPELL-ATTACK-SAVE-DAMAGE-SUBSTRATES blocker action=doResolveChromaticOrbDuplicateDamageLeap#step:doResolveChromaticOrbDuplicateDamageLeap | FU01B-BLOCKED-CHROMATIC-ORB-CHAINED-DAMAGE | blocked |
| cleanroom-input/qnt/battle-runtime/battle-runtime-level1-damage-spell-selected-identity.mbt.qnt#step:doResolveStarryWispObjectSpellAttackDamageAndDimLight | tasks/target-replay-evidence/L15-RR07-FU01B-SPELL-ATTACK-SAVE-DAMAGE-SUBSTRATES.json#L15-RR07-FU01B-SPELL-ATTACK-SAVE-DAMAGE-SUBSTRATES blocker action=doResolveStarryWispObjectSpellAttackDamageAndDimLight#step:doResolveStarryWispObjectSpellAttackDamageAndDimLight | FU01B-BLOCKED-STARRY-WISP-OBJECT-TARGET | blocked |
| cleanroom-input/qnt/battle-runtime/battle-runtime-level1-buff-mark-smite-selected-identity.mbt.qnt#step:doDivineFavorWeaponDamageRider | tasks/target-replay-evidence/L15-RR07-FU01C-WEAPON-BUFF-MARK-SMITE-SUBSTRATES.json#L15-RR07-FU01C-WEAPON-BUFF-MARK-SMITE-SUBSTRATES route action=doDivineFavorWeaponDamageRider#step:doDivineFavorWeaponDamageRider | pass | covered |
| cleanroom-input/qnt/battle-runtime/battle-runtime-level1-buff-mark-smite-selected-identity.mbt.qnt#step:doDivineSmiteAfterHitDamage | tasks/target-replay-evidence/L15-RR07-FU01C-WEAPON-BUFF-MARK-SMITE-SUBSTRATES.json#L15-RR07-FU01C-WEAPON-BUFF-MARK-SMITE-SUBSTRATES route action=doDivineSmiteAfterHitDamage#step:doDivineSmiteAfterHitDamage | pass | covered |
| cleanroom-input/qnt/battle-runtime/battle-runtime-level1-buff-mark-smite-selected-identity.mbt.qnt#step:doEnsnaringStrikeAfterHitRestraintTurnStartDamageAndEscape | tasks/target-replay-evidence/L15-RR07-FU01C-WEAPON-BUFF-MARK-SMITE-SUBSTRATES.json#L15-RR07-FU01C-WEAPON-BUFF-MARK-SMITE-SUBSTRATES route action=doEnsnaringStrikeAfterHitRestraintTurnStartDamageAndEscape#step:doEnsnaringStrikeAfterHitRestraintTurnStartDamageAndEscape | pass | covered |
| cleanroom-input/qnt/battle-runtime/battle-runtime-level1-buff-mark-smite-selected-identity.mbt.qnt#step:doFalseLifeTemporaryHitPoints | tasks/target-replay-evidence/L15-RR07-FU01C-WEAPON-BUFF-MARK-SMITE-SUBSTRATES.json#L15-RR07-FU01C-WEAPON-BUFF-MARK-SMITE-SUBSTRATES route action=doFalseLifeTemporaryHitPoints#step:doFalseLifeTemporaryHitPoints | pass | covered |
| cleanroom-input/qnt/battle-runtime/battle-runtime-level1-buff-mark-smite-selected-identity.mbt.qnt#step:doLongstriderSpeedIncrease | tasks/target-replay-evidence/L15-RR07-FU01C-WEAPON-BUFF-MARK-SMITE-SUBSTRATES.json#L15-RR07-FU01C-WEAPON-BUFF-MARK-SMITE-SUBSTRATES route action=doLongstriderSpeedIncrease#step:doLongstriderSpeedIncrease | pass | covered |
| cleanroom-input/qnt/battle-runtime/battle-runtime-level1-buff-mark-smite-selected-identity.mbt.qnt#step:doSearingSmiteAfterHitTimedDamageAndSaveCleanup | tasks/target-replay-evidence/L15-RR07-FU01C-WEAPON-BUFF-MARK-SMITE-SUBSTRATES.json#L15-RR07-FU01C-WEAPON-BUFF-MARK-SMITE-SUBSTRATES route action=doSearingSmiteAfterHitTimedDamageAndSaveCleanup#step:doSearingSmiteAfterHitTimedDamageAndSaveCleanup | pass | covered |
| cleanroom-input/qnt/battle-runtime/battle-runtime-level1-buff-mark-smite-selected-identity.mbt.qnt#step:doShillelaghWeaponAttackOverride | tasks/target-replay-evidence/L15-RR07-FU01C-WEAPON-BUFF-MARK-SMITE-SUBSTRATES.json#L15-RR07-FU01C-WEAPON-BUFF-MARK-SMITE-SUBSTRATES route action=doShillelaghWeaponAttackOverride#step:doShillelaghWeaponAttackOverride | pass | covered |
| cleanroom-input/qnt/battle-runtime/battle-runtime-level1-buff-mark-smite-selected-identity.mbt.qnt#step:doTrueStrikeSpellHostedWeaponAttack | tasks/target-replay-evidence/L15-RR07-FU01C-WEAPON-BUFF-MARK-SMITE-SUBSTRATES.json#L15-RR07-FU01C-WEAPON-BUFF-MARK-SMITE-SUBSTRATES route action=doTrueStrikeSpellHostedWeaponAttack#step:doTrueStrikeSpellHostedWeaponAttack | pass | covered |
| cleanroom-input/qnt/battle-runtime/battle-runtime-level1-buff-mark-smite-selected-identity.mbt.qnt#step:doHeroismFrightenedImmunityTurnStartTemporaryHitPoints | tasks/target-replay-evidence/L15-RR07-FU01C-WEAPON-BUFF-MARK-SMITE-SUBSTRATES.json#L15-RR07-FU01C-WEAPON-BUFF-MARK-SMITE-SUBSTRATES blocker action=doHeroismFrightenedImmunityTurnStartTemporaryHitPoints#step:doHeroismFrightenedImmunityTurnStartTemporaryHitPoints | FU01C-BLOCKED-HEROISM-CONDITION-IMMUNITY-TEMP-HP | blocked |
| cleanroom-input/qnt/battle-runtime/battle-runtime-level1-buff-mark-smite-selected-identity.mbt.qnt#step:doHeroismFrightenedImmunityTurnStartTemporaryHitPointsCleanup | tasks/target-replay-evidence/L15-RR07-FU01C-WEAPON-BUFF-MARK-SMITE-SUBSTRATES.json#L15-RR07-FU01C-WEAPON-BUFF-MARK-SMITE-SUBSTRATES blocker action=doHeroismFrightenedImmunityTurnStartTemporaryHitPointsCleanup#step:doHeroismFrightenedImmunityTurnStartTemporaryHitPointsCleanup | FU01C-BLOCKED-HEROISM-CONDITION-IMMUNITY-CLEANUP | blocked |
| cleanroom-input/qnt/battle-runtime/battle-runtime-level1-buff-mark-smite-selected-identity.mbt.qnt#step:doHuntersMarkMarkedDamageRiderConcentrationAndSameTurnTransfer | tasks/target-replay-evidence/L15-RR07-FU01C-WEAPON-BUFF-MARK-SMITE-SUBSTRATES.json#L15-RR07-FU01C-WEAPON-BUFF-MARK-SMITE-SUBSTRATES blocker action=doHuntersMarkMarkedDamageRiderConcentrationAndSameTurnTransfer#step:doHuntersMarkMarkedDamageRiderConcentrationAndSameTurnTransfer | FU01C-BLOCKED-HUNTERS-MARK-TRANSFER | blocked |
| cleanroom-input/qnt/battle-runtime/battle-runtime-level1-buff-mark-smite-selected-identity.mbt.qnt#step:doHexMarkedDamageRiderAndLaterTurnTransfer | tasks/target-replay-evidence/L15-RR07-FU01C-WEAPON-BUFF-MARK-SMITE-SUBSTRATES.json#L15-RR07-FU01C-WEAPON-BUFF-MARK-SMITE-SUBSTRATES blocker action=doHexMarkedDamageRiderAndLaterTurnTransfer#step:doHexMarkedDamageRiderAndLaterTurnTransfer | FU01C-BLOCKED-HEX-TRANSFER | blocked |

## L15-RRCP5-F-INDEPENDENT-SPELL-ATTACK-SEQUENCE-ROUTES

- Manifest source commit SHA: `564376fd95218a209bb9eae5c9ccb54ca3e04a52`
- Source branch inventory SHA: `4bb2b20a85d94e3b90b7c59cbfe6e1edd5ab3ef40410641e999527861f3d3a32`
- Machine-readable ledger: `tasks/RUN_LEDGER.json`
- Selected drivers:
  - `cleanroom-input/qnt/battle-runtime/battle-runtime-chained-attack-sequence.mbt.qnt`
  - `cleanroom-input/qnt/battle-runtime/battle-runtime-eldritch-blast.mbt.qnt`

Allowed inputs used:
- `cleanroom-input/qnt/battle-runtime/battle-runtime-chained-attack-sequence.mbt.qnt`
- `cleanroom-input/qnt/battle-runtime/battle-runtime-chained-attack-sequence.route.mbt.qnt`
- `cleanroom-input/qnt/battle-runtime/battle-runtime-eldritch-blast.mbt.qnt`
- `cleanroom-input/qnt/battle-runtime/battle-runtime-eldritch-blast.route.mbt.qnt`
- `cleanroom-input/branch-coverage/source-branch-inventory.json`
- `cleanroom-input/branch-coverage/reducer-route-inventory.json`
- `cleanroom-input/raw/srd-5.2.1` and `cleanroom-input/domain/UBIQUITOUS_LANGUAGE.md` excerpts named by the reducer-route inventory

Behavior implemented:
- Chained spell attack sequence route fills use reducer-owned SpellAttack procedure route subjects for damage type, target choice, attack roll, damage, and continuation.
- Independent spell attack sequence route fills use reducer-owned SpellAttack procedure route subjects for target choice, object boundary discovery, per-attack resolution, damage, continuation, and stale replay rejection.
- Production route emission is keyed by reducer subject shape, generic fills, and route-owner facts rather than authored spell identity.

Generated branch coverage:
- L15-RR16: 10 covered, 0 blocked.
- L15-RR22: 8 covered, 0 blocked.

Target replay evidence:
| Obligation | Target replay evidence | Diagnostic tests | Status |
| --- | --- | --- | --- |
| `cleanroom-input/qnt/battle-runtime/battle-runtime-chained-attack-sequence.mbt.qnt#step:doStartCast` | `tasks/target-replay-evidence/L15-RR16-chained-attack-procedure-routes.json#L15-RR16 chained spell attack procedure route adapter seed=1 action=doStartCast#step:doStartCast` | `src/tests/mod.rs` | `covered` |
| `cleanroom-input/qnt/battle-runtime/battle-runtime-chained-attack-sequence.mbt.qnt#step:doChooseDamageType` | `tasks/target-replay-evidence/L15-RR16-chained-attack-procedure-routes.json#L15-RR16 chained spell attack procedure route adapter seed=1 action=doChooseDamageType#step:doChooseDamageType` | `src/tests/mod.rs` | `covered` |
| `cleanroom-input/qnt/battle-runtime/battle-runtime-chained-attack-sequence.mbt.qnt#step:doChooseInitialTarget` | `tasks/target-replay-evidence/L15-RR16-chained-attack-procedure-routes.json#L15-RR16 chained spell attack procedure route adapter seed=1 action=doChooseInitialTarget#step:doChooseInitialTarget` | `src/tests/mod.rs` | `covered` |
| `cleanroom-input/qnt/battle-runtime/battle-runtime-chained-attack-sequence.mbt.qnt#step:doResolveStep0AttackHit` | `tasks/target-replay-evidence/L15-RR16-chained-attack-procedure-routes.json#L15-RR16 chained spell attack procedure route adapter seed=1 action=doResolveStep0AttackHit#step:doResolveStep0AttackHit` | `src/tests/mod.rs` | `covered` |
| `cleanroom-input/qnt/battle-runtime/battle-runtime-chained-attack-sequence.mbt.qnt#step:doResolveStep0DamageNoDuplicate` | `tasks/target-replay-evidence/L15-RR16-chained-attack-procedure-routes.json#L15-RR16 chained spell attack procedure route adapter seed=1 action=doResolveStep0DamageNoDuplicate#step:doResolveStep0DamageNoDuplicate` | `src/tests/mod.rs` | `covered` |
| `cleanroom-input/qnt/battle-runtime/battle-runtime-chained-attack-sequence.mbt.qnt#step:doResolveStep0DamageDuplicate` | `tasks/target-replay-evidence/L15-RR16-chained-attack-procedure-routes.json#L15-RR16 chained spell attack procedure route adapter seed=1 action=doResolveStep0DamageDuplicate#step:doResolveStep0DamageDuplicate` | `src/tests/mod.rs` | `covered` |
| `cleanroom-input/qnt/battle-runtime/battle-runtime-chained-attack-sequence.mbt.qnt#step:doChooseFirstLeapTarget` | `tasks/target-replay-evidence/L15-RR16-chained-attack-procedure-routes.json#L15-RR16 chained spell attack procedure route adapter seed=1 action=doChooseFirstLeapTarget#step:doChooseFirstLeapTarget` | `src/tests/mod.rs` | `covered` |
| `cleanroom-input/qnt/battle-runtime/battle-runtime-chained-attack-sequence.mbt.qnt#step:doResolveStep1AttackHit` | `tasks/target-replay-evidence/L15-RR16-chained-attack-procedure-routes.json#L15-RR16 chained spell attack procedure route adapter seed=1 action=doResolveStep1AttackHit#step:doResolveStep1AttackHit` | `src/tests/mod.rs` | `covered` |
| `cleanroom-input/qnt/battle-runtime/battle-runtime-chained-attack-sequence.mbt.qnt#step:doResolveStep1DuplicateDamageSlot1Limit` | `tasks/target-replay-evidence/L15-RR16-chained-attack-procedure-routes.json#L15-RR16 chained spell attack procedure route adapter seed=1 action=doResolveStep1DuplicateDamageSlot1Limit#step:doResolveStep1DuplicateDamageSlot1Limit` | `src/tests/mod.rs` | `covered` |
| `cleanroom-input/qnt/battle-runtime/battle-runtime-chained-attack-sequence.mbt.qnt#step:doResolveStep1DuplicateDamageSlot2AllowsLeap` | `tasks/target-replay-evidence/L15-RR16-chained-attack-procedure-routes.json#L15-RR16 chained spell attack procedure route adapter seed=1 action=doResolveStep1DuplicateDamageSlot2AllowsLeap#step:doResolveStep1DuplicateDamageSlot2AllowsLeap` | `src/tests/mod.rs` | `covered` |
| `cleanroom-input/qnt/battle-runtime/battle-runtime-eldritch-blast.mbt.qnt#step:doFillFirstAttackHit` | `tasks/target-replay-evidence/L15-RR22-independent-spell-attack-sequence-routes.json#L15-RR22 independent spell attack sequence route adapter seed=1 action=doFillFirstAttackHit#step:doFillFirstAttackHit` | `src/tests/mod.rs` | `covered` |
| `cleanroom-input/qnt/battle-runtime/battle-runtime-eldritch-blast.mbt.qnt#step:doFillFirstAttackMiss` | `tasks/target-replay-evidence/L15-RR22-independent-spell-attack-sequence-routes.json#L15-RR22 independent spell attack sequence route adapter seed=1 action=doFillFirstAttackMiss#step:doFillFirstAttackMiss` | `src/tests/mod.rs` | `covered` |
| `cleanroom-input/qnt/battle-runtime/battle-runtime-eldritch-blast.mbt.qnt#step:doFillFirstDamageLow` | `tasks/target-replay-evidence/L15-RR22-independent-spell-attack-sequence-routes.json#L15-RR22 independent spell attack sequence route adapter seed=1 action=doFillFirstDamageLow#step:doFillFirstDamageLow` | `src/tests/mod.rs` | `covered` |
| `cleanroom-input/qnt/battle-runtime/battle-runtime-eldritch-blast.mbt.qnt#step:doFillSecondAttackHit` | `tasks/target-replay-evidence/L15-RR22-independent-spell-attack-sequence-routes.json#L15-RR22 independent spell attack sequence route adapter seed=1 action=doFillSecondAttackHit#step:doFillSecondAttackHit` | `src/tests/mod.rs` | `covered` |
| `cleanroom-input/qnt/battle-runtime/battle-runtime-eldritch-blast.mbt.qnt#step:doFillSecondAttackMiss` | `tasks/target-replay-evidence/L15-RR22-independent-spell-attack-sequence-routes.json#L15-RR22 independent spell attack sequence route adapter seed=1 action=doFillSecondAttackMiss#step:doFillSecondAttackMiss` | `src/tests/mod.rs` | `covered` |
| `cleanroom-input/qnt/battle-runtime/battle-runtime-eldritch-blast.mbt.qnt#step:doFillSecondDamageLow` | `tasks/target-replay-evidence/L15-RR22-independent-spell-attack-sequence-routes.json#L15-RR22 independent spell attack sequence route adapter seed=1 action=doFillSecondDamageLow#step:doFillSecondDamageLow` | `src/tests/mod.rs` | `covered` |
| `cleanroom-input/qnt/battle-runtime/battle-runtime-eldritch-blast.mbt.qnt#step:doFillTwoCreatureTargets` | `tasks/target-replay-evidence/L15-RR22-independent-spell-attack-sequence-routes.json#L15-RR22 independent spell attack sequence route adapter seed=1 action=doFillTwoCreatureTargets#step:doFillTwoCreatureTargets` | `src/tests/mod.rs` | `covered` |
| `cleanroom-input/qnt/battle-runtime/battle-runtime-eldritch-blast.mbt.qnt#step:doRejectStaleAfterResolved` | `tasks/target-replay-evidence/L15-RR22-independent-spell-attack-sequence-routes.json#L15-RR22 independent spell attack sequence route adapter seed=1 action=doRejectStaleAfterResolved#step:doRejectStaleAfterResolved` | `src/tests/mod.rs` | `covered` |

Harness artifacts:
- `tasks/START_GATE.json`
- `tasks/ENGINE_DEPTH_MANIFEST.json`
- `tasks/STATE_OWNER_MANIFEST.json`
- `tasks/REVIEW_LOOP.json`
- `tasks/DECIDER_DECISION.json`
- `tasks/target-replay-evidence/L15-RR16-chained-attack-procedure-routes.json`
- `tasks/target-replay-evidence/L15-RR22-independent-spell-attack-sequence-routes.json`

Remaining gaps:
- None for L15-RR16 or L15-RR22.

Verification results:
- Focused adapter route tests passed.
- Full command results are recorded in `tasks/RUN_LEDGER.json`.
