# Validation Report

## Work Loop Status

- Current manifest source commit SHA: `04249edf345a7752de2f1551dd3d509a2fffc160`
- Source branch inventory SHA: `b4e7e101def7969fc420563dc4da020c22e700f0dc0cc1d27accad6e8631225d`
- Scope file: `tasks/LEVEL_1_2_SCOPE.md`
- Work Loop instructions: `tasks/WORK_LOOP.md`
- Last completed current-snapshot queued branch set: `cleanroom-input/qnt/battle-runtime/battle-runtime-sorcerer-metamagic-twinned-selected-identity.mbt.qnt`
- Next queued driver: `cleanroom-input/qnt/battle-runtime/battle-runtime-species-passive-trait-selected-identity.mbt.qnt`
- Next task id: `T057`

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

## T056: battle-runtime-sorcerer-metamagic-twinned-selected-identity

- Manifest source commit SHA: `04249edf345a7752de2f1551dd3d509a2fffc160`
- Source branch inventory SHA: `b4e7e101def7969fc420563dc4da020c22e700f0dc0cc1d27accad6e8631225d`
- Driver: `cleanroom-input/qnt/battle-runtime/battle-runtime-sorcerer-metamagic-twinned-selected-identity.mbt.qnt`
- Branch obligations:
  - `step:doResolveTwinnedTargetCount`
- Allowed inputs used:
  - `cleanroom-input/MANIFEST.md`
  - `cleanroom-input/branch-coverage/source-branch-inventory.json`
  - `cleanroom-input/qnt/battle-runtime/battle-runtime-sorcerer-metamagic-twinned-selected-identity.mbt.qnt`
  - `cleanroom-input/qnt/battle-runtime/battle-runtime-witness-protocol.qnt`
  - `cleanroom-input/qnt/battle-runtime/battle-runtime-metamagic.qnt`
  - `cleanroom-input/raw/srd-5.2.1/Classes/Sorcerer.md`
  - `tasks/LEVEL_1_2_SCOPE.md`

Behavior implemented:

- Extended `src/rules/sorcerer_metamagic.rs` with a Twinned Spell target-count projection: Magic Action is spent, Bonus Action remains available, one Sorcery Point is spent from the fixed witness pool, the target remains at 10 Hit Points, one active effect is recorded, and the protocol resolves.
- Kept exact QNT branch action, scenario label, and protocol strings quarantined in `src/qnt_adapters/battle_runtime_sorcerer_metamagic_twinned_selected_identity.rs`.

Generated branch coverage:

| Obligation | Target replay evidence | Diagnostic tests | Status |
| --- | --- | --- | --- |
| `cleanroom-input/qnt/battle-runtime/battle-runtime-sorcerer-metamagic-twinned-selected-identity.mbt.qnt#step:doResolveTwinnedTargetCount` | `tasks/target-replay-evidence/T056-battle-runtime-sorcerer-metamagic-twinned-selected-identity.json#T056-resolve-twinned-target-count#step:doResolveTwinnedTargetCount` | `src/tests/mod.rs::twinned_spell_adapter_replays_all_branches` | `covered` |

Target replay evidence:

- Evidence file: `tasks/target-replay-evidence/T056-battle-runtime-sorcerer-metamagic-twinned-selected-identity.json`
- Target profile: `rust`
- Target profile SHA-256: `6d4cc6c6a4769962798133d57aff01438fb2b661941f71d1aa8a3333f4b7ecc1`
- Quint binding: Rust quint-connect harness
- Reproduction seed or trace id: `T056-resolve-twinned-target-count`

Harness artifacts:

- Start gate: `tasks/START_GATE.json`
- Engine depth: `tasks/ENGINE_DEPTH_MANIFEST.json`
- State ownership: `tasks/STATE_OWNER_MANIFEST.json`
- Reviewer loop: `tasks/REVIEW_LOOP.json`
- Decider decision: `tasks/DECIDER_DECISION.json`

Diagnostic tests:

- `src/tests/mod.rs::twinned_spell_projects_target_count_case`

Remaining gaps:

- `_none_`

Verification results:

- `cargo fmt --check` passed.
- `cargo test` passed.
- `cargo clippy --all-targets -- -D warnings` passed.
- `node scripts/check-cleanroom-harness.cjs` passed.

## T055: battle-runtime-sorcerer-metamagic-transmuted-selected-identity

- Manifest source commit SHA: `04249edf345a7752de2f1551dd3d509a2fffc160`
- Source branch inventory SHA: `b4e7e101def7969fc420563dc4da020c22e700f0dc0cc1d27accad6e8631225d`
- Driver: `cleanroom-input/qnt/battle-runtime/battle-runtime-sorcerer-metamagic-transmuted-selected-identity.mbt.qnt`
- Branch obligations:
  - `step:doResolveTransmutedSaveGatedDamage`
  - `step:doResolveTransmutedSpellAttack`
- Allowed inputs used:
  - `cleanroom-input/MANIFEST.md`
  - `cleanroom-input/branch-coverage/source-branch-inventory.json`
  - `cleanroom-input/qnt/battle-runtime/battle-runtime-sorcerer-metamagic-transmuted-selected-identity.mbt.qnt`
  - `cleanroom-input/qnt/battle-runtime/battle-runtime-witness-protocol.qnt`
  - `cleanroom-input/qnt/battle-runtime/battle-runtime-metamagic.qnt`
  - `cleanroom-input/raw/srd-5.2.1/Classes/Sorcerer.md`
  - `cleanroom-input/raw/srd-5.2.1/Playing-the-Game.md`
  - `cleanroom-input/domain/UBIQUITOUS_LANGUAGE.md`
  - `tasks/LEVEL_1_2_SCOPE.md`

Behavior implemented:

- Extended `src/rules/sorcerer_metamagic.rs` with Transmuted Spell projections for save-gated damage and spell-attack damage: Magic Action is spent, Bonus Action remains available, one Sorcery Point is spent from the fixed witness pool, and each branch resolves with the QNT target Hit Points and active-effect count.
- Kept exact QNT branch actions, scenario labels, and protocol strings quarantined in `src/qnt_adapters/battle_runtime_sorcerer_metamagic_transmuted_selected_identity.rs`.

Generated branch coverage:

| Obligation | Target replay evidence | Diagnostic tests | Status |
| --- | --- | --- | --- |
| `cleanroom-input/qnt/battle-runtime/battle-runtime-sorcerer-metamagic-transmuted-selected-identity.mbt.qnt#step:doResolveTransmutedSaveGatedDamage` | `tasks/target-replay-evidence/T055-battle-runtime-sorcerer-metamagic-transmuted-selected-identity.json#T055-resolve-transmuted-save-gated-damage#step:doResolveTransmutedSaveGatedDamage` | `src/tests/mod.rs::transmuted_spell_adapter_replays_all_branches` | `covered` |
| `cleanroom-input/qnt/battle-runtime/battle-runtime-sorcerer-metamagic-transmuted-selected-identity.mbt.qnt#step:doResolveTransmutedSpellAttack` | `tasks/target-replay-evidence/T055-battle-runtime-sorcerer-metamagic-transmuted-selected-identity.json#T055-resolve-transmuted-spell-attack#step:doResolveTransmutedSpellAttack` | `src/tests/mod.rs::transmuted_spell_adapter_replays_all_branches` | `covered` |

Target replay evidence:

- Evidence file: `tasks/target-replay-evidence/T055-battle-runtime-sorcerer-metamagic-transmuted-selected-identity.json`
- Target profile: `rust`
- Target profile SHA-256: `6d4cc6c6a4769962798133d57aff01438fb2b661941f71d1aa8a3333f4b7ecc1`
- Quint binding: Rust quint-connect harness
- Reproduction seed or trace id: `T055-resolve-transmuted-save-gated-damage`
- Reproduction seed or trace id: `T055-resolve-transmuted-spell-attack`

Harness artifacts:

- Start gate: `tasks/START_GATE.json`
- Engine depth: `tasks/ENGINE_DEPTH_MANIFEST.json`
- State ownership: `tasks/STATE_OWNER_MANIFEST.json`
- Reviewer loop: `tasks/REVIEW_LOOP.json`
- Decider decision: `tasks/DECIDER_DECISION.json`

Diagnostic tests:

- `src/tests/mod.rs::transmuted_spell_projects_save_gated_and_spell_attack_cases`

Remaining gaps:

- `_none_`

Verification results:

- `cargo fmt --check` passed.
- `cargo test` passed.
- `cargo clippy --all-targets -- -D warnings` passed.
- `node scripts/check-cleanroom-harness.cjs` passed.

## T054: battle-runtime-sorcerer-metamagic-subtle-selected-identity

- Manifest source commit SHA: `04249edf345a7752de2f1551dd3d509a2fffc160`
- Source branch inventory SHA: `b4e7e101def7969fc420563dc4da020c22e700f0dc0cc1d27accad6e8631225d`
- Driver: `cleanroom-input/qnt/battle-runtime/battle-runtime-sorcerer-metamagic-subtle-selected-identity.mbt.qnt`
- Branch obligations:
  - `step:doRejectSubtleFalseLifeWithoutSorceryPoints`
  - `step:doResolveSubtleFalseLife`
- Allowed inputs used:
  - `cleanroom-input/MANIFEST.md`
  - `cleanroom-input/branch-coverage/source-branch-inventory.json`
  - `cleanroom-input/qnt/battle-runtime/battle-runtime-sorcerer-metamagic-subtle-selected-identity.mbt.qnt`
  - `cleanroom-input/qnt/battle-runtime/battle-runtime-witness-protocol.qnt`
  - `cleanroom-input/raw/srd-5.2.1/Classes/Sorcerer.md`
  - `cleanroom-input/raw/srd-5.2.1/Spells/Descriptions-E-L.md`
  - `cleanroom-input/raw/srd-5.2.1/Playing-the-Game.md`
  - `cleanroom-input/domain/UBIQUITOUS_LANGUAGE.md`
  - `tasks/LEVEL_1_2_SCOPE.md`

Behavior implemented:

- Extended `src/rules/sorcerer_metamagic.rs` with Subtle Spell projections for False Life: successful casting suppresses Verbal, Somatic, and focus-replaceable Material components, spends 1 Sorcery Point, grants 11 Temporary Hit Points, and resolves.
- Added the unaffordable branch that preserves component requirements, leaves Temporary Hit Points at 0, records 0 Sorcery Points, and projects `WUnsupportedActOption` through the invalid protocol.
- Kept exact QNT branch actions, scenario labels, protocol labels, and invalid reason labels quarantined in `src/qnt_adapters/battle_runtime_sorcerer_metamagic_subtle_selected_identity.rs`.

Generated branch coverage:

| Obligation | Target replay evidence | Diagnostic tests | Status |
| --- | --- | --- | --- |
| `cleanroom-input/qnt/battle-runtime/battle-runtime-sorcerer-metamagic-subtle-selected-identity.mbt.qnt#step:doRejectSubtleFalseLifeWithoutSorceryPoints` | `tasks/target-replay-evidence/T054-battle-runtime-sorcerer-metamagic-subtle-selected-identity.json#T054-reject-subtle-false-life-without-sorcery-points#step:doRejectSubtleFalseLifeWithoutSorceryPoints` | `src/tests/mod.rs::subtle_spell_adapter_replays_all_branches` | `covered` |
| `cleanroom-input/qnt/battle-runtime/battle-runtime-sorcerer-metamagic-subtle-selected-identity.mbt.qnt#step:doResolveSubtleFalseLife` | `tasks/target-replay-evidence/T054-battle-runtime-sorcerer-metamagic-subtle-selected-identity.json#T054-resolve-subtle-false-life#step:doResolveSubtleFalseLife` | `src/tests/mod.rs::subtle_spell_adapter_replays_all_branches` | `covered` |

Target replay evidence:

- Evidence file: `tasks/target-replay-evidence/T054-battle-runtime-sorcerer-metamagic-subtle-selected-identity.json`
- Target profile: `rust`
- Target profile SHA-256: `6d4cc6c6a4769962798133d57aff01438fb2b661941f71d1aa8a3333f4b7ecc1`
- Quint binding: Rust quint-connect harness
- Reproduction seed or trace id: `T054-reject-subtle-false-life-without-sorcery-points`
- Reproduction seed or trace id: `T054-resolve-subtle-false-life`

Harness artifacts:

- Start gate: `tasks/START_GATE.json`
- Engine depth: `tasks/ENGINE_DEPTH_MANIFEST.json`
- State ownership: `tasks/STATE_OWNER_MANIFEST.json`
- Reviewer loop: `tasks/REVIEW_LOOP.json`
- Decider decision: `tasks/DECIDER_DECISION.json`

Diagnostic tests:

- `src/tests/mod.rs::subtle_spell_projects_false_life_and_unaffordable_cases`

Remaining gaps:

- `_none_`

Verification results:

- `cargo fmt --check` passed.
- `cargo test` passed.
- `cargo clippy --all-targets -- -D warnings` passed.
- `node scripts/check-cleanroom-harness.cjs` passed.

## T053: battle-runtime-sorcerer-metamagic-spell-attack-sequence-selected-identity

- Manifest source commit SHA: `04249edf345a7752de2f1551dd3d509a2fffc160`
- Source branch inventory SHA: `b4e7e101def7969fc420563dc4da020c22e700f0dc0cc1d27accad6e8631225d`
- Driver: `cleanroom-input/qnt/battle-runtime/battle-runtime-sorcerer-metamagic-spell-attack-sequence-selected-identity.mbt.qnt`
- Branch obligations:
  - `step:doResolveQuickenedSpellAttackSequence`
- Allowed inputs used:
  - `cleanroom-input/MANIFEST.md`
  - `cleanroom-input/branch-coverage/source-branch-inventory.json`
  - `cleanroom-input/qnt/battle-runtime/battle-runtime-sorcerer-metamagic-spell-attack-sequence-selected-identity.mbt.qnt`
  - `cleanroom-input/qnt/battle-runtime/battle-runtime-witness-protocol.qnt`
  - `cleanroom-input/raw/srd-5.2.1/Classes/Sorcerer.md`
  - `tasks/LEVEL_1_2_SCOPE.md`

Behavior implemented:

- Extended `src/rules/sorcerer_metamagic.rs` with a Quickened Spell spell-attack-sequence projection: Magic Action remains available, Bonus Action is spent, 2 Sorcery Points are spent from the fixed witness pool, target Hit Points are reduced to 6, target active-effect count remains 0, and the protocol resolves.
- Kept the exact QNT branch action, scenario label, and protocol strings quarantined in `src/qnt_adapters/battle_runtime_sorcerer_metamagic_spell_attack_sequence_selected_identity.rs`.

Generated branch coverage:

| Obligation | Target replay evidence | Diagnostic tests | Status |
| --- | --- | --- | --- |
| `cleanroom-input/qnt/battle-runtime/battle-runtime-sorcerer-metamagic-spell-attack-sequence-selected-identity.mbt.qnt#step:doResolveQuickenedSpellAttackSequence` | `tasks/target-replay-evidence/T053-battle-runtime-sorcerer-metamagic-spell-attack-sequence-selected-identity.json#T053-resolve-quickened-spell-attack-sequence#step:doResolveQuickenedSpellAttackSequence` | `src/tests/mod.rs::quickened_spell_attack_sequence_adapter_replays_all_branches` | `covered` |

Target replay evidence:

- Evidence file: `tasks/target-replay-evidence/T053-battle-runtime-sorcerer-metamagic-spell-attack-sequence-selected-identity.json`
- Target profile: `rust`
- Target profile SHA-256: `6d4cc6c6a4769962798133d57aff01438fb2b661941f71d1aa8a3333f4b7ecc1`
- Quint binding: Rust quint-connect harness
- Reproduction seed or trace id: `T053-resolve-quickened-spell-attack-sequence`

Harness artifacts:

- Start gate: `tasks/START_GATE.json`
- Engine depth: `tasks/ENGINE_DEPTH_MANIFEST.json`
- State ownership: `tasks/STATE_OWNER_MANIFEST.json`
- Reviewer loop: `tasks/REVIEW_LOOP.json`
- Decider decision: `tasks/DECIDER_DECISION.json`

Diagnostic tests:

- `src/tests/mod.rs::quickened_spell_attack_sequence_projects_sequence_case`

Remaining gaps:

- `_none_`

Verification results:

- `cargo fmt --check` passed.
- `cargo test` passed.
- `cargo clippy --all-targets -- -D warnings` passed.
- `node scripts/check-cleanroom-harness.cjs` passed.

## T052: battle-runtime-sorcerer-metamagic-spell-attack-selected-identity

- Manifest source commit SHA: `04249edf345a7752de2f1551dd3d509a2fffc160`
- Source branch inventory SHA: `b4e7e101def7969fc420563dc4da020c22e700f0dc0cc1d27accad6e8631225d`
- Driver: `cleanroom-input/qnt/battle-runtime/battle-runtime-sorcerer-metamagic-spell-attack-selected-identity.mbt.qnt`
- Branch obligations:
  - `step:doResolveQuickenedSpellAttack`
- Allowed inputs used:
  - `cleanroom-input/MANIFEST.md`
  - `cleanroom-input/branch-coverage/source-branch-inventory.json`
  - `cleanroom-input/qnt/battle-runtime/battle-runtime-sorcerer-metamagic-spell-attack-selected-identity.mbt.qnt`
  - `cleanroom-input/qnt/battle-runtime/battle-runtime-witness-protocol.qnt`
  - `cleanroom-input/raw/srd-5.2.1/Classes/Sorcerer.md`
  - `tasks/LEVEL_1_2_SCOPE.md`

Behavior implemented:

- Extended `src/rules/sorcerer_metamagic.rs` with a Quickened Spell spell-attack projection: Magic Action remains available, Bonus Action is spent, 2 Sorcery Points are spent from the fixed witness pool, target Hit Points are reduced to 3, one target active effect is retained, and the protocol resolves.
- Kept the exact QNT branch action, scenario label, and protocol strings quarantined in `src/qnt_adapters/battle_runtime_sorcerer_metamagic_spell_attack_selected_identity.rs`.

Generated branch coverage:

| Obligation | Target replay evidence | Diagnostic tests | Status |
| --- | --- | --- | --- |
| `cleanroom-input/qnt/battle-runtime/battle-runtime-sorcerer-metamagic-spell-attack-selected-identity.mbt.qnt#step:doResolveQuickenedSpellAttack` | `tasks/target-replay-evidence/T052-battle-runtime-sorcerer-metamagic-spell-attack-selected-identity.json#T052-resolve-quickened-spell-attack#step:doResolveQuickenedSpellAttack` | `src/tests/mod.rs::quickened_spell_attack_adapter_replays_all_branches` | `covered` |

Target replay evidence:

- Evidence file: `tasks/target-replay-evidence/T052-battle-runtime-sorcerer-metamagic-spell-attack-selected-identity.json`
- Target profile: `rust`
- Target profile SHA-256: `6d4cc6c6a4769962798133d57aff01438fb2b661941f71d1aa8a3333f4b7ecc1`
- Quint binding: Rust quint-connect harness
- Reproduction seed or trace id: `T052-resolve-quickened-spell-attack`

Harness artifacts:

- Start gate: `tasks/START_GATE.json`
- Engine depth: `tasks/ENGINE_DEPTH_MANIFEST.json`
- State ownership: `tasks/STATE_OWNER_MANIFEST.json`
- Reviewer loop: `tasks/REVIEW_LOOP.json`
- Decider decision: `tasks/DECIDER_DECISION.json`

Diagnostic tests:

- `src/tests/mod.rs::quickened_spell_attack_projects_spell_attack_case`

Remaining gaps:

- `_none_`

Verification results:

- `cargo fmt --check` passed.
- `cargo test` passed.
- `cargo clippy --all-targets -- -D warnings` passed.
- `node scripts/check-cleanroom-harness.cjs` passed.

## T051: battle-runtime-sorcerer-metamagic-selected-identity

- Manifest source commit SHA: `04249edf345a7752de2f1551dd3d509a2fffc160`
- Source branch inventory SHA: `b4e7e101def7969fc420563dc4da020c22e700f0dc0cc1d27accad6e8631225d`
- Driver: `cleanroom-input/qnt/battle-runtime/battle-runtime-sorcerer-metamagic-selected-identity.mbt.qnt`
- Branch obligations:
  - `step:doResolveQuickenedSaveGatedDamage`
- Allowed inputs used:
  - `cleanroom-input/MANIFEST.md`
  - `cleanroom-input/branch-coverage/source-branch-inventory.json`
  - `cleanroom-input/qnt/battle-runtime/battle-runtime-sorcerer-metamagic-selected-identity.mbt.qnt`
  - `cleanroom-input/qnt/battle-runtime/battle-runtime-witness-protocol.qnt`
  - `cleanroom-input/raw/srd-5.2.1/Classes/Sorcerer.md`
  - `tasks/LEVEL_1_2_SCOPE.md`

Behavior implemented:

- Extended `src/rules/sorcerer_metamagic.rs` with a Quickened Spell save-gated damage projection: Magic Action remains available, Bonus Action is spent, 2 Sorcery Points are spent from the fixed witness pool, target Hit Points are reduced to 1, target active-effect count remains 0, and the protocol resolves.
- Kept the exact QNT branch action, scenario label, and protocol strings quarantined in `src/qnt_adapters/battle_runtime_sorcerer_metamagic_selected_identity.rs`.

Generated branch coverage:

| Obligation | Target replay evidence | Diagnostic tests | Status |
| --- | --- | --- | --- |
| `cleanroom-input/qnt/battle-runtime/battle-runtime-sorcerer-metamagic-selected-identity.mbt.qnt#step:doResolveQuickenedSaveGatedDamage` | `tasks/target-replay-evidence/T051-battle-runtime-sorcerer-metamagic-selected-identity.json#T051-resolve-quickened-save-gated-damage#step:doResolveQuickenedSaveGatedDamage` | `src/tests/mod.rs::quickened_metamagic_adapter_replays_all_branches` | `covered` |

Target replay evidence:

- Evidence file: `tasks/target-replay-evidence/T051-battle-runtime-sorcerer-metamagic-selected-identity.json`
- Target profile: `rust`
- Target profile SHA-256: `6d4cc6c6a4769962798133d57aff01438fb2b661941f71d1aa8a3333f4b7ecc1`
- Quint binding: Rust quint-connect harness
- Reproduction seed or trace id: `T051-resolve-quickened-save-gated-damage`

Harness artifacts:

- Start gate: `tasks/START_GATE.json`
- Engine depth: `tasks/ENGINE_DEPTH_MANIFEST.json`
- State ownership: `tasks/STATE_OWNER_MANIFEST.json`
- Reviewer loop: `tasks/REVIEW_LOOP.json`
- Decider decision: `tasks/DECIDER_DECISION.json`

Diagnostic tests:

- `src/tests/mod.rs::quickened_metamagic_projects_save_gated_damage_case`

Remaining gaps:

- `_none_`

Verification results:

- `cargo fmt --check` passed.
- `cargo test` passed.
- `cargo clippy --all-targets -- -D warnings` passed.
- `node scripts/check-cleanroom-harness.cjs` passed.

## T050: battle-runtime-sorcerer-metamagic-seeking-selected-identity

- Manifest source commit SHA: `04249edf345a7752de2f1551dd3d509a2fffc160`
- Source branch inventory SHA: `b4e7e101def7969fc420563dc4da020c22e700f0dc0cc1d27accad6e8631225d`
- Driver: `cleanroom-input/qnt/battle-runtime/battle-runtime-sorcerer-metamagic-seeking-selected-identity.mbt.qnt`
- Branch obligations:
  - `step:doResolveSeekingSpellAttackReroll`
- Allowed inputs used:
  - `cleanroom-input/MANIFEST.md`
  - `cleanroom-input/branch-coverage/source-branch-inventory.json`
  - `cleanroom-input/qnt/battle-runtime/battle-runtime-sorcerer-metamagic-seeking-selected-identity.mbt.qnt`
  - `cleanroom-input/qnt/battle-runtime/battle-runtime-witness-protocol.qnt`
  - `cleanroom-input/raw/srd-5.2.1/Classes/Sorcerer.md`
  - `tasks/LEVEL_1_2_SCOPE.md`

Behavior implemented:

- Extended `src/rules/sorcerer_metamagic.rs` with a Seeking Spell attack-reroll projection: Magic Action spent, Bonus Action still available, one Sorcery Point spent from the fixed witness pool, target Hit Points reduced to 3, one target active effect retained, and resolved protocol.
- Kept the exact QNT action name, scenario label, and protocol strings quarantined in `src/qnt_adapters/battle_runtime_sorcerer_metamagic_seeking_selected_identity.rs`.

Generated branch coverage:

| Obligation | Target replay evidence | Diagnostic tests | Status |
| --- | --- | --- | --- |
| `cleanroom-input/qnt/battle-runtime/battle-runtime-sorcerer-metamagic-seeking-selected-identity.mbt.qnt#step:doResolveSeekingSpellAttackReroll` | `tasks/target-replay-evidence/T050-battle-runtime-sorcerer-metamagic-seeking-selected-identity.json#T050-resolve-seeking-spell-attack-reroll#step:doResolveSeekingSpellAttackReroll` | `src/tests/mod.rs::seeking_spell_adapter_replays_all_branches` | `covered` |

Target replay evidence:

- Evidence file: `tasks/target-replay-evidence/T050-battle-runtime-sorcerer-metamagic-seeking-selected-identity.json`
- Target profile: `rust`
- Target profile SHA-256: `6d4cc6c6a4769962798133d57aff01438fb2b661941f71d1aa8a3333f4b7ecc1`
- Quint binding: Rust quint-connect harness
- Reproduction seed or trace id: `T050-resolve-seeking-spell-attack-reroll`

Harness artifacts:

- Start gate: `tasks/START_GATE.json`
- Engine depth: `tasks/ENGINE_DEPTH_MANIFEST.json`
- State ownership: `tasks/STATE_OWNER_MANIFEST.json`
- Reviewer loop: `tasks/REVIEW_LOOP.json`
- Decider decision: `tasks/DECIDER_DECISION.json`

Diagnostic tests:

- `src/tests/mod.rs::seeking_spell_projects_attack_reroll_case`

Remaining gaps:

- `_none_`

Verification results:

- `cargo fmt --check` passed.
- `cargo test` passed.
- `cargo clippy --all-targets -- -D warnings` passed.
- `node scripts/check-cleanroom-harness.cjs` passed.

## T049: battle-runtime-sorcerer-metamagic-heightened-selected-identity

- Manifest source commit SHA: `04249edf345a7752de2f1551dd3d509a2fffc160`
- Source branch inventory SHA: `b4e7e101def7969fc420563dc4da020c22e700f0dc0cc1d27accad6e8631225d`
- Driver: `cleanroom-input/qnt/battle-runtime/battle-runtime-sorcerer-metamagic-heightened-selected-identity.mbt.qnt`
- Branch obligations:
  - `step:doResolveHeightenedGreaseEntrySave`
  - `step:doResolveHeightenedGustOfWindEndTurnSave`
  - `step:doResolveHeightenedHideousLaughter`
  - `step:doResolveHeightenedSaveGatedConditionEndTurnSave`
  - `step:doResolveHeightenedSaveGatedDamage`
- Allowed inputs used:
  - `cleanroom-input/MANIFEST.md`
  - `cleanroom-input/branch-coverage/source-branch-inventory.json`
  - `cleanroom-input/qnt/battle-runtime/battle-runtime-sorcerer-metamagic-heightened-selected-identity.mbt.qnt`
  - `cleanroom-input/qnt/battle-runtime/battle-runtime-witness-protocol.qnt`
  - `cleanroom-input/raw/srd-5.2.1/Classes/Sorcerer.md`
  - `tasks/LEVEL_1_2_SCOPE.md`

Behavior implemented:

- Extended `src/rules/sorcerer_metamagic.rs` with Heightened Spell projections for save-gated damage, Hideous Laughter, Grease entry saves, Gust of Wind end-turn saves, and save-gated condition end-turn saves.
- Projected the 2 Sorcery Point cost, Magic Action availability differences between cast-time and later-save cases, target Hit Points, target active-effect count, scenario result, and resolved protocol.
- Kept exact QNT branch actions, scenario labels, and protocol strings quarantined in `src/qnt_adapters/battle_runtime_sorcerer_metamagic_heightened_selected_identity.rs`.

Generated branch coverage:

| Obligation | Target replay evidence | Diagnostic tests | Status |
| --- | --- | --- | --- |
| `cleanroom-input/qnt/battle-runtime/battle-runtime-sorcerer-metamagic-heightened-selected-identity.mbt.qnt#step:doResolveHeightenedGreaseEntrySave` | `tasks/target-replay-evidence/T049-battle-runtime-sorcerer-metamagic-heightened-selected-identity.json#T049-resolve-heightened-grease-entry-save#step:doResolveHeightenedGreaseEntrySave` | `src/tests/mod.rs::heightened_spell_adapter_replays_all_branches` | `covered` |
| `cleanroom-input/qnt/battle-runtime/battle-runtime-sorcerer-metamagic-heightened-selected-identity.mbt.qnt#step:doResolveHeightenedGustOfWindEndTurnSave` | `tasks/target-replay-evidence/T049-battle-runtime-sorcerer-metamagic-heightened-selected-identity.json#T049-resolve-heightened-gust-of-wind-end-turn-save#step:doResolveHeightenedGustOfWindEndTurnSave` | `src/tests/mod.rs::heightened_spell_adapter_replays_all_branches` | `covered` |
| `cleanroom-input/qnt/battle-runtime/battle-runtime-sorcerer-metamagic-heightened-selected-identity.mbt.qnt#step:doResolveHeightenedHideousLaughter` | `tasks/target-replay-evidence/T049-battle-runtime-sorcerer-metamagic-heightened-selected-identity.json#T049-resolve-heightened-hideous-laughter#step:doResolveHeightenedHideousLaughter` | `src/tests/mod.rs::heightened_spell_adapter_replays_all_branches` | `covered` |
| `cleanroom-input/qnt/battle-runtime/battle-runtime-sorcerer-metamagic-heightened-selected-identity.mbt.qnt#step:doResolveHeightenedSaveGatedConditionEndTurnSave` | `tasks/target-replay-evidence/T049-battle-runtime-sorcerer-metamagic-heightened-selected-identity.json#T049-resolve-heightened-save-gated-condition-end-turn-save#step:doResolveHeightenedSaveGatedConditionEndTurnSave` | `src/tests/mod.rs::heightened_spell_adapter_replays_all_branches` | `covered` |
| `cleanroom-input/qnt/battle-runtime/battle-runtime-sorcerer-metamagic-heightened-selected-identity.mbt.qnt#step:doResolveHeightenedSaveGatedDamage` | `tasks/target-replay-evidence/T049-battle-runtime-sorcerer-metamagic-heightened-selected-identity.json#T049-resolve-heightened-save-gated-damage#step:doResolveHeightenedSaveGatedDamage` | `src/tests/mod.rs::heightened_spell_adapter_replays_all_branches` | `covered` |

Target replay evidence:

- Evidence file: `tasks/target-replay-evidence/T049-battle-runtime-sorcerer-metamagic-heightened-selected-identity.json`
- Target profile: `rust`
- Target profile SHA-256: `6d4cc6c6a4769962798133d57aff01438fb2b661941f71d1aa8a3333f4b7ecc1`
- Quint binding: Rust quint-connect harness
- Reproduction seed or trace id: `T049-resolve-heightened-grease-entry-save`
- Reproduction seed or trace id: `T049-resolve-heightened-gust-of-wind-end-turn-save`
- Reproduction seed or trace id: `T049-resolve-heightened-hideous-laughter`
- Reproduction seed or trace id: `T049-resolve-heightened-save-gated-condition-end-turn-save`
- Reproduction seed or trace id: `T049-resolve-heightened-save-gated-damage`

Harness artifacts:

- Start gate: `tasks/START_GATE.json`
- Engine depth: `tasks/ENGINE_DEPTH_MANIFEST.json`
- State ownership: `tasks/STATE_OWNER_MANIFEST.json`
- Reviewer loop: `tasks/REVIEW_LOOP.json`
- Decider decision: `tasks/DECIDER_DECISION.json`

Diagnostic tests:

- `src/tests/mod.rs::heightened_spell_projects_damage_condition_and_later_save_cases`

Remaining gaps:

- `_none_`

Verification results:

- `cargo fmt --check` passed.
- `cargo test` passed.
- `cargo clippy --all-targets -- -D warnings` passed.
- `node scripts/check-cleanroom-harness.cjs` passed.

## T048: battle-runtime-sorcerer-metamagic-extended-selected-identity

- Manifest source commit SHA: `04249edf345a7752de2f1551dd3d509a2fffc160`
- Source branch inventory SHA: `b4e7e101def7969fc420563dc4da020c22e700f0dc0cc1d27accad6e8631225d`
- Driver: `cleanroom-input/qnt/battle-runtime/battle-runtime-sorcerer-metamagic-extended-selected-identity.mbt.qnt`
- Branch obligations:
  - `step:doResolveExtendedCreatureSizeIncrease`
- Allowed inputs used:
  - `cleanroom-input/MANIFEST.md`
  - `cleanroom-input/branch-coverage/source-branch-inventory.json`
  - `cleanroom-input/qnt/battle-runtime/battle-runtime-sorcerer-metamagic-extended-selected-identity.mbt.qnt`
  - `cleanroom-input/qnt/battle-runtime/battle-runtime-witness-protocol.qnt`
  - `cleanroom-input/raw/srd-5.2.1/Classes/Sorcerer.md`
  - `tasks/LEVEL_1_2_SCOPE.md`

Behavior implemented:

- Extended `src/rules/sorcerer_metamagic.rs` with an Extended Spell creature-size projection: one Sorcery Point spent, duration doubled to 20 witness ticks, Concentration-maintenance saving throws set to Advantage, and resolved protocol.
- Kept the exact QNT action name, scenario label, Concentration mode labels, and protocol strings quarantined in `src/qnt_adapters/battle_runtime_sorcerer_metamagic_extended_selected_identity.rs`.

Generated branch coverage:

| Obligation | Target replay evidence | Diagnostic tests | Status |
| --- | --- | --- | --- |
| `cleanroom-input/qnt/battle-runtime/battle-runtime-sorcerer-metamagic-extended-selected-identity.mbt.qnt#step:doResolveExtendedCreatureSizeIncrease` | `tasks/target-replay-evidence/T048-battle-runtime-sorcerer-metamagic-extended-selected-identity.json#T048-resolve-extended-creature-size-increase#step:doResolveExtendedCreatureSizeIncrease` | `src/tests/mod.rs::extended_spell_adapter_replays_all_branches` | `covered` |

Target replay evidence:

- Evidence file: `tasks/target-replay-evidence/T048-battle-runtime-sorcerer-metamagic-extended-selected-identity.json`
- Target profile: `rust`
- Target profile SHA-256: `6d4cc6c6a4769962798133d57aff01438fb2b661941f71d1aa8a3333f4b7ecc1`
- Quint binding: Rust quint-connect harness
- Reproduction seed or trace id: `T048-resolve-extended-creature-size-increase`

Harness artifacts:

- Start gate: `tasks/START_GATE.json`
- Engine depth: `tasks/ENGINE_DEPTH_MANIFEST.json`
- State ownership: `tasks/STATE_OWNER_MANIFEST.json`
- Reviewer loop: `tasks/REVIEW_LOOP.json`
- Decider decision: `tasks/DECIDER_DECISION.json`

Diagnostic tests:

- `src/tests/mod.rs::extended_spell_projects_creature_size_duration_and_concentration_mode`

Remaining gaps:

- `_none_`

Verification results:

- `cargo fmt --check` passed.
- `cargo test` passed.
- `cargo clippy --all-targets -- -D warnings` passed.
- `node scripts/check-cleanroom-harness.cjs` passed.

## T047: battle-runtime-sorcerer-metamagic-empowered-selected-identity

- Manifest source commit SHA: `04249edf345a7752de2f1551dd3d509a2fffc160`
- Source branch inventory SHA: `b4e7e101def7969fc420563dc4da020c22e700f0dc0cc1d27accad6e8631225d`
- Driver: `cleanroom-input/qnt/battle-runtime/battle-runtime-sorcerer-metamagic-empowered-selected-identity.mbt.qnt`
- Branch obligations:
  - `step:doResolveEmpoweredSpellDamageReroll`
- Allowed inputs used:
  - `cleanroom-input/MANIFEST.md`
  - `cleanroom-input/branch-coverage/source-branch-inventory.json`
  - `cleanroom-input/qnt/battle-runtime/battle-runtime-sorcerer-metamagic-empowered-selected-identity.mbt.qnt`
  - `cleanroom-input/qnt/battle-runtime/battle-runtime-witness-protocol.qnt`
  - `cleanroom-input/raw/srd-5.2.1/Classes/Sorcerer.md`
  - `tasks/LEVEL_1_2_SCOPE.md`

Behavior implemented:

- Extended `src/rules/sorcerer_metamagic.rs` with an Empowered Spell damage-reroll projection: Magic Action spent, Bonus Action still available, one Sorcery Point spent from the fixed witness pool, target Hit Points reduced to 1, one target active effect retained, and resolved protocol.
- Kept the exact QNT action name, scenario label, and protocol strings quarantined in `src/qnt_adapters/battle_runtime_sorcerer_metamagic_empowered_selected_identity.rs`.

Generated branch coverage:

| Obligation | Target replay evidence | Diagnostic tests | Status |
| --- | --- | --- | --- |
| `cleanroom-input/qnt/battle-runtime/battle-runtime-sorcerer-metamagic-empowered-selected-identity.mbt.qnt#step:doResolveEmpoweredSpellDamageReroll` | `tasks/target-replay-evidence/T047-battle-runtime-sorcerer-metamagic-empowered-selected-identity.json#T047-resolve-empowered-spell-damage-reroll#step:doResolveEmpoweredSpellDamageReroll` | `src/tests/mod.rs::empowered_spell_adapter_replays_all_branches` | `covered` |

Target replay evidence:

- Evidence file: `tasks/target-replay-evidence/T047-battle-runtime-sorcerer-metamagic-empowered-selected-identity.json`
- Target profile: `rust`
- Target profile SHA-256: `6d4cc6c6a4769962798133d57aff01438fb2b661941f71d1aa8a3333f4b7ecc1`
- Quint binding: Rust quint-connect harness
- Reproduction seed or trace id: `T047-resolve-empowered-spell-damage-reroll`

Harness artifacts:

- Start gate: `tasks/START_GATE.json`
- Engine depth: `tasks/ENGINE_DEPTH_MANIFEST.json`
- State ownership: `tasks/STATE_OWNER_MANIFEST.json`
- Reviewer loop: `tasks/REVIEW_LOOP.json`
- Decider decision: `tasks/DECIDER_DECISION.json`

Diagnostic tests:

- `src/tests/mod.rs::empowered_spell_projects_damage_reroll_case`

Remaining gaps:

- `_none_`

Verification results:

- `cargo fmt --check` passed.
- `cargo test` passed.
- `cargo clippy --all-targets -- -D warnings` passed.
- `node scripts/check-cleanroom-harness.cjs` passed.

## T046: battle-runtime-sorcerer-metamagic-distant-selected-identity

- Manifest source commit SHA: `04249edf345a7752de2f1551dd3d509a2fffc160`
- Source branch inventory SHA: `b4e7e101def7969fc420563dc4da020c22e700f0dc0cc1d27accad6e8631225d`
- Driver: `cleanroom-input/qnt/battle-runtime/battle-runtime-sorcerer-metamagic-distant-selected-identity.mbt.qnt`
- Branch obligations:
  - `step:doResolveDistantObjectLight`
- Allowed inputs used:
  - `cleanroom-input/MANIFEST.md`
  - `cleanroom-input/branch-coverage/source-branch-inventory.json`
  - `cleanroom-input/qnt/battle-runtime/battle-runtime-sorcerer-metamagic-distant-selected-identity.mbt.qnt`
  - `cleanroom-input/qnt/battle-runtime/battle-runtime-witness-protocol.qnt`
  - `cleanroom-input/raw/srd-5.2.1/Classes/Sorcerer.md`
  - `tasks/LEVEL_1_2_SCOPE.md`

Behavior implemented:

- Extended `src/rules/sorcerer_metamagic.rs` with a Distant Spell object-light projection: one Sorcery Point spent from the fixed witness pool, one Light emitter created, 20-foot Bright Light radius, 20 additional feet of Dim Light, and resolved protocol.
- Kept the exact QNT action name, scenario label, and protocol strings quarantined in `src/qnt_adapters/battle_runtime_sorcerer_metamagic_distant_selected_identity.rs`.

Generated branch coverage:

| Obligation | Target replay evidence | Diagnostic tests | Status |
| --- | --- | --- | --- |
| `cleanroom-input/qnt/battle-runtime/battle-runtime-sorcerer-metamagic-distant-selected-identity.mbt.qnt#step:doResolveDistantObjectLight` | `tasks/target-replay-evidence/T046-battle-runtime-sorcerer-metamagic-distant-selected-identity.json#T046-resolve-distant-object-light#step:doResolveDistantObjectLight` | `src/tests/mod.rs::distant_spell_adapter_replays_all_branches` | `covered` |

Target replay evidence:

- Evidence file: `tasks/target-replay-evidence/T046-battle-runtime-sorcerer-metamagic-distant-selected-identity.json`
- Target profile: `rust`
- Target profile SHA-256: `6d4cc6c6a4769962798133d57aff01438fb2b661941f71d1aa8a3333f4b7ecc1`
- Quint binding: Rust quint-connect harness
- Reproduction seed or trace id: `T046-resolve-distant-object-light`

Harness artifacts:

- Start gate: `tasks/START_GATE.json`
- Engine depth: `tasks/ENGINE_DEPTH_MANIFEST.json`
- State ownership: `tasks/STATE_OWNER_MANIFEST.json`
- Reviewer loop: `tasks/REVIEW_LOOP.json`
- Decider decision: `tasks/DECIDER_DECISION.json`

Diagnostic tests:

- `src/tests/mod.rs::distant_spell_projects_object_light_range_case`

Remaining gaps:

- `_none_`

Verification results:

- `cargo fmt --check` passed.
- `cargo test` passed.
- `cargo clippy --all-targets -- -D warnings` passed.
- `node scripts/check-cleanroom-harness.cjs` passed.

## T045: battle-runtime-sorcerer-metamagic-careful-selected-identity

- Manifest source commit SHA: `04249edf345a7752de2f1551dd3d509a2fffc160`
- Source branch inventory SHA: `b4e7e101def7969fc420563dc4da020c22e700f0dc0cc1d27accad6e8631225d`
- Driver: `cleanroom-input/qnt/battle-runtime/battle-runtime-sorcerer-metamagic-careful-selected-identity.mbt.qnt`
- Branch obligations:
  - `step:doResolveCarefulSaveGatedDamage`
  - `step:doResolveCarefulSaveGatedNoEffect`
- Allowed inputs used:
  - `cleanroom-input/MANIFEST.md`
  - `cleanroom-input/branch-coverage/source-branch-inventory.json`
  - `cleanroom-input/qnt/battle-runtime/battle-runtime-sorcerer-metamagic-careful-selected-identity.mbt.qnt`
  - `cleanroom-input/qnt/battle-runtime/battle-runtime-witness-protocol.qnt`
  - `cleanroom-input/raw/srd-5.2.1/Classes/Sorcerer.md`
  - `cleanroom-input/domain/UBIQUITOUS_LANGUAGE.md`
  - `tasks/LEVEL_1_2_SCOPE.md`

Behavior implemented:

- Added `src/rules/sorcerer_metamagic.rs` with Careful Spell projections for save-gated damage and save-gated no-effect cases.
- Projected Magic Action consumption, unchanged Bonus Action availability, one Sorcery Point spent from the fixed witness pool, unchanged target Hit Points, unchanged target active-effect count, scenario result, and resolved protocol.
- Kept exact QNT branch actions, scenario labels, and protocol strings quarantined in `src/qnt_adapters/battle_runtime_sorcerer_metamagic_careful_selected_identity.rs`.

Generated branch coverage:

| Obligation | Target replay evidence | Diagnostic tests | Status |
| --- | --- | --- | --- |
| `cleanroom-input/qnt/battle-runtime/battle-runtime-sorcerer-metamagic-careful-selected-identity.mbt.qnt#step:doResolveCarefulSaveGatedDamage` | `tasks/target-replay-evidence/T045-battle-runtime-sorcerer-metamagic-careful-selected-identity.json#T045-resolve-careful-save-gated-damage#step:doResolveCarefulSaveGatedDamage` | `src/tests/mod.rs::careful_spell_adapter_replays_all_branches` | `covered` |
| `cleanroom-input/qnt/battle-runtime/battle-runtime-sorcerer-metamagic-careful-selected-identity.mbt.qnt#step:doResolveCarefulSaveGatedNoEffect` | `tasks/target-replay-evidence/T045-battle-runtime-sorcerer-metamagic-careful-selected-identity.json#T045-resolve-careful-save-gated-no-effect#step:doResolveCarefulSaveGatedNoEffect` | `src/tests/mod.rs::careful_spell_adapter_replays_all_branches` | `covered` |

Target replay evidence:

- Evidence file: `tasks/target-replay-evidence/T045-battle-runtime-sorcerer-metamagic-careful-selected-identity.json`
- Target profile: `rust`
- Target profile SHA-256: `6d4cc6c6a4769962798133d57aff01438fb2b661941f71d1aa8a3333f4b7ecc1`
- Quint binding: Rust quint-connect harness
- Reproduction seed or trace id: `T045-resolve-careful-save-gated-damage`
- Reproduction seed or trace id: `T045-resolve-careful-save-gated-no-effect`

Harness artifacts:

- Start gate: `tasks/START_GATE.json`
- Engine depth: `tasks/ENGINE_DEPTH_MANIFEST.json`
- State ownership: `tasks/STATE_OWNER_MANIFEST.json`
- Reviewer loop: `tasks/REVIEW_LOOP.json`
- Decider decision: `tasks/DECIDER_DECISION.json`

Diagnostic tests:

- `src/tests/mod.rs::careful_spell_projects_save_gated_damage_and_no_effect_cases`

Remaining gaps:

- `_none_`

Verification results:

- `cargo fmt --check` passed.
- `cargo test` passed.
- `cargo clippy --all-targets -- -D warnings` passed.
- `node scripts/check-cleanroom-harness.cjs` passed.

## T044: battle-runtime-sleep-repeat-save

- Manifest source commit SHA: `04249edf345a7752de2f1551dd3d509a2fffc160`
- Source branch inventory SHA: `b4e7e101def7969fc420563dc4da020c22e700f0dc0cc1d27accad6e8631225d`
- Driver: `cleanroom-input/qnt/battle-runtime/battle-runtime-sleep-repeat-save.mbt.qnt`
- Branch obligations:
  - `step:doBreakConcentrationBeforeRepeat`
  - `step:doDiscoverRepeatSave`
  - `step:doEndCasterTurn`
  - `step:doEndCasterTurnAfterConcentrationBreak`
  - `step:doEndTargetTurnAfterConcentrationBreak`
  - `step:doFillInitialSaveFailure`
  - `step:doFillRepeatSaveFailure`
  - `step:doFillRepeatSaveSuccess`
- Allowed inputs used:
  - `cleanroom-input/MANIFEST.md`
  - `cleanroom-input/branch-coverage/source-branch-inventory.json`
  - `cleanroom-input/qnt/battle-runtime/battle-runtime-sleep-repeat-save.mbt.qnt`
  - `cleanroom-input/qnt/battle-runtime/battle-runtime-witness-protocol.qnt`
  - `cleanroom-input/raw/srd-5.2.1/Spells/Descriptions-S-Z.md`
  - `cleanroom-input/raw/srd-5.2.1/Rules-Glossary.md`
  - `cleanroom-input/domain/CLEANROOM_ASSUMPTIONS.md`
  - `cleanroom-input/domain/UBIQUITOUS_LANGUAGE.md`
  - `tasks/LEVEL_1_2_SCOPE.md`

Behavior implemented:

- Added `src/rules/sleep_repeat_save.rs` with Sleep lifecycle projections for the fixed failed-initial-save scenario: target Incapacitated under Sleep, caster Concentration, action consumption, caster and target turn advancement, repeat Saving Throw discovery, repeat-save success, repeat-save failure, and concentration-break cleanup.
- Projected repeat-save failure as target Incapacitated, Unconscious, and Prone, matching Sleep RAW and the Unconscious condition glossary.
- Kept exact QNT branch actions, turn-role labels, hole labels, and protocol strings quarantined in `src/qnt_adapters/battle_runtime_sleep_repeat_save.rs`.

Generated branch coverage:

| Obligation | Target replay evidence | Diagnostic tests | Status |
| --- | --- | --- | --- |
| `cleanroom-input/qnt/battle-runtime/battle-runtime-sleep-repeat-save.mbt.qnt#step:doBreakConcentrationBeforeRepeat` | `tasks/target-replay-evidence/T044-battle-runtime-sleep-repeat-save.json#T044-break-concentration-before-repeat#step:doBreakConcentrationBeforeRepeat` | `src/tests/mod.rs::sleep_repeat_save_adapter_replays_all_branches` | `covered` |
| `cleanroom-input/qnt/battle-runtime/battle-runtime-sleep-repeat-save.mbt.qnt#step:doDiscoverRepeatSave` | `tasks/target-replay-evidence/T044-battle-runtime-sleep-repeat-save.json#T044-discover-repeat-save#step:doDiscoverRepeatSave` | `src/tests/mod.rs::sleep_repeat_save_adapter_replays_all_branches` | `covered` |
| `cleanroom-input/qnt/battle-runtime/battle-runtime-sleep-repeat-save.mbt.qnt#step:doEndCasterTurn` | `tasks/target-replay-evidence/T044-battle-runtime-sleep-repeat-save.json#T044-end-caster-turn#step:doEndCasterTurn` | `src/tests/mod.rs::sleep_repeat_save_adapter_replays_all_branches` | `covered` |
| `cleanroom-input/qnt/battle-runtime/battle-runtime-sleep-repeat-save.mbt.qnt#step:doEndCasterTurnAfterConcentrationBreak` | `tasks/target-replay-evidence/T044-battle-runtime-sleep-repeat-save.json#T044-end-caster-turn-after-concentration-break#step:doEndCasterTurnAfterConcentrationBreak` | `src/tests/mod.rs::sleep_repeat_save_adapter_replays_all_branches` | `covered` |
| `cleanroom-input/qnt/battle-runtime/battle-runtime-sleep-repeat-save.mbt.qnt#step:doEndTargetTurnAfterConcentrationBreak` | `tasks/target-replay-evidence/T044-battle-runtime-sleep-repeat-save.json#T044-end-target-turn-after-concentration-break#step:doEndTargetTurnAfterConcentrationBreak` | `src/tests/mod.rs::sleep_repeat_save_adapter_replays_all_branches` | `covered` |
| `cleanroom-input/qnt/battle-runtime/battle-runtime-sleep-repeat-save.mbt.qnt#step:doFillInitialSaveFailure` | `tasks/target-replay-evidence/T044-battle-runtime-sleep-repeat-save.json#T044-fill-initial-save-failure#step:doFillInitialSaveFailure` | `src/tests/mod.rs::sleep_repeat_save_adapter_replays_all_branches` | `covered` |
| `cleanroom-input/qnt/battle-runtime/battle-runtime-sleep-repeat-save.mbt.qnt#step:doFillRepeatSaveFailure` | `tasks/target-replay-evidence/T044-battle-runtime-sleep-repeat-save.json#T044-fill-repeat-save-failure#step:doFillRepeatSaveFailure` | `src/tests/mod.rs::sleep_repeat_save_adapter_replays_all_branches` | `covered` |
| `cleanroom-input/qnt/battle-runtime/battle-runtime-sleep-repeat-save.mbt.qnt#step:doFillRepeatSaveSuccess` | `tasks/target-replay-evidence/T044-battle-runtime-sleep-repeat-save.json#T044-fill-repeat-save-success#step:doFillRepeatSaveSuccess` | `src/tests/mod.rs::sleep_repeat_save_adapter_replays_all_branches` | `covered` |

Target replay evidence:

- Evidence file: `tasks/target-replay-evidence/T044-battle-runtime-sleep-repeat-save.json`
- Target profile: `rust`
- Target profile SHA-256: `6d4cc6c6a4769962798133d57aff01438fb2b661941f71d1aa8a3333f4b7ecc1`
- Quint binding: Rust quint-connect harness
- Reproduction seed or trace id: `T044-break-concentration-before-repeat`
- Reproduction seed or trace id: `T044-discover-repeat-save`
- Reproduction seed or trace id: `T044-end-caster-turn`
- Reproduction seed or trace id: `T044-end-caster-turn-after-concentration-break`
- Reproduction seed or trace id: `T044-end-target-turn-after-concentration-break`
- Reproduction seed or trace id: `T044-fill-initial-save-failure`
- Reproduction seed or trace id: `T044-fill-repeat-save-failure`
- Reproduction seed or trace id: `T044-fill-repeat-save-success`

Harness artifacts:

- Start gate: `tasks/START_GATE.json`
- Engine depth: `tasks/ENGINE_DEPTH_MANIFEST.json`
- State ownership: `tasks/STATE_OWNER_MANIFEST.json`
- Reviewer loop: `tasks/REVIEW_LOOP.json`
- Decider decision: `tasks/DECIDER_DECISION.json`

Diagnostic tests:

- `src/tests/mod.rs::sleep_repeat_save_projects_initial_repeat_and_concentration_paths`

Remaining gaps:

- `_none_`

Verification results:

- `cargo fmt --check` passed.
- `cargo test` passed.
- `cargo clippy --all-targets -- -D warnings` passed.
- `node scripts/check-cleanroom-harness.cjs` passed.

## T043: battle-runtime-scalar-buff

- Manifest source commit SHA: `04249edf345a7752de2f1551dd3d509a2fffc160`
- Source branch inventory SHA: `b4e7e101def7969fc420563dc4da020c22e700f0dc0cc1d27accad6e8631225d`
- Driver: `cleanroom-input/qnt/battle-runtime/battle-runtime-scalar-buff.mbt.qnt`
- Branch obligations:
  - `step:doFillLongstriderTarget`
  - `step:doRejectStaleAfterResolved`
- Allowed inputs used:
  - `cleanroom-input/MANIFEST.md`
  - `cleanroom-input/branch-coverage/source-branch-inventory.json`
  - `cleanroom-input/qnt/battle-runtime/battle-runtime-scalar-buff.mbt.qnt`
  - `cleanroom-input/qnt/battle-runtime/battle-runtime-witness-protocol.qnt`
  - `cleanroom-input/raw/srd-5.2.1/Spells/Descriptions-E-L.md`
  - `cleanroom-input/raw/srd-5.2.1/Playing-the-Game.md`
  - `cleanroom-input/domain/UBIQUITOUS_LANGUAGE.md`
  - `tasks/ACTIVE_WORK.json`
  - `tasks/LEVEL_1_2_SCOPE.md`

Behavior implemented:

- Added `src/rules/scalar_buff.rs` with a Longstrider target-choice projection: initial Fighter and Goblin Speed 30 feet, a pending Target Choice hole, an available action, and a fill that targets the Goblin, increases the Goblin Speed to 40 feet, resolves the protocol, and spends the action.
- Added stale-subject rejection after the Longstrider target choice has resolved, preserving the resolved Speed/action state while projecting `WStaleSubject`.
- Kept exact QNT action names, hole labels, protocol labels, and witness invalid reason labels quarantined in `src/qnt_adapters/battle_runtime_scalar_buff.rs`.

