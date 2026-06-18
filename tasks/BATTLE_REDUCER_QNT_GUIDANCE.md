# Battle Reducer From QNT Guidance

## Current Answer

QNT is enough to derive a small reducer spine, route seven existing battle MBT
adapter tests through it, and derive two reusable rule-core reducer-control
components, but the cleanroom repo has not yet proven that QNT is enough to
derive the full battle reducer.

The tracked experiment in `src/rules/battle_reducer_spine.rs` proves a narrow
path:

- QNT `battle-runtime-model.qnt` provides a reducer-shaped `BattleState` seed.
- QNT `battle-runtime-turn-order.qnt` provides current actor and attack
  eligibility.
- QNT `battle-runtime-combat-holes.qnt` provides the first open hole.
- QNT `battle-runtime-weapon-attack-ordering.qnt` provides target, attack-roll,
  and damage-roll ordering.
- QNT `shared-algebras/proofs/rule-core/attack-damage-composition.qnt`
  provides attack-roll hit logic.
- Existing Rust HP helpers derived from rule-core QNT apply damage to durable
  combatant HP.
- QNT `battle-runtime-weapon-attack-skeleton.mbt.qnt` provides a richer literal
  projection for Skeleton HP, action spending, sneak attack use, and Skeleton
  multiattack dispatch.
- QNT `rule-core-stat-block-controls.mbt.qnt` and
  `shared-algebras/proofs/rule-core/stat-block-controls.qnt` provide a reusable
  stat-block multiattack control transition for primary/secondary dispatches,
  movement/end-turn interleaving, and stale-subject rejection.
- QNT `battle-runtime-stat-block-action-ordering.mbt.qnt`,
  `battle-runtime-stat-block-action-ordering.qnt`, and
  `battle-runtime-stat-block-bridge.qnt` provide stat-block attack-control
  ordering for rolled/static damage, recharge-gated actions, and Goblin
  multiattack dispatch projection.
- QNT `battle-runtime-stat-block-multi-damage.mbt.qnt` provides the adjacent
  stat-block damage-mode projection: rolled hit waits for a damage roll, rolled
  damage changes target HP from 12 to 10, and static hit changes target HP from
  12 to 9 without opening a damage-roll hole.
- QNT `battle-runtime-stat-block-size-gated-condition-rider.mbt.qnt` provides
  the adjacent stat-block rider projection: a hit against a Medium-or-smaller
  target applies Prone, larger targets reject that rider, Prone immunity rejects
  that rider, and damage resolution preserves the accepted rider result.
- QNT `battle-runtime-turn-boundary-effect-lifecycle.mbt.qnt`,
  `battle-runtime-turn-advancement.qnt`, and
  `battle-runtime-turn-order.qnt` provide the first bounded turn-advancement
  projection: two `end_turn` calls advance Source -> Target -> Source, mutate
  target HP at start/end boundaries, reset turn resources, and expire the
  fixture's turn-boundary effects.
- QNT `rule-core-reactions.mbt.qnt` and
  `shared-algebras/proofs/rule-core/reactions-continuations-concentration.qnt`
  provide a reusable reaction transition component for opportunity-attack
  offers/declines, readied movement offers/declines/takes/rejections, and
  concentration-after-damage checks.
- QNT `battle-runtime-interrupt-stack-resume.mbt.qnt`,
  `battle-runtime-replay-equivalence.qnt`, and
  `shared-algebras/proofs/rule-core/reactions-continuations-concentration.qnt`
  provide the first battle-owned nested interrupt route: active+suspended
  reaction-window depth, resumed holes, responder reaction spending, active
  effect mutation on resume, and replay-from-root equivalence.

The experiment passes through reducer-shaped functions:

- `start_battle`
- `start_skeleton_battle`
- `start_skeleton_actor_turn`
- `discover_battle_acts`
- `resolve_battle_subject`
- `resolve_stat_block_action_subject`
- `end_turn`
- `resolve_rule_core_reaction_subject`
- `start_interrupt_stack_resume_battle`
- `interrupt_stack_resume_projection_from_battle`

