# CP6 Closure Report

Campaign: `level-1-2-runtime-reducer-route`

## Goal

The real goal is to prove and refine the QNT-driven reducer-route architecture: focused `.qnt` slices and copied cleanroom guidance should define enough reducer-shaped semantics for an independent character/battle runtime to be built without TypeScript implementation knowledge or production authored-identity dispatch.

The dirty Rust cleanroom in `/workspace/typescript/.codex-worktrees/dnd-cleanroom-rrconv-19` is only the rehearsal and evidence target. It is not the final product.

## Files To Read First

- `tasks/campaigns/level-1-2-runtime-reducer-route/README.md`
- `tasks/campaigns/level-1-2-runtime-reducer-route/PLAN.md`
- `tasks/campaigns/level-1-2-runtime-reducer-route/STATE.json`
- `tasks/campaigns/level-1-2-runtime-reducer-route/LANES.json`
- `tasks/campaigns/level-1-2-runtime-reducer-route/WORKTREE_LEDGER.md`
- `tasks/campaigns/level-1-2-runtime-reducer-route/CHECKPOINT_REPORT.md`
- `tasks/campaigns/level-1-2-runtime-reducer-route/CP6_AUDIT.json`
- `tasks/RUN_LEDGER.json`
- `tasks/VALIDATION_REPORT.md`
- `tasks/target-replay-evidence/*.json`

## Audit Result

CP6 recomputed coverage by exact row key, `driverPath#branchFamily:branchAction`, from:

- `cleanroom-input/branch-coverage/reducer-route-inventory.json`
- `cleanroom-input/branch-coverage/source-branch-inventory.json`
- `tasks/RUN_LEDGER.json`
- `tasks/target-replay-evidence/*.json`
- `tasks/history/L15-RR07-FU01D-PROTECTION-CHARM-WARD-SUBSTRATES/DECIDER_DECISION.json`

Result:

- Driver denominator: `97`
- In-scope obligation denominator: `668`
- Out-of-scope obligations: `36`
- Accepted pass drivers: `97 / 97 = 100.0%`
- Accepted pass obligations: `643 / 668 = 96.3%`
- Blocked obligations: `25`
- Unresolved in-scope obligations: `0`

Blocked obligations split:

- `25` target-implementation blockers recorded in target replay evidence.
- `0` source-QNT corpus blockers. FU01D's former 16 source-QNT blockers are now accepted by `tasks/target-replay-evidence/L15-RR07-FU01D-PROTECTION-CHARM-WARD-SUBSTRATES.json` and `tasks/history/L15-RR07-FU01D-PROTECTION-CHARM-WARD-SUBSTRATES/DECIDER_DECISION.json`.

## Important Accounting Note

Some older evidence refs in `tasks/RUN_LEDGER.json` use the legacy shape:

`path#traceId#branchFamily:branchAction`

That shape is ambiguous when several drivers share a branch action such as `doFillTargetChoice`. The evidence rows themselves include `driverPath`, and `CP6_AUDIT.json` uses the unambiguous row key. Future generated evidence refs should include `driverPath` or the full obligation id.

This does not create an unresolved obligation in the current closure sweep, but it is a harness-accounting improvement to carry forward.

## Remaining Blockers

No source-QNT corpus blocker remains in this closure audit.

The remaining `25` blockers are target-side reducer-route coverage gaps already recorded in target replay evidence. FU01D is no longer among them: both protection/charm/ward drivers have accepted copied-connector route replay evidence, and production route architecture remains generic subject/fill/owner/hole shape rather than authored identity dispatch.

## Verification

CP6 closure bookkeeping was refreshed over audited integration head `2455ec2f9b3c5d8b696d9cc144f196e9393038c6`.

Current verification:

- `git diff --check`: passed
- `cargo test route_replays_all_branches`: passed
- JSON parse checks for edited JSON: passed
- `node scripts/check-cleanroom-harness.cjs`: failed only on global stale non-FU01D evidence/ledger entries pinned to source `564376fd95218a209bb9eae5c9ccb54ca3e04a52` after `cleanroom-input` moved to source `53642cf0b1bc98f4426b6081fe37c98a960939fc`; this is dirty-cleanroom global harness debt, not a FU01D failure.
