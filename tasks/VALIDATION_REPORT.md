# Validation Report

## Work Loop Status

- Current manifest source commit SHA: `04249edf345a7752de2f1551dd3d509a2fffc160`
- Source branch inventory SHA: `b4e7e101def7969fc420563dc4da020c22e700f0dc0cc1d27accad6e8631225d`
- Scope file: `tasks/LEVEL_1_2_SCOPE.md`
- Work Loop instructions: `tasks/WORK_LOOP.md`
- Last completed current-snapshot queued branch set: `cleanroom-input/qnt/battle-runtime/battle-runtime-concentration-break-teardown.mbt.qnt`
- Next queued driver: `cleanroom-input/qnt/battle-runtime/battle-runtime-creature-type-protection-and-charm-selected-identity.mbt.qnt`
- Next task id: `T020`

Completion rule: a queued branch set is complete only when this report has an
entry that names the exact `.mbt.qnt` driver, records the current manifest
source commit SHA, records the current source branch inventory SHA, lists the
allowed inputs used, renders branch coverage from harness-generated target
replay evidence, and records verification results. Entries with older manifest
source commit SHAs or inventory SHAs are historical unless they include a
current-snapshot revalidation note.

## T000: Report Shape Example Only

- Manifest source commit SHA: `04249edf345a7752de2f1551dd3d509a2fffc160`
- Source branch inventory SHA: `b4e7e101def7969fc420563dc4da020c22e700f0dc0cc1d27accad6e8631225d`
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

Diagnostic tests:

- Focused target-language tests may be listed here as supplemental diagnostics.

Remaining gaps:

- `_none_`

Verification results:

- `cargo fmt --check` passed.
- `cargo test` passed.
- `cargo clippy --all-targets -- -D warnings` passed.
- `node scripts/check-cleanroom-harness.cjs` passed.

## T019: battle-runtime-concentration-break-teardown

- Manifest source commit SHA: `04249edf345a7752de2f1551dd3d509a2fffc160`
- Source branch inventory SHA: `b4e7e101def7969fc420563dc4da020c22e700f0dc0cc1d27accad6e8631225d`
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
  - `cleanroom-input/qnt/battle-runtime/battle-runtime-concentration-break-teardown.mbt.qnt`
  - `cleanroom-input/qnt/battle-runtime/battle-runtime-concentration.qnt`
  - `cleanroom-input/qnt/battle-runtime/battle-runtime-witness-protocol.qnt`
  - `cleanroom-input/raw/srd-5.2.1/Rules-Glossary.md`
  - `cleanroom-input/raw/srd-5.2.1/Spells/Gaining-and-Casting.md`
  - `cleanroom-input/domain/UBIQUITOUS_LANGUAGE.md`
  - `cleanroom-input/domain/CLEANROOM_ASSUMPTIONS.md`
  - `cleanroom-input/guidance/README.md`

Behavior implemented:

- Added typed Concentration state, protocol, damage facts, and saving throw facts in `src/rules/concentration.rs`.
- Implemented the RAW/QNT damage-triggered Concentration saving throw DC: half damage rounded down, minimum 10, maximum 30.
- Projected Concentration spell start, damage requesting a save, failed save teardown before the next command, voluntary Concentration end, and replacement Concentration spell teardown-before-new-effect.
- Kept QNT action names, sampled pick names, witness scenario labels, and protocol hole labels quarantined in `src/qnt_adapters/battle_runtime_concentration_break_teardown.rs`.

Generated branch coverage:

| Obligation | Target replay evidence | Diagnostic tests | Status |
| --- | --- | --- | --- |
| `cleanroom-input/qnt/battle-runtime/battle-runtime-concentration-break-teardown.mbt.qnt#step:doCastConcentrationSpell` | `tasks/target-replay-evidence/T019-battle-runtime-concentration-break-teardown.json#T019-cast-concentration-spell#step:doCastConcentrationSpell` | `src/tests/mod.rs::concentration_break_teardown_adapter_replays_all_branches` | `covered` |
| `cleanroom-input/qnt/battle-runtime/battle-runtime-concentration-break-teardown.mbt.qnt#step:doCastReplacementConcentrationSpell` | `tasks/target-replay-evidence/T019-battle-runtime-concentration-break-teardown.json#T019-cast-replacement-concentration-spell#step:doCastReplacementConcentrationSpell` | `src/tests/mod.rs::concentration_break_teardown_adapter_replays_all_branches` | `covered` |
| `cleanroom-input/qnt/battle-runtime/battle-runtime-concentration-break-teardown.mbt.qnt#step:doDamageRequestsConcentrationSave` | `tasks/target-replay-evidence/T019-battle-runtime-concentration-break-teardown.json#T019-damage-requests-concentration-save#step:doDamageRequestsConcentrationSave` | `src/tests/mod.rs::concentration_break_teardown_adapter_replays_all_branches` | `covered` |
| `cleanroom-input/qnt/battle-runtime/battle-runtime-concentration-break-teardown.mbt.qnt#step:doFailConcentrationSave` | `tasks/target-replay-evidence/T019-battle-runtime-concentration-break-teardown.json#T019-fail-concentration-save#step:doFailConcentrationSave` | `src/tests/mod.rs::concentration_break_teardown_adapter_replays_all_branches` | `covered` |
| `cleanroom-input/qnt/battle-runtime/battle-runtime-concentration-break-teardown.mbt.qnt#step:doVoluntaryEndConcentration` | `tasks/target-replay-evidence/T019-battle-runtime-concentration-break-teardown.json#T019-voluntary-end-concentration#step:doVoluntaryEndConcentration` | `src/tests/mod.rs::concentration_break_teardown_adapter_replays_all_branches` | `covered` |

Target replay evidence:

- Evidence file: `tasks/target-replay-evidence/T019-battle-runtime-concentration-break-teardown.json`
- Target profile: `rust`
- Target profile SHA-256: `6d4cc6c6a4769962798133d57aff01438fb2b661941f71d1aa8a3333f4b7ecc1`
- Quint binding: Rust quint-connect harness
- Reproduction seed or trace id: `T019-cast-concentration-spell`
- Reproduction seed or trace id: `T019-cast-replacement-concentration-spell`
- Reproduction seed or trace id: `T019-damage-requests-concentration-save`
- Reproduction seed or trace id: `T019-fail-concentration-save`
- Reproduction seed or trace id: `T019-voluntary-end-concentration`

Harness artifacts:

- Start gate: `tasks/START_GATE.json`
- Engine depth: `tasks/ENGINE_DEPTH_MANIFEST.json`
- State ownership: `tasks/STATE_OWNER_MANIFEST.json`
- Reviewer loop: `tasks/REVIEW_LOOP.json`
- Decider decision: `tasks/DECIDER_DECISION.json`

Diagnostic tests:

- `src/tests/mod.rs::concentration_breaks_after_failed_save_voluntary_end_or_replacement`

Remaining gaps:

- `_none_`

Verification results:

- `cargo fmt --check` passed.
- `cargo test` passed.
- `cargo clippy --all-targets -- -D warnings` passed.
- `node scripts/check-cleanroom-harness.cjs` passed.

## T018: battle-runtime-command-ordering

- Manifest source commit SHA: `04249edf345a7752de2f1551dd3d509a2fffc160`
- Source branch inventory SHA: `b4e7e101def7969fc420563dc4da020c22e700f0dc0cc1d27accad6e8631225d`
- Driver: `cleanroom-input/qnt/battle-runtime/battle-runtime-command-ordering.mbt.qnt`
- Branch obligations:
  - `step:doDiscoverCommand`
  - `step:doSubmitOptionBeforeTargetList`
  - `step:doFillTargetList`
  - `step:doSubmitSavingThrowBeforeOption`
  - `step:doFillGrovelOption`
  - `step:doFillFailedGrovelSavingThrow`
  - `step:doFollowGrovel`
  - `step:doDropNeedsHeldObjectFacts`
  - `step:doFillDropHeldObjectFacts`
  - `step:doHaltSuppresses`
  - `step:doApproachMovementContinues`
  - `step:doFillApproachMovementContinues`
  - `step:doFillApproachMovementWithinFive`
  - `step:doApproachNoMovement`
  - `step:doFleeMovement`
  - `step:doFillFleeMovement`
  - `step:doRejectFleePartialMovement`
  - `step:doFleeNoMovement`
  - `step:doFleeOpportunityAttack`
- Allowed inputs used:
  - `cleanroom-input/MANIFEST.md`
  - `cleanroom-input/branch-coverage/source-branch-inventory.json`
  - `cleanroom-input/qnt/battle-runtime/battle-runtime-command-ordering.mbt.qnt`
  - `cleanroom-input/qnt/battle-runtime/battle-runtime-command-ordering.qnt`
  - `cleanroom-input/qnt/battle-runtime/battle-runtime-witness-protocol.qnt`
  - `cleanroom-input/qnt/battle-runtime/battle-runtime-hole-kinds.qnt`
  - `cleanroom-input/qnt/battle-runtime/battle-runtime-fill-kinds.qnt`
  - `cleanroom-input/raw/srd-5.2.1/Spells/Descriptions-A-D.md`
  - `cleanroom-input/domain/UBIQUITOUS_LANGUAGE.md`
  - `cleanroom-input/domain/CLEANROOM_ASSUMPTIONS.md`
  - `cleanroom-input/guidance/README.md`

Behavior implemented:

- Extended the Command module with typed fill-frontier stages, hole kinds, fill kinds, ordering errors, and fill-order results for Command target list, option choice, saving throw, held-object facts, and movement stages.
- Implemented QNT ordering rules: target list is required before option, option before saving throw, failed saves route Grovel/Halt to resolved, Drop to table-owned held-object facts when required, and Approach/Flee to movement stages when movement is available.
- Modeled Flee movement rejection when movement is required but unavailable, table-fact frontier detection for Drop held-object facts, and protocol projection from each stage's hole frontier.
- Kept QNT action dispatch, hole-kind labels, stage labels, and witness error labels quarantined in `src/qnt_adapters/battle_runtime_command_ordering.rs`.

Generated branch coverage:

| Obligation | Target replay evidence | Diagnostic tests | Status |
| --- | --- | --- | --- |
| `cleanroom-input/qnt/battle-runtime/battle-runtime-command-ordering.mbt.qnt#step:doDiscoverCommand` | `tasks/target-replay-evidence/T018-battle-runtime-command-ordering.json#T018-discover-command#step:doDiscoverCommand` | `src/tests/mod.rs::command_ordering_adapter_replays_all_branches` | `covered` |
| `cleanroom-input/qnt/battle-runtime/battle-runtime-command-ordering.mbt.qnt#step:doSubmitOptionBeforeTargetList` | `tasks/target-replay-evidence/T018-battle-runtime-command-ordering.json#T018-submit-option-before-target-list#step:doSubmitOptionBeforeTargetList` | `src/tests/mod.rs::command_ordering_adapter_replays_all_branches` | `covered` |
| `cleanroom-input/qnt/battle-runtime/battle-runtime-command-ordering.mbt.qnt#step:doFillTargetList` | `tasks/target-replay-evidence/T018-battle-runtime-command-ordering.json#T018-fill-target-list#step:doFillTargetList` | `src/tests/mod.rs::command_ordering_adapter_replays_all_branches` | `covered` |
| `cleanroom-input/qnt/battle-runtime/battle-runtime-command-ordering.mbt.qnt#step:doSubmitSavingThrowBeforeOption` | `tasks/target-replay-evidence/T018-battle-runtime-command-ordering.json#T018-submit-saving-throw-before-option#step:doSubmitSavingThrowBeforeOption` | `src/tests/mod.rs::command_ordering_adapter_replays_all_branches` | `covered` |
| `cleanroom-input/qnt/battle-runtime/battle-runtime-command-ordering.mbt.qnt#step:doFillGrovelOption` | `tasks/target-replay-evidence/T018-battle-runtime-command-ordering.json#T018-fill-grovel-option#step:doFillGrovelOption` | `src/tests/mod.rs::command_ordering_adapter_replays_all_branches` | `covered` |
| `cleanroom-input/qnt/battle-runtime/battle-runtime-command-ordering.mbt.qnt#step:doFillFailedGrovelSavingThrow` | `tasks/target-replay-evidence/T018-battle-runtime-command-ordering.json#T018-fill-failed-grovel-saving-throw#step:doFillFailedGrovelSavingThrow` | `src/tests/mod.rs::command_ordering_adapter_replays_all_branches` | `covered` |
| `cleanroom-input/qnt/battle-runtime/battle-runtime-command-ordering.mbt.qnt#step:doFollowGrovel` | `tasks/target-replay-evidence/T018-battle-runtime-command-ordering.json#T018-follow-grovel#step:doFollowGrovel` | `src/tests/mod.rs::command_ordering_adapter_replays_all_branches` | `covered` |
| `cleanroom-input/qnt/battle-runtime/battle-runtime-command-ordering.mbt.qnt#step:doDropNeedsHeldObjectFacts` | `tasks/target-replay-evidence/T018-battle-runtime-command-ordering.json#T018-drop-needs-held-object-facts#step:doDropNeedsHeldObjectFacts` | `src/tests/mod.rs::command_ordering_adapter_replays_all_branches` | `covered` |
| `cleanroom-input/qnt/battle-runtime/battle-runtime-command-ordering.mbt.qnt#step:doFillDropHeldObjectFacts` | `tasks/target-replay-evidence/T018-battle-runtime-command-ordering.json#T018-fill-drop-held-object-facts#step:doFillDropHeldObjectFacts` | `src/tests/mod.rs::command_ordering_adapter_replays_all_branches` | `covered` |
| `cleanroom-input/qnt/battle-runtime/battle-runtime-command-ordering.mbt.qnt#step:doHaltSuppresses` | `tasks/target-replay-evidence/T018-battle-runtime-command-ordering.json#T018-halt-suppresses#step:doHaltSuppresses` | `src/tests/mod.rs::command_ordering_adapter_replays_all_branches` | `covered` |
| `cleanroom-input/qnt/battle-runtime/battle-runtime-command-ordering.mbt.qnt#step:doApproachMovementContinues` | `tasks/target-replay-evidence/T018-battle-runtime-command-ordering.json#T018-approach-movement-continues#step:doApproachMovementContinues` | `src/tests/mod.rs::command_ordering_adapter_replays_all_branches` | `covered` |
| `cleanroom-input/qnt/battle-runtime/battle-runtime-command-ordering.mbt.qnt#step:doFillApproachMovementContinues` | `tasks/target-replay-evidence/T018-battle-runtime-command-ordering.json#T018-fill-approach-movement-continues#step:doFillApproachMovementContinues` | `src/tests/mod.rs::command_ordering_adapter_replays_all_branches` | `covered` |
| `cleanroom-input/qnt/battle-runtime/battle-runtime-command-ordering.mbt.qnt#step:doFillApproachMovementWithinFive` | `tasks/target-replay-evidence/T018-battle-runtime-command-ordering.json#T018-fill-approach-movement-within-five#step:doFillApproachMovementWithinFive` | `src/tests/mod.rs::command_ordering_adapter_replays_all_branches` | `covered` |
| `cleanroom-input/qnt/battle-runtime/battle-runtime-command-ordering.mbt.qnt#step:doApproachNoMovement` | `tasks/target-replay-evidence/T018-battle-runtime-command-ordering.json#T018-approach-no-movement#step:doApproachNoMovement` | `src/tests/mod.rs::command_ordering_adapter_replays_all_branches` | `covered` |
| `cleanroom-input/qnt/battle-runtime/battle-runtime-command-ordering.mbt.qnt#step:doFleeMovement` | `tasks/target-replay-evidence/T018-battle-runtime-command-ordering.json#T018-flee-movement#step:doFleeMovement` | `src/tests/mod.rs::command_ordering_adapter_replays_all_branches` | `covered` |
| `cleanroom-input/qnt/battle-runtime/battle-runtime-command-ordering.mbt.qnt#step:doFillFleeMovement` | `tasks/target-replay-evidence/T018-battle-runtime-command-ordering.json#T018-fill-flee-movement#step:doFillFleeMovement` | `src/tests/mod.rs::command_ordering_adapter_replays_all_branches` | `covered` |
| `cleanroom-input/qnt/battle-runtime/battle-runtime-command-ordering.mbt.qnt#step:doRejectFleePartialMovement` | `tasks/target-replay-evidence/T018-battle-runtime-command-ordering.json#T018-reject-flee-partial-movement#step:doRejectFleePartialMovement` | `src/tests/mod.rs::command_ordering_adapter_replays_all_branches` | `covered` |
| `cleanroom-input/qnt/battle-runtime/battle-runtime-command-ordering.mbt.qnt#step:doFleeNoMovement` | `tasks/target-replay-evidence/T018-battle-runtime-command-ordering.json#T018-flee-no-movement#step:doFleeNoMovement` | `src/tests/mod.rs::command_ordering_adapter_replays_all_branches` | `covered` |
| `cleanroom-input/qnt/battle-runtime/battle-runtime-command-ordering.mbt.qnt#step:doFleeOpportunityAttack` | `tasks/target-replay-evidence/T018-battle-runtime-command-ordering.json#T018-flee-opportunity-attack#step:doFleeOpportunityAttack` | `src/tests/mod.rs::command_ordering_adapter_replays_all_branches` | `covered` |

