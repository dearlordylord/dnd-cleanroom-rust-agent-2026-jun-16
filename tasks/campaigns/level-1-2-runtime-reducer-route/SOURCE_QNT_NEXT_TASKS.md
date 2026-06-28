# Source-QNT Next Task Queue

Campaign: `level-1-2-runtime-reducer-route`

Purpose: record the next source-side QNT/guidance tasks after the dirty
cleanroom replay queue was exhausted. These are not dirty target implementation
lanes. A target worker must not infer these facts from TypeScript or dirty Rust
history.

## Current Boundary

Dirty replay status:

- FCSF-04 object stale public protocol: accepted in dirty replay; 6 connector
  rows, 7 Starry Wisp object obligations, 0 blockers.
- FCSF-05 reaction/interrupt payload taxonomy: accepted in dirty replay; 5
  connector rows, 3 selected reaction-spell context obligations, 0 blockers.
- FCSF-06 character/sheet/handoff payloads: accepted-with-target-blockers in
  dirty replay; 30 accepted rows, 14 target blockers.
- SQNT-03A Hit Point regain prevention: accepted in dirty replay at campaign
  merge `73627315f70528e73f5eb4ef781606e876e87367`; 3 connector transitions,
  1 attack-shape obligation, 0 SQNT-03A target behavior blockers.
- SQNT-03B Next Attack Roll mode: accepted in dirty replay at campaign merge
  `4437eacc311a8ea069bc1d7c9dd9d2b334a8fb4e`; 6 connector transitions, 2
  selected-driver obligations, 0 SQNT-03B target behavior blockers.
- SQNT-03C Opportunity Attack Denial: accepted in dirty replay at campaign
  merge `7521115f61077326b67e933dc9663f19d7e41570`; 4 connector transitions,
  1 selected-driver obligation, 0 SQNT-03C target behavior blockers.
- SQNT-03D Condition And Poison Riders: source connector accepted and packaged
  at source commit `3e9297cab2dd998e5dc09670f771bfcdf0a04c93`; dirty target
  replay is accepted at campaign merge
  `4d4107b51c595b8cbbaa6c0c11f458b35d2070c1`; 16 connector rows, 3
  selected-driver obligations, with Hold Person and Hideous Laughter lifecycle
  shapes still explicit source-QNT blockers rather than inferred
  selected-identity behavior.

Source-QNT status:

- SQNT-03A Hit Point regain prevention: source connector accepted and packaged
  at source commit `c83c4a2321ff45c796245d65ba979b9068c6718a`; dirty target
  replay is accepted. Fresh target replay remains pending.
- SQNT-03B Next Attack Roll mode: source connector accepted and packaged at
  source commit `d00c92a3d12531e50d95ead220303b66a5265e1e`; dirty target replay
  is accepted. Fresh target replay remains pending.
- SQNT-03C Opportunity Attack Denial: source connector accepted and packaged at
  source commit `ebc37e935fdd45ac07198bbec6b3bcc23be2270e`; dirty target replay
  is accepted. Fresh target replay remains pending.
- SQNT-03D Condition And Poison Riders: source connector accepted and packaged
  at source commit `3e9297cab2dd998e5dc09670f771bfcdf0a04c93`; dirty target
  replay is accepted. Fresh target replay remains pending.
- SQNT-03E Object And Light Riders: source connector accepted and packaged at
  source commit `32e51f46a71f5a714034966e018fe79abbb7fcae`; dirty target
  replay is accepted at campaign merge
  `9b4ece63f3bad64209afa1ba4c1d44efdc1bbe67`. Fresh target replay remains
  pending.
- SQNT-03F Mixed Target Outcomes: source connector accepted and packaged at
  source commit `2de6a5e5401985e5673f5c2be737bbe2374d46d0`; dirty target
  replay is accepted at campaign merge
  `fde412cbe2cce580b0dc2a44270e63ba601b17ef`. Fresh target replay remains
  pending.
- SQNT-03G Exact Damage Projection Facts: source component witness accepted and
  packaged at source commit `21504ef764118f5fd13086aa6266f19280196664`;
  dirty target replay is accepted at campaign merge
  `f94d2fb6def9535ee7e2f7d420879012e573b420`. Fresh target replay remains
  pending.

Remaining target work should not start until the source package exposes the
generic route facts below. The target acceptance gate stays the same:

- expected records come from copied `.qnt` connector action bodies or copied
  connector vocabulary;
- observed records come from public reducer, character, sheet, or handoff
  entrypoints;
- durable target fields need state-owner derivability records;
- production code must not dispatch on authored identity.

## Source Task Queue

