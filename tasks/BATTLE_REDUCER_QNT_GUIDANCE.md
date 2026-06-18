# Battle Reducer From QNT Guidance

## Current Answer

QNT is enough to derive a small reducer spine and route two existing MBT
adapter tests through it, but the cleanroom repo has not yet proven that QNT is
enough to derive the full battle reducer.

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

The experiment passes through reducer-shaped functions:

- `start_battle`
- `start_skeleton_battle`
- `start_skeleton_actor_turn`
- `discover_battle_acts`
- `resolve_battle_subject`

That is materially closer to the main goal than per-driver replay adapters, but
it is still an experiment. The `battle_runtime_weapon_attack_ordering` and
`battle_runtime_weapon_attack_skeleton` adapters now use the spine for observed
replay and their prior focused helpers for expected projection. The T063 and
T064 target replay evidence files now have current manifest and inventory
metadata and record the spine replay functions in `generatedBy.name`, but the
repo-wide work-loop artifacts do not yet select either driver as an accepted
task.

T074 now adds a reducer-control component rather than another battle-spine
route: `rule_core_stat_block_controls_mbt` observes through
`resolve_stat_block_control_subject` and
`start_stat_block_multiattack_from`. This proves that the copied QNT supports a
general stat-block continuation rule, but that component is not yet integrated
into `BattleState`.

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

Done for `battle_runtime_weapon_attack_ordering` and
`battle_runtime_weapon_attack_skeleton`. This was the fastest way to test
whether existing focused MBT projection can use reducer-shaped observed replay
without changing source QNT.

Result: it works for the ordering driver and for the Skeleton weapon-attack
driver. The per-file evidence for T063 and T064 is now current and validates
cleanly when checked against each selected inventory slice. The remaining issue
is work-loop acceptance: `tasks/RUN_LEDGER.json`, rolling artifacts, and history
do not yet declare these drivers as accepted selected work. A T063/T064-only
work-loop record would still not make this dirty repo pass
`node scripts/check-cleanroom-harness.cjs`, because the harness denominator
also includes the other evidence files and undeclared adapter modules.

### Option G: Factor Rule-Core Reducer Components First

Started for `rule-core-stat-block-controls`. This is the right shape when a
driver owns reusable reducer policy but is not itself a full battle state
driver. The T074 transition API is now available for later battle-spine
integration:

- `start_stat_block_multiattack_from`
- `resolve_stat_block_control_subject`
- `resolve_stat_block_multiattack_dispatch_from`
- `end_stat_block_turn_from`

Use this for shared control rules before adding more one-off battle-spine
branches.

## What Is Still Missing

- No complete work-loop task currently accepts `battle_reducer_spine.rs`.
- Two existing `.mbt.qnt` adapter tests and per-file evidence files have been
  replayed through reducer-shaped entrypoints, but the JSON evidence schema
  cannot independently prove that call graph.
- The spine handles only the Fighter/Goblin ordering path and the
  Rogue/Skeleton weapon-attack skeleton path.
- It does not yet model:
  - stat-block action discovery for the Goblin;
  - stat-block primary/secondary dispatches inside durable `BattleState`;
  - initiative advancement;
  - bonus actions, reactions, movement, or spell slots;
  - interrupt stack continuations;
  - concentration, ongoing effects, and turn-boundary effects;
  - general multiattack dispatch beyond the T074 rule-core component and the
    Skeleton fixture path;
  - character battle-init projection or settlement.

## Best Next Step

Do not try to promote T063/T064/T074 alone inside the current dirty repo. Use
one of two paths:

1. Add a source-side reducer-spine witness if the current evidence schema is not
   strong enough. This is the cleanest reducer-specific next step because it
   makes `start_battle` / `discover_battle_acts` / `resolve_battle_subject`
   explicit source input for future cleanroom runs.
2. Continue per-file reducer-spine diagnostics in this repo while expanding the
   spine to another driver. Use:

```bash
node scripts/check-target-replay-evidence-file.cjs \
  --driver cleanroom-input/qnt/battle-runtime/battle-runtime-weapon-attack-ordering.mbt.qnt \
  --evidence tasks/target-replay-evidence/T063-battle-runtime-weapon-attack-ordering.json

node scripts/check-target-replay-evidence-file.cjs \
  --driver cleanroom-input/qnt/battle-runtime/battle-runtime-weapon-attack-skeleton.mbt.qnt \
  --evidence tasks/target-replay-evidence/T064-battle-runtime-weapon-attack-skeleton.json

node scripts/check-target-replay-evidence-file.cjs \
  --driver cleanroom-input/qnt/battle-runtime/rule-core-stat-block-controls.mbt.qnt \
  --evidence tasks/target-replay-evidence/T074-rule-core-stat-block-controls.json
```

This diagnostic command validates current per-file evidence but does not close
work-loop acceptance.

When the dirty harness denominator is cleaned up, the preferred work-loop path
is:

1. Use `battle-runtime-weapon-attack-ordering.mbt.qnt`,
   `battle-runtime-weapon-attack-skeleton.mbt.qnt`, and
   `rule-core-stat-block-controls.mbt.qnt`, because they already route observed
   replay through reducer-shaped entrypoints and their per-file evidence is
   current.
2. Build complete accepted T063/T064/T074 work-loop records: `START_GATE`,
   engine depth, state ownership, review loop, decider decision, history, run
   ledger, and validation report.
3. If the evidence schema must identify the reducer-spine surface more strongly,
   add a small source-side reducer-spine witness or evidence schema note before
   broadening to more drivers.
4. If a fact is missing, stop and improve source QNT/guidance rather than
   importing TypeScript knowledge.

This is the first experiment that directly measures the goal: whether battle
reducer behavior can be reconstructed from QNT, not merely whether focused QNT
traces can be copied into Rust.
