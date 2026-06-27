# Checkpoint Report

Campaign: `level-1-2-runtime-reducer-route`

## Bootstrap Snapshot

- Integration worktree: `/workspace/typescript/.codex-worktrees/dnd-cleanroom-rrconv-19`
- Integration branch: `ralph/rrconv-19-cleanroom`
- Bootstrap HEAD: `6e3ec7c4fff70a28a4ab29cfebeaf9133daec4f0`
- Lane base policy: resolve from integration worktree `HEAD` at lane launch.
- Denominator: `97` driver files, `668` canonical in-scope obligations.
- Accepted coverage at bootstrap: `7` drivers, `52` obligations.
- Coverage percentages: `7.2%` by driver, `7.8%` by obligation.

## Checkpoint Status

| Checkpoint | Status | Notes |
| --- | --- | --- |
| CP0 Bootstrap | complete | Source baseline pinned at `6e3ec7c4fff70a28a4ab29cfebeaf9133daec4f0`; campaign-control files live on the integration branch. |
| CP1 Battle Reducer Core Expansion | complete | `L15-RR03`, `L15-RR05`, and `L15-RR06` merged and verified. |
| CP2 Rule-Core Component Connectors | complete | Four `L15-RR04` component sublanes merged and verified. |
| CP3 Character Creation, Sheet, And Handoff | complete | RR08 creation, RR09 sheet, and RR10 handoff merged and verified. |
| CP4 Feature And Catalog Substrates | running | CP4 small feature substrate batch A/B/C is merged and verified. FU01 and FU08 still need dedicated split planning before execution, so parent CP4 remains open. |
| CP5 Remaining Battle Families | blocked-on-checkpoint | Depends on CP1, CP2, CP4. |
| CP6 Closure Sweep | blocked-on-checkpoint | Runs after all implementation checkpoints. |

## Last Known Verification

At `1b0d0dbc1615de1efdafce3f74b1b6372e2df8d9`:

- `cargo fmt --check`: pass
- `cargo test`: pass, `207 passed`
- `cargo clippy --all-targets -- -D warnings`: pass
- `node scripts/check-cleanroom-harness.cjs`: pass
- `git diff --check HEAD~1...HEAD`: pass
- `cargo test adapter_replays_all_branches --quiet`: pass, `76 passed`

## Active Work

- `L15-RR03-FINISH-CURRENT-DIAGNOSTIC-QUEUE` merged at `4c7e12d7645360adb7ab23af61144ceb243c13fe`. Lane head `7ef32d308d51fb54d1032d01b937d168fa63bb64` includes reducer-route evidence for death saving throw and concentration teardown plus the harness quarantine fix.
- `L15-RR05-BATTLE-ACTION-ATTACK-STATBLOCK-ROUTES` merged at `a235602664bbae19c3bfac5e38b85b1bbc4c23a5`. Lane head `1b928b16bfed2c87ad95efb6aae0a5d384fdb903` covers seven RR05 drivers and 53 route obligations.
- `L15-RR06-BATTLE-SPELL-EFFECT-ROUTES` merged at `1aa2ff3c6e4ca9d466a8eb0b8bc312ad3eeda025`. Lane head `9d17264679d8207c716f51148c52418629684891` covers command/scalar spell-effect routes and closes CP1.
- CP2 completed:
  - `L15-RR04A-RULE-CORE-DAMAGE-STATBLOCK-COMPONENTS` merged at `b20ce8ff40dc438d93d9e09582078af4d0fa8e24`
  - `L15-RR04B-RULE-CORE-MOVEMENT-REACTION-SHOVE-COMPONENTS` merged at `727655c`
  - `L15-RR04D-RULE-CORE-FEATURE-PROFILE-COMPONENTS` merged at `d0af3dc`
  - `L15-RR04C-RULE-CORE-SPELL-ABILITY-COMPONENTS` merged at `ee30e831b0bc0fa49fa54100e54a45c32a43a60a`