Target replay evidence:

- Evidence file: `tasks/target-replay-evidence/T018-battle-runtime-command-ordering.json`
- Target profile: `rust`
- Target profile SHA-256: `6d4cc6c6a4769962798133d57aff01438fb2b661941f71d1aa8a3333f4b7ecc1`
- Quint binding: Rust quint-connect harness
- Reproduction seed or trace id: `T018-discover-command`
- Reproduction seed or trace id: `T018-submit-option-before-target-list`
- Reproduction seed or trace id: `T018-fill-target-list`
- Reproduction seed or trace id: `T018-submit-saving-throw-before-option`
- Reproduction seed or trace id: `T018-fill-grovel-option`
- Reproduction seed or trace id: `T018-fill-failed-grovel-saving-throw`
- Reproduction seed or trace id: `T018-follow-grovel`
- Reproduction seed or trace id: `T018-drop-needs-held-object-facts`
- Reproduction seed or trace id: `T018-fill-drop-held-object-facts`
- Reproduction seed or trace id: `T018-halt-suppresses`
- Reproduction seed or trace id: `T018-approach-movement-continues`
- Reproduction seed or trace id: `T018-fill-approach-movement-continues`
- Reproduction seed or trace id: `T018-fill-approach-movement-within-five`
- Reproduction seed or trace id: `T018-approach-no-movement`
- Reproduction seed or trace id: `T018-flee-movement`
- Reproduction seed or trace id: `T018-fill-flee-movement`
- Reproduction seed or trace id: `T018-reject-flee-partial-movement`
- Reproduction seed or trace id: `T018-flee-no-movement`
- Reproduction seed or trace id: `T018-flee-opportunity-attack`

Harness artifacts:

- Start gate: `tasks/START_GATE.json`
- Engine depth: `tasks/ENGINE_DEPTH_MANIFEST.json`
- State ownership: `tasks/STATE_OWNER_MANIFEST.json`
- Reviewer loop: `tasks/REVIEW_LOOP.json`
- Decider decision: `tasks/DECIDER_DECISION.json`

Diagnostic tests:

- `src/tests/mod.rs::command_ordering_adapter_replays_all_branches`
- `src/tests/mod.rs::command_ordering_frontier_requires_target_option_save_then_movement`

Remaining gaps:

- `_none_`

Verification results:

- `cargo fmt --check` passed.
- `cargo test` passed.
- `cargo clippy --all-targets -- -D warnings` passed.
- `node scripts/check-cleanroom-harness.cjs` passed.

## T017: battle-runtime-command-option-next-turn

- Manifest source commit SHA: `04249edf345a7752de2f1551dd3d509a2fffc160`
- Source branch inventory SHA: `b4e7e101def7969fc420563dc4da020c22e700f0dc0cc1d27accad6e8631225d`
- Driver: `cleanroom-input/qnt/battle-runtime/battle-runtime-command-option-next-turn.mbt.qnt`
- Branch obligations:
  - `step:doFailedSaveRecordsPending`
  - `step:doFollowGrovel`
  - `step:doFollowDrop`
  - `step:doHaltSuppresses`
  - `step:doHaltEndTurnCleanup`
  - `step:doApproachContinues`
  - `step:doApproachWithinFiveEndsTurn`
  - `step:doApproachMovementRejected`
  - `step:doApproachNoMovementCleanup`
  - `step:doFleeFullMovementEndsTurn`
  - `step:doFleePartialMovementRejected`
  - `step:doFleeNoMovementCleanup`
  - `step:doFleeOpportunityAttackWindow`
  - `step:doFleeOpportunityAttackDeclinedContinuation`
  - `step:doComplete`
- Allowed inputs used:
  - `cleanroom-input/MANIFEST.md`
  - `cleanroom-input/branch-coverage/source-branch-inventory.json`
  - `cleanroom-input/qnt/battle-runtime/battle-runtime-command-option-next-turn.mbt.qnt`
  - `cleanroom-input/qnt/battle-runtime/battle-runtime-witness-protocol.qnt`
  - `cleanroom-input/qnt/battle-runtime/battle-runtime-command-ordering.qnt`
  - `cleanroom-input/raw/srd-5.2.1/Spells/Descriptions-A-D.md`
  - `cleanroom-input/raw/srd-5.2.1/Playing-the-Game.md`
  - `cleanroom-input/raw/srd-5.2.1/Rules-Glossary.md`
  - `cleanroom-input/domain/UBIQUITOUS_LANGUAGE.md`
  - `cleanroom-input/domain/CLEANROOM_ASSUMPTIONS.md`
  - `cleanroom-input/guidance/README.md`

Behavior implemented:

- Added typed Command next-turn option projections for failed-save pending effects and the Grovel, Drop, Halt, Approach, and Flee option outcomes.
- Modeled Grovel applying Prone, Drop releasing held objects, Halt suppressing Movement/Action/Bonus Action for the target turn, Approach movement continuation and invalid movement cleanup, and Flee movement completion, invalid movement cleanup, and Opportunity Attack decision windows.
- Kept QNT action dispatch, scenario strings, actor witness names, invalid-fill witness reason, and interrupt hole labels quarantined in `src/qnt_adapters/battle_runtime_command_option_next_turn.rs`.

Generated branch coverage:

| Obligation | Target replay evidence | Diagnostic tests | Status |
| --- | --- | --- | --- |
| `cleanroom-input/qnt/battle-runtime/battle-runtime-command-option-next-turn.mbt.qnt#step:doFailedSaveRecordsPending` | `tasks/target-replay-evidence/T017-battle-runtime-command-option-next-turn.json#T017-failed-save-records-pending#step:doFailedSaveRecordsPending` | `src/tests/mod.rs::command_option_next_turn_adapter_replays_all_branches` | `covered` |
| `cleanroom-input/qnt/battle-runtime/battle-runtime-command-option-next-turn.mbt.qnt#step:doFollowGrovel` | `tasks/target-replay-evidence/T017-battle-runtime-command-option-next-turn.json#T017-follow-grovel#step:doFollowGrovel` | `src/tests/mod.rs::command_option_next_turn_adapter_replays_all_branches` | `covered` |
| `cleanroom-input/qnt/battle-runtime/battle-runtime-command-option-next-turn.mbt.qnt#step:doFollowDrop` | `tasks/target-replay-evidence/T017-battle-runtime-command-option-next-turn.json#T017-follow-drop#step:doFollowDrop` | `src/tests/mod.rs::command_option_next_turn_adapter_replays_all_branches` | `covered` |
| `cleanroom-input/qnt/battle-runtime/battle-runtime-command-option-next-turn.mbt.qnt#step:doHaltSuppresses` | `tasks/target-replay-evidence/T017-battle-runtime-command-option-next-turn.json#T017-halt-suppresses#step:doHaltSuppresses` | `src/tests/mod.rs::command_option_next_turn_adapter_replays_all_branches` | `covered` |
| `cleanroom-input/qnt/battle-runtime/battle-runtime-command-option-next-turn.mbt.qnt#step:doHaltEndTurnCleanup` | `tasks/target-replay-evidence/T017-battle-runtime-command-option-next-turn.json#T017-halt-end-turn-cleanup#step:doHaltEndTurnCleanup` | `src/tests/mod.rs::command_option_next_turn_adapter_replays_all_branches` | `covered` |
| `cleanroom-input/qnt/battle-runtime/battle-runtime-command-option-next-turn.mbt.qnt#step:doApproachContinues` | `tasks/target-replay-evidence/T017-battle-runtime-command-option-next-turn.json#T017-approach-continues#step:doApproachContinues` | `src/tests/mod.rs::command_option_next_turn_adapter_replays_all_branches` | `covered` |
| `cleanroom-input/qnt/battle-runtime/battle-runtime-command-option-next-turn.mbt.qnt#step:doApproachWithinFiveEndsTurn` | `tasks/target-replay-evidence/T017-battle-runtime-command-option-next-turn.json#T017-approach-within-five-ends-turn#step:doApproachWithinFiveEndsTurn` | `src/tests/mod.rs::command_option_next_turn_adapter_replays_all_branches` | `covered` |
| `cleanroom-input/qnt/battle-runtime/battle-runtime-command-option-next-turn.mbt.qnt#step:doApproachMovementRejected` | `tasks/target-replay-evidence/T017-battle-runtime-command-option-next-turn.json#T017-approach-movement-rejected#step:doApproachMovementRejected` | `src/tests/mod.rs::command_option_next_turn_adapter_replays_all_branches` | `covered` |
| `cleanroom-input/qnt/battle-runtime/battle-runtime-command-option-next-turn.mbt.qnt#step:doApproachNoMovementCleanup` | `tasks/target-replay-evidence/T017-battle-runtime-command-option-next-turn.json#T017-approach-no-movement-cleanup#step:doApproachNoMovementCleanup` | `src/tests/mod.rs::command_option_next_turn_adapter_replays_all_branches` | `covered` |
| `cleanroom-input/qnt/battle-runtime/battle-runtime-command-option-next-turn.mbt.qnt#step:doFleeFullMovementEndsTurn` | `tasks/target-replay-evidence/T017-battle-runtime-command-option-next-turn.json#T017-flee-full-movement-ends-turn#step:doFleeFullMovementEndsTurn` | `src/tests/mod.rs::command_option_next_turn_adapter_replays_all_branches` | `covered` |
| `cleanroom-input/qnt/battle-runtime/battle-runtime-command-option-next-turn.mbt.qnt#step:doFleePartialMovementRejected` | `tasks/target-replay-evidence/T017-battle-runtime-command-option-next-turn.json#T017-flee-partial-movement-rejected#step:doFleePartialMovementRejected` | `src/tests/mod.rs::command_option_next_turn_adapter_replays_all_branches` | `covered` |
| `cleanroom-input/qnt/battle-runtime/battle-runtime-command-option-next-turn.mbt.qnt#step:doFleeNoMovementCleanup` | `tasks/target-replay-evidence/T017-battle-runtime-command-option-next-turn.json#T017-flee-no-movement-cleanup#step:doFleeNoMovementCleanup` | `src/tests/mod.rs::command_option_next_turn_adapter_replays_all_branches` | `covered` |
| `cleanroom-input/qnt/battle-runtime/battle-runtime-command-option-next-turn.mbt.qnt#step:doFleeOpportunityAttackWindow` | `tasks/target-replay-evidence/T017-battle-runtime-command-option-next-turn.json#T017-flee-opportunity-attack-window#step:doFleeOpportunityAttackWindow` | `src/tests/mod.rs::command_option_next_turn_adapter_replays_all_branches` | `covered` |
| `cleanroom-input/qnt/battle-runtime/battle-runtime-command-option-next-turn.mbt.qnt#step:doFleeOpportunityAttackDeclinedContinuation` | `tasks/target-replay-evidence/T017-battle-runtime-command-option-next-turn.json#T017-flee-opportunity-attack-declined-continuation#step:doFleeOpportunityAttackDeclinedContinuation` | `src/tests/mod.rs::command_option_next_turn_adapter_replays_all_branches` | `covered` |
| `cleanroom-input/qnt/battle-runtime/battle-runtime-command-option-next-turn.mbt.qnt#step:doComplete` | `tasks/target-replay-evidence/T017-battle-runtime-command-option-next-turn.json#T017-complete#step:doComplete` | `src/tests/mod.rs::command_option_next_turn_adapter_replays_all_branches` | `covered` |

Target replay evidence:

- Evidence file: `tasks/target-replay-evidence/T017-battle-runtime-command-option-next-turn.json`
- Target profile: `rust`
- Target profile SHA-256: `6d4cc6c6a4769962798133d57aff01438fb2b661941f71d1aa8a3333f4b7ecc1`
- Quint binding: Rust quint-connect harness
- Reproduction seed or trace id: `T017-failed-save-records-pending`
- Reproduction seed or trace id: `T017-follow-grovel`
- Reproduction seed or trace id: `T017-follow-drop`
- Reproduction seed or trace id: `T017-halt-suppresses`
- Reproduction seed or trace id: `T017-halt-end-turn-cleanup`
- Reproduction seed or trace id: `T017-approach-continues`
- Reproduction seed or trace id: `T017-approach-within-five-ends-turn`
- Reproduction seed or trace id: `T017-approach-movement-rejected`
- Reproduction seed or trace id: `T017-approach-no-movement-cleanup`
- Reproduction seed or trace id: `T017-flee-full-movement-ends-turn`
- Reproduction seed or trace id: `T017-flee-partial-movement-rejected`
- Reproduction seed or trace id: `T017-flee-no-movement-cleanup`
- Reproduction seed or trace id: `T017-flee-opportunity-attack-window`
- Reproduction seed or trace id: `T017-flee-opportunity-attack-declined-continuation`
- Reproduction seed or trace id: `T017-complete`

Harness artifacts:

- Start gate: `tasks/START_GATE.json`
- Engine depth: `tasks/ENGINE_DEPTH_MANIFEST.json`
- State ownership: `tasks/STATE_OWNER_MANIFEST.json`
- Reviewer loop: `tasks/REVIEW_LOOP.json`
- Decider decision: `tasks/DECIDER_DECISION.json`

Diagnostic tests:

- `src/tests/mod.rs::command_option_next_turn_adapter_replays_all_branches`
- `src/tests/mod.rs::command_options_project_next_turn_effects_and_cleanup`

Remaining gaps:

- `_none_`

Verification results:

- `cargo fmt --check` passed.
- `cargo test` passed.
- `cargo clippy --all-targets -- -D warnings` passed.
- `node scripts/check-cleanroom-harness.cjs` passed.

## T016: battle-runtime-chained-attack-sequence

- Manifest source commit SHA: `04249edf345a7752de2f1551dd3d509a2fffc160`
- Source branch inventory SHA: `b4e7e101def7969fc420563dc4da020c22e700f0dc0cc1d27accad6e8631225d`
- Driver: `cleanroom-input/qnt/battle-runtime/battle-runtime-chained-attack-sequence.mbt.qnt`
- Branch obligations:
  - `step:doStartCast`
  - `step:doChooseDamageType`
  - `step:doChooseInitialTarget`
  - `step:doResolveStep0AttackHit`
  - `step:doResolveStep0DamageNoDuplicate`
  - `step:doResolveStep0DamageDuplicate`
  - `step:doChooseFirstLeapTarget`
  - `step:doResolveStep1AttackHit`
  - `step:doResolveStep1DuplicateDamageSlot1Limit`
  - `step:doResolveStep1DuplicateDamageSlot2AllowsLeap`
- Allowed inputs used:
  - `cleanroom-input/MANIFEST.md`
  - `cleanroom-input/branch-coverage/source-branch-inventory.json`
  - `cleanroom-input/qnt/battle-runtime/battle-runtime-chained-attack-sequence.mbt.qnt`
  - `cleanroom-input/qnt/battle-runtime/battle-runtime-witness-protocol.qnt`
  - `cleanroom-input/qnt/battle-runtime/battle-runtime-chained-spell-attack.qnt`
  - `cleanroom-input/qnt/shared-algebras/proofs/rule-core/spell-chained-attack-damage-projection-core.qnt`
  - `cleanroom-input/raw/srd-5.2.1/Spells/Descriptions-A-D.md`
  - `cleanroom-input/domain/UBIQUITOUS_LANGUAGE.md`
  - `cleanroom-input/domain/CLEANROOM_ASSUMPTIONS.md`
  - `cleanroom-input/guidance/README.md`

Behavior implemented:

- Added a typed Chromatic Orb chained spell attack state machine with selected damage type, cast slot level, target history, previous target, leaps used, step index, target Hit Points, per-step attack and damage facts, scenario outcome, and replay protocol.
- Implemented Chromatic Orb damage metadata from RAW/QNT: base spell level 1, d8 damage dice, 3 dice plus one die per spell slot level above 1, and duplicate d8 faces enabling a leap only while `leaps_used < slot_level`.
- Enforced first leap target checks for a different target within 30 feet of the previous target and retained target-once-per-casting history.
- Kept QNT action dispatch, witness hole names, scenario strings, and sampled nondeterministic replay choices quarantined in `src/qnt_adapters/battle_runtime_chained_attack_sequence.rs`.

Generated branch coverage:

| Obligation | Target replay evidence | Diagnostic tests | Status |
| --- | --- | --- | --- |
| `cleanroom-input/qnt/battle-runtime/battle-runtime-chained-attack-sequence.mbt.qnt#step:doStartCast` | `tasks/target-replay-evidence/T016-battle-runtime-chained-attack-sequence.json#T016-start-cast#step:doStartCast` | `src/tests/mod.rs::chained_attack_sequence_adapter_replays_all_branches` | `covered` |
| `cleanroom-input/qnt/battle-runtime/battle-runtime-chained-attack-sequence.mbt.qnt#step:doChooseDamageType` | `tasks/target-replay-evidence/T016-battle-runtime-chained-attack-sequence.json#T016-choose-damage-type#step:doChooseDamageType` | `src/tests/mod.rs::chained_attack_sequence_adapter_replays_all_branches` | `covered` |
| `cleanroom-input/qnt/battle-runtime/battle-runtime-chained-attack-sequence.mbt.qnt#step:doChooseInitialTarget` | `tasks/target-replay-evidence/T016-battle-runtime-chained-attack-sequence.json#T016-choose-initial-target#step:doChooseInitialTarget` | `src/tests/mod.rs::chained_attack_sequence_adapter_replays_all_branches` | `covered` |
| `cleanroom-input/qnt/battle-runtime/battle-runtime-chained-attack-sequence.mbt.qnt#step:doResolveStep0AttackHit` | `tasks/target-replay-evidence/T016-battle-runtime-chained-attack-sequence.json#T016-resolve-step0-attack-hit#step:doResolveStep0AttackHit` | `src/tests/mod.rs::chained_attack_sequence_adapter_replays_all_branches` | `covered` |
| `cleanroom-input/qnt/battle-runtime/battle-runtime-chained-attack-sequence.mbt.qnt#step:doResolveStep0DamageNoDuplicate` | `tasks/target-replay-evidence/T016-battle-runtime-chained-attack-sequence.json#T016-resolve-step0-damage-no-duplicate#step:doResolveStep0DamageNoDuplicate` | `src/tests/mod.rs::chained_attack_sequence_adapter_replays_all_branches` | `covered` |
| `cleanroom-input/qnt/battle-runtime/battle-runtime-chained-attack-sequence.mbt.qnt#step:doResolveStep0DamageDuplicate` | `tasks/target-replay-evidence/T016-battle-runtime-chained-attack-sequence.json#T016-resolve-step0-damage-duplicate#step:doResolveStep0DamageDuplicate` | `src/tests/mod.rs::chained_attack_sequence_adapter_replays_all_branches` | `covered` |
| `cleanroom-input/qnt/battle-runtime/battle-runtime-chained-attack-sequence.mbt.qnt#step:doChooseFirstLeapTarget` | `tasks/target-replay-evidence/T016-battle-runtime-chained-attack-sequence.json#T016-choose-first-leap-target#step:doChooseFirstLeapTarget` | `src/tests/mod.rs::chained_attack_sequence_adapter_replays_all_branches` | `covered` |
| `cleanroom-input/qnt/battle-runtime/battle-runtime-chained-attack-sequence.mbt.qnt#step:doResolveStep1AttackHit` | `tasks/target-replay-evidence/T016-battle-runtime-chained-attack-sequence.json#T016-resolve-step1-attack-hit#step:doResolveStep1AttackHit` | `src/tests/mod.rs::chained_attack_sequence_adapter_replays_all_branches` | `covered` |
| `cleanroom-input/qnt/battle-runtime/battle-runtime-chained-attack-sequence.mbt.qnt#step:doResolveStep1DuplicateDamageSlot1Limit` | `tasks/target-replay-evidence/T016-battle-runtime-chained-attack-sequence.json#T016-resolve-step1-duplicate-damage-slot1-limit#step:doResolveStep1DuplicateDamageSlot1Limit` | `src/tests/mod.rs::chained_attack_sequence_adapter_replays_all_branches` | `covered` |
| `cleanroom-input/qnt/battle-runtime/battle-runtime-chained-attack-sequence.mbt.qnt#step:doResolveStep1DuplicateDamageSlot2AllowsLeap` | `tasks/target-replay-evidence/T016-battle-runtime-chained-attack-sequence.json#T016-resolve-step1-duplicate-damage-slot2-allows-leap#step:doResolveStep1DuplicateDamageSlot2AllowsLeap` | `src/tests/mod.rs::chained_attack_sequence_adapter_replays_all_branches` | `covered` |

Target replay evidence:

- Evidence file: `tasks/target-replay-evidence/T016-battle-runtime-chained-attack-sequence.json`
- Target profile: `rust`
- Target profile SHA-256: `6d4cc6c6a4769962798133d57aff01438fb2b661941f71d1aa8a3333f4b7ecc1`
- Quint binding: Rust quint-connect harness
- Reproduction seed or trace id: `T016-start-cast`
- Reproduction seed or trace id: `T016-choose-damage-type`
- Reproduction seed or trace id: `T016-choose-initial-target`
- Reproduction seed or trace id: `T016-resolve-step0-attack-hit`
- Reproduction seed or trace id: `T016-resolve-step0-damage-no-duplicate`
- Reproduction seed or trace id: `T016-resolve-step0-damage-duplicate`
- Reproduction seed or trace id: `T016-choose-first-leap-target`
- Reproduction seed or trace id: `T016-resolve-step1-attack-hit`
- Reproduction seed or trace id: `T016-resolve-step1-duplicate-damage-slot1-limit`
- Reproduction seed or trace id: `T016-resolve-step1-duplicate-damage-slot2-allows-leap`

Harness artifacts:

- Start gate: `tasks/START_GATE.json`
- Engine depth: `tasks/ENGINE_DEPTH_MANIFEST.json`
- State ownership: `tasks/STATE_OWNER_MANIFEST.json`
- Reviewer loop: `tasks/REVIEW_LOOP.json`
- Decider decision: `tasks/DECIDER_DECISION.json`

Diagnostic tests:

- `src/tests/mod.rs::chained_attack_sequence_adapter_replays_all_branches`
- `src/tests/mod.rs::chromatic_orb_sequence_tracks_duplicate_leap_limit`

Remaining gaps:

- `_none_`

Verification results:

- `cargo fmt --check` passed.
- `cargo test` passed.
- `cargo clippy --all-targets -- -D warnings` passed.
- `node scripts/check-cleanroom-harness.cjs` passed.

## T015: battle-runtime-attack-spell-shape-selected-identity

- Manifest source commit SHA: `04249edf345a7752de2f1551dd3d509a2fffc160`
- Source branch inventory SHA: `b4e7e101def7969fc420563dc4da020c22e700f0dc0cc1d27accad6e8631225d`
- Driver: `cleanroom-input/qnt/battle-runtime/battle-runtime-attack-spell-shape-selected-identity.mbt.qnt`
- Branch obligations:
  - `step:doChillTouchHitPointRegainPrevention`
  - `step:doFireBoltHit`
  - `step:doGuidingBoltNextAttackAdvantage`
  - `step:doInflictWoundsFailedSave`
  - `step:doInflictWoundsSuccessfulSave`
  - `step:doShockingGraspOpportunityAttackDenied`
- Allowed inputs used:
  - `cleanroom-input/MANIFEST.md`
  - `cleanroom-input/branch-coverage/source-branch-inventory.json`
  - `cleanroom-input/qnt/battle-runtime/battle-runtime-attack-spell-shape-selected-identity.mbt.qnt`
  - `cleanroom-input/qnt/battle-runtime/battle-runtime-witness-protocol.qnt`
  - `cleanroom-input/qnt/battle-runtime/battle-runtime-spell-attack.qnt`
  - `cleanroom-input/qnt/battle-runtime/battle-runtime-save-gated-spell.qnt`
  - `cleanroom-input/qnt/battle-runtime/battle-runtime-spell-bridge.qnt`
  - `cleanroom-input/qnt/battle-runtime/battle-runtime-spell-invocation.qnt`
  - `cleanroom-input/qnt/shared-algebras/proofs/rule-core/spell-attack-damage-projection-core.qnt`
  - `cleanroom-input/qnt/shared-algebras/proofs/rule-core/spell-save-damage-projection-core.qnt`
  - `cleanroom-input/raw/srd-5.2.1/Classes/Cleric.md`
  - `cleanroom-input/raw/srd-5.2.1/Classes/Sorcerer.md`
  - `cleanroom-input/raw/srd-5.2.1/Classes/Wizard.md`
  - `cleanroom-input/raw/srd-5.2.1/Spells/Descriptions-A-D.md`
  - `cleanroom-input/raw/srd-5.2.1/Spells/Descriptions-E-L.md`
  - `cleanroom-input/raw/srd-5.2.1/Spells/Descriptions-S-Z.md`
  - `cleanroom-input/domain/UBIQUITOUS_LANGUAGE.md`
  - `cleanroom-input/domain/CLEANROOM_ASSUMPTIONS.md`
  - `cleanroom-input/guidance/README.md`

Behavior implemented:

- Added reusable spell-shape projections for attack spell hits and save-gated spell damage.
- Attack spell hit projection covers Fire Bolt damage with no rider, Chill Touch Hit Point regain prevention, Guiding Bolt next-attack Advantage and level-1 slot spend, and Shocking Grasp Opportunity Attack denial.
- Save-gated projection covers Inflict Wounds failed save full damage and successful save half damage, with level-1 slot spend.
- Kept QNT action dispatch, scenario outcome strings, and witness protocol mapping quarantined in `src/qnt_adapters/battle_runtime_attack_spell_shape_selected_identity.rs`.

Generated branch coverage:

| Obligation | Target replay evidence | Diagnostic tests | Status |
| --- | --- | --- | --- |
| `cleanroom-input/qnt/battle-runtime/battle-runtime-attack-spell-shape-selected-identity.mbt.qnt#step:doChillTouchHitPointRegainPrevention` | `tasks/target-replay-evidence/T015-battle-runtime-attack-spell-shape-selected-identity.json#T015-chill-touch-hit-point-regain-prevention#step:doChillTouchHitPointRegainPrevention` | `src/tests/mod.rs::attack_spell_shape_adapter_replays_all_branches` | `covered` |
| `cleanroom-input/qnt/battle-runtime/battle-runtime-attack-spell-shape-selected-identity.mbt.qnt#step:doFireBoltHit` | `tasks/target-replay-evidence/T015-battle-runtime-attack-spell-shape-selected-identity.json#T015-fire-bolt-hit#step:doFireBoltHit` | `src/tests/mod.rs::attack_spell_shape_adapter_replays_all_branches` | `covered` |
| `cleanroom-input/qnt/battle-runtime/battle-runtime-attack-spell-shape-selected-identity.mbt.qnt#step:doGuidingBoltNextAttackAdvantage` | `tasks/target-replay-evidence/T015-battle-runtime-attack-spell-shape-selected-identity.json#T015-guiding-bolt-next-attack-advantage#step:doGuidingBoltNextAttackAdvantage` | `src/tests/mod.rs::attack_spell_shape_adapter_replays_all_branches` | `covered` |
| `cleanroom-input/qnt/battle-runtime/battle-runtime-attack-spell-shape-selected-identity.mbt.qnt#step:doInflictWoundsFailedSave` | `tasks/target-replay-evidence/T015-battle-runtime-attack-spell-shape-selected-identity.json#T015-inflict-wounds-failed-save#step:doInflictWoundsFailedSave` | `src/tests/mod.rs::attack_spell_shape_adapter_replays_all_branches` | `covered` |
| `cleanroom-input/qnt/battle-runtime/battle-runtime-attack-spell-shape-selected-identity.mbt.qnt#step:doInflictWoundsSuccessfulSave` | `tasks/target-replay-evidence/T015-battle-runtime-attack-spell-shape-selected-identity.json#T015-inflict-wounds-successful-save#step:doInflictWoundsSuccessfulSave` | `src/tests/mod.rs::attack_spell_shape_adapter_replays_all_branches` | `covered` |
| `cleanroom-input/qnt/battle-runtime/battle-runtime-attack-spell-shape-selected-identity.mbt.qnt#step:doShockingGraspOpportunityAttackDenied` | `tasks/target-replay-evidence/T015-battle-runtime-attack-spell-shape-selected-identity.json#T015-shocking-grasp-opportunity-attack-denied#step:doShockingGraspOpportunityAttackDenied` | `src/tests/mod.rs::attack_spell_shape_adapter_replays_all_branches` | `covered` |

Target replay evidence:

- Evidence file: `tasks/target-replay-evidence/T015-battle-runtime-attack-spell-shape-selected-identity.json`
- Target profile: `rust`
- Target profile SHA-256: `6d4cc6c6a4769962798133d57aff01438fb2b661941f71d1aa8a3333f4b7ecc1`
- Quint binding: Rust quint-connect harness
- Reproduction seed or trace id: `T015-chill-touch-hit-point-regain-prevention`
- Reproduction seed or trace id: `T015-fire-bolt-hit`
- Reproduction seed or trace id: `T015-guiding-bolt-next-attack-advantage`
- Reproduction seed or trace id: `T015-inflict-wounds-failed-save`
- Reproduction seed or trace id: `T015-inflict-wounds-successful-save`
- Reproduction seed or trace id: `T015-shocking-grasp-opportunity-attack-denied`

Harness artifacts:

- Start gate: `tasks/START_GATE.json`
- Engine depth: `tasks/ENGINE_DEPTH_MANIFEST.json`
- State ownership: `tasks/STATE_OWNER_MANIFEST.json`
- Reviewer loop: `tasks/REVIEW_LOOP.json`
- Decider decision: `tasks/DECIDER_DECISION.json`

Diagnostic tests:

- `src/tests/mod.rs::attack_spell_shape_adapter_replays_all_branches`
- `src/tests/mod.rs::attack_spell_shapes_project_slots_effects_and_save_damage`

Remaining gaps:

- `_none_`

Verification results:

- `cargo fmt --check` passed.
- `cargo test` passed.
- `cargo clippy --all-targets -- -D warnings` passed.
- `node scripts/check-cleanroom-harness.cjs` passed.

## T014: battle-runtime-adrenaline-rush

- Manifest source commit SHA: `04249edf345a7752de2f1551dd3d509a2fffc160`
- Source branch inventory SHA: `b4e7e101def7969fc420563dc4da020c22e700f0dc0cc1d27accad6e8631225d`
- Driver: `cleanroom-input/qnt/battle-runtime/battle-runtime-adrenaline-rush.mbt.qnt`
- Branch obligations:
  - `step:doAdrenalineRushDash`
  - `step:doRejectSecondDash`
- Allowed inputs used:
  - `cleanroom-input/MANIFEST.md`
  - `cleanroom-input/branch-coverage/source-branch-inventory.json`
  - `cleanroom-input/qnt/battle-runtime/battle-runtime-adrenaline-rush.mbt.qnt`
  - `cleanroom-input/qnt/battle-runtime/battle-runtime-witness-protocol.qnt`
  - `cleanroom-input/qnt/battle-runtime/battle-runtime-movement.qnt`
  - `cleanroom-input/qnt/battle-runtime/battle-runtime-movement-bridge.qnt`
  - `cleanroom-input/qnt/shared-algebras/proofs/rule-core/action-turn-procedures.qnt`
  - `cleanroom-input/raw/srd-5.2.1/Character-Origins.md`
  - `cleanroom-input/raw/srd-5.2.1/Playing-the-Game.md`
  - `cleanroom-input/domain/UBIQUITOUS_LANGUAGE.md`
  - `cleanroom-input/domain/CLEANROOM_ASSUMPTIONS.md`
  - `cleanroom-input/guidance/README.md`

Behavior implemented:

- Added reusable battle feature resolution for Orc Adrenaline Rush as a Bonus Action Dash that requires the actor's turn, available Bonus Action, remaining feature use, valid Proficiency Bonus, and ability to act.
- Resolved Adrenaline Rush spends the Bonus Action, adds Speed to dash bonus feet, grants non-stacking Temporary Hit Points equal to Proficiency Bonus, and spends one feature use.
- Rejected repeated dash after the Bonus Action is spent leaves battle state and feature uses unchanged with a typed stale-subject rejection.
- Added reusable `apply_temporary_hit_points` helper for the RAW non-stacking Temporary Hit Points rule.
- Kept QNT action dispatch, witness protocol strings, and branch-specific replay setup quarantined in `src/qnt_adapters/battle_runtime_adrenaline_rush.rs`.

Generated branch coverage:

| Obligation | Target replay evidence | Diagnostic tests | Status |
| --- | --- | --- | --- |
| `cleanroom-input/qnt/battle-runtime/battle-runtime-adrenaline-rush.mbt.qnt#step:doAdrenalineRushDash` | `tasks/target-replay-evidence/T014-battle-runtime-adrenaline-rush.json#T014-adrenaline-rush-dash#step:doAdrenalineRushDash` | `src/tests/mod.rs::adrenaline_rush_adapter_replays_all_branches` | `covered` |
| `cleanroom-input/qnt/battle-runtime/battle-runtime-adrenaline-rush.mbt.qnt#step:doRejectSecondDash` | `tasks/target-replay-evidence/T014-battle-runtime-adrenaline-rush.json#T014-reject-second-dash#step:doRejectSecondDash` | `src/tests/mod.rs::adrenaline_rush_adapter_replays_all_branches` | `covered` |

Target replay evidence:

- Evidence file: `tasks/target-replay-evidence/T014-battle-runtime-adrenaline-rush.json`
- Target profile: `rust`
- Target profile SHA-256: `6d4cc6c6a4769962798133d57aff01438fb2b661941f71d1aa8a3333f4b7ecc1`
- Quint binding: Rust quint-connect harness
- Reproduction seed or trace id: `T014-adrenaline-rush-dash`
- Reproduction seed or trace id: `T014-reject-second-dash`

Harness artifacts:

- Start gate: `tasks/START_GATE.json`
- Engine depth: `tasks/ENGINE_DEPTH_MANIFEST.json`
- State ownership: `tasks/STATE_OWNER_MANIFEST.json`
- Reviewer loop: `tasks/REVIEW_LOOP.json`
- Decider decision: `tasks/DECIDER_DECISION.json`

Diagnostic tests:

- `src/tests/mod.rs::adrenaline_rush_adapter_replays_all_branches`
- `src/tests/mod.rs::adrenaline_rush_bonus_action_dash_spends_use_and_keeps_higher_temp_hp`

Remaining gaps:

- `_none_`

