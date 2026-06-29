# Next Completion Audit Candidate Lanes

Campaign: `level-1-2-runtime-reducer-route`

Status: candidate plan only. No lane in this file is launched. A resumed
orchestrator must still do the normal base check, worktree creation, active
ledger update, worker review, merge, and verifier loop before treating any row
as implementation work.

Current accepted fresh checkpoint:

- Fresh target: `ff09b6579ebb507f3c8dca06f2b040d0d46d05fb`
- Source package: `4d196258a51f4264803ff11f7c806c969f0aff2d`
- Current verifier: `python3 tools/verify_current_fresh_target.py`
- Inventory: 100 driver files, 667 in-scope branch obligations, 61 out-of-scope branch obligations
- Aggregate current-package runtime claims: the 15 strings in
  `STATE.json.currentFreshPackageCheckpoint.acceptedRuntimeClaims`

## Campaign Intent

The next work should move from "many accepted fresh tracer lanes" toward a
completion proof for the full objective:

- focused `.qnt` slices and curated guidance constrain an independent
  character/battle runtime;
- accepted runtime behavior is observed through public reducer, character,
  sheet, or handoff entrypoints;
- production code stays shape-based and does not dispatch on authored identity;
- historical dirty or pre-refresh fresh evidence is either renewed, superseded,
  or explicitly left outside the completion claim.

## Recommended First Batch

Run these as audit/reconciliation lanes before opening broad implementation.
They are intentionally small and mostly read-only because the previous checkpoint
showed that stale bookkeeping can create false next work.

| Lane ID | Kind | Inputs | Outputs | Write scope | Parallelism |
| --- | --- | --- | --- | --- | --- |
| `COMP-AUDIT-01-CURRENT-PACKAGE-GATE-MATRIX` | audit | fresh target `ff09b657`, `tools/verify_current_fresh_target.py`, `FRESH_RUN_STATE.json`, `FRESH_EXPANSION_STATE.json`, `BLOCKERS.json`, campaign `STATE.json` | objective-to-evidence matrix that lists every fresh current-package claim, every historical-only claim, every retained blocker cluster, and whether it is source-QNT, target replay, or optional dirty cleanup | new report under this campaign directory only | may run with 02/03 |
| `COMP-AUDIT-02-HISTORICAL-FOUNDATION-RENEWAL-DECISION` | audit | historical FC-01/02/03/04/05/07, SDK tracer, FEXP-01/02/03/04/05/07 evidence and current aggregate verifier | decision table: renew, superseded-by-current-claim, or exclude-from-completion-claim for each historical foundation artifact | new report under this campaign directory only | may run with 01/03 |
| `COMP-AUDIT-03-BLOCKER-TO-LANE-MAP` | audit | current `BLOCKERS.json`, copied branch inventories, `SOURCE_QNT_NEXT_TASKS.md`, `FEXP_BLOCKER_SOURCE_FEEDBACK.md` | branch-level blocker map with lane candidates grouped by generic substrate family, not selected identity driver | new report under this campaign directory only | may run with 01/02 |

Merge the three audit reports before selecting implementation lanes. The merge
should update `STATE.json.nextExecutableLaneSet` only if the reports identify a
concrete next batch with disjoint write scopes.

## Candidate Implementation Lanes

These are not ready to launch until the audit batch above confirms inputs and
write scopes. The purpose is to keep the likely next work visible without making
it executable by accident.

| Candidate ID | Depends on | Purpose | Expected implementation input | Expected output |
| --- | --- | --- | --- | --- |
| `FRESH-COMP-FOUNDATION-RENEWAL` | `COMP-AUDIT-02` | Renew or explicitly supersede early reducer-spine and SDK/tracer evidence against the current package. | accepted fresh target, current package, historical FC/FEXP/SDK evidence | focused fresh verifier(s), evidence records, and aggregate-gate update only for rows that are current-package and reducer-observed |
| `FRESH-SQNT07A-CONDITION-SAVING-SELECTED-REPLAY` | `COMP-AUDIT-03` | Reconsider selected condition-saving rows through already accepted generic condition lifecycle and Saving Throw route facts. | copied SQNT-07A condition selected witnesses plus generic condition route facts | fresh replay evidence or explicit blockers; no broad selected-identity dispatch |
| `SOURCE-REACTION-PROJECTION-FACTS` | `COMP-AUDIT-03` | Add or confirm focused source-QNT route facts for selected reaction AC projection, post-damage save/damage resolution, and interrupted slot preservation. | current copied reaction taxonomy facts, selected reaction blockers, RAW/domain anchors | source-QNT/guidance task plan or source lane; fresh replay waits for package refresh |
| `SOURCE-SPATIAL-OBJECT-BOUNDARY-FACTS` | `COMP-AUDIT-03` | Decide which movable light, presentation/invisible-revealing, object durability, and stale object history facts are reducer-owned versus table-boundary facts. | spatial/object blockers, object/light route facts, current accepted spatial/damage evidence | source-QNT/guidance task plan or explicit table-boundary classification |
| `SOURCE-SELECTED-SPELL-RESIDUAL-BRANCH-MAP` | `COMP-AUDIT-03` | Map residual selected spell branches to existing generic substrates or exact missing facts. | selected spell residual queue, SQNT-03A through SQNT-03G route/component facts, current inventories | branch-level map; no whole-driver blocker if only individual branches remain blocked |

## Parallelism Rule

The three `COMP-AUDIT-*` lanes are parallelizable because they write separate
reports only. Implementation lanes should not run until those reports are
merged. After that:

- source-QNT lanes can run in parallel only when they own disjoint QNT files and
  the package refresh is serialized;
- fresh target replay lanes can run in parallel only when runtime/example/test
  write scopes do not overlap;
- aggregate files such as `STATE.json`, `BLOCKERS.json`, `FRESH_RUN_STATE.json`,
  `FRESH_EXPANSION_STATE.json`, and `tools/verify_current_fresh_target.py`
  should be updated by one integration lane after the parallel workers finish.

## Verification For Audit Lanes

Each audit lane should at minimum run:

- `git merge-base --is-ancestor <base> HEAD`
- `python3 -m json.tool tasks/campaigns/level-1-2-runtime-reducer-route/STATE.json >/dev/null`
- `python3 -m json.tool tasks/campaigns/level-1-2-runtime-reducer-route/FRESH_EXPANSION_LANES.json >/dev/null`
- `git diff --check`

The integration merge should additionally run the accepted fresh target gate:

```sh
cd /workspace/typescript/.codex-worktrees/dnd-fresh-cleanroom-dry-run-fc00
python3 tools/verify_current_fresh_target.py
```
