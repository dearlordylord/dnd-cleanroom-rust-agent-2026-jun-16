# Battle Reducer QNT Remaining Driver Triage

## Purpose

This note answers the current cleanroom research question from the target repo:
what is still needed before "reducers from QNT" is a credible claim for the
level-1/2 battle reducer?

It is a coarse triage, not an accepted work-loop ledger. It uses only files
available in this cleanroom repo: `cleanroom-input/**`, `tasks/**`, and `src/**`.
It does not use TypeScript source as cleanroom input.

## Current Measurement

- Active battle/rule-core denominator: 73 drivers.
- Active battle/rule-core in-scope obligations: 502.
- Current reducer-spine/control routed drivers: 12.
- Current reducer-spine/control routed obligations: 77.
- Remaining drivers not routed through the reducer spine or reusable control
  transitions: 61.
- Remaining obligations not routed through the reducer spine or reusable
  control transitions: 425.

Already routed/current for reducer-spine or reusable control evidence:

- `battle-runtime-interrupt-stack-resume.mbt.qnt`
- `battle-runtime-reaction-casting-time.mbt.qnt`
- `battle-runtime-reaction-spell-selected-identity.mbt.qnt`
- `battle-runtime-spell-attack-ordering.mbt.qnt`
- `battle-runtime-stat-block-action-ordering.mbt.qnt`
- `battle-runtime-stat-block-multi-damage.mbt.qnt`
- `battle-runtime-stat-block-size-gated-condition-rider.mbt.qnt`
- `battle-runtime-turn-boundary-effect-lifecycle.mbt.qnt`
- `battle-runtime-weapon-attack-ordering.mbt.qnt`
- `battle-runtime-weapon-attack-skeleton.mbt.qnt`
- `rule-core-reactions.mbt.qnt`
- `rule-core-stat-block-controls.mbt.qnt`

## Classification Used

- `spine-now`: good next Rust reducer-spine experiment; it should reuse current
  `BattleState` owners instead of adding a slice-local state.
- `component-first`: implement or refresh a reusable rule-core/control
  transition before battle-spine integration.
- `substrate-first`: needs a broader durable subsystem first, such as active
  effects, movement, concentration, spell invocation, companions, or form
  lifecycle.
- `catalog-after-substrate`: selected-identity/catalog witness; useful after
  generic substrate exists, weak reducer evidence if implemented as literal
  replay now.
- `replay-refresh-only`: can be refreshed for current-manifest coverage, but it
  does not materially answer the reducer-from-QNT question by itself.

## Exhaustive Remaining Driver Actions

All driver paths below are relative to `cleanroom-input/qnt/battle-runtime/`.