- CP3 running:
  - `L15-RR08-CHARACTER-CREATION-ROUTES` launched in `/workspace/typescript/.codex-worktrees/dnd-cleanroom-l15-rr08` with worker Harvey `019f06a6-8ed9-7560-9daa-6922b4bb277d`
  - `L15-RR09-CHARACTER-SHEET-ROUTES` launched in `/workspace/typescript/.codex-worktrees/dnd-cleanroom-l15-rr09` with worker Volta `019f06a6-f828-76d0-b989-42488be5d4b9`
  - `L15-RR08-CHARACTER-CREATION-ROUTES` fixer Kierkegaard `019f06bd-b6ee-7771-81e1-d412340ae0e9` committed `e11862d68ab0b02ec0db303504923e9222c3446f`; re-reviewer Meitner `019f06cd-b784-7a62-99fe-39612ff205f4` returned clean against Anscombe's blocker findings.
  - `L15-RR09-CHARACTER-SHEET-ROUTES` fixer Aristotle `019f06b9-b640-7042-ab31-0bec31939b48` committed `410e3db7d181c7fba9265cc753ab977198dacca7`; re-reviewer Jason `019f06c9-bc48-74b1-87dd-8312784588dd` returned clean against Pauli's three blocker findings.
  - RR08 and RR09 overlap in `src/tests/mod.rs`, task artifacts, and `src/qnt_adapters/character_sheet_weapon_mastery_containers_selected_identity.rs`; merge only after RR08 re-review and an explicit conflict review.
  - `L15-RR08-CHARACTER-CREATION-ROUTES` merged at `76f2c5c61c1553c002dcd3f026a6ac9f444d1fdf` and passed integration verification: `cargo fmt --check`, `node scripts/check-cleanroom-harness.cjs`, `git diff --check HEAD~1...HEAD`, `cargo test` (`199 passed`), `cargo clippy --all-targets -- -D warnings`, and `cargo test adapter_replays_all_branches --quiet` (`72 passed`).
  - `L15-RR09-CHARACTER-SHEET-ROUTES` merged at `616b6c27104807a0cf312d4cd68485e0c7fef1c3` after resolving shared task-artifact conflicts by preserving cumulative ledger evidence and both validation sections. Integration verification passed: `cargo fmt --check`, `node scripts/check-cleanroom-harness.cjs`, `git diff --check HEAD~1...HEAD`, `cargo test` (`202 passed`), `cargo clippy --all-targets -- -D warnings`, and `cargo test adapter_replays_all_branches --quiet` (`75 passed`).
  - `L15-RR10-CHARACTER-BATTLE-HANDOFF-ROUTES` launched in `/workspace/typescript/.codex-worktrees/dnd-cleanroom-l15-rr10` from base `6af492188311839dd4839b464b2e7049e3330568` with worker Darwin `019f06d8-d9f9-76b1-a057-efe51b02c54f`.
  - `L15-RR10-CHARACTER-BATTLE-HANDOFF-ROUTES` worker Darwin committed `421541a1958b20bdfb3b785174ea125673135d72`; reviewer Bacon `019f06f4-070e-7dd1-9cc4-7c6567885937` found blockers. Observed and expected route witnesses share construction helpers across the changed handoff adapters, so the route checks are not independent enough to support the accepted evidence.
  - RR10 fixer Plato `019f06f8-3a2a-7863-8286-a48ce02dd0d4` committed `4ea5566fc503fcfa17f430b523faa87dc189943c`; re-reviewer Hilbert `019f0701-6e43-7150-aff6-d5cc6067bf7a` returned clean against Bacon's independence findings.
  - `L15-RR10-CHARACTER-BATTLE-HANDOFF-ROUTES` merged at `3a0c18402e5620a55d0f2046329ab5697023c2e3` and passed integration verification: `cargo fmt --check`, `node scripts/check-cleanroom-harness.cjs`, `git diff --check HEAD~1...HEAD`, `cargo test` (`202 passed`), `cargo clippy --all-targets -- -D warnings`, and `cargo test adapter_replays_all_branches --quiet` (`75 passed`).
  - CP3 is complete. CP4 is now ready.
- CP4 small feature substrate batch merged:
  - `L15-RR07S-B-ZERO-HP-FORM-MOVEMENT-SUBSTRATES` merged at `0ef395a`; lane head `81fd387c6dda4b00c28a752492d6aa0d2fee4d3d`.
  - `L15-RR07S-C-WEAPON-BREATH-FEATURE-SUBSTRATES` merged at `67bc22f`; lane head `6d89dd42b345c3253d36759d723c1fe4a3271c2c`.
  - `L15-RR07S-A-PASSIVE-ROLL-RESOURCE-SUBSTRATES` merged at `1b0d0dbc1615de1efdafce3f74b1b6372e2df8d9`; lane head `582b2a71c6573c480d4ccddec15381308a2bd667`.
  - Integration verification passed: `cargo fmt --check`, `node scripts/check-cleanroom-harness.cjs`, `git diff --check HEAD~1...HEAD`, `cargo test` (`207 passed`), `cargo clippy --all-targets -- -D warnings`, and `cargo test adapter_replays_all_branches --quiet` (`76 passed`).
  - CP4 remains running because FU01 and FU08 are still split/blocker-planning lanes.