| Task | Source input to add | Blocks | Acceptance shape |
| --- | --- | --- | --- |
| SQNT-03A Hit Point Regain Prevention | Done: `cleanroom-input/qnt/battle-runtime/battle-runtime-hit-point-regain-prevention.route.mbt.qnt` now exposes generic active-effect admission, later healing interdiction, turn-boundary duration expiry, and active-effect cleanup. | `FCSF-03A-HIT-POINT-REGAIN-PREVENTION` | Source input is packaged and dirty target replay is accepted through observed reducer route events without selected spell identity. Fresh target replay remains pending. |
| SQNT-03B Next Attack Roll Mode | Done: `cleanroom-input/qnt/battle-runtime/battle-runtime-next-attack-roll-mode.route.mbt.qnt` and `battle-runtime-next-attack-roll-mode-route-facts.qnt` now expose generic next-Attack-Roll roll-mode route evidence with source, carrier, attacker, target-scope, expiration-boundary, and Advantage/Disadvantage facts. | `FCSF-03B-NEXT-ATTACK-ROLL-MODE` | Source input is packaged and dirty target replay is accepted for 6 connector transitions and 2 selected-driver obligations through shared roll-mode projection, consumption, expiration, and active-effect cleanup owners. Fresh target replay remains pending. |
| SQNT-03C Opportunity Attack Denial | Done: `cleanroom-input/qnt/battle-runtime/battle-runtime-opportunity-attack-denial.route.mbt.qnt` and `battle-runtime-opportunity-attack-denial-route-facts.qnt` now expose generic reaction-interdiction route evidence with attack-hit source, affected target carrier/scope, denied Opportunity Attack trigger family, duration boundary, admission, movement/reaction projection, expiration, and cleanup facts. | `FCSF-03C-OPPORTUNITY-ATTACK-DENIAL` | Source input is packaged and dirty target replay is accepted for 4 connector transitions and 1 selected-driver obligation through shared active-effect, movement-resource, interrupt-stack, and turn-boundary owners. Fresh target replay remains pending. |
| SQNT-03D Condition And Poison Riders | Done: `cleanroom-input/qnt/battle-runtime/battle-runtime-condition-riders.route.mbt.qnt` and `battle-runtime-condition-rider-route-facts.qnt` now expose generic condition-rider route evidence for attack-hit Poisoned, failed-save Blinded duration, failed-save Blinded/Deafened repeat-save cleanup, failed-save Restrained Athletics escape cleanup, Sleep-style Incapacitated-to-Unconscious transition, condition-immunity rejection, and cleanup owner facts. | `FCSF-03D-CONDITION-AND-POISON-RIDERS` | Source input is packaged and dirty target replay is accepted for 16 connector rows and 3 selected-driver obligations. Hold Person and Hideous Laughter remain explicit source-QNT blockers. Fresh target replay remains pending. |
| SQNT-03E Object And Light Riders | Done: `cleanroom-input/qnt/battle-runtime/battle-runtime-object-light-riders.route.mbt.qnt` and `battle-runtime-object-light-rider-route-facts.qnt` now expose generic ObjectLightRiderRouteSubject evidence for object admission/rejection, object-attached and held emitter facts, replacement cleanup, hurl cleanup, duration cleanup, Bright/Dim Light projection through BattleLightProjectionOwner, and table-owned object/geometry/cover/opaque-blocker/presentation/durability witness facts. | `FCSF-03E-OBJECT-AND-LIGHT-RIDERS` | Source input is packaged and dirty target replay is accepted for 9 copied connector actions and 2 selected spatial obligations. Dancing Lights, outline/invisible-revealing/attack-advantage, area hazard/obscurement, movement replacement, and forced-movement/object-push presentation remain explicit blockers. Fresh target replay remains pending. |
| SQNT-03F Mixed Target Outcomes | Done: `cleanroom-input/qnt/battle-runtime/battle-runtime-mixed-target-outcomes.route.mbt.qnt` and `battle-runtime-mixed-target-outcome-route-facts.qnt` now expose generic mixed-outcome route evidence with explicit target attack/Saving Throw outcomes, shared spend facts, separate shared Saving Throw damage-roll facts, per-target damage projection, and secondary projections through one invocation owner. | `FCSF-03F-MIXED-TARGET-OUTCOMES` | Source input is packaged and dirty target replay is accepted for all 7 connector rows and 3 selected level-1 damage obligations. Exact damage/dice/type facts, active-effect riders, object/light lifecycle, next-Attack-Roll lifecycle, and selected identity wholesale acceptance remain explicit blockers. Fresh target replay remains pending. |
| SQNT-03G Exact Damage Projection Facts | Done: `cleanroom-input/qnt/battle-runtime/rule-core-exact-damage-projection.mbt.qnt` now exposes exact damage amount, type, instance count, dice count, critical doubling, success policy, target Hit Point facts, and both spell procedure profile and Hit Point damage owner evidence as component witness facts. | `FCSF-03G-EXACT-DAMAGE-PROJECTION-FACTS` | Source input is packaged and dirty target replay is accepted for all 4 component rows. Selected attack-spell and level-1 damage drivers remain unaccepted wholesale; non-damage selected identity, mixed-target ordering, active-effect riders, duplicate-face limits, object/light lifecycle, and catalog admission remain outside this lane. Fresh target replay remains pending. |
| SQNT-07A Selected/Grouped Identity Substrate | Remaining source-side work is not one broad identity task. Split by the residual selected-spell substrate queue in `task3ResidualSelectedSpellRouteTasks[]` and the explicit blockers named above: Paralyzed/Incapacitated lifecycle, laughter lifecycle, marked-target damage rider/transfer, condition-immunity plus turn-start Temporary Hit Points, mixed spell-attack plus burst Saving Throw, movable multi-emitter light, outline/invisible-revealing/attack-advantage, area hazard/obscurement, movement replacement, forced-movement/object-push presentation, and any exact-damage branch still lacking current copied facts. | `FEXP-07-selected-and-grouped-identity-drivers-not-accepted` plus residual selected-spell source blockers | Source-QNT lanes should add one generic substrate family at a time. Target replay for a selected/catalog-heavy row may count only through referenced generic substrate facts. |
| SQNT-07B Species Passive Facts | Done source-side at source commit `21504ef764118f5fd13086aa6266f19280196664`: `cleanroom-input/qnt/battle-runtime/battle-runtime-species-passive-trait-substrates.route.mbt.qnt` exposes typed passive damage adjustment, saving-throw/ability-check roll mode, creature-space movement, and creature-stat projection facts. | `FEXP-07-residual-species-passive-substrates-need-generic-runtime-facts` | Source input is packaged and current-package dirty replay is accepted for 15 selected species/passive-adjacent rows through public reducer route events. Two inherited Adrenaline Rush rows are explicit out-of-scope rows for this lane. Fresh target replay remains pending. |
| SQNT-07C Metamagic Spell Modification Facts | Done source-side at source commit `21504ef764118f5fd13086aa6266f19280196664`: `cleanroom-input/qnt/battle-runtime/battle-runtime-sorcerer-metamagic.route.mbt.qnt` exposes typed option facts, spell-modification payloads, Sorcery Point/resource owners, and spell procedure route subjects. | `FEXP-07-residual-metamagic-projections-need-typed-spell-modification-facts` | Source input is packaged and current-package dirty replay is accepted for 30 selected metamagic rows through metamagic/resource/procedure facts, not selected option identity. Fresh target replay remains pending. |
| SQNT-07D Active Feature Spell Benefit Facts | Done source-side at source commit `21504ef764118f5fd13086aa6266f19280196664`: `cleanroom-input/qnt/battle-runtime/battle-runtime-feature-selected-identity.route.mbt.qnt` exposes active-feature Spell Save DC and spell Attack Roll mode benefit route facts. | `FEXP-07-feature-selected-identity-residual-branches-not-accepted` | Source input is packaged and current-package dirty replay is accepted for all 3 rows through observed reducer entrypoints and source-derived active-feature facts. Fresh target replay remains pending. |

## Current Replay Queue

Audit result at SQNT-07 replay merge head
`4b2c415259ad5f3b10d281a536a5aa8499f926b7`, with current dirty campaign
control head `ebd3699fb03c99d0ec674361ebf69835e64bfd0c`:

- SQNT-07B, SQNT-07C, and SQNT-07D dirty current-package replay lanes are
  merged and verified against copied package source
  `21504ef764118f5fd13086aa6266f19280196664`.
- Keep SQNT-07A as source-input work. Do not launch a broad selected/grouped
  dirty replay lane until the specific generic substrate family exists in the
  copied package.
- Treat older RR07 dirty evidence outside the merged SQNT-07B/C/D files as
  historical snapshots unless its evidence file is refreshed to the current
  copied package source SHA.

## Ralph Operating Rule

Each source-QNT task should be its own Ralph lane or a small parallel group with
non-overlapping files. Before launching target replay for any row above, refresh
the cleanroom input package and verify the new connector appears in
`cleanroom-input/branch-coverage/reducer-route-inventory.json`.