| Driver | In-scope obligations | Bucket | Action needed |
| --- | ---: | --- | --- |
| `battle-runtime-ability-check-choice-search.mbt.qnt` | 9 | component-first | Factor ability/skill command choice and Search/Guidance transitions from rule-core, then route battle discovery through that component. |
| `battle-runtime-adrenaline-rush.mbt.qnt` | 2 | substrate-first | Route only after battle state has a general bonus-action/movement-resource owner for feature-granted Dash. |
| `battle-runtime-after-hit-damage-riders.mbt.qnt` | 22 | substrate-first | Build an after-hit event/rider pipeline that composes weapon hit, spell slot/free-cast choice, ongoing smite/mark effects, saves, start-turn damage, and escape checks. |
| `battle-runtime-attack-spell-shape-selected-identity.mbt.qnt` | 6 | catalog-after-substrate | Defer until spell attack/save damage and active-effect projections are battle-owned; then admit these as spell-shape catalog cases. |
| `battle-runtime-chained-attack-sequence.mbt.qnt` | 10 | spine-now | Good follow-up after T058: route chained spell attacks through the spell-attack procedure and durable target/damage state. |
| `battle-runtime-command-option-next-turn.mbt.qnt` | 15 | substrate-first | Route after command ordering, movement, opportunity-reaction windows, and turn cleanup share durable state. |
| `battle-runtime-command-ordering.mbt.qnt` | 19 | substrate-first | Build the command subject model first: target list, option save ordering, movement option fills, held-object table facts, and suppression effects. |
| `battle-runtime-concentration-break-teardown.mbt.qnt` | 5 | spine-now | Add a battle-owned concentration slot/effect owner, then route replacement, voluntary end, damage save request, and failed-save teardown. |
| `battle-runtime-condition-saving-throw-selected-identity.mbt.qnt` | 4 | catalog-after-substrate | Defer until save-gated condition effects and repeat-save lifecycle are battle-owned. |
| `battle-runtime-creature-type-protection-and-charm-selected-identity.mbt.qnt` | 7 | substrate-first | Needs durable creature-type facts, protection/charm active effects, save advantage, scoped attack disadvantage, and damage-break handling. |
| `battle-runtime-danger-sense-selected-identity.mbt.qnt` | 2 | replay-refresh-only | Small passive save-advantage projection; refresh when save/disadvantage substrate exists. |
| `battle-runtime-death-saving-throw.mbt.qnt` | 3 | spine-now | Add end-turn death-save discovery over durable HP/lifecycle state and reject stale/wrong actor after resolution. |
| `battle-runtime-dragonborn-breath-weapon.mbt.qnt` | 5 | substrate-first | Needs feature resource ownership, area save damage, action replacement/extra-attack interaction, and damage-roll validation. |
| `battle-runtime-druid-wild-shape-form-lifecycle.mbt.qnt` | 7 | substrate-first | Requires a durable form lifecycle owner before battle routing: assume/reuse/dismiss/revert, mounted form facts, incapacitated/death reversion, and turn stutter. |
| `battle-runtime-eldritch-blast.mbt.qnt` | 8 | spine-now | Good spell-attack sequence follow-up after T058: route multi-beam targets, hit/miss fills, damage fills, and stale rejection through battle state. |
| `battle-runtime-feature-selected-identity.mbt.qnt` | 3 | catalog-after-substrate | Defer until feature resource and spell-benefit modifiers are battle-owned; then route Innate Sorcery as a catalog case. |
| `battle-runtime-find-familiar-companion-lifecycle.mbt.qnt` | 5 | substrate-first | Needs companion ownership, replacement, senses, touch-spell delivery, and familiar attack permission before reducer routing. |
| `battle-runtime-find-familiar-selected-identity.mbt.qnt` | 4 | catalog-after-substrate | Defer until the companion lifecycle substrate exists; then this becomes selected-identity admission. |
| `battle-runtime-halfling-nimbleness-selected-identity.mbt.qnt` | 4 | substrate-first | Needs movement occupancy/creature-size traversal substrate. |
| `battle-runtime-healing-stabilization-selected-identity.mbt.qnt` | 1 | replay-refresh-only | Small zero-HP stabilization case; route with death-save/zero-HP lifecycle rather than as its own owner. |
| `battle-runtime-hit-point-restoration-ordering.mbt.qnt` | 9 | spine-now | Good next healing subject route: discover feature pool/spell healing, target choice/list fills, healing roll ordering, and HP mutation. |
| `battle-runtime-level1-buff-mark-smite-selected-identity.mbt.qnt` | 12 | catalog-after-substrate | Defer until weapon-hosted attacks, after-hit riders, concentration, turn-start effects, and active scalar buffs are battle-owned. |
| `battle-runtime-level1-damage-spell-selected-identity.mbt.qnt` | 10 | catalog-after-substrate | Defer until save-gated, spell-attack, object-target, and chained-attack spell substrates are routed. |
| `battle-runtime-level1-spatial-witness-selected-identity.mbt.qnt` | 10 | substrate-first | Needs area/environment effect ownership, light/object emitters, movement hazards, reaction mitigation, and concentration cleanup. |
| `battle-runtime-mage-armor-selected-identity.mbt.qnt` | 4 | catalog-after-substrate | Route after active armor-class effects and duration expiry are battle-owned. |
| `battle-runtime-magic-missile.mbt.qnt` | 2 | spine-now | Small high-value spell-damage route: allocation fill and damage fill should mutate durable target HP through spell invocation state. |
| `battle-runtime-movement-forced-movement-selected-identity.mbt.qnt` | 4 | substrate-first | Needs movement/reaction substrate for forced movement, immediate Dash, and unarmored movement interactions. |
| `battle-runtime-quickened-spell-governor.mbt.qnt` | 11 | substrate-first | Needs battle-owned Magic action, bonus action, slot use, sorcery resource, and one-per-spell/one-level-1-plus-spell-per-turn gates. |
| `battle-runtime-roll-modifier-active-effects.mbt.qnt` | 16 | substrate-first | Build active roll-modifier/concentration storage and discovery before routing Bane/Bless/Enhance/Guidance/Pass without Trace/Thaumaturgy. |
| `battle-runtime-roll-modifier-buff-selected-identity.mbt.qnt` | 5 | catalog-after-substrate | Defer until roll-modifier active effects and damage reduction/AC modifier projection are battle-owned. |
| `battle-runtime-sanctuary-selected-identity.mbt.qnt` | 9 | substrate-first | Needs ward ownership, attack/spell interdiction, retargeting, area-effect exclusion, and ward teardown on hostile acts. |
| `battle-runtime-save-gated-spell-ordering.mbt.qnt` | 10 | spine-now | Best next spell-ordering route if it reuses T084/T085/T058 Magic-action and spell-slot ownership; otherwise it only repeats local ordering evidence. |
| `battle-runtime-scalar-buff-active-effects.mbt.qnt` | 6 | substrate-first | Needs durable scalar active-effect storage for HP max/temp HP/speed/AC/climb effects and stale handling. |
| `battle-runtime-scalar-buff.mbt.qnt` | 2 | replay-refresh-only | Local Longstrider target/stale case; route as part of scalar active-effect substrate, not alone. |
| `battle-runtime-sleep-repeat-save.mbt.qnt` | 8 | substrate-first | Needs concentration, repeat-save discovery, turn advancement, initial save failure, and wake/sleep condition lifecycle. |
| `battle-runtime-sorcerer-metamagic-careful-selected-identity.mbt.qnt` | 2 | catalog-after-substrate | Defer until save-gated damage and metamagic resource/admission are battle-owned. |
| `battle-runtime-sorcerer-metamagic-distant-selected-identity.mbt.qnt` | 1 | replay-refresh-only | Small object-light/range projection; refresh after spell range/effect substrate exists. |
| `battle-runtime-sorcerer-metamagic-empowered-selected-identity.mbt.qnt` | 1 | catalog-after-substrate | Route after spell damage rolls and sorcery resource reroll ownership exist. |
| `battle-runtime-sorcerer-metamagic-extended-selected-identity.mbt.qnt` | 1 | catalog-after-substrate | Route after active-effect duration ownership exists. |
| `battle-runtime-sorcerer-metamagic-heightened-selected-identity.mbt.qnt` | 5 | catalog-after-substrate | Route after save-gated condition/damage and end-turn save effects are battle-owned. |
| `battle-runtime-sorcerer-metamagic-seeking-selected-identity.mbt.qnt` | 1 | catalog-after-substrate | Route after spell-attack reroll ownership exists. |
| `battle-runtime-sorcerer-metamagic-selected-identity.mbt.qnt` | 1 | catalog-after-substrate | Route after Quickened/save-gated substrate exists. |
| `battle-runtime-sorcerer-metamagic-spell-attack-selected-identity.mbt.qnt` | 1 | catalog-after-substrate | Route after T058 spell-attack state and metamagic resource ownership are composed. |
| `battle-runtime-sorcerer-metamagic-spell-attack-sequence-selected-identity.mbt.qnt` | 1 | catalog-after-substrate | Route after chained/sequence spell attacks and Quickened ownership are composed. |
| `battle-runtime-sorcerer-metamagic-subtle-selected-identity.mbt.qnt` | 2 | catalog-after-substrate | Route after sorcery resource admission and spell invocation ownership exist. |
| `battle-runtime-sorcerer-metamagic-transmuted-selected-identity.mbt.qnt` | 2 | catalog-after-substrate | Route after spell damage type ownership exists for save-gated and spell-attack paths. |
| `battle-runtime-sorcerer-metamagic-twinned-selected-identity.mbt.qnt` | 1 | catalog-after-substrate | Route after spell target-cardinality ownership exists. |
| `battle-runtime-species-passive-trait-selected-identity.mbt.qnt` | 4 | replay-refresh-only | Passive feature projections; refresh once resistance/save/movement support is battle-owned. |
| `battle-runtime-starry-wisp-object.mbt.qnt` | 7 | spine-now | Good object-target spell-attack follow-up after T058: target, attack hit/miss, damage high/low, no-object rejection, and stale rejection. |
| `battle-runtime-thaumaturgy-selected-identity.mbt.qnt` | 1 | replay-refresh-only | Small roll-mode projection; not reducer-informative until roll-modifier active effects are battle-owned. |
| `battle-runtime-weapon-hosted-attack-and-riders.mbt.qnt` | 23 | substrate-first | Needs a weapon-hosted spell procedure that composes weapon attack ordering, spell invocation, active weapon buffs, duration cleanup, and rider damage. |
| `battle-runtime-weapon-mastery-selected-identity.mbt.qnt` | 3 | substrate-first | Needs weapon attack rider integration for Cleave/Sap/Topple plus condition/save/movement facts. |
| `battle-runtime-zero-hit-point-mid-resolution.mbt.qnt` | 1 | spine-now | Small but important: route a multi-hit spell resolution where zero HP tears down concentration before the next hit. |
| `creature-attack.mbt.qnt` | 2 | substrate-first | Generalize attack discovery from fixture actors to creature/stat-block actors. |
| `rule-core-ability-skill-command.mbt.qnt` | 31 | component-first | Extract reusable ability/skill command transitions for Guidance, Search, and Command before battle routing. |
| `rule-core-attack-damage-disposition.mbt.qnt` | 2 | component-first | Refresh current evidence and make the melee knockout/ranged rejection rule feed durable HP/lifecycle mutation. |
| `rule-core-features.mbt.qnt` | 18 | component-first | Factor feature resource/action transitions: Action Surge, Second Wind, Rage, Reckless, Cunning Action, Sneak Attack, Tactical Mind, and zero-HP replacement. |
| `rule-core-hit-point-damage.mbt.qnt` | 4 | component-first | Refresh and compose durable HP lifecycle: temp HP absorption, monster death, PC unconsciousness, and massive damage death. |
| `rule-core-movement.mbt.qnt` | 21 | component-first | High-priority foundation for Command and forced movement: movement spend, Dash, Disengage, opportunity attack, grapple/escape/release, prone stand, overspend rejection. |
| `rule-core-shove-outcome.mbt.qnt` | 6 | component-first | Factor Push/Prone/save/legal-destination outcomes for weapon mastery, movement, and condition rider composition. |
| `rule-core-spells.mbt.qnt` | 24 | component-first | Factor reusable spell invocation/procedure transitions for slot use, Mage Armor, Magic Missile, Ray of Frost, Cure Wounds, Healing Word, and readied spells. |

