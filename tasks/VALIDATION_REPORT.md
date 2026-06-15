# Validation Report

## Work Loop Status

- Current manifest source commit SHA: `04249edf345a7752de2f1551dd3d509a2fffc160`
- Source branch inventory SHA: `b4e7e101def7969fc420563dc4da020c22e700f0dc0cc1d27accad6e8631225d`
- Scope file: `tasks/LEVEL_1_2_SCOPE.md`
- Work Loop instructions: `tasks/WORK_LOOP.md`
- Last completed current-snapshot queued branch set: `cleanroom-input/qnt/character-creation-runtime/character-creation-cleric-druid-order-selected-identity.mbt.qnt`
- Next queued driver: `cleanroom-input/qnt/character-creation-runtime/character-creation-fighter-fighting-style-selected-identity.mbt.qnt`
- Next task id: `T003`

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
