# Battle Reducer From QNT Guidance

## Current Answer

QNT is enough to derive a small reducer spine and route one existing MBT adapter
test through it, but the cleanroom repo has not yet proven that QNT is enough to
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

The experiment passes through reducer-shaped functions:

- `start_battle`
- `discover_battle_acts`
- `resolve_battle_subject`

That is materially closer to the main goal than per-driver replay adapters, but
it is still an experiment. The `battle_runtime_weapon_attack_ordering` adapter
now uses the spine for observed replay and the prior focused helper for expected
projection. The T063 target replay evidence file now has current manifest and
inventory metadata and records the spine replay function in `generatedBy.name`,
but the repo-wide work-loop artifacts do not yet select T063 as an accepted
task.

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

### Option F: Route One Existing Adapter Through The Spine

Done for `battle_runtime_weapon_attack_ordering`. This was the fastest way to
test whether existing focused MBT projection can use reducer-shaped observed
replay without changing source QNT.

Result: it works for the ordering driver. The per-file evidence for T063 is now
current and validates cleanly when checked against the selected T063 inventory
slice. The remaining issue is work-loop acceptance: `tasks/RUN_LEDGER.json`,
rolling artifacts, and history do not yet declare T063 as the accepted selected
driver.

## What Is Still Missing

- No complete work-loop task currently accepts `battle_reducer_spine.rs`.
- One existing `.mbt.qnt` adapter test and per-file evidence have been replayed
  through
  `start_battle`/`discover_battle_acts`/`resolve_battle_subject`, but the JSON
  evidence schema cannot independently prove that call graph.
- The spine handles only a Fighter weapon attack against the Goblin-like QNT
  initial state.
- It does not yet model:
  - stat-block action discovery for the Goblin;
  - initiative advancement;
  - bonus actions, reactions, movement, or spell slots;
  - interrupt stack continuations;
  - concentration, ongoing effects, and turn-boundary effects;
  - multiattack dispatch;
  - character battle-init projection or settlement.

## Best Next Step

Promote the T063 spine route into a complete work-loop task, or add a
source-side reducer-spine witness if the current evidence schema is not strong
enough. The preferred path is:

1. Use `battle-runtime-weapon-attack-ordering.mbt.qnt`, because its Rust adapter
   already routes observed replay through the spine and its per-file evidence is
   current.
2. Build a complete accepted T063 work-loop record: `START_GATE`, engine depth,
   state ownership, review loop, decider decision, history, run ledger, and
   validation report.
3. If the evidence schema must identify the reducer-spine surface more strongly,
   add a small source-side reducer-spine witness or evidence schema note before
   broadening to more drivers.
4. If a fact is missing, stop and improve source QNT/guidance rather than
   importing TypeScript knowledge.

This is the first experiment that directly measures the goal: whether battle
reducer behavior can be reconstructed from QNT, not merely whether focused QNT
traces can be copied into Rust.
