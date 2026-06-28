# Fresh Expansion Blocker Source Feedback

Campaign: `level-1-2-runtime-reducer-route`

Accepted fresh target baseline:
`cd4465556d18729121f56f5834ac00f8b0b3d15c`

Current gate:

```sh
cd /workspace/typescript/.codex-worktrees/dnd-fresh-cleanroom-dry-run-fc00
python3 tools/verify_current_fresh_target.py
```

## Purpose

FEXP-00 through FEXP-07 proved that the copied package can drive a fresh target
through a growing reducer-shaped runtime without TypeScript implementation
knowledge or production authored-identity dispatch.

The remaining blockers are now source-input feedback. Do not treat them as
target implementation TODOs until source-side QNT, RAW/domain guidance, or
cleanroom guidance supplies the missing generic facts.

## Blocker Clusters

### Selected Spell Residual Shapes

Source blockers:

- `FEXP-02-attack-spell-shape-selected-identity-effects-outside-generic-substrate`
- `FEXP-02-level1-damage-spell-selected-identity-effects-outside-generic-substrate`

Needed source input:

- focused generic route connectors for hit-point-regain prevention,
  next-attack advantage/disadvantage, opportunity-attack denial,
  poison/condition riders, object/light riders, mixed target outcomes, and
  exact damage projection facts;
- shape facts must be independent of selected spell names, ids, slugs, source
  headings, or fixture labels.

### Public Observability And Stale Subjects

Source blocker:

- `FEXP-03-object-stale-replay-isolated-route-surface-not-publicly-observable`

Needed source input:

- a public route connector that reaches object stale replay through discovery
  and resolution history; or
- an explicit public reducer token protocol for stale object subjects.

### Active-Effect Cleanup And Choice Payloads

Source blockers:

- `FEXP-04-roll-modifier-concentration-break-route-not-publicly-observable`
- `FEXP-04-scalar-active-effect-cumulative-sequence-needs-profile-progression`

Needed source input:

- public concentration-break cleanup routes;
- generic active-effect profile progression facts;

Resolved source input:

- `FEXP-04-exact-roll-choice-payloads-not-admitted-by-route-fills` is resolved
  on source `master` by commit `0c2ba34c5a45f18b73dfe590e0e86419ba377375`.
  The source package now exposes exact skill-choice, ability-choice, and
  two-target ability-choice route-fill payload evidence in focused route QNT
  without importing the `battle-runtime-model.qnt` barrel or adding production
  authored-identity dispatch. Dirty target commit
  `801b05df55a1393d6acd5c3fa7b2624ed91f9494` focused-replays those payloads
  through reducer route events. Fresh target commit
  `b43797af240c1486e5ad92c3698bf2cd2958a91e` refreshes the copied package to
  source `0c2ba34c5a45f18b73dfe590e0e86419ba377375` and accepts the
  roll-modifier exact skill-choice, ability-choice, and two-target
  ability-choice payloads through public reducer route events.
- `FEXP-04-ability-check-search-choice-payloads-not-publicly-observable` is
  resolved in the fresh target by commit
  `cd4465556d18729121f56f5834ac00f8b0b3d15c`. The copied
  `battle-runtime-ability-check-choice-search.route.mbt.qnt` connector already
  exposed `AbilityCheckSearchRouteSubject`; the fresh target now observes Search
  target choice, Ability Check opening, invalid Search target/fill rejection,
  Search failure, and Search success through public reducer entrypoints instead
  of a replay-only island.

### Reaction And Interrupt Payload Taxonomy

Source blockers:

- `FEXP-05-selected-reaction-spell-projections-need-generic-source-shapes`
- `FEXP-05-interrupt-trigger-taxonomy-not-admitted-by-route-inputs`

Needed source input:

- generic reaction active-effect, post-damage save/damage, slot expenditure,
  and interrupted spell-slot preservation routes;
- typed interrupt trigger and continuation payload vocabulary, or explicit
  guidance that trigger labels remain evidence-only.

### Character, Sheet, And Settlement Payloads

Source blockers:

- `FEXP-06-creation-rejection-and-partial-fill-branches-not-accepted`
- `FEXP-06-sheet-spell-resource-rejection-rest-feature-branches-not-accepted`
- `FEXP-06-battle-settlement-conflict-and-zero-hp-branches-not-accepted`
- `FEXP-06-non-route-projection-fields-not-runtime-settled`

Needed source input:

- typed draft facts for partial fills, stale rejection, selected-reference
  retention, and build projection;
- durable rest/resource payloads for sheet spell resources and feature
  recovery;
- executable settlement conflict, source-exact resource delta, feature-resource
  handoff, and zero-HP stable lifecycle facts;
- generic build-projection inputs for hit-point maximum arithmetic without
  authored class or feature identity.

### Feature, Species, And Metamagic Substrates

Source blockers:

- `FEXP-07-selected-and-grouped-identity-drivers-not-accepted`
- `FEXP-07-residual-species-passive-substrates-need-generic-runtime-facts`
- `FEXP-07-residual-metamagic-projections-need-typed-spell-modification-facts`
- `FEXP-07-feature-selected-identity-residual-branches-not-accepted`

Corrected campaign note:

- `battle-runtime-sorcerer-metamagic.mbt.qnt` is not a source package driver.
  The source package exposes selected-identity metamagic MBT drivers plus
  `battle-runtime-sorcerer-metamagic.route.mbt.qnt`. The earlier exact-driver
  blocker was a fresh-expansion lane manifest naming error, not a source-input
  blocker.

Needed source input:

- focused generic route connectors for selected/grouped identity branches;
- typed runtime facts for passive damage, saving-throw/ability-check roll mode,
  creature-space movement, spell-modification payloads, spell-source profiles,
  and active-feature spell attack roll-mode.

## Review Gate To Preserve

Any source-side response to these blockers should keep the FEXP acceptance rule:

- expected route records are derived from `.qnt` connector action bodies/helper
  vocabulary;
- observed records come from public reducer, character, sheet, or handoff
  entrypoints;
- every durable runtime field has a state-owner derivability record;
- selected identity stays out of production reducer dispatch.