That is materially closer to the main goal than per-driver replay adapters, but
it is still an experiment. The `battle_runtime_weapon_attack_ordering` and
`battle_runtime_weapon_attack_skeleton` adapters now use the spine for observed
replay and their prior focused helpers for expected projection. The
`battle_runtime_stat_block_action_ordering` and
`battle_runtime_stat_block_multi_damage` and
`battle_runtime_stat_block_size_gated_condition_rider` adapters now do the same
for Goblin stat-block action paths. The
`battle_runtime_turn_boundary_effect_lifecycle` adapter now does the same for a
bounded Source/Target `end_turn` path. The
`battle_runtime_interrupt_stack_resume` adapter now does the same for a bounded
nested reaction-window/resume path. The T031, T060, T062, T063, T064, T072,
T074, T079, and T080 target replay evidence files now have current manifest and
inventory metadata. The rolling start gate currently selects T031 for this
diagnostic, but repo-wide work-loop acceptance still lacks the
run-ledger/history denominator needed to accept it as a completed queued task.

T074 now supplies the reducer-control component used by the T064 Skeleton
multiattack path. `BattleState` stores `StatBlockControlState`; Skeleton
multiattack start calls `start_stat_block_multiattack_from`; dispatch spending
calls `resolve_stat_block_control_subject`; and the T064 projection derives
`qMultiattackDispatchesAvailable` from the stat-block control dispatch counts.
This proves that the copied QNT supports a general stat-block continuation rule
and that the battle spine can reuse it for both the Skeleton fixture path and
the Goblin stat-block action-ordering path. T079 then proves static and rolled
stat-block damage can mutate durable `BattleState` HP through the same
stat-block subject/fill entrypoint. T080 proves a size-gated Prone rider can
use durable target size and condition-immunity facts through that same
entrypoint. T062 proves a bounded turn-boundary lifecycle path can use durable
initiative, target HP, and active-effect facts through `end_turn`. These still
do not prove a general stat-block action catalog, arbitrary stat-block profile
admission, or general active-effect storage.

T072 now supplies the reducer-control component for reactions. It exposes
`resolve_rule_core_reaction_subject`, which is the rule-core transition surface
needed before the battle spine can own nested reaction windows and continuation
resume. T031 shows that the battle spine also needs the broader nested
reaction-window model from `reactions-continuations-concentration.qnt`; T072's
public transition surface is useful for simple reaction decisions but is not
yet the one general reaction substrate.

## How To Guide The Next Cleanroom Work

Tell the cleanroom implementer to start from the reducer spine, not from a
single MBT adapter.

Concrete instruction:

1. Treat `battle-runtime-model.qnt`, `battle-runtime-turn-order.qnt`,
   `battle-runtime-actor-combatants.qnt`,
   `battle-runtime-combat-holes.qnt`, and the relevant rule-core QNT as the
   reducer-spine source.
2. Implement reusable reducer entrypoints first:
   - start/init battle state;
   - discover acts from durable state;
   - resolve one subject with typed fills;
   - return either resolved state, next holes, or typed invalid result.
3. Only then wire an MBT adapter to those entrypoints.
4. Record a blocker whenever the implementer needs a fact that is not in copied
   QNT, RAW, domain docs, or assumptions.
5. Compare against TypeScript after the cleanroom derivation, not during it.

## Options Considered

### Option A: Keep Expanding Per-Slice Replay Adapters

Useful for branch coverage, but not sufficient for the main goal. It proves the
target can mimic QNT-shaped traces, not that it can discover and resolve battle
acts through a durable reducer.

Use this only after the reducer-spine replay pattern exists.

### Option B: Classify All 73 Active Battle/Rule-Core Drivers First

Not the next best step. Classification is useful later, but a full audit before
the reducer-spine MBT integration would spend effort on labels before proving
the harness can consume the right abstraction.

Use a small classification only for drivers selected into the spine experiment.

### Option C: Directly Port The TypeScript Reducer Shape

