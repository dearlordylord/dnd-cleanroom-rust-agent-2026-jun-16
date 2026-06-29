# COMP-AUDIT-03 Blocker To Lane Map

Campaign: `level-1-2-runtime-reducer-route`
Lane: `COMP-AUDIT-03-BLOCKER-TO-LANE-MAP`
Audit date: 2026-06-29

## Base And Input Check

- Declared base SHA: `73e2db4eb5ab420123a6eb8cc7a803f0b5d091d5`
- Worktree: `/workspace/typescript/.codex-worktrees/dnd-comp-audit-03-blocker-map`
- Branch: `ralph/comp-audit-03-blocker-map`
- `git rev-parse HEAD`: `73e2db4eb5ab420123a6eb8cc7a803f0b5d091d5`
- Base ancestry check: `git merge-base --is-ancestor 73e2db4eb5ab420123a6eb8cc7a803f0b5d091d5 HEAD` passed.
- Accepted fresh target inspected: `/workspace/typescript/.codex-worktrees/dnd-fresh-cleanroom-dry-run-fc00`
- Fresh target `HEAD` inspected: `ff09b6579ebb507f3c8dca06f2b040d0d46d05fb`
- Fresh target branch inspected: `master`
- Current source package inspected: `4d196258a51f4264803ff11f7c806c969f0aff2d`
- Fresh package admission inspected: `BLOCKERS.json.currentPackageAdmission.sourceCommitSha = 4d196258a51f4264803ff11f7c806c969f0aff2d`, file count `544`, manifest SHA-256 `0f3f8b73a42a3f72cb72b150a156933de9e25f545bfabbff1307da32f021b417`.
- Fresh verifier inspected and run: `python3 tools/verify_current_fresh_target.py`, which passed and reported `100` drivers, `667` in-scope obligations, and `61` out-of-scope obligations.

This audit uses `BLOCKERS.json.phaseBlockers` as retained branch-level blocker evidence. `BLOCKERS.json.blockers` is empty in the accepted fresh target, so none of the rows below should be read as a failed current gate.

## Classification Rules

- Grouping is by reducer substrate family, not selected spell, feature, species, item, action, or authored catalog identity.
- A source-QNT lane is proposed only when the copied package lacks generic route facts, typed guidance, or a branch-level admission map.
- A fresh-replay lane is proposed when source input appears packaged but current fresh target runtime evidence is still missing.
- Broad selected/grouped identity rows are not executable by themselves. They only spawn work after they are split into generic substrate families.
- Aggregate target files such as `BLOCKERS.json`, `FRESH_RUN_STATE.json`, `FRESH_EXPANSION_STATE.json`, and `tools/verify_current_fresh_target.py` should be updated by a serial integration lane after parallel workers land their focused evidence.

## Retained Blocker Clusters

