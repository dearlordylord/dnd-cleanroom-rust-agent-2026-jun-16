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
| L15-RR04A-RULE-CORE-DAMAGE-STATBLOCK-COMPONENTS | `/workspace/typescript/.codex-worktrees/dnd-cleanroom-l15-rr04a` | `ralph/l15-rr04a-rule-core-damage-statblock` | `a625d3e7190eb33396c17ee5dca7ae73f413b348` | Cicero `019f0670-c7a0-72d1-9fd3-cfeb18268fab`; reviewer Planck `019f0679-9f1a-7872-bda1-4f19ad5ce3d4`; fixer Poincare `019f067d-e564-7380-861f-837a1b321272`; re-reviewer Archimedes `019f0681-d1de-7813-b55a-6f1c8d97ffc9` | reviewed-clean-pending-merge, lane head `9d3ee41d081b7cade2daaba50c0730b412b5fc92` |
| L15-RR04B-RULE-CORE-MOVEMENT-REACTION-SHOVE-COMPONENTS | `/workspace/typescript/.codex-worktrees/dnd-cleanroom-l15-rr04b` | `ralph/l15-rr04b-rule-core-movement-reaction-shove` | `a625d3e7190eb33396c17ee5dca7ae73f413b348` | Russell `019f0671-240b-7540-9f83-d8c2b4e83b30`; reviewer Sagan `019f0678-5397-7e20-b042-9349633364b4` | review-findings, lane head `629d9ef2c992042ddf007a6517f7e428478598d1` |
| L15-RR04C-RULE-CORE-SPELL-ABILITY-COMPONENTS | `/workspace/typescript/.codex-worktrees/dnd-cleanroom-l15-rr04c` | `ralph/l15-rr04c-rule-core-spell-ability` | `a625d3e7190eb33396c17ee5dca7ae73f413b348` | Avicenna `019f0671-762b-7002-954d-a26a7c37c13c` | implemented-pending-review, lane head `399690a03ae83538875768950e5d985c932a723f` |
| L15-RR04D-RULE-CORE-FEATURE-PROFILE-COMPONENTS | `/workspace/typescript/.codex-worktrees/dnd-cleanroom-l15-rr04d` | `ralph/l15-rr04d-rule-core-feature-profile` | `a625d3e7190eb33396c17ee5dca7ae73f413b348` | Mill `019f0671-d268-7f62-b173-674d8d90738f`; reviewer Banach `019f067c-1408-76d3-ade3-0e83ab165998` | review-findings, lane head `063ceefb6b07dec0d5afb7c4435d2687d48dc621` |

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
| L15-RR05-BATTLE-ACTION-ATTACK-STATBLOCK-ROUTES | `/workspace/typescript/.codex-worktrees/dnd-cleanroom-l15-rr05` | `ralph/l15-rr05-attack-statblock` | `1b928b16bfed2c87ad95efb6aae0a5d384fdb903` | `a235602664bbae19c3bfac5e38b85b1bbc4c23a5` | removable-after-operator-confirms-no-local-use |
| L15-RR06-BATTLE-SPELL-EFFECT-ROUTES | `/workspace/typescript/.codex-worktrees/dnd-cleanroom-l15-rr06` | `ralph/l15-rr06-spell-effect-routes` | `9d17264679d8207c716f51148c52418629684891` | `1aa2ff3c6e4ca9d466a8eb0b8bc312ad3eeda025` | removable-after-operator-confirms-no-local-use |