Verification results:

- `cargo fmt --check` passed.
- `cargo test` passed.
- `cargo clippy --all-targets -- -D warnings` passed.
- `node scripts/check-cleanroom-harness.cjs` passed.

## T013: character-battle-origin-feat-selected-identity

- Manifest source commit SHA: `04249edf345a7752de2f1551dd3d509a2fffc160`
- Source branch inventory SHA: `b4e7e101def7969fc420563dc4da020c22e700f0dc0cc1d27accad6e8631225d`
- Driver: `cleanroom-input/qnt/character-battle-runtime/character-battle-origin-feat-selected-identity.mbt.qnt`
- Branch obligations:
  - `step:doFinalizeCriminalAlertOriginFeat`
  - `step:doProjectAlertInitiativeHandoff`
- Allowed inputs used:
  - `cleanroom-input/MANIFEST.md`
  - `cleanroom-input/branch-coverage/source-branch-inventory.json`
  - `cleanroom-input/qnt/character-battle-runtime/character-battle-origin-feat-selected-identity.mbt.qnt`
  - `cleanroom-input/raw/srd-5.2.1/Character-Origins.md`
  - `cleanroom-input/raw/srd-5.2.1/Feats.md`
  - `cleanroom-input/raw/srd-5.2.1/Playing-the-Game.md`
  - `cleanroom-input/raw/srd-5.2.1/Rules-Glossary.md`
  - `cleanroom-input/domain/UBIQUITOUS_LANGUAGE.md`
  - `cleanroom-input/domain/CLEANROOM_ASSUMPTIONS.md`
  - `cleanroom-input/guidance/README.md`

Behavior implemented:

- Added typed origin-feat projection for the Criminal background selecting the Alert Origin feat.
- Added initiative handoff projection that starts from the Initiative score base, includes Dexterity modifier, and adds proficiency bonus when Alert initiative proficiency is present.
- Kept QNT action dispatch, unit id strings, and witness field mapping quarantined in `src/qnt_adapters/character_battle_origin_feat_selected_identity.rs`.

Generated branch coverage:

| Obligation | Target replay evidence | Diagnostic tests | Status |
| --- | --- | --- | --- |
| `cleanroom-input/qnt/character-battle-runtime/character-battle-origin-feat-selected-identity.mbt.qnt#step:doFinalizeCriminalAlertOriginFeat` | `tasks/target-replay-evidence/T013-character-battle-origin-feat-selected-identity.json#T013-finalize-criminal-alert-origin-feat#step:doFinalizeCriminalAlertOriginFeat` | `src/tests/mod.rs::origin_feat_adapter_replays_all_branches` | `covered` |
| `cleanroom-input/qnt/character-battle-runtime/character-battle-origin-feat-selected-identity.mbt.qnt#step:doProjectAlertInitiativeHandoff` | `tasks/target-replay-evidence/T013-character-battle-origin-feat-selected-identity.json#T013-project-alert-initiative-handoff#step:doProjectAlertInitiativeHandoff` | `src/tests/mod.rs::origin_feat_adapter_replays_all_branches` | `covered` |

Target replay evidence:

- Evidence file: `tasks/target-replay-evidence/T013-character-battle-origin-feat-selected-identity.json`
- Target profile: `rust`
- Target profile SHA-256: `6d4cc6c6a4769962798133d57aff01438fb2b661941f71d1aa8a3333f4b7ecc1`
- Quint binding: Rust quint-connect harness
- Reproduction seed or trace id: `T013-finalize-criminal-alert-origin-feat`
- Reproduction seed or trace id: `T013-project-alert-initiative-handoff`

Harness artifacts:

- Start gate: `tasks/START_GATE.json`
- Engine depth: `tasks/ENGINE_DEPTH_MANIFEST.json`
- State ownership: `tasks/STATE_OWNER_MANIFEST.json`
- Reviewer loop: `tasks/REVIEW_LOOP.json`
- Decider decision: `tasks/DECIDER_DECISION.json`

Diagnostic tests:

- `src/tests/mod.rs::origin_feat_adapter_replays_all_branches`
- `src/tests/mod.rs::alert_origin_feat_adds_proficiency_to_initiative_score`

Remaining gaps:

- `_none_`

Verification results:

- `cargo fmt --check` passed.
- `cargo test` passed.
- `cargo clippy --all-targets -- -D warnings` passed.
- `node scripts/check-cleanroom-harness.cjs` passed.

## T012: character-sheet-weapon-mastery-containers-selected-identity

- Manifest source commit SHA: `04249edf345a7752de2f1551dd3d509a2fffc160`
- Source branch inventory SHA: `b4e7e101def7969fc420563dc4da020c22e700f0dc0cc1d27accad6e8631225d`
- Driver: `cleanroom-input/qnt/character-sheet-runtime/character-sheet-weapon-mastery-containers-selected-identity.mbt.qnt`
- Branch obligations:
  - `step:doAcceptOneChangeWeaponMasteryReselection`
  - `step:doRejectTooManyChangesWeaponMasteryReselection`
  - `step:doReselectPaladinWeaponMasteryOnLongRest`
  - `step:doReselectRangerWeaponMasteryOnLongRest`
  - `step:doReselectRogueWeaponMasteryOnLongRest`
  - `step:doSelectPaladinWeaponMastery`
  - `step:doSelectRangerWeaponMastery`
  - `step:doSelectRogueWeaponMastery`
- Allowed inputs used:
  - `cleanroom-input/MANIFEST.md`
  - `cleanroom-input/branch-coverage/source-branch-inventory.json`
  - `cleanroom-input/qnt/character-sheet-runtime/character-sheet-weapon-mastery-containers-selected-identity.mbt.qnt`
  - `cleanroom-input/qnt/shared-algebras/proofs/rule-core/weapon-mastery-reselection.qnt`
  - `cleanroom-input/raw/srd-5.2.1/Classes/Paladin.md`
  - `cleanroom-input/raw/srd-5.2.1/Classes/Ranger.md`
  - `cleanroom-input/raw/srd-5.2.1/Classes/Rogue.md`
  - `cleanroom-input/raw/srd-5.2.1/Equipment.md`
  - `cleanroom-input/domain/UBIQUITOUS_LANGUAGE.md`
  - `cleanroom-input/domain/CLEANROOM_ASSUMPTIONS.md`
  - `cleanroom-input/guidance/README.md`

Behavior implemented:

- Extended reusable Weapon Mastery rules with Shortbow as an SRD weapon choice needed by Rogue sheet witnesses.
- Added generic Long Rest Weapon Mastery reselection facts and projection that enforce positive choice count, current/requested choice counts, distinct current/requested choices, requested-choice eligibility, and changed-choice count within the Long Rest allowance.
- Reused creation-time Weapon Mastery projection for initial Paladin, Ranger, and Rogue sheet selections.
- Kept QNT action dispatch, placeholder weapon refs, and witness field mapping quarantined in `src/qnt_adapters/character_sheet_weapon_mastery_containers_selected_identity.rs`.

Generated branch coverage:

| Obligation | Target replay evidence | Diagnostic tests | Status |
| --- | --- | --- | --- |
| `cleanroom-input/qnt/character-sheet-runtime/character-sheet-weapon-mastery-containers-selected-identity.mbt.qnt#step:doAcceptOneChangeWeaponMasteryReselection` | `tasks/target-replay-evidence/T012-character-sheet-weapon-mastery-containers-selected-identity.json#T012-accept-one-change-weapon-mastery-reselection#step:doAcceptOneChangeWeaponMasteryReselection` | `src/tests/mod.rs::sheet_weapon_mastery_adapter_replays_all_branches` | `covered` |
| `cleanroom-input/qnt/character-sheet-runtime/character-sheet-weapon-mastery-containers-selected-identity.mbt.qnt#step:doRejectTooManyChangesWeaponMasteryReselection` | `tasks/target-replay-evidence/T012-character-sheet-weapon-mastery-containers-selected-identity.json#T012-reject-too-many-changes-weapon-mastery-reselection#step:doRejectTooManyChangesWeaponMasteryReselection` | `src/tests/mod.rs::sheet_weapon_mastery_adapter_replays_all_branches` | `covered` |
| `cleanroom-input/qnt/character-sheet-runtime/character-sheet-weapon-mastery-containers-selected-identity.mbt.qnt#step:doReselectPaladinWeaponMasteryOnLongRest` | `tasks/target-replay-evidence/T012-character-sheet-weapon-mastery-containers-selected-identity.json#T012-reselect-paladin-weapon-mastery-on-long-rest#step:doReselectPaladinWeaponMasteryOnLongRest` | `src/tests/mod.rs::sheet_weapon_mastery_adapter_replays_all_branches` | `covered` |
| `cleanroom-input/qnt/character-sheet-runtime/character-sheet-weapon-mastery-containers-selected-identity.mbt.qnt#step:doReselectRangerWeaponMasteryOnLongRest` | `tasks/target-replay-evidence/T012-character-sheet-weapon-mastery-containers-selected-identity.json#T012-reselect-ranger-weapon-mastery-on-long-rest#step:doReselectRangerWeaponMasteryOnLongRest` | `src/tests/mod.rs::sheet_weapon_mastery_adapter_replays_all_branches` | `covered` |
| `cleanroom-input/qnt/character-sheet-runtime/character-sheet-weapon-mastery-containers-selected-identity.mbt.qnt#step:doReselectRogueWeaponMasteryOnLongRest` | `tasks/target-replay-evidence/T012-character-sheet-weapon-mastery-containers-selected-identity.json#T012-reselect-rogue-weapon-mastery-on-long-rest#step:doReselectRogueWeaponMasteryOnLongRest` | `src/tests/mod.rs::sheet_weapon_mastery_adapter_replays_all_branches` | `covered` |
| `cleanroom-input/qnt/character-sheet-runtime/character-sheet-weapon-mastery-containers-selected-identity.mbt.qnt#step:doSelectPaladinWeaponMastery` | `tasks/target-replay-evidence/T012-character-sheet-weapon-mastery-containers-selected-identity.json#T012-select-paladin-weapon-mastery#step:doSelectPaladinWeaponMastery` | `src/tests/mod.rs::sheet_weapon_mastery_adapter_replays_all_branches` | `covered` |
| `cleanroom-input/qnt/character-sheet-runtime/character-sheet-weapon-mastery-containers-selected-identity.mbt.qnt#step:doSelectRangerWeaponMastery` | `tasks/target-replay-evidence/T012-character-sheet-weapon-mastery-containers-selected-identity.json#T012-select-ranger-weapon-mastery#step:doSelectRangerWeaponMastery` | `src/tests/mod.rs::sheet_weapon_mastery_adapter_replays_all_branches` | `covered` |
| `cleanroom-input/qnt/character-sheet-runtime/character-sheet-weapon-mastery-containers-selected-identity.mbt.qnt#step:doSelectRogueWeaponMastery` | `tasks/target-replay-evidence/T012-character-sheet-weapon-mastery-containers-selected-identity.json#T012-select-rogue-weapon-mastery#step:doSelectRogueWeaponMastery` | `src/tests/mod.rs::sheet_weapon_mastery_adapter_replays_all_branches` | `covered` |

Target replay evidence:

- Evidence file: `tasks/target-replay-evidence/T012-character-sheet-weapon-mastery-containers-selected-identity.json`
- Target profile: `rust`
- Target profile SHA-256: `6d4cc6c6a4769962798133d57aff01438fb2b661941f71d1aa8a3333f4b7ecc1`
- Quint binding: Rust quint-connect harness
- Reproduction seed or trace id: `T012-accept-one-change-weapon-mastery-reselection`
- Reproduction seed or trace id: `T012-reject-too-many-changes-weapon-mastery-reselection`
- Reproduction seed or trace id: `T012-reselect-paladin-weapon-mastery-on-long-rest`
- Reproduction seed or trace id: `T012-reselect-ranger-weapon-mastery-on-long-rest`
- Reproduction seed or trace id: `T012-reselect-rogue-weapon-mastery-on-long-rest`
- Reproduction seed or trace id: `T012-select-paladin-weapon-mastery`
- Reproduction seed or trace id: `T012-select-ranger-weapon-mastery`
- Reproduction seed or trace id: `T012-select-rogue-weapon-mastery`

Harness artifacts:

- Start gate: `tasks/START_GATE.json`
- Engine depth: `tasks/ENGINE_DEPTH_MANIFEST.json`
- State ownership: `tasks/STATE_OWNER_MANIFEST.json`
- Reviewer loop: `tasks/REVIEW_LOOP.json`
- Decider decision: `tasks/DECIDER_DECISION.json`

Diagnostic tests:

- `src/tests/mod.rs::sheet_weapon_mastery_adapter_replays_all_branches`
- `src/tests/mod.rs::weapon_mastery_reselection_applies_long_rest_change_limit`

Remaining gaps:

- `_none_`

Verification results:

- `cargo fmt --check` passed.
- `cargo test` passed.
- `cargo clippy --all-targets -- -D warnings` passed.
- `node scripts/check-cleanroom-harness.cjs` passed.

## T011: character-sheet-spellbook-ritual-selected-identity

- Manifest source commit SHA: `04249edf345a7752de2f1551dd3d509a2fffc160`
- Source branch inventory SHA: `b4e7e101def7969fc420563dc4da020c22e700f0dc0cc1d27accad6e8631225d`
- Driver: `cleanroom-input/qnt/character-sheet-runtime/character-sheet-spellbook-ritual-selected-identity.mbt.qnt`
- Branch obligations:
  - `step:doInvokeSpellbookRitual`
  - `step:doRejectMissingRitualAccessFeature`
  - `step:doRejectNonLeveledRitualSpellbookSpell`
  - `step:doRejectNonRitualSpellbookSpell`
  - `step:doRejectPreparedOnlyRitual`
- Allowed inputs used:
  - `cleanroom-input/MANIFEST.md`
  - `cleanroom-input/branch-coverage/source-branch-inventory.json`
  - `cleanroom-input/qnt/character-sheet-runtime/character-sheet-spellbook-ritual-selected-identity.mbt.qnt`
  - `cleanroom-input/qnt/shared-algebras/proofs/rule-core/spellbook-ritual-access.qnt`
  - `cleanroom-input/raw/srd-5.2.1/Classes/Wizard.md`
  - `cleanroom-input/raw/srd-5.2.1/Spells/Gaining-and-Casting.md`
  - `cleanroom-input/raw/srd-5.2.1/Spells/Descriptions-A-D.md`
  - `cleanroom-input/raw/srd-5.2.1/Rules-Glossary.md`
  - `cleanroom-input/domain/UBIQUITOUS_LANGUAGE.md`
  - `cleanroom-input/domain/CLEANROOM_ASSUMPTIONS.md`
  - `cleanroom-input/guidance/README.md`

Behavior implemented:

- Added reusable spellbook ritual facts and invocation projection for Wizard Ritual Adept access to a level 1+ Ritual spell in the spellbook.
- Accepted spellbook ritual invocation projects no spell-slot cost, no preparation requirement, required spellbook access, 10 additional casting-time minutes, reading from the spellbook, and zero first-level spell slots expended.
- Rejected prepared-only ritual access for the Wizard spellbook feature path, non-Ritual level 1+ spellbook spells, missing Ritual Adept access, and non-leveled spells.
- Kept QNT action dispatch and witness field mapping quarantined in `src/qnt_adapters/character_sheet_spellbook_ritual_selected_identity.rs`.

Generated branch coverage:

| Obligation | Target replay evidence | Diagnostic tests | Status |
| --- | --- | --- | --- |
| `cleanroom-input/qnt/character-sheet-runtime/character-sheet-spellbook-ritual-selected-identity.mbt.qnt#step:doInvokeSpellbookRitual` | `tasks/target-replay-evidence/T011-character-sheet-spellbook-ritual-selected-identity.json#T011-invoke-spellbook-ritual#step:doInvokeSpellbookRitual` | `src/tests/mod.rs::spellbook_ritual_adapter_replays_all_branches` | `covered` |
| `cleanroom-input/qnt/character-sheet-runtime/character-sheet-spellbook-ritual-selected-identity.mbt.qnt#step:doRejectMissingRitualAccessFeature` | `tasks/target-replay-evidence/T011-character-sheet-spellbook-ritual-selected-identity.json#T011-reject-missing-ritual-access-feature#step:doRejectMissingRitualAccessFeature` | `src/tests/mod.rs::spellbook_ritual_adapter_replays_all_branches` | `covered` |
| `cleanroom-input/qnt/character-sheet-runtime/character-sheet-spellbook-ritual-selected-identity.mbt.qnt#step:doRejectNonLeveledRitualSpellbookSpell` | `tasks/target-replay-evidence/T011-character-sheet-spellbook-ritual-selected-identity.json#T011-reject-non-leveled-ritual-spellbook-spell#step:doRejectNonLeveledRitualSpellbookSpell` | `src/tests/mod.rs::spellbook_ritual_adapter_replays_all_branches` | `covered` |
| `cleanroom-input/qnt/character-sheet-runtime/character-sheet-spellbook-ritual-selected-identity.mbt.qnt#step:doRejectNonRitualSpellbookSpell` | `tasks/target-replay-evidence/T011-character-sheet-spellbook-ritual-selected-identity.json#T011-reject-non-ritual-spellbook-spell#step:doRejectNonRitualSpellbookSpell` | `src/tests/mod.rs::spellbook_ritual_adapter_replays_all_branches` | `covered` |
| `cleanroom-input/qnt/character-sheet-runtime/character-sheet-spellbook-ritual-selected-identity.mbt.qnt#step:doRejectPreparedOnlyRitual` | `tasks/target-replay-evidence/T011-character-sheet-spellbook-ritual-selected-identity.json#T011-reject-prepared-only-ritual#step:doRejectPreparedOnlyRitual` | `src/tests/mod.rs::spellbook_ritual_adapter_replays_all_branches` | `covered` |

