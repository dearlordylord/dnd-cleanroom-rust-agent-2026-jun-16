# COMP-AUDIT-01 Current Package Gate Matrix

Lane: `COMP-AUDIT-01-CURRENT-PACKAGE-GATE-MATRIX`

Audit date: 2026-06-29

## Scope And Base Check

| Item | Result |
| --- | --- |
| Declared lane base | `73e2db4eb5ab420123a6eb8cc7a803f0b5d091d5` |
| Worktree | `/workspace/typescript/.codex-worktrees/dnd-comp-audit-01-current-package-gate` |
| Branch | `ralph/comp-audit-01-current-package-gate` |
| `git rev-parse HEAD` at start | `73e2db4eb5ab420123a6eb8cc7a803f0b5d091d5` |
| Ancestor check | `git merge-base --is-ancestor 73e2db4eb5ab420123a6eb8cc7a803f0b5d091d5 HEAD` exited `0` |
| Accepted fresh target inspected | `/workspace/typescript/.codex-worktrees/dnd-fresh-cleanroom-dry-run-fc00` |
| Fresh target `HEAD` inspected | `ff09b6579ebb507f3c8dca06f2b040d0d46d05fb` |
| Fresh target current verifier | `python3 tools/verify_current_fresh_target.py` |
| Source package commit inspected | `4d196258a51f4264803ff11f7c806c969f0aff2d` |
| Source package manifest SHA-256 | `0f3f8b73a42a3f72cb72b150a156933de9e25f545bfabbff1307da32f021b417` |
| Current package inventory | `544` copied input files; `100` drivers; `667` in-scope obligations; `61` out-of-scope obligations |

Cleanroom boundary used for this audit: this report was prepared from this lane worktree, the accepted fresh target, their `cleanroom-input/**`, `tasks/**`, repo-local verifiers, and JSON/report state files. No update was made to `STATE.json`, ledger files, or the accepted fresh target.

## Current-Package Runtime Claims

These are the only runtime acceptance claims enforced by `tools/verify_current_fresh_target.py` through `CURRENT_RUNTIME_ACCEPTANCE_CLAIMS`, `FRESH_RUN_STATE.json`, `FRESH_EXPANSION_STATE.json`, and `BLOCKERS.json`. Classification: `fresh target replay`.

