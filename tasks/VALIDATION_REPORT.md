# Validation Report

## Work Loop Status

- Current manifest source commit SHA: `0e024f089687526c6bc4b9e6f9c7e640414f6486`
- Source branch inventory SHA: `47b0589f442c0aaff2a814c19384fcaed7a6dbe3e7a78b5d0df8b011f56e7eae`
- Scope file: `tasks/LEVEL_1_2_SCOPE.md`
- Work Loop instructions: `tasks/WORK_LOOP.md`
- Machine-readable run ledger: `tasks/RUN_LEDGER.json`
- Last completed current-snapshot queued branch set: `<none>`
- Next queued driver: `cleanroom-input/qnt/battle-runtime/battle-runtime-magic-missile.mbt.qnt`
- Next task id: `T001`

Completion rule: a queued branch set is complete only when this report has an
entry that names the exact `.mbt.qnt` driver, records the current manifest
source commit SHA, records the current source branch inventory SHA, lists the
allowed inputs used, renders branch coverage from harness-generated target
replay evidence, and records verification results. Entries with older manifest
source commit SHAs or inventory SHAs are historical unless they include a
current-snapshot revalidation note.

## Dirty Reducer-Spine Rehearsal

This target is currently a dirty rehearsal, not an accepted cleanroom evidence
run. Current status is recorded in `tasks/DIRTY_REHEARSAL_STATUS.md`.

Reducer-routed diagnostic tests now pass for:

- `cleanroom-input/qnt/battle-runtime/battle-runtime-magic-missile.mbt.qnt`
- `cleanroom-input/qnt/battle-runtime/battle-runtime-save-gated-spell-ordering.mbt.qnt`
- `cleanroom-input/qnt/battle-runtime/battle-runtime-hit-point-restoration-ordering.mbt.qnt`
- `cleanroom-input/qnt/battle-runtime/battle-runtime-death-saving-throw.mbt.qnt`
- `cleanroom-input/qnt/battle-runtime/battle-runtime-concentration-break-teardown.mbt.qnt`

Verification results for the dirty reducer-spine batch:

- `cargo fmt --check` passed.
- `cargo test` passed: 173 tests.
- `cargo clippy --all-targets -- -D warnings` passed.
- `node scripts/check-cleanroom-harness.cjs` failed because current-snapshot
  ledger/history/evidence artifacts and historical dirty report rows are
  missing or stale in this dirty target.

These passing tests do not close branch coverage until replay evidence,
`tasks/RUN_LEDGER.json`, history artifacts, and covered validation rows are
regenerated for manifest
`0e024f089687526c6bc4b9e6f9c7e640414f6486`.

## T000: Report Shape Example Only

- Manifest source commit SHA: `0e024f089687526c6bc4b9e6f9c7e640414f6486`
- Source branch inventory SHA: `47b0589f442c0aaff2a814c19384fcaed7a6dbe3e7a78b5d0df8b011f56e7eae`
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
