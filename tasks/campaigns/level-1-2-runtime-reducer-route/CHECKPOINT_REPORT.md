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
| CP2 Rule-Core Component Connectors | ready-needs-split | CP1 is complete. Split `L15-RR04` into component sublanes before execution. |
| CP3 Character Creation, Sheet, And Handoff | blocked-on-checkpoint | Depends on CP2. Creation and sheet can run in parallel; handoff merges after both. |
| CP4 Feature And Catalog Substrates | blocked-on-checkpoint | Depends on CP2 and CP3. Split large FU lanes before execution. |
| CP5 Remaining Battle Families | blocked-on-checkpoint | Depends on CP1, CP2, CP4. |
| CP6 Closure Sweep | blocked-on-checkpoint | Runs after all implementation checkpoints. |

## Last Known Verification

At `1aa2ff3c6e4ca9d466a8eb0b8bc312ad3eeda025`:

- `cargo fmt --check`: pass
- `cargo test`: pass, `185 passed`
- `cargo clippy --all-targets -- -D warnings`: pass
- `node scripts/check-cleanroom-harness.cjs`: pass
- `git diff --check HEAD~1...HEAD`: pass
- JSON parse for `STATE.json`, `LANES.json`, `tasks/ENGINE_DEPTH_MANIFEST.json`, `tasks/RUN_LEDGER.json`, and `tasks/STATE_OWNER_MANIFEST.json`: pass
- `cargo test adapter_replays_all_branches --quiet`: pass, `70 passed`

## Active Work

- `L15-RR03-FINISH-CURRENT-DIAGNOSTIC-QUEUE` merged at `4c7e12d7645360adb7ab23af61144ceb243c13fe`. Lane head `7ef32d308d51fb54d1032d01b937d168fa63bb64` includes reducer-route evidence for death saving throw and concentration teardown plus the harness quarantine fix.
- `L15-RR05-BATTLE-ACTION-ATTACK-STATBLOCK-ROUTES` merged at `a235602664bbae19c3bfac5e38b85b1bbc4c23a5`. Lane head `1b928b16bfed2c87ad95efb6aae0a5d384fdb903` covers seven RR05 drivers and 53 route obligations.
- `L15-RR06-BATTLE-SPELL-EFFECT-ROUTES` merged at `1aa2ff3c6e4ca9d466a8eb0b8bc312ad3eeda025`. Lane head `9d17264679d8207c716f51148c52418629684891` covers command/scalar spell-effect routes and closes CP1.
- Next checkpoint: split `L15-RR04-RULE-CORE-COMPONENT-CONNECTORS` before launching CP2 Ralph lanes.

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
