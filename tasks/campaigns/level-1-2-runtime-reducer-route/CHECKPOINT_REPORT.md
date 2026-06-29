# Checkpoint Report

Campaign: `level-1-2-runtime-reducer-route`

## Latest Checkpoint

- Source master merged `SOURCE-SQNT07A-WEAPON-HOSTED-SELECTED-ROUTE-WITNESS` at `06f8042513df94bc96e02ec9d213a0d52d942bac`; focused source checks passed (`quint typecheck`, selected route connector Vitest, MBT driver closure).
- Fresh target `/workspace/typescript/.codex-worktrees/dnd-fresh-cleanroom-dry-run-fc00` is accepted at `444295121a3cbe0631dc36ef2280c27d2ef5631c`.
- Fresh accepted in this checkpoint: `FRESH-RR-SQNT07A-JUMP-LANDING-PRONE-REPLAY` and `FRESH-RR-SQNT07A-CONCENTRATION-AREA-HAZARD-REPLAY`.
- Final fresh checks passed: `python3 tools/verify_current_fresh_target.py`, jump/concentration focused verifiers, focused reducer tests for `jump_landing` and `concentration`, `cargo fmt --check`, `cargo test`, and `cargo clippy --all-targets -- -D warnings`.
- Active Ralph lanes: none. Retained blockers: fresh consumption of source weapon-hosted selected witness, Hex ability-check roll-mode, selected concentration hazard exactness, and exact damage/arithmetic details.

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
| CP4 Feature And Catalog Substrates | complete | Small feature substrate batch, FU01 split lanes, and FU08 split lanes are merged and verified; FU01D is accepted after the copied route connector refresh. |
| CP5 Remaining Battle Families | complete | Six CP5 sublanes merged and verified. |
| CP6 Closure Sweep | complete | Closure audit recorded in `CP6_AUDIT.json` and `CP6_CLOSURE_REPORT.md`; every in-scope obligation is accepted or explicitly blocked. |
| CP7 Post-CP6 Target Blocker Reduction | complete after source refresh | `L15-RRCP7-A` through `L15-RRCP7-E` merged and verified; source refresh `d5a70b23` moved nine fixture scenario transition rows out of the reducer-route denominator and supplied a generic spell base Armor Class route connector. |
| CP8 Post-Refresh Mage Armor Route Acceptance | complete | `L15-RRCP8-A-MAGE-ARMOR-GENERIC-AC-ROUTES` merged and verified; the three remaining Mage Armor admission/lifecycle rows now replay through `SpellBaseArmorClassEffectRouteSubject`. |

## Current Fresh Verification

At campaign-control head `63075124dc0de6a2cfbc0ce9516fea7a3d8aaf1a`
before this checkpoint update and fresh target head
`ce9f653e3cba6a9eefa0d2f14e11757f7081e618`:

- `python3 tools/verify_current_fresh_target.py`: pass
- `python3 tools/verify_fresh_rr_sqnt07a_level1_after_hit_timed_selected.py`:
  pass
- `python3 tools/verify_fresh_rr_reaction_interrupt_taxonomy.py`: pass
- `python3 tools/verify_fresh_rr_battle_active_effects.py`: pass
- `python3 tools/verify_fresh_rr_sqnt07a_level1_marked_immunity_selected.py`:
  pass
- `python3 tools/verify_fresh_rr_sqnt07a_level1_scalar_buff_selected.py`: pass
- `python3 tools/verify_fresh_rr_sqnt07a_level1_weapon_hosted_selected.py`:
  pass
- `cargo test fresh_reaction_interrupt_taxonomy_routes_use_generic_payload_shapes --test reducer_spine`: pass
- `cargo test sqnt07a_level1_ -- --nocapture`: pass
- `cargo fmt --check`: pass
- `cargo test`: pass
- `cargo clippy --all-targets -- -D warnings`: pass
- `git diff --check HEAD~1...HEAD`: pass
- current package: `545d7848692fcb18adf14e5c009d9e7f4d0cb1d5`
- inventory: `98` drivers, `663` in-scope obligations, `45` out-of-scope
  obligations
- current fresh runtime claims: FEXP-08, FEXP-09A through FEXP-09G, SQNT-07A
  active-effect/condition, spatial/damage, character/sheet/handoff,
  reaction/interrupt taxonomy, battle active-effects, and level-1
  marked/immunity, scalar-buff, after-hit/timed rider selected-row replays, and
  weapon-hosted generic route-surface replay
- active fresh expansion lanes: none

This supersedes the older FEXP-09G and partial renewal checkpoints as the
current fresh target state. It does not change the CP8 dirty rehearsal
denominator. No planned current-package proof-renewal lane remains in this batch.

## Last Dirty Rehearsal Verification

At campaign-control head `ebd3699fb03c99d0ec674361ebf69835e64bfd0c`
after the SQNT-07 current-package dirty replay batch:

- focused SQNT-07B species/passive tests: pass
- focused SQNT-07C metamagic route connector test: pass
- focused SQNT-07D active-feature test and evidence checker: pass
- `cargo fmt --check`: pass
- `cargo test`: pass, `234` tests
- `cargo clippy --all-targets -- -D warnings`: pass
- `git diff --check`: pass
- `node scripts/check-cleanroom-harness.cjs`: progresses past guidance and
  approved validator provenance, then fails on older historical replay/ledger
  debt still pinned to source package `564376fd95218a209bb9eae5c9ccb54ca3e04a52`
  plus pre-existing heuristic findings outside the SQNT-07B/C/D replay batch.

## Source Corpus Boundary

CP8 closes the post-refresh target-side dirty cleanroom rehearsal. Dirty
rehearsal accepted coverage is `659 / 659` refreshed in-scope obligations. The
refreshed source inventory also marks `45` obligations out of scope, including
the nine scenario sequencing rows moved out of the reducer-route denominator.

The current fresh package checkpoint is separate: the fresh verifier at
`ce9f653e3cba6a9eefa0d2f14e11757f7081e618` reports `98` drivers, `663`
in-scope obligations, and `45` out-of-scope obligations for source package
`545d7848692fcb18adf14e5c009d9e7f4d0cb1d5`.

Detailed source-refresh handoff: `tasks/campaigns/level-1-2-runtime-reducer-route/SOURCE_CORPUS_HANDOFF.md`.

Remaining blocker groups:

- None in the refreshed in-scope denominator. The dirty Rust target now has accepted target replay evidence for every refreshed in-scope obligation in this campaign.

Resolved by source refresh `d5a70b23ad05abd4188b1f0d37d9c6aba600cce5`:

- The nine scenario transition rows are now out-of-scope transit-only fixture sequencing rows.
- The Mage Armor selected-identity driver is now reducer-routed through `cleanroom-input/qnt/battle-runtime/battle-runtime-spell-base-armor-class-effect.route.mbt.qnt`.
- Refreshed denominator: `659` in-scope obligations, `45` out-of-scope obligations. CP8 accepted the final `3` Mage Armor rows, bringing refreshed accepted coverage to `659 / 659`.

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
  - CP4 is complete: FU01 and FU08 split lanes are merged, with FU01D accepted after the copied route connector refresh.

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
- Obligations added: `16` accepted counted obligations; the two residual rows are accepted only as route-shape evidence. Total accepted obligations moved from `478` to `494`.
- New total driver coverage: `73 / 97 = 75.3%`
- New total obligation coverage: `494 / 668 = 74.0%`
- Integration verification: focused FU01B adapter tests, FU08C mismatch regression, `cargo fmt --check`, `node scripts/check-cleanroom-harness.cjs`, `git diff --check`, `cargo test` (`209 passed`), and `cargo clippy --all-targets -- -D warnings` passed.
- Review/fixer notes: Carson found stale FU01B artifacts and dishonest blocker witnesses. Schrodinger fixed artifacts/history/evidence; this fixer pass keeps the two residual rows accepted only as route-shape evidence and removes claims that their selected damage-spell witness semantics are `BattleResolutionResult`-derived.
- Worktrees marked removable: `/workspace/typescript/.codex-worktrees/dnd-cleanroom-l15-rr07-fu01b`

