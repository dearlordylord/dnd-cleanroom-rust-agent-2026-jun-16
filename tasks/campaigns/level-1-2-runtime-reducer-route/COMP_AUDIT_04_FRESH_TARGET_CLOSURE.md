# COMP-AUDIT-04 Fresh Target Closure

Campaign: `level-1-2-runtime-reducer-route`
Lane: `COMP-AUDIT-04-FRESH-TARGET-CLOSURE`
Audit date: 2026-06-29

Verdict: `closure-proven`

The current accepted fresh target proves the campaign's architecture objective:
focused copied QNT slices and curated cleanroom input now define enough
reducer-shaped semantics for an independent character/battle runtime evidence
target to be built without TypeScript implementation knowledge and without
production authored-identity dispatch. This is not a claim that every D&D rule
or selected/grouped driver branch is complete; it is a closure claim for the
QNT-driven reducer-route architecture proof.

## Base And Boundary Check

- Declared base SHA: `b650352e9c8e40a031f50ae22ed2bb264be26d2c`
- Audit worktree: `/workspace/typescript/.codex-worktrees/dnd-comp-audit-04-fresh-target-closure`
- Branch: `ralph/comp-audit-04-fresh-target-closure`
- `git rev-parse HEAD`: `b650352e9c8e40a031f50ae22ed2bb264be26d2c`
- Base ancestry check: `git merge-base --is-ancestor b650352e9c8e40a031f50ae22ed2bb264be26d2c HEAD` passed.
- Accepted fresh target inspected: `/workspace/typescript/.codex-worktrees/dnd-fresh-cleanroom-dry-run-fc00`
- Accepted fresh target `HEAD`: `02a1337d5e2be04307d44c63e2596f60c2104301`
- Copied source package commit: `01ffe81e9166af50443d4aa4a76a4c34ca79d2c1`
- Boundary followed: no `/workspace/typescript/dnd` or sibling source checkout was read. Evidence came from this campaign-control worktree, the accepted fresh target, copied `cleanroom-input/**`, target evidence/state, verifier scripts, tests, examples, and Cargo tooling.

## Evidence Table

| Requirement | Authoritative evidence inspected | Result | Notes |
| --- | --- | --- | --- |
| Fresh target head and package identity are coherent. | Campaign `STATE.json.currentFreshPackageCheckpoint`; fresh target `git rev-parse HEAD`; `FRESH_RUN_STATE.json.manifestFacts`; `FRESH_EXPANSION_STATE.json.currentPackageAdmission`; `tools/verify_current_fresh_target.py`. | Passed. | Head is `02a1337d5e2be04307d44c63e2596f60c2104301`; source package is `01ffe81e9166af50443d4aa4a76a4c34ca79d2c1`; inventory is 100 driver files, 667 in-scope obligations, 61 out-of-scope obligations; aggregate current runtime claim set has 17 ids. |
| Aggregate gate covers the current package. | Ran `python3 tools/verify_current_fresh_target.py` in the accepted fresh target. | Passed. | The gate recomputed inventory, ran focused verifiers, checked package hash/file count, checked current claim sets in state/blocker manifests, and scanned for forbidden source/dirty boundary paths. |
| Aggregate gate includes object-stale and reaction projection. | `tools/verify_current_fresh_target.py` includes `tools/verify_fresh_object_stale_public_history_replay.py` and `tools/verify_fresh_rr_reaction_interrupt_taxonomy.py`; current claim set includes `FRESH-OBJECT-STALE-PUBLIC-HISTORY-REPLAY` and `FRESH-RR-REACTION-INTERRUPT-TAXONOMY-CURRENT-PACKAGE-REPLAY`. | Passed. | The verifier output accepted object-stale public history and 5 reaction/interrupt taxonomy route surfaces. |
| Shared battle reducer entrypoints exist and drive observed evidence. | `src/lib.rs` public `start_battle`, `discover_battle_acts`, `resolve_battle_subject`, `advance_turn`; `tests/reducer_spine.rs`; `tests/sdk_tracer_bullet.rs`; `FRESH_RUN_REPORT.md`. | Passed. | Observed evidence for accepted route families is emitted through public reducer entrypoints, not local replay-only adapters. |
| Public battle resolution result exists. | `src/lib.rs` defines public `ResolutionResult`; `resolve_battle_subject` returns `ReducerOutcome<ResolutionResult>`; tests consume `Resolved`, `NeedsHoles`, and `Rejected`. | Passed with naming note. | The fresh Rust target names the public battle resolution type `ResolutionResult`, not `BattleResolutionResult`; it is the target's public equivalent. |
| Public character/sheet/handoff entrypoints exist. | `EVIDENCE/sdk-tracer-bullet-programmatic-surface.json`; `tools/verify_sdk_tracer_bullet.py`; `tests/sdk_tracer_bullet.rs`; `STATE_OWNERS.md`; `FRESH_SDK_COMPOSITION_ACCEPTANCE.md`. | Passed. | Public APIs include `CharacterSheet::new`, `project_character_sheet_to_battle`, `enter_battle_runtime`, `CharacterBattleEncounterSetup::new`, `SheetEncounterParticipant::new`, and `enter_composed_battle_runtime`. |
| SDK tracer bullet proves an integrated scenario. | Ran `cargo test --test sdk_tracer_bullet`; inspected `EVIDENCE/sdk-tracer-bullet-programmatic-surface.json`. | Passed. | All 3 SDK tracer tests passed; evidence records programmatic sheet creation, projection, composed encounter entry, generic battle act discovery/resolution, damage, action spend, and turn advancement. |
| Expected records are copied-QNT-derived and observed records are public reducer records. | Focused verifiers for object-stale and reaction taxonomy; `EVIDENCE/fresh-object-stale-public-history-replay.json`; `EVIDENCE/fresh-rr-reaction-interrupt-taxonomy.json`. | Passed. | Object-stale expected route records are built from copied `battle-runtime-starry-wisp-object.route.mbt.qnt`; observed records come from a Rust example using public reducer entrypoints. Reaction taxonomy expected records/projection facts are mechanically derived from copied connector definitions and compared to public reducer observations. |
| No production authored-identity dispatch is used for accepted runtime semantics. | `tools/verify_current_fresh_target.py` static boundary checks; focused verifier scans; `STATE_OWNERS.md`; `FRESH_RUN_REPORT.md`; evidence `implementationFacts.productionAuthoredIdentityDispatch = false`. | Passed for the accepted target scope. | Identity remains in copied QNT/evidence/test/selection boundaries. Production routing is by typed subject profiles, fill kinds, hole frontiers, owner groups, and explicit runtime facts. |
| Object-stale blocker is truly superseded. | `EVIDENCE/fresh-object-stale-public-history-replay.json`; `tools/verify_fresh_object_stale_public_history_replay.py`; `BLOCKERS.json.phaseBlockers`. | Passed. | The prior blocker `FEXP-03-object-stale-replay-isolated-route-surface-not-publicly-observable` is recorded as superseded, with resolution evidence. The focused verifier checks a 9-record public route history and verifies stale rejection through `BattleHoleFrontierOwner`; this is not merely hidden. |
| Selected-spell residual branch map classification. | `STATE.json.nextExecutableLaneSet.deferredLanes`; `COMP_AUDIT_03_BLOCKER_TO_LANE_MAP.md`; current `BLOCKERS.json.phaseBlockers`; accepted fresh target claim set. | Not mandatory before closure. | `SOURCE-SELECTED-SPELL-RESIDUAL-BRANCH-MAP` is still useful future guidance if the residual queue is ambiguous. It is not required for this real objective because the architecture proof is based on accepted generic route substrates and explicit refusal to dispatch on selected identity, not wholesale selected-driver acceptance. |
| Dirty cleanroom historical harness debt classification. | `COMP_AUDIT_01_CURRENT_PACKAGE_GATE_MATRIX.md`; `COMP_AUDIT_02_HISTORICAL_FOUNDATION_RENEWAL_DECISION.md`; `COMP_AUDIT_03_BLOCKER_TO_LANE_MAP.md`; campaign `STATE.json`. | Does not block closure. | Dirty harness stale-validator debt is historical diagnostic debt with `freshRunBlocker: false`. Current closure rests on the accepted fresh target aggregate gate, not the dirty harness. |