| Claim | Evidence artifact | Focused verifier | Accepted evidence enforced |
| --- | --- | --- | --- |
| `FRESH-RR-SQNT07A-JUMP-LANDING-PRONE-REPLAY` | `EVIDENCE/fresh-rr-sqnt07a-jump-landing-prone.json` | `tools/verify_fresh_rr_sqnt07a_jump_landing_prone.py` | `4` route surfaces, `0` blockers; copied movement-presentation route/fact input compared to public reducer observations. |
| `FRESH-RR-REACTION-INTERRUPT-TAXONOMY-CURRENT-PACKAGE-REPLAY` | `EVIDENCE/fresh-rr-reaction-interrupt-taxonomy.json` | `tools/verify_fresh_rr_reaction_interrupt_taxonomy.py` | `5` generic reaction/interrupt taxonomy route surfaces; residual selected reaction projection blocker remains explicit. |
| `FRESH-RR-BATTLE-ACTIVE-EFFECTS-CURRENT-PACKAGE-REPLAY` | `EVIDENCE/fresh-rr-battle-active-effects.json` | `tools/verify_fresh_rr_battle_active_effects.py` | `9` SQNT-03A/B/C/D active-effect route surfaces, `0` blockers; expected records derived from copied route connectors and helper vocabulary. |
| `FEXP-08-SQNT07A-SELECTED-SPATIAL-CURRENT-PACKAGE-REPLAY` | `EVIDENCE/fexp08-sqnt07a-selected-spatial-current-package-replay.json` | `tools/verify_fresh_rr_sqnt07a_spatial_movement.py` | `10` selected-spatial route rows, `0` blockers; copied selected-spatial route connector evidence compared to public reducer observations. |
| `FRESH-RR-SQNT07A-LEVEL1-WEAPON-HOSTED-SELECTED-REPLAY` | `EVIDENCE/fresh-rr-sqnt07a-level1-weapon-hosted-selected.json` | `tools/verify_fresh_rr_sqnt07a_level1_weapon_hosted_selected.py` | `3` selected rows, `3` generic route-surface groups, and `2` capability surfaces; retained identity-only details are outside production runtime dispatch, and exact arithmetic is covered by the Hex/exact-damage claim. |
| `FRESH-RR-SQNT07A-CONCENTRATION-AREA-HAZARD-REPLAY` | `EVIDENCE/fresh-rr-sqnt07a-concentration-area-hazard.json` | `tools/verify_fresh_rr_sqnt07a_concentration_area_hazard.py` | `2` route surfaces plus `2` exact damage scenarios; `0` selected exactness blockers. |
| `FEXP-09A-SQNT07D-ACTIVE-FEATURE-CURRENT-PACKAGE-REPLAY` | `EVIDENCE/fexp09a-sqnt07d-active-feature-current-package-replay.json` | `tools/verify_fexp09a_sqnt07d_active_feature.py` | `3` active-feature route rows, `0` blockers; selected identity is confined to copied QNT/evidence boundaries. |
| `FEXP-09B-SQNT07B-SPECIES-PASSIVE-CURRENT-PACKAGE-REPLAY` | `EVIDENCE/fexp09b-sqnt07b-species-passive-current-package-replay.json` | `tools/verify_fexp09b_sqnt07b_species_passive.py` | `15` species/passive-adjacent rows, `0` blockers, `2` inherited Adrenaline Rush rows out of scope. |
| `FEXP-09C-SQNT07C-QUICKENED-GOVERNOR-CURRENT-PACKAGE-REPLAY` | `EVIDENCE/fexp09c-sqnt07c-quickened-governor-current-package-replay.json` | `tools/verify_fexp09c_sqnt07c_quickened_governor.py` | `12` accepted route surfaces: `11` Quickened governor obligations plus `1` selected Quickened save-damage row; `11` other SQNT-07C option groups out of scope. |
| `FEXP-09D-SQNT07C-METAMAGIC-SAVE-MODES` | `EVIDENCE/fexp09d-sqnt07c-metamagic-save-modes.json` | `tools/verify_fexp09d_sqnt07c_metamagic_save_modes.py` | `9` accepted rows: `7` selected save-mode rows plus `2` route rows; `0` blockers. |
| `FEXP-09E-SQNT07C-METAMAGIC-SPELL-SHAPE` | `EVIDENCE/fexp09e-sqnt07c-metamagic-spell-shape.json` | `tools/verify_fexp09e_sqnt07c_metamagic_spell_shape.py` | `7` accepted rows: `4` selected projection rows plus `3` route projection rows; `0` blockers. |
| `FEXP-09F-SQNT07C-METAMAGIC-REROLLS` | `EVIDENCE/fexp09f-sqnt07c-metamagic-rerolls.json` | `tools/verify_fexp09f_sqnt07c_metamagic_rerolls.py` | `4` accepted rows: `2` selected projection rows plus `2` route projection rows; `0` blockers. |
| `FEXP-09G-SQNT07C-METAMAGIC-COMPONENT-DURATION` | `EVIDENCE/fexp09g-sqnt07c-metamagic-component-duration.json` | `tools/verify_fexp09g_sqnt07c_metamagic_component_duration.py` | `5` accepted rows: `3` selected projection rows plus `2` route projection rows; `0` blockers. |
| `FRESH-RR-CHARACTER-SHEET-HANDOFF-CURRENT-PACKAGE-REPLAY` | `EVIDENCE/fexp06-character-creation-sheet-handoff-pack.json` | `tools/verify_fexp06.py` | `26` character creation, sheet resource/rest, handoff settlement, and zero-HP lifecycle route scenarios; `0` blockers. |
| `FRESH-RR-SQNT07A-HEX-AND-EXACT-DAMAGE-SOURCE-CONSUMPTION` | `EVIDENCE/fresh-rr-sqnt07a-hex-exact-damage-source-consumption.json` | `tools/verify_fresh_rr_sqnt07a_hex_exact_damage_source_consumption.py` | `3` accepted claim families, `6` exact damage scenarios, `4` concentration-hazard rows, `0` retained blockers; copied QNT import closure and definition hashes are pinned. |

## Source-QNT And Guidance Inputs

