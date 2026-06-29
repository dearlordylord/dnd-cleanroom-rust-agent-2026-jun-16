# COMP-AUDIT-02 Historical Foundation Renewal Decision

Lane: `COMP-AUDIT-02-HISTORICAL-FOUNDATION-RENEWAL-DECISION`

Audit date: 2026-06-29

## Scope And Base Check

- Worktree: `/workspace/typescript/.codex-worktrees/dnd-comp-audit-02-foundation-renewal`
- Branch: `ralph/comp-audit-02-foundation-renewal`
- Declared base SHA: `73e2db4eb5ab420123a6eb8cc7a803f0b5d091d5`
- `git rev-parse HEAD` at audit start: `73e2db4eb5ab420123a6eb8cc7a803f0b5d091d5`
- Base ancestry command: `git merge-base --is-ancestor 73e2db4eb5ab420123a6eb8cc7a803f0b5d091d5 HEAD`
- Base ancestry result: passed, exit code `0`

Accepted fresh target inspected:

- Worktree: `/workspace/typescript/.codex-worktrees/dnd-fresh-cleanroom-dry-run-fc00`
- Fresh target `HEAD`: `ff09b6579ebb507f3c8dca06f2b040d0d46d05fb`
- Current source package: `4d196258a51f4264803ff11f7c806c969f0aff2d`
- Current package manifest hash: `0f3f8b73a42a3f72cb72b150a156933de9e25f545bfabbff1307da32f021b417`
- Current inventory: `100` drivers, `667` in-scope obligations, `61` out-of-scope obligations, `544` copied input files
- Aggregate verifier: `python3 tools/verify_current_fresh_target.py`

Inspected current-package evidence files:

- Fresh target: `FRESH_RUN_STATE.json`, `FRESH_EXPANSION_STATE.json`, `FRESH_EXPANSION_REPORT.md`, `tools/verify_current_fresh_target.py`, focused verifier files under `tools/`, and current `EVIDENCE/*.json` named by the state files.
- Campaign worktree: `STATE.json`, `PLAN.md`, `CHECKPOINT_REPORT.md`, `GOAL_STATUS_AUDIT.md`, `FRESH_CLEANROOM_READINESS.md`, `FRESH_CLEANROOM_DRY_RUN_PLAN.md`, `FRESH_CLEANROOM_EXPANSION_PLAN.md`, `FRESH_EXPANSION_LANES.json`, and `WORKTREE_LEDGER.md`.

`FRESH_SDK_COMPOSITION_ACCEPTANCE.md` is not present in the accepted fresh target inspected. The SDK tracer artifact that exists there is `EVIDENCE/sdk-tracer-bullet-programmatic-surface.json`, and the current aggregate gate classifies it as historical snapshot evidence.

## Decision Rule

The completion claim must use current-package evidence, not stale dirty or older fresh snapshots. A historical row is counted only if it is:

- `renewed-current`: the same row family has a current-package focused verifier and evidence in the accepted fresh target;
- `superseded-current`: the old artifact is no longer counted directly, but a stronger current-package claim covers the proof need with a narrower or better-shaped current verifier;
- `exclude-until-renewed`: no current-package claim replaces the artifact for the same proof need, so the artifact must not contribute to completion until a renewal lane exists;
- `optional-dirty-retention`: historical dirty/fresh rehearsal evidence may be retained as diagnostic context only and is not needed for the current completion claim.

The controlling mechanical fact is that `tools/verify_current_fresh_target.py` names these stale artifacts in `HISTORICAL_PHASES` and requires `FRESH_RUN_STATE.json.verification.historicalEvidenceClassification[]` to mark them `historical-snapshot-evidence` with `currentPackageHashRecomputed: false`: FC-01, FC-02, FC-03, FC-04, FC-05, FC-07, SDK tracer, FEXP-01, FEXP-02, FEXP-03, FEXP-04, FEXP-05, FEXP-07, older `FRESH-RR-FEATURE-SPECIES-METAMAGIC`, and older `FRESH-RR-SQNT07A-SPATIAL-MOVEMENT`.

## Renewal Matrix