## Coverage Delta Log

Append one section per merged lane.

Template:

```md
### <lane id>

- Merge commit:
- Lane commit(s):
- Drivers added:
- Obligations added:
- New total driver coverage:
- New total obligation coverage:
- Integration verification:
- Review/fixer notes:
- Worktrees marked removable:
```

### L15-RR03-FINISH-CURRENT-DIAGNOSTIC-QUEUE

- Merge commit: `4c7e12d7645360adb7ab23af61144ceb243c13fe`
- Lane commit(s): `7b0997b0ad8f76a99d0a905a0e68ef679eecccde`, `155ca2bed52639f3de011a2a22de2b0f63d6c318`, `7ef32d308d51fb54d1032d01b937d168fa63bb64`
- Drivers added: `0` net-new unique drivers; the two selected drivers already had earlier diagnostic evidence.
- Obligations added: `0` net-new counted obligations; `8` existing diagnostic obligations received reducer-route evidence and refreshed artifacts.
- New total driver coverage: `7 / 97 = 7.2%`
- New total obligation coverage: `52 / 668 = 7.8%`
- Integration verification: `cargo fmt --check`, `cargo test`, `cargo clippy --all-targets -- -D warnings`, `node scripts/check-cleanroom-harness.cjs`, JSON parse checks, and `git diff --check HEAD~1...HEAD` passed.
- Review/fixer notes: first review found `battle_reducer_spine.rs` incorrectly listed as an adapter/quarantine module; first fix removed that inconsistency; re-review found sampled-pick adapter quarantine enforcement weakened; second fix restored full adapter-quarantine enforcement.
- Worktrees marked removable: `/workspace/typescript/.codex-worktrees/dnd-cleanroom-l15-rr03`

### L15-RR05-BATTLE-ACTION-ATTACK-STATBLOCK-ROUTES

- Merge commit: `a235602664bbae19c3bfac5e38b85b1bbc4c23a5`
- Lane commit(s): `76e8bd99e1d705cb31e9180bf831fd1625410ed5`, `4256dd5e7cdb0d0693a32125b1ec3cf41befbf32`, `97acec8abd003763e71346e789e6699a432e41df`, `1b928b16bfed2c87ad95efb6aae0a5d384fdb903`
- Drivers added: `6` net-new unique drivers; `battle-runtime-weapon-attack-skeleton.mbt.qnt` was already counted.
- Obligations added: `39` net-new counted obligations; total accepted obligations moved from `52` to `91`.
- New total driver coverage: `13 / 97 = 13.4%`
- New total obligation coverage: `91 / 668 = 13.6%`
- Integration verification: `cargo fmt --check`, `cargo test`, `cargo clippy --all-targets -- -D warnings`, `node scripts/check-cleanroom-harness.cjs`, `cargo test adapter_replays_all_branches --quiet`, JSON parse checks, and `git diff --check HEAD~1...HEAD` passed.
- Review/fixer notes: review loop fixed missing RR05 evidence, creature/spell/stat-block reducer routing gaps, self-referential route expectations, sampled-pick quarantine regression, RR03 artifact integration, and stat-block `BattleResolutionResult` needs-holes/discovery-route contract issues.
- Worktrees marked removable: `/workspace/typescript/.codex-worktrees/dnd-cleanroom-l15-rr05`

### L15-RR06-BATTLE-SPELL-EFFECT-ROUTES

- Merge commit: `1aa2ff3c6e4ca9d466a8eb0b8bc312ad3eeda025`
- Lane commit(s): `f0e6057dedc100f253a5bec4d34f13bd3c2ca8da`, `0b1dbfb3d7ace478fcf389d076870c9907e7ff27`, `0bc81aa0ffdf97f36ff2f054751afd299ad6c7f5`, `dd616f0d1d94ad0a5bfb78c1a830aef0bc8a6074`, `729cf31d8b1e02a33b897f24288e58f64aff4bb2`, `6a84db8e051e1f9a045f51bf1164a87103e4591f`, `9d17264679d8207c716f51148c52418629684891`
- Drivers added: `3` net-new unique drivers.
- Obligations added: `36` net-new counted obligations; total accepted obligations moved from `91` to `127`.
- New total driver coverage: `16 / 97 = 16.5%`
- New total obligation coverage: `127 / 668 = 19.0%`
- Integration verification: `cargo fmt --check`, `cargo test`, `cargo clippy --all-targets -- -D warnings`, `node scripts/check-cleanroom-harness.cjs`, `cargo test adapter_replays_all_branches --quiet`, JSON parse checks, and `git diff --check HEAD~1...HEAD` passed.
- Review/fixer notes: review loop removed command fixture-role fallbacks, made route equality include `BattleResolutionResult` outcome, replaced self-referential command/scalar expected routes, corrected state-owner and validation artifacts, added a mechanical copied-QNT connector check for disputed command route examples, and removed scalar-buff subject fallback.
- Worktrees marked removable: `/workspace/typescript/.codex-worktrees/dnd-cleanroom-l15-rr06`