Generated branch coverage:

| Obligation | Target replay evidence | Diagnostic tests | Status |
| --- | --- | --- | --- |
| `cleanroom-input/qnt/battle-runtime/battle-runtime-scalar-buff.mbt.qnt#step:doFillLongstriderTarget` | `tasks/target-replay-evidence/T043-battle-runtime-scalar-buff.json#T043-fill-longstrider-target#step:doFillLongstriderTarget` | `src/tests/mod.rs::scalar_buff_adapter_replays_all_branches` | `covered` |
| `cleanroom-input/qnt/battle-runtime/battle-runtime-scalar-buff.mbt.qnt#step:doRejectStaleAfterResolved` | `tasks/target-replay-evidence/T043-battle-runtime-scalar-buff.json#T043-reject-stale-after-resolved#step:doRejectStaleAfterResolved` | `src/tests/mod.rs::scalar_buff_adapter_replays_all_branches` | `covered` |

Target replay evidence:

- Evidence file: `tasks/target-replay-evidence/T043-battle-runtime-scalar-buff.json`
- Target profile: `rust`
- Target profile SHA-256: `6d4cc6c6a4769962798133d57aff01438fb2b661941f71d1aa8a3333f4b7ecc1`
- Quint binding: Rust quint-connect harness
- Reproduction seed or trace id: `T043-fill-longstrider-target`
- Reproduction seed or trace id: `T043-reject-stale-after-resolved`

Harness artifacts:

- Start gate: `tasks/START_GATE.json`
- Engine depth: `tasks/ENGINE_DEPTH_MANIFEST.json`
- State ownership: `tasks/STATE_OWNER_MANIFEST.json`
- Reviewer loop: `tasks/REVIEW_LOOP.json`
- Decider decision: `tasks/DECIDER_DECISION.json`

Diagnostic tests:

- `src/tests/mod.rs::scalar_buff_projects_longstrider_target_and_stale_rejection`

Remaining gaps:

- `_none_`

Verification results:

- `cargo fmt --check` passed.
- `cargo test` passed.
- `cargo clippy --all-targets -- -D warnings` passed.
- `node scripts/check-cleanroom-harness.cjs` passed.

## T042: battle-runtime-scalar-buff-active-effects

- Manifest source commit SHA: `04249edf345a7752de2f1551dd3d509a2fffc160`
- Source branch inventory SHA: `b4e7e101def7969fc420563dc4da020c22e700f0dc0cc1d27accad6e8631225d`
- Driver: `cleanroom-input/qnt/battle-runtime/battle-runtime-scalar-buff-active-effects.mbt.qnt`
- Branch obligations:
  - `step:doCastAid`
  - `step:doCastFalseLife`
  - `step:doCastLongstrider`
  - `step:doCastShieldOfFaith`
  - `step:doCastSpiderClimb`
  - `step:doStutter`
- Allowed inputs used:
  - `cleanroom-input/MANIFEST.md`
  - `cleanroom-input/branch-coverage/source-branch-inventory.json`
  - `cleanroom-input/qnt/battle-runtime/battle-runtime-scalar-buff-active-effects.mbt.qnt`
  - `cleanroom-input/raw/srd-5.2.1/Spells/Descriptions-A-D.md`
  - `cleanroom-input/raw/srd-5.2.1/Spells/Descriptions-E-L.md`
  - `cleanroom-input/raw/srd-5.2.1/Spells/Descriptions-S-Z.md`
  - `cleanroom-input/raw/srd-5.2.1/Playing-the-Game.md`
  - `cleanroom-input/raw/srd-5.2.1/Rules-Glossary.md`
  - `cleanroom-input/domain/UBIQUITOUS_LANGUAGE.md`
  - `cleanroom-input/guidance/README.md`
  - `tasks/LEVEL_1_2_SCOPE.md`

Behavior implemented:

- Added `src/rules/scalar_buff_active_effects.rs` with scalar buff active-effect projections for Aid, False Life, Longstrider, Shield of Faith, Spider Climb, and the stutter branch over False Life state.
- Projected Armor Class, Speed, Climb Speed, Hit Point maximum/current, Temporary Hit Points, active effect flags, concentration, scenario result, and protocol result from the QNT driver and cited RAW spell/effect text.
- Kept exact QNT action names, scenario labels, and protocol strings quarantined in `src/qnt_adapters/battle_runtime_scalar_buff_active_effects.rs`.

Generated branch coverage:

| Obligation | Target replay evidence | Diagnostic tests | Status |
| --- | --- | --- | --- |
| `cleanroom-input/qnt/battle-runtime/battle-runtime-scalar-buff-active-effects.mbt.qnt#step:doCastAid` | `tasks/target-replay-evidence/T042-battle-runtime-scalar-buff-active-effects.json#T042-cast-aid#step:doCastAid` | `src/tests/mod.rs::scalar_buff_active_effects_adapter_replays_all_branches` | `covered` |
| `cleanroom-input/qnt/battle-runtime/battle-runtime-scalar-buff-active-effects.mbt.qnt#step:doCastFalseLife` | `tasks/target-replay-evidence/T042-battle-runtime-scalar-buff-active-effects.json#T042-cast-false-life#step:doCastFalseLife` | `src/tests/mod.rs::scalar_buff_active_effects_adapter_replays_all_branches` | `covered` |
| `cleanroom-input/qnt/battle-runtime/battle-runtime-scalar-buff-active-effects.mbt.qnt#step:doCastLongstrider` | `tasks/target-replay-evidence/T042-battle-runtime-scalar-buff-active-effects.json#T042-cast-longstrider#step:doCastLongstrider` | `src/tests/mod.rs::scalar_buff_active_effects_adapter_replays_all_branches` | `covered` |
| `cleanroom-input/qnt/battle-runtime/battle-runtime-scalar-buff-active-effects.mbt.qnt#step:doCastShieldOfFaith` | `tasks/target-replay-evidence/T042-battle-runtime-scalar-buff-active-effects.json#T042-cast-shield-of-faith#step:doCastShieldOfFaith` | `src/tests/mod.rs::scalar_buff_active_effects_adapter_replays_all_branches` | `covered` |
| `cleanroom-input/qnt/battle-runtime/battle-runtime-scalar-buff-active-effects.mbt.qnt#step:doCastSpiderClimb` | `tasks/target-replay-evidence/T042-battle-runtime-scalar-buff-active-effects.json#T042-cast-spider-climb#step:doCastSpiderClimb` | `src/tests/mod.rs::scalar_buff_active_effects_adapter_replays_all_branches` | `covered` |
| `cleanroom-input/qnt/battle-runtime/battle-runtime-scalar-buff-active-effects.mbt.qnt#step:doStutter` | `tasks/target-replay-evidence/T042-battle-runtime-scalar-buff-active-effects.json#T042-stutter-false-life#step:doStutter` | `src/tests/mod.rs::scalar_buff_active_effects_adapter_replays_all_branches` | `covered` |

Target replay evidence:

- Evidence file: `tasks/target-replay-evidence/T042-battle-runtime-scalar-buff-active-effects.json`
- Target profile: `rust`
- Target profile SHA-256: `6d4cc6c6a4769962798133d57aff01438fb2b661941f71d1aa8a3333f4b7ecc1`
- Quint binding: Rust quint-connect harness
- Reproduction seed or trace id: `T042-cast-aid`
- Reproduction seed or trace id: `T042-cast-false-life`
- Reproduction seed or trace id: `T042-cast-longstrider`
- Reproduction seed or trace id: `T042-cast-shield-of-faith`
- Reproduction seed or trace id: `T042-cast-spider-climb`
- Reproduction seed or trace id: `T042-stutter-false-life`

Harness artifacts:

- Start gate: `tasks/START_GATE.json`
- Engine depth: `tasks/ENGINE_DEPTH_MANIFEST.json`
- State ownership: `tasks/STATE_OWNER_MANIFEST.json`
- Reviewer loop: `tasks/REVIEW_LOOP.json`
- Decider decision: `tasks/DECIDER_DECISION.json`

Diagnostic tests:

- `src/tests/mod.rs::scalar_buff_active_effects_project_spell_scalar_fields`

Remaining gaps:

- `_none_`

Verification results:

- `cargo fmt --check` passed.
- `cargo test` passed.
- `cargo clippy --all-targets -- -D warnings` passed.
- `node scripts/check-cleanroom-harness.cjs` passed.

## T041: battle-runtime-save-gated-spell-ordering

- Manifest source commit SHA: `04249edf345a7752de2f1551dd3d509a2fffc160`
- Source branch inventory SHA: `b4e7e101def7969fc420563dc4da020c22e700f0dc0cc1d27accad6e8631225d`
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
  - `cleanroom-input/qnt/battle-runtime/battle-runtime-save-gated-spell-ordering.mbt.qnt`
  - `cleanroom-input/qnt/battle-runtime/battle-runtime-save-gated-spell-ordering.qnt`
  - `cleanroom-input/qnt/battle-runtime/battle-runtime-hole-kinds.qnt`
  - `cleanroom-input/qnt/battle-runtime/battle-runtime-fill-kinds.qnt`
  - `cleanroom-input/raw/srd-5.2.1/Playing-the-Game.md`
  - `cleanroom-input/raw/srd-5.2.1/Rules-Glossary.md`
  - `cleanroom-input/domain/UBIQUITOUS_LANGUAGE.md`
  - `cleanroom-input/guidance/README.md`
  - `tasks/LEVEL_1_2_SCOPE.md`

Behavior implemented:

- Added `src/rules/save_gated_spell_ordering.rs` with typed frontier stages, hole kinds, fill kinds, ordering errors, runtime results, protocol states, and projections for save-gated spell ordering.
- Implemented the area-damage path requiring a Saving Throw outcome before damage dice, with premature damage fill retaining the Saving Throw frontier and recording `savingThrowRequired`.
- Implemented target-list plus condition-choice ordering in either accepted order before the condition Saving Throw outcome, resolving after that save.
- Kept exact QNT branch actions, stage labels, hole labels, error labels, and protocol strings quarantined in `src/qnt_adapters/battle_runtime_save_gated_spell_ordering.rs`.

Generated branch coverage:

| Obligation | Target replay evidence | Diagnostic tests | Status |
| --- | --- | --- | --- |
| `cleanroom-input/qnt/battle-runtime/battle-runtime-save-gated-spell-ordering.mbt.qnt#step:doDiscoverAreaSaveDamage` | `tasks/target-replay-evidence/T041-battle-runtime-save-gated-spell-ordering.json#T041-discover-area-save-damage#step:doDiscoverAreaSaveDamage` | `src/tests/mod.rs::save_gated_spell_ordering_adapter_replays_all_branches` | `covered` |
| `cleanroom-input/qnt/battle-runtime/battle-runtime-save-gated-spell-ordering.mbt.qnt#step:doDiscoverTargetListConditionChoice` | `tasks/target-replay-evidence/T041-battle-runtime-save-gated-spell-ordering.json#T041-discover-target-list-condition-choice#step:doDiscoverTargetListConditionChoice` | `src/tests/mod.rs::save_gated_spell_ordering_adapter_replays_all_branches` | `covered` |
| `cleanroom-input/qnt/battle-runtime/battle-runtime-save-gated-spell-ordering.mbt.qnt#step:doFillAreaDamageDice` | `tasks/target-replay-evidence/T041-battle-runtime-save-gated-spell-ordering.json#T041-fill-area-damage-dice#step:doFillAreaDamageDice` | `src/tests/mod.rs::save_gated_spell_ordering_adapter_replays_all_branches` | `covered` |
| `cleanroom-input/qnt/battle-runtime/battle-runtime-save-gated-spell-ordering.mbt.qnt#step:doFillAreaSaveFailed` | `tasks/target-replay-evidence/T041-battle-runtime-save-gated-spell-ordering.json#T041-fill-area-save-failed#step:doFillAreaSaveFailed` | `src/tests/mod.rs::save_gated_spell_ordering_adapter_replays_all_branches` | `covered` |
| `cleanroom-input/qnt/battle-runtime/battle-runtime-save-gated-spell-ordering.mbt.qnt#step:doFillConditionChoiceAfterTargetList` | `tasks/target-replay-evidence/T041-battle-runtime-save-gated-spell-ordering.json#T041-fill-condition-choice-after-target-list#step:doFillConditionChoiceAfterTargetList` | `src/tests/mod.rs::save_gated_spell_ordering_adapter_replays_all_branches` | `covered` |
| `cleanroom-input/qnt/battle-runtime/battle-runtime-save-gated-spell-ordering.mbt.qnt#step:doFillConditionChoiceBeforeTargetList` | `tasks/target-replay-evidence/T041-battle-runtime-save-gated-spell-ordering.json#T041-fill-condition-choice-before-target-list#step:doFillConditionChoiceBeforeTargetList` | `src/tests/mod.rs::save_gated_spell_ordering_adapter_replays_all_branches` | `covered` |
| `cleanroom-input/qnt/battle-runtime/battle-runtime-save-gated-spell-ordering.mbt.qnt#step:doFillConditionSavingThrow` | `tasks/target-replay-evidence/T041-battle-runtime-save-gated-spell-ordering.json#T041-fill-condition-saving-throw#step:doFillConditionSavingThrow` | `src/tests/mod.rs::save_gated_spell_ordering_adapter_replays_all_branches` | `covered` |
| `cleanroom-input/qnt/battle-runtime/battle-runtime-save-gated-spell-ordering.mbt.qnt#step:doFillTargetListAfterConditionChoice` | `tasks/target-replay-evidence/T041-battle-runtime-save-gated-spell-ordering.json#T041-fill-target-list-after-condition-choice#step:doFillTargetListAfterConditionChoice` | `src/tests/mod.rs::save_gated_spell_ordering_adapter_replays_all_branches` | `covered` |
| `cleanroom-input/qnt/battle-runtime/battle-runtime-save-gated-spell-ordering.mbt.qnt#step:doFillTargetListBeforeConditionChoice` | `tasks/target-replay-evidence/T041-battle-runtime-save-gated-spell-ordering.json#T041-fill-target-list-before-condition-choice#step:doFillTargetListBeforeConditionChoice` | `src/tests/mod.rs::save_gated_spell_ordering_adapter_replays_all_branches` | `covered` |
| `cleanroom-input/qnt/battle-runtime/battle-runtime-save-gated-spell-ordering.mbt.qnt#step:doSubmitDamageBeforeSavingThrow` | `tasks/target-replay-evidence/T041-battle-runtime-save-gated-spell-ordering.json#T041-submit-damage-before-saving-throw#step:doSubmitDamageBeforeSavingThrow` | `src/tests/mod.rs::save_gated_spell_ordering_adapter_replays_all_branches` | `covered` |

Target replay evidence:

- Evidence file: `tasks/target-replay-evidence/T041-battle-runtime-save-gated-spell-ordering.json`
- Target profile: `rust`
- Target profile SHA-256: `6d4cc6c6a4769962798133d57aff01438fb2b661941f71d1aa8a3333f4b7ecc1`
- Quint binding: Rust quint-connect harness
- Reproduction seed or trace id: `T041-discover-area-save-damage`
- Reproduction seed or trace id: `T041-discover-target-list-condition-choice`
- Reproduction seed or trace id: `T041-fill-area-damage-dice`
- Reproduction seed or trace id: `T041-fill-area-save-failed`
- Reproduction seed or trace id: `T041-fill-condition-choice-after-target-list`
- Reproduction seed or trace id: `T041-fill-condition-choice-before-target-list`
- Reproduction seed or trace id: `T041-fill-condition-saving-throw`
- Reproduction seed or trace id: `T041-fill-target-list-after-condition-choice`
- Reproduction seed or trace id: `T041-fill-target-list-before-condition-choice`
- Reproduction seed or trace id: `T041-submit-damage-before-saving-throw`

Harness artifacts:

- Start gate: `tasks/START_GATE.json`
- Engine depth: `tasks/ENGINE_DEPTH_MANIFEST.json`
- State ownership: `tasks/STATE_OWNER_MANIFEST.json`
- Reviewer loop: `tasks/REVIEW_LOOP.json`
- Decider decision: `tasks/DECIDER_DECISION.json`

Diagnostic tests:

- `src/tests/mod.rs::save_gated_spell_ordering_tracks_area_damage_and_condition_paths`

Remaining gaps:

- `_none_`

Verification results:

- `cargo fmt --check` passed.
- `cargo test` passed.
- `cargo clippy --all-targets -- -D warnings` passed.
- `node scripts/check-cleanroom-harness.cjs` passed.

## T040: battle-runtime-sanctuary-selected-identity

- Manifest source commit SHA: `04249edf345a7752de2f1551dd3d509a2fffc160`
- Source branch inventory SHA: `b4e7e101def7969fc420563dc4da020c22e700f0dc0cc1d27accad6e8631225d`
- Driver: `cleanroom-input/qnt/battle-runtime/battle-runtime-sanctuary-selected-identity.mbt.qnt`
- Branch obligations:
  - `step:doCastSanctuaryWardCreation`
  - `step:doEndWardOnWardedAttackRoll`
  - `step:doEndWardOnWardedDamageDealt`
  - `step:doEndWardOnWardedSpellCast`
  - `step:doExcludeAreaEffectFromInterdiction`
  - `step:doInterdictDirectAttackFailedSaveLoss`
  - `step:doInterdictDirectSpellSuccessfulSavePassThrough`
  - `step:doRejectIllegalReplacementTarget`
  - `step:doRetargetDirectAttackToLegalReplacement`
- Allowed inputs used:
  - `cleanroom-input/MANIFEST.md`
  - `cleanroom-input/branch-coverage/source-branch-inventory.json`
  - `cleanroom-input/qnt/battle-runtime/battle-runtime-sanctuary-selected-identity.mbt.qnt`
  - `cleanroom-input/raw/srd-5.2.1/Spells/Descriptions-S-Z.md`
  - `cleanroom-input/domain/UBIQUITOUS_LANGUAGE.md`
  - `cleanroom-input/guidance/README.md`
  - `tasks/LEVEL_1_2_SCOPE.md`

Behavior implemented:

- Added `src/rules/sanctuary_selected_identity.rs` with typed Sanctuary selected-identity projections for ward creation, direct attack and spell interdiction, legal and illegal replacement targets, area-effect exclusion, and ward-ending attack, spell, or damage events.
- Projected ward presence, Sanctuary source identity, Wisdom saving throw request, lost attack or spell, successful save pass-through, replacement pass-through or rejection, area-effect bypass, warded Hit Points, scenario outcome, and protocol result from the QNT driver and cited Sanctuary RAW.
- Kept exact QNT action names, witness scenario labels, and protocol strings quarantined in `src/qnt_adapters/battle_runtime_sanctuary_selected_identity.rs`.

Generated branch coverage:

| Obligation | Target replay evidence | Diagnostic tests | Status |
| --- | --- | --- | --- |
| `cleanroom-input/qnt/battle-runtime/battle-runtime-sanctuary-selected-identity.mbt.qnt#step:doCastSanctuaryWardCreation` | `tasks/target-replay-evidence/T040-battle-runtime-sanctuary-selected-identity.json#T040-cast-sanctuary-ward-creation#step:doCastSanctuaryWardCreation` | `src/tests/mod.rs::sanctuary_selected_identity_adapter_replays_all_branches` | `covered` |
| `cleanroom-input/qnt/battle-runtime/battle-runtime-sanctuary-selected-identity.mbt.qnt#step:doEndWardOnWardedAttackRoll` | `tasks/target-replay-evidence/T040-battle-runtime-sanctuary-selected-identity.json#T040-end-ward-on-warded-attack-roll#step:doEndWardOnWardedAttackRoll` | `src/tests/mod.rs::sanctuary_selected_identity_adapter_replays_all_branches` | `covered` |
| `cleanroom-input/qnt/battle-runtime/battle-runtime-sanctuary-selected-identity.mbt.qnt#step:doEndWardOnWardedDamageDealt` | `tasks/target-replay-evidence/T040-battle-runtime-sanctuary-selected-identity.json#T040-end-ward-on-warded-damage-dealt#step:doEndWardOnWardedDamageDealt` | `src/tests/mod.rs::sanctuary_selected_identity_adapter_replays_all_branches` | `covered` |
| `cleanroom-input/qnt/battle-runtime/battle-runtime-sanctuary-selected-identity.mbt.qnt#step:doEndWardOnWardedSpellCast` | `tasks/target-replay-evidence/T040-battle-runtime-sanctuary-selected-identity.json#T040-end-ward-on-warded-spell-cast#step:doEndWardOnWardedSpellCast` | `src/tests/mod.rs::sanctuary_selected_identity_adapter_replays_all_branches` | `covered` |
| `cleanroom-input/qnt/battle-runtime/battle-runtime-sanctuary-selected-identity.mbt.qnt#step:doExcludeAreaEffectFromInterdiction` | `tasks/target-replay-evidence/T040-battle-runtime-sanctuary-selected-identity.json#T040-exclude-area-effect-from-interdiction#step:doExcludeAreaEffectFromInterdiction` | `src/tests/mod.rs::sanctuary_selected_identity_adapter_replays_all_branches` | `covered` |
| `cleanroom-input/qnt/battle-runtime/battle-runtime-sanctuary-selected-identity.mbt.qnt#step:doInterdictDirectAttackFailedSaveLoss` | `tasks/target-replay-evidence/T040-battle-runtime-sanctuary-selected-identity.json#T040-interdict-direct-attack-failed-save-loss#step:doInterdictDirectAttackFailedSaveLoss` | `src/tests/mod.rs::sanctuary_selected_identity_adapter_replays_all_branches` | `covered` |
| `cleanroom-input/qnt/battle-runtime/battle-runtime-sanctuary-selected-identity.mbt.qnt#step:doInterdictDirectSpellSuccessfulSavePassThrough` | `tasks/target-replay-evidence/T040-battle-runtime-sanctuary-selected-identity.json#T040-interdict-direct-spell-successful-save-pass-through#step:doInterdictDirectSpellSuccessfulSavePassThrough` | `src/tests/mod.rs::sanctuary_selected_identity_adapter_replays_all_branches` | `covered` |
| `cleanroom-input/qnt/battle-runtime/battle-runtime-sanctuary-selected-identity.mbt.qnt#step:doRejectIllegalReplacementTarget` | `tasks/target-replay-evidence/T040-battle-runtime-sanctuary-selected-identity.json#T040-reject-illegal-replacement-target#step:doRejectIllegalReplacementTarget` | `src/tests/mod.rs::sanctuary_selected_identity_adapter_replays_all_branches` | `covered` |
| `cleanroom-input/qnt/battle-runtime/battle-runtime-sanctuary-selected-identity.mbt.qnt#step:doRetargetDirectAttackToLegalReplacement` | `tasks/target-replay-evidence/T040-battle-runtime-sanctuary-selected-identity.json#T040-retarget-direct-attack-to-legal-replacement#step:doRetargetDirectAttackToLegalReplacement` | `src/tests/mod.rs::sanctuary_selected_identity_adapter_replays_all_branches` | `covered` |

Target replay evidence:

- Evidence file: `tasks/target-replay-evidence/T040-battle-runtime-sanctuary-selected-identity.json`
- Target profile: `rust`
- Target profile SHA-256: `6d4cc6c6a4769962798133d57aff01438fb2b661941f71d1aa8a3333f4b7ecc1`
- Quint binding: Rust quint-connect harness
- Reproduction seed or trace id: `T040-cast-sanctuary-ward-creation`
- Reproduction seed or trace id: `T040-end-ward-on-warded-attack-roll`
- Reproduction seed or trace id: `T040-end-ward-on-warded-damage-dealt`
- Reproduction seed or trace id: `T040-end-ward-on-warded-spell-cast`
- Reproduction seed or trace id: `T040-exclude-area-effect-from-interdiction`
- Reproduction seed or trace id: `T040-interdict-direct-attack-failed-save-loss`
- Reproduction seed or trace id: `T040-interdict-direct-spell-successful-save-pass-through`
- Reproduction seed or trace id: `T040-reject-illegal-replacement-target`
- Reproduction seed or trace id: `T040-retarget-direct-attack-to-legal-replacement`

Harness artifacts:

- Start gate: `tasks/START_GATE.json`
- Engine depth: `tasks/ENGINE_DEPTH_MANIFEST.json`
- State ownership: `tasks/STATE_OWNER_MANIFEST.json`
- Reviewer loop: `tasks/REVIEW_LOOP.json`
- Decider decision: `tasks/DECIDER_DECISION.json`

Diagnostic tests:

- `src/tests/mod.rs::sanctuary_selected_identity_projects_ward_interdiction_and_teardown`

Remaining gaps:

- `_none_`

Verification results:

- `cargo fmt --check` passed.
- `cargo test` passed.
- `cargo clippy --all-targets -- -D warnings` passed.
- `node scripts/check-cleanroom-harness.cjs` passed.

## T039: battle-runtime-roll-modifier-buff-selected-identity

- Manifest source commit SHA: `04249edf345a7752de2f1551dd3d509a2fffc160`
- Source branch inventory SHA: `b4e7e101def7969fc420563dc4da020c22e700f0dc0cc1d27accad6e8631225d`
- Driver: `cleanroom-input/qnt/battle-runtime/battle-runtime-roll-modifier-buff-selected-identity.mbt.qnt`
- Branch obligations:
  - `step:doBaneFailedSavePenalty`
  - `step:doBlessAttackAndSaveModifier`
  - `step:doGuidanceSkillAbilityCheckModifier`
  - `step:doResistanceReducesMatchingDamage`
  - `step:doShieldOfFaithArmorClassBonus`
- Allowed inputs used:
  - `cleanroom-input/MANIFEST.md`
  - `cleanroom-input/branch-coverage/source-branch-inventory.json`
  - `cleanroom-input/qnt/battle-runtime/battle-runtime-roll-modifier-buff-selected-identity.mbt.qnt`
  - `cleanroom-input/raw/srd-5.2.1/Spells/Descriptions-A-D.md`
  - `cleanroom-input/raw/srd-5.2.1/Spells/Descriptions-E-L.md`
  - `cleanroom-input/raw/srd-5.2.1/Spells/Descriptions-Q-R.md`
  - `cleanroom-input/raw/srd-5.2.1/Spells/Descriptions-S-Z.md`
  - `cleanroom-input/domain/UBIQUITOUS_LANGUAGE.md`
  - `cleanroom-input/guidance/README.md`
  - `tasks/LEVEL_1_2_SCOPE.md`

Behavior implemented:

- Added `src/rules/roll_modifier_buff_selected_identity.rs` with typed selected-identity projections for Bane, Bless, Guidance, Resistance, and Shield of Faith.
- Projected caster concentration, effect counts, target Armor Class and Hit Points, D20 modifier sign and scope, Guidance skill identity, invalid-target rejection, Resistance damage type and use, scenario outcome, and resolved protocol state from the QNT driver and cited spell descriptions.
- Kept exact QNT action names, witness scenario labels, D20 sign strings, damage type strings, and protocol strings quarantined in `src/qnt_adapters/battle_runtime_roll_modifier_buff_selected_identity.rs`.

Generated branch coverage:

| Obligation | Target replay evidence | Diagnostic tests | Status |
| --- | --- | --- | --- |
| `cleanroom-input/qnt/battle-runtime/battle-runtime-roll-modifier-buff-selected-identity.mbt.qnt#step:doBaneFailedSavePenalty` | `tasks/target-replay-evidence/T039-battle-runtime-roll-modifier-buff-selected-identity.json#T039-bane-failed-save-penalty#step:doBaneFailedSavePenalty` | `src/tests/mod.rs::roll_modifier_buff_selected_identity_adapter_replays_all_branches` | `covered` |
| `cleanroom-input/qnt/battle-runtime/battle-runtime-roll-modifier-buff-selected-identity.mbt.qnt#step:doBlessAttackAndSaveModifier` | `tasks/target-replay-evidence/T039-battle-runtime-roll-modifier-buff-selected-identity.json#T039-bless-attack-and-save-modifier#step:doBlessAttackAndSaveModifier` | `src/tests/mod.rs::roll_modifier_buff_selected_identity_adapter_replays_all_branches` | `covered` |
| `cleanroom-input/qnt/battle-runtime/battle-runtime-roll-modifier-buff-selected-identity.mbt.qnt#step:doGuidanceSkillAbilityCheckModifier` | `tasks/target-replay-evidence/T039-battle-runtime-roll-modifier-buff-selected-identity.json#T039-guidance-skill-ability-check-modifier#step:doGuidanceSkillAbilityCheckModifier` | `src/tests/mod.rs::roll_modifier_buff_selected_identity_adapter_replays_all_branches` | `covered` |
| `cleanroom-input/qnt/battle-runtime/battle-runtime-roll-modifier-buff-selected-identity.mbt.qnt#step:doResistanceReducesMatchingDamage` | `tasks/target-replay-evidence/T039-battle-runtime-roll-modifier-buff-selected-identity.json#T039-resistance-reduces-matching-damage#step:doResistanceReducesMatchingDamage` | `src/tests/mod.rs::roll_modifier_buff_selected_identity_adapter_replays_all_branches` | `covered` |
| `cleanroom-input/qnt/battle-runtime/battle-runtime-roll-modifier-buff-selected-identity.mbt.qnt#step:doShieldOfFaithArmorClassBonus` | `tasks/target-replay-evidence/T039-battle-runtime-roll-modifier-buff-selected-identity.json#T039-shield-of-faith-armor-class-bonus#step:doShieldOfFaithArmorClassBonus` | `src/tests/mod.rs::roll_modifier_buff_selected_identity_adapter_replays_all_branches` | `covered` |

Target replay evidence:

- Evidence file: `tasks/target-replay-evidence/T039-battle-runtime-roll-modifier-buff-selected-identity.json`
- Target profile: `rust`
- Target profile SHA-256: `6d4cc6c6a4769962798133d57aff01438fb2b661941f71d1aa8a3333f4b7ecc1`
- Quint binding: Rust quint-connect harness
- Reproduction seed or trace id: `T039-bane-failed-save-penalty`
- Reproduction seed or trace id: `T039-bless-attack-and-save-modifier`
- Reproduction seed or trace id: `T039-guidance-skill-ability-check-modifier`
- Reproduction seed or trace id: `T039-resistance-reduces-matching-damage`
- Reproduction seed or trace id: `T039-shield-of-faith-armor-class-bonus`

Harness artifacts:

- Start gate: `tasks/START_GATE.json`
- Engine depth: `tasks/ENGINE_DEPTH_MANIFEST.json`
- State ownership: `tasks/STATE_OWNER_MANIFEST.json`
- Reviewer loop: `tasks/REVIEW_LOOP.json`
- Decider decision: `tasks/DECIDER_DECISION.json`

Diagnostic tests:

- `src/tests/mod.rs::roll_modifier_buff_selected_identity_projects_spell_modifiers`

Remaining gaps:

- `_none_`

Verification results:

- `cargo fmt --check` passed.
- `cargo test` passed.
- `cargo clippy --all-targets -- -D warnings` passed.
- `node scripts/check-cleanroom-harness.cjs` passed.

## T038: battle-runtime-roll-modifier-active-effects

- Manifest source commit SHA: `04249edf345a7752de2f1551dd3d509a2fffc160`
- Source branch inventory SHA: `b4e7e101def7969fc420563dc4da020c22e700f0dc0cc1d27accad6e8631225d`
- Driver: `cleanroom-input/qnt/battle-runtime/battle-runtime-roll-modifier-active-effects.mbt.qnt`
- Branch obligations:
  - `step:doBreakConcentration`
  - `step:doCastBaneFailed`
  - `step:doCastBless`
  - `step:doCastEnhanceDex`
  - `step:doCastEnhancePerTarget`
  - `step:doCastEnthrall`
  - `step:doCastGuidanceStealth`
  - `step:doCastPassWithoutTrace`
  - `step:doCastThaumaturgyBoomingVoice`
  - `step:doCastThaumaturgyCancelled`
  - `step:doDiscoverBaneSave`
  - `step:doDiscoverEnhanceAbilityChoice`
  - `step:doDiscoverEnhanceTargetAbilityChoices`
  - `step:doDiscoverGuidanceSkillChoice`
  - `step:doDiscoverThaumaturgyCount`
  - `step:doStutter`
- Allowed inputs used:
  - `cleanroom-input/MANIFEST.md`
  - `cleanroom-input/branch-coverage/source-branch-inventory.json`
  - `cleanroom-input/qnt/battle-runtime/battle-runtime-roll-modifier-active-effects.mbt.qnt`
  - `cleanroom-input/raw/srd-5.2.1/Spells/Descriptions-A-D.md`
  - `cleanroom-input/raw/srd-5.2.1/Spells/Descriptions-E-L.md`
  - `cleanroom-input/raw/srd-5.2.1/Spells/Descriptions-M-P.md`
  - `cleanroom-input/raw/srd-5.2.1/Spells/Descriptions-S-Z.md`
  - `cleanroom-input/domain/UBIQUITOUS_LANGUAGE.md`
  - `cleanroom-input/guidance/README.md`
  - `tasks/LEVEL_1_2_SCOPE.md`

Behavior implemented:

- Added `src/rules/roll_modifier_active_effects.rs` with typed active-effect projections for Bane and Bless, Guidance and Pass without Trace, Enhance Ability, Enthrall, Thaumaturgy, concentration cleanup, and stutter replay.
- Projected action and spell availability, concentration, attack and save modifiers, ability-check modifiers, selected skills and abilities, roll modes, passive Perception delta, protocol holes, and scenario outcomes from the QNT driver and cited spell descriptions.
- Kept exact QNT action names, witness labels, protocol strings, and target replay projection strings quarantined in `src/qnt_adapters/battle_runtime_roll_modifier_active_effects.rs`.

Generated branch coverage:

| Obligation | Target replay evidence | Diagnostic tests | Status |
| --- | --- | --- | --- |
| `cleanroom-input/qnt/battle-runtime/battle-runtime-roll-modifier-active-effects.mbt.qnt#step:doDiscoverBaneSave` | `tasks/target-replay-evidence/T038-battle-runtime-roll-modifier-active-effects.json#T038-discover-bane-save#step:doDiscoverBaneSave` | `src/tests/mod.rs::roll_modifier_active_effects_adapter_replays_all_branches` | `covered` |
| `cleanroom-input/qnt/battle-runtime/battle-runtime-roll-modifier-active-effects.mbt.qnt#step:doCastBaneFailed` | `tasks/target-replay-evidence/T038-battle-runtime-roll-modifier-active-effects.json#T038-cast-bane-failed#step:doCastBaneFailed` | `src/tests/mod.rs::roll_modifier_active_effects_adapter_replays_all_branches` | `covered` |
| `cleanroom-input/qnt/battle-runtime/battle-runtime-roll-modifier-active-effects.mbt.qnt#step:doCastBless` | `tasks/target-replay-evidence/T038-battle-runtime-roll-modifier-active-effects.json#T038-cast-bless#step:doCastBless` | `src/tests/mod.rs::roll_modifier_active_effects_adapter_replays_all_branches` | `covered` |
| `cleanroom-input/qnt/battle-runtime/battle-runtime-roll-modifier-active-effects.mbt.qnt#step:doDiscoverGuidanceSkillChoice` | `tasks/target-replay-evidence/T038-battle-runtime-roll-modifier-active-effects.json#T038-discover-guidance-skill-choice#step:doDiscoverGuidanceSkillChoice` | `src/tests/mod.rs::roll_modifier_active_effects_adapter_replays_all_branches` | `covered` |
| `cleanroom-input/qnt/battle-runtime/battle-runtime-roll-modifier-active-effects.mbt.qnt#step:doCastGuidanceStealth` | `tasks/target-replay-evidence/T038-battle-runtime-roll-modifier-active-effects.json#T038-cast-guidance-stealth#step:doCastGuidanceStealth` | `src/tests/mod.rs::roll_modifier_active_effects_adapter_replays_all_branches` | `covered` |
| `cleanroom-input/qnt/battle-runtime/battle-runtime-roll-modifier-active-effects.mbt.qnt#step:doCastPassWithoutTrace` | `tasks/target-replay-evidence/T038-battle-runtime-roll-modifier-active-effects.json#T038-cast-pass-without-trace#step:doCastPassWithoutTrace` | `src/tests/mod.rs::roll_modifier_active_effects_adapter_replays_all_branches` | `covered` |
| `cleanroom-input/qnt/battle-runtime/battle-runtime-roll-modifier-active-effects.mbt.qnt#step:doDiscoverEnhanceAbilityChoice` | `tasks/target-replay-evidence/T038-battle-runtime-roll-modifier-active-effects.json#T038-discover-enhance-ability-choice#step:doDiscoverEnhanceAbilityChoice` | `src/tests/mod.rs::roll_modifier_active_effects_adapter_replays_all_branches` | `covered` |
| `cleanroom-input/qnt/battle-runtime/battle-runtime-roll-modifier-active-effects.mbt.qnt#step:doCastEnhanceDex` | `tasks/target-replay-evidence/T038-battle-runtime-roll-modifier-active-effects.json#T038-cast-enhance-dex#step:doCastEnhanceDex` | `src/tests/mod.rs::roll_modifier_active_effects_adapter_replays_all_branches` | `covered` |
| `cleanroom-input/qnt/battle-runtime/battle-runtime-roll-modifier-active-effects.mbt.qnt#step:doDiscoverEnhanceTargetAbilityChoices` | `tasks/target-replay-evidence/T038-battle-runtime-roll-modifier-active-effects.json#T038-discover-enhance-target-ability-choices#step:doDiscoverEnhanceTargetAbilityChoices` | `src/tests/mod.rs::roll_modifier_active_effects_adapter_replays_all_branches` | `covered` |
| `cleanroom-input/qnt/battle-runtime/battle-runtime-roll-modifier-active-effects.mbt.qnt#step:doCastEnhancePerTarget` | `tasks/target-replay-evidence/T038-battle-runtime-roll-modifier-active-effects.json#T038-cast-enhance-per-target#step:doCastEnhancePerTarget` | `src/tests/mod.rs::roll_modifier_active_effects_adapter_replays_all_branches` | `covered` |
| `cleanroom-input/qnt/battle-runtime/battle-runtime-roll-modifier-active-effects.mbt.qnt#step:doCastEnthrall` | `tasks/target-replay-evidence/T038-battle-runtime-roll-modifier-active-effects.json#T038-cast-enthrall#step:doCastEnthrall` | `src/tests/mod.rs::roll_modifier_active_effects_adapter_replays_all_branches` | `covered` |
| `cleanroom-input/qnt/battle-runtime/battle-runtime-roll-modifier-active-effects.mbt.qnt#step:doDiscoverThaumaturgyCount` | `tasks/target-replay-evidence/T038-battle-runtime-roll-modifier-active-effects.json#T038-discover-thaumaturgy-count#step:doDiscoverThaumaturgyCount` | `src/tests/mod.rs::roll_modifier_active_effects_adapter_replays_all_branches` | `covered` |
| `cleanroom-input/qnt/battle-runtime/battle-runtime-roll-modifier-active-effects.mbt.qnt#step:doCastThaumaturgyBoomingVoice` | `tasks/target-replay-evidence/T038-battle-runtime-roll-modifier-active-effects.json#T038-cast-thaumaturgy-booming-voice#step:doCastThaumaturgyBoomingVoice` | `src/tests/mod.rs::roll_modifier_active_effects_adapter_replays_all_branches` | `covered` |
| `cleanroom-input/qnt/battle-runtime/battle-runtime-roll-modifier-active-effects.mbt.qnt#step:doCastThaumaturgyCancelled` | `tasks/target-replay-evidence/T038-battle-runtime-roll-modifier-active-effects.json#T038-cast-thaumaturgy-cancelled#step:doCastThaumaturgyCancelled` | `src/tests/mod.rs::roll_modifier_active_effects_adapter_replays_all_branches` | `covered` |
| `cleanroom-input/qnt/battle-runtime/battle-runtime-roll-modifier-active-effects.mbt.qnt#step:doBreakConcentration` | `tasks/target-replay-evidence/T038-battle-runtime-roll-modifier-active-effects.json#T038-break-concentration#step:doBreakConcentration` | `src/tests/mod.rs::roll_modifier_active_effects_adapter_replays_all_branches` | `covered` |
| `cleanroom-input/qnt/battle-runtime/battle-runtime-roll-modifier-active-effects.mbt.qnt#step:doStutter` | `tasks/target-replay-evidence/T038-battle-runtime-roll-modifier-active-effects.json#T038-stutter-bless#step:doStutter` | `src/tests/mod.rs::roll_modifier_active_effects_adapter_replays_all_branches` | `covered` |

Target replay evidence:

- Evidence file: `tasks/target-replay-evidence/T038-battle-runtime-roll-modifier-active-effects.json`
- Target profile: `rust`
- Target profile SHA-256: `6d4cc6c6a4769962798133d57aff01438fb2b661941f71d1aa8a3333f4b7ecc1`
- Quint binding: Rust quint-connect harness
- Reproduction seed or trace id: `T038-discover-bane-save`
- Reproduction seed or trace id: `T038-cast-bane-failed`
- Reproduction seed or trace id: `T038-cast-bless`
- Reproduction seed or trace id: `T038-discover-guidance-skill-choice`
- Reproduction seed or trace id: `T038-cast-guidance-stealth`
- Reproduction seed or trace id: `T038-cast-pass-without-trace`
- Reproduction seed or trace id: `T038-discover-enhance-ability-choice`
- Reproduction seed or trace id: `T038-cast-enhance-dex`
- Reproduction seed or trace id: `T038-discover-enhance-target-ability-choices`
- Reproduction seed or trace id: `T038-cast-enhance-per-target`
- Reproduction seed or trace id: `T038-cast-enthrall`
- Reproduction seed or trace id: `T038-discover-thaumaturgy-count`
- Reproduction seed or trace id: `T038-cast-thaumaturgy-booming-voice`
- Reproduction seed or trace id: `T038-cast-thaumaturgy-cancelled`
- Reproduction seed or trace id: `T038-break-concentration`
- Reproduction seed or trace id: `T038-stutter-bless`

Harness artifacts:

- Start gate: `tasks/START_GATE.json`
- Engine depth: `tasks/ENGINE_DEPTH_MANIFEST.json`
- State ownership: `tasks/STATE_OWNER_MANIFEST.json`
- Reviewer loop: `tasks/REVIEW_LOOP.json`
- Decider decision: `tasks/DECIDER_DECISION.json`

Diagnostic tests:

- `src/tests/mod.rs::roll_modifier_active_effects_project_modifiers_holes_and_cleanup`

Remaining gaps:

- `_none_`

Verification results:

- `cargo fmt --check` passed.
- `cargo test` passed.
- `cargo clippy --all-targets -- -D warnings` passed.
- `node scripts/check-cleanroom-harness.cjs` passed.

## T037: battle-runtime-quickened-spell-governor

- Manifest source commit SHA: `04249edf345a7752de2f1551dd3d509a2fffc160`
- Source branch inventory SHA: `b4e7e101def7969fc420563dc4da020c22e700f0dc0cc1d27accad6e8631225d`
- Driver: `cleanroom-input/qnt/battle-runtime/battle-runtime-quickened-spell-governor.mbt.qnt`
- Branch obligations:
  - `step:doResolveQuickenedRestoration`
  - `step:doResolveQuickenedSaveGatedCondition`
  - `step:doResolveQuickenedSaveGatedConditionImmunity`
  - `step:doResolveQuickenedDirectCondition`
  - `step:doResolveQuickenedRollModifier`
  - `step:doResolveQuickenedAfterMagicActionSpent`
  - `step:doRejectUnaffordable`
  - `step:doRejectUnknownOption`
  - `step:doRejectUnsupportedSecondOption`
  - `step:doRejectOnePerSpell`
  - `step:doRejectPriorLevelOnePlusSpell`
- Allowed inputs used:
  - `cleanroom-input/MANIFEST.md`
  - `cleanroom-input/branch-coverage/source-branch-inventory.json`
  - `cleanroom-input/qnt/battle-runtime/battle-runtime-quickened-spell-governor.mbt.qnt`
  - `cleanroom-input/raw/srd-5.2.1/Classes/Sorcerer.md`
  - `cleanroom-input/raw/srd-5.2.1/Spells/Gaining-and-Casting.md`
  - `cleanroom-input/domain/UBIQUITOUS_LANGUAGE.md`
  - `cleanroom-input/guidance/README.md`
  - `tasks/LEVEL_1_2_SCOPE.md`

Behavior implemented:

- Added `src/rules/quickened_spell_governor.rs` with typed Quickened Spell governor projections for restoration, save-gated condition, save-gated immunity, direct condition, roll modifier, post-Magic-action resolution, unaffordable use, unknown option, unsupported second option, one-per-spell rejection, and same-turn level-1+ rejection.
- Projected Sorcery Point spend, Action and Bonus Action availability, spell-slot commitment, same-turn level-1+ spell flags, spell-slot act availability, target Hit Points, and effect booleans from the QNT witness table and RAW Quickened Spell rule.
- Kept exact QNT action names, scenario labels, invalid-kind strings, protocol labels, and target replay witness strings quarantined in `src/qnt_adapters/battle_runtime_quickened_spell_governor.rs`.

Generated branch coverage:

| Obligation | Target replay evidence | Diagnostic tests | Status |
| --- | --- | --- | --- |
| `cleanroom-input/qnt/battle-runtime/battle-runtime-quickened-spell-governor.mbt.qnt#step:doResolveQuickenedRestoration` | `tasks/target-replay-evidence/T037-battle-runtime-quickened-spell-governor.json#T037-resolve-quickened-restoration#step:doResolveQuickenedRestoration` | `src/tests/mod.rs::quickened_spell_governor_adapter_replays_all_branches` | `covered` |
| `cleanroom-input/qnt/battle-runtime/battle-runtime-quickened-spell-governor.mbt.qnt#step:doResolveQuickenedSaveGatedCondition` | `tasks/target-replay-evidence/T037-battle-runtime-quickened-spell-governor.json#T037-resolve-quickened-save-gated-condition#step:doResolveQuickenedSaveGatedCondition` | `src/tests/mod.rs::quickened_spell_governor_adapter_replays_all_branches` | `covered` |
| `cleanroom-input/qnt/battle-runtime/battle-runtime-quickened-spell-governor.mbt.qnt#step:doResolveQuickenedSaveGatedConditionImmunity` | `tasks/target-replay-evidence/T037-battle-runtime-quickened-spell-governor.json#T037-resolve-quickened-save-gated-condition-immunity#step:doResolveQuickenedSaveGatedConditionImmunity` | `src/tests/mod.rs::quickened_spell_governor_adapter_replays_all_branches` | `covered` |
| `cleanroom-input/qnt/battle-runtime/battle-runtime-quickened-spell-governor.mbt.qnt#step:doResolveQuickenedDirectCondition` | `tasks/target-replay-evidence/T037-battle-runtime-quickened-spell-governor.json#T037-resolve-quickened-direct-condition#step:doResolveQuickenedDirectCondition` | `src/tests/mod.rs::quickened_spell_governor_adapter_replays_all_branches` | `covered` |
| `cleanroom-input/qnt/battle-runtime/battle-runtime-quickened-spell-governor.mbt.qnt#step:doResolveQuickenedRollModifier` | `tasks/target-replay-evidence/T037-battle-runtime-quickened-spell-governor.json#T037-resolve-quickened-roll-modifier#step:doResolveQuickenedRollModifier` | `src/tests/mod.rs::quickened_spell_governor_adapter_replays_all_branches` | `covered` |
| `cleanroom-input/qnt/battle-runtime/battle-runtime-quickened-spell-governor.mbt.qnt#step:doResolveQuickenedAfterMagicActionSpent` | `tasks/target-replay-evidence/T037-battle-runtime-quickened-spell-governor.json#T037-resolve-quickened-after-magic-action-spent#step:doResolveQuickenedAfterMagicActionSpent` | `src/tests/mod.rs::quickened_spell_governor_adapter_replays_all_branches` | `covered` |
| `cleanroom-input/qnt/battle-runtime/battle-runtime-quickened-spell-governor.mbt.qnt#step:doRejectUnaffordable` | `tasks/target-replay-evidence/T037-battle-runtime-quickened-spell-governor.json#T037-reject-unaffordable#step:doRejectUnaffordable` | `src/tests/mod.rs::quickened_spell_governor_adapter_replays_all_branches` | `covered` |
| `cleanroom-input/qnt/battle-runtime/battle-runtime-quickened-spell-governor.mbt.qnt#step:doRejectUnknownOption` | `tasks/target-replay-evidence/T037-battle-runtime-quickened-spell-governor.json#T037-reject-unknown-option#step:doRejectUnknownOption` | `src/tests/mod.rs::quickened_spell_governor_adapter_replays_all_branches` | `covered` |
| `cleanroom-input/qnt/battle-runtime/battle-runtime-quickened-spell-governor.mbt.qnt#step:doRejectUnsupportedSecondOption` | `tasks/target-replay-evidence/T037-battle-runtime-quickened-spell-governor.json#T037-reject-unsupported-second-option#step:doRejectUnsupportedSecondOption` | `src/tests/mod.rs::quickened_spell_governor_adapter_replays_all_branches` | `covered` |
| `cleanroom-input/qnt/battle-runtime/battle-runtime-quickened-spell-governor.mbt.qnt#step:doRejectOnePerSpell` | `tasks/target-replay-evidence/T037-battle-runtime-quickened-spell-governor.json#T037-reject-one-per-spell#step:doRejectOnePerSpell` | `src/tests/mod.rs::quickened_spell_governor_adapter_replays_all_branches` | `covered` |
| `cleanroom-input/qnt/battle-runtime/battle-runtime-quickened-spell-governor.mbt.qnt#step:doRejectPriorLevelOnePlusSpell` | `tasks/target-replay-evidence/T037-battle-runtime-quickened-spell-governor.json#T037-reject-prior-level-one-plus-spell#step:doRejectPriorLevelOnePlusSpell` | `src/tests/mod.rs::quickened_spell_governor_adapter_replays_all_branches` | `covered` |

Target replay evidence:

- Evidence file: `tasks/target-replay-evidence/T037-battle-runtime-quickened-spell-governor.json`
- Target profile: `rust`
- Target profile SHA-256: `6d4cc6c6a4769962798133d57aff01438fb2b661941f71d1aa8a3333f4b7ecc1`
- Quint binding: Rust quint-connect harness
- Reproduction seed or trace id: `T037-resolve-quickened-restoration`
- Reproduction seed or trace id: `T037-resolve-quickened-save-gated-condition`
- Reproduction seed or trace id: `T037-resolve-quickened-save-gated-condition-immunity`
- Reproduction seed or trace id: `T037-resolve-quickened-direct-condition`
- Reproduction seed or trace id: `T037-resolve-quickened-roll-modifier`
- Reproduction seed or trace id: `T037-resolve-quickened-after-magic-action-spent`
- Reproduction seed or trace id: `T037-reject-unaffordable`
- Reproduction seed or trace id: `T037-reject-unknown-option`
- Reproduction seed or trace id: `T037-reject-unsupported-second-option`
- Reproduction seed or trace id: `T037-reject-one-per-spell`
- Reproduction seed or trace id: `T037-reject-prior-level-one-plus-spell`

Harness artifacts:

- Start gate: `tasks/START_GATE.json`
- Engine depth: `tasks/ENGINE_DEPTH_MANIFEST.json`
- State ownership: `tasks/STATE_OWNER_MANIFEST.json`
- Reviewer loop: `tasks/REVIEW_LOOP.json`
- Decider decision: `tasks/DECIDER_DECISION.json`

Diagnostic tests:

- `src/tests/mod.rs::quickened_spell_governor_projects_successes_and_rejections`

Remaining gaps:

- `_none_`

Verification results:

- `cargo fmt --check` passed.
- `cargo test` passed.
- `cargo clippy --all-targets -- -D warnings` passed.
- `node scripts/check-cleanroom-harness.cjs` passed.

## T036: battle-runtime-magic-missile

- Manifest source commit SHA: `04249edf345a7752de2f1551dd3d509a2fffc160`
- Source branch inventory SHA: `b4e7e101def7969fc420563dc4da020c22e700f0dc0cc1d27accad6e8631225d`
- Driver: `cleanroom-input/qnt/battle-runtime/battle-runtime-magic-missile.mbt.qnt`
- Branch obligations:
  - `step:doFillMagicMissileAllocation`
  - `step:doFillMagicMissileDamage`
- Allowed inputs used:
  - `cleanroom-input/MANIFEST.md`
  - `cleanroom-input/branch-coverage/source-branch-inventory.json`
  - `cleanroom-input/qnt/battle-runtime/battle-runtime-magic-missile.mbt.qnt`
  - `cleanroom-input/qnt/battle-runtime/battle-runtime-core-combat-tests.qnt`
  - `cleanroom-input/raw/srd-5.2.1/Spells/Descriptions-M-P.md`
  - `cleanroom-input/raw/srd-5.2.1/Spells/Gaining-and-Casting.md`
  - `cleanroom-input/domain/UBIQUITOUS_LANGUAGE.md`
  - `cleanroom-input/guidance/README.md`
  - `tasks/LEVEL_1_2_SCOPE.md`

Behavior implemented:

- Added `src/rules/magic_missile.rs` with typed Magic Missile state for target-allocation and damage-fill protocol stages, skeleton Hit Points/death, Action availability, multiattack and Sneak Attack non-interference fields, and protocol holes/results.
- Implemented three-dart Force damage as the sampled dart pips plus three fixed dart modifiers, with target Hit Points floored at 0 for the high-damage sample.
- Kept exact QNT action names, sampled input name, hole labels, protocol labels, and target replay witness strings quarantined in `src/qnt_adapters/battle_runtime_magic_missile.rs`.

Generated branch coverage:

| Obligation | Target replay evidence | Diagnostic tests | Status |
| --- | --- | --- | --- |
| `cleanroom-input/qnt/battle-runtime/battle-runtime-magic-missile.mbt.qnt#step:doFillMagicMissileAllocation` | `tasks/target-replay-evidence/T036-battle-runtime-magic-missile.json#T036-fill-magic-missile-allocation#step:doFillMagicMissileAllocation` | `src/tests/mod.rs::magic_missile_adapter_replays_all_branches` | `covered` |
| `cleanroom-input/qnt/battle-runtime/battle-runtime-magic-missile.mbt.qnt#step:doFillMagicMissileDamage` | `tasks/target-replay-evidence/T036-battle-runtime-magic-missile.json#T036-fill-magic-missile-damage-low#step:doFillMagicMissileDamage` | `src/tests/mod.rs::magic_missile_adapter_replays_all_branches` | `covered` |

Target replay evidence:

- Evidence file: `tasks/target-replay-evidence/T036-battle-runtime-magic-missile.json`
- Target profile: `rust`
- Target profile SHA-256: `6d4cc6c6a4769962798133d57aff01438fb2b661941f71d1aa8a3333f4b7ecc1`
- Quint binding: Rust quint-connect harness
- Reproduction seed or trace id: `T036-fill-magic-missile-allocation`
- Reproduction seed or trace id: `T036-fill-magic-missile-damage-low` with `dartRollTotal=3`
- Reproduction seed or trace id: `T036-fill-magic-missile-damage-high` with `dartRollTotal=12`