### L15-RR07-FU01F-SPATIAL-LIGHT-AREA-MOVEMENT-SUBSTRATES

- Merge commit: `78be1eccb627f3ae7987380922d873b9ca7e497b`
- Lane commit(s): `c4dbd72c098d85dc6fac652bd7b1758e5c6f61dc`, `15cfbe5193ed93e6119873732cccfa93c7f37349`
- Drivers added: `1` net-new unique driver.
- Obligations added: `10` accepted counted obligations; no blockers or demotions. Total accepted obligations moved from `492` to `502`.
- New total driver coverage: `74 / 97 = 76.3%`
- New total obligation coverage: `502 / 668 = 75.1%`
- Integration verification: focused FU01F adapter test, `cargo fmt --check`, `node scripts/check-cleanroom-harness.cjs`, `git diff --check`, `cargo test` (`209 passed`), and `cargo clippy --all-targets -- -D warnings` passed.
- Review/fixer notes: first review found helper-synthesized route evidence and self-referential expected routes; fixer added public spatial subject/fill support and discoverable spatial route subjects, then re-review returned clean.
- Worktrees marked removable: `/workspace/typescript/.codex-worktrees/dnd-cleanroom-l15-rr07-fu01f`

### L15-RR07-FU01E-ARMOR-CLASS-REACTION-SUBSTRATES

- Merge commit: `4f69ca5c407cbc53c5e3a5a431cedf0f1d3148c9`
- Lane commit(s): `6b4866d40d05c68ce2da897571443f58ffbf17ac`, `93dca1a549522070d23e80e13c4ffa6d915c4bb5`, `c135ceb24f7b0f44d93f14763d55dbb9aaa8cf4f`
- Drivers added: `2` net-new unique drivers.
- Obligations added: `3` accepted counted obligations; target-admission/discovery/rejection/expiry rows remain blocked or demoted, and the level-3 reaction branch remains out of scope. Total accepted obligations moved from `502` to `505`.
- New total driver coverage: `76 / 97 = 78.4%`
- New total obligation coverage: `505 / 668 = 75.6%`
- Integration verification: focused FU01E adapter tests and reactor-mismatch regression, `cargo fmt --check`, `node scripts/check-cleanroom-harness.cjs`, `git diff --check HEAD~1...HEAD`, `cargo test` (`210 passed`), and `cargo clippy --all-targets -- -D warnings` passed.
- Review/fixer notes: first review found adapter-manufactured ReactionSpell subjects and overstated artifacts; fixer routed ReactionSpell discovery through production reaction-window substrates. Re-review fixed one fill/reactor mismatch at the reducer boundary and then returned clean. Integration repaired `RUN_LEDGER.json` to preserve cumulative evidence while appending FU01E.
- Worktrees marked removable: `/workspace/typescript/.codex-worktrees/dnd-cleanroom-l15-rr07-fu01e`

### L15-RR07-FU01C-WEAPON-BUFF-MARK-SMITE-SUBSTRATES

- Merge commit: `925c298ac923b894891a34331865839f5a1be371`
- Lane commit(s): `bf62c115801af2ff95e7df2418316747f99af010`, `45f5bbb71c2d7d2ac946112dacdc63e94c7b4af4`, `947f76c009833c8a0702e78956accfd1a3417da9`
- Drivers added: `1` net-new unique driver.
- Obligations added: `8` accepted counted obligations; `4` rows remain target blockers for generic condition-immunity active-effect and marked-effect transfer route subjects. Total accepted obligations moved from `505` to `513`.
- New total driver coverage: `77 / 97 = 79.4%`
- New total obligation coverage: `513 / 668 = 76.8%`
- Integration verification: focused FU01C adapter test, `cargo fmt --check`, `node scripts/check-cleanroom-harness.cjs`, `git diff --check HEAD...`, `cargo test` (`210 passed`), and `cargo clippy --all-targets -- -D warnings` passed.
- Review/fixer notes: first review found false durable ownership claims for nonexistent `BattleState.genericRouteSubjects`; fixer reframed evidence around real subject kinds and route events. Re-review found duplicate continuation ownership in `GenericRouteShape.next_holes`; follow-up fixer removed that field so `generic_route_next_holes(subject.kind, fill)` is the single continuation source. Final re-review returned clean.
- Worktrees marked removable: `/workspace/typescript/.codex-worktrees/dnd-cleanroom-l15-rr07-fu01c`

### L15-RR07-FU01D-PROTECTION-CHARM-WARD-SUBSTRATES

- Merge commit: `91e141c8998682ebf6daf65e0ff5594aaf24551e`
- Lane commit(s): `879b44d3d7dec602df492f4570429b9fc92cb6f0`, `97c2e1feb4da16f7f60c96dcf3a16374d748283c`, `b5a005e4cfb4b35b47a9b65f1dd688f61e059142`, `61a1bc984aa104b439c4adf3d2e43b0ad57e1674`
- Source-refresh commit: `2455ec2f9b3c5d8b696d9cc144f196e9393038c6`
- Drivers added after source refresh: `2` accepted drivers.
- Obligations added after source refresh: `16` accepted obligations; no selected FU01D rows remain blocked.
- Current closure driver coverage: `97 / 97 = 100.0%`
- Current closure obligation coverage: `643 / 668 = 96.3%`
- Integration verification: focused FU01D evidence check, focused FU01D adapter tests, `cargo fmt --check`, `cargo test route_replays_all_branches`, full `cargo test`, `cargo clippy --all-targets -- -D warnings`, and `git diff --check 410a784738fba3b80566eae292140327d4e30877...HEAD` passed. `node scripts/check-cleanroom-harness.cjs` now fails only on global stale non-FU01D evidence after the cleanroom-input source refresh.
- Review/fixer notes: first review removed duplicated durable projection state and self-referential witnesses. Second review correctly rejected Rust-only evidence without copied executable connector evidence. The later source refresh added copied executable route connectors, restored accepted FU01D evidence through generic subject/fill/owner/hole route replay, and left production authored identity out of dispatch.
- Worktrees marked removable: `/workspace/typescript/.codex-worktrees/dnd-cleanroom-l15-rr07-fu01d`

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

## CP5 Split Plan

`CP5-REMAINING-BATTLE-FAMILIES` is split into six parallel lanes from integration head `91e141c8998682ebf6daf65e0ff5594aaf24551e`:

- `L15-RRCP5-A-RIDER-AND-WEAPON-HOSTED-ROUTES`: after-hit damage riders and weapon-hosted attack/rider procedures (`2` drivers, `49` obligations).
- `L15-RRCP5-B-ACTIVE-EFFECT-LIFECYCLE-ROUTES`: roll modifiers, scalar active effects, repeat saves, turn-boundary lifecycle, and zero-HP mid-resolution (`5` drivers, `33` obligations).
- `L15-RRCP5-C-REACTION-INTERRUPT-ROUTES`: interrupt stack resume and reaction casting time (`2` drivers, `6` obligations).
- `L15-RRCP5-D-COMPANION-OBJECT-BOUNDARY-ROUTES`: companion lifecycle and object-boundary spell behavior (`2` drivers, `12` obligations).
- `L15-RRCP5-E-ABILITY-SEARCH-CHOICE-ROUTES`: ability-check/search choice behavior (`1` driver, `12` obligations).
- `L15-RRCP5-F-INDEPENDENT-SPELL-ATTACK-SEQUENCE-ROUTES`: chained attack sequence and independent spell-attack sequence (`2` drivers, `18` obligations).

All CP5 lanes must preserve the campaign rule: accepted coverage requires reducer-routed evidence through public reducer entrypoints and copied executable connector evidence where the inventory demands it; otherwise rows are blockers, not accepted coverage.

### L15-RRCP5-F-INDEPENDENT-SPELL-ATTACK-SEQUENCE-ROUTES