### CP2: L15-RR04 Rule-Core Component Connectors

- Merge commits: `b20ce8ff40dc438d93d9e09582078af4d0fa8e24`, `727655c`, `d0af3dc`, `ee30e831b0bc0fa49fa54100e54a45c32a43a60a`
- Lane heads: `9d3ee41d081b7cade2daaba50c0730b412b5fc92`, `22e6ff0ef095602ce05037382499f8715d0cef8e`, `3b522e37510064953d299db7cc4a739af9cd9d04`, `8b3bd015c34357b38962bfaada4e6f1d0ba3d500`
- Drivers added: `9` net-new unique rule-core component drivers.
- Obligations added: `129` net-new counted obligations; total accepted obligations moved from `127` to `256`.
- New total driver coverage: `25 / 97 = 25.8%`
- New total obligation coverage: `256 / 668 = 38.3%`
- Integration verification: `cargo fmt --check`, `cargo test`, `cargo clippy --all-targets -- -D warnings`, `node scripts/check-cleanroom-harness.cjs`, `cargo test adapter_replays_all_branches --quiet`, JSON parse checks, and `git diff --check HEAD~1...HEAD` passed.
- Review/fixer notes: RR04A and RR04B were accepted after fixing overclaimed state ownership, stale reports, self-referential witnesses, fixture identity naming, and duplicate evidence refs. RR04D was accepted only as adapter-only component projection evidence, with no production reusable feature-component semantics claimed. RR04C added shared route facts in `src/rules/rule_core_components.rs`, independent literal expected witnesses, and split accepted spell rows from non-acceptance regression rows.
- Worktrees marked removable: `/workspace/typescript/.codex-worktrees/dnd-cleanroom-l15-rr04a`, `/workspace/typescript/.codex-worktrees/dnd-cleanroom-l15-rr04b`, `/workspace/typescript/.codex-worktrees/dnd-cleanroom-l15-rr04c`, `/workspace/typescript/.codex-worktrees/dnd-cleanroom-l15-rr04d`

### L15-RR08-CHARACTER-CREATION-ROUTES

- Merge commit: `76f2c5c61c1553c002dcd3f026a6ac9f444d1fdf`
- Lane commit(s): `8416ee533fed413f9f072b982e0d425d89d90bac`, `e11862d68ab0b02ec0db303504923e9222c3446f`
- Drivers added: `8` net-new unique character-creation drivers.
- Obligations added: `55` net-new counted obligations; total accepted obligations moved from `256` to `311`.
- New total driver coverage: `33 / 97 = 34.0%`
- New total obligation coverage: `311 / 668 = 46.6%`
- Integration verification: `cargo fmt --check`, `node scripts/check-cleanroom-harness.cjs`, `git diff --check HEAD~1...HEAD`, `cargo test`, `cargo clippy --all-targets -- -D warnings`, and `cargo test adapter_replays_all_branches --quiet` passed.
- Review/fixer notes: review loop replaced self-referential expected route witnesses with adapter-local literal route witnesses, removed production selected-identity dispatch from reported class-feature paths, and corrected overclaimed evidence artifacts.
- Worktrees marked removable: `/workspace/typescript/.codex-worktrees/dnd-cleanroom-l15-rr08`

### L15-RR09-CHARACTER-SHEET-ROUTES

- Merge commit: `616b6c27104807a0cf312d4cd68485e0c7fef1c3`
- Lane commit(s): `cf9a8974fc9285878f6a8276dff8aa3cda3f3682`, `410e3db7d181c7fba9265cc753ab977198dacca7`
- Drivers added: `10` net-new unique character-sheet drivers.
- Obligations added: `67` net-new counted obligations; total accepted obligations moved from `311` to `378`.
- New total driver coverage: `43 / 97 = 44.3%`
- New total obligation coverage: `378 / 668 = 56.6%`
- Integration verification: `cargo fmt --check`, `node scripts/check-cleanroom-harness.cjs`, `git diff --check HEAD~1...HEAD`, `cargo test`, `cargo clippy --all-targets -- -D warnings`, and `cargo test adapter_replays_all_branches --quiet` passed.
- Review/fixer notes: review loop replaced self-referential route and projection witnesses, added route comparisons for previously projection-only selected drivers, and resolved RR08/RR09 artifact overlap by keeping cumulative run-ledger and validation evidence for both lanes.
- Worktrees marked removable: `/workspace/typescript/.codex-worktrees/dnd-cleanroom-l15-rr09`