These rows are accepted input or source-side evidence for target work. They are not, by themselves, fresh current-package runtime completion proof unless they also appear in the 15-row table above.

| Input row | Classification | Current audit result |
| --- | --- | --- |
| Current package admission and inventory from `cleanroom-input/MANIFEST.md`, `fc00_inventory`, and `FRESH_RUN_STATE.json` | source-QNT/guidance input | Enforced by `tools/verify_current_fresh_target.py`: source package `4d196258a51f4264803ff11f7c806c969f0aff2d`, file count `544`, driver count `100`, in-scope obligations `667`, out-of-scope obligations `61`. |
| Consumed source wave: Hex ability-check roll-mode, exact-damage route bridge, concentration-backed hazard selected route witness | source-QNT/guidance input | `SOURCE_QNT_NEXT_TASKS.md` records the wave as consumed by fresh target `ff09b6579ebb507f3c8dca06f2b040d0d46d05fb`; do not relaunch it. |
| SQNT-03A/B/C/D active-effect connectors | source-QNT/guidance input plus current fresh target replay | Source connectors are accepted and dirty-replayed historically; current runtime proof is the `FRESH-RR-BATTLE-ACTIVE-EFFECTS-CURRENT-PACKAGE-REPLAY` claim. |
| SQNT-07B/C/D route facts | source-QNT/guidance input plus current fresh target replay | Source package facts feed the FEXP-09A through FEXP-09G current runtime claims. |
| SQNT-03E/F/G object/light, mixed-target, and exact-damage source inputs | source-QNT/guidance input | They appear as source/dirty rehearsal evidence and partial blockers; current fresh runtime proof exists only where consumed by `FRESH-RR-SPATIAL-DAMAGE-CURRENT-PACKAGE-REPLAY` and `FRESH-RR-SQNT07A-HEX-AND-EXACT-DAMAGE-SOURCE-CONSUMPTION`. |
| Dirty harness stale validator hashes | optional dirty cleanup | `BLOCKERS.json.limitations[]` classifies this as `dirty-target-limitation`, `freshRunBlocker: false`; not a current fresh runtime blocker. |

## Historical-Only Or Superseded Claims

The aggregate verifier requires these phases to be recorded as `historical-snapshot-evidence` with `currentPackageHashRecomputed: false`. They should not be counted as current completion proof without renewal.

| Row | Evidence artifact | Classification | Current status |
| --- | --- | --- | --- |
| `FC-01` | `EVIDENCE/fc01-reducer-substrate.json` | historical-only | Historical snapshot classification enforced by the current verifier. |
| `FC-02` | `EVIDENCE/fc02-weapon-attack-route.json` | historical-only | Historical snapshot classification enforced by the current verifier. |
| `FC-03` | `EVIDENCE/fc03-save-gated-spell-ordering-route.json` | historical-only | Historical snapshot classification enforced; grouped selected drivers remain blocked separately. |
| `FC-04` | `EVIDENCE/fc04-base-armor-class-effect-route.json` | historical-only | Historical snapshot classification enforced. |
| `FC-05` | `EVIDENCE/fc05-character-battle-init-projection-route.json` | historical-only | Historical snapshot classification enforced. |
| `FC-07` | `EVIDENCE/fc07-pact-slot-handoff-route.json` | historical-only | Historical snapshot classification enforced. |
| `SDK-tracer-bullet` | `EVIDENCE/sdk-tracer-bullet-programmatic-surface.json` | historical-only | Historical snapshot classification enforced. |
| `FEXP-01` | `EVIDENCE/fexp01-diagnostic-battle-route-pack.json` | historical-only | Historical snapshot classification enforced. |
| `FEXP-02` | `EVIDENCE/fexp02-spell-attack-save-gated-unblock.json` | historical-only | Historical snapshot classification enforced; residual selected spell effects remain blockers. |
| `FEXP-03` | `EVIDENCE/fexp03-chained-and-object-spell-attacks.json` | historical-only | Historical snapshot classification enforced; object stale route history remains blocked unless renewed. |
| `FEXP-04` | `EVIDENCE/fexp04-active-effect-lifecycle-and-roll-modifiers.json` | historical-only | Historical snapshot classification enforced, despite prior accepted fresh evidence. |
| `FEXP-05` | `EVIDENCE/fexp05-reaction-interrupt-and-boundary.json` | historical-only | Historical snapshot classification enforced; selected reaction projections remain blocked. |
| `FEXP-07` | `EVIDENCE/fexp07-feature-species-metamagic-substrates.json` | historical-only | Historical snapshot classification enforced; FEXP-09 rows are the current-package replacements. |
| `FRESH-RR-FEATURE-SPECIES-METAMAGIC` | `EVIDENCE/fexp07-feature-species-metamagic-substrates.json` | historical-only | Historical snapshot classification enforced; current proof is split into FEXP-09A through FEXP-09G. |
| `FRESH-RR-SQNT07A-SPATIAL-MOVEMENT` | `EVIDENCE/fresh-rr-sqnt07a-spatial-movement.json` | superseded/historical-only | Superseded by `FEXP-08-SQNT07A-SELECTED-SPATIAL-CURRENT-PACKAGE-REPLAY`. |
| `FRESH-RR-SQNT07A-ACTIVE-EFFECT-CONDITION` | `EVIDENCE/fresh-rr-sqnt07a-active-effect-condition.json` | source-QNT/guidance input plus prior replay | Present in expansion coverage rows but not one of the current aggregate verifier's 15 runtime claim ids. Count only through specific current claims that consume its generic facts. |
| `FRESH-RR-SPATIAL-DAMAGE-CURRENT-PACKAGE-REPLAY` | `EVIDENCE/fresh-rr-spatial-damage.json` | fresh target replay support, not aggregate 15-claim row | Present in expansion coverage rows and blocker clusters; current aggregate proof counts only the 15 verifier claims. |
| Level-1 marked/immunity, scalar-buff, and after-hit/timed selected replays | `EVIDENCE/fresh-rr-sqnt07a-level1-*.json` | fresh target replay support, not aggregate 15-claim rows | These are useful predecessor rows. Their unresolved pieces are either superseded by later accepted claims or remain in blocker clusters. |