- Merge commit: `964d6672c999db82c4d297543315a65d054f7703`
- Lane commit(s): `4e84842fa5a27bb1db91cf855a76d5e9cc9860da`
- Drivers added: `2` net-new unique drivers.
- Obligations added: `18` accepted counted obligations; no blockers.
- New total driver coverage: `79 / 97 = 81.4%`
- New total obligation coverage: `531 / 668 = 79.5%`
- Integration verification: focused chained attack and independent spell-attack adapter tests, `cargo fmt --check`, `node scripts/check-cleanroom-harness.cjs`, `git diff --check HEAD~1...HEAD`, `cargo test` (`210 passed`), and `cargo clippy --all-targets -- -D warnings` passed.
- Review/fixer notes: reviewer returned clean; observed replay uses public reducer entrypoints, route evidence covers exactly `10` RR16 plus `8` RR22 obligations, and no production spell-name/selected-identity dispatch was introduced.
- Worktrees marked removable: `/workspace/typescript/.codex-worktrees/dnd-cleanroom-l15-rrcp5-f`

### L15-RRCP5-D-COMPANION-OBJECT-BOUNDARY-ROUTES

- Merge commit: `5818c1573fae64e63fbab82900b3f41dc06576d1`
- Lane commit(s): `122431e66c696399d504924546879dd2da3d7e90`
- Drivers added: `2` net-new unique drivers.
- Obligations added: `12` accepted counted obligations; no blockers.
- New total driver coverage: `81 / 97 = 83.5%`
- New total obligation coverage: `543 / 668 = 81.3%`
- Integration verification: focused Starry Wisp object and Find Familiar companion lifecycle adapter tests, `cargo fmt --check`, `node scripts/check-cleanroom-harness.cjs`, `git diff --check`, `cargo test` (`210 passed`), and `cargo clippy --all-targets -- -D warnings` passed.
- Review/fixer notes: reviewer returned clean; observed replay uses public reducer entrypoints, expected routes are independently enumerated, the object-target boundary owner is distinct from generic target selection, and production dispatch is by companion/object-target route shape rather than authored identity.
- Worktrees marked removable: `/workspace/typescript/.codex-worktrees/dnd-cleanroom-l15-rrcp5-d`

### L15-RRCP5-B-ACTIVE-EFFECT-LIFECYCLE-ROUTES

- Merge commit: `09414700e40d3f4c1818fdf10aa42f7bff69ff71`
- Lane commit(s): `aa2cc0d6dabacbdc77c9eedc1b299f6b7a3c9b86`, `83cf6603eca84c380fd434150b662c4341d45126`
- Drivers added: `5` net-new unique drivers.
- Obligations added: `33` accepted counted obligations; no blockers.
- New total driver coverage: `86 / 97 = 88.7%`
- New total obligation coverage: `576 / 668 = 86.2%`
- Integration verification: `cargo test routes_ -- --nocapture`, `node scripts/check-cleanroom-harness.cjs`, `cargo fmt --check`, `git diff --check HEAD~1...HEAD`, `cargo test` (`215 passed`), and `cargo clippy --all-targets -- -D warnings` passed.
- Review/fixer notes: first review found self-confirming expected routes and stale lane ids; fixer replaced expected routes with independent copied witnesses, kept observed replay on public reducer entrypoints, renamed artifacts to `L15-RRCP5-B-ACTIVE-EFFECT-LIFECYCLE-ROUTES`, and re-review returned clean.
- Worktrees marked removable: `/workspace/typescript/.codex-worktrees/dnd-cleanroom-l15-rrcp5-b`

### L15-RRCP5-A-RIDER-AND-WEAPON-HOSTED-ROUTES

- Merge commit: `a0a88e469f71f5069f4f01c19bbdbc2745660155`
- Lane commit(s): `bef84dac88c28fa00250c29d752904dafbb1f036`, `5ecdd9ef43f9598fe19fdda58fcab4596c0c68c2`
- Drivers added: `2` net-new unique drivers.
- Obligations added: `29` accepted counted obligations; `20` rows remain target-blocked.
- New total driver coverage: `88 / 97 = 90.7%`
- New total obligation coverage: `605 / 668 = 90.6%`
- Integration verification: focused after-hit rider and weapon-hosted adapter tests, `node scripts/check-cleanroom-harness.cjs`, `cargo fmt --check`, `git diff --check HEAD~1...HEAD`, `cargo test` (`217 passed`), and `cargo clippy --all-targets -- -D warnings` passed.
- Review/fixer notes: first review found unsupported illumination rows, duplicate `doFinish` ids, and self-comparison witness coverage. Fixer removed unsupported production `AfterHitDamageRiderIllumination*` route subjects, kept Shining rows blocked, qualified blocker/trace ids, and tied accepted coverage to reducer route evidence. Re-review returned clean.
- Worktrees marked removable: `/workspace/typescript/.codex-worktrees/dnd-cleanroom-l15-rrcp5-a`

### L15-RRCP5-C-REACTION-INTERRUPT-ROUTES

- Merge commit: `97763b0f981424c462d7af86f0dd0463dd1b9012`
- Lane commit(s): `555ebff0f658c1a4d090c46b49637ef0283578ea`, `8b97ed1fa16870bc42689ca3176a92b191e91a91`, `6300fce274c2c9774e0e159b068463d50a8958e2`
- Drivers added: `2` net-new unique drivers.
- Obligations added: `4` accepted counted obligations; `2` Counterspell rows remain target-blocked.
- New total driver coverage: `90 / 97 = 92.8%`
- New total obligation coverage: `609 / 668 = 91.2%`
- Integration verification: focused interrupt-stack and reaction-casting adapter tests, `node scripts/check-cleanroom-harness.cjs`, `cargo fmt --check`, `git diff --check HEAD~1...HEAD`, `cargo test` (`217 passed`), and `cargo clippy --all-targets -- -D warnings` passed.
- Review/fixer notes: first review found self-fulfilling accepted route evidence and missing blocked Counterspell reporting. Fixer routed accepted qRoute replay through `BattleEntrypointTrace`, `discover_generic_route_subject_observed`, and `resolve_battle_subject_observed`; Counterspell rows remain blocked rather than accepted. Re-review returned clean.
- Worktrees marked removable: `/workspace/typescript/.codex-worktrees/dnd-cleanroom-l15-rrcp5-c`

### L15-RRCP5-E-ABILITY-SEARCH-CHOICE-ROUTES

- Merge commit: `e1cb8616d0a3fbe2aa59a69d7eeba28da6e7622d`
- Lane commit(s): `4ce08a7458f4e5ad81b11c5dc67025b80422ad35`, `015af29280a59f87abaf9aa677d8f6f8cd4135a2`, `2e5a7d9cb45a12517949bba0ad8c127160992384`, `3dd749a6440a073c10d303d56e05b8ad40af024c`
- Drivers added: `1` net-new unique driver.
- Obligations added: `9` accepted counted obligations; `3` Enhance Ability rows remain out-of-scope and route-empty.
- New total driver coverage: `91 / 97 = 93.8%`
- New total obligation coverage: `618 / 668 = 92.5%`
- Integration verification: focused ability-search adapter test, `node scripts/check-cleanroom-harness.cjs`, `cargo fmt --check`, `git diff --check HEAD~1...HEAD`, `cargo test` (`218 passed`), and `cargo clippy --all-targets -- -D warnings` passed.
- Review/fixer notes: review loop removed out-of-scope Enhance Ability rows from accepted replay evidence, corrected active/history engine-depth metadata, updated ledger hashes, corrected validation-report wording and focused command, and final re-review returned clean.
- Worktrees marked removable: `/workspace/typescript/.codex-worktrees/dnd-cleanroom-l15-rrcp5-e`

## CP5 Completion

- CP5 is complete: F, D, B, A, C, and E are merged and verified.
- CP5 net accepted coverage: `13` drivers and `105` obligations.
- Campaign accepted coverage after CP5: `91 / 97 = 93.8%` drivers and `618 / 668 = 92.5%` obligations.
- CP6 closure sweep is now ready.

## CP6 Closure Sweep