### L15-RR10-CHARACTER-BATTLE-HANDOFF-ROUTES

- Merge commit: `3a0c18402e5620a55d0f2046329ab5697023c2e3`
- Lane commit(s): `421541a1958b20bdfb3b785174ea125673135d72`, `4ea5566fc503fcfa17f430b523faa87dc189943c`
- Drivers added: `5` net-new unique character-battle handoff drivers.
- Obligations added: `33` net-new counted obligations; total accepted obligations moved from `378` to `411`.
- New total driver coverage: `48 / 97 = 49.5%`
- New total obligation coverage: `411 / 668 = 61.5%`
- Integration verification: `cargo fmt --check`, `node scripts/check-cleanroom-harness.cjs`, `git diff --check HEAD~1...HEAD`, `cargo test`, `cargo clippy --all-targets -- -D warnings`, and `cargo test adapter_replays_all_branches --quiet` passed.
- Review/fixer notes: review loop found non-independent expected route witnesses; fixer replaced shared observed/expected route builders with independent adapter-local literal route witnesses while keeping observed replay on reducer-facing route construction.
- Worktrees marked removable: `/workspace/typescript/.codex-worktrees/dnd-cleanroom-l15-rr10`

### L15-RR07S-B-ZERO-HP-FORM-MOVEMENT-SUBSTRATES

- Merge commit: `0ef395a`
- Lane commit(s): `81fd387c6dda4b00c28a752492d6aa0d2fee4d3d`
- Drivers added: `3` net-new unique drivers.
- Obligations added: `12` net-new counted obligations; total accepted obligations moved from `411` to `423`.
- New total driver coverage: `51 / 97 = 52.6%`
- New total obligation coverage: `423 / 668 = 63.3%`
- Integration verification: `cargo fmt --check`, `node scripts/check-cleanroom-harness.cjs`, `git diff --check HEAD~1...HEAD`, `cargo test`, `cargo clippy --all-targets -- -D warnings`, and `cargo test adapter_replays_all_branches --quiet` passed.
- Review/fixer notes: reviewer returned clean; Barbarian Fast Movement and Ranger Roving remain blocked or out-of-lane.
- Worktrees marked removable: `/workspace/typescript/.codex-worktrees/dnd-cleanroom-l15-rr07s-b`

### L15-RR07S-C-WEAPON-BREATH-FEATURE-SUBSTRATES

- Merge commit: `67bc22f`
- Lane commit(s): `3da4fbbd6d0630c096d0d386f28cc0ce02fb432b`, `e65678a9e310be2f97ba582711a036454c98ab2d`, `6d89dd42b345c3253d36759d723c1fe4a3271c2c`
- Drivers added: `3` net-new unique drivers.
- Obligations added: `11` net-new counted obligations; total accepted obligations moved from `423` to `434`.
- New total driver coverage: `54 / 97 = 55.7%`
- New total obligation coverage: `434 / 668 = 65.0%`
- Integration verification: `cargo fmt --check`, `node scripts/check-cleanroom-harness.cjs`, `git diff --check HEAD~1...HEAD`, `cargo test`, `cargo clippy --all-targets -- -D warnings`, and `cargo test adapter_replays_all_branches --quiet` passed.
- Review/fixer notes: review loop replaced self-referential expected routes and moved observed routes through reducer observer entrypoints.
- Worktrees marked removable: `/workspace/typescript/.codex-worktrees/dnd-cleanroom-l15-rr07s-c`

### L15-RR07S-A-PASSIVE-ROLL-RESOURCE-SUBSTRATES

