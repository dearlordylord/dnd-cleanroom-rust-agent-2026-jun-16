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
| L15-RR08-CHARACTER-CREATION-ROUTES | `/workspace/typescript/.codex-worktrees/dnd-cleanroom-l15-rr08` | `ralph/l15-rr08-character-creation` | `9e1bbb9a9efc4a6a23307b1f7b8d48415e4853c9` | Harvey `019f06a6-8ed9-7560-9daa-6922b4bb277d` | implemented-pending-review, lane head `8416ee533fed413f9f072b982e0d425d89d90bac` |
| L15-RR09-CHARACTER-SHEET-ROUTES | `/workspace/typescript/.codex-worktrees/dnd-cleanroom-l15-rr09` | `ralph/l15-rr09-character-sheet` | `9e1bbb9a9efc4a6a23307b1f7b8d48415e4853c9` | Volta `019f06a6-f828-76d0-b989-42488be5d4b9`; reviewer Pauli `019f06b4-d8d0-7050-9873-35e567b871b6` | review-findings, lane head `cf9a8974fc9285878f6a8276dff8aa3cda3f3682` |

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
| L15-RR04C-RULE-CORE-SPELL-ABILITY-COMPONENTS | `/workspace/typescript/.codex-worktrees/dnd-cleanroom-l15-rr04c` | `ralph/l15-rr04c-rule-core-spell-ability` | `8b3bd015c34357b38962bfaada4e6f1d0ba3d500` | `ee30e831b0bc0fa49fa54100e54a45c32a43a60a` | removable-after-operator-confirms-no-local-use |
| L15-RR04D-RULE-CORE-FEATURE-PROFILE-COMPONENTS | `/workspace/typescript/.codex-worktrees/dnd-cleanroom-l15-rr04d` | `ralph/l15-rr04d-rule-core-feature-profile` | `3b522e37510064953d299db7cc4a739af9cd9d04` | `d0af3dc` | removable-after-operator-confirms-no-local-use |
| L15-RR04B-RULE-CORE-MOVEMENT-REACTION-SHOVE-COMPONENTS | `/workspace/typescript/.codex-worktrees/dnd-cleanroom-l15-rr04b` | `ralph/l15-rr04b-rule-core-movement-reaction-shove` | `22e6ff0ef095602ce05037382499f8715d0cef8e` | `727655c` | removable-after-operator-confirms-no-local-use |
| L15-RR04A-RULE-CORE-DAMAGE-STATBLOCK-COMPONENTS | `/workspace/typescript/.codex-worktrees/dnd-cleanroom-l15-rr04a` | `ralph/l15-rr04a-rule-core-damage-statblock` | `9d3ee41d081b7cade2daaba50c0730b412b5fc92` | `b20ce8ff40dc438d93d9e09582078af4d0fa8e24` | removable-after-operator-confirms-no-local-use |
| L15-RR03-FINISH-CURRENT-DIAGNOSTIC-QUEUE | `/workspace/typescript/.codex-worktrees/dnd-cleanroom-l15-rr03` | `ralph/l15-rr03-diagnostic-queue` | `7ef32d308d51fb54d1032d01b937d168fa63bb64` | `4c7e12d7645360adb7ab23af61144ceb243c13fe` | removable-after-operator-confirms-no-local-use |
| L15-RR05-BATTLE-ACTION-ATTACK-STATBLOCK-ROUTES | `/workspace/typescript/.codex-worktrees/dnd-cleanroom-l15-rr05` | `ralph/l15-rr05-attack-statblock` | `1b928b16bfed2c87ad95efb6aae0a5d384fdb903` | `a235602664bbae19c3bfac5e38b85b1bbc4c23a5` | removable-after-operator-confirms-no-local-use |
| L15-RR06-BATTLE-SPELL-EFFECT-ROUTES | `/workspace/typescript/.codex-worktrees/dnd-cleanroom-l15-rr06` | `ralph/l15-rr06-spell-effect-routes` | `9d17264679d8207c716f51148c52418629684891` | `1aa2ff3c6e4ca9d466a8eb0b8bc312ad3eeda025` | removable-after-operator-confirms-no-local-use |