- Audit artifact: `tasks/campaigns/level-1-2-runtime-reducer-route/CP6_AUDIT.json`
- Closure report: `tasks/campaigns/level-1-2-runtime-reducer-route/CP6_CLOSURE_REPORT.md`
- Accepted pass coverage by exact row key: `97 / 97 = 100.0%` drivers and `643 / 668 = 96.3%` in-scope obligations.
- Explicit blockers: `25` in-scope target-implementation blockers with target replay evidence.
- Source-QNT corpus blockers: `0`; FU01D's former 16 source-QNT blockers are accepted after the copied route connector refresh.
- Out-of-scope obligations: `36`, from the copied source branch inventory.
- Unresolved in-scope obligations: `0`.
- Accounting note: some legacy evidence refs in `tasks/RUN_LEDGER.json` are ambiguous by string because they omit `driverPath`; `CP6_AUDIT.json` computes closure by `driverPath#branchFamily:branchAction`.
- Campaign status after CP6: complete for the dirty cleanroom rehearsal. Remaining gaps are target-side blockers only; FU01D is accepted and no source-QNT corpus blocker remains in this closure accounting.

## CP7 Target Blocker Reduction

- Audit artifact: `tasks/campaigns/level-1-2-runtime-reducer-route/CP7_AUDIT.json`
- `L15-RRCP7-A-BUFF-MARK-ACTIVE-EFFECT-ROUTES` merge commit: `f8a438ebff90eb79e274b207c5c286080aaf6726`
- Lane commit(s): `7bd0b47d48ce88979ab23112c3130e5ef4ba0bba`, `a62478648dde859d7e814e1c99d8bdf7987b4af6`
- Drivers added: `0` net-new unique drivers; FU01C was already counted.
- Obligations added: `4` accepted counted obligations; total accepted obligations moved from `643` to `647`.
- New total driver coverage: `97 / 97 = 100.0%`
- New total obligation coverage: `647 / 668 = 96.9%`
- Remaining blockers: `21` target-side blockers, `0` source-QNT corpus blockers, `0` unresolved in-scope obligations.
- Integration verification: focused FU01C adapter test, focused FU01C target replay evidence check, `cargo fmt --check`, `git diff --check HEAD~1...HEAD`, `cargo test` (`220 passed`), and `cargo clippy --all-targets -- -D warnings` passed. `node scripts/check-cleanroom-harness.cjs` still fails on pre-existing stale non-FU01C global evidence/manifest debt.
- Review/fixer notes: review confirmed the four selected FU01C rows route through shared reducer entrypoints and `BattleEntrypointTrace.route_events`, with generic `ConditionImmunityActiveEffect` and `MarkedEffect` subjects and no new durable `BattleState` fields. Fixer corrected `RUN_LEDGER.json` so the known global harness failure is recorded as `classified-nonblocking-fail` rather than `pass`; rereview returned clean.
- Worktrees marked removable: `/workspace/typescript/.codex-worktrees/dnd-cleanroom-l15-rrcp7-a`

### L15-RRCP7-B-DAMAGE-SPELL-RESIDUAL-BRIDGES

- Merge commit: `b33d9bbc1080c0d7a96ab606da07d0f517417096`
- Lane commit(s): `38ee4e9d7f2331c6cb7a9fcd65d15778310f3aa3`, `2741da1671caa183a4907e6eec54de26c744d7e8`
- Drivers added: `0` net-new unique drivers; FU01B was already counted.
- Obligations added: `2` accepted counted obligations; total accepted obligations moved from `647` to `649`.
- New total driver coverage: `97 / 97 = 100.0%`
- New total obligation coverage: `649 / 668 = 97.2%`
- Remaining blockers: `19` target-side blockers, `0` source-QNT corpus blockers, `0` unresolved in-scope obligations.
- Integration verification: focused FU01B level-1 damage spell adapter test, focused attack spell shape adapter test, affected JSON parse checks, `cargo fmt --check`, `git diff --check HEAD~1...HEAD`, `cargo test` (`220` tests), and `cargo clippy --all-targets -- -D warnings` passed. `node scripts/check-cleanroom-harness.cjs` still fails on pre-existing stale non-FU01B global evidence/manifest debt; the saved output contained no FU01B/Chromatic/Starry references.
- Review/fixer notes: review found omitted harness failure bookkeeping and overclaimed residual-row semantics. Fixer recorded the harness as `classified-nonblocking-fail`, narrowed the two residual rows to route-shape evidence only, and made the Chromatic Orb expected route witness independent. Rereview returned clean.
- Worktrees marked removable: `/workspace/typescript/.codex-worktrees/dnd-cleanroom-l15-rrcp7-b`

### L15-RRCP7-C-WEAPON-PREHIT-ATTACK-SETUP-ROUTES

- Merge commit: `554e2e2c0adae1127496d4a12023ae5ac8979f88`
- Lane commit(s): `2c27a3c83162374661ccc229ac9332c96605c966`, `855e702770866c18ae5b657367b95af8410efbe9`
- Selected rows: `doDiscoverWeaponHit`, `doFillTargetChoice`, `doDiscoverDivineFavorAttack`, `doFillDivineFavorHit`, and `doFillDivineFavorTarget`.
- Obligations added: `5` accepted counted obligations; total accepted obligations moved from `649` to `654`.
- New total driver coverage: `97 / 97 = 100.0%`
- New total obligation coverage: `654 / 668 = 97.9%`
- Remaining blockers: `14` target-side blockers, `0` source-QNT corpus blockers, `0` unresolved in-scope obligations.
- Integration verification: focused after-hit and weapon-hosted route adapter tests, affected JSON parse checks, `cargo fmt --check`, `git diff --check HEAD~1...HEAD`, `cargo test` (`220` tests), and `cargo clippy --all-targets -- -D warnings` passed. `node scripts/check-cleanroom-harness.cjs` still fails on pre-existing stale global evidence/manifest debt; the saved output contained no CP7-C lane references.
- Review/fixer notes: review accepted the route implementation and classified the `Actor::Goblin` target as fixture fill data, not production identity dispatch. The only finding was stale CP7 audit head metadata; the worker fixed it and rereview returned clean.
- Explicit non-goals preserved: the nine scenario transition rows and two `MagicWeaponTargetItem` rows remain blocked.
- Worktrees marked removable: `/workspace/typescript/.codex-worktrees/dnd-cleanroom-l15-rrcp7-c`

### L15-RRCP7-D-MAGE-ARMOR-ADMISSION-LIFECYCLE-ROUTES

- Merge commit: `ffc162a61eeea40b0e922b28c7cdeff95281728d`
- Lane commit(s): `39d0947`, `a922b3307fb65766d8f55bd642d0ad55c0013643`, `61b1534b2cfc9b3d403ca358864beeb6266c0e66`
- Selected rows: `doDiscoverMageArmorUnarmoredSelfTarget`, `doRejectMageArmorArmoredTarget`, and `doExpireMageArmorDuration`.
- Obligations added: `0` accepted counted obligations; all three selected rows remain target-blocked.
- New total driver coverage: `97 / 97 = 100.0%`
- New total obligation coverage: `654 / 668 = 97.9%`
- Remaining blockers: `14` target-side blockers, `0` source-QNT corpus blockers, `0` unresolved in-scope obligations.
- Integration verification: focused Mage Armor adapter test, affected JSON parse checks, `cargo fmt --check`, `git diff --check HEAD~1...HEAD`, `cargo test` (`220` tests), and `cargo clippy --all-targets -- -D warnings` passed. `node scripts/check-cleanroom-harness.cjs` still fails on pre-existing stale global evidence/manifest debt; the saved output contained no CP7-D/FU01E/Mage Armor-specific failures.
- Review/fixer notes: initial worker output tried to accept Mage Armor through adapter-driven generic route synthesis. Review rejected that as unsupported by copied cleanroom input. The worker converted the lane into an honest blocker/research result and refreshed FU01E metadata hashes; rereview returned clean.
- Blocker recorded: the copied reducer-route inventory has no Mage Armor `qRoute` connector or generic armor-class target-admission/active-effect lifecycle route owner, so accepting these rows in target code would be inference beyond the cleanroom package.
- Worktrees marked removable: `/workspace/typescript/.codex-worktrees/dnd-cleanroom-l15-rrcp7-d`

### L15-RRCP7-E-MAGIC-WEAPON-ITEM-TARGET-ROUTES