## Options Exhausted For Now

1. **More fixture-only stat-block micro-slices.** T060/T079/T080 plus T074
   already prove the nearest Goblin/stat-block cluster. More of the same would
   give diminishing reducer evidence until general creature/stat-block
   discovery is required.
2. **More ordering-only drivers without shared ownership.** T063 and T058 prove
   weapon and spell attack ordering. Save-gated ordering is still useful only if
   it reuses the battle-owned Magic action and spell-slot substrate.
3. **Selected-identity catalog fan-out before substrates.** Many remaining
   drivers can be replayed literally, but that would not prove reducer
   reconstruction. They should wait behind generic spell, active-effect,
   movement, feature-resource, and concentration owners.
4. **Full 73-driver semantic classification before the next experiment.** The
   coarse triage is enough to choose next work. Detailed classification should
   happen when a driver is selected, because the useful question is what exact
   fact blocks that route.

## Best Next Steps

1. **Best source-side step:** add a reducer-spine witness to the source QNT and
   sync it into `cleanroom-input`. It should make the cleanroom-visible contract
   explicit: `BattleState` init, act discovery, subject resolution, typed fills,
   HP mutation, action/bonus/reaction/spell-slot ownership, interrupt-stack
   continuation, and turn advancement.
2. **Best target-side Rust experiment:** route
   `battle-runtime-save-gated-spell-ordering.mbt.qnt` only if it reuses the
   T084/T085/T058 Magic-action and spell-slot owner. That tests whether the
   spell substrate extends from spell attacks to save-gated spells.
3. **Best small reducer-spine follow-ups:** route Magic Missile, Eldritch Blast,
   Starry Wisp object, hit-point restoration ordering, death saving throw,
   concentration teardown, and zero-hit-point mid-resolution. These each add a
   different reducer fact without requiring a full catalog.
4. **Best foundational component work:** refresh and factor rule-core movement,
   rule-core spells, rule-core features, and rule-core hit-point damage. These
   are the components that would reduce one-off battle branches later.
5. **Best harness/accounting work:** fix the run-ledger/history denominator
   after the research direction is settled. Without that, current per-file
   evidence can be locally validator-clean but repo-wide acceptance stays noisy.

## Guidance For Future Cleanroom Runs

- Start with the reducer spine, not the selected-identity catalog.
- Use focused QNT witnesses as expected projections, but force observed replay
  through `BattleState` when the driver claims battle behavior.
- Add no slice-local resource owner when a battle-owned owner already exists.
- Record a blocker the first time a selected route needs a fact not present in
  copied QNT/RAW/domain/assumptions.
- Promote current T031/T058/T060/T062/T063/T064/T072/T074/T079/T080/T084/T085
  evidence to work-loop acceptance only after the run-ledger/history
  denominator is cleaned up.
