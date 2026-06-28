# Fresh Expansion Blocker Source Feedback

Campaign: `level-1-2-runtime-reducer-route`

Accepted fresh target baseline:
`05280a8e2d6e9705411c114c80ae2a4e4290de2c`

Current gate:

```sh
cd /workspace/typescript/.codex-worktrees/dnd-fresh-cleanroom-dry-run-fc00
python3 tools/verify_current_fresh_target.py
```

Current dirty package refresh:

- source package: `d63838e22137c4b329dc877ca0d963876f3459bf`
- branch: `ralph/rrconv-19-cleanroom`
- boundary: package/state reconciliation only; this refresh does not claim
  fresh target acceptance or new Rust runtime coverage.
- packaged source-feedback items: Task 3 residual selected-spell route-task
  queue, Task 8 active reducer diagnostic seed, FCSF-04 object stale public
  route-history evidence, FCSF-05 reaction/interrupt payload taxonomy
  evidence, and FCSF-06 character/sheet/handoff rejection and resource payload
  evidence.

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

Packaged source input:

- Source-feedback Task 3 is now carried in
  `cleanroom-input/branch-coverage/reducer-route-inventory.json` as
  `task3ResidualSelectedSpellRouteTasks[]`.
- Those records are the future source route-task queue for Hit Point regain
  prevention, next-Attack-Roll mode, Opportunity Attack denial, condition and
  poison riders, object/light riders, mixed target outcomes, and exact damage
  projection facts.
- They are not target replay acceptance. Each residual branch still needs
  copied generic route or component connector evidence before a target may
  count it, and production runtime code must not dispatch on selected spell
  identity.

Needed source input:

- focused generic route connectors for hit-point-regain prevention,
  next-attack advantage/disadvantage, opportunity-attack denial,
  poison/condition riders, object/light riders, mixed target outcomes, and
  exact damage projection facts;
- shape facts must be independent of selected spell names, ids, slugs, source
  headings, or fixture labels.

### Public Observability And Stale Subjects

Resolved source input:

- `FEXP-03-object-stale-replay-isolated-route-surface-not-publicly-observable`
  is resolved as source input by FCSF-04 at source commit
  `e8cb231ce1183ed96b2865000562f0395f2d712f`. The copied
  `battle-runtime-starry-wisp-object.route.mbt.qnt` connector now reaches
  stale object replay through public object-target discovery,
  target-boundary resolution, Attack Roll discovery/resolution, damage-roll
  discovery/resolution, active-effect resolution, and then
  `BattleHoleFrontierOwner` stale rejection.

Target status:

- not yet fresh-target replay acceptance;
- future target evidence must observe this route through public reducer
  entrypoints instead of fabricating an isolated stale subject token.

### Active-Effect Cleanup And Choice Payloads

Source blockers:

- none currently open for FEXP-04.

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
- `FEXP-04-roll-modifier-concentration-break-route-not-publicly-observable` is
  resolved by source feedback task `FCSF-01` at source commit
  `c62aa73be7f80e4d3a5b460aa2bef42cea0c0f7d` and accepted in the fresh target
  by commit `bd6c6ba2407ac00a8295bbe1cd66a70e5ae8364c`. The refreshed
  Concentration teardown route connector and reducer-spine guidance expose
  failed-save, voluntary-end, and replacement Concentration cleanup through
  `BattleConcentrationOwner` and `BattleActiveEffectOwner`; the fresh target
  observes those surfaces through public reducer entrypoints instead of a
  replay-only island.
- `FEXP-04-scalar-active-effect-cumulative-sequence-needs-profile-progression`
  is resolved by source feedback task `FCSF-02` at source commit
  `ee4894fa71e9307b9251639f0b54577ff764c63f` and accepted in the fresh target
  by commit `05280a8e2d6e9705411c114c80ae2a4e4290de2c`. The refreshed package
  exposes generic scalar-buff active-effect projections and projection domains
  through copied QNT, and the fresh target verifies the scalar profile sequence
  through public reducer route observations without authored identity dispatch.

Packaged source input:

- Source-feedback Task 8 remains packaged in current source package
  `d63838e22137c4b329dc877ca0d963876f3459bf` as a six-driver active
  reducer-spine diagnostic seed. The active batch includes Concentration
  teardown and scalar-buff active-effect profile projection replay work, and
  `cleanroom-input/guidance/reducer-spine.md` requires future target evidence
  to prove those facts through public reducer/source entrypoints rather than
  dirty cleanroom history.
- Condition-immunity appears in the scalar-buff rule-core vocabulary, but it is
  still not accepted as scalar-buff route replay evidence. It remains with the
  selected-spell residual blocker until a generic condition-immunity plus
  turn-start Temporary Hit Point active-effect substrate exists.

### Reaction And Interrupt Payload Taxonomy

Resolved source input:

- `FEXP-05-selected-reaction-spell-projections-need-generic-source-shapes`
- `FEXP-05-interrupt-trigger-taxonomy-not-admitted-by-route-inputs`
  are resolved as source input by FCSF-05 at source commit
  `cf60f7a5b822ee9d9458e98577d47026242fd16e`. The copied
  `battle-runtime-reaction-interrupt-payload-taxonomy.route.mbt.qnt` connector
  exposes generic reaction trigger/procedure/continuation facts and owner
  routes for reaction Armor Class effects, after-damage save/damage payloads,
  spell interruption end/resume, and falling mitigation.

Target status:

- not yet fresh-target replay acceptance;
- future target evidence must consume the generic taxonomy route through public
  reducer entrypoints rather than selected reaction spell identity.

### Character, Sheet, And Settlement Payloads

Resolved source input:

- `FEXP-06-creation-rejection-and-partial-fill-branches-not-accepted`
- `FEXP-06-sheet-spell-resource-rejection-rest-feature-branches-not-accepted`
- `FEXP-06-battle-settlement-conflict-and-zero-hp-branches-not-accepted`
- `FEXP-06-non-route-projection-fields-not-runtime-settled`
  are resolved as source input by FCSF-06 at source commit
  `d63838e22137c4b329dc877ca0d963876f3459bf`. The copied character creation,
  character sheet, and character-battle route connectors expose typed draft
  partial-fill, stale-rejection, selected-reference retention, build-projection,
  sheet spell-resource/rest/feature-resource, source-exact resource delta,
  settlement conflict, generic Hit Point maximum, and zero-HP Stable lifecycle
  facts without authored identity dispatch.

Target status:

- not yet fresh-target replay acceptance;
- future target evidence must consume these typed route facts through public
  character creation, character sheet, handoff, settlement, and battle reducer
  entrypoints rather than local replay payload islands.

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