| Historical artifact | Decision | Current replacement or exclusion basis |
| --- | --- | --- |
| FC-00 package admission and inventory | renewed-current | Current package admission is renewed by `tools/verify_fc00.py` and by `tools/verify_current_fresh_target.py`, which recompute the current `4d196258a51f4264803ff11f7c806c969f0aff2d` package manifest, file count, route connector inventory, `100` drivers, `667` in-scope obligations, and `61` out-of-scope obligations. |
| FC-01 reducer spine substrate | superseded-current | The old `EVIDENCE/fc01-reducer-substrate.json` is historical. Its proof need is superseded by the accepted current-package focused verifiers that all observe public reducer entrypoints, especially `tools/verify_fresh_rr_battle_active_effects.py`, `tools/verify_fresh_rr_reaction_interrupt_taxonomy.py`, `tools/verify_fresh_rr_sqnt07a_hex_exact_damage_source_consumption.py`, and the aggregate current gate. |
| FC-02 minimal weapon attack route | superseded-current | The old `EVIDENCE/fc02-weapon-attack-route.json` is historical. Current weapon/item/attack route capability evidence is stronger and more focused in `EVIDENCE/fresh-rr-sqnt07a-level1-weapon-hosted-selected.json` and `EVIDENCE/fresh-rr-sqnt07a-hex-exact-damage-source-consumption.json`, verified by `tools/verify_fresh_rr_sqnt07a_level1_weapon_hosted_selected.py`, `tools/verify_fresh_rr_sqnt07a_hex_exact_damage_source_consumption.py`, and the aggregate gate. Exact authored item/attack/spell identity remains outside production runtime dispatch. |
| FC-03 save-gated spell ordering route | superseded-current | The old `EVIDENCE/fc03-save-gated-spell-ordering-route.json` is historical. Current save-mode and save-triggered damage proof needs are covered by `tools/verify_fexp09d_sqnt07c_metamagic_save_modes.py`, `tools/verify_fresh_rr_sqnt07a_concentration_area_hazard.py`, and `tools/verify_fresh_rr_sqnt07a_hex_exact_damage_source_consumption.py`. Broad grouped selected-spell rows are not renewed and remain excluded unless a future completion claim requires them. |
| FC-04 base Armor Class effect route | exclude-until-renewed | The old `EVIDENCE/fc04-base-armor-class-effect-route.json` is historical and no current-package accepted claim in `FRESH_RUN_STATE.json.verification.currentPackageRuntimeAcceptanceClaims[]` renews the generic spell base Armor Class effect route. Dirty CP8 Mage Armor route acceptance is retained only as diagnostic rehearsal evidence. If base Armor Class effect routing is required for the global proof, it needs a current-package fresh renewal lane. |
| FC-05 character battle init projection route | renewed-current | The old `EVIDENCE/fc05-character-battle-init-projection-route.json` is historical, but the same character/sheet/handoff proof family is renewed by `EVIDENCE/fexp06-character-creation-sheet-handoff-pack.json`, verified by `tools/verify_fexp06.py`, and included in the aggregate gate as `FRESH-RR-CHARACTER-SHEET-HANDOFF-CURRENT-PACKAGE-REPLAY` with `26` accepted route surfaces and `0` blocked route surfaces. |
| FC-07 Pact Slot handoff route | renewed-current | The old `EVIDENCE/fc07-pact-slot-handoff-route.json` is historical. Pact-slot/resource handoff is renewed inside `FRESH-RR-CHARACTER-SHEET-HANDOFF-CURRENT-PACKAGE-REPLAY` through `EVIDENCE/fexp06-character-creation-sheet-handoff-pack.json` and `tools/verify_fexp06.py`, which derive expected route records from copied character-sheet and character-battle QNT route connectors. |
| SDK tracer bullet | superseded-current | `EVIDENCE/sdk-tracer-bullet-programmatic-surface.json` is historical and `FRESH_SDK_COMPOSITION_ACCEPTANCE.md` is absent from the inspected fresh target. Its reducer-surface and character/battle proof need is superseded for completion by current `FRESH-RR-CHARACTER-SHEET-HANDOFF-CURRENT-PACKAGE-REPLAY`, `FRESH-RR-BATTLE-ACTIVE-EFFECTS-CURRENT-PACKAGE-REPLAY`, and the aggregate current gate. Do not count the exact old SDK composition artifact as current-package evidence. |
| FEXP-00 baseline lock | renewed-current | Baseline/package admission is renewed by the current accepted fresh target `ff09b6579ebb507f3c8dca06f2b040d0d46d05fb`, `FRESH_EXPANSION_STATE.json.currentPackageAdmission`, and `tools/verify_current_fresh_target.py`. It is bookkeeping evidence, not runtime coverage. |
| FEXP-01 diagnostic battle route pack | exclude-until-renewed | `EVIDENCE/fexp01-diagnostic-battle-route-pack.json` is historical. Current public reducer verifiers supersede the general reducer-shape lesson, but the exact diagnostic pack rows are not current-package runtime claims. Do not count FEXP-01 row coverage in completion unless a focused current-package replay is added. |
| FEXP-02 spell-attack/save-gated unblock | exclude-until-renewed | `EVIDENCE/fexp02-spell-attack-save-gated-unblock.json` is historical. Current save-mode and hazard lanes cover narrower save-related proof needs, but the old FEXP-02 generic spell-attack/save-gated substrate and its eight selected-branch subset claim are not renewed as current-package evidence. Residual selected spell effects remain excluded. |
| FEXP-03 chained/object spell attacks | superseded-current | `EVIDENCE/fexp03-chained-and-object-spell-attacks.json` is historical. Its object/light and mixed-target public-route proof need is superseded by `EVIDENCE/fresh-rr-spatial-damage.json`, verified by `tools/verify_fresh_rr_spatial_damage.py` and the aggregate gate as `FRESH-RR-SPATIAL-DAMAGE-CURRENT-PACKAGE-REPLAY`. The current replacement is accepted-with-blockers: movable multi-emitter light, table presentation/invisible-revealing facts, grouped selected identity, exact component facts, and stale object route-history remain blocked. |
| FEXP-04 active-effect lifecycle and roll modifiers | superseded-current | `EVIDENCE/fexp04-active-effect-lifecycle-and-roll-modifiers.json` is historical. Its active-effect, roll/scalar, and concentration proof needs are superseded by current granular claims: `FRESH-RR-BATTLE-ACTIVE-EFFECTS-CURRENT-PACKAGE-REPLAY` (`tools/verify_fresh_rr_battle_active_effects.py`), `FRESH-RR-SQNT07A-LEVEL1-SCALAR-BUFF-SELECTED-REPLAY` (`tools/verify_fresh_rr_sqnt07a_level1_scalar_buff_selected.py`), `FRESH-RR-SQNT07A-CONCENTRATION-AREA-HAZARD-REPLAY` (`tools/verify_fresh_rr_sqnt07a_concentration_area_hazard.py`), and `FRESH-RR-SQNT07A-HEX-AND-EXACT-DAMAGE-SOURCE-CONSUMPTION` (`tools/verify_fresh_rr_sqnt07a_hex_exact_damage_source_consumption.py`). |
| FEXP-05 reaction interrupt and boundary | superseded-current | `EVIDENCE/fexp05-reaction-interrupt-and-boundary.json` is historical. Current generic reaction/interrupt payload taxonomy is accepted in `EVIDENCE/fresh-rr-reaction-interrupt-taxonomy.json`, verified by `tools/verify_fresh_rr_reaction_interrupt_taxonomy.py`, and battle active-effect support is covered by `tools/verify_fresh_rr_battle_active_effects.py`. Exact selected reaction spell projections remain excluded. |
| FEXP-06 character creation/sheet/handoff pack | renewed-current | Although not in the aggregate historical list, campaign docs still cite it as foundation evidence. It is current-package runtime evidence now: `EVIDENCE/fexp06-character-creation-sheet-handoff-pack.json`, `tools/verify_fexp06.py`, and the aggregate gate accept `26` public character/sheet/handoff route surfaces with no blocked route surfaces. |
| FEXP-07 feature/species/metamagic substrates | superseded-current | `EVIDENCE/fexp07-feature-species-metamagic-substrates.json` and older `FRESH-RR-FEATURE-SPECIES-METAMAGIC` are historical. Their proof need is superseded by split current-package claims: `FEXP-09A` through `FEXP-09G`, with focused verifiers `tools/verify_fexp09a_sqnt07d_active_feature.py`, `tools/verify_fexp09b_sqnt07b_species_passive.py`, `tools/verify_fexp09c_sqnt07c_quickened_governor.py`, `tools/verify_fexp09d_sqnt07c_metamagic_save_modes.py`, `tools/verify_fexp09e_sqnt07c_metamagic_spell_shape.py`, `tools/verify_fexp09f_sqnt07c_metamagic_rerolls.py`, and `tools/verify_fexp09g_sqnt07c_metamagic_component_duration.py`. |
| Older `FRESH-RR-SQNT07A-SPATIAL-MOVEMENT` | renewed-current | The aggregate gate classifies older `EVIDENCE/fresh-rr-sqnt07a-spatial-movement.json` as historical, but current selected-spatial evidence is renewed by `EVIDENCE/fexp08-sqnt07a-selected-spatial-current-package-replay.json`, verified by `tools/verify_fresh_rr_sqnt07a_spatial_movement.py`, and included as `FEXP-08-SQNT07A-SELECTED-SPATIAL-CURRENT-PACKAGE-REPLAY` with `10` accepted route surfaces and `0` blockers. |
| Dirty CP8 Mage Armor/base Armor Class rehearsal | optional-dirty-retention | Campaign docs record `659 / 659` dirty rehearsal coverage and CP8 Mage Armor acceptance. This remains useful diagnostic evidence only. It must not be counted as fresh current-package proof unless renewed in the accepted fresh target. |

