# Worktree Ledger

Campaign: `level-1-2-runtime-reducer-route`

This file prevents orphaned or confused Ralph worktrees.

## Integration Worktree

| Purpose | Path | Branch | Lane base policy |
| --- | --- | --- | --- |
| Integration | `/workspace/typescript/.codex-worktrees/dnd-cleanroom-rrconv-19` | `ralph/rrconv-19-cleanroom` | Resolve with `git rev-parse HEAD` at lane launch. |

## Active Lane Worktrees

| Lane | Path | Branch | Base SHA | Agent | Status |
| --- | --- | --- | --- | --- | --- |
| L15-RR05-BATTLE-ACTION-ATTACK-STATBLOCK-ROUTES | `/workspace/typescript/.codex-worktrees/dnd-cleanroom-l15-rr05` | `ralph/l15-rr05-attack-statblock` | `72f81250789a4e7bc6216503f2a93abac6771852` | `019f05b4-6ea8-7f00-bdb1-264528b3ee63` Ampere; reviewer `019f05c2-759e-7892-87f5-384db29ab630` James; fixer `019f05c7-37c3-7d30-bb5d-a62b6d1c3c3a` Parfit; re-reviewer `019f05d8-0d76-7a42-ab94-4a3a716a0695` Hegel; second fixer `019f05dd-57d7-7c90-86a7-12d85a9c5f8c` Confucius | re-review findings; second fixer running |

## Historical RRCONV-19 Worktrees

These predate this campaign bootstrap. They may be removable after confirming no unmerged work is needed.

| Lane | Path | Branch | Status |
| --- | --- | --- | --- |
| RRCONV-19A | `/workspace/typescript/.codex-worktrees/dnd-cleanroom-rrconv-19a` | `ralph/rrconv-19a-cleanroom` | merged-before-campaign |
| RRCONV-19B | `/workspace/typescript/.codex-worktrees/dnd-cleanroom-rrconv-19b` | `ralph/rrconv-19b-cleanroom` | merged-before-campaign |
| RRCONV-19C | `/workspace/typescript/.codex-worktrees/dnd-cleanroom-rrconv-19c` | `ralph/rrconv-19c-resolution-result` | merged-before-campaign |
| RRCONV-19D | `/workspace/typescript/.codex-worktrees/dnd-cleanroom-rrconv-19d` | `ralph/rrconv-19d-cleanroom` | merged-before-campaign |
| RRCONV-19E | `/workspace/typescript/.codex-worktrees/dnd-cleanroom-rrconv-19e` | `ralph/rrconv-19e-end-turn-subject` | merged-before-campaign |
| RRCONV-19F | `/workspace/typescript/.codex-worktrees/dnd-cleanroom-rrconv-19f` | `ralph/rrconv-19f-route-event-from-result` | merged-before-campaign |
| RRCONV-19G | `/workspace/typescript/.codex-worktrees/dnd-cleanroom-rrconv-19g` | `ralph/rrconv-19g-subject-continuation` | merged-before-campaign |

## New Worktree Protocol

When launching a lane:

1. Add a row under Active Lane Worktrees.
2. Record lane id, path, branch, base SHA, Ralph agent id, status.
3. After worker commit, record lane commit SHA.
4. After review, record review result.
5. After merge and integration verification, move row to Completed Lane Worktrees and mark removable.

## Completed Lane Worktrees

| Lane | Path | Branch | Lane Head | Merge Commit | Status |
| --- | --- | --- | --- | --- | --- |
| L15-RR03-FINISH-CURRENT-DIAGNOSTIC-QUEUE | `/workspace/typescript/.codex-worktrees/dnd-cleanroom-l15-rr03` | `ralph/l15-rr03-diagnostic-queue` | `7ef32d308d51fb54d1032d01b937d168fa63bb64` | `4c7e12d7645360adb7ab23af61144ceb243c13fe` | removable-after-operator-confirms-no-local-use |