| Blocker ids or branch refs | Generic substrate family | Missing input class | Proposed lane id | Lane type | Dependencies | Disjoint write scope notes |
| --- | --- | --- | --- | --- | --- | --- |
| `SQNT-07A-condition-selected-identity-adapter-not-refreshed`; condition-saving branch refs in `battle-runtime-condition-saving-throw-selected-identity.mbt.qnt` | Selected condition-saving substrate: save-gated condition admission plus repeat-save/cleanup projection through condition lifecycle owners | Fresh target runtime replay. Generic condition rider, save-gated, and repeat-save source facts are present in the current package; the missing piece is selected-row-to-generic-route replay evidence without identity dispatch. | `FRESH-SQNT07A-CONDITION-SAVING-SELECTED-REPLAY` | `fresh-replay` | Current fresh package `4d196258`; accepted `FRESH-RR-BATTLE-ACTIVE-EFFECTS-CURRENT-PACKAGE-REPLAY`; accepted `FRESH-RR-SQNT07A-ACTIVE-EFFECT-CONDITION`; copied condition-rider and repeat-save route facts. | Fresh target only. Focus on a new condition-saving evidence file, observation example, focused verifier, and any minimal generic runtime route support. Do not edit source QNT. Aggregate verifier/state updates should be serialized after the focused lane. |
| `FEXP-05-selected-reaction-spell-projections-need-generic-source-shapes`; `FRESH-RR-REACTION-INTERRUPT-TAXONOMY-selected-reaction-spell-projections-remain-outside-taxonomy` | Reaction projection substrate: reaction Armor Class projection, after-damage save/damage payloads, spell-interruption end/resume, and slot-preservation facts as typed trigger/procedure/continuation shapes | Source-QNT/guidance. Current taxonomy facts admit generic trigger/procedure/continuation rows, but retained selected projection rows still need exact runtime projection facts independent of authored identity. | `SOURCE-REACTION-PROJECTION-FACTS` | `source-qnt` | Accepted `FRESH-RR-REACTION-INTERRUPT-TAXONOMY-CURRENT-PACKAGE-REPLAY`; copied reaction casting-time and payload taxonomy route connectors; RAW/domain anchors for Reaction, interruption, Saving Throw, Damage, and Armor Class effect projection. | Source package only. Expected scope is focused reaction QNT/guidance and branch-inventory records. No fresh target runtime files should be edited until the package refresh is merged. |
| `FEXP-03-object-stale-replay-isolated-route-surface-not-publicly-observable`; current `FRESH-RR-SPATIAL-DAMAGE-CURRENT-PACKAGE-REPLAY` retained stale-object row | Object-target stale route history: public reducer history from object boundary through attack/damage/active-effect to stale subject rejection | Fresh target runtime replay. FCSF-04 source input is packaged as route-history evidence, but current fresh acceptance still does not prove the stale sequence through public reducer discovery/resolution. | `FRESH-OBJECT-STALE-PUBLIC-HISTORY-REPLAY` | `fresh-replay` | Copied `battle-runtime-starry-wisp-object.route.mbt.qnt`; accepted object-target and mixed-target substrate evidence; current fresh target public reducer surface. | Fresh target only. Keep separate from condition replay: object-boundary/stale route observations, object-target evidence, focused verifier. Likely touches `src/lib.rs`, so do not run concurrently with another fresh runtime lane unless the orchestrator confirms non-overlap. |
| `FC-03-attack-spell-shape-selected-identity-grouped-driver`; `FC-03-level1-damage-spell-selected-identity-grouped-driver`; `FEXP-02-attack-spell-shape-selected-identity-effects-outside-generic-substrate`; `FEXP-02-level1-damage-spell-selected-identity-effects-outside-generic-substrate`; `SQNT-07A-selected-grouped-residual-blockers-carried-forward`; `FRESH-RR-SPATIAL-DAMAGE-selected-identity-drivers-not-accepted-wholesale` | Selected spell residual branch map: selected/grouped witnesses split into reusable shape families such as active-effect rider, condition lifecycle, mixed-target outcome, object/light, movement presentation, reaction, exact damage, and scalar/marked projection | Source-QNT/guidance. This is not one implementation lane; it is a branch-level admission map to prevent duplicate selected-identity work. | `SOURCE-SELECTED-SPELL-RESIDUAL-BRANCH-MAP` | `source-qnt` | Current `task3ResidualSelectedSpellRouteTasks[]`; accepted SQNT-03A through SQNT-03H-style source facts where present; accepted current fresh claims for battle active effects, selected spatial, weapon-hosted, jump, concentration hazard, and exact damage consumption. | Source package/guidance only. This lane must not run in parallel with narrower source lanes that edit the same residual selected-spell inventory rows. It should produce a map, not runtime acceptance. |
| `FRESH-RR-SPATIAL-DAMAGE-movable-multi-emitter-light-route-not-covered`; `FRESH-RR-SPATIAL-DAMAGE-presentation-and-invisible-revealing-remain-table-witnesses` | Spatial/object boundary classification: battle-owned light/sight/obscurement/hazard facts versus table-owned geometry, presentation, durability, and witness facts | Mostly current-package evidence already exists; any remaining work is guidance/classification, not selected-identity replay. | `SOURCE-SPATIAL-OBJECT-BOUNDARY-CLASSIFICATION` | `exclude/optional` | Accepted `FEXP-08-SQNT07A-SELECTED-SPATIAL-CURRENT-PACKAGE-REPLAY`; accepted `FRESH-RR-SQNT07A-JUMP-LANDING-PRONE-REPLAY`; accepted `FRESH-RR-SQNT07A-CONCENTRATION-AREA-HAZARD-REPLAY`; copied spatial-effect and movement-presentation route facts. | Do not launch as a default implementation lane. If launched, limit it to source guidance clarifying reducer-owned versus table-boundary facts. It should not duplicate FEXP-08 fresh evidence. |

## Blockers That Should Not Spawn Work

