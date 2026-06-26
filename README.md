# Cleanroom Rust Engine

Cleanroom experiment: implement a Rust D&D SRD 5.2.1
rules engine for character levels 1-2 from the copied
formal/domain corpus only (`cleanroom-input/`), without reading the production
implementation. Success means the corpus is sufficient implementation
guidance; failures and blockers are research data.

All agent rules live in `AGENTS.md`. The corpus snapshot and its source commit
SHA live in `cleanroom-input/MANIFEST.md`. The source branch inventory lives in
`cleanroom-input/branch-coverage/source-branch-inventory.json`. Reducer-spine
diagnostic route selection lives in
`cleanroom-input/branch-coverage/reducer-route-inventory.json`.

Cleanroom boundary rule: production reducers route by runtime shape and typed
facts, not authored or fixture identity. Using fixture identity to choose
production behavior is treated as the same boundary violation class as reading
forbidden source code; fixture names belong in adapters, tests, evidence, or
explicit catalog/selection and support-profile boundaries.

Target profile: `rust`. Target package/tooling:
Cargo. Target source extensions: `.rs`.

## Layout

- `cleanroom-input/` — the only rules corpus (RAW, QNT, domain, source branch
  inventory, reducer route inventory, and guidance pack). Read-only; populated
  by the source repo's sync script.
- `BOOTSTRAP_QUERY.md` — owner-facing query for starting a cleanroom session
  after the corpus and scaffold files have been copied here.
- `src` — target implementation and its tests.
- `tasks/` — active work assignment, task specs, `VALIDATION_REPORT.md`,
  `RUN_LEDGER.json`, `BLOCKERS.md`, and generated target replay evidence.
- `scripts/` — copied harness validators; run these from inside the cleanroom
  repo only.

## Owner Bootstrap

1. Copy or render the cleanroom package into this repo: `AGENTS.md`,
   `README.md`, `BOOTSTRAP_QUERY.md`, `target-profile.json`, `tasks/**`, and
   `scripts/**`, and `cleanroom-input/**`.
2. Start a cleanroom session manually with this directory as its only working
   root — ideally with file access restricted to this repo.
3. Commit the generated cleanroom files if they are not already committed.
4. Paste the query from `BOOTSTRAP_QUERY.md`.
5. After the implementation run, review against the same copied corpus using
   `tasks/REVIEWER_CHECKLIST.md`.
6. Have the decider evaluate `tasks/DECIDER_CHECKLIST.md` and the
   machine-readable artifacts before accepting the task.
7. Audit the run for forbidden-path reads before trusting it as cleanroom
   evidence.

## Verification

The target test suite is also the conformance lane: tasks with applicable
`.mbt.qnt` drivers must wire them through Rust quint-connect harness and emit
target replay evidence. Target-language tests may supplement diagnosis, but
they do not close source branch coverage.

```bash
cargo fmt --check
cargo test
cargo clippy --all-targets -- -D warnings
node scripts/check-cleanroom-harness.cjs
```

Use the target harness seed or trace id recorded in `tasks/target-replay-evidence/*.json` to reproduce a failing Rust quint-connect run.

## Harness Artifacts

Every accepted task records:

- `tasks/START_GATE.json`
- `tasks/ENGINE_DEPTH_MANIFEST.json`
- `tasks/STATE_OWNER_MANIFEST.json`
- `tasks/REVIEW_LOOP.json`
- `tasks/DECIDER_DECISION.json`
- `tasks/target-replay-evidence/*.json`
- `tasks/history/<taskId>/*.json`
- `tasks/RUN_LEDGER.json`

`tasks/RUN_LEDGER.json` is the machine-readable run ledger. The validation
report is the human-readable view generated from it. These JSON artifacts are
the acceptance contract.