- Merge commit: `24ad8f3b493b2affa4dba3607a619f92bf54eb16`
- Lane commit(s): `be0b02ec32b5182c031471812ca8fa1625183c51`, `d08c66e271cae1d4f3ddd5eb1ca101ad5b11c14c`
- Selected rows: `doDiscoverMagicWeapon` and `doFillMagicWeaponTarget`.
- Route connector: `cleanroom-input/qnt/battle-runtime/battle-runtime-weapon-enhancement-item-target.route.mbt.qnt`.
- Obligations added: `2` accepted counted obligations; total accepted obligations moved from `654` to `656`.
- New total driver coverage: `97 / 97 = 100.0%`
- New total obligation coverage: `656 / 668 = 98.2%`
- Remaining blockers: `12` target-side blockers, `0` source-QNT corpus blockers, `0` unresolved in-scope obligations.
- Integration verification: focused weapon-hosted route adapter test, affected JSON parse checks, `cargo fmt --check`, `git diff --check HEAD~1...HEAD`, `cargo test` (`220` tests), and `cargo clippy --all-targets -- -D warnings` passed. `node scripts/check-cleanroom-harness.cjs` still fails on pre-existing stale global evidence/manifest debt; the saved output contained no CP7-E/Magic Weapon/WeaponEnhancement-specific failures.
- Review/fixer notes: review confirmed the two accepted rows replay through observed reducer entrypoints and match `WeaponEnhancementItemTargetRouteSubject` with empty route holes, `BattleItemTargetBoundaryOwner` discovery, and no-fill resolution owned by `BattleActiveEffectOwner`. The only finding was a stale CP7 audit Run Ledger hash; the worker fixed it and rereview by inspection was clean.
- Explicit non-goals preserved at CP7-E: `MagicWeaponTargetItem` was not added as reducer-owned hole/fill vocabulary; the nine scenario transition rows and three Mage Armor rows remained blocked until the later source refresh and CP8 acceptance.
- Worktrees marked removable: `/workspace/typescript/.codex-worktrees/dnd-cleanroom-l15-rrcp7-e`

### L15-RRCP8-A-MAGE-ARMOR-GENERIC-AC-ROUTES

- Merge commit: `5b1e976b6af7fadefa4ea5a065ae81de53b94d09`
- Lane commit(s): `e4e95b013982241e1c5d510ea4de44cc592c11cd`, `a8b96ec0aa91b4249b352009b1384292f917f7e2`
- Selected rows: `doDiscoverMageArmorUnarmoredSelfTarget`, `doRejectMageArmorArmoredTarget`, and `doExpireMageArmorDuration`.
- Route connector: `cleanroom-input/qnt/battle-runtime/battle-runtime-spell-base-armor-class-effect.route.mbt.qnt`.
- Obligations added: `3` accepted counted obligations; refreshed total accepted obligations moved from `656` to `659`.
- New total driver coverage: `97 / 97 = 100.0%`
- New total obligation coverage: `659 / 659 = 100.0%`
- Remaining blockers: `0` target-side blockers, `0` source-QNT corpus blockers, `0` unresolved in-scope obligations.
- Integration verification: focused Mage Armor route adapter test, focused Mage Armor base Armor Class/duration test, affected JSON parse checks, `cargo fmt --check`, `git diff --check HEAD~1...HEAD`, `cargo test` (`220` tests), and `cargo clippy --all-targets -- -D warnings` passed. `node scripts/check-cleanroom-harness.cjs` still fails only on known stale validator hashes in `cleanroom-input/MANIFEST.md`; the saved output contained no CP8/FU01E/Mage Armor-specific failures.
- Review/fixer notes: initial review found the pre-existing base AC projection row still used the stale pre-refresh route shape. The follow-up commit refreshed that row to the generic connector shape: target-choice discovery, target-choice resolution, active-effect no-fill, and Armor Class no-fill. CP8 accounting still counts exactly the three newly accepted rows.
- Production identity boundary: no production Mage Armor/name/id/provenance dispatch was introduced. Production reducer behavior uses `BattleArmorClassSpellEffectFill`, `BattleSubjectKind::ArmorClassSpellEffect`, route holes/fills, and owner groups. Selected spell identity remains adapter/evidence row identity only.
- Worktrees marked removable: `/workspace/typescript/.codex-worktrees/dnd-cleanroom-l15-rrcp8-a`

### PACT-SLOT-HANDOFF-INIT-PROJECTION-ROUTE-REFRESH

- Merge commit: `1abe8f87fcff9e72efd2e006f549373ff9f00b83`
- Lane commit: `23eca8a08e055864630161ad953c65f00fb9027b`
- Worker: Beauvoir the 2nd (`019f0a7c-0f68-76c3-953e-cc2a149f15df`)
- Driver: `cleanroom-input/qnt/character-battle-runtime/character-battle-init-projection.mbt.qnt`
- Route connector: `cleanroom-input/qnt/character-battle-runtime/character-battle-init-projection.route.mbt.qnt`
- Evidence: `tasks/target-replay-evidence/pact-slot-handoff-init-projection-route.json`
- Result: dirty diagnostic replay evidence accepted for the refreshed Pact Slot handoff route surface. The mixed Spell Slot/Pact Slot rejection now matches the copied connector shape: `HandoffResourceProjectionRouteSubject`, `HandoffResourceDeltaFill`, `HandoffSpellResourceProjectionHoleFamily`, and `CharacterBattleResourceProjectionOwner`.
- Evidence checker: `node scripts/check-target-replay-evidence-file.cjs --driver cleanroom-input/qnt/character-battle-runtime/character-battle-init-projection.mbt.qnt --evidence tasks/target-replay-evidence/pact-slot-handoff-init-projection-route.json` passed with five obligations covered.
- Integration verification: `cargo fmt --check`, focused `battle_init_projection_adapter_replays_all_driver_branches`, `cargo test` (`220` tests), `cargo clippy --all-targets -- -D warnings`, and `git diff --check HEAD~1...HEAD` passed in the replay worktree before merge.
- Scope note: this does not change fresh target acceptance and does not add new canonical dirty-campaign denominator coverage beyond the already recorded 659/659. It proves the refreshed copied QNT route connector can constrain the existing dirty Rust target for these Pact Slot branches.
- Worktree marked removable: `/workspace/typescript/.codex-worktrees/dnd-cleanroom-pact-slot-replay`

### FC-07-PACT-SLOT-HANDOFF-REPLAY

- Target commit: `f0ee8f8eb95192639afe5b6af17764dfe46c5303`
- Setup commit: `8ec4bc46bfd961345c4f73115d8dc523c5d9163b`
- Implementation commit: `c6385e25aa87bae5dcb718fbc765b61157c4e1ba`
- Metadata fix commit: `f0ee8f8eb95192639afe5b6af17764dfe46c5303`
- Worker: Halley the 2nd (`019f0a88-a545-76e1-9b1b-ee15007116e1`)
- Evidence: `/workspace/typescript/.codex-worktrees/dnd-fresh-cleanroom-dry-run-fc00/EVIDENCE/fc07-pact-slot-handoff-route.json`
- Result: fresh target replay accepted the newly copied Pact Slot handoff route surfaces from source commit `b57772b459f1b75592fd45b9196fd60965b534d3`. Pure Pact Slot resource projection emits `HandoffResourceProjectionRouteSubject` through `CharacterBattleResourceProjectionOwner` and then enters runtime through `CharacterBattleInitProjectionOwner`; mixed ordinary Spell Slot/Pact Slot input rejects with `HandoffResourceDeltaFill` and `HandoffSpellResourceProjectionHoleFamily`.
- Runtime shape: production state now distinguishes ordinary spell resources from Pact Slot resources through typed resource shapes. Mixed ordinary/Pact input is representable at the sheet handoff boundary only so the refreshed QNT rejection route can be observed; accepted battle state has no mixed variant.
- Verification: `cargo fmt --check`, `cargo test` (`25` reducer-spine tests plus `3` SDK tests), `cargo clippy --all-targets -- -D warnings`, `python3 tools/verify_fc00.py`, `python3 tools/verify_sdk_tracer_bullet.py`, forbidden source/dirty-implementation path search, JSON parse checks, and `git diff --check 8ec4bc46bfd961345c4f73115d8dc523c5d9163b...HEAD` passed.
- Residual risk before FC-08: old FC-03/FC-04/FC-05 verifier scripts still pinned historical source-input hashes and failed after the package refresh on `cleanroom-input/guidance/reducer-spine.md`; FC-08 replaced that with a current verifier gate and historical snapshot classification.
- Worktree marked removable: `/workspace/typescript/.codex-worktrees/dnd-fresh-cleanroom-pact-slot-replay`