Fast for a dirty usefulness experiment, but it answers a different question. It
would measure how well Rust can mirror TypeScript, not whether QNT carries the
contract. TypeScript should be used only as a post-hoc comparison.

### Option D: Add Source-Side QNT Guidance Before Any More Rust

Useful if the spine integration hits blockers. Premature before that. The
current copied QNT is sufficient for the first narrow spine, so the immediate
need is to find the first missing fact by trying MBT-through-spine.

### Option E: Make A New Reducer-Spine MBT Witness

Likely useful. Existing drivers are mostly focused on slice projection. A small
source-side witness that explicitly names `Start -> Discover -> Resolve`
through durable `BattleState` would make the cleanroom task much less dependent
on hidden architecture inference.

Do this if adapting an existing MBT driver to the spine requires too much
interpretation.

### Option F: Route Existing Adapters Through The Spine

Done for `battle_runtime_weapon_attack_ordering`,
`battle_runtime_weapon_attack_skeleton`,
`battle_runtime_interrupt_stack_resume`,
`battle_runtime_stat_block_action_ordering`,
`battle_runtime_stat_block_multi_damage`,
`battle_runtime_stat_block_size_gated_condition_rider`, and
`battle_runtime_turn_boundary_effect_lifecycle`. This was the fastest
way to test whether existing focused MBT projection can use reducer-shaped
observed replay without changing source QNT.

Result: it works for the ordering driver and for the Skeleton weapon-attack
driver, and for the Goblin stat-block action-ordering, multi-damage, and
size-gated condition-rider drivers. The per-file evidence for T060, T063, T064,
T079, and T080 is now current and validates cleanly when checked against each
selected inventory slice; T062 is also current and validates cleanly for its two
turn-boundary obligations. T031 is current and validates cleanly for its three
interrupt-stack obligations. T072 and T074 are current reusable rule-core
components rather than full battle-spine routes. The remaining issue is
work-loop acceptance:
`tasks/RUN_LEDGER.json`, rolling artifacts, and history do not yet declare
these drivers as accepted selected work. A T031/T060/T062/T063/T064/T072/T074/T079/T080
work-loop record would still not make this dirty repo pass
`node scripts/check-cleanroom-harness.cjs`, because the harness denominator also
includes the other evidence files and undeclared adapter modules.

### Option G: Factor Rule-Core Reducer Components First

Started for `rule-core-stat-block-controls` and now composed into the Skeleton
battle-spine path. This is the right shape when a driver owns reusable reducer
policy but is not itself a full battle state driver. The same pattern is now
used for `rule-core-reactions`: T072 exposes a transition API, and T031 now
routes the first nested interrupt path through battle state. The T074
transition API is available for
broader stat-block battle integration:

- `start_stat_block_multiattack_from`
- `resolve_stat_block_control_subject`
- `resolve_stat_block_multiattack_dispatch_from`
- `end_stat_block_turn_from`

Use this for shared control rules before adding more one-off battle-spine
branches.

## What Is Still Missing

- No complete work-loop task currently accepts `battle_reducer_spine.rs`.
- Seven existing battle `.mbt.qnt` adapter tests and two reusable rule-core
  component adapter tests have been replayed through reducer-shaped entrypoints
  or transition APIs, but the JSON evidence schema cannot independently prove
  that call graph.
- The spine handles only the Fighter/Goblin weapon ordering path, the
  Rogue/Skeleton weapon-attack skeleton path, and the Goblin stat-block
  action-ordering/multi-damage/size-gated rider paths.
- It does not yet model:
  - general stat-block action discovery beyond the Goblin fixture route;
  - general stat-block primary/secondary dispatch selection beyond the Skeleton
    and Goblin fixture profiles;
  - general initiative advancement beyond the bounded Source/Target T062
    fixture;
  - general bonus actions, battle-integrated reactions, movement, or spell
    slots;
  - interrupt stack continuations;
  - general concentration, ongoing effects, and turn-boundary effects;
  - general multiattack dispatch beyond the T074-composed Skeleton and Goblin
    fixture paths;
  - character battle-init projection or settlement.

