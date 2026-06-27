# FC-06 Source Feedback

Campaign: `level-1-2-runtime-reducer-route`

Status: `complete`

Fresh target: `/workspace/typescript/.codex-worktrees/dnd-fresh-cleanroom-dry-run-fc00`

## What The Fresh Run Proved

- FC-00 through FC-04 proved that the copied cleanroom package can drive a new
  reducer-shaped target without copying dirty Rust implementation code.
- FC-05 proved a character-battle handoff route slice through public
  character-side entrypoints:
  `project_character_sheet_to_battle`,
  `project_character_resources_to_battle`, and `enter_battle_runtime`.
- Successful FC-05 entry calls `start_battle` and stores projected HP,
  Temporary Hit Points, Armor Class, conditions, Spell Slot counts, passive
  Armor Class profile count, and metamagic option count in
  `BattleState` / `CombatantState`, not in a parallel handoff cache.
- The SDK tracer-bullet proved three public surface pieces separately:
  programmatic synthetic sheet creation, sheet-to-battle handoff, and a simple
  reducer-driven battle turn using `BattleSetup`, `start_battle`,
  `discover_battle_acts`, `resolve_battle_subject`, and `advance_turn`.

Evidence:

- `EVIDENCE/fc05-character-battle-init-projection-route.json`
- `EVIDENCE/sdk-tracer-bullet-programmatic-surface.json`
- `FRESH_RUN_REPORT.md` in the fresh target
- `BLOCKERS.json` in the fresh target

## What It Did Not Prove

- FC-05 did not prove the pure Pact Slot or mixed Spell Slot/Pact Slot branches
  in `character-battle-init-projection.mbt.qnt`. The projection witness has
  those branches, but `character-battle-init-projection.route.mbt.qnt` has no
  matching generic route surfaces.
- The SDK tracer-bullet did not prove one integrated
  sheet-handoff-to-simple-turn scenario. The public handoff path creates a
  `BattleState` with the projected character only, with no opponent and no
  subject profiles; the target has no public composition API to add those facts
  before or after entry.
- The fresh run did not prove any source semantics for encounter composition.
  It only exposed that the current cleanroom inputs do not tell a future target
  how to combine a sheet-derived combatant, other combatants, and generic
  battle subject profiles.

## Source-Side Gaps

### Pact Slot Handoff Route Surfaces

Current source inputs:

- `cleanroom-input/qnt/character-battle-runtime/character-battle-init-projection.mbt.qnt`
  includes `doProjectPurePactMagicSlot` and
  `doRejectMixedSpellAndPactSlotInit`.
- `cleanroom-input/qnt/character-battle-runtime/character-battle-init-projection.route.mbt.qnt`
  exposes route facts only for sheet HP/AC/conditions/profile projection,
  spellcasting/metamagic projection, max-HP rejection, and stable-recovery
  rejection.
- `cleanroom-input/qnt/character-battle-runtime/character-battle-reducer-route.qnt`
  has resource-oriented subject, fill, hole, and owner vocabulary, but no
  connector action states how pure Pact Slot projection or mixed
  Spell Slot/Pact Slot rejection routes through that vocabulary.

Required source task:

- Add or split a focused character-battle route connector for Pact Slot handoff
  semantics.
- Inputs: the existing init projection witness, character-battle route
  vocabulary, SRD Spell Slot/Pact Slot/domain terminology, and any curated
  assumption needed for source-distinct battle slots.
- Outputs: executable `qRoute` facts for pure Pact Slot projection and mixed
  Spell Slot/Pact Slot rejection, or an explicit out-of-scope/source-blocker
  record if the source corpus should not model those branches yet.
- Boundary: do not make a cleanroom target decide whether Pact Slot state is a
  Spell Slot subtype, a separate resource owner, or an unsupported mix. That is
  source vocabulary and route ownership.

### Encounter Composition Surface

Current source inputs:

- `cleanroom-input/guidance/reducer-spine.md` defines `start_battle` from
  battle setup and separately defines character-battle handoff entrypoints.
- The FC-05 route connector defines projection and battle-entry route events,
  not encounter assembly.
- The SDK tracer evidence shows that public sheet handoff and simple battle
  turn APIs exist separately, but no source QNT/guidance says how a
  sheet-derived combatant is composed with opponents and generic subject
  profiles.

Required source task:

- Add QNT/guidance for encounter composition at the character-to-battle
  boundary.
- Inputs: character-battle init projection route, battle reducer route
  vocabulary, reducer-spine guidance, and RAW/domain anchors for battle
  participants and Initiative.
- Outputs: a small source-side contract describing how a sheet-derived
  combatant enters an encounter with other combatants and subject profiles.
  The contract should specify owners for participant membership, subject
  profile availability, Initiative/current actor facts, and whether composition
  occurs before `start_battle`, inside handoff entry, or through a typed
  post-entry reducer operation.
- Boundary: the target API shape should follow this contract. A future
  cleanroom agent should not infer encounter composition from the Rust target,
  prior dirty scaffolding, or convenience methods.

## Recommended Source-Side Tasks

1. Create a Pact Slot handoff route connector task.
   Input boundary: existing character-battle init projection witness and route
   vocabulary only. Output boundary: route connector facts or an explicit
   source blocker; no Rust target APIs.

2. Create an encounter composition QNT/guidance task.
   Input boundary: character-battle handoff QNT, battle reducer route QNT,
   reducer-spine guidance, and SRD/domain terms for combat participants and
   Initiative. Output boundary: source-side contract plus cleanroom guidance
   for public target surfaces; no target implementation.

3. Update future fresh-run prompts to require these source facts before
   claiming a full integrated SDK tracer-bullet.
   Input boundary: this FC-06 artifact and fresh target evidence. Output
   boundary: campaign prompts/readiness notes that keep the limitation visible.

## Why This Belongs In QNT And Guidance

Both gaps are ownership and route-shape decisions. If a cleanroom target fills
them in locally, it can pass tests while teaching the wrong architecture:

- Pact Slot handling decides whether resource projection has distinct owners,
  distinct slot facts, or a typed rejection path.
- Encounter composition decides who owns participant membership, subject
  profiles, Initiative/current actor facts, and handoff-to-battle sequencing.

Those are not implementation conveniences. They are cross-layer contracts that
future targets need to share. Encoding them in QNT/guidance keeps route
evidence independent, prevents selected or fixture identity dispatch, and
avoids making dirty or fresh Rust target choices into hidden source semantics.
