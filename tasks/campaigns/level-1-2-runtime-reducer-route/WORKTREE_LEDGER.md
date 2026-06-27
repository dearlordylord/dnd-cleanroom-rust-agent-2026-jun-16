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
| L15-RR06-BATTLE-SPELL-EFFECT-ROUTES | `/workspace/typescript/.codex-worktrees/dnd-cleanroom-l15-rr06` | `ralph/l15-rr06-spell-effect-routes` | `d7ec2d7957660673aaea3799787dc3224344b3d8` | Laplace `019f0600-b370-7d41-8886-aa5750bdcf84`; reviewer Dirac `019f0611-dad6-7373-9249-296531f5716f`; fixer Sartre `019f0615-ccf7-70c3-a241-9784c8cedcae`; re-reviewer Singer `019f0625-ebf0-7cf3-b37e-080a70023cc3`; fixer Gauss `019f0629-dbd6-7603-a63d-3205d2ebb315`; re-reviewer Lorentz `019f0638-1d0f-76f3-b15a-ef7e7aa7d188`; fixer Faraday `019f063c-d520-7bc3-b09e-a431c12e698e`; re-reviewer Ohm `019f0647-4cc2-79d1-b803-62640dd3f79c`; fixer Maxwell `019f064c-4200-7663-9cb3-525119462d4c`; re-reviewer Helmholtz `019f0657-e797-7193-b7e9-b34489d88ca0`; fixer Lagrange `019f065b-7727-78a1-a409-166c44b1d243`; re-reviewer Chandrasekhar `019f0660-cf0f-76e0-becd-6a1d5f7443ce`; fixer Locke `019f0664-c383-7b83-8233-824bb01ca15f` | sixth-fix-committed-pending-rereview, lane head `9d17264679d8207c716f51148c52418629684891` |

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