### FC-08-FRESH-VERIFIER-REFRESH

- Target commit: `a30e6729711ddc3f595cf008931ba5cd6265c58a`
- Implementation commit: `e460e9ed7f3448b139ded01128111e276a7fbb41`
- Static-boundary fix commit: `a30e6729711ddc3f595cf008931ba5cd6265c58a`
- Workers: Ampere the 2nd (`019f0aa1-145d-7c41-bfff-114cb4d6427d`) and Harvey the 2nd (`019f0aab-ccd3-70d3-9438-5cbe58724ba1`)
- Result: fresh target `master` fast-forwarded to the current verifier gate after the `b57772b459f1b75592fd45b9196fd60965b534d3` input package refresh. `python3 tools/verify_current_fresh_target.py` now validates the current package inventory, strict FC-07 Pact Slot source-input hashes and route comparisons, SDK tracer-bullet surface, and FC-03/FC-04/FC-05 historical snapshot classification.
- Static boundary: the broad forbidden source/dirty-implementation path search returns no matches while the verifier still checks for those paths at runtime.
- Verification in the accepted fresh target: `cargo fmt --check`, `cargo test` (`25` reducer-spine tests plus `3` SDK tests), `cargo clippy --all-targets -- -D warnings`, `python3 tools/verify_current_fresh_target.py`, and the forbidden source/dirty-implementation path search passed.
- Worktree marked removable: `/workspace/typescript/.codex-worktrees/dnd-fresh-cleanroom-verifier-refresh`

### SQNT-07-CURRENT-PACKAGE-DIRTY-REPLAY-BATCH

- Integration merge head: `4b2c415259ad5f3b10d281a536a5aa8499f926b7`
- Bookkeeping/provenance-cleanup head: `ebd3699fb03c99d0ec674361ebf69835e64bfd0c`
- Source package commit: `21504ef764118f5fd13086aa6266f19280196664`
- Lanes merged:
  - `SQNT-07B-SPECIES-PASSIVE-CURRENT-PACKAGE-REPLAY`: lane head `2e3b1e21ebfbe634263b5034d9a531cc34fccb0a`, merge commit `4b2c415259ad5f3b10d281a536a5aa8499f926b7`.
  - `SQNT-07C-METAMAGIC-CURRENT-PACKAGE-REPLAY`: lane head `45ebc0b1913c0ab178829461e2ca1cec6478c8bf`, merge commit `344111e89f012a2b1d05c6bf64660b30810c3217`.
  - `SQNT-07D-ACTIVE-FEATURE-BENEFITS-CURRENT-PACKAGE-REPLAY`: lane head `001945b955ac351a87163f804c4fca13663f7492`, merge commit `e1a35bc598b8f45fda2fc65c2ae0886f925d1305`.
- Result: dirty current-package replay accepted 15 species/passive-adjacent rows, 30 selected metamagic rows, and 3 active-feature spell-benefit rows through copied QNT route connectors and observed reducer entrypoints.
- Scope note: this is diagnostic dirty target evidence. It does not claim fresh target acceptance, and it does not close SQNT-07A selected/grouped residual source-input work.
- Out-of-scope rows: SQNT-07B explicitly leaves two inherited Adrenaline Rush feature-resource rows out of scope for this lane because they remain owned by the copied Adrenaline Rush connector.
- Review/fixer notes: the first SQNT-07D worker commit was rejected because it manually constructed observed route events. The accepted remediation obtains route events from `start..._observed`, `discover_battle_acts_observed`, and `resolve_battle_subject_observed`. SQNT-07B was also remediated after review found evidence-scope and count-reconciliation issues; the accepted branch records all connector authorities and the 15 accepted plus 2 out-of-scope split.
- Integration verification: focused SQNT-07B species/passive tests, focused SQNT-07C metamagic route connector test, focused SQNT-07D active-feature test and evidence checker, `cargo fmt --check`, `cargo test` (`234` tests), `cargo clippy --all-targets -- -D warnings`, and `git diff --check` passed.
- Harness status: after refreshing the guidance manifest hashes and carrying forward the approved validator patch to source package `21504ef764118f5fd13086aa6266f19280196664`, `node scripts/check-cleanroom-harness.cjs` now progresses past validator provenance. It fails on older historical replay/ledger debt still pinned to source package `564376fd95218a209bb9eae5c9ccb54ca3e04a52`, plus pre-existing heuristic findings in `src/rules/battle_reducer_spine.rs` and `src/rules/rule_core_exact_damage_projection.rs`. This batch does not attempt repo-wide historical evidence repair.
- Worktrees marked merged/closed: `/workspace/typescript/.codex-worktrees/dnd-cleanroom-sqnt07b-species-replay`, `/workspace/typescript/.codex-worktrees/dnd-cleanroom-sqnt07c-metamagic-replay`, and `/workspace/typescript/.codex-worktrees/dnd-cleanroom-sqnt07d-active-feature-replay`.

### SQNT-07A-MOVEMENT-PRESENTATION-SOURCE-QNT

- Source commit: `f7d2ea22d27e9457fd46014697fc55a4000f823e`
- Worker: Copernicus the 3rd (`019f0f94-4f64-7d12-8804-2f0c710d7c3f`)
- Reviewers: Faraday the 3rd (`019f0fa7-e124-7a61-8994-54b50699ff4a`) and McClintock the 3rd (`019f0fb2-1359-79a1-8f95-b11cb29fe559`)
- Result: source-QNT route facts accepted for generic movement replacement, forced creature movement, object-push/audible presentation, and table-supplied movement path/landing presentation witnesses.
- Review/fixer notes: initial review rejected a Jump overclaim that treated table-supplied landing presentation as route legality. The accepted fix renamed the fact to `TableSuppliedMovementPathWitness`, kept landing facts under table presentation, and preserved explicit blockers for landing legality and failed-landing Prone consequences.
- Verification: focused movement-presentation MBT, `pnpm check:mbt-driver-closure`, `pnpm check:reducer-route-connectors`, `pnpm rules-kernel-coverage:check`, `pnpm cleanroom-branch-coverage:check`, `pnpm --filter @dnd/battle-runtime typecheck`, and `git diff --check HEAD~1...HEAD` passed on source master after fast-forward.
- Scope note: this is source-input work only. It does not claim dirty target replay or fresh target acceptance; those wait for package refresh after the remaining Wave 2 source lanes land.
- Worktree marked removable: `/workspace/typescript/.codex-worktrees/dnd-source-sqnt-07a-movement-presentation`

### SQNT-07A-SPATIAL-EFFECTS-SOURCE-QNT

- Source commit: `c596f55f6ccea08a65e94804ede4ca443adf8bd4`
- Worker: Carson the 3rd (`019f0f94-1228-7dc0-b29a-a9ef5a01a908`)
- Reviewers: Ampere the 3rd (`019f0fb0-0a92-7862-ad2b-ab535a129b9a`) and Beauvoir the 3rd (`019f102d-7296-7a41-9b33-56a279d8f19b`)
- Result: source-QNT route facts accepted for movable multi-emitter light, outline/invisible-benefit denial/attack advantage, object outline, non-Concentration area hazard, separate movement-damage trigger, and obscurement duration/dispersal cleanup.
- Review/fixer notes: initial review rejected overbroad facts that made Grease-like hazards imply Concentration and movement damage, combined Fog Cloud strong-wind dispersal with duration cleanup, omitted Concentration for Dancing Lights-style movable emitters, and invented a SpellTargetList frontier for outline semantics. The accepted fix split those shapes and kept unsupported rows blocked.
- Verification: focused spatial route connector MBT, `pnpm check:mbt-driver-closure`, `pnpm check:reducer-route-connectors`, `pnpm rules-kernel-coverage:check`, `pnpm cleanroom-branch-coverage:check`, `pnpm --filter @dnd/battle-runtime typecheck`, and `git diff --check HEAD~1...HEAD` passed on source master after fast-forward.
- Scope note: this is source-input work only. It does not claim dirty target replay or fresh target acceptance; those wait for package refresh after the remaining Wave 2 source lane lands.
- Worktree marked removable: `/workspace/typescript/.codex-worktrees/dnd-source-sqnt-07a-spatial-effects`

