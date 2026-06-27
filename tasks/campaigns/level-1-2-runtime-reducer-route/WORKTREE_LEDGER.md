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
| _none_ | | | | | |

## Active Fresh Dry Run Targets

| Phase | Path | Branch | Input Commit | Worker | Reviewer | Status |
| --- | --- | --- | --- | --- | --- | --- |
| _none_ | | | | | | |

## Active Fresh Expansion Targets

| Lane | Path | Branch | Base SHA | Worker | Reviewer | Status |
| --- | --- | --- | --- | --- | --- | --- |
| FEXP-01-DIAGNOSTIC-BATTLE-ROUTE-PACK | `/workspace/typescript/.codex-worktrees/dnd-fresh-cleanroom-fexp-01` | `ralph/fexp-01-diagnostic-battle-route-pack` | `574c99d28ef2fe8779c500dad34879efa7aa4177` | Mencius the 2nd (`019f0acb-0a8f-79b0-a25a-d17dc816d6cd`) | pending | running |

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
| FC-06-SOURCE-FEEDBACK-AUDIT | `/workspace/typescript/.codex-worktrees/dnd-cleanroom-rrconv-19` | `ralph/rrconv-19-cleanroom` | recorded in final FC-06 commit | n/a | complete; artifact `tasks/campaigns/level-1-2-runtime-reducer-route/FC06_SOURCE_FEEDBACK.md` |
| PACT-SLOT-HANDOFF-INIT-PROJECTION-ROUTE-REFRESH | `/workspace/typescript/.codex-worktrees/dnd-cleanroom-pact-slot-replay` | `ralph/pact-slot-handoff-replay` | `23eca8a08e055864630161ad953c65f00fb9027b` | `1abe8f87fcff9e72efd2e006f549373ff9f00b83` | removable; dirty diagnostic replay only |
| L15-RRCP8-A-MAGE-ARMOR-GENERIC-AC-ROUTES | `/workspace/typescript/.codex-worktrees/dnd-cleanroom-l15-rrcp8-a` | `ralph/l15-rrcp8-a-mage-armor-generic-ac` | `a8b96ec0aa91b4249b352009b1384292f917f7e2` | `5b1e976b6af7fadefa4ea5a065ae81de53b94d09` | removable |
| L15-RR04C-RULE-CORE-SPELL-ABILITY-COMPONENTS | `/workspace/typescript/.codex-worktrees/dnd-cleanroom-l15-rr04c` | `ralph/l15-rr04c-rule-core-spell-ability` | `8b3bd015c34357b38962bfaada4e6f1d0ba3d500` | `ee30e831b0bc0fa49fa54100e54a45c32a43a60a` | removable |
| L15-RR04D-RULE-CORE-FEATURE-PROFILE-COMPONENTS | `/workspace/typescript/.codex-worktrees/dnd-cleanroom-l15-rr04d` | `ralph/l15-rr04d-rule-core-feature-profile` | `3b522e37510064953d299db7cc4a739af9cd9d04` | `d0af3dc` | removable |
| L15-RR04B-RULE-CORE-MOVEMENT-REACTION-SHOVE-COMPONENTS | `/workspace/typescript/.codex-worktrees/dnd-cleanroom-l15-rr04b` | `ralph/l15-rr04b-rule-core-movement-reaction-shove` | `22e6ff0ef095602ce05037382499f8715d0cef8e` | `727655c` | removable |
| L15-RR04A-RULE-CORE-DAMAGE-STATBLOCK-COMPONENTS | `/workspace/typescript/.codex-worktrees/dnd-cleanroom-l15-rr04a` | `ralph/l15-rr04a-rule-core-damage-statblock` | `9d3ee41d081b7cade2daaba50c0730b412b5fc92` | `b20ce8ff40dc438d93d9e09582078af4d0fa8e24` | removable |
| L15-RR03-FINISH-CURRENT-DIAGNOSTIC-QUEUE | `/workspace/typescript/.codex-worktrees/dnd-cleanroom-l15-rr03` | `ralph/l15-rr03-diagnostic-queue` | `7ef32d308d51fb54d1032d01b937d168fa63bb64` | `4c7e12d7645360adb7ab23af61144ceb243c13fe` | removable |
| L15-RR05-BATTLE-ACTION-ATTACK-STATBLOCK-ROUTES | `/workspace/typescript/.codex-worktrees/dnd-cleanroom-l15-rr05` | `ralph/l15-rr05-attack-statblock` | `1b928b16bfed2c87ad95efb6aae0a5d384fdb903` | `a235602664bbae19c3bfac5e38b85b1bbc4c23a5` | removable |
| L15-RR06-BATTLE-SPELL-EFFECT-ROUTES | `/workspace/typescript/.codex-worktrees/dnd-cleanroom-l15-rr06` | `ralph/l15-rr06-spell-effect-routes` | `9d17264679d8207c716f51148c52418629684891` | `1aa2ff3c6e4ca9d466a8eb0b8bc312ad3eeda025` | removable |
| L15-RR08-CHARACTER-CREATION-ROUTES | `/workspace/typescript/.codex-worktrees/dnd-cleanroom-l15-rr08` | `ralph/l15-rr08-character-creation` | `e11862d68ab0b02ec0db303504923e9222c3446f` | `76f2c5c61c1553c002dcd3f026a6ac9f444d1fdf` | removable |
| L15-RR09-CHARACTER-SHEET-ROUTES | `/workspace/typescript/.codex-worktrees/dnd-cleanroom-l15-rr09` | `ralph/l15-rr09-character-sheet` | `410e3db7d181c7fba9265cc753ab977198dacca7` | `616b6c27104807a0cf312d4cd68485e0c7fef1c3` | removable |
| L15-RR10-CHARACTER-BATTLE-HANDOFF-ROUTES | `/workspace/typescript/.codex-worktrees/dnd-cleanroom-l15-rr10` | `ralph/l15-rr10-character-battle-handoff` | `4ea5566fc503fcfa17f430b523faa87dc189943c` | `3a0c18402e5620a55d0f2046329ab5697023c2e3` | removable |
| L15-RR07S-B-ZERO-HP-FORM-MOVEMENT-SUBSTRATES | `/workspace/typescript/.codex-worktrees/dnd-cleanroom-l15-rr07s-b` | `ralph/l15-rr07s-b-zero-hp-form-movement` | `81fd387c6dda4b00c28a752492d6aa0d2fee4d3d` | `0ef395a` | removable |
| L15-RR07S-C-WEAPON-BREATH-FEATURE-SUBSTRATES | `/workspace/typescript/.codex-worktrees/dnd-cleanroom-l15-rr07s-c` | `ralph/l15-rr07s-c-weapon-breath-feature` | `6d89dd42b345c3253d36759d723c1fe4a3271c2c` | `67bc22f` | removable |
| L15-RR07S-A-PASSIVE-ROLL-RESOURCE-SUBSTRATES | `/workspace/typescript/.codex-worktrees/dnd-cleanroom-l15-rr07s-a` | `ralph/l15-rr07s-a-passive-roll-resource` | `582b2a71c6573c480d4ccddec15381308a2bd667` | `1b0d0dbc1615de1efdafce3f74b1b6372e2df8d9` | removable |
| L15-RR07-FU08A-METAMAGIC-GOVERNOR-QUICKENED-SUBSTRATE | `/workspace/typescript/.codex-worktrees/dnd-cleanroom-l15-rr07-fu08a` | `ralph/l15-rr07-fu08a-metamagic-governor` | `07e47f01a9e728fcfe919b4af2e1a4592b97069f` | `b909dfdaa01bca96953a590b9984518bd5e9bc68` | removable |
| L15-RR07-FU01A-CATALOG-READY-SPELL-SUBSTRATES | `/workspace/typescript/.codex-worktrees/dnd-cleanroom-l15-rr07-fu01a` | `ralph/l15-rr07-fu01a-catalog-ready` | `86290c012cdefb8afdcc526a8caa58da190b407b` | `5a6e5d2d1975788acef1373786eb94dd0074407e` | removable |
| L15-RR07-FU08B-METAMAGIC-SAVE-RANGE-TARGET-SUBSTRATES | `/workspace/typescript/.codex-worktrees/dnd-cleanroom-l15-rr07-fu08b` | `ralph/l15-rr07-fu08b-metamagic-save-range` | `3727abed465765fbc2b3f223076ed265d31393ff` | `20daf3ede80af636df4e71a776d95783182fef2d` | removable |
| L15-RR07-FU08C-METAMAGIC-REROLL-DAMAGE-PROJECTION-SUBSTRATES | `/workspace/typescript/.codex-worktrees/dnd-cleanroom-l15-rr07-fu08c` | `ralph/l15-rr07-fu08c-metamagic-reroll-damage` | `a0ac254c610b4c0e64b4f6d160c0ef533f400a40` | `5bec99231b38d67c5930922cb77328d2b71c031a` | removable |
| L15-RR07-FU01F-SPATIAL-LIGHT-AREA-MOVEMENT-SUBSTRATES | `/workspace/typescript/.codex-worktrees/dnd-cleanroom-l15-rr07-fu01f` | `ralph/l15-rr07-fu01f-spatial-light-area` | `15cfbe5193ed93e6119873732cccfa93c7f37349` | `78be1eccb627f3ae7987380922d873b9ca7e497b` | removable |
| L15-RR07-FU01B-SPELL-ATTACK-SAVE-DAMAGE-SUBSTRATES | `/workspace/typescript/.codex-worktrees/dnd-cleanroom-l15-rr07-fu01b` | `ralph/l15-rr07-fu01b-spell-attack-damage` | `195a473783bc666f5413dabe0a2bf93ea18003df` | `6834ee18356e01eb9b00bd4b32f0169b75a7220d` | removable |
| L15-RR07-FU01E-ARMOR-CLASS-REACTION-SUBSTRATES | `/workspace/typescript/.codex-worktrees/dnd-cleanroom-l15-rr07-fu01e` | `ralph/l15-rr07-fu01e-armor-reaction` | `c135ceb24f7b0f44d93f14763d55dbb9aaa8cf4f` | `4f69ca5c407cbc53c5e3a5a431cedf0f1d3148c9` | removable |
| L15-RR07-FU01C-WEAPON-BUFF-MARK-SMITE-SUBSTRATES | `/workspace/typescript/.codex-worktrees/dnd-cleanroom-l15-rr07-fu01c` | `ralph/l15-rr07-fu01c-weapon-buff-mark-smite` | `947f76c009833c8a0702e78956accfd1a3417da9` | `925c298ac923b894891a34331865839f5a1be371` | removable |
| L15-RR07-FU01D-PROTECTION-CHARM-WARD-SUBSTRATES | `/workspace/typescript/.codex-worktrees/dnd-cleanroom-l15-rr07-fu01d` | `ralph/l15-rr07-fu01d-protection-charm-ward` | `61a1bc984aa104b439c4adf3d2e43b0ad57e1674` | `91e141c8998682ebf6daf65e0ff5594aaf24551e` | removable; zero accepted coverage, source-QNT connector blocker recorded |
| L15-RRCP5-F-INDEPENDENT-SPELL-ATTACK-SEQUENCE-ROUTES | `/workspace/typescript/.codex-worktrees/dnd-cleanroom-l15-rrcp5-f` | `ralph/l15-rrcp5-f-spell-attack-sequence` | `4e84842fa5a27bb1db91cf855a76d5e9cc9860da` | `964d6672c999db82c4d297543315a65d054f7703` | removable |
| L15-RRCP5-D-COMPANION-OBJECT-BOUNDARY-ROUTES | `/workspace/typescript/.codex-worktrees/dnd-cleanroom-l15-rrcp5-d` | `ralph/l15-rrcp5-d-companion-object` | `122431e66c696399d504924546879dd2da3d7e90` | `5818c1573fae64e63fbab82900b3f41dc06576d1` | removable |
| L15-RRCP5-B-ACTIVE-EFFECT-LIFECYCLE-ROUTES | `/workspace/typescript/.codex-worktrees/dnd-cleanroom-l15-rrcp5-b` | `ralph/l15-rrcp5-b-active-effects` | `83cf6603eca84c380fd434150b662c4341d45126` | `09414700e40d3f4c1818fdf10aa42f7bff69ff71` | removable |
| L15-RRCP5-A-RIDER-AND-WEAPON-HOSTED-ROUTES | `/workspace/typescript/.codex-worktrees/dnd-cleanroom-l15-rrcp5-a` | `ralph/l15-rrcp5-a-rider-weapon` | `5ecdd9ef43f9598fe19fdda58fcab4596c0c68c2` | `a0a88e469f71f5069f4f01c19bbdbc2745660155` | removable |
| L15-RRCP5-C-REACTION-INTERRUPT-ROUTES | `/workspace/typescript/.codex-worktrees/dnd-cleanroom-l15-rrcp5-c` | `ralph/l15-rrcp5-c-reactions` | `6300fce274c2c9774e0e159b068463d50a8958e2` | `97763b0f981424c462d7af86f0dd0463dd1b9012` | removable |
| L15-RRCP5-E-ABILITY-SEARCH-CHOICE-ROUTES | `/workspace/typescript/.codex-worktrees/dnd-cleanroom-l15-rrcp5-e` | `ralph/l15-rrcp5-e-ability-search` | `3dd749a6440a073c10d303d56e05b8ad40af024c` | `e1cb8616d0a3fbe2aa59a69d7eeba28da6e7622d` | removable |
| L15-RRCP7-A-BUFF-MARK-ACTIVE-EFFECT-ROUTES | `/workspace/typescript/.codex-worktrees/dnd-cleanroom-l15-rrcp7-a` | `ralph/l15-rrcp7-a-buff-mark-active-effects` | `a62478648dde859d7e814e1c99d8bdf7987b4af6` | `f8a438ebff90eb79e274b207c5c286080aaf6726` | removable |
| L15-RRCP7-B-DAMAGE-SPELL-RESIDUAL-BRIDGES | `/workspace/typescript/.codex-worktrees/dnd-cleanroom-l15-rrcp7-b` | `ralph/l15-rrcp7-b-damage-spell-bridges` | `2741da1671caa183a4907e6eec54de26c744d7e8` | `b33d9bbc1080c0d7a96ab606da07d0f517417096` | removable |
| L15-RRCP7-C-WEAPON-PREHIT-ATTACK-SETUP-ROUTES | `/workspace/typescript/.codex-worktrees/dnd-cleanroom-l15-rrcp7-c` | `ralph/l15-rrcp7-c-weapon-prehit` | `855e702770866c18ae5b657367b95af8410efbe9` | `554e2e2c0adae1127496d4a12023ae5ac8979f88` | removable |
| L15-RRCP7-D-MAGE-ARMOR-ADMISSION-LIFECYCLE-ROUTES | `/workspace/typescript/.codex-worktrees/dnd-cleanroom-l15-rrcp7-d` | `ralph/l15-rrcp7-d-mage-armor` | `61b1534b2cfc9b3d403ca358864beeb6266c0e66` | `ffc162a61eeea40b0e922b28c7cdeff95281728d` | removable; zero accepted coverage, Mage Armor route connector blocker recorded |
| L15-RRCP7-E-MAGIC-WEAPON-ITEM-TARGET-ROUTES | `/workspace/typescript/.codex-worktrees/dnd-cleanroom-l15-rrcp7-e` | `ralph/l15-rrcp7-e-magic-weapon` | `d08c66e271cae1d4f3ddd5eb1ca101ad5b11c14c` | `24ad8f3b493b2affa4dba3607a619f92bf54eb16` | removable |