Harness artifacts:

- Start gate: `tasks/START_GATE.json`
- Engine depth: `tasks/ENGINE_DEPTH_MANIFEST.json`
- State ownership: `tasks/STATE_OWNER_MANIFEST.json`
- Reviewer loop: `tasks/REVIEW_LOOP.json`
- Decider decision: `tasks/DECIDER_DECISION.json`

Diagnostic tests:

- `src/tests/mod.rs::magic_missile_projects_allocation_and_sampled_damage`

Remaining gaps:

- `_none_`

Verification results:

- `cargo fmt --check` passed.
- `cargo test` passed.
- `cargo clippy --all-targets -- -D warnings` passed.
- `node scripts/check-cleanroom-harness.cjs` passed.

## T035: battle-runtime-mage-armor-selected-identity

- Manifest source commit SHA: `04249edf345a7752de2f1551dd3d509a2fffc160`
- Source branch inventory SHA: `b4e7e101def7969fc420563dc4da020c22e700f0dc0cc1d27accad6e8631225d`
- Driver: `cleanroom-input/qnt/battle-runtime/battle-runtime-mage-armor-selected-identity.mbt.qnt`
- Branch obligations:
  - `step:doDiscoverMageArmorUnarmoredSelfTarget`
  - `step:doExpireMageArmorDuration`
  - `step:doRejectMageArmorArmoredTarget`
  - `step:doResolveMageArmorBaseArmorClassProjection`
- Allowed inputs used:
  - `cleanroom-input/MANIFEST.md`
  - `cleanroom-input/branch-coverage/source-branch-inventory.json`
  - `cleanroom-input/qnt/battle-runtime/battle-runtime-mage-armor-selected-identity.mbt.qnt`
  - `cleanroom-input/qnt/battle-runtime/battle-runtime-armor-class.qnt`
  - `cleanroom-input/qnt/battle-runtime/battle-runtime-armor-spell-resolution.qnt`
  - `cleanroom-input/qnt/battle-runtime/battle-runtime-hp-armor-buff-spatial-tests.qnt`
  - `cleanroom-input/raw/srd-5.2.1/Playing-the-Game.md`
  - `cleanroom-input/raw/srd-5.2.1/Spells/Descriptions-M-P.md`
  - `cleanroom-input/domain/UBIQUITOUS_LANGUAGE.md`
  - `cleanroom-input/guidance/README.md`
  - `tasks/LEVEL_1_2_SCOPE.md`

Behavior implemented:

- Added `src/rules/level_one_armor_spells.rs` with typed Mage Armor projections for unarmored self-target discovery, armored-target rejection, base Armor Class replacement, duration expiration, level-1 slot expenditure, and Action availability.
- Reused the existing Armor Class formula helper to project default unarmored `10 + Dexterity modifier` and Mage Armor `13 + Dexterity modifier` totals from RAW and QNT.
- Kept exact QNT action names, scenario labels, protocol labels, and target replay witness strings quarantined in `src/qnt_adapters/battle_runtime_mage_armor_selected_identity.rs`.

Generated branch coverage:

| Obligation | Target replay evidence | Diagnostic tests | Status |
| --- | --- | --- | --- |
| `cleanroom-input/qnt/battle-runtime/battle-runtime-mage-armor-selected-identity.mbt.qnt#step:doDiscoverMageArmorUnarmoredSelfTarget` | `tasks/target-replay-evidence/T035-battle-runtime-mage-armor-selected-identity.json#T035-discover-mage-armor-unarmored-self-target#step:doDiscoverMageArmorUnarmoredSelfTarget` | `src/tests/mod.rs::mage_armor_adapter_replays_all_branches` | `covered` |
| `cleanroom-input/qnt/battle-runtime/battle-runtime-mage-armor-selected-identity.mbt.qnt#step:doExpireMageArmorDuration` | `tasks/target-replay-evidence/T035-battle-runtime-mage-armor-selected-identity.json#T035-expire-mage-armor-duration#step:doExpireMageArmorDuration` | `src/tests/mod.rs::mage_armor_adapter_replays_all_branches` | `covered` |
| `cleanroom-input/qnt/battle-runtime/battle-runtime-mage-armor-selected-identity.mbt.qnt#step:doRejectMageArmorArmoredTarget` | `tasks/target-replay-evidence/T035-battle-runtime-mage-armor-selected-identity.json#T035-reject-mage-armor-armored-target#step:doRejectMageArmorArmoredTarget` | `src/tests/mod.rs::mage_armor_adapter_replays_all_branches` | `covered` |
| `cleanroom-input/qnt/battle-runtime/battle-runtime-mage-armor-selected-identity.mbt.qnt#step:doResolveMageArmorBaseArmorClassProjection` | `tasks/target-replay-evidence/T035-battle-runtime-mage-armor-selected-identity.json#T035-resolve-mage-armor-base-armor-class-projection#step:doResolveMageArmorBaseArmorClassProjection` | `src/tests/mod.rs::mage_armor_adapter_replays_all_branches` | `covered` |

Target replay evidence:

- Evidence file: `tasks/target-replay-evidence/T035-battle-runtime-mage-armor-selected-identity.json`
- Target profile: `rust`
- Target profile SHA-256: `6d4cc6c6a4769962798133d57aff01438fb2b661941f71d1aa8a3333f4b7ecc1`
- Quint binding: Rust quint-connect harness
- Reproduction seed or trace id: `T035-discover-mage-armor-unarmored-self-target`
- Reproduction seed or trace id: `T035-expire-mage-armor-duration`
- Reproduction seed or trace id: `T035-reject-mage-armor-armored-target`
- Reproduction seed or trace id: `T035-resolve-mage-armor-base-armor-class-projection`

Harness artifacts:

- Start gate: `tasks/START_GATE.json`
- Engine depth: `tasks/ENGINE_DEPTH_MANIFEST.json`
- State ownership: `tasks/STATE_OWNER_MANIFEST.json`
- Reviewer loop: `tasks/REVIEW_LOOP.json`
- Decider decision: `tasks/DECIDER_DECISION.json`

Diagnostic tests:

- `src/tests/mod.rs::mage_armor_projects_base_armor_class_and_duration`

Remaining gaps:

- `_none_`

Verification results:

- `cargo fmt --check` passed.
- `cargo test` passed.
- `cargo clippy --all-targets -- -D warnings` passed.
- `node scripts/check-cleanroom-harness.cjs` passed.

## T034: battle-runtime-level1-spatial-witness-selected-identity

- Manifest source commit SHA: `04249edf345a7752de2f1551dd3d509a2fffc160`
- Source branch inventory SHA: `b4e7e101def7969fc420563dc4da020c22e700f0dc0cc1d27accad6e8631225d`
- Driver: `cleanroom-input/qnt/battle-runtime/battle-runtime-level1-spatial-witness-selected-identity.mbt.qnt`
- Branch obligations:
  - `step:doDancingLightsMovableDimLight`
  - `step:doFaerieFireOutlineAdvantageInvisibleDimLight`
  - `step:doFeatherFallReactionMitigationLanding`
  - `step:doFogCloudAreaIdentityObscurementStrongWindCleanup`
  - `step:doGreaseCastGroundHazardSavingThrows`
  - `step:doGreaseMovementAndTurnTriggers`
  - `step:doJumpMovementReplacementLandingWitness`
  - `step:doLightObjectEmitterProjectionReplacementCleanup`
  - `step:doProduceFlameHeldLightProjectionHurlCleanup`
  - `step:doThunderwaveSavePushObjectsBoom`
- Allowed inputs used:
  - `cleanroom-input/MANIFEST.md`
  - `cleanroom-input/branch-coverage/source-branch-inventory.json`
  - `cleanroom-input/qnt/battle-runtime/battle-runtime-level1-spatial-witness-selected-identity.mbt.qnt`
  - `cleanroom-input/qnt/battle-runtime/battle-runtime-witness-protocol.qnt`
  - `cleanroom-input/qnt/battle-runtime/battle-runtime-light.qnt`
  - `cleanroom-input/qnt/battle-runtime/battle-runtime-feather-fall.qnt`
  - `cleanroom-input/qnt/battle-runtime/battle-runtime-jump-movement.qnt`
  - `cleanroom-input/qnt/battle-runtime/battle-runtime-area-trigger-timing.qnt`
  - `cleanroom-input/qnt/battle-runtime/battle-runtime-spell-bridge-examples.qnt`
  - `cleanroom-input/raw/srd-5.2.1/Spells/Descriptions-A-D.md`
  - `cleanroom-input/raw/srd-5.2.1/Spells/Descriptions-E-L.md`
  - `cleanroom-input/raw/srd-5.2.1/Spells/Descriptions-M-P.md`
  - `cleanroom-input/raw/srd-5.2.1/Spells/Descriptions-S-Z.md`
  - `cleanroom-input/domain/UBIQUITOUS_LANGUAGE.md`
  - `cleanroom-input/domain/CLEANROOM_ASSUMPTIONS.md`
  - `cleanroom-input/guidance/README.md`
  - `tasks/LEVEL_1_2_SCOPE.md`

Behavior implemented:

- Added `src/rules/level_one_spatial_spells.rs` with typed projections for level-one spatial spell witnesses covering light emitters, Faerie Fire outlines and Advantage, Feather Fall reaction mitigation and landing cleanup, Fog Cloud area identity and strong-wind cleanup, Grease cast and movement/end-turn triggers, Jump movement replacement and landing witness, Light object emitter replacement and cleanup, Produce Flame held light and hurl cleanup, and Thunderwave push/object/boom witness facts.
- Projected illumination, sight obscurement, attack roll mode, concentration, action availability, spell slot use, target identity, hazard lifecycle, and witness protocol fields from the QNT-selected branches and RAW spell text.
- Kept exact QNT action names, scenario labels, protocol labels, and target replay witness strings quarantined in `src/qnt_adapters/battle_runtime_level1_spatial_witness_selected_identity.rs`.

Generated branch coverage:

| Obligation | Target replay evidence | Diagnostic tests | Status |
| --- | --- | --- | --- |
| `cleanroom-input/qnt/battle-runtime/battle-runtime-level1-spatial-witness-selected-identity.mbt.qnt#step:doDancingLightsMovableDimLight` | `tasks/target-replay-evidence/T034-battle-runtime-level1-spatial-witness-selected-identity.json#T034-dancing-lights-movable-dim-light#step:doDancingLightsMovableDimLight` | `src/tests/mod.rs::level1_spatial_witness_adapter_replays_all_branches` | `covered` |
| `cleanroom-input/qnt/battle-runtime/battle-runtime-level1-spatial-witness-selected-identity.mbt.qnt#step:doFaerieFireOutlineAdvantageInvisibleDimLight` | `tasks/target-replay-evidence/T034-battle-runtime-level1-spatial-witness-selected-identity.json#T034-faerie-fire-outline-advantage-invisible-dim-light#step:doFaerieFireOutlineAdvantageInvisibleDimLight` | `src/tests/mod.rs::level1_spatial_witness_adapter_replays_all_branches` | `covered` |
| `cleanroom-input/qnt/battle-runtime/battle-runtime-level1-spatial-witness-selected-identity.mbt.qnt#step:doFeatherFallReactionMitigationLanding` | `tasks/target-replay-evidence/T034-battle-runtime-level1-spatial-witness-selected-identity.json#T034-feather-fall-reaction-mitigation-landing#step:doFeatherFallReactionMitigationLanding` | `src/tests/mod.rs::level1_spatial_witness_adapter_replays_all_branches` | `covered` |
| `cleanroom-input/qnt/battle-runtime/battle-runtime-level1-spatial-witness-selected-identity.mbt.qnt#step:doFogCloudAreaIdentityObscurementStrongWindCleanup` | `tasks/target-replay-evidence/T034-battle-runtime-level1-spatial-witness-selected-identity.json#T034-fog-cloud-area-identity-obscurement-strong-wind-cleanup#step:doFogCloudAreaIdentityObscurementStrongWindCleanup` | `src/tests/mod.rs::level1_spatial_witness_adapter_replays_all_branches` | `covered` |
| `cleanroom-input/qnt/battle-runtime/battle-runtime-level1-spatial-witness-selected-identity.mbt.qnt#step:doGreaseCastGroundHazardSavingThrows` | `tasks/target-replay-evidence/T034-battle-runtime-level1-spatial-witness-selected-identity.json#T034-grease-cast-ground-hazard-saving-throws#step:doGreaseCastGroundHazardSavingThrows` | `src/tests/mod.rs::level1_spatial_witness_adapter_replays_all_branches` | `covered` |
| `cleanroom-input/qnt/battle-runtime/battle-runtime-level1-spatial-witness-selected-identity.mbt.qnt#step:doGreaseMovementAndTurnTriggers` | `tasks/target-replay-evidence/T034-battle-runtime-level1-spatial-witness-selected-identity.json#T034-grease-movement-and-turn-triggers#step:doGreaseMovementAndTurnTriggers` | `src/tests/mod.rs::level1_spatial_witness_adapter_replays_all_branches` | `covered` |
| `cleanroom-input/qnt/battle-runtime/battle-runtime-level1-spatial-witness-selected-identity.mbt.qnt#step:doJumpMovementReplacementLandingWitness` | `tasks/target-replay-evidence/T034-battle-runtime-level1-spatial-witness-selected-identity.json#T034-jump-movement-replacement-landing-witness#step:doJumpMovementReplacementLandingWitness` | `src/tests/mod.rs::level1_spatial_witness_adapter_replays_all_branches` | `covered` |
| `cleanroom-input/qnt/battle-runtime/battle-runtime-level1-spatial-witness-selected-identity.mbt.qnt#step:doLightObjectEmitterProjectionReplacementCleanup` | `tasks/target-replay-evidence/T034-battle-runtime-level1-spatial-witness-selected-identity.json#T034-light-object-emitter-projection-replacement-cleanup#step:doLightObjectEmitterProjectionReplacementCleanup` | `src/tests/mod.rs::level1_spatial_witness_adapter_replays_all_branches` | `covered` |
| `cleanroom-input/qnt/battle-runtime/battle-runtime-level1-spatial-witness-selected-identity.mbt.qnt#step:doProduceFlameHeldLightProjectionHurlCleanup` | `tasks/target-replay-evidence/T034-battle-runtime-level1-spatial-witness-selected-identity.json#T034-produce-flame-held-light-projection-hurl-cleanup#step:doProduceFlameHeldLightProjectionHurlCleanup` | `src/tests/mod.rs::level1_spatial_witness_adapter_replays_all_branches` | `covered` |
| `cleanroom-input/qnt/battle-runtime/battle-runtime-level1-spatial-witness-selected-identity.mbt.qnt#step:doThunderwaveSavePushObjectsBoom` | `tasks/target-replay-evidence/T034-battle-runtime-level1-spatial-witness-selected-identity.json#T034-thunderwave-save-push-objects-boom#step:doThunderwaveSavePushObjectsBoom` | `src/tests/mod.rs::level1_spatial_witness_adapter_replays_all_branches` | `covered` |

Target replay evidence:

- Evidence file: `tasks/target-replay-evidence/T034-battle-runtime-level1-spatial-witness-selected-identity.json`
- Target profile: `rust`
- Target profile SHA-256: `6d4cc6c6a4769962798133d57aff01438fb2b661941f71d1aa8a3333f4b7ecc1`
- Quint binding: Rust quint-connect harness
- Reproduction seed or trace id: `T034-dancing-lights-movable-dim-light`
- Reproduction seed or trace id: `T034-faerie-fire-outline-advantage-invisible-dim-light`
- Reproduction seed or trace id: `T034-feather-fall-reaction-mitigation-landing`
- Reproduction seed or trace id: `T034-fog-cloud-area-identity-obscurement-strong-wind-cleanup`
- Reproduction seed or trace id: `T034-grease-cast-ground-hazard-saving-throws`
- Reproduction seed or trace id: `T034-grease-movement-and-turn-triggers`
- Reproduction seed or trace id: `T034-jump-movement-replacement-landing-witness`
- Reproduction seed or trace id: `T034-light-object-emitter-projection-replacement-cleanup`
- Reproduction seed or trace id: `T034-produce-flame-held-light-projection-hurl-cleanup`
- Reproduction seed or trace id: `T034-thunderwave-save-push-objects-boom`

Harness artifacts:

- Start gate: `tasks/START_GATE.json`
- Engine depth: `tasks/ENGINE_DEPTH_MANIFEST.json`
- State ownership: `tasks/STATE_OWNER_MANIFEST.json`
- Reviewer loop: `tasks/REVIEW_LOOP.json`
- Decider decision: `tasks/DECIDER_DECISION.json`

Diagnostic tests:

- `src/tests/mod.rs::level1_spatial_witness_projects_light_hazards_and_reactions`

Remaining gaps:

- `_none_`

Verification results:

- `cargo fmt --check` passed.
- `cargo test` passed.
- `cargo clippy --all-targets -- -D warnings` passed.
- `node scripts/check-cleanroom-harness.cjs` passed.

## T033: battle-runtime-level1-damage-spell-selected-identity

- Manifest source commit SHA: `04249edf345a7752de2f1551dd3d509a2fffc160`
- Source branch inventory SHA: `b4e7e101def7969fc420563dc4da020c22e700f0dc0cc1d27accad6e8631225d`
- Driver: `cleanroom-input/qnt/battle-runtime/battle-runtime-level1-damage-spell-selected-identity.mbt.qnt`
- Branch obligations:
  - `step:doResolveBurningHandsMixedConeSavingThrows`
  - `step:doResolveChromaticOrbDuplicateDamageLeap`
  - `step:doResolveIceKnifeHitAttackDamageAndBurstSavingThrows`
  - `step:doResolveIceKnifeMissBurstSavingThrows`
  - `step:doResolvePoisonSpraySpellAttackDamage`
  - `step:doResolveRayOfSicknessSpellAttackDamageAndPoisoned`
  - `step:doResolveSacredFlameDexteritySavingThrowRadiantDamage`
  - `step:doResolveSorcerousBurstSpellAttackDamage`
  - `step:doResolveStarryWispObjectSpellAttackDamageAndDimLight`
  - `step:doResolveViciousMockeryWisdomSavingThrowPsychicDamageAndNextAttackDisadvantage`
- Allowed inputs used:
  - `cleanroom-input/MANIFEST.md`
  - `cleanroom-input/branch-coverage/source-branch-inventory.json`
  - `cleanroom-input/qnt/battle-runtime/battle-runtime-level1-damage-spell-selected-identity.mbt.qnt`
  - `cleanroom-input/qnt/battle-runtime/battle-runtime-spell-attack.qnt`
  - `cleanroom-input/qnt/battle-runtime/battle-runtime-save-gated-spell.qnt`
  - `cleanroom-input/qnt/battle-runtime/battle-runtime-chained-spell-attack.qnt`
  - `cleanroom-input/qnt/battle-runtime/battle-runtime-spell-bridge-examples.qnt`
  - `cleanroom-input/qnt/battle-runtime/battle-runtime-sorcerous-burst-damage-choice.qnt`
  - `cleanroom-input/raw/srd-5.2.1/Spells/Descriptions-A-D.md`
  - `cleanroom-input/raw/srd-5.2.1/Spells/Descriptions-E-L.md`
  - `cleanroom-input/raw/srd-5.2.1/Spells/Descriptions-M-P.md`
  - `cleanroom-input/raw/srd-5.2.1/Spells/Descriptions-Q-R.md`
  - `cleanroom-input/raw/srd-5.2.1/Spells/Descriptions-S-Z.md`
  - `cleanroom-input/domain/UBIQUITOUS_LANGUAGE.md`
  - `cleanroom-input/domain/CLEANROOM_ASSUMPTIONS.md`
  - `cleanroom-input/guidance/README.md`
  - `tasks/LEVEL_1_2_SCOPE.md`

Behavior implemented:

- Added `src/rules/level_one_damage_spells.rs` with a typed projection state for level-one and cantrip damage spell outcomes, covering action availability, spell slot spend/remain, primary and secondary target Hit Points, Poisoned, next attack roll Disadvantage, scenario outcome, and witness protocol state.
- Implemented selected identity projections for Burning Hands, Chromatic Orb, Ice Knife hit and miss burst cases, Poison Spray, Ray of Sickness, Sacred Flame, Sorcerous Burst, Starry Wisp object targeting and Dim Light, and Vicious Mockery next-attack Disadvantage.
- Kept QNT action names, scenario labels, protocol labels, and target replay witness strings quarantined in `src/qnt_adapters/battle_runtime_level1_damage_spell_selected_identity.rs`.

Generated branch coverage:

| Obligation | Target replay evidence | Diagnostic tests | Status |
| --- | --- | --- | --- |
| `cleanroom-input/qnt/battle-runtime/battle-runtime-level1-damage-spell-selected-identity.mbt.qnt#step:doResolveBurningHandsMixedConeSavingThrows` | `tasks/target-replay-evidence/T033-battle-runtime-level1-damage-spell-selected-identity.json#T033-burning-hands-mixed-cone-saving-throws#step:doResolveBurningHandsMixedConeSavingThrows` | `src/tests/mod.rs::level1_damage_spell_adapter_replays_all_branches` | `covered` |
| `cleanroom-input/qnt/battle-runtime/battle-runtime-level1-damage-spell-selected-identity.mbt.qnt#step:doResolveChromaticOrbDuplicateDamageLeap` | `tasks/target-replay-evidence/T033-battle-runtime-level1-damage-spell-selected-identity.json#T033-chromatic-orb-duplicate-damage-leap#step:doResolveChromaticOrbDuplicateDamageLeap` | `src/tests/mod.rs::level1_damage_spell_adapter_replays_all_branches` | `covered` |
| `cleanroom-input/qnt/battle-runtime/battle-runtime-level1-damage-spell-selected-identity.mbt.qnt#step:doResolveIceKnifeHitAttackDamageAndBurstSavingThrows` | `tasks/target-replay-evidence/T033-battle-runtime-level1-damage-spell-selected-identity.json#T033-ice-knife-hit-attack-damage-burst-saving-throws#step:doResolveIceKnifeHitAttackDamageAndBurstSavingThrows` | `src/tests/mod.rs::level1_damage_spell_adapter_replays_all_branches` | `covered` |
| `cleanroom-input/qnt/battle-runtime/battle-runtime-level1-damage-spell-selected-identity.mbt.qnt#step:doResolveIceKnifeMissBurstSavingThrows` | `tasks/target-replay-evidence/T033-battle-runtime-level1-damage-spell-selected-identity.json#T033-ice-knife-miss-burst-saving-throws#step:doResolveIceKnifeMissBurstSavingThrows` | `src/tests/mod.rs::level1_damage_spell_adapter_replays_all_branches` | `covered` |
| `cleanroom-input/qnt/battle-runtime/battle-runtime-level1-damage-spell-selected-identity.mbt.qnt#step:doResolvePoisonSpraySpellAttackDamage` | `tasks/target-replay-evidence/T033-battle-runtime-level1-damage-spell-selected-identity.json#T033-poison-spray-spell-attack-damage#step:doResolvePoisonSpraySpellAttackDamage` | `src/tests/mod.rs::level1_damage_spell_adapter_replays_all_branches` | `covered` |
| `cleanroom-input/qnt/battle-runtime/battle-runtime-level1-damage-spell-selected-identity.mbt.qnt#step:doResolveRayOfSicknessSpellAttackDamageAndPoisoned` | `tasks/target-replay-evidence/T033-battle-runtime-level1-damage-spell-selected-identity.json#T033-ray-of-sickness-spell-attack-damage-poisoned#step:doResolveRayOfSicknessSpellAttackDamageAndPoisoned` | `src/tests/mod.rs::level1_damage_spell_adapter_replays_all_branches` | `covered` |
| `cleanroom-input/qnt/battle-runtime/battle-runtime-level1-damage-spell-selected-identity.mbt.qnt#step:doResolveSacredFlameDexteritySavingThrowRadiantDamage` | `tasks/target-replay-evidence/T033-battle-runtime-level1-damage-spell-selected-identity.json#T033-sacred-flame-dexterity-saving-throw-radiant-damage#step:doResolveSacredFlameDexteritySavingThrowRadiantDamage` | `src/tests/mod.rs::level1_damage_spell_adapter_replays_all_branches` | `covered` |
| `cleanroom-input/qnt/battle-runtime/battle-runtime-level1-damage-spell-selected-identity.mbt.qnt#step:doResolveSorcerousBurstSpellAttackDamage` | `tasks/target-replay-evidence/T033-battle-runtime-level1-damage-spell-selected-identity.json#T033-sorcerous-burst-spell-attack-damage#step:doResolveSorcerousBurstSpellAttackDamage` | `src/tests/mod.rs::level1_damage_spell_adapter_replays_all_branches` | `covered` |
| `cleanroom-input/qnt/battle-runtime/battle-runtime-level1-damage-spell-selected-identity.mbt.qnt#step:doResolveStarryWispObjectSpellAttackDamageAndDimLight` | `tasks/target-replay-evidence/T033-battle-runtime-level1-damage-spell-selected-identity.json#T033-starry-wisp-object-spell-attack-damage-dim-light#step:doResolveStarryWispObjectSpellAttackDamageAndDimLight` | `src/tests/mod.rs::level1_damage_spell_adapter_replays_all_branches` | `covered` |
| `cleanroom-input/qnt/battle-runtime/battle-runtime-level1-damage-spell-selected-identity.mbt.qnt#step:doResolveViciousMockeryWisdomSavingThrowPsychicDamageAndNextAttackDisadvantage` | `tasks/target-replay-evidence/T033-battle-runtime-level1-damage-spell-selected-identity.json#T033-vicious-mockery-wisdom-saving-throw-psychic-damage-next-attack-disadvantage#step:doResolveViciousMockeryWisdomSavingThrowPsychicDamageAndNextAttackDisadvantage` | `src/tests/mod.rs::level1_damage_spell_adapter_replays_all_branches` | `covered` |

Target replay evidence:

- Evidence file: `tasks/target-replay-evidence/T033-battle-runtime-level1-damage-spell-selected-identity.json`
- Target profile: `rust`
- Target profile SHA-256: `6d4cc6c6a4769962798133d57aff01438fb2b661941f71d1aa8a3333f4b7ecc1`
- Quint binding: Rust quint-connect harness
- Reproduction seed or trace id: `T033-burning-hands-mixed-cone-saving-throws`
- Reproduction seed or trace id: `T033-chromatic-orb-duplicate-damage-leap`
- Reproduction seed or trace id: `T033-ice-knife-hit-attack-damage-burst-saving-throws`
- Reproduction seed or trace id: `T033-ice-knife-miss-burst-saving-throws`
- Reproduction seed or trace id: `T033-poison-spray-spell-attack-damage`
- Reproduction seed or trace id: `T033-ray-of-sickness-spell-attack-damage-poisoned`
- Reproduction seed or trace id: `T033-sacred-flame-dexterity-saving-throw-radiant-damage`
- Reproduction seed or trace id: `T033-sorcerous-burst-spell-attack-damage`
- Reproduction seed or trace id: `T033-starry-wisp-object-spell-attack-damage-dim-light`
- Reproduction seed or trace id: `T033-vicious-mockery-wisdom-saving-throw-psychic-damage-next-attack-disadvantage`

Harness artifacts:

- Start gate: `tasks/START_GATE.json`
- Engine depth: `tasks/ENGINE_DEPTH_MANIFEST.json`
- State ownership: `tasks/STATE_OWNER_MANIFEST.json`
- Reviewer loop: `tasks/REVIEW_LOOP.json`
- Decider decision: `tasks/DECIDER_DECISION.json`

Diagnostic tests:

- `src/tests/mod.rs::level1_damage_spell_projects_selected_damage_cases`

Remaining gaps:

- `_none_`

Verification results:

- `cargo fmt --check` passed.
- `cargo test` passed.
- `cargo clippy --all-targets -- -D warnings` passed.
- `node scripts/check-cleanroom-harness.cjs` passed.

## T032: battle-runtime-level1-buff-mark-smite-selected-identity

- Manifest source commit SHA: `04249edf345a7752de2f1551dd3d509a2fffc160`
- Source branch inventory SHA: `b4e7e101def7969fc420563dc4da020c22e700f0dc0cc1d27accad6e8631225d`
- Driver: `cleanroom-input/qnt/battle-runtime/battle-runtime-level1-buff-mark-smite-selected-identity.mbt.qnt`
- Branch obligations:
  - `step:doDivineFavorWeaponDamageRider`
  - `step:doDivineSmiteAfterHitDamage`
  - `step:doEnsnaringStrikeAfterHitRestraintTurnStartDamageAndEscape`
  - `step:doFalseLifeTemporaryHitPoints`
  - `step:doHeroismFrightenedImmunityTurnStartTemporaryHitPoints`
  - `step:doHeroismFrightenedImmunityTurnStartTemporaryHitPointsCleanup`
  - `step:doHexMarkedDamageRiderAndLaterTurnTransfer`
  - `step:doHuntersMarkMarkedDamageRiderConcentrationAndSameTurnTransfer`
  - `step:doLongstriderSpeedIncrease`
  - `step:doSearingSmiteAfterHitTimedDamageAndSaveCleanup`
  - `step:doShillelaghWeaponAttackOverride`
  - `step:doTrueStrikeSpellHostedWeaponAttack`
- Allowed inputs used:
  - `cleanroom-input/MANIFEST.md`
  - `cleanroom-input/branch-coverage/source-branch-inventory.json`
  - `cleanroom-input/qnt/battle-runtime/battle-runtime-level1-buff-mark-smite-selected-identity.mbt.qnt`
  - `cleanroom-input/qnt/battle-runtime/battle-runtime-attack-facts.qnt`
  - `cleanroom-input/qnt/battle-runtime/battle-runtime-light.qnt`
  - `cleanroom-input/qnt/battle-runtime/battle-runtime-marked-riders.qnt`
  - `cleanroom-input/qnt/battle-runtime/battle-runtime-marked-spells.qnt`
  - `cleanroom-input/qnt/battle-runtime/battle-runtime-restoration-and-buffs.qnt`
  - `cleanroom-input/qnt/shared-algebras/proofs/rule-core/spell-damage-rider-projection-core.qnt`
  - `cleanroom-input/qnt/shared-algebras/proofs/rule-core/spell-scalar-buff-projection-core.qnt`
  - `cleanroom-input/raw/srd-5.2.1/Spells/Descriptions-A-D.md`
  - `cleanroom-input/raw/srd-5.2.1/Spells/Descriptions-E-L.md`
  - `cleanroom-input/raw/srd-5.2.1/Spells/Descriptions-S-Z.md`
  - `cleanroom-input/raw/srd-5.2.1/Playing-the-Game.md`
  - `cleanroom-input/raw/srd-5.2.1/Rules-Glossary.md`
  - `cleanroom-input/domain/UBIQUITOUS_LANGUAGE.md`
  - `cleanroom-input/guidance/README.md`
  - `tasks/LEVEL_1_2_SCOPE.md`

Behavior implemented:

- Added `src/rules/level_one_spell_effects.rs` with typed component projections for level-1 damage riders, temporary HP, speed effects, frightened immunity, restrained escape lifecycles, marked damage transfer, timed smite damage/save cleanup, Shillelagh weapon attack overrides, and True Strike spell-hosted weapon attacks.
- Extended the shared `DamageType` vocabulary with `Piercing` for Ensnaring Strike turn-start damage.
- Implemented the T032 driver projections for Divine Favor, Divine Smite, Ensnaring Strike, False Life, Heroism and cleanup, Hunter's Mark, Hex, Longstrider, Searing Smite, Shillelagh, and True Strike.
- Kept QNT action names, spell string ids, weapon ids, scenario labels, and witness protocol labels quarantined in `src/qnt_adapters/battle_runtime_level1_buff_mark_smite_selected_identity.rs`.

Generated branch coverage:

| Obligation | Target replay evidence | Diagnostic tests | Status |
| --- | --- | --- | --- |
| `cleanroom-input/qnt/battle-runtime/battle-runtime-level1-buff-mark-smite-selected-identity.mbt.qnt#step:doDivineFavorWeaponDamageRider` | `tasks/target-replay-evidence/T032-battle-runtime-level1-buff-mark-smite-selected-identity.json#T032-divine-favor-weapon-damage-rider#step:doDivineFavorWeaponDamageRider` | `src/tests/mod.rs::level1_buff_mark_smite_adapter_replays_all_branches` | `covered` |
| `cleanroom-input/qnt/battle-runtime/battle-runtime-level1-buff-mark-smite-selected-identity.mbt.qnt#step:doDivineSmiteAfterHitDamage` | `tasks/target-replay-evidence/T032-battle-runtime-level1-buff-mark-smite-selected-identity.json#T032-divine-smite-after-hit-damage#step:doDivineSmiteAfterHitDamage` | `src/tests/mod.rs::level1_buff_mark_smite_adapter_replays_all_branches` | `covered` |
| `cleanroom-input/qnt/battle-runtime/battle-runtime-level1-buff-mark-smite-selected-identity.mbt.qnt#step:doEnsnaringStrikeAfterHitRestraintTurnStartDamageAndEscape` | `tasks/target-replay-evidence/T032-battle-runtime-level1-buff-mark-smite-selected-identity.json#T032-ensnaring-strike-restraint-turn-start-damage-escape#step:doEnsnaringStrikeAfterHitRestraintTurnStartDamageAndEscape` | `src/tests/mod.rs::level1_buff_mark_smite_adapter_replays_all_branches` | `covered` |
| `cleanroom-input/qnt/battle-runtime/battle-runtime-level1-buff-mark-smite-selected-identity.mbt.qnt#step:doFalseLifeTemporaryHitPoints` | `tasks/target-replay-evidence/T032-battle-runtime-level1-buff-mark-smite-selected-identity.json#T032-false-life-temporary-hit-points#step:doFalseLifeTemporaryHitPoints` | `src/tests/mod.rs::level1_buff_mark_smite_adapter_replays_all_branches` | `covered` |
| `cleanroom-input/qnt/battle-runtime/battle-runtime-level1-buff-mark-smite-selected-identity.mbt.qnt#step:doHeroismFrightenedImmunityTurnStartTemporaryHitPoints` | `tasks/target-replay-evidence/T032-battle-runtime-level1-buff-mark-smite-selected-identity.json#T032-heroism-frightened-immunity-turn-start-temporary-hit-points#step:doHeroismFrightenedImmunityTurnStartTemporaryHitPoints` | `src/tests/mod.rs::level1_buff_mark_smite_adapter_replays_all_branches` | `covered` |
| `cleanroom-input/qnt/battle-runtime/battle-runtime-level1-buff-mark-smite-selected-identity.mbt.qnt#step:doHeroismFrightenedImmunityTurnStartTemporaryHitPointsCleanup` | `tasks/target-replay-evidence/T032-battle-runtime-level1-buff-mark-smite-selected-identity.json#T032-heroism-frightened-immunity-turn-start-temporary-hit-points-cleanup#step:doHeroismFrightenedImmunityTurnStartTemporaryHitPointsCleanup` | `src/tests/mod.rs::level1_buff_mark_smite_adapter_replays_all_branches` | `covered` |
| `cleanroom-input/qnt/battle-runtime/battle-runtime-level1-buff-mark-smite-selected-identity.mbt.qnt#step:doHexMarkedDamageRiderAndLaterTurnTransfer` | `tasks/target-replay-evidence/T032-battle-runtime-level1-buff-mark-smite-selected-identity.json#T032-hex-damage-rider-later-turn-transfer#step:doHexMarkedDamageRiderAndLaterTurnTransfer` | `src/tests/mod.rs::level1_buff_mark_smite_adapter_replays_all_branches` | `covered` |
| `cleanroom-input/qnt/battle-runtime/battle-runtime-level1-buff-mark-smite-selected-identity.mbt.qnt#step:doHuntersMarkMarkedDamageRiderConcentrationAndSameTurnTransfer` | `tasks/target-replay-evidence/T032-battle-runtime-level1-buff-mark-smite-selected-identity.json#T032-hunters-mark-damage-rider-concentration-same-turn-transfer#step:doHuntersMarkMarkedDamageRiderConcentrationAndSameTurnTransfer` | `src/tests/mod.rs::level1_buff_mark_smite_adapter_replays_all_branches` | `covered` |
| `cleanroom-input/qnt/battle-runtime/battle-runtime-level1-buff-mark-smite-selected-identity.mbt.qnt#step:doLongstriderSpeedIncrease` | `tasks/target-replay-evidence/T032-battle-runtime-level1-buff-mark-smite-selected-identity.json#T032-longstrider-speed-increase#step:doLongstriderSpeedIncrease` | `src/tests/mod.rs::level1_buff_mark_smite_adapter_replays_all_branches` | `covered` |
| `cleanroom-input/qnt/battle-runtime/battle-runtime-level1-buff-mark-smite-selected-identity.mbt.qnt#step:doSearingSmiteAfterHitTimedDamageAndSaveCleanup` | `tasks/target-replay-evidence/T032-battle-runtime-level1-buff-mark-smite-selected-identity.json#T032-searing-smite-timed-damage-save-cleanup#step:doSearingSmiteAfterHitTimedDamageAndSaveCleanup` | `src/tests/mod.rs::level1_buff_mark_smite_adapter_replays_all_branches` | `covered` |
| `cleanroom-input/qnt/battle-runtime/battle-runtime-level1-buff-mark-smite-selected-identity.mbt.qnt#step:doShillelaghWeaponAttackOverride` | `tasks/target-replay-evidence/T032-battle-runtime-level1-buff-mark-smite-selected-identity.json#T032-shillelagh-weapon-attack-override#step:doShillelaghWeaponAttackOverride` | `src/tests/mod.rs::level1_buff_mark_smite_adapter_replays_all_branches` | `covered` |
| `cleanroom-input/qnt/battle-runtime/battle-runtime-level1-buff-mark-smite-selected-identity.mbt.qnt#step:doTrueStrikeSpellHostedWeaponAttack` | `tasks/target-replay-evidence/T032-battle-runtime-level1-buff-mark-smite-selected-identity.json#T032-true-strike-spell-hosted-weapon-attack#step:doTrueStrikeSpellHostedWeaponAttack` | `src/tests/mod.rs::level1_buff_mark_smite_adapter_replays_all_branches` | `covered` |

Target replay evidence:

- Evidence file: `tasks/target-replay-evidence/T032-battle-runtime-level1-buff-mark-smite-selected-identity.json`
- Target profile: `rust`
- Target profile SHA-256: `6d4cc6c6a4769962798133d57aff01438fb2b661941f71d1aa8a3333f4b7ecc1`
- Quint binding: Rust quint-connect harness
- Reproduction seed or trace id: `T032-divine-favor-weapon-damage-rider`
- Reproduction seed or trace id: `T032-divine-smite-after-hit-damage`
- Reproduction seed or trace id: `T032-ensnaring-strike-restraint-turn-start-damage-escape`
- Reproduction seed or trace id: `T032-false-life-temporary-hit-points`
- Reproduction seed or trace id: `T032-heroism-frightened-immunity-turn-start-temporary-hit-points`
- Reproduction seed or trace id: `T032-heroism-frightened-immunity-turn-start-temporary-hit-points-cleanup`
- Reproduction seed or trace id: `T032-hex-damage-rider-later-turn-transfer`
- Reproduction seed or trace id: `T032-hunters-mark-damage-rider-concentration-same-turn-transfer`
- Reproduction seed or trace id: `T032-longstrider-speed-increase`
- Reproduction seed or trace id: `T032-searing-smite-timed-damage-save-cleanup`
- Reproduction seed or trace id: `T032-shillelagh-weapon-attack-override`
- Reproduction seed or trace id: `T032-true-strike-spell-hosted-weapon-attack`

Harness artifacts:

- Start gate: `tasks/START_GATE.json`
- Engine depth: `tasks/ENGINE_DEPTH_MANIFEST.json`
- State ownership: `tasks/STATE_OWNER_MANIFEST.json`
- Reviewer loop: `tasks/REVIEW_LOOP.json`
- Decider decision: `tasks/DECIDER_DECISION.json`

Diagnostic tests:

- `src/tests/mod.rs::level1_buff_mark_smite_projects_core_spell_effects`

Remaining gaps:

- `_none_`

Verification results:

- `cargo fmt --check` passed.
- `cargo test` passed.
- `cargo clippy --all-targets -- -D warnings` passed.
- `node scripts/check-cleanroom-harness.cjs` passed.

## T031: battle-runtime-interrupt-stack-resume

- Manifest source commit SHA: `04249edf345a7752de2f1551dd3d509a2fffc160`
- Source branch inventory SHA: `b4e7e101def7969fc420563dc4da020c22e700f0dc0cc1d27accad6e8631225d`
- Driver: `cleanroom-input/qnt/battle-runtime/battle-runtime-interrupt-stack-resume.mbt.qnt`
- Branch obligations:
  - `step:doNestedDeclineResumesOuterInterrupt`
  - `step:doReplayRecordedProcedureFromRoot`
  - `step:doShieldMutationResumesInterruptedAttack`
- Allowed inputs used:
  - `cleanroom-input/MANIFEST.md`
  - `cleanroom-input/branch-coverage/source-branch-inventory.json`
  - `cleanroom-input/qnt/battle-runtime/battle-runtime-interrupt-stack-resume.mbt.qnt`
  - `cleanroom-input/qnt/battle-runtime/battle-runtime-replay-equivalence.qnt`
  - `cleanroom-input/qnt/battle-runtime/battle-runtime-witness-protocol.qnt`
  - `cleanroom-input/qnt/shared-algebras/proofs/rule-core/reactions-continuations-concentration.qnt`
  - `cleanroom-input/raw/srd-5.2.1/Playing-the-Game.md`
  - `cleanroom-input/raw/srd-5.2.1/Rules-Glossary.md`
  - `cleanroom-input/domain/UBIQUITOUS_LANGUAGE.md`
  - `cleanroom-input/domain/CLEANROOM_ASSUMPTIONS.md`
  - `cleanroom-input/guidance/README.md`
  - `tasks/LEVEL_1_2_SCOPE.md`

Behavior implemented:

- Added `src/rules/interrupt_stack_resume.rs` with reusable typed reaction-window offers, bounded nesting depth, suspended-window restoration, decline handling, matching Reaction spend-and-advance handling, and interrupt-resume projection state.
- Nested interrupt decline now observes depth 2, declines the inner window without spending the Reaction quota, restores the outer interruption at depth 1, resumes the rolled-dice hole, and preserves the responder's Reaction availability.
- Matching Reaction resolution now spends the responder's Reaction quota, advances back to the interrupted attack continuation, records the active-effect mutation on resume, and leaves target HP unchanged.
- Replay-from-root projection now compares the independent recorded weapon attack resolution with root plus continuation plus suffix fills, matching the QNT replay-equivalence shape.
- Kept QNT action names, string projection constants, scenario labels, and witness protocol labels quarantined in `src/qnt_adapters/battle_runtime_interrupt_stack_resume.rs`.

Generated branch coverage:

| Obligation | Target replay evidence | Diagnostic tests | Status |
| --- | --- | --- | --- |
| `cleanroom-input/qnt/battle-runtime/battle-runtime-interrupt-stack-resume.mbt.qnt#step:doNestedDeclineResumesOuterInterrupt` | `tasks/target-replay-evidence/T031-battle-runtime-interrupt-stack-resume.json#T031-nested-decline-resumes-outer-interrupt#step:doNestedDeclineResumesOuterInterrupt` | `src/tests/mod.rs::interrupt_stack_resume_adapter_replays_all_branches` | `covered` |
| `cleanroom-input/qnt/battle-runtime/battle-runtime-interrupt-stack-resume.mbt.qnt#step:doReplayRecordedProcedureFromRoot` | `tasks/target-replay-evidence/T031-battle-runtime-interrupt-stack-resume.json#T031-replay-recorded-procedure-from-root#step:doReplayRecordedProcedureFromRoot` | `src/tests/mod.rs::interrupt_stack_resume_adapter_replays_all_branches` | `covered` |
| `cleanroom-input/qnt/battle-runtime/battle-runtime-interrupt-stack-resume.mbt.qnt#step:doShieldMutationResumesInterruptedAttack` | `tasks/target-replay-evidence/T031-battle-runtime-interrupt-stack-resume.json#T031-shield-mutation-resumes-interrupted-attack#step:doShieldMutationResumesInterruptedAttack` | `src/tests/mod.rs::interrupt_stack_resume_adapter_replays_all_branches` | `covered` |

Target replay evidence:

- Evidence file: `tasks/target-replay-evidence/T031-battle-runtime-interrupt-stack-resume.json`
- Target profile: `rust`
- Target profile SHA-256: `6d4cc6c6a4769962798133d57aff01438fb2b661941f71d1aa8a3333f4b7ecc1`
- Quint binding: Rust quint-connect harness
- Reproduction seed or trace id: `T031-nested-decline-resumes-outer-interrupt`
- Reproduction seed or trace id: `T031-replay-recorded-procedure-from-root`
- Reproduction seed or trace id: `T031-shield-mutation-resumes-interrupted-attack`

Harness artifacts:

- Start gate: `tasks/START_GATE.json`
- Engine depth: `tasks/ENGINE_DEPTH_MANIFEST.json`
- State ownership: `tasks/STATE_OWNER_MANIFEST.json`
- Reviewer loop: `tasks/REVIEW_LOOP.json`
- Decider decision: `tasks/DECIDER_DECISION.json`

Diagnostic tests:

- `src/tests/mod.rs::interrupt_stack_resume_declines_spends_and_replays_from_root`

Remaining gaps:

- `_none_`

Verification results:

- `cargo fmt --check` passed.
- `cargo test` passed.
- `cargo clippy --all-targets -- -D warnings` passed.
- `node scripts/check-cleanroom-harness.cjs` passed.

## T030: battle-runtime-hit-point-restoration-ordering

- Manifest source commit SHA: `04249edf345a7752de2f1551dd3d509a2fffc160`
- Source branch inventory SHA: `b4e7e101def7969fc420563dc4da020c22e700f0dc0cc1d27accad6e8631225d`
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
  - `cleanroom-input/qnt/battle-runtime/battle-runtime-hit-point-restoration-ordering.mbt.qnt`
  - `cleanroom-input/qnt/battle-runtime/battle-runtime-hit-point-restoration-ordering.qnt`
  - `cleanroom-input/qnt/shared-algebras/proofs/rule-core/spell-hit-point-restoration-core.qnt`
  - `cleanroom-input/qnt/shared-algebras/proofs/rule-core/hit-point-recovery.qnt`
  - `cleanroom-input/raw/srd-5.2.1/Playing-the-Game.md`
  - `cleanroom-input/domain/UBIQUITOUS_LANGUAGE.md`
  - `tasks/LEVEL_1_2_SCOPE.md`

Behavior implemented:

- Added `src/rules/hit_point_restoration_ordering.rs` with reusable frontier stages, hole kinds, fill kinds, fill-order results, runtime protocol projection, and healing-from-zero projection.
- Single-target spell healing opens a target-choice frontier before the healing roll; target-list spell healing opens a target-list frontier before the healing roll.
- Submitting a healing roll before either target frontier records `HealingTargetRequired` and keeps the current target frontier open.
- Filling the spell target choice or spell target list advances to the healing-roll frontier; filling the healing roll resolves and applies 5 HP from zero, clearing the zero-HP lifecycle.
- Feature healing opens a pool-distribution frontier; filling the distribution resolves and applies 8 HP from zero, clearing the zero-HP lifecycle.
- Kept QNT action names, stage labels, hole labels, ordering-error strings, and witness protocol labels quarantined in `src/qnt_adapters/battle_runtime_hit_point_restoration_ordering.rs`.

Generated branch coverage:

| Obligation | Target replay evidence | Diagnostic tests | Status |
| --- | --- | --- | --- |
| `cleanroom-input/qnt/battle-runtime/battle-runtime-hit-point-restoration-ordering.mbt.qnt#step:doDiscoverFeatureHealingPool` | `tasks/target-replay-evidence/T030-battle-runtime-hit-point-restoration-ordering.json#T030-discover-feature-healing-pool#step:doDiscoverFeatureHealingPool` | `src/tests/mod.rs::hit_point_restoration_ordering_adapter_replays_all_branches` | `covered` |
| `cleanroom-input/qnt/battle-runtime/battle-runtime-hit-point-restoration-ordering.mbt.qnt#step:doDiscoverSingleTargetSpellHealing` | `tasks/target-replay-evidence/T030-battle-runtime-hit-point-restoration-ordering.json#T030-discover-single-target-spell-healing#step:doDiscoverSingleTargetSpellHealing` | `src/tests/mod.rs::hit_point_restoration_ordering_adapter_replays_all_branches` | `covered` |
| `cleanroom-input/qnt/battle-runtime/battle-runtime-hit-point-restoration-ordering.mbt.qnt#step:doDiscoverTargetListSpellHealing` | `tasks/target-replay-evidence/T030-battle-runtime-hit-point-restoration-ordering.json#T030-discover-target-list-spell-healing#step:doDiscoverTargetListSpellHealing` | `src/tests/mod.rs::hit_point_restoration_ordering_adapter_replays_all_branches` | `covered` |
| `cleanroom-input/qnt/battle-runtime/battle-runtime-hit-point-restoration-ordering.mbt.qnt#step:doFillFeatureHealingDistribution` | `tasks/target-replay-evidence/T030-battle-runtime-hit-point-restoration-ordering.json#T030-fill-feature-healing-distribution#step:doFillFeatureHealingDistribution` | `src/tests/mod.rs::hit_point_restoration_ordering_adapter_replays_all_branches` | `covered` |
| `cleanroom-input/qnt/battle-runtime/battle-runtime-hit-point-restoration-ordering.mbt.qnt#step:doFillSpellHealingRoll` | `tasks/target-replay-evidence/T030-battle-runtime-hit-point-restoration-ordering.json#T030-fill-spell-healing-roll#step:doFillSpellHealingRoll` | `src/tests/mod.rs::hit_point_restoration_ordering_adapter_replays_all_branches` | `covered` |
| `cleanroom-input/qnt/battle-runtime/battle-runtime-hit-point-restoration-ordering.mbt.qnt#step:doFillSpellHealingTargetChoice` | `tasks/target-replay-evidence/T030-battle-runtime-hit-point-restoration-ordering.json#T030-fill-spell-healing-target-choice#step:doFillSpellHealingTargetChoice` | `src/tests/mod.rs::hit_point_restoration_ordering_adapter_replays_all_branches` | `covered` |
| `cleanroom-input/qnt/battle-runtime/battle-runtime-hit-point-restoration-ordering.mbt.qnt#step:doFillSpellHealingTargetList` | `tasks/target-replay-evidence/T030-battle-runtime-hit-point-restoration-ordering.json#T030-fill-spell-healing-target-list#step:doFillSpellHealingTargetList` | `src/tests/mod.rs::hit_point_restoration_ordering_adapter_replays_all_branches` | `covered` |
| `cleanroom-input/qnt/battle-runtime/battle-runtime-hit-point-restoration-ordering.mbt.qnt#step:doSubmitHealingRollBeforeTargetChoice` | `tasks/target-replay-evidence/T030-battle-runtime-hit-point-restoration-ordering.json#T030-submit-healing-roll-before-target-choice#step:doSubmitHealingRollBeforeTargetChoice` | `src/tests/mod.rs::hit_point_restoration_ordering_adapter_replays_all_branches` | `covered` |
| `cleanroom-input/qnt/battle-runtime/battle-runtime-hit-point-restoration-ordering.mbt.qnt#step:doSubmitHealingRollBeforeTargetList` | `tasks/target-replay-evidence/T030-battle-runtime-hit-point-restoration-ordering.json#T030-submit-healing-roll-before-target-list#step:doSubmitHealingRollBeforeTargetList` | `src/tests/mod.rs::hit_point_restoration_ordering_adapter_replays_all_branches` | `covered` |

Target replay evidence:

- Evidence file: `tasks/target-replay-evidence/T030-battle-runtime-hit-point-restoration-ordering.json`
- Target profile: `rust`
- Target profile SHA-256: `6d4cc6c6a4769962798133d57aff01438fb2b661941f71d1aa8a3333f4b7ecc1`
- Quint binding: Rust quint-connect harness
- Reproduction seed or trace id: `T030-discover-feature-healing-pool`
- Reproduction seed or trace id: `T030-discover-single-target-spell-healing`
- Reproduction seed or trace id: `T030-discover-target-list-spell-healing`
- Reproduction seed or trace id: `T030-fill-feature-healing-distribution`
- Reproduction seed or trace id: `T030-fill-spell-healing-roll`
- Reproduction seed or trace id: `T030-fill-spell-healing-target-choice`
- Reproduction seed or trace id: `T030-fill-spell-healing-target-list`
- Reproduction seed or trace id: `T030-submit-healing-roll-before-target-choice`
- Reproduction seed or trace id: `T030-submit-healing-roll-before-target-list`

Harness artifacts:

- Start gate: `tasks/START_GATE.json`
- Engine depth: `tasks/ENGINE_DEPTH_MANIFEST.json`
- State ownership: `tasks/STATE_OWNER_MANIFEST.json`
- Reviewer loop: `tasks/REVIEW_LOOP.json`
- Decider decision: `tasks/DECIDER_DECISION.json`

Diagnostic tests:

- `src/tests/mod.rs::hit_point_restoration_ordering_requires_target_before_roll_and_applies_healing`

Remaining gaps:

- `_none_`

Verification results:

- `cargo fmt --check` passed.
- `cargo test` passed.
- `cargo clippy --all-targets -- -D warnings` passed.
- `node scripts/check-cleanroom-harness.cjs` passed.

## T029: battle-runtime-healing-stabilization-selected-identity

- Manifest source commit SHA: `04249edf345a7752de2f1551dd3d509a2fffc160`
- Source branch inventory SHA: `b4e7e101def7969fc420563dc4da020c22e700f0dc0cc1d27accad6e8631225d`
- Driver: `cleanroom-input/qnt/battle-runtime/battle-runtime-healing-stabilization-selected-identity.mbt.qnt`
- Branch obligations:
  - `step:doResolveSpareTheDyingStable`
- Allowed inputs used:
  - `cleanroom-input/MANIFEST.md`
  - `cleanroom-input/branch-coverage/source-branch-inventory.json`
  - `cleanroom-input/qnt/battle-runtime/battle-runtime-healing-stabilization-selected-identity.mbt.qnt`
  - `cleanroom-input/raw/srd-5.2.1/Spells/Descriptions-S-Z.md`
  - `cleanroom-input/raw/srd-5.2.1/Playing-the-Game.md`
  - `cleanroom-input/raw/srd-5.2.1/Rules-Glossary.md`
  - `cleanroom-input/domain/UBIQUITOUS_LANGUAGE.md`
  - `tasks/LEVEL_1_2_SCOPE.md`

Behavior implemented:

- Added `HealingStabilizationState` in `src/rules/hit_points.rs` as a small action/protocol wrapper around the existing death-saving target state.
- Initialized the selected scenario with the target at 0 HP, Unconscious, not Stable, two death-save successes, one death-save failure, and an available Action.
- Resolved Spare the Dying by spending the Action, keeping HP at 0, making the target Stable, preserving Unconscious, and resetting death-save successes and failures to zero.
- Kept the QNT action name, scenario labels, and witness protocol labels quarantined in `src/qnt_adapters/battle_runtime_healing_stabilization_selected_identity.rs`.

Generated branch coverage:

| Obligation | Target replay evidence | Diagnostic tests | Status |
| --- | --- | --- | --- |
| `cleanroom-input/qnt/battle-runtime/battle-runtime-healing-stabilization-selected-identity.mbt.qnt#step:doResolveSpareTheDyingStable` | `tasks/target-replay-evidence/T029-battle-runtime-healing-stabilization-selected-identity.json#T029-resolve-spare-the-dying-stable#step:doResolveSpareTheDyingStable` | `src/tests/mod.rs::healing_stabilization_adapter_replays_spare_the_dying_branch` | `covered` |

Target replay evidence:

- Evidence file: `tasks/target-replay-evidence/T029-battle-runtime-healing-stabilization-selected-identity.json`
- Target profile: `rust`
- Target profile SHA-256: `6d4cc6c6a4769962798133d57aff01438fb2b661941f71d1aa8a3333f4b7ecc1`
- Quint binding: Rust quint-connect harness
- Reproduction seed or trace id: `T029-resolve-spare-the-dying-stable`

Harness artifacts:

- Start gate: `tasks/START_GATE.json`
- Engine depth: `tasks/ENGINE_DEPTH_MANIFEST.json`
- State ownership: `tasks/STATE_OWNER_MANIFEST.json`
- Reviewer loop: `tasks/REVIEW_LOOP.json`
- Decider decision: `tasks/DECIDER_DECISION.json`

Diagnostic tests:

- `src/tests/mod.rs::spare_the_dying_stabilizes_without_healing_or_waking`

Remaining gaps:

- `_none_`

Verification results:

- `cargo fmt --check` passed.
- `cargo test` passed.
- `cargo clippy --all-targets -- -D warnings` passed.
- `node scripts/check-cleanroom-harness.cjs` passed.

## T028: battle-runtime-find-familiar-selected-identity

- Manifest source commit SHA: `04249edf345a7752de2f1551dd3d509a2fffc160`
- Source branch inventory SHA: `b4e7e101def7969fc420563dc4da020c22e700f0dc0cc1d27accad6e8631225d`
- Driver: `cleanroom-input/qnt/battle-runtime/battle-runtime-find-familiar-selected-identity.mbt.qnt`
- Branch obligations:
  - `step:doCastFindFamiliar`
  - `step:doDeliverTouchSpellThroughFindFamiliar`
  - `step:doDismissAndReappearFindFamiliar`
  - `step:doRecastFindFamiliarReplacement`
- Allowed inputs used:
  - `cleanroom-input/MANIFEST.md`
  - `cleanroom-input/branch-coverage/source-branch-inventory.json`
  - `cleanroom-input/qnt/battle-runtime/battle-runtime-find-familiar-selected-identity.mbt.qnt`
  - `cleanroom-input/qnt/battle-runtime/battle-runtime-find-familiar.qnt`
  - `cleanroom-input/qnt/battle-runtime/battle-runtime-model.qnt`
  - `cleanroom-input/raw/srd-5.2.1/Spells/Descriptions-E-L.md`
  - `cleanroom-input/domain/UBIQUITOUS_LANGUAGE.md`
  - `cleanroom-input/domain/CLEANROOM_ASSUMPTIONS.md`
  - `tasks/LEVEL_1_2_SCOPE.md`

Behavior implemented:

- Extended `src/rules/find_familiar.rs` with owner Magic Action availability, a reusable recast replacement helper, and a dismiss-and-reappear helper for Find Familiar lifecycle projections.
- Cast Find Familiar creates a present primary cat familiar, keeps a single combatant, leaves the familiar Reaction available, and preserves the target HP projected by the scenario.
- Recast replacement uses the one-familiar rule to adopt the rat form without creating a replacement combatant.
- Dismiss-and-reappear spends the owner Magic Action while returning the same present cat familiar with its Reaction available.
- Touch delivery spends the owner action, spends the familiar Reaction, commits the touch spell slot, and keeps the target HP projected by the QNT driver.
- Kept QNT action names, scenario labels, familiar status/form strings, and witness protocol labels quarantined in `src/qnt_adapters/battle_runtime_find_familiar_selected_identity.rs`.

Generated branch coverage:

| Obligation | Target replay evidence | Diagnostic tests | Status |
| --- | --- | --- | --- |
| `cleanroom-input/qnt/battle-runtime/battle-runtime-find-familiar-selected-identity.mbt.qnt#step:doCastFindFamiliar` | `tasks/target-replay-evidence/T028-battle-runtime-find-familiar-selected-identity.json#T028-cast-find-familiar#step:doCastFindFamiliar` | `src/tests/mod.rs::find_familiar_selected_identity_adapter_replays_all_branches` | `covered` |
| `cleanroom-input/qnt/battle-runtime/battle-runtime-find-familiar-selected-identity.mbt.qnt#step:doDeliverTouchSpellThroughFindFamiliar` | `tasks/target-replay-evidence/T028-battle-runtime-find-familiar-selected-identity.json#T028-deliver-touch-spell-through-find-familiar#step:doDeliverTouchSpellThroughFindFamiliar` | `src/tests/mod.rs::find_familiar_selected_identity_adapter_replays_all_branches` | `covered` |
| `cleanroom-input/qnt/battle-runtime/battle-runtime-find-familiar-selected-identity.mbt.qnt#step:doDismissAndReappearFindFamiliar` | `tasks/target-replay-evidence/T028-battle-runtime-find-familiar-selected-identity.json#T028-dismiss-and-reappear-find-familiar#step:doDismissAndReappearFindFamiliar` | `src/tests/mod.rs::find_familiar_selected_identity_adapter_replays_all_branches` | `covered` |
| `cleanroom-input/qnt/battle-runtime/battle-runtime-find-familiar-selected-identity.mbt.qnt#step:doRecastFindFamiliarReplacement` | `tasks/target-replay-evidence/T028-battle-runtime-find-familiar-selected-identity.json#T028-recast-find-familiar-replacement#step:doRecastFindFamiliarReplacement` | `src/tests/mod.rs::find_familiar_selected_identity_adapter_replays_all_branches` | `covered` |

Target replay evidence:

- Evidence file: `tasks/target-replay-evidence/T028-battle-runtime-find-familiar-selected-identity.json`
- Target profile: `rust`
- Target profile SHA-256: `6d4cc6c6a4769962798133d57aff01438fb2b661941f71d1aa8a3333f4b7ecc1`
- Quint binding: Rust quint-connect harness
- Reproduction seed or trace id: `T028-cast-find-familiar`
- Reproduction seed or trace id: `T028-deliver-touch-spell-through-find-familiar`
- Reproduction seed or trace id: `T028-dismiss-and-reappear-find-familiar`
- Reproduction seed or trace id: `T028-recast-find-familiar-replacement`

Harness artifacts:

- Start gate: `tasks/START_GATE.json`
- Engine depth: `tasks/ENGINE_DEPTH_MANIFEST.json`
- State ownership: `tasks/STATE_OWNER_MANIFEST.json`
- Reviewer loop: `tasks/REVIEW_LOOP.json`
- Decider decision: `tasks/DECIDER_DECISION.json`

Diagnostic tests:

- `src/tests/mod.rs::find_familiar_selected_identity_recasts_reappears_and_delivers_touch`

Remaining gaps:

- `_none_`

Verification results:

- `cargo fmt --check` passed.
- `cargo test` passed.
- `cargo clippy --all-targets -- -D warnings` passed.
- `node scripts/check-cleanroom-harness.cjs` passed.

## T027: battle-runtime-find-familiar-companion-lifecycle

- Manifest source commit SHA: `04249edf345a7752de2f1551dd3d509a2fffc160`
- Source branch inventory SHA: `b4e7e101def7969fc420563dc4da020c22e700f0dc0cc1d27accad6e8631225d`
- Driver: `cleanroom-input/qnt/battle-runtime/battle-runtime-find-familiar-companion-lifecycle.mbt.qnt`
- Branch obligations:
  - `step:doCreateCatFamiliar`
  - `step:doDeliverTouchSpell`
  - `step:doPactFamiliarAttack`
  - `step:doReplaceWithRatFamiliar`
  - `step:doShareSenses`
- Allowed inputs used:
  - `cleanroom-input/MANIFEST.md`
  - `cleanroom-input/branch-coverage/source-branch-inventory.json`
  - `cleanroom-input/qnt/battle-runtime/battle-runtime-find-familiar-companion-lifecycle.mbt.qnt`
  - `cleanroom-input/qnt/battle-runtime/battle-runtime-find-familiar.qnt`
  - `cleanroom-input/qnt/battle-runtime/battle-runtime-model.qnt`
  - `cleanroom-input/raw/srd-5.2.1/Spells/Descriptions-E-L.md`
  - `cleanroom-input/raw/srd-5.2.1/Classes/Warlock.md`
  - `cleanroom-input/domain/UBIQUITOUS_LANGUAGE.md`
  - `cleanroom-input/domain/CLEANROOM_ASSUMPTIONS.md`
  - `cleanroom-input/guidance/README.md`

Behavior implemented:

- Added `src/rules/find_familiar.rs` with reusable companion lifecycle state, typed form facts, typed creature type override facts, and operation helpers for creation, recast replacement, shared senses, touch spell delivery, and pact reaction attack.
- Created a present primary companion with one-at-a-time companion count, telepathy, chosen form, chosen creature type override, and available familiar Reaction.
- Recast replacement preserves the primary companion slot and creature type override while replacing the form and retaining a single companion.
- Shared senses spends the owner's Bonus Action, touch delivery spends the familiar Reaction and commits the touch spell slot, and pact reaction attack spends one owner Attack-action attack plus the familiar Reaction and applies fixed target damage.
- Kept QNT action names, scenario labels, familiar id/status/form strings, creature type strings, and witness protocol labels quarantined in `src/qnt_adapters/battle_runtime_find_familiar_companion_lifecycle.rs`.

Generated branch coverage:

| Obligation | Target replay evidence | Diagnostic tests | Status |
| --- | --- | --- | --- |
| `cleanroom-input/qnt/battle-runtime/battle-runtime-find-familiar-companion-lifecycle.mbt.qnt#step:doCreateCatFamiliar` | `tasks/target-replay-evidence/T027-battle-runtime-find-familiar-companion-lifecycle.json#T027-create-cat-familiar#step:doCreateCatFamiliar` | `src/tests/mod.rs::find_familiar_companion_lifecycle_adapter_replays_all_branches` | `covered` |
| `cleanroom-input/qnt/battle-runtime/battle-runtime-find-familiar-companion-lifecycle.mbt.qnt#step:doDeliverTouchSpell` | `tasks/target-replay-evidence/T027-battle-runtime-find-familiar-companion-lifecycle.json#T027-deliver-touch-spell#step:doDeliverTouchSpell` | `src/tests/mod.rs::find_familiar_companion_lifecycle_adapter_replays_all_branches` | `covered` |
| `cleanroom-input/qnt/battle-runtime/battle-runtime-find-familiar-companion-lifecycle.mbt.qnt#step:doPactFamiliarAttack` | `tasks/target-replay-evidence/T027-battle-runtime-find-familiar-companion-lifecycle.json#T027-pact-familiar-attack#step:doPactFamiliarAttack` | `src/tests/mod.rs::find_familiar_companion_lifecycle_adapter_replays_all_branches` | `covered` |
| `cleanroom-input/qnt/battle-runtime/battle-runtime-find-familiar-companion-lifecycle.mbt.qnt#step:doReplaceWithRatFamiliar` | `tasks/target-replay-evidence/T027-battle-runtime-find-familiar-companion-lifecycle.json#T027-replace-with-rat-familiar#step:doReplaceWithRatFamiliar` | `src/tests/mod.rs::find_familiar_companion_lifecycle_adapter_replays_all_branches` | `covered` |
| `cleanroom-input/qnt/battle-runtime/battle-runtime-find-familiar-companion-lifecycle.mbt.qnt#step:doShareSenses` | `tasks/target-replay-evidence/T027-battle-runtime-find-familiar-companion-lifecycle.json#T027-share-senses#step:doShareSenses` | `src/tests/mod.rs::find_familiar_companion_lifecycle_adapter_replays_all_branches` | `covered` |

Target replay evidence:

- Evidence file: `tasks/target-replay-evidence/T027-battle-runtime-find-familiar-companion-lifecycle.json`
- Target profile: `rust`
- Target profile SHA-256: `6d4cc6c6a4769962798133d57aff01438fb2b661941f71d1aa8a3333f4b7ecc1`
- Quint binding: Rust quint-connect harness
- Reproduction seed or trace id: `T027-create-cat-familiar`
- Reproduction seed or trace id: `T027-deliver-touch-spell`
- Reproduction seed or trace id: `T027-pact-familiar-attack`
- Reproduction seed or trace id: `T027-replace-with-rat-familiar`
- Reproduction seed or trace id: `T027-share-senses`

Harness artifacts:

- Start gate: `tasks/START_GATE.json`
- Engine depth: `tasks/ENGINE_DEPTH_MANIFEST.json`
- State ownership: `tasks/STATE_OWNER_MANIFEST.json`
- Reviewer loop: `tasks/REVIEW_LOOP.json`
- Decider decision: `tasks/DECIDER_DECISION.json`

Diagnostic tests:

- `src/tests/mod.rs::find_familiar_companion_creates_replaces_shares_and_spends_reactions`

Remaining gaps:

- `_none_`

Verification results:

- `cargo fmt --check` passed.
- `cargo test` passed.
- `cargo clippy --all-targets -- -D warnings` passed.
- `node scripts/check-cleanroom-harness.cjs` passed.

## T026: battle-runtime-feature-selected-identity

- Manifest source commit SHA: `04249edf345a7752de2f1551dd3d509a2fffc160`
- Source branch inventory SHA: `b4e7e101def7969fc420563dc4da020c22e700f0dc0cc1d27accad6e8631225d`
- Driver: `cleanroom-input/qnt/battle-runtime/battle-runtime-feature-selected-identity.mbt.qnt`
- Branch obligations:
  - `step:doActivateInnateSorcery`
  - `step:doExcludeInnateSorceryNonSorcererSpellBenefits`
  - `step:doProjectInnateSorcerySpellBenefits`
- Allowed inputs used:
  - `cleanroom-input/MANIFEST.md`
  - `cleanroom-input/branch-coverage/source-branch-inventory.json`
  - `cleanroom-input/qnt/battle-runtime/battle-runtime-feature-selected-identity.mbt.qnt`
  - `cleanroom-input/qnt/battle-runtime/battle-runtime-feature-bridge-examples.qnt`
  - `cleanroom-input/qnt/battle-runtime/battle-runtime-model.qnt`
  - `cleanroom-input/raw/srd-5.2.1/Classes/Sorcerer.md`
  - `cleanroom-input/domain/UBIQUITOUS_LANGUAGE.md`
  - `cleanroom-input/domain/CLEANROOM_ASSUMPTIONS.md`
  - `cleanroom-input/guidance/README.md`

Behavior implemented:

- Extended `src/rules/battle_features.rs` with Innate Sorcery activation state, occurrence duration, use spending, base Sorcerer Spell Save DC calculation, and spell-benefit projection.
- Activated Innate Sorcery by spending the Bonus Action and one of two uses, producing an active 10-round occurrence and the Sorcerer-spell save DC increase.
- Scoped the attack-roll Advantage and +1 Spell Save DC benefits to Sorcerer spells while excluding non-Sorcerer spell invocations from those spell benefits.
- Kept QNT action names, scenario labels, occurrence strings, roll-mode strings, and witness protocol labels quarantined in `src/qnt_adapters/battle_runtime_feature_selected_identity.rs`.

Generated branch coverage:

| Obligation | Target replay evidence | Diagnostic tests | Status |
| --- | --- | --- | --- |
| `cleanroom-input/qnt/battle-runtime/battle-runtime-feature-selected-identity.mbt.qnt#step:doActivateInnateSorcery` | `tasks/target-replay-evidence/T026-battle-runtime-feature-selected-identity.json#T026-activate-innate-sorcery#step:doActivateInnateSorcery` | `src/tests/mod.rs::feature_selected_identity_adapter_replays_all_branches` | `covered` |
| `cleanroom-input/qnt/battle-runtime/battle-runtime-feature-selected-identity.mbt.qnt#step:doExcludeInnateSorceryNonSorcererSpellBenefits` | `tasks/target-replay-evidence/T026-battle-runtime-feature-selected-identity.json#T026-exclude-innate-sorcery-non-sorcerer-spell-benefits#step:doExcludeInnateSorceryNonSorcererSpellBenefits` | `src/tests/mod.rs::feature_selected_identity_adapter_replays_all_branches` | `covered` |
| `cleanroom-input/qnt/battle-runtime/battle-runtime-feature-selected-identity.mbt.qnt#step:doProjectInnateSorcerySpellBenefits` | `tasks/target-replay-evidence/T026-battle-runtime-feature-selected-identity.json#T026-project-innate-sorcery-spell-benefits#step:doProjectInnateSorcerySpellBenefits` | `src/tests/mod.rs::feature_selected_identity_adapter_replays_all_branches` | `covered` |

Target replay evidence:

- Evidence file: `tasks/target-replay-evidence/T026-battle-runtime-feature-selected-identity.json`
- Target profile: `rust`
- Target profile SHA-256: `6d4cc6c6a4769962798133d57aff01438fb2b661941f71d1aa8a3333f4b7ecc1`
- Quint binding: Rust quint-connect harness
- Reproduction seed or trace id: `T026-activate-innate-sorcery`
- Reproduction seed or trace id: `T026-exclude-innate-sorcery-non-sorcerer-spell-benefits`
- Reproduction seed or trace id: `T026-project-innate-sorcery-spell-benefits`

Harness artifacts:

- Start gate: `tasks/START_GATE.json`
- Engine depth: `tasks/ENGINE_DEPTH_MANIFEST.json`
- State ownership: `tasks/STATE_OWNER_MANIFEST.json`
- Reviewer loop: `tasks/REVIEW_LOOP.json`
- Decider decision: `tasks/DECIDER_DECISION.json`

Diagnostic tests:

- `src/tests/mod.rs::innate_sorcery_spends_bonus_action_use_and_scopes_spell_benefits`

Remaining gaps:

- `_none_`

Verification results:

- `cargo fmt --check` passed.
- `cargo test` passed.
- `cargo clippy --all-targets -- -D warnings` passed.
- `node scripts/check-cleanroom-harness.cjs` passed.

## T025: battle-runtime-eldritch-blast

- Manifest source commit SHA: `04249edf345a7752de2f1551dd3d509a2fffc160`
- Source branch inventory SHA: `b4e7e101def7969fc420563dc4da020c22e700f0dc0cc1d27accad6e8631225d`
- Driver: `cleanroom-input/qnt/battle-runtime/battle-runtime-eldritch-blast.mbt.qnt`
- Branch obligations:
  - `step:doFillFirstAttackHit`
  - `step:doFillFirstAttackMiss`
  - `step:doFillFirstDamageLow`
  - `step:doFillSecondAttackHit`
  - `step:doFillSecondAttackMiss`
  - `step:doFillSecondDamageLow`
  - `step:doFillTwoCreatureTargets`
  - `step:doRejectStaleAfterResolved`
- Allowed inputs used:
  - `cleanroom-input/MANIFEST.md`
  - `cleanroom-input/branch-coverage/source-branch-inventory.json`
  - `cleanroom-input/qnt/battle-runtime/battle-runtime-eldritch-blast.mbt.qnt`
  - `cleanroom-input/qnt/battle-runtime/battle-runtime-spell-bridge-examples.qnt`
  - `cleanroom-input/qnt/shared-algebras/proofs/rule-core/spell-independent-attack-sequence-core.qnt`
  - `cleanroom-input/qnt/shared-algebras/proofs/rule-core/spell-procedure-profiles-examples.qnt`
  - `cleanroom-input/raw/srd-5.2.1/Spells/Descriptions-E-L.md`
  - `cleanroom-input/raw/srd-5.2.1/Spells/Gaining-and-Casting.md`
  - `cleanroom-input/domain/UBIQUITOUS_LANGUAGE.md`
  - `cleanroom-input/domain/CLEANROOM_ASSUMPTIONS.md`
  - `cleanroom-input/guidance/README.md`

Behavior implemented:

- Extended `src/rules/spell_shapes.rs` with Eldritch Blast Force damage typing, cantrip beam-count scaling, two-beam witness state, and protocol transitions for target selection, separate spell attack rolls, damage rolls, completion, and stale-subject rejection.
- Applied hit damage through the existing hit point damage helper and preserved misses as beam-resolution progress without damaging the target.
- Exposed a quarantined Rust MBT adapter in `src/qnt_adapters/battle_runtime_eldritch_blast.rs` for the eight QNT `step` branches, keeping action names, hole names, and witness invalid-reason labels out of production rules.

Generated branch coverage:

| Obligation | Target replay evidence | Diagnostic tests | Status |
| --- | --- | --- | --- |
| `cleanroom-input/qnt/battle-runtime/battle-runtime-eldritch-blast.mbt.qnt#step:doFillFirstAttackHit` | `tasks/target-replay-evidence/T025-battle-runtime-eldritch-blast.json#T025-fill-first-attack-hit#step:doFillFirstAttackHit` | `src/tests/mod.rs::eldritch_blast_adapter_replays_all_branches` | `covered` |
| `cleanroom-input/qnt/battle-runtime/battle-runtime-eldritch-blast.mbt.qnt#step:doFillFirstAttackMiss` | `tasks/target-replay-evidence/T025-battle-runtime-eldritch-blast.json#T025-fill-first-attack-miss#step:doFillFirstAttackMiss` | `src/tests/mod.rs::eldritch_blast_adapter_replays_all_branches` | `covered` |
| `cleanroom-input/qnt/battle-runtime/battle-runtime-eldritch-blast.mbt.qnt#step:doFillFirstDamageLow` | `tasks/target-replay-evidence/T025-battle-runtime-eldritch-blast.json#T025-fill-first-damage-low#step:doFillFirstDamageLow` | `src/tests/mod.rs::eldritch_blast_adapter_replays_all_branches` | `covered` |
| `cleanroom-input/qnt/battle-runtime/battle-runtime-eldritch-blast.mbt.qnt#step:doFillSecondAttackHit` | `tasks/target-replay-evidence/T025-battle-runtime-eldritch-blast.json#T025-fill-second-attack-hit#step:doFillSecondAttackHit` | `src/tests/mod.rs::eldritch_blast_adapter_replays_all_branches` | `covered` |
| `cleanroom-input/qnt/battle-runtime/battle-runtime-eldritch-blast.mbt.qnt#step:doFillSecondAttackMiss` | `tasks/target-replay-evidence/T025-battle-runtime-eldritch-blast.json#T025-fill-second-attack-miss#step:doFillSecondAttackMiss` | `src/tests/mod.rs::eldritch_blast_adapter_replays_all_branches` | `covered` |
| `cleanroom-input/qnt/battle-runtime/battle-runtime-eldritch-blast.mbt.qnt#step:doFillSecondDamageLow` | `tasks/target-replay-evidence/T025-battle-runtime-eldritch-blast.json#T025-fill-second-damage-low#step:doFillSecondDamageLow` | `src/tests/mod.rs::eldritch_blast_adapter_replays_all_branches` | `covered` |
| `cleanroom-input/qnt/battle-runtime/battle-runtime-eldritch-blast.mbt.qnt#step:doFillTwoCreatureTargets` | `tasks/target-replay-evidence/T025-battle-runtime-eldritch-blast.json#T025-fill-two-creature-targets#step:doFillTwoCreatureTargets` | `src/tests/mod.rs::eldritch_blast_adapter_replays_all_branches` | `covered` |
| `cleanroom-input/qnt/battle-runtime/battle-runtime-eldritch-blast.mbt.qnt#step:doRejectStaleAfterResolved` | `tasks/target-replay-evidence/T025-battle-runtime-eldritch-blast.json#T025-reject-stale-after-resolved#step:doRejectStaleAfterResolved` | `src/tests/mod.rs::eldritch_blast_adapter_replays_all_branches` | `covered` |

Target replay evidence:

- Evidence file: `tasks/target-replay-evidence/T025-battle-runtime-eldritch-blast.json`
- Target profile: `rust`
- Target profile SHA-256: `6d4cc6c6a4769962798133d57aff01438fb2b661941f71d1aa8a3333f4b7ecc1`
- Quint binding: Rust quint-connect harness
- Reproduction seed or trace id: `T025-fill-first-attack-hit`
- Reproduction seed or trace id: `T025-fill-first-attack-miss`
- Reproduction seed or trace id: `T025-fill-first-damage-low`
- Reproduction seed or trace id: `T025-fill-second-attack-hit`
- Reproduction seed or trace id: `T025-fill-second-attack-miss`
- Reproduction seed or trace id: `T025-fill-second-damage-low`
- Reproduction seed or trace id: `T025-fill-two-creature-targets`
- Reproduction seed or trace id: `T025-reject-stale-after-resolved`

Harness artifacts:

- Start gate: `tasks/START_GATE.json`
- Engine depth: `tasks/ENGINE_DEPTH_MANIFEST.json`
- State ownership: `tasks/STATE_OWNER_MANIFEST.json`
- Reviewer loop: `tasks/REVIEW_LOOP.json`
- Decider decision: `tasks/DECIDER_DECISION.json`

Diagnostic tests:

- `src/tests/mod.rs::eldritch_blast_resolves_two_beam_spell_attack_sequence`

Remaining gaps:

- `_none_`

Verification results:

- `cargo fmt --check` passed.
- `cargo test` passed.
- `cargo clippy --all-targets -- -D warnings` passed.
- `node scripts/check-cleanroom-harness.cjs` passed.

## T024: battle-runtime-druid-wild-shape-form-lifecycle

- Manifest source commit SHA: `04249edf345a7752de2f1551dd3d509a2fffc160`
- Source branch inventory SHA: `b4e7e101def7969fc420563dc4da020c22e700f0dc0cc1d27accad6e8631225d`
- Driver: `cleanroom-input/qnt/battle-runtime/battle-runtime-druid-wild-shape-form-lifecycle.mbt.qnt`
- Branch obligations:
  - `step:doAssumeRidingHorse`
  - `step:doBeginNextTurn`
  - `step:doDeathReversion`
  - `step:doDismissForm`
  - `step:doIncapacitatedReversion`
  - `step:doReuseAsCat`
  - `step:doStutter`
- Allowed inputs used:
  - `cleanroom-input/MANIFEST.md`
  - `cleanroom-input/branch-coverage/source-branch-inventory.json`
  - `cleanroom-input/qnt/battle-runtime/battle-runtime-druid-wild-shape-form-lifecycle.mbt.qnt`
  - `cleanroom-input/qnt/battle-runtime/battle-runtime-druid-wild-shape.qnt`
  - `cleanroom-input/qnt/battle-runtime/battle-runtime-shape-shifting.qnt`
  - `cleanroom-input/raw/srd-5.2.1/Classes/Druid.md`
  - `cleanroom-input/raw/srd-5.2.1/Rules-Glossary.md`
  - `cleanroom-input/domain/UBIQUITOUS_LANGUAGE.md`
  - `cleanroom-input/domain/CLEANROOM_ASSUMPTIONS.md`
  - `cleanroom-input/guidance/README.md`

Behavior implemented:

- Added `src/rules/wild_shape.rs` with reusable Wild Shape form lifecycle state, known form facts, and true-form reversion helpers.
- Projected Bonus Action form assumption into Riding Horse, spending one Wild Shape use, setting form Temporary Hit Points to Druid level, merging equipment, suppressing spellcasting, and applying Beast stat-block combat fields.
- Projected next-turn Bonus Action refresh, reuse into Cat with replacement form fields and use spending, Bonus Action dismissal, Incapacitated reversion, death reversion, and a no-op stutter after death.
- Kept QNT action names, witness form labels, status labels, scenario labels, and witness protocol hole labels quarantined in `src/qnt_adapters/battle_runtime_druid_wild_shape_form_lifecycle.rs`.