- Merge commit: `1b0d0dbc1615de1efdafce3f74b1b6372e2df8d9`
- Lane commit(s): `d2c54fb02a1b37f7b0957f8560bdc6ed2a8c4c62`, `b857f65367b3a921406ad5c9c4627eadf45f460e`, `969f4526e4e2bb797ee01e2717fbede918f8c768`, `582b2a71c6573c480d4ccddec15381308a2bd667`
- Drivers added: `5` net-new unique drivers.
- Obligations added: `17` net-new counted obligations; total accepted obligations moved from `434` to `451`.
- New total driver coverage: `59 / 97 = 60.8%`
- New total obligation coverage: `451 / 668 = 67.5%`
- Integration verification: `cargo fmt --check`, `node scripts/check-cleanroom-harness.cjs`, `git diff --check HEAD~1...HEAD`, `cargo test` (`207 passed`), `cargo clippy --all-targets -- -D warnings`, and `cargo test adapter_replays_all_branches --quiet` (`76 passed`) passed.
- Review/fixer notes: review loop replaced projection-only and adapter-synthesized route evidence with production-observed route traces; `BattleState.feature_resources` is the battle-owned durable resource owner.
- Worktrees marked removable: `/workspace/typescript/.codex-worktrees/dnd-cleanroom-l15-rr07s-a`

### L15-RR07-FU08A-METAMAGIC-GOVERNOR-QUICKENED-SUBSTRATE

- Merge commit: `b909dfdaa01bca96953a590b9984518bd5e9bc68`
- Lane commit(s): `07e47f01a9e728fcfe919b4af2e1a4592b97069f`
- Drivers added: `4` net-new unique drivers.
- Obligations added: `11` net-new counted obligations; total accepted obligations moved from `451` to `462`.
- New total driver coverage: `63 / 97 = 64.9%`
- New total obligation coverage: `462 / 668 = 69.2%`
- Integration verification: `cargo fmt --check`, `node scripts/check-cleanroom-harness.cjs`, `git diff --check HEAD~1...HEAD`, `cargo test` (`207 passed`), and `cargo clippy --all-targets -- -D warnings` passed.
- Review/fixer notes: reviewer returned clean; observed Quickened/metamagic evidence routes through production observer entrypoints, expected route witnesses are literal, and production routing dispatches by metamagic option/effect shape rather than authored identity.
- Worktrees marked removable: `/workspace/typescript/.codex-worktrees/dnd-cleanroom-l15-rr07-fu08a`

### L15-RR07-FU01A-CATALOG-READY-SPELL-SUBSTRATES

- Merge commit: `5a6e5d2d1975788acef1373786eb94dd0074407e`
- Lane commit(s): `710ff08b6114640820b147e5d0b4a460693e73e2`, `c1945b17d33c5f97ed0779284007039fb7dd090a`, `86290c012cdefb8afdcc526a8caa58da190b407b`
- Drivers added: `3` net-new unique drivers.
- Obligations added: `9` net-new counted obligations; total accepted obligations moved from `462` to `471`.
- New total driver coverage: `66 / 97 = 68.0%`
- New total obligation coverage: `471 / 668 = 70.5%`
- Integration verification: `node scripts/check-cleanroom-harness.cjs`, `cargo fmt --check`, `git diff --check --cached`, `cargo test` (`208 passed`), and `cargo clippy --all-targets -- -D warnings` passed.
- Review/fixer notes: first reviewer found Find Familiar observed route evidence was adapter-local; fixer added production companion route ownership and Find Familiar route observation, retained the four Find Familiar accepted rows, and left existing level-2/3 condition-save rows out of scope. Re-review confirmed the code path clean; artifact fixer aligned RUN_LEDGER with VALIDATION_REPORT.
- Worktrees marked removable: `/workspace/typescript/.codex-worktrees/dnd-cleanroom-l15-rr07-fu01a`

### L15-RR07-FU08B-METAMAGIC-SAVE-RANGE-TARGET-SUBSTRATES

- Merge commit: `20daf3ede80af636df4e71a776d95783182fef2d`
- Lane commit(s): `3727abed465765fbc2b3f223076ed265d31393ff`
- Drivers added: `4` net-new unique drivers.
- Obligations added: `9` net-new counted obligations.
- Integration verification: `cargo fmt --check`, `node scripts/check-cleanroom-harness.cjs`, `git diff --check HEAD~1...HEAD`, `cargo test` (`208 passed`), and `cargo clippy --all-targets -- -D warnings` passed.
- Review/fixer notes: reviewer returned clean; observed Careful/Heightened/Distant/Twinned metamagic routes go through production BattleState observer entrypoints with literal independent expected route witnesses.
- Worktrees marked removable: `/workspace/typescript/.codex-worktrees/dnd-cleanroom-l15-rr07-fu08b`

### L15-RR07-FU08C-METAMAGIC-REROLL-DAMAGE-PROJECTION-SUBSTRATES