## Completed Fresh Dry Run Targets

| Phase | Path | Branch | Target Commit | Input Commit | Worker | Reviewer | Status |
| --- | --- | --- | --- | --- | --- | --- | --- |
| FC-00-FRESH-PACKAGE-ADMISSION | `/workspace/typescript/.codex-worktrees/dnd-fresh-cleanroom-dry-run-fc00` | standalone fresh target `master` | `cd2e1ffaf726bd3d315c0c4babf3a498ce3f89a3` | `de7c090391ea62931d27640b6ad76015b65c88c5` | Dewey the 2nd (`019f09a7-f614-70a3-9397-b8fca74e1979`) | James the 2nd (`019f09ad-f0e6-7152-a82d-66ce21331f75`) | FC-00 accepted; ready for FC-01 |
| FC-01-REDUCER-SPINE-SUBSTRATE | `/workspace/typescript/.codex-worktrees/dnd-fresh-cleanroom-dry-run-fc00` | standalone fresh target `master` | `1f26d3e4fc792912e9b84f8e2cf242f3869c3807` | `a58f893` campaign / `cd2e1ffaf726bd3d315c0c4babf3a498ce3f89a3` target | Zeno the 2nd (`019f09b1-9bad-7da0-ab9b-5b6b9e3dd2c9`) | Dalton the 2nd (`019f09cd-58b1-7e41-b16c-845ac7fb3035`) | FC-01 accepted after two route-honesty lifecycle fixes; ready for FC-02 |
| FC-02-MINIMAL-BATTLE-ACTION-ROUTE | `/workspace/typescript/.codex-worktrees/dnd-fresh-cleanroom-dry-run-fc00` | standalone fresh target `master` | `9b4232da35ca3532ac6815475c2998a21c734452` | `74d38f9` campaign / `1f26d3e4fc792912e9b84f8e2cf242f3869c3807` target | Noether the 2nd (`019f09d2-d8b3-7fb2-a66c-b30dccae865e`) | Sagan the 2nd (`019f09dd-a4cf-7761-8763-876b4888bf79`) | FC-02 accepted; ready for FC-03 |
| FC-03-SPELL-CONNECTOR | `/workspace/typescript/.codex-worktrees/dnd-fresh-cleanroom-dry-run-fc00` | standalone fresh target `master` | `7b690c80e6fa4f3177ff41ccedf2083175b1b00c` | `c62945c3cf70653db7aa363ebbaeb945de04d4ec` campaign / `9b4232da35ca3532ac6815475c2998a21c734452` target | Volta the 2nd (`019f09e3-eb5a-7843-983c-df5cee14caef`) | Faraday the 2nd (`019f09ee-0dae-7603-8190-9030e94ed671`) | FC-03 accepted-with-blockers; ready for FC-04 |
| FC-04-GENERIC-BASE-ARMOR-CLASS | `/workspace/typescript/.codex-worktrees/dnd-fresh-cleanroom-dry-run-fc00` | standalone fresh target `master` | `101603476fd473483f4480f55bea9c3555d9aefa` | `f487298` campaign / `7b690c80e6fa4f3177ff41ccedf2083175b1b00c` target | Socrates the 2nd (`019f09f3-de14-78c3-b696-a672305e19c0`) | Sartre the 2nd (`019f0a02-f2c7-7253-b65d-8fddd58a642c`) | FC-04 accepted; ready for FC-05 |
| FC-05-CHARACTER-BATTLE-INIT-PROJECTION | `/workspace/typescript/.codex-worktrees/dnd-fresh-cleanroom-dry-run-fc00` | standalone fresh target `master` | `380cf29e49d509bc0f246cf9aa022f226e42efd8` | `85b6dea` campaign / `101603476fd473483f4480f55bea9c3555d9aefa` target | Godel the 2nd (`019f0a07-f2ad-7f72-985f-dad61f35ea1e`) | Arendt the 2nd (`019f0a17-0887-76e3-8163-c569e61faf54`) | FC-05 accepted-with-limitations after one fix round; runtime/evidence commit `63e6fb93a67b17d8bf2d4fbb0b44bd644c0cf419`; ready for tracer-bullet gate and FC-06 |
| SDK-TRACER-BULLET-PROGRAMMATIC-SURFACE | `/workspace/typescript/.codex-worktrees/dnd-fresh-cleanroom-dry-run-fc00` | standalone fresh target `master` | `893198ce66a35c8aad007ad8ac7a61c4631c64d9` | `0387d29f9282037637b4256c3c7f292bab7ef85c` source / `e6394e544900ad5e851e918cf1980a7b5b1f78cd` target | Popper the 2nd (`019f0a55-85f0-76b3-9cc2-228e24c200c9`) | orchestrator review | accepted; full integrated sheet-to-composed-encounter-to-simple-turn scenario passes; implementation evidence commit `ed768b281796887341cbed8eead4a80c05bbd09c`; metadata commit `893198ce66a35c8aad007ad8ac7a61c4631c64d9` |
| FC-07-PACT-SLOT-HANDOFF-REPLAY | `/workspace/typescript/.codex-worktrees/dnd-fresh-cleanroom-dry-run-fc00`; removed replay branch worktree `/workspace/typescript/.codex-worktrees/dnd-fresh-cleanroom-pact-slot-replay` | standalone fresh target `master` / removed `ralph/fresh-pact-slot-replay` worktree | `f0ee8f8eb95192639afe5b6af17764dfe46c5303` | `b57772b459f1b75592fd45b9196fd60965b534d3` source / `8ec4bc46bfd961345c4f73115d8dc523c5d9163b` setup | Halley the 2nd (`019f0a88-a545-76e1-9b1b-ee15007116e1`) | orchestrator review | accepted; pure Pact Slot handoff projection and mixed ordinary/Pact rejection replayed from refreshed QNT connector; replay worktree removed after merge |
| FC-08-FRESH-VERIFIER-REFRESH | `/workspace/typescript/.codex-worktrees/dnd-fresh-cleanroom-dry-run-fc00`; removed verifier branch worktree `/workspace/typescript/.codex-worktrees/dnd-fresh-cleanroom-verifier-refresh` | standalone fresh target `master` / removed `ralph/fresh-verifier-refresh` worktree | `a30e6729711ddc3f595cf008931ba5cd6265c58a` | `b57772b459f1b75592fd45b9196fd60965b534d3` source / `f0ee8f8eb95192639afe5b6af17764dfe46c5303` target | Ampere the 2nd (`019f0aa1-145d-7c41-bfff-114cb4d6427d`) plus Harvey the 2nd (`019f0aab-ccd3-70d3-9438-5cbe58724ba1`) | orchestrator review | accepted; current verifier validates b57772b inventory, strict FC-07 hashes, SDK tracer surface, and FC-03/FC-04/FC-05 historical snapshot classification; verifier worktree removed after merge |

## Completed Fresh Expansion Targets

| Lane | Path | Branch | Target Commit | Base Commit | Worker | Reviewer | Status |
| --- | --- | --- | --- | --- | --- | --- | --- |
| FEXP-00-BASELINE-LOCK | `/workspace/typescript/.codex-worktrees/dnd-fresh-cleanroom-dry-run-fc00`; removed branch worktree `/workspace/typescript/.codex-worktrees/dnd-fresh-cleanroom-fexp-00` | standalone fresh target `master` / removed `ralph/fexp-00-baseline-lock` worktree | `574c99d28ef2fe8779c500dad34879efa7aa4177` | `a30e6729711ddc3f595cf008931ba5cd6265c58a` | Kant the 2nd (`019f0abd-0855-7ba1-9b55-eb0c84d83f7c`) | Newton the 2nd (`019f0ac0-b67b-71e1-87b8-1e4087aad7db`) | accepted and merged; baseline expansion state only, no runtime coverage claimed; branch worktree removed after merge |