### SQNT-07A-MARKED-DAMAGE-AND-IMMUNITY-SOURCE-QNT

- Source commit: `e9f75e22a10891cd438fb06f6ea1ca666f79aaeb`
- Worker: Singer the 3rd (`019f0f93-d066-77c3-b853-6f44a6ede5b1`)
- Reviewers: Curie the 3rd (`019f0fad-031e-7123-bd8d-bfc6f67838d1`), Ramanujan the 3rd (`019f102d-4d71-7cf0-a1c9-42ec12656a5b`), and Epicurus the 4th (`019f1037-62ae-7ac2-b436-ea96bba880d7`)
- Result: source-QNT route facts accepted for attack-roll marked damage, marked-damage transfer timing, condition-immunity rejection, turn-start Temporary Hit Points, and Concentration cleanup.
- Review/fixer notes: initial review rejected an unsupported `MarkedTargetAbilityCheckModifier` fact and missing coverage ownership. Follow-up review found generated coverage artifacts still claimed ability-check support. The accepted branch removes the ability-check route claim from QNT/TS and generated coverage, keeps ability-check roll-mode semantics out of this lane, and records only the supported marked damage/immunity/Temporary HP facts.
- Verification: focused marked damage/immunity route connector MBT, `pnpm check:mbt-driver-closure`, `pnpm check:reducer-route-connectors`, `pnpm rules-kernel-coverage:check`, `pnpm cleanroom-branch-coverage:check`, `pnpm --filter @dnd/battle-runtime typecheck`, `pnpm check:authored-id-dispatch`, and `git diff --check HEAD~1...HEAD` passed on source master after fast-forward.
- Scope note: this is source-input work only. It does not claim dirty target replay or fresh target acceptance. Wave 2 source-QNT lanes are now complete and ready for cleanroom-input package refresh.
- Worktree marked removable: `/workspace/typescript/.codex-worktrees/dnd-source-sqnt-07a-marked-damage-immunity`

### SQNT-07A-WAVE2-CLEANROOM-INPUT-REFRESH

- Source package commit: `e9f75e22a10891cd438fb06f6ea1ca666f79aaeb`
- Command: `node scripts/sync-cleanroom-input.cjs --target /workspace/typescript/.codex-worktrees/dnd-cleanroom-rrconv-19`
- Result: `cleanroom-input/` refreshed to 535 files with 1547 QNT imports resolved and 98 branch-inventory drivers checked.
- Newly packaged route connectors: condition lifecycle updates, `battle-runtime-movement-presentation.route.mbt.qnt`, `battle-runtime-spatial-effects.route.mbt.qnt`, and `battle-runtime-marked-damage-immunity-active-effects.route.mbt.qnt`.
- Validation: manifest validator hashes were updated through the approved `tasks/VALIDATOR_PATCH.json` carry-forward for source `e9f75e22a10891cd438fb06f6ea1ca666f79aaeb`; `node scripts/check-cleanroom-harness.cjs` then progressed past validator provenance and failed on historical dirty RUN_LEDGER/target replay evidence still pinned to older source packages plus pre-existing heuristic findings. This refresh does not attempt repo-wide dirty ledger repair.
- Scope note: this is a copied source-input package refresh only. It does not claim dirty target replay, fresh target acceptance, or Rust runtime coverage. Target replay lanes must consume only the copied QNT/RAW/domain/guidance package and must keep unsupported selected rows blocked.

### SQNT-07A-DIRTY-REPLAY-LANE-INTEGRATION

- Merge commits: spatial/movement `86c0c70b22fbb4e619a6f154a56b38c7382c7d22`; active-effect/condition `6d9712af44372a73134364778a9bbbc255527e24`.
- Lane heads: spatial/movement `ff870c11ae3f3d8dbc78f43cbb4b7cf6f3b65d36`; active-effect/condition `a4de4c49b45d8aff896078f90737746f61c61162`.
- Result: both dirty replay lanes are accepted-with-known-harness-debt. This is diagnostic dirty target replay only and does not claim fresh target acceptance.
- Accepted rows: spatial/movement accepted 16 route connector rows. Active-effect/condition accepted 18 condition connector rows, 9 marked/immunity connector rows, and exactly 4 selected level-one rows through copied generic route facts. The selected condition-saving refresh accepts 5 additional selected rows through copied generic route facts and carries forward the existing 4 selected rows without recounting them as new.
- Blockers preserved: Jump landing legality and failed-landing Prone; concentration-backed Sleet Storm / Spike Growth / Web-like hazards; 8 selected level-one rows outside lane generic route facts.
- Conflict resolution: shared `tasks/RUN_LEDGER.json`, `tasks/STATE_OWNER_MANIFEST.json`, and `tasks/VALIDATION_REPORT.md` were resolved by preserving both lanes' route support, tests, evidence, state-owner records, validation records, accepted rows, and blockers.
- Verification passed: `cargo fmt --check`, the four requested focused cargo tests, `cargo test` (`237` tests), `cargo clippy --all-targets -- -D warnings`, `git diff --check`, and JSON parse checks for touched JSON artifacts.
- Harness status: `node scripts/check-cleanroom-harness.cjs` still fails on historical stale ledger/evidence and pre-existing heuristic findings outside these lanes. Captured harness output had no SQNT-07A-specific matches.

### FRESH-RR-SQNT07A-PACKAGE-AND-REPLAY

- Fresh target head: `ead584abf5b6aa07f9365e4fdd3694f4c9dd18bb`.
- Package baseline refresh: `16993df27e490539c7b7a8ab8d9cea6aba167715` refreshed the fresh target to copied source package `e9f75e22a10891cd438fb06f6ea1ca666f79aaeb` and classified older runtime evidence as historical for the current gate.
- Active-effect/condition lane: worker head `efd1716c57a0942992f471cb202123650a92fe83`, integrated by `ead584abf5b6aa07f9365e4fdd3694f4c9dd18bb`. Accepted 27 copied connector action rows and 19 public reducer route surfaces for condition lifecycle plus marked damage/immunity active-effect facts.
- Spatial/movement lane: worker head `04b9f235e889b99cc70a1ea3a4d5a4d075cb280a`, integrated by `ead584abf5b6aa07f9365e4fdd3694f4c9dd18bb`. Accepted 16 copied spatial-effect and movement/presentation rows.
- Review/fixer notes: active-effect verifier now requires exact retained blockers and scans production for lane-relevant authored-identity dispatch. Spatial review found a real single-slot follow-up overwrite risk; the accepted fix uses a keyed follow-up queue and proves two pending spatial follow-ups cannot overwrite each other.
- Verification in fresh target: `python3 tools/verify_current_fresh_target.py`, `python3 tools/verify_fresh_rr_sqnt07a_active_effect_condition.py`, `python3 tools/verify_fresh_rr_sqnt07a_spatial_movement.py`, `cargo fmt --check`, `cargo test`, `cargo clippy --all-targets -- -D warnings`, and `git diff --check` passed in the integration lane before fast-forward.
- Residual blockers preserved at that checkpoint: selected level-1 marked/immunity
  residual rows later split into scalar-buff, after-hit/timed rider,
  weapon-hosted, and Hex ability-check residual groups;
  `FRESH-RR-SQNT07A-jump-landing-legality-and-failed-landing-prone-blocked`;
  and `FRESH-RR-SQNT07A-concentration-backed-area-hazards-blocked`.
- Scope note: this is fresh cleanroom replay acceptance for the generic SQNT-07A route facts now present in the copied cleanroom-input package. It does not accept broad selected/grouped identity drivers wholesale.