## Current Claims That Replace Historical Foundation Rows

The accepted current-package runtime claim set in `FRESH_RUN_STATE.json`, `FRESH_EXPANSION_STATE.json`, and `tools/verify_current_fresh_target.py` is:

- `FRESH-RR-SQNT07A-JUMP-LANDING-PRONE-REPLAY` via `tools/verify_fresh_rr_sqnt07a_jump_landing_prone.py`
- `FRESH-RR-REACTION-INTERRUPT-TAXONOMY-CURRENT-PACKAGE-REPLAY` via `tools/verify_fresh_rr_reaction_interrupt_taxonomy.py`
- `FRESH-RR-BATTLE-ACTIVE-EFFECTS-CURRENT-PACKAGE-REPLAY` via `tools/verify_fresh_rr_battle_active_effects.py`
- `FEXP-08-SQNT07A-SELECTED-SPATIAL-CURRENT-PACKAGE-REPLAY` via `tools/verify_fresh_rr_sqnt07a_spatial_movement.py`
- `FRESH-RR-SQNT07A-LEVEL1-WEAPON-HOSTED-SELECTED-REPLAY` via `tools/verify_fresh_rr_sqnt07a_level1_weapon_hosted_selected.py`
- `FRESH-RR-SQNT07A-CONCENTRATION-AREA-HAZARD-REPLAY` via `tools/verify_fresh_rr_sqnt07a_concentration_area_hazard.py`
- `FEXP-09A-SQNT07D-ACTIVE-FEATURE-CURRENT-PACKAGE-REPLAY` via `tools/verify_fexp09a_sqnt07d_active_feature.py`
- `FEXP-09B-SQNT07B-SPECIES-PASSIVE-CURRENT-PACKAGE-REPLAY` via `tools/verify_fexp09b_sqnt07b_species_passive.py`
- `FEXP-09C-SQNT07C-QUICKENED-GOVERNOR-CURRENT-PACKAGE-REPLAY` via `tools/verify_fexp09c_sqnt07c_quickened_governor.py`
- `FEXP-09D-SQNT07C-METAMAGIC-SAVE-MODES` via `tools/verify_fexp09d_sqnt07c_metamagic_save_modes.py`
- `FEXP-09E-SQNT07C-METAMAGIC-SPELL-SHAPE` via `tools/verify_fexp09e_sqnt07c_metamagic_spell_shape.py`
- `FEXP-09F-SQNT07C-METAMAGIC-REROLLS` via `tools/verify_fexp09f_sqnt07c_metamagic_rerolls.py`
- `FEXP-09G-SQNT07C-METAMAGIC-COMPONENT-DURATION` via `tools/verify_fexp09g_sqnt07c_metamagic_component_duration.py`
- `FRESH-RR-CHARACTER-SHEET-HANDOFF-CURRENT-PACKAGE-REPLAY` via `tools/verify_fexp06.py`
- `FRESH-RR-SQNT07A-HEX-AND-EXACT-DAMAGE-SOURCE-CONSUMPTION` via `tools/verify_fresh_rr_sqnt07a_hex_exact_damage_source_consumption.py`