Target replay evidence:

- Evidence file: `tasks/target-replay-evidence/T011-character-sheet-spellbook-ritual-selected-identity.json`
- Target profile: `rust`
- Target profile SHA-256: `6d4cc6c6a4769962798133d57aff01438fb2b661941f71d1aa8a3333f4b7ecc1`
- Quint binding: Rust quint-connect harness
- Reproduction seed or trace id: `T011-invoke-spellbook-ritual`
- Reproduction seed or trace id: `T011-reject-missing-ritual-access-feature`
- Reproduction seed or trace id: `T011-reject-non-leveled-ritual-spellbook-spell`
- Reproduction seed or trace id: `T011-reject-non-ritual-spellbook-spell`
- Reproduction seed or trace id: `T011-reject-prepared-only-ritual`

Harness artifacts:

- Start gate: `tasks/START_GATE.json`
- Engine depth: `tasks/ENGINE_DEPTH_MANIFEST.json`
- State ownership: `tasks/STATE_OWNER_MANIFEST.json`
- Reviewer loop: `tasks/REVIEW_LOOP.json`
- Decider decision: `tasks/DECIDER_DECISION.json`

Diagnostic tests:

- `src/tests/mod.rs::spellbook_ritual_adapter_replays_all_branches`
- `src/tests/mod.rs::spellbook_ritual_requires_spellbook_access_and_ritual_adept`

Remaining gaps:

- `_none_`

Verification results:

- `cargo fmt --check` passed.
- `cargo test` passed.
- `cargo clippy --all-targets -- -D warnings` passed.
- `node scripts/check-cleanroom-harness.cjs` passed.

## T010: character-sheet-hp-rest-hit-dice

- Manifest source commit SHA: `04249edf345a7752de2f1551dd3d509a2fffc160`
- Source branch inventory SHA: `b4e7e101def7969fc420563dc4da020c22e700f0dc0cc1d27accad6e8631225d`
- Driver: `cleanroom-input/qnt/character-sheet-runtime/character-sheet-hp-rest-hit-dice.mbt.qnt`
- Branch obligations:
  - `step:doCompleteLongRestRestoresHpHitPointDiceAndMaximum`
  - `step:doInterruptLongRestBeforeOneHourNoBenefit`
  - `step:doInterruptLongRestWithShortRestBenefits`
  - `step:doInterruptShortRestNoBenefit`
  - `step:doRejectLongRestBeforeSixteenHourWait`
  - `step:doRejectLongRestDurationTooShort`
  - `step:doRejectLongRestInterruptionAtRequiredDuration`
  - `step:doRejectLongRestPhysicalExertionTooShort`
  - `step:doRejectLongRestStartAtZeroHp`
  - `step:doRejectShortRestDurationTooShort`
  - `step:doRejectShortRestStartAtZeroHp`
  - `step:doSpendShortRestHitPointDiceSequentially`
  - `step:doSpendShortRestHitPointDie`
- Allowed inputs used:
  - `cleanroom-input/MANIFEST.md`
  - `cleanroom-input/branch-coverage/source-branch-inventory.json`
  - `cleanroom-input/qnt/character-sheet-runtime/character-sheet-hp-rest-hit-dice.mbt.qnt`
  - `cleanroom-input/qnt/shared-algebras/proofs/rule-core/hit-point-damage.qnt`
  - `cleanroom-input/qnt/shared-algebras/proofs/rule-core/hit-point-recovery.qnt`
  - `cleanroom-input/qnt/shared-algebras/proofs/rule-core/zero-hit-point-lifecycle.qnt`
  - `cleanroom-input/raw/srd-5.2.1/Rules-Glossary.md`
  - `cleanroom-input/raw/srd-5.2.1/Playing-the-Game.md`
  - `cleanroom-input/raw/srd-5.2.1/Character-Creation.md`
  - `cleanroom-input/domain/UBIQUITOUS_LANGUAGE.md`
  - `cleanroom-input/domain/CLEANROOM_ASSUMPTIONS.md`
  - `cleanroom-input/guidance/README.md`

Behavior implemented:

- Added reusable Character Sheet Hit Point state transitions for Hit Die healing, Long Rest reset benefits, interrupted Long Rest outcomes, and explicit rest rejection reasons.
- Reused feature-resource healing to clamp Short Rest Hit Die healing to the effective Hit Point Maximum.
- Modeled Long Rest benefits restoring all HP, spent Hit Dice, Temporary Hit Points expiry, and reduced Hit Point Maximum reset.
- Kept QNT action dispatch and witness field mapping quarantined in `src/qnt_adapters/character_sheet_hp_rest_hit_dice.rs`.

Generated branch coverage:

| Obligation | Target replay evidence | Diagnostic tests | Status |
| --- | --- | --- | --- |
| `cleanroom-input/qnt/character-sheet-runtime/character-sheet-hp-rest-hit-dice.mbt.qnt#step:doCompleteLongRestRestoresHpHitPointDiceAndMaximum` | `tasks/target-replay-evidence/T010-character-sheet-hp-rest-hit-dice.json#T010-complete-long-rest-restores-hp-hit-point-dice-and-maximum#step:doCompleteLongRestRestoresHpHitPointDiceAndMaximum` | `src/tests/mod.rs::hp_rest_hit_dice_adapter_replays_all_branches` | `covered` |
| `cleanroom-input/qnt/character-sheet-runtime/character-sheet-hp-rest-hit-dice.mbt.qnt#step:doInterruptLongRestBeforeOneHourNoBenefit` | `tasks/target-replay-evidence/T010-character-sheet-hp-rest-hit-dice.json#T010-interrupt-long-rest-before-one-hour-no-benefit#step:doInterruptLongRestBeforeOneHourNoBenefit` | `src/tests/mod.rs::hp_rest_hit_dice_adapter_replays_all_branches` | `covered` |
| `cleanroom-input/qnt/character-sheet-runtime/character-sheet-hp-rest-hit-dice.mbt.qnt#step:doInterruptLongRestWithShortRestBenefits` | `tasks/target-replay-evidence/T010-character-sheet-hp-rest-hit-dice.json#T010-interrupt-long-rest-with-short-rest-benefits#step:doInterruptLongRestWithShortRestBenefits` | `src/tests/mod.rs::hp_rest_hit_dice_adapter_replays_all_branches` | `covered` |
| `cleanroom-input/qnt/character-sheet-runtime/character-sheet-hp-rest-hit-dice.mbt.qnt#step:doInterruptShortRestNoBenefit` | `tasks/target-replay-evidence/T010-character-sheet-hp-rest-hit-dice.json#T010-interrupt-short-rest-no-benefit#step:doInterruptShortRestNoBenefit` | `src/tests/mod.rs::hp_rest_hit_dice_adapter_replays_all_branches` | `covered` |
| `cleanroom-input/qnt/character-sheet-runtime/character-sheet-hp-rest-hit-dice.mbt.qnt#step:doRejectLongRestBeforeSixteenHourWait` | `tasks/target-replay-evidence/T010-character-sheet-hp-rest-hit-dice.json#T010-reject-long-rest-before-sixteen-hour-wait#step:doRejectLongRestBeforeSixteenHourWait` | `src/tests/mod.rs::hp_rest_hit_dice_adapter_replays_all_branches` | `covered` |
| `cleanroom-input/qnt/character-sheet-runtime/character-sheet-hp-rest-hit-dice.mbt.qnt#step:doRejectLongRestDurationTooShort` | `tasks/target-replay-evidence/T010-character-sheet-hp-rest-hit-dice.json#T010-reject-long-rest-duration-too-short#step:doRejectLongRestDurationTooShort` | `src/tests/mod.rs::hp_rest_hit_dice_adapter_replays_all_branches` | `covered` |
| `cleanroom-input/qnt/character-sheet-runtime/character-sheet-hp-rest-hit-dice.mbt.qnt#step:doRejectLongRestInterruptionAtRequiredDuration` | `tasks/target-replay-evidence/T010-character-sheet-hp-rest-hit-dice.json#T010-reject-long-rest-interruption-at-required-duration#step:doRejectLongRestInterruptionAtRequiredDuration` | `src/tests/mod.rs::hp_rest_hit_dice_adapter_replays_all_branches` | `covered` |
| `cleanroom-input/qnt/character-sheet-runtime/character-sheet-hp-rest-hit-dice.mbt.qnt#step:doRejectLongRestPhysicalExertionTooShort` | `tasks/target-replay-evidence/T010-character-sheet-hp-rest-hit-dice.json#T010-reject-long-rest-physical-exertion-too-short#step:doRejectLongRestPhysicalExertionTooShort` | `src/tests/mod.rs::hp_rest_hit_dice_adapter_replays_all_branches` | `covered` |
| `cleanroom-input/qnt/character-sheet-runtime/character-sheet-hp-rest-hit-dice.mbt.qnt#step:doRejectLongRestStartAtZeroHp` | `tasks/target-replay-evidence/T010-character-sheet-hp-rest-hit-dice.json#T010-reject-long-rest-start-at-zero-hp#step:doRejectLongRestStartAtZeroHp` | `src/tests/mod.rs::hp_rest_hit_dice_adapter_replays_all_branches` | `covered` |
| `cleanroom-input/qnt/character-sheet-runtime/character-sheet-hp-rest-hit-dice.mbt.qnt#step:doRejectShortRestDurationTooShort` | `tasks/target-replay-evidence/T010-character-sheet-hp-rest-hit-dice.json#T010-reject-short-rest-duration-too-short#step:doRejectShortRestDurationTooShort` | `src/tests/mod.rs::hp_rest_hit_dice_adapter_replays_all_branches` | `covered` |
| `cleanroom-input/qnt/character-sheet-runtime/character-sheet-hp-rest-hit-dice.mbt.qnt#step:doRejectShortRestStartAtZeroHp` | `tasks/target-replay-evidence/T010-character-sheet-hp-rest-hit-dice.json#T010-reject-short-rest-start-at-zero-hp#step:doRejectShortRestStartAtZeroHp` | `src/tests/mod.rs::hp_rest_hit_dice_adapter_replays_all_branches` | `covered` |
| `cleanroom-input/qnt/character-sheet-runtime/character-sheet-hp-rest-hit-dice.mbt.qnt#step:doSpendShortRestHitPointDiceSequentially` | `tasks/target-replay-evidence/T010-character-sheet-hp-rest-hit-dice.json#T010-spend-short-rest-hit-point-dice-sequentially#step:doSpendShortRestHitPointDiceSequentially` | `src/tests/mod.rs::hp_rest_hit_dice_adapter_replays_all_branches` | `covered` |
| `cleanroom-input/qnt/character-sheet-runtime/character-sheet-hp-rest-hit-dice.mbt.qnt#step:doSpendShortRestHitPointDie` | `tasks/target-replay-evidence/T010-character-sheet-hp-rest-hit-dice.json#T010-spend-short-rest-hit-point-die#step:doSpendShortRestHitPointDie` | `src/tests/mod.rs::hp_rest_hit_dice_adapter_replays_all_branches` | `covered` |

Target replay evidence:

- Evidence file: `tasks/target-replay-evidence/T010-character-sheet-hp-rest-hit-dice.json`
- Target profile: `rust`
- Target profile SHA-256: `6d4cc6c6a4769962798133d57aff01438fb2b661941f71d1aa8a3333f4b7ecc1`
- Quint binding: Rust quint-connect harness
- Reproduction seed or trace id: `T010-complete-long-rest-restores-hp-hit-point-dice-and-maximum`
- Reproduction seed or trace id: `T010-interrupt-long-rest-before-one-hour-no-benefit`
- Reproduction seed or trace id: `T010-interrupt-long-rest-with-short-rest-benefits`
- Reproduction seed or trace id: `T010-interrupt-short-rest-no-benefit`
- Reproduction seed or trace id: `T010-reject-long-rest-before-sixteen-hour-wait`
- Reproduction seed or trace id: `T010-reject-long-rest-duration-too-short`
- Reproduction seed or trace id: `T010-reject-long-rest-interruption-at-required-duration`
- Reproduction seed or trace id: `T010-reject-long-rest-physical-exertion-too-short`
- Reproduction seed or trace id: `T010-reject-long-rest-start-at-zero-hp`
- Reproduction seed or trace id: `T010-reject-short-rest-duration-too-short`
- Reproduction seed or trace id: `T010-reject-short-rest-start-at-zero-hp`
- Reproduction seed or trace id: `T010-spend-short-rest-hit-point-dice-sequentially`
- Reproduction seed or trace id: `T010-spend-short-rest-hit-point-die`

Harness artifacts:

- Start gate: `tasks/START_GATE.json`
- Engine depth: `tasks/ENGINE_DEPTH_MANIFEST.json`
- State ownership: `tasks/STATE_OWNER_MANIFEST.json`
- Reviewer loop: `tasks/REVIEW_LOOP.json`
- Decider decision: `tasks/DECIDER_DECISION.json`

Diagnostic tests:

- `src/tests/mod.rs::hp_rest_hit_dice_adapter_replays_all_branches`
- `src/tests/mod.rs::hp_rest_hit_dice_spending_caps_healing_and_long_rest_resets`

Remaining gaps:

- `_none_`

Verification results:

- `cargo fmt --check` passed.
- `cargo test` passed.
- `cargo clippy --all-targets -- -D warnings` passed.
- `node scripts/check-cleanroom-harness.cjs` passed.

## T009: character-sheet-hit-point-maximum

- Manifest source commit SHA: `04249edf345a7752de2f1551dd3d509a2fffc160`
- Source branch inventory SHA: `b4e7e101def7969fc420563dc4da020c22e700f0dc0cc1d27accad6e8631225d`
- Driver: `cleanroom-input/qnt/character-sheet-runtime/character-sheet-hit-point-maximum.mbt.qnt`
- Branch obligations:
  - `step:doProjectFighterLevelOne`
  - `step:doProjectFighterLevelTwo`
  - `step:doProjectMinimumHigherLevelGain`
  - `step:doProjectReducedEffectiveMaximum`
  - `step:doProjectSorcererDraconicResilience`
  - `step:doProjectWizardFighterMulticlass`
- Allowed inputs used:
  - `cleanroom-input/MANIFEST.md`
  - `cleanroom-input/branch-coverage/source-branch-inventory.json`
  - `cleanroom-input/qnt/character-sheet-runtime/character-sheet-hit-point-maximum.mbt.qnt`
  - `cleanroom-input/qnt/shared-algebras/proofs/rule-core/hit-point-maximum.qnt`
  - `cleanroom-input/raw/srd-5.2.1/Character-Creation.md`
  - `cleanroom-input/raw/srd-5.2.1/Classes/Fighter.md`
  - `cleanroom-input/raw/srd-5.2.1/Classes/Sorcerer.md`
  - `cleanroom-input/raw/srd-5.2.1/Classes/Wizard.md`
  - `cleanroom-input/raw/srd-5.2.1/Rules-Glossary.md`
  - `cleanroom-input/domain/UBIQUITOUS_LANGUAGE.md`
  - `cleanroom-input/domain/CLEANROOM_ASSUMPTIONS.md`
  - `cleanroom-input/guidance/README.md`

Behavior implemented:

- Added reusable Hit Point Maximum facts and projection for level-1 starting Hit Die, fixed higher-level Hit Dice, Constitution modifier, bonuses, and reductions.
- Implemented fixed higher-level gains as `hitPointDie / 2 + 1 + Constitution modifier`, with the RAW minimum gain of 1.
- Projected normal and effective Hit Point Maximums, total Hit Dice, Wizard/Fighter multiclass arithmetic, Draconic Resilience bonus, and reduced effective maximum.
- Kept QNT action dispatch and witness field mapping quarantined in `src/qnt_adapters/character_sheet_hit_point_maximum.rs`.

Generated branch coverage:

| Obligation | Target replay evidence | Diagnostic tests | Status |
| --- | --- | --- | --- |
| `cleanroom-input/qnt/character-sheet-runtime/character-sheet-hit-point-maximum.mbt.qnt#step:doProjectFighterLevelOne` | `tasks/target-replay-evidence/T009-character-sheet-hit-point-maximum.json#T009-project-fighter-level-one#step:doProjectFighterLevelOne` | `src/tests/mod.rs::hit_point_maximum_adapter_replays_all_branches` | `covered` |
| `cleanroom-input/qnt/character-sheet-runtime/character-sheet-hit-point-maximum.mbt.qnt#step:doProjectFighterLevelTwo` | `tasks/target-replay-evidence/T009-character-sheet-hit-point-maximum.json#T009-project-fighter-level-two#step:doProjectFighterLevelTwo` | `src/tests/mod.rs::hit_point_maximum_adapter_replays_all_branches` | `covered` |
| `cleanroom-input/qnt/character-sheet-runtime/character-sheet-hit-point-maximum.mbt.qnt#step:doProjectMinimumHigherLevelGain` | `tasks/target-replay-evidence/T009-character-sheet-hit-point-maximum.json#T009-project-minimum-higher-level-gain#step:doProjectMinimumHigherLevelGain` | `src/tests/mod.rs::hit_point_maximum_adapter_replays_all_branches` | `covered` |
| `cleanroom-input/qnt/character-sheet-runtime/character-sheet-hit-point-maximum.mbt.qnt#step:doProjectReducedEffectiveMaximum` | `tasks/target-replay-evidence/T009-character-sheet-hit-point-maximum.json#T009-project-reduced-effective-maximum#step:doProjectReducedEffectiveMaximum` | `src/tests/mod.rs::hit_point_maximum_adapter_replays_all_branches` | `covered` |
| `cleanroom-input/qnt/character-sheet-runtime/character-sheet-hit-point-maximum.mbt.qnt#step:doProjectSorcererDraconicResilience` | `tasks/target-replay-evidence/T009-character-sheet-hit-point-maximum.json#T009-project-sorcerer-draconic-resilience#step:doProjectSorcererDraconicResilience` | `src/tests/mod.rs::hit_point_maximum_adapter_replays_all_branches` | `covered` |
| `cleanroom-input/qnt/character-sheet-runtime/character-sheet-hit-point-maximum.mbt.qnt#step:doProjectWizardFighterMulticlass` | `tasks/target-replay-evidence/T009-character-sheet-hit-point-maximum.json#T009-project-wizard-fighter-multiclass#step:doProjectWizardFighterMulticlass` | `src/tests/mod.rs::hit_point_maximum_adapter_replays_all_branches` | `covered` |

Target replay evidence:

- Evidence file: `tasks/target-replay-evidence/T009-character-sheet-hit-point-maximum.json`
- Target profile: `rust`
- Target profile SHA-256: `6d4cc6c6a4769962798133d57aff01438fb2b661941f71d1aa8a3333f4b7ecc1`
- Quint binding: Rust quint-connect harness
- Reproduction seed or trace id: `T009-project-fighter-level-one`
- Reproduction seed or trace id: `T009-project-fighter-level-two`
- Reproduction seed or trace id: `T009-project-minimum-higher-level-gain`
- Reproduction seed or trace id: `T009-project-reduced-effective-maximum`
- Reproduction seed or trace id: `T009-project-sorcerer-draconic-resilience`
- Reproduction seed or trace id: `T009-project-wizard-fighter-multiclass`

Harness artifacts:

- Start gate: `tasks/START_GATE.json`
- Engine depth: `tasks/ENGINE_DEPTH_MANIFEST.json`
- State ownership: `tasks/STATE_OWNER_MANIFEST.json`
- Reviewer loop: `tasks/REVIEW_LOOP.json`
- Decider decision: `tasks/DECIDER_DECISION.json`

Diagnostic tests:

- `src/tests/mod.rs::hit_point_maximum_adapter_replays_all_branches`
- `src/tests/mod.rs::hit_point_maximum_applies_fixed_gain_minimum_and_reduction`

Remaining gaps:

- `_none_`

Verification results:

- `cargo fmt --check` passed.
- `cargo test` passed.
- `cargo clippy --all-targets -- -D warnings` passed.
- `node scripts/check-cleanroom-harness.cjs` passed.

## T008: character-sheet-healing-resource-selected-identity

- Manifest source commit SHA: `04249edf345a7752de2f1551dd3d509a2fffc160`
- Source branch inventory SHA: `b4e7e101def7969fc420563dc4da020c22e700f0dc0cc1d27accad6e8631225d`
- Driver: `cleanroom-input/qnt/character-sheet-runtime/character-sheet-healing-resource-selected-identity.mbt.qnt`
- Branch obligations:
  - `step:doLayOnHandsRestoreHpAndRemovePoisoned`
- Allowed inputs used:
  - `cleanroom-input/MANIFEST.md`
  - `cleanroom-input/branch-coverage/source-branch-inventory.json`
  - `cleanroom-input/qnt/character-sheet-runtime/character-sheet-healing-resource-selected-identity.mbt.qnt`
  - `cleanroom-input/qnt/shared-algebras/proofs/rule-core/lay-on-hands-resource.qnt`
  - `cleanroom-input/qnt/shared-algebras/proofs/rule-core/feature-resource-pool.qnt`
  - `cleanroom-input/qnt/shared-algebras/proofs/rule-core/feature-resource-hit-point-healing.qnt`
  - `cleanroom-input/qnt/shared-algebras/proofs/rule-core/hit-point-recovery.qnt`
  - `cleanroom-input/qnt/shared-algebras/proofs/rule-core/hit-point-damage.qnt`
  - `cleanroom-input/raw/srd-5.2.1/Classes/Paladin.md`
  - `cleanroom-input/raw/srd-5.2.1/Playing-the-Game.md`
  - `cleanroom-input/raw/srd-5.2.1/Rules-Glossary.md`
  - `cleanroom-input/domain/UBIQUITOUS_LANGUAGE.md`
  - `cleanroom-input/domain/CLEANROOM_ASSUMPTIONS.md`
  - `cleanroom-input/guidance/README.md`

Behavior implemented:

- Added reusable Resource Pool facts and spend helpers for legal capacity, remaining points, and pool expenditure.
- Added feature-resource Hit Point healing with QNT/RAW clamping to the target's Hit Point Maximum.
- Added Lay On Hands projection that spends restored Hit Points plus 5 pool points per removed condition, keeps condition-removal spend separate from healing, and projects the resulting pool and target Hit Points.
- Kept QNT action dispatch and witness field mapping quarantined in `src/qnt_adapters/character_sheet_healing_resource_selected_identity.rs`.

Generated branch coverage:

| Obligation | Target replay evidence | Diagnostic tests | Status |
| --- | --- | --- | --- |
| `cleanroom-input/qnt/character-sheet-runtime/character-sheet-healing-resource-selected-identity.mbt.qnt#step:doLayOnHandsRestoreHpAndRemovePoisoned` | `tasks/target-replay-evidence/T008-character-sheet-healing-resource-selected-identity.json#T008-lay-on-hands-restore-hp-and-remove-poisoned#step:doLayOnHandsRestoreHpAndRemovePoisoned` | `src/tests/mod.rs::healing_resource_adapter_replays_lay_on_hands_branch` | `covered` |

Target replay evidence:

- Evidence file: `tasks/target-replay-evidence/T008-character-sheet-healing-resource-selected-identity.json`
- Target profile: `rust`
- Target profile SHA-256: `6d4cc6c6a4769962798133d57aff01438fb2b661941f71d1aa8a3333f4b7ecc1`
- Quint binding: Rust quint-connect harness
- Reproduction seed or trace id: `T008-lay-on-hands-restore-hp-and-remove-poisoned`

Harness artifacts:

- Start gate: `tasks/START_GATE.json`
- Engine depth: `tasks/ENGINE_DEPTH_MANIFEST.json`
- State ownership: `tasks/STATE_OWNER_MANIFEST.json`
- Reviewer loop: `tasks/REVIEW_LOOP.json`
- Decider decision: `tasks/DECIDER_DECISION.json`

Diagnostic tests:

- `src/tests/mod.rs::healing_resource_adapter_replays_lay_on_hands_branch`
- `src/tests/mod.rs::lay_on_hands_spends_condition_cost_separately_from_healing`

Remaining gaps:

- `_none_`

Verification results:

- `cargo fmt --check` passed.
- `cargo test` passed.
- `cargo clippy --all-targets -- -D warnings` passed.
- `node scripts/check-cleanroom-harness.cjs` passed.

## T007: character-sheet-armor-class-base-selected-identity

- Manifest source commit SHA: `04249edf345a7752de2f1551dd3d509a2fffc160`
- Source branch inventory SHA: `b4e7e101def7969fc420563dc4da020c22e700f0dc0cc1d27accad6e8631225d`
- Driver: `cleanroom-input/qnt/character-sheet-runtime/character-sheet-armor-class-base-selected-identity.mbt.qnt`
- Branch obligations:
  - `step:doProjectHeavyArmorWithShield`
  - `step:doProjectLightArmor`
  - `step:doProjectMediumArmorDexCap`
  - `step:doSelectBarbarianUnarmoredDefense`
  - `step:doSelectBarbarianUnarmoredDefenseWithShield`
  - `step:doSelectMonkUnarmoredDefense`
- Allowed inputs used:
  - `cleanroom-input/MANIFEST.md`
  - `cleanroom-input/branch-coverage/source-branch-inventory.json`
  - `cleanroom-input/qnt/character-sheet-runtime/character-sheet-armor-class-base-selected-identity.mbt.qnt`
  - `cleanroom-input/qnt/shared-algebras/proofs/rule-core/armor-class-base.qnt`
  - `cleanroom-input/raw/srd-5.2.1/Equipment.md`
  - `cleanroom-input/raw/srd-5.2.1/Classes/Barbarian.md`
  - `cleanroom-input/raw/srd-5.2.1/Classes/Monk.md`
  - `cleanroom-input/raw/srd-5.2.1/Character-Creation.md`
  - `cleanroom-input/raw/srd-5.2.1/Rules-Glossary.md`
  - `cleanroom-input/domain/UBIQUITOUS_LANGUAGE.md`
  - `cleanroom-input/domain/CLEANROOM_ASSUMPTIONS.md`
  - `cleanroom-input/guidance/README.md`

Behavior implemented:

- Added a reusable armor class base projection for ability-sum, Light armor, Medium armor with Dexterity cap, Heavy fixed armor, and trained Shield bonuses.
- Projected Barbarian and Monk Unarmored Defense formulas, including Barbarian's Shield-compatible case.
- Preserved the QNT distinction between the source base Armor Class number and the computed current Armor Class.
- Kept QNT action dispatch and witness field mapping quarantined in `src/qnt_adapters/character_sheet_armor_class_base_selected_identity.rs`.

Generated branch coverage:

| Obligation | Target replay evidence | Diagnostic tests | Status |
| --- | --- | --- | --- |
| `cleanroom-input/qnt/character-sheet-runtime/character-sheet-armor-class-base-selected-identity.mbt.qnt#step:doProjectHeavyArmorWithShield` | `tasks/target-replay-evidence/T007-character-sheet-armor-class-base-selected-identity.json#T007-project-heavy-armor-with-shield#step:doProjectHeavyArmorWithShield` | `src/tests/mod.rs::armor_class_adapter_replays_selected_identity_branches` | `covered` |
| `cleanroom-input/qnt/character-sheet-runtime/character-sheet-armor-class-base-selected-identity.mbt.qnt#step:doProjectLightArmor` | `tasks/target-replay-evidence/T007-character-sheet-armor-class-base-selected-identity.json#T007-project-light-armor#step:doProjectLightArmor` | `src/tests/mod.rs::armor_class_adapter_replays_selected_identity_branches` | `covered` |
| `cleanroom-input/qnt/character-sheet-runtime/character-sheet-armor-class-base-selected-identity.mbt.qnt#step:doProjectMediumArmorDexCap` | `tasks/target-replay-evidence/T007-character-sheet-armor-class-base-selected-identity.json#T007-project-medium-armor-dex-cap#step:doProjectMediumArmorDexCap` | `src/tests/mod.rs::armor_class_adapter_replays_selected_identity_branches` | `covered` |
| `cleanroom-input/qnt/character-sheet-runtime/character-sheet-armor-class-base-selected-identity.mbt.qnt#step:doSelectBarbarianUnarmoredDefense` | `tasks/target-replay-evidence/T007-character-sheet-armor-class-base-selected-identity.json#T007-select-barbarian-unarmored-defense#step:doSelectBarbarianUnarmoredDefense` | `src/tests/mod.rs::armor_class_adapter_replays_selected_identity_branches` | `covered` |
| `cleanroom-input/qnt/character-sheet-runtime/character-sheet-armor-class-base-selected-identity.mbt.qnt#step:doSelectBarbarianUnarmoredDefenseWithShield` | `tasks/target-replay-evidence/T007-character-sheet-armor-class-base-selected-identity.json#T007-select-barbarian-unarmored-defense-with-shield#step:doSelectBarbarianUnarmoredDefenseWithShield` | `src/tests/mod.rs::armor_class_adapter_replays_selected_identity_branches` | `covered` |
| `cleanroom-input/qnt/character-sheet-runtime/character-sheet-armor-class-base-selected-identity.mbt.qnt#step:doSelectMonkUnarmoredDefense` | `tasks/target-replay-evidence/T007-character-sheet-armor-class-base-selected-identity.json#T007-select-monk-unarmored-defense#step:doSelectMonkUnarmoredDefense` | `src/tests/mod.rs::armor_class_adapter_replays_selected_identity_branches` | `covered` |

Target replay evidence:

- Evidence file: `tasks/target-replay-evidence/T007-character-sheet-armor-class-base-selected-identity.json`
- Target profile: `rust`
- Target profile SHA-256: `6d4cc6c6a4769962798133d57aff01438fb2b661941f71d1aa8a3333f4b7ecc1`
- Quint binding: Rust quint-connect harness
- Reproduction seed or trace id: `T007-project-heavy-armor-with-shield`
- Reproduction seed or trace id: `T007-project-light-armor`
- Reproduction seed or trace id: `T007-project-medium-armor-dex-cap`
- Reproduction seed or trace id: `T007-select-barbarian-unarmored-defense`
- Reproduction seed or trace id: `T007-select-barbarian-unarmored-defense-with-shield`
- Reproduction seed or trace id: `T007-select-monk-unarmored-defense`

Harness artifacts:

- Start gate: `tasks/START_GATE.json`
- Engine depth: `tasks/ENGINE_DEPTH_MANIFEST.json`
- State ownership: `tasks/STATE_OWNER_MANIFEST.json`
- Reviewer loop: `tasks/REVIEW_LOOP.json`
- Decider decision: `tasks/DECIDER_DECISION.json`

Diagnostic tests:

- `src/tests/mod.rs::armor_class_adapter_replays_selected_identity_branches`
- `src/tests/mod.rs::armor_class_projection_caps_medium_dex_and_requires_trained_shield`

Remaining gaps:

- `_none_`

Verification results:

- `cargo fmt --check` passed.
- `cargo test` passed.
- `cargo clippy --all-targets -- -D warnings` passed.
- `node scripts/check-cleanroom-harness.cjs` passed.

## T006: character-sheet-ability-check-proficiency-bonus

- Manifest source commit SHA: `04249edf345a7752de2f1551dd3d509a2fffc160`
- Source branch inventory SHA: `b4e7e101def7969fc420563dc4da020c22e700f0dc0cc1d27accad6e8631225d`
- Driver: `cleanroom-input/qnt/character-sheet-runtime/character-sheet-ability-check-proficiency-bonus.mbt.qnt`
- Branch obligations:
  - `step:doProjectExpertise`
  - `step:doProjectJackOfAllTradesLevelTwo`
  - `step:doProjectJackOfAllTradesRoundedDown`
  - `step:doProjectSkillProficiency`
  - `step:doRejectMissingBardLevelTwo`
  - `step:doRejectOtherProficiencyBonus`
- Allowed inputs used:
  - `cleanroom-input/MANIFEST.md`
  - `cleanroom-input/branch-coverage/source-branch-inventory.json`
  - `cleanroom-input/qnt/character-sheet-runtime/character-sheet-ability-check-proficiency-bonus.mbt.qnt`
  - `cleanroom-input/qnt/shared-algebras/proofs/rule-core/ability-check-proficiency-bonus.qnt`
  - `cleanroom-input/raw/srd-5.2.1/Classes/Bard.md`
  - `cleanroom-input/raw/srd-5.2.1/Classes/Rogue.md`
  - `cleanroom-input/raw/srd-5.2.1/Playing-the-Game.md`
  - `cleanroom-input/raw/srd-5.2.1/Character-Creation.md`
  - `cleanroom-input/raw/srd-5.2.1/Rules-Glossary.md`
  - `cleanroom-input/domain/UBIQUITOUS_LANGUAGE.md`
  - `cleanroom-input/domain/CLEANROOM_ASSUMPTIONS.md`
  - `cleanroom-input/guidance/README.md`

Behavior implemented:

- Added a reusable sheet rule for ability-check proficiency bonus projection from skill training, Jack of All Trades state, and other-proficiency-bonus state.
- Projected ordinary skill proficiency, Expertise doubling, Jack of All Trades at level 2, and Jack of All Trades rounded-down behavior.
- Projected no-bonus outcomes when Jack of All Trades is absent or another proficiency bonus already applies.
- Kept QNT action dispatch and witness field mapping quarantined in `src/qnt_adapters/character_sheet_ability_check_proficiency_bonus.rs`.

Generated branch coverage:

| Obligation | Target replay evidence | Diagnostic tests | Status |
| --- | --- | --- | --- |
| `cleanroom-input/qnt/character-sheet-runtime/character-sheet-ability-check-proficiency-bonus.mbt.qnt#step:doProjectExpertise` | `tasks/target-replay-evidence/T006-character-sheet-ability-check-proficiency-bonus.json#T006-project-expertise#step:doProjectExpertise` | `src/tests/mod.rs::ability_check_proficiency_adapter_replays_all_branches` | `covered` |
| `cleanroom-input/qnt/character-sheet-runtime/character-sheet-ability-check-proficiency-bonus.mbt.qnt#step:doProjectJackOfAllTradesLevelTwo` | `tasks/target-replay-evidence/T006-character-sheet-ability-check-proficiency-bonus.json#T006-project-jack-of-all-trades-level-two#step:doProjectJackOfAllTradesLevelTwo` | `src/tests/mod.rs::ability_check_proficiency_adapter_replays_all_branches` | `covered` |
| `cleanroom-input/qnt/character-sheet-runtime/character-sheet-ability-check-proficiency-bonus.mbt.qnt#step:doProjectJackOfAllTradesRoundedDown` | `tasks/target-replay-evidence/T006-character-sheet-ability-check-proficiency-bonus.json#T006-project-jack-of-all-trades-rounded-down#step:doProjectJackOfAllTradesRoundedDown` | `src/tests/mod.rs::ability_check_proficiency_adapter_replays_all_branches` | `covered` |
| `cleanroom-input/qnt/character-sheet-runtime/character-sheet-ability-check-proficiency-bonus.mbt.qnt#step:doProjectSkillProficiency` | `tasks/target-replay-evidence/T006-character-sheet-ability-check-proficiency-bonus.json#T006-project-skill-proficiency#step:doProjectSkillProficiency` | `src/tests/mod.rs::ability_check_proficiency_adapter_replays_all_branches` | `covered` |
| `cleanroom-input/qnt/character-sheet-runtime/character-sheet-ability-check-proficiency-bonus.mbt.qnt#step:doRejectMissingBardLevelTwo` | `tasks/target-replay-evidence/T006-character-sheet-ability-check-proficiency-bonus.json#T006-reject-missing-bard-level-two#step:doRejectMissingBardLevelTwo` | `src/tests/mod.rs::ability_check_proficiency_adapter_replays_all_branches` | `covered` |
| `cleanroom-input/qnt/character-sheet-runtime/character-sheet-ability-check-proficiency-bonus.mbt.qnt#step:doRejectOtherProficiencyBonus` | `tasks/target-replay-evidence/T006-character-sheet-ability-check-proficiency-bonus.json#T006-reject-other-proficiency-bonus#step:doRejectOtherProficiencyBonus` | `src/tests/mod.rs::ability_check_proficiency_adapter_replays_all_branches` | `covered` |

Target replay evidence:

- Evidence file: `tasks/target-replay-evidence/T006-character-sheet-ability-check-proficiency-bonus.json`
- Target profile: `rust`
- Target profile SHA-256: `6d4cc6c6a4769962798133d57aff01438fb2b661941f71d1aa8a3333f4b7ecc1`
- Quint binding: Rust quint-connect harness
- Reproduction seed or trace id: `T006-project-expertise`
- Reproduction seed or trace id: `T006-project-jack-of-all-trades-level-two`
- Reproduction seed or trace id: `T006-project-jack-of-all-trades-rounded-down`
- Reproduction seed or trace id: `T006-project-skill-proficiency`
- Reproduction seed or trace id: `T006-reject-missing-bard-level-two`
- Reproduction seed or trace id: `T006-reject-other-proficiency-bonus`

Harness artifacts:

- Start gate: `tasks/START_GATE.json`
- Engine depth: `tasks/ENGINE_DEPTH_MANIFEST.json`
- State ownership: `tasks/STATE_OWNER_MANIFEST.json`
- Reviewer loop: `tasks/REVIEW_LOOP.json`
- Decider decision: `tasks/DECIDER_DECISION.json`

Diagnostic tests:

- `src/tests/mod.rs::ability_check_proficiency_adapter_replays_all_branches`
- `src/tests/mod.rs::ability_check_proficiency_prefers_training_over_jack_of_all_trades`

Remaining gaps:

- `_none_`

Verification results:

- `cargo fmt --check` passed.
- `cargo test` passed.
- `cargo clippy --all-targets -- -D warnings` passed.
- `node scripts/check-cleanroom-harness.cjs` passed.

## T005: character-creation-weapon-mastery-containers-selected-identity

- Manifest source commit SHA: `04249edf345a7752de2f1551dd3d509a2fffc160`
- Source branch inventory SHA: `b4e7e101def7969fc420563dc4da020c22e700f0dc0cc1d27accad6e8631225d`
- Driver: `cleanroom-input/qnt/character-creation-runtime/character-creation-weapon-mastery-containers-selected-identity.mbt.qnt`
- Branch obligations:
  - `step:doFinalizeBarbarianWeaponMastery`
  - `step:doFinalizeFighterWeaponMastery`
  - `step:doFinalizePaladinWeaponMastery`
  - `step:doFinalizeRangerWeaponMastery`
  - `step:doFinalizeRogueWeaponMastery`
- Allowed inputs used:
  - `cleanroom-input/MANIFEST.md`
  - `cleanroom-input/branch-coverage/source-branch-inventory.json`
  - `cleanroom-input/qnt/character-creation-runtime/character-creation-weapon-mastery-containers-selected-identity.mbt.qnt`
  - `cleanroom-input/raw/srd-5.2.1/Classes/Barbarian.md`
  - `cleanroom-input/raw/srd-5.2.1/Classes/Fighter.md`
  - `cleanroom-input/raw/srd-5.2.1/Classes/Paladin.md`
  - `cleanroom-input/raw/srd-5.2.1/Classes/Ranger.md`
  - `cleanroom-input/raw/srd-5.2.1/Classes/Rogue.md`
  - `cleanroom-input/raw/srd-5.2.1/Equipment.md`
  - `cleanroom-input/domain/UBIQUITOUS_LANGUAGE.md`
  - `cleanroom-input/domain/CLEANROOM_ASSUMPTIONS.md`
  - `cleanroom-input/guidance/README.md`

Behavior implemented:

- Added reusable level-1 Weapon Mastery container projections for Barbarian, Fighter, Paladin, Ranger, and Rogue.
- Enforced the level-1 selected weapon count from the copied RAW/QNT model: Fighter selects three weapons; Barbarian, Paladin, Ranger, and Rogue select two.
- Projected selected weapon order, build feature count, open hole count, and total level for the selected-identity witness.
- Kept QNT action dispatch and witness field mapping quarantined in `src/qnt_adapters/character_creation_weapon_mastery_containers_selected_identity.rs`.

Generated branch coverage:

| Obligation | Target replay evidence | Diagnostic tests | Status |
| --- | --- | --- | --- |
| `cleanroom-input/qnt/character-creation-runtime/character-creation-weapon-mastery-containers-selected-identity.mbt.qnt#step:doFinalizeBarbarianWeaponMastery` | `tasks/target-replay-evidence/T005-character-creation-weapon-mastery-containers-selected-identity.json#T005-finalize-barbarian-weapon-mastery#step:doFinalizeBarbarianWeaponMastery` | `src/tests/mod.rs::weapon_mastery_adapter_replays_container_identity_branches` | `covered` |
| `cleanroom-input/qnt/character-creation-runtime/character-creation-weapon-mastery-containers-selected-identity.mbt.qnt#step:doFinalizeFighterWeaponMastery` | `tasks/target-replay-evidence/T005-character-creation-weapon-mastery-containers-selected-identity.json#T005-finalize-fighter-weapon-mastery#step:doFinalizeFighterWeaponMastery` | `src/tests/mod.rs::weapon_mastery_adapter_replays_container_identity_branches` | `covered` |
| `cleanroom-input/qnt/character-creation-runtime/character-creation-weapon-mastery-containers-selected-identity.mbt.qnt#step:doFinalizePaladinWeaponMastery` | `tasks/target-replay-evidence/T005-character-creation-weapon-mastery-containers-selected-identity.json#T005-finalize-paladin-weapon-mastery#step:doFinalizePaladinWeaponMastery` | `src/tests/mod.rs::weapon_mastery_adapter_replays_container_identity_branches` | `covered` |
| `cleanroom-input/qnt/character-creation-runtime/character-creation-weapon-mastery-containers-selected-identity.mbt.qnt#step:doFinalizeRangerWeaponMastery` | `tasks/target-replay-evidence/T005-character-creation-weapon-mastery-containers-selected-identity.json#T005-finalize-ranger-weapon-mastery#step:doFinalizeRangerWeaponMastery` | `src/tests/mod.rs::weapon_mastery_adapter_replays_container_identity_branches` | `covered` |
| `cleanroom-input/qnt/character-creation-runtime/character-creation-weapon-mastery-containers-selected-identity.mbt.qnt#step:doFinalizeRogueWeaponMastery` | `tasks/target-replay-evidence/T005-character-creation-weapon-mastery-containers-selected-identity.json#T005-finalize-rogue-weapon-mastery#step:doFinalizeRogueWeaponMastery` | `src/tests/mod.rs::weapon_mastery_adapter_replays_container_identity_branches` | `covered` |

Target replay evidence:

- Evidence file: `tasks/target-replay-evidence/T005-character-creation-weapon-mastery-containers-selected-identity.json`
- Target profile: `rust`
- Target profile SHA-256: `6d4cc6c6a4769962798133d57aff01438fb2b661941f71d1aa8a3333f4b7ecc1`
- Quint binding: Rust quint-connect harness
- Reproduction seed or trace id: `T005-finalize-barbarian-weapon-mastery`
- Reproduction seed or trace id: `T005-finalize-fighter-weapon-mastery`
- Reproduction seed or trace id: `T005-finalize-paladin-weapon-mastery`
- Reproduction seed or trace id: `T005-finalize-ranger-weapon-mastery`
- Reproduction seed or trace id: `T005-finalize-rogue-weapon-mastery`

Harness artifacts:

- Start gate: `tasks/START_GATE.json`
- Engine depth: `tasks/ENGINE_DEPTH_MANIFEST.json`
- State ownership: `tasks/STATE_OWNER_MANIFEST.json`
- Reviewer loop: `tasks/REVIEW_LOOP.json`
- Decider decision: `tasks/DECIDER_DECISION.json`

Diagnostic tests:

- `src/tests/mod.rs::weapon_mastery_adapter_replays_container_identity_branches`
- `src/tests/mod.rs::weapon_mastery_projection_enforces_class_choice_count`

Remaining gaps:

- `_none_`

Verification results:

- `cargo fmt --check` passed.
- `cargo test` passed.
- `cargo clippy --all-targets -- -D warnings` passed.
- `node scripts/check-cleanroom-harness.cjs` passed.

## T004: character-creation-runtime

- Manifest source commit SHA: `04249edf345a7752de2f1551dd3d509a2fffc160`
- Source branch inventory SHA: `b4e7e101def7969fc420563dc4da020c22e700f0dc0cc1d27accad6e8631225d`
- Driver: `cleanroom-input/qnt/character-creation-runtime/character-creation-runtime.mbt.qnt`
- Branch obligations:
  - `step:doFillAbilityScoresOnly`
  - `step:doFillInitialChoicesOnly`
  - `step:doFillInitialManifest`
  - `step:doFillManifestChoices`
  - `step:doFillManifestLoadout`
  - `step:doFillManifestPurchase`
  - `step:doRejectClosedInitialProgressionHole`
  - `step:doRejectDuplicateFill`
  - `step:doRejectDuplicateLanguage`
  - `step:doRejectStaleInitialManifest`
  - `step:doRejectTooFewLanguages`
  - `step:doRejectTooManyLanguages`
  - `step:doRejectUnknownLoadoutArmor`
  - `step:doRejectUnsupportedClassEquipment`
  - `step:doRejectUnsupportedLanguage`
  - `step:doRejectWrongKindPrimaryClass`
- Allowed inputs used:
  - `cleanroom-input/MANIFEST.md`
  - `cleanroom-input/branch-coverage/source-branch-inventory.json`
  - `cleanroom-input/qnt/character-creation-runtime/character-creation-runtime.mbt.qnt`
  - `cleanroom-input/qnt/character-creation-runtime/character-creation-runtime-slice.qnt`
  - `cleanroom-input/raw/srd-5.2.1/Character-Creation.md`
  - `cleanroom-input/raw/srd-5.2.1/Character-Origins.md`
  - `cleanroom-input/raw/srd-5.2.1/Classes/Fighter.md`
  - `cleanroom-input/raw/srd-5.2.1/Equipment.md`
  - `cleanroom-input/domain/UBIQUITOUS_LANGUAGE.md`
  - `cleanroom-input/domain/CLEANROOM_ASSUMPTIONS.md`
  - `cleanroom-input/guidance/README.md`

Behavior implemented:

- Added a reusable character creation draft engine for the `character-creation-runtime-slice.qnt` Fighter/Soldier/Orc manifest path.
- Implemented open hole discovery, draft finalization, accepted fill application, and rejected batch reporting for stale revisions, duplicate fills, wrong fill kinds, invalid choices, unsupported choices, and choice cardinality failures.
- Replayed the initial manifest, later choice fills, equipment purchase, and loadout progression through `Ready` finalization.
- Kept QNT action dispatch and witness field mapping quarantined in `src/qnt_adapters/character_creation_runtime.rs`.

Generated branch coverage:

| Obligation | Target replay evidence | Diagnostic tests | Status |
| --- | --- | --- | --- |
| `cleanroom-input/qnt/character-creation-runtime/character-creation-runtime.mbt.qnt#step:doFillAbilityScoresOnly` | `tasks/target-replay-evidence/T004-character-creation-runtime.json#T004-fill-ability-scores-only#step:doFillAbilityScoresOnly` | `src/tests/mod.rs::character_creation_runtime_adapter_replays_all_branch_actions` | `covered` |
| `cleanroom-input/qnt/character-creation-runtime/character-creation-runtime.mbt.qnt#step:doFillInitialChoicesOnly` | `tasks/target-replay-evidence/T004-character-creation-runtime.json#T004-fill-initial-choices-only#step:doFillInitialChoicesOnly` | `src/tests/mod.rs::character_creation_runtime_adapter_replays_all_branch_actions` | `covered` |
| `cleanroom-input/qnt/character-creation-runtime/character-creation-runtime.mbt.qnt#step:doFillInitialManifest` | `tasks/target-replay-evidence/T004-character-creation-runtime.json#T004-fill-initial-manifest#step:doFillInitialManifest` | `src/tests/mod.rs::character_creation_runtime_accepts_manifest_progression` | `covered` |
| `cleanroom-input/qnt/character-creation-runtime/character-creation-runtime.mbt.qnt#step:doFillManifestChoices` | `tasks/target-replay-evidence/T004-character-creation-runtime.json#T004-fill-manifest-choices#step:doFillManifestChoices` | `src/tests/mod.rs::character_creation_runtime_accepts_manifest_progression` | `covered` |
| `cleanroom-input/qnt/character-creation-runtime/character-creation-runtime.mbt.qnt#step:doFillManifestLoadout` | `tasks/target-replay-evidence/T004-character-creation-runtime.json#T004-fill-manifest-loadout#step:doFillManifestLoadout` | `src/tests/mod.rs::character_creation_runtime_accepts_manifest_progression` | `covered` |
| `cleanroom-input/qnt/character-creation-runtime/character-creation-runtime.mbt.qnt#step:doFillManifestPurchase` | `tasks/target-replay-evidence/T004-character-creation-runtime.json#T004-fill-manifest-purchase#step:doFillManifestPurchase` | `src/tests/mod.rs::character_creation_runtime_accepts_manifest_progression` | `covered` |
| `cleanroom-input/qnt/character-creation-runtime/character-creation-runtime.mbt.qnt#step:doRejectClosedInitialProgressionHole` | `tasks/target-replay-evidence/T004-character-creation-runtime.json#T004-reject-closed-initial-progression-hole#step:doRejectClosedInitialProgressionHole` | `src/tests/mod.rs::character_creation_runtime_rejects_manifest_protocol_issues` | `covered` |
| `cleanroom-input/qnt/character-creation-runtime/character-creation-runtime.mbt.qnt#step:doRejectDuplicateFill` | `tasks/target-replay-evidence/T004-character-creation-runtime.json#T004-reject-duplicate-fill#step:doRejectDuplicateFill` | `src/tests/mod.rs::character_creation_runtime_rejects_manifest_protocol_issues` | `covered` |
| `cleanroom-input/qnt/character-creation-runtime/character-creation-runtime.mbt.qnt#step:doRejectDuplicateLanguage` | `tasks/target-replay-evidence/T004-character-creation-runtime.json#T004-reject-duplicate-language#step:doRejectDuplicateLanguage` | `src/tests/mod.rs::character_creation_runtime_rejects_manifest_protocol_issues` | `covered` |
| `cleanroom-input/qnt/character-creation-runtime/character-creation-runtime.mbt.qnt#step:doRejectStaleInitialManifest` | `tasks/target-replay-evidence/T004-character-creation-runtime.json#T004-reject-stale-initial-manifest#step:doRejectStaleInitialManifest` | `src/tests/mod.rs::character_creation_runtime_rejects_manifest_protocol_issues` | `covered` |
| `cleanroom-input/qnt/character-creation-runtime/character-creation-runtime.mbt.qnt#step:doRejectTooFewLanguages` | `tasks/target-replay-evidence/T004-character-creation-runtime.json#T004-reject-too-few-languages#step:doRejectTooFewLanguages` | `src/tests/mod.rs::character_creation_runtime_rejects_manifest_protocol_issues` | `covered` |
| `cleanroom-input/qnt/character-creation-runtime/character-creation-runtime.mbt.qnt#step:doRejectTooManyLanguages` | `tasks/target-replay-evidence/T004-character-creation-runtime.json#T004-reject-too-many-languages#step:doRejectTooManyLanguages` | `src/tests/mod.rs::character_creation_runtime_rejects_manifest_protocol_issues` | `covered` |
| `cleanroom-input/qnt/character-creation-runtime/character-creation-runtime.mbt.qnt#step:doRejectUnknownLoadoutArmor` | `tasks/target-replay-evidence/T004-character-creation-runtime.json#T004-reject-unknown-loadout-armor#step:doRejectUnknownLoadoutArmor` | `src/tests/mod.rs::character_creation_runtime_rejects_manifest_protocol_issues` | `covered` |
| `cleanroom-input/qnt/character-creation-runtime/character-creation-runtime.mbt.qnt#step:doRejectUnsupportedClassEquipment` | `tasks/target-replay-evidence/T004-character-creation-runtime.json#T004-reject-unsupported-class-equipment#step:doRejectUnsupportedClassEquipment` | `src/tests/mod.rs::character_creation_runtime_rejects_manifest_protocol_issues` | `covered` |
| `cleanroom-input/qnt/character-creation-runtime/character-creation-runtime.mbt.qnt#step:doRejectUnsupportedLanguage` | `tasks/target-replay-evidence/T004-character-creation-runtime.json#T004-reject-unsupported-language#step:doRejectUnsupportedLanguage` | `src/tests/mod.rs::character_creation_runtime_rejects_manifest_protocol_issues` | `covered` |
| `cleanroom-input/qnt/character-creation-runtime/character-creation-runtime.mbt.qnt#step:doRejectWrongKindPrimaryClass` | `tasks/target-replay-evidence/T004-character-creation-runtime.json#T004-reject-wrong-kind-primary-class#step:doRejectWrongKindPrimaryClass` | `src/tests/mod.rs::character_creation_runtime_rejects_manifest_protocol_issues` | `covered` |