### FRESH-RR-SQNT07A-LEVEL1-MARKED-IMMUNITY-SELECTED-REPLAY

- Fresh target merge commit: `a7b4a30fae4b293ca48feec50041c9fa6a706db8`.
- Worker head: `e6f3a440ec69949dde17d351e3172b2897006b07`.
- Result: accepted-with-blockers. Exactly four selected level-1 rows route
  through copied generic marked-damage/immunity facts: Heroism frightened
  immunity turn-start Temporary Hit Points, Heroism cleanup, Hunter's Mark
  marked damage plus same-turn transfer, and Hex marked damage plus later-turn
  transfer.
- Review/fixer notes: review rejected overclaims that accepted transfer without
  marked-damage projection, local-current source hashes instead of exact copied
  input hashes, missing exact blocked-row checks, and transfer availability when
  a marked target was already at 0 HP. The accepted lane projects marked damage
  before transfer, pins exact copied input hashes, asserts exact blockers, and
  opens transfer only on an above-zero to zero HP transition.
- Verification in fresh target: `python3
  tools/verify_fresh_rr_sqnt07a_level1_marked_immunity_selected.py`, `python3
  tools/verify_current_fresh_target.py`, `cargo fmt --check`, `cargo test`,
  `cargo clippy --all-targets -- -D warnings`, and `git diff --check
  HEAD~1...HEAD` passed.
- Residual blockers preserved at that checkpoint: scalar-buff selected rows,
  after-hit/timed rider selected rows, weapon-hosted selected rows, Hex
  ability-check roll-mode, Jump landing legality and failed-landing Prone, and
  concentration-backed area hazards. The scalar-buff subset is closed by
  `FRESH-RR-SQNT07A-LEVEL1-SCALAR-BUFF-SELECTED-REPLAY`.
- Scope note: this is selected-row acceptance only where the selected rows
  reduce to generic copied QNT route facts. It does not accept broad selected
  identity replay.

### FRESH-RR-SQNT07A-LEVEL1-SCALAR-BUFF-SELECTED-REPLAY

- Fresh target merge commit: `6f526cb20165a00707bf90c59096087464a5d108`.
- Worker head: `9a79b6942a8a2e839f5be04e3e0663c21754fd3b`.
- Worker: Planck the 4th (`019f129d-43d6-7250-9d4f-5b4a0e11ecdd`).
- Reviewer: Fermat the 4th (`019f12ab-9d5e-7f50-94e4-d1ee2609f2c8`).
- Result: accepted-with-blockers. Exactly two selected level-1 rows route
  through copied generic scalar-buff facts: False Life immediate Temporary Hit
  Points and Longstrider Speed increase.
- Review/fixer notes: orchestrator pre-review caught a duplicate evidence JSON
  key before commit; the worker fixed it and added duplicate-key checks. The
  independent reviewer found no findings. The reviewer noted residual trust in
  the narrow copied-QNT parser, mitigated by source hash pins, duplicate-key
  rejection, public reducer observations, and aggregate verifier coverage.
- Verification in fresh target: `python3
  tools/verify_fresh_rr_sqnt07a_level1_scalar_buff_selected.py`, `python3
  tools/verify_fresh_rr_sqnt07a_level1_marked_immunity_selected.py`, `python3
  tools/verify_current_fresh_target.py`, `cargo test sqnt07a_level1_ --
  --nocapture`, `cargo fmt --check`, `cargo test`, `cargo clippy --all-targets
  -- -D warnings`, and `git diff --check HEAD~1...HEAD` passed after merge.
- Residual blockers preserved: after-hit/timed rider selected rows,
  weapon-hosted selected rows, Hex ability-check roll-mode, Jump landing
  legality and failed-landing Prone, and concentration-backed area hazards.
- Scope note: this is selected-row acceptance only where the selected rows
  reduce to generic copied scalar-buff route facts. It does not accept broad
  selected identity replay.

### FRESH-RR-SQNT07A-LEVEL1-AFTER-HIT-TIMED-RIDER-SELECTED-REPLAY

- Fresh target merge commit: `9737474a21fb77df382cf3504dd8a4b3b46ffb5d`.
- Worker head: `73d7da78b5e350cefc4115292f3fce119885a499`.
- Worker: Bacon the 4th (`019f12b7-3baa-7752-ba05-b524f5a94e49`).
- Reviewer: Faraday the 4th (`019f12c3-7c4b-76f3-a5ce-86164de59e7e`).
- Result: accepted-with-blockers. Exactly three selected level-1 rows route
  through copied generic after-hit rider route/lifecycle/owner facts: Divine
  Smite after-hit damage, Ensnaring Strike restraint/turn-start/escape, and
  Searing Smite timed damage/save cleanup.
- Review/fixer notes: reviewer found verifier and bookkeeping weaknesses. The
  follow-up commit broadened production authored-identity scans, made the
  focused verifier check `BLOCKERS.json`, `FRESH_RUN_STATE.json`, and
  `FRESH_RUN_REPORT.md`, and preserved older checkpoint after-hit blockers as
  superseded records instead of deleting them.
- Verification in fresh target: `python3
  tools/verify_fresh_rr_sqnt07a_level1_after_hit_timed_selected.py`, `python3
  tools/verify_current_fresh_target.py`, `cargo test
  after_hit_timed_save_cleanup_surface_is_publicly_routed -- --nocapture`,
  `cargo fmt --check`, `cargo test`, `cargo clippy --all-targets -- -D
  warnings`, and `git diff --check HEAD~1...HEAD` passed after merge.
- Residual blockers preserved: exact after-hit/timed damage type, dice, and
  amount, weapon-hosted selected rows, Hex ability-check roll-mode, Jump landing
  legality and failed-landing Prone, and concentration-backed area hazards.
- Scope note: this is selected-row acceptance only where the selected rows
  reduce to generic copied after-hit route facts. It does not accept exact
  damage details or broad selected identity replay.

### FRESH-RR-SQNT07A-LEVEL1-WEAPON-HOSTED-SELECTED-REPLAY

- Fresh target merge commit: `ce9f653e3cba6a9eefa0d2f14e11757f7081e618`.
- Worker head: `6216453d8599c3ab5069e647ffd5109573c79c78`.
- Worker: Volta the 4th (`019f12d4-3c6b-7b42-911a-422e3b0f5c63`).
- Reviewers: Ptolemy the 4th
  (`019f12e6-e23c-7032-97e4-88ff3cb3fc18`) and Zeno the 4th
  (`019f12f0-86d0-7010-91ee-dd9700f70b90`).
- Result: accepted-with-blockers. Generic weapon-hosted route-surface groups
  route through copied connector-derived expected records and public reducer
  observations:
  `WeaponDamageRiderGenericRouteSurface`,
  `HeldWeaponActiveEffectGenericRouteSurface`, and
  `SpellHostedWeaponAttackGenericRouteSurface`.
- Review/fixer notes: the initial worker claim was too strong. The lane was
  downgraded so `acceptedSelectedRows` is empty, selected-row exactness remains
  blocked without a copied public selected-row route witness, production
  `src/**/*.rs` is scanned for lane-relevant authored spell/item/row identity
  terms, older after-hit blocker history remains blocked, and the verifier
  checks `STATE_OWNERS.md` wording.
- Verification in fresh target: `python3
  tools/verify_fresh_rr_sqnt07a_level1_weapon_hosted_selected.py`, `python3
  tools/verify_current_fresh_target.py`, `cargo test sqnt07a_level1_ --
  --nocapture`, `cargo fmt --check`, `cargo test`, `cargo clippy
  --all-targets -- -D warnings`, and `git diff --check HEAD~1...HEAD` passed
  after merge.
- Residual blockers preserved: selected-row-to-public-route witness for
  weapon-hosted exactness, exact weapon-hosted arithmetic and identity details,
  exact after-hit/timed damage details, Hex ability-check roll-mode, Jump
  landing legality and failed-landing Prone, and concentration-backed area
  hazards.
- Scope note: this is not selected-row exactness acceptance and not broad
  selected identity replay.