| Blocker ids or records | Decision | Reason |
| --- | --- | --- |
| `FRESH-RR-SPATIAL-DAMAGE-exact-damage-component-facts-not-public-route-records`; `SQNT-07A-level1-after-hit-timed-exact-damage-details-not-publicly-routed`; `SQNT-07A-level1-weapon-hosted-exact-damage-arithmetic-not-publicly-routed`; `SQNT-07A-hex-ability-check-disadvantage-not-covered-by-marked-damage-facts` | Do not spawn. | Superseded by `FRESH-RR-SQNT07A-HEX-AND-EXACT-DAMAGE-SOURCE-CONSUMPTION`, accepted in the current fresh target with source package `4d196258...`. |
| `SQNT-07A-level1-weapon-hosted-selected-rows-not-reduced-to-generic-facts` | Do not spawn for route-event exactness. | Resolved by `FRESH-RR-SQNT07A-LEVEL1-WEAPON-HOSTED-SELECTED-REPLAY`; exact arithmetic is covered by Hex/exact-damage source consumption. |
| `SQNT-07A-level1-weapon-hosted-authored-identity-details-outside-production-runtime-dispatch` | Do not spawn. | This is not a production runtime blocker. Authored item/attack/spell identity belongs only at catalog, selection, fixture, adapter, or evidence boundaries. |
| `FRESH-RR-SQNT07A-jump-landing-legality-and-failed-landing-prone-blocked` | Do not spawn. | Superseded by `FRESH-RR-SQNT07A-JUMP-LANDING-PRONE-REPLAY`. |
| `FRESH-RR-SQNT07A-concentration-backed-area-hazards-blocked` and retained concentration-hazard exactness rows | Do not spawn. | Superseded by `FRESH-RR-SQNT07A-CONCENTRATION-AREA-HAZARD-REPLAY` and `FRESH-RR-SQNT07A-HEX-AND-EXACT-DAMAGE-SOURCE-CONSUMPTION`. |
| SQNT-03A/B/C/D active-effect route blockers for Hit Point regain prevention, next Attack Roll mode, Opportunity Attack denial, and condition/poison riders | Do not spawn. | Current fresh target accepts `FRESH-RR-BATTLE-ACTIVE-EFFECTS-CURRENT-PACKAGE-REPLAY` with no retained blockers. |
| FCSF-06 character/sheet/handoff dirty target blockers | Do not spawn. | Current fresh target accepts `FRESH-RR-CHARACTER-SHEET-HANDOFF-CURRENT-PACKAGE-REPLAY` with no blocked rows. Historical dirty blockers are superseded. |
| `FEXP-07-exact-sorcerer-metamagic-driver-absent`; `FEXP-07-residual-species-passive-substrates-need-generic-runtime-facts`; `FEXP-07-residual-metamagic-projections-need-typed-spell-modification-facts`; `FEXP-07-feature-selected-identity-residual-branches-not-accepted` | Do not spawn from these stale row names. | Current fresh target accepts FEXP-09A through FEXP-09G current-package evidence for active-feature, species/passive, and metamagic families. Any future metamagic work should be selected from current out-of-scope option groups, not these historical blocker labels. |
| `FRESH-RR-SQNT07A-spatial-movement-superseded-by-FEXP-08-current-package` | Do not spawn. | Explicitly superseded by `FEXP-08-SQNT07A-SELECTED-SPATIAL-CURRENT-PACKAGE-REPLAY`. |
| `dirty-harness-stale-validator-hashes` | Do not spawn as runtime work. | Classified as dirty target limitation with `freshRunBlocker: false`. This is optional dirty cleanup, not a blocker to current fresh acceptance. |
| Historical FC/FEXP evidence rows not renewed against the current package | Do not spawn from this lane. | Foundation renewal or exclusion belongs to `COMP-AUDIT-02-HISTORICAL-FOUNDATION-RENEWAL-DECISION`, not this blocker map. |

## Proposed First Implementation Batch

After the three completion audit reports are merged, the maximum safe first batch from this lane is **2 parallel lanes**:

| Batch slot | Lane | Why it is safe in parallel |
| --- | --- | --- |
| 1 | `SOURCE-REACTION-PROJECTION-FACTS` | Source-QNT/guidance scope only. It should edit focused reaction route/guidance/inventory inputs in the source package, not the accepted fresh target runtime. |
| 2 | `FRESH-SQNT07A-CONDITION-SAVING-SELECTED-REPLAY` | Fresh target replay scope only. It consumes already-packaged generic condition/save/repeat-save facts and should write focused fresh evidence, observations, verifier, and any necessary condition route runtime support. |

Do not run `FRESH-OBJECT-STALE-PUBLIC-HISTORY-REPLAY` concurrently with `FRESH-SQNT07A-CONDITION-SAVING-SELECTED-REPLAY` unless the orchestrator first confirms the fresh target write scopes are separated. The fresh target currently has a monolithic `src/lib.rs`, and both lanes may also need aggregate verifier/state changes. A safer sequence is:

1. Run `SOURCE-REACTION-PROJECTION-FACTS` and `FRESH-SQNT07A-CONDITION-SAVING-SELECTED-REPLAY` in parallel.
2. Merge and serialize aggregate fresh target updates.
3. Launch `FRESH-OBJECT-STALE-PUBLIC-HISTORY-REPLAY`.
4. Launch `SOURCE-SELECTED-SPELL-RESIDUAL-BRANCH-MAP` only if the orchestrator wants a refreshed residual selected-spell queue after the narrower lanes land; otherwise keep it as planning guidance.

`SOURCE-SPATIAL-OBJECT-BOUNDARY-CLASSIFICATION` should not be in the first implementation batch because current-package FEXP-08, Jump, concentration hazard, and Hex/exact-damage evidence already consume most of that family. Treat it as optional guidance if later audits find table-boundary claims still ambiguous.

## Residual Uncertainty

- `BLOCKERS.json.phaseBlockers` intentionally retains historical blocked labels under old phases even when newer current-package claims supersede them. This report classifies those rows by current package evidence rather than by label alone.
- The object-stale row appears to have source-side FCSF-04 evidence and dirty replay history but no current fresh replay acceptance. That makes it the strongest fresh-replay candidate after condition-saving selected replay.
- The broad selected/grouped residual rows are real guardrails, but they are not executable as a single runtime lane. Launching them as one implementation task would recreate the identity-grouping problem this campaign is trying to avoid.