Target replay evidence:

- Evidence file: `tasks/target-replay-evidence/T004-character-creation-runtime.json`
- Target profile: `rust`
- Target profile SHA-256: `6d4cc6c6a4769962798133d57aff01438fb2b661941f71d1aa8a3333f4b7ecc1`
- Quint binding: Rust quint-connect harness
- Reproduction seed or trace id: `T004-fill-ability-scores-only`
- Reproduction seed or trace id: `T004-fill-initial-choices-only`
- Reproduction seed or trace id: `T004-fill-initial-manifest`
- Reproduction seed or trace id: `T004-fill-manifest-choices`
- Reproduction seed or trace id: `T004-fill-manifest-loadout`
- Reproduction seed or trace id: `T004-fill-manifest-purchase`
- Reproduction seed or trace id: `T004-reject-closed-initial-progression-hole`
- Reproduction seed or trace id: `T004-reject-duplicate-fill`
- Reproduction seed or trace id: `T004-reject-duplicate-language`
- Reproduction seed or trace id: `T004-reject-stale-initial-manifest`
- Reproduction seed or trace id: `T004-reject-too-few-languages`
- Reproduction seed or trace id: `T004-reject-too-many-languages`
- Reproduction seed or trace id: `T004-reject-unknown-loadout-armor`
- Reproduction seed or trace id: `T004-reject-unsupported-class-equipment`
- Reproduction seed or trace id: `T004-reject-unsupported-language`
- Reproduction seed or trace id: `T004-reject-wrong-kind-primary-class`

Harness artifacts:

- Start gate: `tasks/START_GATE.json`
- Engine depth: `tasks/ENGINE_DEPTH_MANIFEST.json`
- State ownership: `tasks/STATE_OWNER_MANIFEST.json`
- Reviewer loop: `tasks/REVIEW_LOOP.json`
- Decider decision: `tasks/DECIDER_DECISION.json`

Diagnostic tests:

- `src/tests/mod.rs::character_creation_runtime_adapter_replays_all_branch_actions`
- `src/tests/mod.rs::character_creation_runtime_accepts_manifest_progression`
- `src/tests/mod.rs::character_creation_runtime_rejects_manifest_protocol_issues`

Remaining gaps:

- `_none_`

Verification results:

- `cargo fmt --check` passed.
- `cargo test` passed.
- `cargo clippy --all-targets -- -D warnings` passed.
- `node scripts/check-cleanroom-harness.cjs` passed.

## T003: character-creation-fighter-fighting-style-selected-identity

- Manifest source commit SHA: `04249edf345a7752de2f1551dd3d509a2fffc160`
- Source branch inventory SHA: `b4e7e101def7969fc420563dc4da020c22e700f0dc0cc1d27accad6e8631225d`
- Driver: `cleanroom-input/qnt/character-creation-runtime/character-creation-fighter-fighting-style-selected-identity.mbt.qnt`
- Branch obligations:
  - `step:doSelectDefenseFightingStyle`
  - `step:doReplaceDefenseWithArcheryOnFighterLevelGain`
- Allowed inputs used:
  - `cleanroom-input/MANIFEST.md`
  - `cleanroom-input/branch-coverage/source-branch-inventory.json`
  - `cleanroom-input/qnt/character-creation-runtime/character-creation-fighter-fighting-style-selected-identity.mbt.qnt`
  - `cleanroom-input/raw/srd-5.2.1/Classes/Fighter.md`
  - `cleanroom-input/raw/srd-5.2.1/Feats.md`
  - `cleanroom-input/domain/UBIQUITOUS_LANGUAGE.md`
  - `cleanroom-input/domain/CLEANROOM_ASSUMPTIONS.md`
  - `cleanroom-input/guidance/README.md`

Behavior implemented:

- Added reusable Fighter Fighting Style projection facts for initial feat selection and Fighter-level-gain replacement.
- Projected Defense as the level-1 selected Fighting Style feat and Archery as the level-2 replacement in the QNT witness path.
- Preserved the previously selected feat in replacement projections while exposing the currently selected feat for downstream character creation state.
- Kept QNT action dispatch and witness field mapping quarantined in `src/qnt_adapters/character_creation_fighter_fighting_style_selected_identity.rs`.

Generated branch coverage:

| Obligation | Target replay evidence | Diagnostic tests | Status |
| --- | --- | --- | --- |
| `cleanroom-input/qnt/character-creation-runtime/character-creation-fighter-fighting-style-selected-identity.mbt.qnt#step:doSelectDefenseFightingStyle` | `tasks/target-replay-evidence/T003-character-creation-fighter-fighting-style-selected-identity.json#T003-select-defense#step:doSelectDefenseFightingStyle` | `src/tests/mod.rs::fighter_fighting_style_adapter_replays_selected_identity_branches` | `covered` |
| `cleanroom-input/qnt/character-creation-runtime/character-creation-fighter-fighting-style-selected-identity.mbt.qnt#step:doReplaceDefenseWithArcheryOnFighterLevelGain` | `tasks/target-replay-evidence/T003-character-creation-fighter-fighting-style-selected-identity.json#T003-replace-defense-with-archery#step:doReplaceDefenseWithArcheryOnFighterLevelGain` | `src/tests/mod.rs::fighter_fighting_style_adapter_replays_selected_identity_branches` | `covered` |

Target replay evidence:

- Evidence file: `tasks/target-replay-evidence/T003-character-creation-fighter-fighting-style-selected-identity.json`
- Target profile: `rust`
- Target profile SHA-256: `6d4cc6c6a4769962798133d57aff01438fb2b661941f71d1aa8a3333f4b7ecc1`
- Quint binding: Rust quint-connect harness
- Reproduction seed or trace id: `T003-select-defense`
- Reproduction seed or trace id: `T003-replace-defense-with-archery`

Harness artifacts:

- Start gate: `tasks/START_GATE.json`
- Engine depth: `tasks/ENGINE_DEPTH_MANIFEST.json`
- State ownership: `tasks/STATE_OWNER_MANIFEST.json`
- Reviewer loop: `tasks/REVIEW_LOOP.json`
- Decider decision: `tasks/DECIDER_DECISION.json`

Diagnostic tests:

- `src/tests/mod.rs::fighter_fighting_style_replacement_records_previous_and_new_feat`

Remaining gaps:

- `_none_`

Verification results:

- `cargo fmt --check` passed.
- `cargo test` passed.
- `cargo clippy --all-targets -- -D warnings` passed.
- `node scripts/check-cleanroom-harness.cjs` passed.

## T002: character-creation-cleric-druid-order-selected-identity

- Manifest source commit SHA: `04249edf345a7752de2f1551dd3d509a2fffc160`
- Source branch inventory SHA: `b4e7e101def7969fc420563dc4da020c22e700f0dc0cc1d27accad6e8631225d`
- Driver: `cleanroom-input/qnt/character-creation-runtime/character-creation-cleric-druid-order-selected-identity.mbt.qnt`
- Branch obligations:
  - `step:doSelectClericProtectorOrder`
  - `step:doSelectClericThaumaturgeOrder`
  - `step:doSelectDruidMagicianOrder`
  - `step:doSelectDruidWardenOrder`
- Allowed inputs used:
  - `cleanroom-input/MANIFEST.md`
  - `cleanroom-input/branch-coverage/source-branch-inventory.json`
  - `cleanroom-input/qnt/character-creation-runtime/character-creation-cleric-druid-order-selected-identity.mbt.qnt`
  - `cleanroom-input/raw/srd-5.2.1/Classes/Cleric.md`
  - `cleanroom-input/raw/srd-5.2.1/Classes/Druid.md`
  - `cleanroom-input/domain/UBIQUITOUS_LANGUAGE.md`
  - `cleanroom-input/domain/CLEANROOM_ASSUMPTIONS.md`
  - `cleanroom-input/guidance/README.md`

Behavior implemented:

- Added reusable level-1 class order projections for Cleric Divine Order and Druid Primal Order.
- Projected Protector and Warden martial weapon and armor training, preserving base class Medium armor where RAW grants it before the order choice.
- Projected Thaumaturge and Magician extra cantrip selections and Wisdom-minimum Intelligence ability-check bonuses for the QNT-selected skills.
- Kept QNT action dispatch and witness field mapping quarantined in `src/qnt_adapters/character_creation_cleric_druid_order_selected_identity.rs`.

Generated branch coverage:

| Obligation | Target replay evidence | Diagnostic tests | Status |
| --- | --- | --- | --- |
| `cleanroom-input/qnt/character-creation-runtime/character-creation-cleric-druid-order-selected-identity.mbt.qnt#step:doSelectClericProtectorOrder` | `tasks/target-replay-evidence/T002-character-creation-cleric-druid-order-selected-identity.json#T002-cleric-protector#step:doSelectClericProtectorOrder` | `src/tests/mod.rs::cleric_druid_order_adapter_replays_all_selected_identity_branches` | `covered` |
| `cleanroom-input/qnt/character-creation-runtime/character-creation-cleric-druid-order-selected-identity.mbt.qnt#step:doSelectClericThaumaturgeOrder` | `tasks/target-replay-evidence/T002-character-creation-cleric-druid-order-selected-identity.json#T002-cleric-thaumaturge#step:doSelectClericThaumaturgeOrder` | `src/tests/mod.rs::cleric_druid_order_adapter_replays_all_selected_identity_branches` | `covered` |
| `cleanroom-input/qnt/character-creation-runtime/character-creation-cleric-druid-order-selected-identity.mbt.qnt#step:doSelectDruidMagicianOrder` | `tasks/target-replay-evidence/T002-character-creation-cleric-druid-order-selected-identity.json#T002-druid-magician#step:doSelectDruidMagicianOrder` | `src/tests/mod.rs::cleric_druid_order_adapter_replays_all_selected_identity_branches` | `covered` |
| `cleanroom-input/qnt/character-creation-runtime/character-creation-cleric-druid-order-selected-identity.mbt.qnt#step:doSelectDruidWardenOrder` | `tasks/target-replay-evidence/T002-character-creation-cleric-druid-order-selected-identity.json#T002-druid-warden#step:doSelectDruidWardenOrder` | `src/tests/mod.rs::cleric_druid_order_adapter_replays_all_selected_identity_branches` | `covered` |

Target replay evidence:

- Evidence file: `tasks/target-replay-evidence/T002-character-creation-cleric-druid-order-selected-identity.json`
- Target profile: `rust`
- Target profile SHA-256: `6d4cc6c6a4769962798133d57aff01438fb2b661941f71d1aa8a3333f4b7ecc1`
- Quint binding: Rust quint-connect harness
- Reproduction seed or trace id: `T002-cleric-protector`
- Reproduction seed or trace id: `T002-cleric-thaumaturge`
- Reproduction seed or trace id: `T002-druid-magician`
- Reproduction seed or trace id: `T002-druid-warden`

Harness artifacts:

- Start gate: `tasks/START_GATE.json`
- Engine depth: `tasks/ENGINE_DEPTH_MANIFEST.json`
- State ownership: `tasks/STATE_OWNER_MANIFEST.json`
- Reviewer loop: `tasks/REVIEW_LOOP.json`
- Decider decision: `tasks/DECIDER_DECISION.json`

Diagnostic tests:

- `src/tests/mod.rs::order_projection_keeps_base_and_selected_training_distinct`

Remaining gaps:

- `_none_`

Verification results:

- `cargo fmt --check` passed.
- `cargo test` passed.
- `cargo clippy --all-targets -- -D warnings` passed.
- `node scripts/check-cleanroom-harness.cjs` passed.

## T001: character-creation-class-feature-projections

- Manifest source commit SHA: `04249edf345a7752de2f1551dd3d509a2fffc160`
- Source branch inventory SHA: `b4e7e101def7969fc420563dc4da020c22e700f0dc0cc1d27accad6e8631225d`
- Driver: `cleanroom-input/qnt/character-creation-runtime/character-creation-class-feature-projections.mbt.qnt`
- Branch obligations:
  - `step:doProjectMonkFocusAndUncannyMetabolism`
  - `step:doProjectSorcererFontAndMetamagic`
- Allowed inputs used:
  - `cleanroom-input/MANIFEST.md`
  - `cleanroom-input/branch-coverage/source-branch-inventory.json`
  - `cleanroom-input/qnt/character-creation-runtime/character-creation-class-feature-projections.mbt.qnt`
  - `cleanroom-input/raw/srd-5.2.1/Classes/Monk.md`
  - `cleanroom-input/raw/srd-5.2.1/Classes/Sorcerer.md`
  - `cleanroom-input/domain/UBIQUITOUS_LANGUAGE.md`
  - `cleanroom-input/domain/CLEANROOM_ASSUMPTIONS.md`
  - `cleanroom-input/guidance/README.md`

Behavior implemented:

- Added a reusable Rust class-feature projection module for level-2 Monk Focus Points and Uncanny Metabolism facts from `Classes/Monk.md`.
- Added level-2 Sorcerer Font of Magic and Metamagic projection facts, including unique option selection and the Empowered Spell and Heightened Spell costs/effects from `Classes/Sorcerer.md`.
- Kept QNT action dispatch and witness field mapping quarantined in `src/qnt_adapters/character_creation_class_feature_projections.rs`.

Generated branch coverage:

| Obligation | Target replay evidence | Diagnostic tests | Status |
| --- | --- | --- | --- |
| `cleanroom-input/qnt/character-creation-runtime/character-creation-class-feature-projections.mbt.qnt#step:doProjectMonkFocusAndUncannyMetabolism` | `tasks/target-replay-evidence/T001-character-creation-class-feature-projections.json#T001-monk-focus-uncanny-metabolism#step:doProjectMonkFocusAndUncannyMetabolism` | `src/tests/mod.rs::class_feature_projection_adapter_replays_monk_branch` | `covered` |
| `cleanroom-input/qnt/character-creation-runtime/character-creation-class-feature-projections.mbt.qnt#step:doProjectSorcererFontAndMetamagic` | `tasks/target-replay-evidence/T001-character-creation-class-feature-projections.json#T001-sorcerer-font-metamagic#step:doProjectSorcererFontAndMetamagic` | `src/tests/mod.rs::class_feature_projection_adapter_replays_sorcerer_branch` | `covered` |

Target replay evidence:

- Evidence file: `tasks/target-replay-evidence/T001-character-creation-class-feature-projections.json`
- Target profile: `rust`
- Target profile SHA-256: `6d4cc6c6a4769962798133d57aff01438fb2b661941f71d1aa8a3333f4b7ecc1`
- Quint binding: Rust quint-connect harness
- Reproduction seed or trace id: `T001-monk-focus-uncanny-metabolism`
- Reproduction seed or trace id: `T001-sorcerer-font-metamagic`

Harness artifacts:

- Start gate: `tasks/START_GATE.json`
- Engine depth: `tasks/ENGINE_DEPTH_MANIFEST.json`
- State ownership: `tasks/STATE_OWNER_MANIFEST.json`
- Reviewer loop: `tasks/REVIEW_LOOP.json`
- Decider decision: `tasks/DECIDER_DECISION.json`

Diagnostic tests:

- `src/tests/mod.rs::sorcerer_metamagic_projection_rejects_duplicate_options`

Remaining gaps:

- `_none_`

Verification results:

- `cargo fmt --check` passed.
- `cargo test` passed.
- `cargo clippy --all-targets -- -D warnings` passed.
- `node scripts/check-cleanroom-harness.cjs` passed.
