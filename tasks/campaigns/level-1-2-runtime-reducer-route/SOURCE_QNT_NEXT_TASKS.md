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

Source-QNT status:

- SQNT-03A Hit Point regain prevention: source connector accepted and packaged
  at source commit `c83c4a2321ff45c796245d65ba979b9068c6718a`; dirty target
  replay is accepted. Fresh target replay remains pending.

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
| SQNT-03B Next Attack Roll Mode | Done: `cleanroom-input/qnt/battle-runtime/battle-runtime-next-attack-roll-mode.route.mbt.qnt` and `battle-runtime-next-attack-roll-mode-route-facts.qnt` now expose generic next-Attack-Roll roll-mode route evidence with source, carrier, attacker, target-scope, expiration-boundary, and Advantage/Disadvantage facts. | `FCSF-03B-NEXT-ATTACK-ROLL-MODE` | Source input is packaged. Dirty/fresh target replay remains pending; target may count only through shared roll-mode projection, consumption, expiration, and active-effect cleanup owners. |
| SQNT-03C Opportunity Attack Denial | Generic reaction-interdiction route subject for attack-hit source, affected target, denied Opportunity Attack trigger family, and duration boundary. | `FCSF-03C-OPPORTUNITY-ATTACK-DENIAL` | Target may count only after observed route events project denial from active-effect state into movement/reaction discovery. |
| SQNT-03D Condition And Poison Riders | Generic condition-rider route subjects carrying host outcome, condition kind, immunity check, duration/repeat-save boundary, and cleanup owner facts. | `FCSF-03D-CONDITION-AND-POISON-RIDERS` | Target may count only after copied route evidence owns condition application, immunity rejection, and cleanup without selected spell identity. |
| SQNT-03E Object And Light Riders | Generic object/light route subjects for object admission, object-attached or held emitter state, replacement cleanup, hurl cleanup, illumination projection, and table-owned geometry witnesses. | `FCSF-03E-OBJECT-AND-LIGHT-RIDERS` | Target may count only after route evidence separates battle-owned emitter/effect facts from table-owned geometry, color, and presentation facts. |
| SQNT-03F Mixed Target Outcomes | Generic mixed-outcome route subject making each target's attack or Saving Throw outcome explicit while routing shared resource spend, per-target damage, and secondary projections through one invocation owner. | `FCSF-03F-MIXED-TARGET-OUTCOMES` | Target may count only after route evidence prevents adapter-local coupling between target order, outcome order, and damage projection. |
| SQNT-03G Exact Damage Projection Facts | Promote exact damage amount, type, instance count, dice count, critical doubling, success policy, and target damage-application facts into route or component connector evidence. | `FCSF-03G-EXACT-DAMAGE-PROJECTION-FACTS` | Target may count only when exact projections are public enough that a fresh target need not infer them from TypeScript or selected spell identity. |
| SQNT-07A Selected/Grouped Identity Substrate | Focused generic route connectors for selected/grouped identity branches. | `FEXP-07-selected-and-grouped-identity-drivers-not-accepted` | Target may count selected/catalog-heavy rows only through referenced generic substrate facts. |
| SQNT-07B Species Passive Facts | Typed runtime facts for passive damage, saving-throw/ability-check roll mode, and creature-space movement. | `FEXP-07-residual-species-passive-substrates-need-generic-runtime-facts` | Target may count only when passive traits are projected from typed facts rather than identity labels. |
| SQNT-07C Metamagic Spell Modification Facts | Typed spell-modification payloads and spell-source profiles. | `FEXP-07-residual-metamagic-projections-need-typed-spell-modification-facts` | Target may count only through metamagic/resource/procedure facts, not selected option identity. |
| SQNT-07D Active Feature Spell Benefit Facts | Generic active-feature spell attack roll-mode and Spell Save DC benefit route facts. | `FEXP-07-feature-selected-identity-residual-branches-not-accepted` | Target may count only after active-feature benefits are routed through public owners and source-derived facts. |

## Ralph Operating Rule

Each source-QNT task should be its own Ralph lane or a small parallel group with
non-overlapping files. Before launching target replay for any row above, refresh
the cleanroom input package and verify the new connector appears in
`cleanroom-input/branch-coverage/reducer-route-inventory.json`.