## Best Next Step

Current measured position:

- 73 active battle/rule-core drivers carry 502 in-scope obligations.
- 9 drivers are currently reducer-spine/control routed:
  T031/T060/T062/T063/T064/T072/T074/T079/T080.
- Those 9 drivers cover 62 obligations.
- 64 battle/rule-core drivers and 440 obligations remain unrouted.
- The copied support corpus already contains reducer-shaped source facts:
  50 support modules mention `BattleState`, 57 mention spell resource/slot
  ownership, 21 mention interrupt stack/reaction windows, and 77 mention turn
  advancement/resource reset.

Do not try to promote T031/T060/T062/T063/T064/T072/T074/T079/T080 alone inside the current
dirty repo. Use one of two paths:

1. Add a source-side reducer-spine witness if the current evidence schema is not
   strong enough. This is the cleanest reducer-specific next step because it
   makes `start_battle` / `discover_battle_acts` / `resolve_battle_subject`
   explicit source input for future cleanroom runs. This is source-repo work,
   not something this cleanroom target may implement by editing
   `cleanroom-input/**`.
2. Continue per-file reducer-spine diagnostics in this repo while expanding the
   spine to another driver. Current focused checks:

```bash
node scripts/check-target-replay-evidence-file.cjs \
  --driver cleanroom-input/qnt/battle-runtime/battle-runtime-interrupt-stack-resume.mbt.qnt \
  --evidence tasks/target-replay-evidence/T031-battle-runtime-interrupt-stack-resume.json

node scripts/check-target-replay-evidence-file.cjs \
  --driver cleanroom-input/qnt/battle-runtime/battle-runtime-stat-block-action-ordering.mbt.qnt \
  --evidence tasks/target-replay-evidence/T060-battle-runtime-stat-block-action-ordering.json

node scripts/check-target-replay-evidence-file.cjs \
  --driver cleanroom-input/qnt/battle-runtime/battle-runtime-weapon-attack-ordering.mbt.qnt \
  --evidence tasks/target-replay-evidence/T063-battle-runtime-weapon-attack-ordering.json

node scripts/check-target-replay-evidence-file.cjs \
  --driver cleanroom-input/qnt/battle-runtime/battle-runtime-weapon-attack-skeleton.mbt.qnt \
  --evidence tasks/target-replay-evidence/T064-battle-runtime-weapon-attack-skeleton.json

node scripts/check-target-replay-evidence-file.cjs \
  --driver cleanroom-input/qnt/battle-runtime/rule-core-stat-block-controls.mbt.qnt \
  --evidence tasks/target-replay-evidence/T074-rule-core-stat-block-controls.json

node scripts/check-target-replay-evidence-file.cjs \
  --driver cleanroom-input/qnt/battle-runtime/rule-core-reactions.mbt.qnt \
  --evidence tasks/target-replay-evidence/T072-rule-core-reactions.json

node scripts/check-target-replay-evidence-file.cjs \
  --driver cleanroom-input/qnt/battle-runtime/battle-runtime-stat-block-multi-damage.mbt.qnt \
  --evidence tasks/target-replay-evidence/T079-battle-runtime-stat-block-multi-damage.json

node scripts/check-target-replay-evidence-file.cjs \
  --driver cleanroom-input/qnt/battle-runtime/battle-runtime-stat-block-size-gated-condition-rider.mbt.qnt \
  --evidence tasks/target-replay-evidence/T080-battle-runtime-stat-block-size-gated-condition-rider.json

node scripts/check-target-replay-evidence-file.cjs \
  --driver cleanroom-input/qnt/battle-runtime/battle-runtime-turn-boundary-effect-lifecycle.mbt.qnt \
  --evidence tasks/target-replay-evidence/T062-battle-runtime-turn-boundary-effect-lifecycle.json
```

This diagnostic command validates current per-file evidence but does not close
work-loop acceptance.

When the dirty harness denominator is cleaned up, the preferred work-loop path
is:

1. Use `battle-runtime-stat-block-action-ordering.mbt.qnt`,
   `battle-runtime-weapon-attack-ordering.mbt.qnt`,
   `battle-runtime-weapon-attack-skeleton.mbt.qnt`, and
   `battle-runtime-stat-block-multi-damage.mbt.qnt`, plus
   `battle-runtime-stat-block-size-gated-condition-rider.mbt.qnt`, plus
   `battle-runtime-turn-boundary-effect-lifecycle.mbt.qnt`, plus
   `battle-runtime-interrupt-stack-resume.mbt.qnt`, plus
   `rule-core-reactions.mbt.qnt`, plus
   `rule-core-stat-block-controls.mbt.qnt`, because they already route observed
   replay through reducer-shaped entrypoints and their per-file evidence is
   current.
2. Build complete accepted T031/T060/T062/T063/T064/T072/T074/T079/T080 work-loop records:
   `START_GATE`, engine depth, state ownership, review loop, decider decision,
   history, run ledger, and validation report.
3. If the evidence schema must identify the reducer-spine surface more strongly,
   add a small source-side reducer-spine witness or evidence schema note before
   broadening to more drivers.
4. If a fact is missing, stop and improve source QNT/guidance rather than
   importing TypeScript knowledge.

The next useful Rust diagnostic, if continuing before a source-side witness,
should leave the already-covered Goblin stat-block attack path and the bounded
T062 lifecycle fixture unless it tests a different reducer subsystem.

Recommended sequence:

1. **Reaction spell casting time.** Route
   `battle-runtime-reaction-casting-time.mbt.qnt` with a minimal battle-owned
   spell-slot/action-resource owner. The copied QNT already specifies the
   needed substrate in `battle-runtime-model.qnt`,
   `battle-runtime-spell-invocation.qnt`,
   `battle-runtime-reaction-window.qnt`, and
   `battle-runtime-reaction-resolution.qnt`: pending slot claims, committed
   slot use, pending-slot release, Counterspell spell-ending/resume behavior,
   reaction spending, reaction-window clearing, and Hellish Rebuke after-damage
   resolution.
2. **Spell attack/save-gated ordering.** Defer
   `battle-runtime-spell-attack-ordering.mbt.qnt` and
   `battle-runtime-save-gated-spell-ordering.mbt.qnt` unless the batch also
   introduces real spell-slot/action-resource ownership. Alone, they mostly
   repeat the already measured weapon/stat-block ordering result.
3. **Command next-turn options.** Defer
   `battle-runtime-command-option-next-turn.mbt.qnt` until turn advancement,
   movement, and reaction-window substrate are present. Its QNT explicitly
   leaves route choice and held-object inventory as table-owned facts, so it is
   an integration test, not the first substrate test.

This is the first experiment that directly measures the goal: whether battle
reducer behavior can be reconstructed from QNT, not merely whether focused QNT
traces can be copied into Rust.

## Exhausted Options For Now

- More Goblin stat-block micro-slices are low value until a new stat-block
  subsystem appears; T060/T079/T080 already cover ordering, rolled/static
  damage mutation, and size-gated condition riders through `BattleState`.
- Another bounded turn-boundary fixture is low value unless it forces general
  active-effect storage; T062 already proves a first `end_turn` route.
- More ordering-only spell drivers are low value until spell-slot/action
  ownership exists; otherwise they repeat T060/T063 with different hole names.
- A full 73-driver classification is not currently useful. Classify a driver
  when selected for spine routing or when a passing replay could be mistaken
  for reducer evidence.
- Directly mirroring TypeScript architecture would answer a different question.
  The cleanroom reducer claim must be driven from copied QNT/RAW/domain input,
  with TypeScript used only as post-hoc comparison outside the cleanroom run.
- Work-loop promotion is blocked by dirty denominator accounting, not by QNT:
  missing `tasks/RUN_LEDGER.json`, 79 undeclared historical evidence files, and
  undeclared adapter scan findings keep repo-wide harness acceptance red.