## Remaining Work

Mandatory before closure:

- None.

Optional/future:

- Run `SOURCE-SELECTED-SPELL-RESIDUAL-BRANCH-MAP` only if the orchestrator wants a refreshed residual selected-spell planning map. It should remain source guidance, not a broad target replay or identity-dispatch implementation lane.
- Clean dirty harness stale-validator or historical ledger debt only for hygiene. It is not proof work for this objective.
- Add future route families for selected/grouped driver branches only when they can be split into generic substrate facts, typed procedure facts, or explicit table-boundary exclusions.
- Productize the architecture into a real runtime implementation campaign if desired. The current closure proves the architecture/evidence target, not complete gameplay parity.

## Residual Risk

- The accepted target proves a representative and growing reducer-route architecture, not exhaustive SRD runtime coverage.
- Static authored-identity checks are lane/verifier scoped. Future production surfaces need the same guardrails.
- Some retained `phaseBlockers` remain as historical guardrails. They are not closure blockers, but they can be misread if someone treats every historical blocker label as current target failure.
- The target has a compact Rust implementation surface. Future broadening should preserve the current pattern: copied-QNT-derived expected records, public reducer observations, and explicit owner/type facts.

## Commands Run

Audit worktree:

```sh
printf 'declared base SHA: b650352e9c8e40a031f50ae22ed2bb264be26d2c\n'
git rev-parse HEAD
git merge-base --is-ancestor b650352e9c8e40a031f50ae22ed2bb264be26d2c HEAD
```

Result: `HEAD` was `b650352e9c8e40a031f50ae22ed2bb264be26d2c`; ancestry check passed.

```sh
python3 -m json.tool tasks/campaigns/level-1-2-runtime-reducer-route/STATE.json >/tmp/comp-audit-state.json
```

Result: passed.

Accepted fresh target:

```sh
git rev-parse HEAD
```

Result: `02a1337d5e2be04307d44c63e2596f60c2104301`.

```sh
python3 tools/verify_current_fresh_target.py
```

Result: passed. It reported 100 drivers, 667 in-scope obligations, 61 out-of-scope obligations, accepted object-stale public history, accepted reaction/interrupt taxonomy, and accepted the current `01ffe` package claim set.

```sh
cargo test --test sdk_tracer_bullet
```

Result: passed, 3 tests.

```sh
cargo test fresh_object_stale_public_history_replay_routes_through_reducer_frontier -- --nocapture
cargo test fresh_reaction_interrupt_taxonomy_routes_use_generic_payload_shapes -- --nocapture
```

Result: both passed as separate focused filters, 1 matching test each.