## Retained Blocker Clusters

This table groups `BLOCKERS.json.phaseBlockers` by live cluster. `blocked` rows are not current completion proof. `superseded`, `resolved`, and `outside-production-runtime-dispatch` rows should not launch implementation by themselves.

| Blocker cluster | BLOCKERS ids / phases | Status in `BLOCKERS.json` | Classification | Audit action |
| --- | --- | --- | --- | --- |
| FC-03 grouped selected spell drivers | `FC-03-attack-spell-shape-selected-identity-grouped-driver`; `FC-03-level1-damage-spell-selected-identity-grouped-driver` | `blocked` | source-QNT/guidance input | Do not launch target replay wholesale; split into generic route-shape source tasks first. |
| FEXP-02 residual selected spell effects | `FEXP-02-attack-spell-shape-selected-identity-effects-outside-generic-substrate`; `FEXP-02-level1-damage-spell-selected-identity-effects-outside-generic-substrate` | `blocked` | source-QNT/guidance input | Candidate input for branch-level mapping, not a target implementation lane yet. |
| FEXP-03 object stale public route history | `FEXP-03-object-stale-replay-isolated-route-surface-not-publicly-observable` | `blocked` | target implementation or source-QNT/guidance, depending on whether source elects a public reducer token protocol | Needs COMP-AUDIT-03 branch decision before implementation. |
| FEXP-05 historical reaction/interrupt gaps | `FEXP-05-selected-reaction-spell-projections-need-generic-source-shapes`; `FEXP-05-interrupt-trigger-taxonomy-not-admitted-by-route-inputs` | `blocked` | source-QNT/guidance input | Historical FEXP-05 evidence is not enough; current taxonomy claim covers only generic taxonomy rows. |
| FEXP-07 manifest/residual feature/species/metamagic gaps | `FEXP-07-exact-sorcerer-metamagic-driver-absent`; `FEXP-07-selected-and-grouped-identity-drivers-not-accepted`; residual species, metamagic, and active-feature blockers | `blocked` | source-QNT/guidance input | Do not relaunch old FEXP-07. Use FEXP-09 current rows as accepted subsets and map residual branches only. |
| Current reaction taxonomy residual selected projections | `FRESH-RR-REACTION-INTERRUPT-TAXONOMY-selected-reaction-spell-projections-remain-outside-taxonomy` | `blocked` | source-QNT/guidance input | Needs generic route shapes for exact AC magnitude, reaction damage/save math, slot preservation, and selected spell projection without authored identity dispatch. |
| Spatial/damage current package residuals | `FRESH-RR-SPATIAL-DAMAGE-movable-multi-emitter-light-route-not-covered`; `presentation-and-invisible-revealing-remain-table-witnesses`; `selected-identity-drivers-not-accepted-wholesale`; stale object blocker | `blocked` | mixed source-QNT/guidance and target implementation | Decide table-boundary versus reducer-owned facts before target implementation. |
| Spatial/damage exact damage component blocker | `FRESH-RR-SPATIAL-DAMAGE-exact-damage-component-facts-not-public-route-records` | `superseded` by `FRESH-RR-SQNT07A-HEX-AND-EXACT-DAMAGE-SOURCE-CONSUMPTION` | superseded fresh target replay | Do not launch. Keep as evidence that the later Hex/exact-damage claim consumed the gap. |
| SQNT-07A selected condition and marked residuals | `SQNT-07A-condition-selected-identity-adapter-not-refreshed`; `SQNT-07A-selected-grouped-marked-immunity-residual-rows-not-generic-facts` | `blocked` | source-QNT/guidance input, then possible fresh target replay | Candidate for a narrow selected condition-saving replay only after the generic mapping is explicit. |
| SQNT-07A weapon-hosted selected exactness | `SQNT-07A-level1-weapon-hosted-selected-rows-not-reduced-to-generic-facts` | mixed historical `blocked` plus current `resolved` | superseded/resolved fresh target replay | Do not launch from older blocked rows; current weapon-hosted claim resolves selected route-event exactness. |
| SQNT-07A authored weapon/item/spell identity details | `SQNT-07A-level1-weapon-hosted-authored-identity-details-outside-production-runtime-dispatch` | `outside-production-runtime-dispatch`, `runtimeAcceptanceBlocker: false` | historical/outside production runtime | Not implementation work for production runtime; identity remains at catalog/evidence/fixture/user-selection boundaries. |
| Jump landing/prone and concentration hazard predecessor blockers | `FRESH-RR-SQNT07A-jump-landing-legality-and-failed-landing-prone-blocked`; `FRESH-RR-SQNT07A-concentration-backed-area-hazards-blocked` | `superseded` | superseded fresh target replay | Do not launch. Current Jump and concentration-hazard claims are accepted. |
| Hex ability-check and exact damage predecessor blockers | `SQNT-07A-hex-ability-check-disadvantage-not-covered-by-marked-damage-facts`; after-hit/weapon-hosted exact damage blockers | `superseded` | superseded fresh target replay | Do not launch. Current Hex/exact-damage claim is accepted. |
| Dirty harness stale validator hashes | `dirty-harness-stale-validator-hashes` in `limitations[]` | `freshRunBlocker: false` | optional dirty cleanup | Only clean up if the orchestrator wants dirty harness hygiene; not a current package gate blocker. |

## Recommendation

Do not launch broad selected/grouped identity target implementation lanes from this audit alone. The aggregate current package gate proves the 15 rows above and explicitly classifies older FC/FEXP/SDK evidence as historical snapshots.

Implementation lanes that should not launch next:

- broad `FEXP-07` or SQNT-07A selected/grouped identity replay;
- target replay for stale historical FC/FEXP rows without a renewal decision;
- target work for authored item, attack, spell, slug, source heading, page reference, or selected-row identity dispatch;
- dirty harness validator cleanup as proof work.

Implementation lanes that may be selected after the three completion audits converge:

- branch-level source-QNT/guidance mapping for residual selected spell effects;
- source or target clarification for spatial/object boundary facts, especially movable multi-emitter light, table-owned presentation/invisible-revealing facts, and stale object public route history;
- narrow selected condition-saving replay only if it reduces to already accepted or newly explicit generic condition lifecycle facts;
- reaction projection facts for exact AC, post-damage save/damage, and interrupted slot preservation, independent of selected spell identity.

The current evidence is enough to say the accepted fresh package gate is mechanically checked at `ff09b6579ebb507f3c8dca06f2b040d0d46d05fb`. It is not enough to claim the global cleanroom campaign goal complete.
