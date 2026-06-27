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
| L15-RRCP5-A-RIDER-AND-WEAPON-HOSTED-ROUTES | `/workspace/typescript/.codex-worktrees/dnd-cleanroom-l15-rrcp5-a` | `ralph/l15-rrcp5-a-rider-weapon` | `73c5ccea0ee7425952f231485325df5a45a0f36a` | Heisenberg the 2nd `019f0814-4361-7420-9980-4380319e11c1` | worker-running |
| L15-RRCP5-B-ACTIVE-EFFECT-LIFECYCLE-ROUTES | `/workspace/typescript/.codex-worktrees/dnd-cleanroom-l15-rrcp5-b` | `ralph/l15-rrcp5-b-active-effects` | `73c5ccea0ee7425952f231485325df5a45a0f36a` | Hypatia the 2nd `019f0814-43dd-7670-b3bd-6761af286704` | worker-running |
| L15-RRCP5-C-REACTION-INTERRUPT-ROUTES | `/workspace/typescript/.codex-worktrees/dnd-cleanroom-l15-rrcp5-c` | `ralph/l15-rrcp5-c-reactions` | `73c5ccea0ee7425952f231485325df5a45a0f36a` | Planck the 2nd `019f0814-4433-7fb1-8f8e-80862b5967b4` | worker-running |
| L15-RRCP5-D-COMPANION-OBJECT-BOUNDARY-ROUTES | `/workspace/typescript/.codex-worktrees/dnd-cleanroom-l15-rrcp5-d` | `ralph/l15-rrcp5-d-companion-object` | `73c5ccea0ee7425952f231485325df5a45a0f36a` | Hume the 2nd `019f0814-446b-78a3-8f87-54aea8e638d8` | worker-running |
| L15-RRCP5-E-ABILITY-SEARCH-CHOICE-ROUTES | `/workspace/typescript/.codex-worktrees/dnd-cleanroom-l15-rrcp5-e` | `ralph/l15-rrcp5-e-ability-search` | `73c5ccea0ee7425952f231485325df5a45a0f36a` | Locke the 2nd `019f0814-452a-77b2-8a73-ea35806cd964` | worker-running |
| L15-RRCP5-F-INDEPENDENT-SPELL-ATTACK-SEQUENCE-ROUTES | `/workspace/typescript/.codex-worktrees/dnd-cleanroom-l15-rrcp5-f` | `ralph/l15-rrcp5-f-spell-attack-sequence` | `73c5ccea0ee7425952f231485325df5a45a0f36a` | Helmholtz the 2nd `019f0814-4594-7963-ac7d-9d95ffdf6990` | worker-running |

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
| L15-RR08-CHARACTER-CREATION-ROUTES | `/workspace/typescript/.codex-worktrees/dnd-cleanroom-l15-rr08` | `ralph/l15-rr08-character-creation` | `e11862d68ab0b02ec0db303504923e9222c3446f` | `76f2c5c61c1553c002dcd3f026a6ac9f444d1fdf` | removable-after-operator-confirms-no-local-use |
| L15-RR09-CHARACTER-SHEET-ROUTES | `/workspace/typescript/.codex-worktrees/dnd-cleanroom-l15-rr09` | `ralph/l15-rr09-character-sheet` | `410e3db7d181c7fba9265cc753ab977198dacca7` | `616b6c27104807a0cf312d4cd68485e0c7fef1c3` | removable-after-operator-confirms-no-local-use |
| L15-RR10-CHARACTER-BATTLE-HANDOFF-ROUTES | `/workspace/typescript/.codex-worktrees/dnd-cleanroom-l15-rr10` | `ralph/l15-rr10-character-battle-handoff` | `4ea5566fc503fcfa17f430b523faa87dc189943c` | `3a0c18402e5620a55d0f2046329ab5697023c2e3` | removable-after-operator-confirms-no-local-use |
| L15-RR07S-B-ZERO-HP-FORM-MOVEMENT-SUBSTRATES | `/workspace/typescript/.codex-worktrees/dnd-cleanroom-l15-rr07s-b` | `ralph/l15-rr07s-b-zero-hp-form-movement` | `81fd387c6dda4b00c28a752492d6aa0d2fee4d3d` | `0ef395a` | removable-after-operator-confirms-no-local-use |
| L15-RR07S-C-WEAPON-BREATH-FEATURE-SUBSTRATES | `/workspace/typescript/.codex-worktrees/dnd-cleanroom-l15-rr07s-c` | `ralph/l15-rr07s-c-weapon-breath-feature` | `6d89dd42b345c3253d36759d723c1fe4a3271c2c` | `67bc22f` | removable-after-operator-confirms-no-local-use |
| L15-RR07S-A-PASSIVE-ROLL-RESOURCE-SUBSTRATES | `/workspace/typescript/.codex-worktrees/dnd-cleanroom-l15-rr07s-a` | `ralph/l15-rr07s-a-passive-roll-resource` | `582b2a71c6573c480d4ccddec15381308a2bd667` | `1b0d0dbc1615de1efdafce3f74b1b6372e2df8d9` | removable-after-operator-confirms-no-local-use |
| L15-RR07-FU08A-METAMAGIC-GOVERNOR-QUICKENED-SUBSTRATE | `/workspace/typescript/.codex-worktrees/dnd-cleanroom-l15-rr07-fu08a` | `ralph/l15-rr07-fu08a-metamagic-governor` | `07e47f01a9e728fcfe919b4af2e1a4592b97069f` | `b909dfdaa01bca96953a590b9984518bd5e9bc68` | removable-after-operator-confirms-no-local-use |
| L15-RR07-FU01A-CATALOG-READY-SPELL-SUBSTRATES | `/workspace/typescript/.codex-worktrees/dnd-cleanroom-l15-rr07-fu01a` | `ralph/l15-rr07-fu01a-catalog-ready` | `86290c012cdefb8afdcc526a8caa58da190b407b` | `5a6e5d2d1975788acef1373786eb94dd0074407e` | removable-after-operator-confirms-no-local-use |
| L15-RR07-FU08B-METAMAGIC-SAVE-RANGE-TARGET-SUBSTRATES | `/workspace/typescript/.codex-worktrees/dnd-cleanroom-l15-rr07-fu08b` | `ralph/l15-rr07-fu08b-metamagic-save-range` | `3727abed465765fbc2b3f223076ed265d31393ff` | `20daf3ede80af636df4e71a776d95783182fef2d` | removable-after-operator-confirms-no-local-use |
| L15-RR07-FU08C-METAMAGIC-REROLL-DAMAGE-PROJECTION-SUBSTRATES | `/workspace/typescript/.codex-worktrees/dnd-cleanroom-l15-rr07-fu08c` | `ralph/l15-rr07-fu08c-metamagic-reroll-damage` | `a0ac254c610b4c0e64b4f6d160c0ef533f400a40` | `5bec99231b38d67c5930922cb77328d2b71c031a` | removable-after-operator-confirms-no-local-use |
| L15-RR07-FU01F-SPATIAL-LIGHT-AREA-MOVEMENT-SUBSTRATES | `/workspace/typescript/.codex-worktrees/dnd-cleanroom-l15-rr07-fu01f` | `ralph/l15-rr07-fu01f-spatial-light-area` | `15cfbe5193ed93e6119873732cccfa93c7f37349` | `78be1eccb627f3ae7987380922d873b9ca7e497b` | removable-after-operator-confirms-no-local-use |
| L15-RR07-FU01B-SPELL-ATTACK-SAVE-DAMAGE-SUBSTRATES | `/workspace/typescript/.codex-worktrees/dnd-cleanroom-l15-rr07-fu01b` | `ralph/l15-rr07-fu01b-spell-attack-damage` | `195a473783bc666f5413dabe0a2bf93ea18003df` | `6834ee18356e01eb9b00bd4b32f0169b75a7220d` | removable-after-operator-confirms-no-local-use |
| L15-RR07-FU01E-ARMOR-CLASS-REACTION-SUBSTRATES | `/workspace/typescript/.codex-worktrees/dnd-cleanroom-l15-rr07-fu01e` | `ralph/l15-rr07-fu01e-armor-reaction` | `c135ceb24f7b0f44d93f14763d55dbb9aaa8cf4f` | `4f69ca5c407cbc53c5e3a5a431cedf0f1d3148c9` | removable-after-operator-confirms-no-local-use |
| L15-RR07-FU01C-WEAPON-BUFF-MARK-SMITE-SUBSTRATES | `/workspace/typescript/.codex-worktrees/dnd-cleanroom-l15-rr07-fu01c` | `ralph/l15-rr07-fu01c-weapon-buff-mark-smite` | `947f76c009833c8a0702e78956accfd1a3417da9` | `925c298ac923b894891a34331865839f5a1be371` | removable-after-operator-confirms-no-local-use |
| L15-RR07-FU01D-PROTECTION-CHARM-WARD-SUBSTRATES | `/workspace/typescript/.codex-worktrees/dnd-cleanroom-l15-rr07-fu01d` | `ralph/l15-rr07-fu01d-protection-charm-ward` | `61a1bc984aa104b439c4adf3d2e43b0ad57e1674` | `91e141c8998682ebf6daf65e0ff5594aaf24551e` | removable-after-operator-confirms-no-local-use; zero accepted coverage, source-QNT connector blocker recorded |