These are the only fresh current-package runtime claims this audit treats as current completion evidence.

## Minimal Renewal Lanes If Historical Foundation Is Still Required

No renewal lane is needed for rows marked `renewed-current` or `superseded-current` if the completion claim is framed around the current accepted claim set above.

If the global proof still requires exact historical row coverage rather than current stronger replacements, the minimal renewal set is:

1. `FC-04` current-package base Armor Class effect fresh replay, because no current accepted fresh claim renews `EVIDENCE/fc04-base-armor-class-effect-route.json`.
2. `FEXP-01` current-package diagnostic battle route pack replay, if Magic Missile, Hit Point restoration ordering, Death Saving Throw, and Concentration teardown must be claimed as those exact diagnostic rows.
3. `FEXP-02` current-package spell-attack/save-gated replay, if the exact generic spell-attack/save-gated substrate and eight selected-branch subset claim remains required.

Optional, only if the completion statement explicitly requires exact integrated SDK currentness:

4. SDK tracer current-package composition replay, because the current gate classifies `EVIDENCE/sdk-tracer-bullet-programmatic-surface.json` as historical and `FRESH_SDK_COMPOSITION_ACCEPTANCE.md` is absent in the inspected accepted fresh target.

Do not count dirty CP8 Mage Armor/base Armor Class rehearsal, old SDK tracer evidence, or stale FEXP-01/FEXP-02 artifacts toward completion unless one of those renewal lanes is run and added to `tools/verify_current_fresh_target.py`.

## Completion Impact

This audit does not claim the global campaign goal is complete. It only decides which foundation artifacts may contribute to a future completion claim.

The safe completion basis today is the accepted fresh target at `ff09b6579ebb507f3c8dca06f2b040d0d46d05fb`, source package `4d196258a51f4264803ff11f7c806c969f0aff2d`, and the current runtime claims mechanically enforced by `python3 tools/verify_current_fresh_target.py`.

All older dirty/fresh foundation artifacts listed in the aggregate historical classification are stale snapshots unless explicitly renewed or superseded above.