- Merge commit: `5bec99231b38d67c5930922cb77328d2b71c031a`
- Lane commit(s): `a6e33d9bad73f82e20e0373986738e943d45233d`, `a0ac254c610b4c0e64b4f6d160c0ef533f400a40`
- Drivers added: `5` net-new unique drivers.
- Obligations added: `7` net-new counted obligations; total accepted obligations moved from `471` to `478`.
- New total driver coverage: `71 / 97 = 73.2%`
- New total obligation coverage: `478 / 668 = 71.6%`
- Integration verification: focused FU08B/FU08C adapter tests, `cargo fmt --check`, `node scripts/check-cleanroom-harness.cjs`, `git diff --check`, `cargo test` (`209 passed`), and `cargo clippy --all-targets -- -D warnings` passed.
- Review/fixer notes: reviewer found contradictory metamagic option facts/effects were representable and accepted; fixer added `MetamagicOptionEffectMismatch` at the reducer boundary before spend/action-lock/projection/routing, plus a regression test. Integration resolved FU08B/FU08C by sharing one `MetamagicOptionSpell` subject with typed modification/effect shape checks.
- Worktrees marked removable: `/workspace/typescript/.codex-worktrees/dnd-cleanroom-l15-rr07-fu08c`

### L15-RR07-FU01B-SPELL-ATTACK-SAVE-DAMAGE-SUBSTRATES

- Merge commit: `6834ee18356e01eb9b00bd4b32f0169b75a7220d`
- Lane commit(s): `11f9f56bf2dccee87204371cb9a5d5f125e6402c`, `195a473783bc666f5413dabe0a2bf93ea18003df`
- Drivers added: `2` net-new unique drivers.
- Obligations added: `14` accepted counted obligations; `2` rows are recorded as target blockers. Total accepted obligations moved from `478` to `492`.
- New total driver coverage: `73 / 97 = 75.3%`
- New total obligation coverage: `492 / 668 = 73.7%`
- Integration verification: focused FU01B adapter tests, FU08C mismatch regression, `cargo fmt --check`, `node scripts/check-cleanroom-harness.cjs`, `git diff --check`, `cargo test` (`209 passed`), and `cargo clippy --all-targets -- -D warnings` passed.
- Review/fixer notes: Carson found stale FU01B artifacts and dishonest blocker witnesses. Schrodinger fixed artifacts/history/evidence and made the two blocker rows reject/report target-blocker status across observed replay, observed route, expected witness, and evidence. Turing re-reviewed clean.
- Worktrees marked removable: `/workspace/typescript/.codex-worktrees/dnd-cleanroom-l15-rr07-fu01b`

## CP4 Launch Plan

The small feature substrate batch was split into three parallel lanes from integration head `8d8576315773c721128fabaf79319bdbf2921eaa` and is now merged:

- `L15-RR07S-A-PASSIVE-ROLL-RESOURCE-SUBSTRATES`: Adrenaline Rush, species passive traits, Danger Sense, and roll-modifier buff substrates (`5` drivers, `17` in-scope obligations). Worker: Copernicus `019f070a-d8bb-7111-b578-d4eee793076f`; committed `d2c54fb02a1b37f7b0957f8560bdc6ed2a8c4c62`; reviewer Herschel `019f071c-a15c-7763-9148-3f7cfdcc6211` found that target replay evidence overclaims route-event comparison while tests assert projection witnesses.
  - Fixer Peirce `019f0721-7a01-7303-bae8-8591e16e1c2d` committed `b857f65367b3a921406ad5c9c4627eadf45f460e`; re-reviewer Hooke `019f0731-733c-7791-9b81-cc60adf5dc58` found that observed route evidence is still adapter-synthesized for the five lane drivers instead of emitted by the production reducer route/observer path. Fixer Kant `019f0736-5669-7d40-89ef-55b4c7cf8f92` committed `969f4526e4e2bb797ee01e2717fbede918f8c768` to route observed evidence through `BattleReducerRouteTrace`; re-reviewer Raman `019f0740-6f67-70d1-9c83-c4012fa2f0a5` found adapters still author the trace by creating `BattleReducerRouteTrace::default()` and calling observe helpers locally. Fixer Arendt `019f0743-c359-70d2-8e7e-fdf4a94a63a1` committed `582b2a71c6573c480d4ccddec15381308a2bd667`, moving accepted observed route evidence to production observer entrypoints in `battle_reducer_spine` and `battle_features`; re-reviewer Curie `019f0757-f7ea-7d50-aade-065a0f11916e` returned clean.