Generated branch coverage:

| Obligation | Target replay evidence | Diagnostic tests | Status |
| --- | --- | --- | --- |
| `cleanroom-input/qnt/battle-runtime/battle-runtime-druid-wild-shape-form-lifecycle.mbt.qnt#step:doAssumeRidingHorse` | `tasks/target-replay-evidence/T024-battle-runtime-druid-wild-shape-form-lifecycle.json#T024-assume-riding-horse#step:doAssumeRidingHorse` | `src/tests/mod.rs::druid_wild_shape_form_lifecycle_adapter_replays_all_branches` | `covered` |
| `cleanroom-input/qnt/battle-runtime/battle-runtime-druid-wild-shape-form-lifecycle.mbt.qnt#step:doBeginNextTurn` | `tasks/target-replay-evidence/T024-battle-runtime-druid-wild-shape-form-lifecycle.json#T024-begin-next-turn#step:doBeginNextTurn` | `src/tests/mod.rs::druid_wild_shape_form_lifecycle_adapter_replays_all_branches` | `covered` |
| `cleanroom-input/qnt/battle-runtime/battle-runtime-druid-wild-shape-form-lifecycle.mbt.qnt#step:doDeathReversion` | `tasks/target-replay-evidence/T024-battle-runtime-druid-wild-shape-form-lifecycle.json#T024-death-reversion#step:doDeathReversion` | `src/tests/mod.rs::druid_wild_shape_form_lifecycle_adapter_replays_all_branches` | `covered` |
| `cleanroom-input/qnt/battle-runtime/battle-runtime-druid-wild-shape-form-lifecycle.mbt.qnt#step:doDismissForm` | `tasks/target-replay-evidence/T024-battle-runtime-druid-wild-shape-form-lifecycle.json#T024-dismiss-form#step:doDismissForm` | `src/tests/mod.rs::druid_wild_shape_form_lifecycle_adapter_replays_all_branches` | `covered` |
| `cleanroom-input/qnt/battle-runtime/battle-runtime-druid-wild-shape-form-lifecycle.mbt.qnt#step:doIncapacitatedReversion` | `tasks/target-replay-evidence/T024-battle-runtime-druid-wild-shape-form-lifecycle.json#T024-incapacitated-reversion#step:doIncapacitatedReversion` | `src/tests/mod.rs::druid_wild_shape_form_lifecycle_adapter_replays_all_branches` | `covered` |
| `cleanroom-input/qnt/battle-runtime/battle-runtime-druid-wild-shape-form-lifecycle.mbt.qnt#step:doReuseAsCat` | `tasks/target-replay-evidence/T024-battle-runtime-druid-wild-shape-form-lifecycle.json#T024-reuse-as-cat#step:doReuseAsCat` | `src/tests/mod.rs::druid_wild_shape_form_lifecycle_adapter_replays_all_branches` | `covered` |
| `cleanroom-input/qnt/battle-runtime/battle-runtime-druid-wild-shape-form-lifecycle.mbt.qnt#step:doStutter` | `tasks/target-replay-evidence/T024-battle-runtime-druid-wild-shape-form-lifecycle.json#T024-stutter-after-death#step:doStutter` | `src/tests/mod.rs::druid_wild_shape_form_lifecycle_adapter_replays_all_branches` | `covered` |

Target replay evidence:

- Evidence file: `tasks/target-replay-evidence/T024-battle-runtime-druid-wild-shape-form-lifecycle.json`
- Target profile: `rust`
- Target profile SHA-256: `6d4cc6c6a4769962798133d57aff01438fb2b661941f71d1aa8a3333f4b7ecc1`
- Quint binding: Rust quint-connect harness
- Reproduction seed or trace id: `T024-assume-riding-horse`
- Reproduction seed or trace id: `T024-begin-next-turn`
- Reproduction seed or trace id: `T024-death-reversion`
- Reproduction seed or trace id: `T024-dismiss-form`
- Reproduction seed or trace id: `T024-incapacitated-reversion`
- Reproduction seed or trace id: `T024-reuse-as-cat`
- Reproduction seed or trace id: `T024-stutter-after-death`

Harness artifacts:

- Start gate: `tasks/START_GATE.json`
- Engine depth: `tasks/ENGINE_DEPTH_MANIFEST.json`
- State ownership: `tasks/STATE_OWNER_MANIFEST.json`
- Reviewer loop: `tasks/REVIEW_LOOP.json`
- Decider decision: `tasks/DECIDER_DECISION.json`

Diagnostic tests:

- `src/tests/mod.rs::druid_wild_shape_assumes_reuses_dismisses_and_reverts_forms`

Remaining gaps:

- `_none_`

Verification results:

- `cargo fmt --check` passed.
- `cargo test` passed.
- `cargo clippy --all-targets -- -D warnings` passed.
- `node scripts/check-cleanroom-harness.cjs` passed.

## T023: battle-runtime-dragonborn-breath-weapon

- Manifest source commit SHA: `04249edf345a7752de2f1551dd3d509a2fffc160`
- Source branch inventory SHA: `b4e7e101def7969fc420563dc4da020c22e700f0dc0cc1d27accad6e8631225d`
- Driver: `cleanroom-input/qnt/battle-runtime/battle-runtime-dragonborn-breath-weapon.mbt.qnt`
- Branch obligations:
  - `step:doOpenExtraAttackSlot`
  - `step:doRejectInvalidDamageRoll`
  - `step:doRejectMismatchedArea`
  - `step:doRejectMissingResource`
  - `step:doResolveBreathWeapon`
- Allowed inputs used:
  - `cleanroom-input/MANIFEST.md`
  - `cleanroom-input/branch-coverage/source-branch-inventory.json`
  - `cleanroom-input/qnt/battle-runtime/battle-runtime-dragonborn-breath-weapon.mbt.qnt`
  - `cleanroom-input/qnt/battle-runtime/battle-runtime-feature-bridge.qnt`
  - `cleanroom-input/raw/srd-5.2.1/Character-Origins.md`
  - `cleanroom-input/domain/UBIQUITOUS_LANGUAGE.md`
  - `cleanroom-input/domain/CLEANROOM_ASSUMPTIONS.md`
  - `cleanroom-input/guidance/README.md`

Behavior implemented:

- Extended `src/rules/battle_features.rs` with Dragonborn Breath Weapon state, DC calculation, level-scaled damage dice count, and attack-replacement resolution.
- Resolved a valid Breath Weapon use by spending one use, applying failed-save full damage to the first target, applying successful-save half damage to the second target, and consuming the replaced attack.
- Resolved the extra-attack continuation branch by spending one use and preserving the remaining Attack action attack slot.
- Rejected missing resource, mismatched area, and invalid damage roll fills without damaging targets.
- Kept QNT action names, witness scenario labels, witness hole labels, and witness invalid-reason labels quarantined in `src/qnt_adapters/battle_runtime_dragonborn_breath_weapon.rs`.

Generated branch coverage:

| Obligation | Target replay evidence | Diagnostic tests | Status |
| --- | --- | --- | --- |
| `cleanroom-input/qnt/battle-runtime/battle-runtime-dragonborn-breath-weapon.mbt.qnt#step:doOpenExtraAttackSlot` | `tasks/target-replay-evidence/T023-battle-runtime-dragonborn-breath-weapon.json#T023-open-extra-attack-slot#step:doOpenExtraAttackSlot` | `src/tests/mod.rs::dragonborn_breath_weapon_adapter_replays_all_branches` | `covered` |
| `cleanroom-input/qnt/battle-runtime/battle-runtime-dragonborn-breath-weapon.mbt.qnt#step:doRejectInvalidDamageRoll` | `tasks/target-replay-evidence/T023-battle-runtime-dragonborn-breath-weapon.json#T023-reject-invalid-damage-roll#step:doRejectInvalidDamageRoll` | `src/tests/mod.rs::dragonborn_breath_weapon_adapter_replays_all_branches` | `covered` |
| `cleanroom-input/qnt/battle-runtime/battle-runtime-dragonborn-breath-weapon.mbt.qnt#step:doRejectMismatchedArea` | `tasks/target-replay-evidence/T023-battle-runtime-dragonborn-breath-weapon.json#T023-reject-mismatched-area#step:doRejectMismatchedArea` | `src/tests/mod.rs::dragonborn_breath_weapon_adapter_replays_all_branches` | `covered` |
| `cleanroom-input/qnt/battle-runtime/battle-runtime-dragonborn-breath-weapon.mbt.qnt#step:doRejectMissingResource` | `tasks/target-replay-evidence/T023-battle-runtime-dragonborn-breath-weapon.json#T023-reject-missing-resource#step:doRejectMissingResource` | `src/tests/mod.rs::dragonborn_breath_weapon_adapter_replays_all_branches` | `covered` |
| `cleanroom-input/qnt/battle-runtime/battle-runtime-dragonborn-breath-weapon.mbt.qnt#step:doResolveBreathWeapon` | `tasks/target-replay-evidence/T023-battle-runtime-dragonborn-breath-weapon.json#T023-resolve-breath-weapon#step:doResolveBreathWeapon` | `src/tests/mod.rs::dragonborn_breath_weapon_adapter_replays_all_branches` | `covered` |

Target replay evidence:

- Evidence file: `tasks/target-replay-evidence/T023-battle-runtime-dragonborn-breath-weapon.json`
- Target profile: `rust`
- Target profile SHA-256: `6d4cc6c6a4769962798133d57aff01438fb2b661941f71d1aa8a3333f4b7ecc1`
- Quint binding: Rust quint-connect harness
- Reproduction seed or trace id: `T023-open-extra-attack-slot`
- Reproduction seed or trace id: `T023-reject-invalid-damage-roll`
- Reproduction seed or trace id: `T023-reject-mismatched-area`
- Reproduction seed or trace id: `T023-reject-missing-resource`
- Reproduction seed or trace id: `T023-resolve-breath-weapon`

Harness artifacts:

- Start gate: `tasks/START_GATE.json`
- Engine depth: `tasks/ENGINE_DEPTH_MANIFEST.json`
- State ownership: `tasks/STATE_OWNER_MANIFEST.json`
- Reviewer loop: `tasks/REVIEW_LOOP.json`
- Decider decision: `tasks/DECIDER_DECISION.json`

Diagnostic tests:

- `src/tests/mod.rs::dragonborn_breath_weapon_spends_use_and_replaces_attack_with_save_damage`

Remaining gaps:

- `_none_`

Verification results:

- `cargo fmt --check` passed.
- `cargo test` passed.
- `cargo clippy --all-targets -- -D warnings` passed.
- `node scripts/check-cleanroom-harness.cjs` passed.

## T022: battle-runtime-death-saving-throw

- Manifest source commit SHA: `04249edf345a7752de2f1551dd3d509a2fffc160`
- Source branch inventory SHA: `b4e7e101def7969fc420563dc4da020c22e700f0dc0cc1d27accad6e8631225d`
- Driver: `cleanroom-input/qnt/battle-runtime/battle-runtime-death-saving-throw.mbt.qnt`
- Branch obligations:
  - `step:doDiscoverEndTurnDeathSavingThrow`
  - `step:doFillDeathSavingThrow`
  - `step:doRejectWrongActorEndTurnAfterResolved`
- Allowed inputs used:
  - `cleanroom-input/MANIFEST.md`
  - `cleanroom-input/branch-coverage/source-branch-inventory.json`
  - `cleanroom-input/qnt/battle-runtime/battle-runtime-death-saving-throw.mbt.qnt`
  - `cleanroom-input/raw/srd-5.2.1/Playing-the-Game.md`
  - `cleanroom-input/raw/srd-5.2.1/Rules-Glossary.md`
  - `cleanroom-input/domain/UBIQUITOUS_LANGUAGE.md`
  - `cleanroom-input/domain/CLEANROOM_ASSUMPTIONS.md`
  - `cleanroom-input/guidance/README.md`

Behavior implemented:

- Extended `src/rules/hit_points.rs` with a reusable Death Saving Throw state, protocol, and resolution path.
- Projected end-turn discovery of the death save hole for a 0 HP Unconscious target.
- Resolved sampled fill outcomes for natural 1, ordinary failed Death Saving Throw, third success Stable reset, and natural 20 recovery to 1 HP with reset counts.
- Projected wrong-actor rejection after the resolved branch.
- Kept QNT action names, sampled pick name `roll`, witness hole labels, and witness invalid-reason labels quarantined in `src/qnt_adapters/battle_runtime_death_saving_throw.rs`.

Generated branch coverage:

| Obligation | Target replay evidence | Diagnostic tests | Status |
| --- | --- | --- | --- |
| `cleanroom-input/qnt/battle-runtime/battle-runtime-death-saving-throw.mbt.qnt#step:doDiscoverEndTurnDeathSavingThrow` | `tasks/target-replay-evidence/T022-battle-runtime-death-saving-throw.json#T022-discover-end-turn-death-saving-throw#step:doDiscoverEndTurnDeathSavingThrow` | `src/tests/mod.rs::death_saving_throw_adapter_replays_all_branches` | `covered` |
| `cleanroom-input/qnt/battle-runtime/battle-runtime-death-saving-throw.mbt.qnt#step:doFillDeathSavingThrow` | `tasks/target-replay-evidence/T022-battle-runtime-death-saving-throw.json#T022-fill-death-saving-throw-natural-1#step:doFillDeathSavingThrow` | `src/tests/mod.rs::death_saving_throw_adapter_replays_all_branches` | `covered` |
| `cleanroom-input/qnt/battle-runtime/battle-runtime-death-saving-throw.mbt.qnt#step:doRejectWrongActorEndTurnAfterResolved` | `tasks/target-replay-evidence/T022-battle-runtime-death-saving-throw.json#T022-reject-wrong-actor-end-turn-after-resolved#step:doRejectWrongActorEndTurnAfterResolved` | `src/tests/mod.rs::death_saving_throw_adapter_replays_all_branches` | `covered` |

Target replay evidence:

- Evidence file: `tasks/target-replay-evidence/T022-battle-runtime-death-saving-throw.json`
- Target profile: `rust`
- Target profile SHA-256: `6d4cc6c6a4769962798133d57aff01438fb2b661941f71d1aa8a3333f4b7ecc1`
- Quint binding: Rust quint-connect harness
- Reproduction seed or trace id: `T022-discover-end-turn-death-saving-throw`
- Reproduction seed or trace id: `T022-fill-death-saving-throw-natural-1`
- Reproduction seed or trace id: `T022-fill-death-saving-throw-failure`
- Reproduction seed or trace id: `T022-fill-death-saving-throw-stable`
- Reproduction seed or trace id: `T022-fill-death-saving-throw-natural-20`
- Reproduction seed or trace id: `T022-reject-wrong-actor-end-turn-after-resolved`

Harness artifacts:

- Start gate: `tasks/START_GATE.json`
- Engine depth: `tasks/ENGINE_DEPTH_MANIFEST.json`
- State ownership: `tasks/STATE_OWNER_MANIFEST.json`
- Reviewer loop: `tasks/REVIEW_LOOP.json`
- Decider decision: `tasks/DECIDER_DECISION.json`

Diagnostic tests:

- `src/tests/mod.rs::death_saving_throw_resolves_all_sampled_edges`

Remaining gaps:

- `_none_`

Verification results:

- `cargo fmt --check` passed.
- `cargo test` passed.
- `cargo clippy --all-targets -- -D warnings` passed.
- `node scripts/check-cleanroom-harness.cjs` passed.

## T021: battle-runtime-danger-sense-selected-identity

- Manifest source commit SHA: `04249edf345a7752de2f1551dd3d509a2fffc160`
- Source branch inventory SHA: `b4e7e101def7969fc420563dc4da020c22e700f0dc0cc1d27accad6e8631225d`
- Driver: `cleanroom-input/qnt/battle-runtime/battle-runtime-danger-sense-selected-identity.mbt.qnt`
- Branch obligations:
  - `step:doProjectDangerSenseDexterityAdvantage`
  - `step:doSuppressDangerSenseWhileIncapacitated`
- Allowed inputs used:
  - `cleanroom-input/MANIFEST.md`
  - `cleanroom-input/branch-coverage/source-branch-inventory.json`
  - `cleanroom-input/qnt/battle-runtime/battle-runtime-danger-sense-selected-identity.mbt.qnt`
  - `cleanroom-input/qnt/battle-runtime/battle-runtime-feature-bridge-examples.qnt`
  - `cleanroom-input/qnt/shared-algebras/proofs/rule-core/unit-feature-procedure-profiles-inductive.qnt`
  - `cleanroom-input/raw/srd-5.2.1/Classes/Barbarian.md`
  - `cleanroom-input/raw/srd-5.2.1/Rules-Glossary.md`
  - `cleanroom-input/domain/UBIQUITOUS_LANGUAGE.md`
  - `cleanroom-input/domain/CLEANROOM_ASSUMPTIONS.md`
  - `cleanroom-input/guidance/README.md`

Behavior implemented:

- Extended `src/rules/battle_features.rs` with a reusable Danger Sense passive saving throw roll-mode projection.
- Projected Advantage for Dexterity Saving Throws when the Barbarian is not Incapacitated, with no Constitution Saving Throw roll-mode change.
- Projected Incapacitated suppression of Danger Sense while preserving feature acceptance for the witness branch.
- Kept QNT action names, the `barbarian_danger_sense` witness identifier, scenario labels, and protocol hole labels quarantined in `src/qnt_adapters/battle_runtime_danger_sense_selected_identity.rs`.

Generated branch coverage:

| Obligation | Target replay evidence | Diagnostic tests | Status |
| --- | --- | --- | --- |
| `cleanroom-input/qnt/battle-runtime/battle-runtime-danger-sense-selected-identity.mbt.qnt#step:doProjectDangerSenseDexterityAdvantage` | `tasks/target-replay-evidence/T021-battle-runtime-danger-sense-selected-identity.json#T021-project-danger-sense-dexterity-advantage#step:doProjectDangerSenseDexterityAdvantage` | `src/tests/mod.rs::danger_sense_adapter_replays_all_branches` | `covered` |
| `cleanroom-input/qnt/battle-runtime/battle-runtime-danger-sense-selected-identity.mbt.qnt#step:doSuppressDangerSenseWhileIncapacitated` | `tasks/target-replay-evidence/T021-battle-runtime-danger-sense-selected-identity.json#T021-suppress-danger-sense-while-incapacitated#step:doSuppressDangerSenseWhileIncapacitated` | `src/tests/mod.rs::danger_sense_adapter_replays_all_branches` | `covered` |

Target replay evidence:

- Evidence file: `tasks/target-replay-evidence/T021-battle-runtime-danger-sense-selected-identity.json`
- Target profile: `rust`
- Target profile SHA-256: `6d4cc6c6a4769962798133d57aff01438fb2b661941f71d1aa8a3333f4b7ecc1`
- Quint binding: Rust quint-connect harness
- Reproduction seed or trace id: `T021-project-danger-sense-dexterity-advantage`
- Reproduction seed or trace id: `T021-suppress-danger-sense-while-incapacitated`

Harness artifacts:

- Start gate: `tasks/START_GATE.json`
- Engine depth: `tasks/ENGINE_DEPTH_MANIFEST.json`
- State ownership: `tasks/STATE_OWNER_MANIFEST.json`
- Reviewer loop: `tasks/REVIEW_LOOP.json`
- Decider decision: `tasks/DECIDER_DECISION.json`

Diagnostic tests:

- `src/tests/mod.rs::danger_sense_only_advantages_dexterity_saves_while_not_incapacitated`

Remaining gaps:

- `_none_`

Verification results:

- `cargo fmt --check` passed.
- `cargo test` passed.
- `cargo clippy --all-targets -- -D warnings` passed.
- `node scripts/check-cleanroom-harness.cjs` passed.

## T020: battle-runtime-creature-type-protection-and-charm-selected-identity

- Manifest source commit SHA: `04249edf345a7752de2f1551dd3d509a2fffc160`
- Source branch inventory SHA: `b4e7e101def7969fc420563dc4da020c22e700f0dc0cc1d27accad6e8631225d`
- Driver: `cleanroom-input/qnt/battle-runtime/battle-runtime-creature-type-protection-and-charm-selected-identity.mbt.qnt`
- Branch obligations:
  - `step:doDiscoverAnimalFriendshipBeastTargetAdmission`
  - `step:doResolveAnimalFriendshipFailedSaveCharmed`
  - `step:doResolveAnimalFriendshipCasterDamageBreak`
  - `step:doResolveProtectionFromEvilAndGoodKnownWillingTargetProtection`
  - `step:doProjectProtectionFromEvilAndGoodScopedAttackDisadvantage`
  - `step:doPreventProtectionFromEvilAndGoodScopedCharmAndPossession`
  - `step:doResolveProtectionFromEvilAndGoodRelevantCharmSaveAdvantage`
- Allowed inputs used:
  - `cleanroom-input/MANIFEST.md`
  - `cleanroom-input/branch-coverage/source-branch-inventory.json`
  - `cleanroom-input/qnt/battle-runtime/battle-runtime-creature-type-protection-and-charm-selected-identity.mbt.qnt`
  - `cleanroom-input/qnt/battle-runtime/battle-runtime-creature-type-protection.qnt`
  - `cleanroom-input/qnt/shared-algebras/proofs/rule-core/spell-save-condition-projection-core.qnt`
  - `cleanroom-input/raw/srd-5.2.1/Spells/Descriptions-A-D.md`
  - `cleanroom-input/raw/srd-5.2.1/Spells/Descriptions-M-P.md`
  - `cleanroom-input/raw/srd-5.2.1/Rules-Glossary.md`
  - `cleanroom-input/domain/UBIQUITOUS_LANGUAGE.md`
  - `cleanroom-input/domain/CLEANROOM_ASSUMPTIONS.md`
  - `cleanroom-input/guidance/README.md`

Behavior implemented:

- Added reusable creature-type targeting and protection rules in `src/rules/creature_type_protection.rs`.
- Implemented Animal Friendship Beast-only target admission, failed-save Charmed effect projection, and damage ending the Animal Friendship effect.
- Implemented Protection from Evil and Good known willing target admission, plain-target rejection, scoped creature-type attack disadvantage, scoped Charmed/Frightened and possession prevention, and relevant-effect save Advantage.
- Kept QNT action dispatch, witness scenario labels, and witness hole labels quarantined in `src/qnt_adapters/battle_runtime_creature_type_protection_and_charm_selected_identity.rs`.

Generated branch coverage:

| Obligation | Target replay evidence | Diagnostic tests | Status |
| --- | --- | --- | --- |
| `cleanroom-input/qnt/battle-runtime/battle-runtime-creature-type-protection-and-charm-selected-identity.mbt.qnt#step:doDiscoverAnimalFriendshipBeastTargetAdmission` | `tasks/target-replay-evidence/T020-battle-runtime-creature-type-protection-and-charm-selected-identity.json#T020-discover-animal-friendship-beast-target-admission#step:doDiscoverAnimalFriendshipBeastTargetAdmission` | `src/tests/mod.rs::creature_type_protection_adapter_replays_all_branches` | `covered` |
| `cleanroom-input/qnt/battle-runtime/battle-runtime-creature-type-protection-and-charm-selected-identity.mbt.qnt#step:doResolveAnimalFriendshipFailedSaveCharmed` | `tasks/target-replay-evidence/T020-battle-runtime-creature-type-protection-and-charm-selected-identity.json#T020-resolve-animal-friendship-failed-save-charmed#step:doResolveAnimalFriendshipFailedSaveCharmed` | `src/tests/mod.rs::creature_type_protection_adapter_replays_all_branches` | `covered` |
| `cleanroom-input/qnt/battle-runtime/battle-runtime-creature-type-protection-and-charm-selected-identity.mbt.qnt#step:doResolveAnimalFriendshipCasterDamageBreak` | `tasks/target-replay-evidence/T020-battle-runtime-creature-type-protection-and-charm-selected-identity.json#T020-resolve-animal-friendship-caster-damage-break#step:doResolveAnimalFriendshipCasterDamageBreak` | `src/tests/mod.rs::creature_type_protection_adapter_replays_all_branches` | `covered` |
| `cleanroom-input/qnt/battle-runtime/battle-runtime-creature-type-protection-and-charm-selected-identity.mbt.qnt#step:doResolveProtectionFromEvilAndGoodKnownWillingTargetProtection` | `tasks/target-replay-evidence/T020-battle-runtime-creature-type-protection-and-charm-selected-identity.json#T020-resolve-protection-known-willing-target#step:doResolveProtectionFromEvilAndGoodKnownWillingTargetProtection` | `src/tests/mod.rs::creature_type_protection_adapter_replays_all_branches` | `covered` |
| `cleanroom-input/qnt/battle-runtime/battle-runtime-creature-type-protection-and-charm-selected-identity.mbt.qnt#step:doProjectProtectionFromEvilAndGoodScopedAttackDisadvantage` | `tasks/target-replay-evidence/T020-battle-runtime-creature-type-protection-and-charm-selected-identity.json#T020-project-protection-scoped-attack-disadvantage#step:doProjectProtectionFromEvilAndGoodScopedAttackDisadvantage` | `src/tests/mod.rs::creature_type_protection_adapter_replays_all_branches` | `covered` |
| `cleanroom-input/qnt/battle-runtime/battle-runtime-creature-type-protection-and-charm-selected-identity.mbt.qnt#step:doPreventProtectionFromEvilAndGoodScopedCharmAndPossession` | `tasks/target-replay-evidence/T020-battle-runtime-creature-type-protection-and-charm-selected-identity.json#T020-prevent-protection-scoped-charm-possession#step:doPreventProtectionFromEvilAndGoodScopedCharmAndPossession` | `src/tests/mod.rs::creature_type_protection_adapter_replays_all_branches` | `covered` |
| `cleanroom-input/qnt/battle-runtime/battle-runtime-creature-type-protection-and-charm-selected-identity.mbt.qnt#step:doResolveProtectionFromEvilAndGoodRelevantCharmSaveAdvantage` | `tasks/target-replay-evidence/T020-battle-runtime-creature-type-protection-and-charm-selected-identity.json#T020-resolve-protection-relevant-charm-save-advantage#step:doResolveProtectionFromEvilAndGoodRelevantCharmSaveAdvantage` | `src/tests/mod.rs::creature_type_protection_adapter_replays_all_branches` | `covered` |

Target replay evidence:

- Evidence file: `tasks/target-replay-evidence/T020-battle-runtime-creature-type-protection-and-charm-selected-identity.json`
- Target profile: `rust`
- Target profile SHA-256: `6d4cc6c6a4769962798133d57aff01438fb2b661941f71d1aa8a3333f4b7ecc1`
- Quint binding: Rust quint-connect harness
- Reproduction seed or trace id: `T020-discover-animal-friendship-beast-target-admission`
- Reproduction seed or trace id: `T020-resolve-animal-friendship-failed-save-charmed`
- Reproduction seed or trace id: `T020-resolve-animal-friendship-caster-damage-break`
- Reproduction seed or trace id: `T020-resolve-protection-known-willing-target`
- Reproduction seed or trace id: `T020-project-protection-scoped-attack-disadvantage`
- Reproduction seed or trace id: `T020-prevent-protection-scoped-charm-possession`
- Reproduction seed or trace id: `T020-resolve-protection-relevant-charm-save-advantage`

Harness artifacts:

- Start gate: `tasks/START_GATE.json`
- Engine depth: `tasks/ENGINE_DEPTH_MANIFEST.json`
- State ownership: `tasks/STATE_OWNER_MANIFEST.json`
- Reviewer loop: `tasks/REVIEW_LOOP.json`
- Decider decision: `tasks/DECIDER_DECISION.json`

Diagnostic tests:

- `src/tests/mod.rs::creature_type_protection_projects_charm_and_scoped_protection_rules`

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
