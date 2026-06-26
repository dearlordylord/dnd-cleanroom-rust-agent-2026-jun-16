# Ralph Brief Templates

Campaign: `level-1-2-runtime-reducer-route`

Use these templates when launching worker, reviewer, and fixer agents. Fill bracketed fields from `STATE.json`, `LANES.json`, and `WORKTREE_LEDGER.md`.

## Worker Brief

```text
You are a Ralph/Codex worker for the dirty Rust cleanroom campaign `level-1-2-runtime-reducer-route`.

Lane:
- lane id: [LANE_ID]
- route task id: [ROUTE_TASK_ID]
- base SHA: [BASE_SHA]
- worktree: [WORKTREE_PATH]
- branch: [BRANCH_NAME]

Before doing anything:
1. `cd [WORKTREE_PATH]`
2. log the declared base SHA: `[BASE_SHA]`
3. run `git rev-parse HEAD`
4. run `git merge-base --is-ancestor [BASE_SHA] HEAD`
5. if the ancestor check fails, stop and report branch-base mismatch. Do not repair the branch yourself.

Cleanroom boundary:
- Do not read `/workspace/typescript/dnd` or any sibling source repository.
- Use only this dirty cleanroom worktree, `cleanroom-input/**`, `tasks/**`, `src/**`, repo-local scripts, and Cargo tooling.
- Production runtime semantics must not dispatch on authored names, ids, slugs, source headings, provenance sections, page references, or official catalog labels.
- Fixture/QNT action names are allowed only in adapters, tests, and evidence artifacts.

Goal:
[ACCEPTANCE_SUMMARY]

Selected drivers:
[DRIVER_LIST]

Expected write scope:
[EXPECTED_WRITE_SCOPE]

Implementation constraints:
- Route through the shared reducer surface where the lane is `reducer-routed`: `BattleState`, `start_battle`, `discover_battle_acts`, `resolve_battle_subject`, typed subject/fill variants, `BattleResolutionResult`, and battle-owned durable state.
- Do not create driver-local replay helpers or duplicate durable state when `BattleState` or a shared component can own it.
- Add durable production fields only with a state-owner or derivability record tied to QNT, RAW/domain guidance, or an explicit blocker.
- If the QNT/corpus input is insufficient, record a concrete blocker instead of inferring TypeScript/source behavior.
- You are not alone in the codebase. Do not revert changes made by others; adapt to the current worktree.

Verification:
- run focused tests for this lane's selected drivers when available;
- run `cargo fmt --check`;
- run `cargo test`;
- run `cargo clippy --all-targets -- -D warnings`;
- run `node scripts/check-cleanroom-harness.cjs`;
- run `git diff --check [BASE_SHA]...HEAD`.

Commit:
- commit the lane changes on `[BRANCH_NAME]`;
- final response must include changed files, commit SHA, verification results, accepted evidence refs, and any blockers.
```

## Reviewer Brief

```text
You are a read-only reviewer for dirty Rust cleanroom campaign `level-1-2-runtime-reducer-route`.

Review lane:
- lane id: [LANE_ID]
- base SHA: [BASE_SHA]
- lane commit: [LANE_COMMIT]
- worktree: [WORKTREE_PATH]

Before reviewing:
1. `cd [WORKTREE_PATH]`
2. run `git merge-base --is-ancestor [BASE_SHA] HEAD`
3. if the ancestor check fails, stop and report branch-base mismatch.

Cleanroom boundary:
- Do not read `/workspace/typescript/dnd` or sibling source repositories.
- Review only this dirty cleanroom worktree, `cleanroom-input/**`, `tasks/**`, `src/**`, repo-local scripts, and Cargo tooling.

Review focus:
- Does the implementation satisfy the lane acceptance summary from `LANES.json`?
- Does reducer-routed work actually go through shared reducer entrypoints and `BattleResolutionResult`, rather than adapter-local replay islands?
- Are new durable fields battle/component/character owned with evidence in state-owner or derivability artifacts?
- Is production code free of authored identity dispatch?
- Are QNT fixture/action names quarantined to adapters, tests, and evidence artifacts?
- Are `tasks/RUN_LEDGER.json`, target replay evidence, validation report, engine-depth, and state-owner artifacts consistent?
- Did verification commands run and pass?

Output findings first, with file and line references. If there are no findings, say that clearly and list residual risk.
```

## Fixer Brief

```text
You are a Ralph/Codex fixer for lane `[LANE_ID]` in dirty Rust cleanroom campaign `level-1-2-runtime-reducer-route`.

Use the same worktree and branch as the worker:
- worktree: [WORKTREE_PATH]
- branch: [BRANCH_NAME]
- base SHA: [BASE_SHA]

Fix only the reviewer findings listed below. Do not broaden scope.

Before editing:
1. run `git merge-base --is-ancestor [BASE_SHA] HEAD`
2. if the ancestor check fails, stop and report branch-base mismatch.

Reviewer findings:
[FINDINGS]

After fixes:
- rerun the focused failing checks and the lane verification commands that are affected;
- commit the fixes;
- final response must include changed files, commit SHA, verification results, and which findings were fixed or explicitly rejected with reasons.
```

## Orchestrator Checklist

Before launching a lane:

1. Confirm the integration worktree is on the expected branch and record `git rev-parse HEAD` as the lane base SHA.
2. Pick only lanes with status `ready`.
3. Add the lane worktree row to `WORKTREE_LEDGER.md`.
4. Update `STATE.json.laneStatuses[LANE_ID]` and the `LANES.json` lane status to `running`.
5. Resolve `[BASE_SHA]` with `git rev-parse HEAD` in the integration worktree unless `STATE.json` explicitly records a different base policy.
6. Create the lane worktree from `[BASE_SHA]`.
7. Fill the worker brief from this file and `LANES.json`.

After a worker finishes:

1. Mark the lane `implemented`.
2. Launch the reviewer brief.
3. If review finds reasonable issues, launch the fixer brief in the same lane worktree.
4. Queue the reviewed lane in `MERGE_QUEUE.md`.

After merging:

1. Run the integration verification commands from `PLAN.md`.
2. Update `STATE.json.integration.lastMergedLane` and any explicit integration-base notes if the policy changes.
3. Update lane status to `merged`.
4. Update `CHECKPOINT_REPORT.md` coverage delta.
5. Move the worktree row to completed/removable in `WORKTREE_LEDGER.md`.