- `L15-RR07S-B-ZERO-HP-FORM-MOVEMENT-SUBSTRATES`: zero-HP stabilization, Wild Shape form lifecycle, and movement/forced-movement substrates (`3` drivers, `12` in-scope obligations; `2` out of scope). Worker: Newton `019f070b-2c30-7de1-bece-a71c78a04a4b`; committed `81fd387c6dda4b00c28a752492d6aa0d2fee4d3d`; reviewer Ramanujan `019f071a-2cf3-7a21-8176-bc3148c52163` returned clean.
- `L15-RR07S-C-WEAPON-BREATH-FEATURE-SUBSTRATES`: weapon mastery property, Dragonborn Breath Weapon, and active feature spell-benefit substrates (`3` drivers, `11` in-scope obligations). Worker: Kuhn `019f070b-ff89-7fd2-8978-44745a13ef82`; committed `3da4fbbd6d0630c096d0d386f28cc0ce02fb432b`; reviewer Nash `019f072c-d0a6-7e53-adcf-02844f72a6f9` found blocker findings. Expected route witnesses shared `route_*` helper construction with observed routes in the weapon mastery, Dragonborn Breath Weapon, and feature selected-identity adapters, and the evidence artifacts overclaimed literal independent expected routes. Fixer Franklin `019f0730-cabf-77a3-801f-b5b9736f7e6b` committed `e65678a9e310be2f97ba582711a036454c98ab2d` to replace expected routes with independent literal `ReducerRouteEvent` records; re-reviewer Dewey `019f0739-45c1-70a3-b026-5757480663b5` found the observed route witnesses are still adapter-synthesized instead of reducer-observed through `start_battle_observed`, `discover_battle_acts_observed`, or `resolve_battle_subject_observed`. Fixer Averroes `019f073c-aa3b-7182-8769-82593a36eb71` committed `6d89dd42b345c3253d36759d723c1fe4a3271c2c`, adding reducer-observed route events to `BattleEntrypointTrace` and using observed reducer entrypoints for the three lane adapters; re-reviewer Noether `019f074b-83da-7fb3-a98a-d833225f491e` returned clean.

`L15-RR07-FU01-LEVEL1-SPELL-IDENTITY-SUBSTRATES` and `L15-RR07-FU08-METAMAGIC-GOVERNOR-AND-OPTION-SUBSTRATES` remain unscheduled until they are split according to their campaign recommendations.

## CP4 Remaining Split Plan

`L15-RR07-FU01-LEVEL1-SPELL-IDENTITY-SUBSTRATES` is split into six sublanes:

- `L15-RR07-FU01A-CATALOG-READY-SPELL-SUBSTRATES`: catalog-ready selected spell witnesses over existing save-gated, repeat-save, companion, and roll-modifier substrates. This is the first FU01 launch lane.
- `L15-RR07-FU01B-SPELL-ATTACK-SAVE-DAMAGE-SUBSTRATES`: spell attack, save-gated damage, chained/object-target, and post-hit/post-damage active-effect substrates.
- `L15-RR07-FU01C-WEAPON-BUFF-MARK-SMITE-SUBSTRATES`: weapon-hosted buff, after-hit rider, scalar, held-weapon, mark-transfer, condition-immunity, and temporary-HP substrates.
- `L15-RR07-FU01D-PROTECTION-CHARM-WARD-SUBSTRATES`: creature-type target admission, protection/charm active effects, charm-break lifecycle, and warded-target interdiction.
- `L15-RR07-FU01E-ARMOR-CLASS-REACTION-SUBSTRATES`: armor-class base active effect and reaction-triggered armor-class spell substrates.
- `L15-RR07-FU01F-SPATIAL-LIGHT-AREA-MOVEMENT-SUBSTRATES`: light, outline, area hazard, falling, movement replacement, forced movement, and object-boundary substrates.

`L15-RR07-FU08-METAMAGIC-GOVERNOR-AND-OPTION-SUBSTRATES` is split into three sublanes:

- `L15-RR07-FU08A-METAMAGIC-GOVERNOR-QUICKENED-SUBSTRATE`: shared Sorcery Point spend, one-option-per-spell, selected-option admission, Bonus Action casting-time, same-turn level-1-plus spell locks, and Quickened selected witnesses. This must run before FU08B/FU08C.
- `L15-RR07-FU08B-METAMAGIC-SAVE-RANGE-TARGET-SUBSTRATES`: Careful, Heightened, Distant, and Twinned option substrates.
- `L15-RR07-FU08C-METAMAGIC-REROLL-DAMAGE-PROJECTION-SUBSTRATES`: Empowered, Seeking, Subtle, Transmuted, and Extended option substrates.

Initial ready lanes after the split: `L15-RR07-FU01A-CATALOG-READY-SPELL-SUBSTRATES` and `L15-RR07-FU08A-METAMAGIC-GOVERNOR-QUICKENED-SUBSTRATE`.
