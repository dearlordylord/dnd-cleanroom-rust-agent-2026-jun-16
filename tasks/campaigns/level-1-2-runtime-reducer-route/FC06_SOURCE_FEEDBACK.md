# FC-06 Source Feedback

Campaign: `level-1-2-runtime-reducer-route`

Status: `resolved`

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
- After source commit `0387d29f9282037637b4256c3c7f292bab7ef85c`, the fresh
  target also proved the integrated SDK tracer bullet: sheet projection,
  encounter composition, battle entry, act discovery, subject resolution, damage
  mutation, action spend, and turn advancement through public APIs.
- After source commit `b57772b459f1b75592fd45b9196fd60965b534d3`, the copied
  cleanroom input also contains generic route surfaces for pure Pact Slot
  projection and mixed Spell Slot/Pact Slot handoff rejection.

Evidence:

- `EVIDENCE/fc05-character-battle-init-projection-route.json`
- `EVIDENCE/sdk-tracer-bullet-programmatic-surface.json`
- `FRESH_RUN_REPORT.md` in the fresh target
- `BLOCKERS.json` in the fresh target
- `FRESH_SDK_COMPOSITION_ACCEPTANCE.md` in this campaign directory

## What It Did Not Prove

- FC-05 itself did not prove the pure Pact Slot or mixed Spell Slot/Pact Slot
  branches in `character-battle-init-projection.mbt.qnt`. That was a source
  package gap at the time. The source gap is now resolved by commit
  `b57772b459f1b75592fd45b9196fd60965b534d3`. Dirty target replay evidence
  now consumes those Pact Slot route surfaces in
  `tasks/target-replay-evidence/pact-slot-handoff-init-projection-route.json`,
  but no fresh target replay has yet accepted them.
- FC-05 by itself did not prove one integrated sheet-handoff-to-simple-turn
  scenario. That gap is now closed by the later encounter-composition source
  connector and fresh target evidence.

## Source-Side Gaps

### Pact Slot Handoff Route Surfaces

Status: `resolved`

Resolved source inputs:

- `cleanroom-input/qnt/character-battle-runtime/character-battle-init-projection.mbt.qnt`
  includes `doProjectPurePactMagicSlot` and
  `doRejectMixedSpellAndPactSlotInit`.
- `cleanroom-input/qnt/character-battle-runtime/character-battle-init-projection.route.mbt.qnt`
  now exposes route facts for sheet HP/AC/conditions/profile projection,
  ordinary spellcasting/metamagic projection, pure Pact Slot resource
  projection, mixed Spell Slot/Pact Slot rejection, max-HP rejection, and
  stable-recovery rejection.
- `cleanroom-input/qnt/character-battle-runtime/character-battle-reducer-route.qnt`
  provides the resource-oriented subject, fill, hole, and owner vocabulary used
  by those connector actions.

Resolution:

- Source commit `b57772b459f1b75592fd45b9196fd60965b534d3` added executable
  `qRoute` facts for pure Pact Slot projection and mixed Spell Slot/Pact Slot
  rejection.
- The route connector uses generic resource/handoff shape facts:
  `HandoffResourceProjectionRouteSubject`,
  `HandoffSpellResourceProjectionHoleFamily`, `HandoffResourceDeltaFill`,
  `CharacterBattleResourceProjectionOwner`, and
  `CharacterBattleInitProjectionOwner`.
- This resolves the source-side vocabulary and route ownership gap. A future
  fresh target replay can now consume the copied connector instead of inferring
  Pact Slot ownership locally; the dirty target diagnostic replay has already
  consumed it for the existing legacy harness.

### Encounter Composition Surface

Status: `resolved`

Resolved source inputs:

- `cleanroom-input/qnt/character-battle-runtime/character-battle-encounter-composition.route.mbt.qnt`
- `cleanroom-input/qnt/character-battle-runtime/character-battle-reducer-route.qnt`
- `cleanroom-input/guidance/reducer-spine.md`

The route connector records the composition sequence and the owned fact-family
sets for sheet-derived participant candidate, non-sheet participant membership,
Encounter Side relationship ownership, subject-profile availability ownership,
Initiative count ownership, stable Initiative order ownership, and current actor
ownership.

Fresh target evidence:

- `enter_composed_battle_runtime`
- `sdk_can_project_sheet_compose_encounter_and_drive_one_battle_turn`
- `character_battle_encounter_composition_matches_route_connector`
- `EVIDENCE/sdk-tracer-bullet-programmatic-surface.json`

## Recommended Source-Side Tasks

1. Keep the Pact Slot handoff route connector as prerequisite evidence for
   future fresh cleanroom runs.
   Input boundary: existing character-battle init projection witness and route
   vocabulary only. Output boundary: source-side route connector facts; no Rust
   target APIs.

2. Keep the encounter composition route connector as prerequisite evidence for
   future fresh cleanroom runs.
   Input boundary: character-battle handoff QNT, battle reducer route QNT,
   reducer-spine guidance, and SRD/domain terms for combat participants and
   Initiative. Output boundary: source-side contract plus cleanroom guidance
   for public target surfaces; no target implementation.

3. Update future fresh-run prompts to require both Pact Slot and
   encounter-composition source facts before claiming those target behaviors.
   Input boundary: this FC-06 artifact and fresh target evidence. Output
   boundary: campaign prompts/readiness notes that keep the limitation visible.

## Why This Belongs In QNT And Guidance

These gaps were ownership and route-shape decisions. If a cleanroom target fills
one in locally before source QNT/guidance exists, it can pass tests while
teaching the wrong architecture:

- Pact Slot handling decides whether resource projection has distinct owners,
  distinct slot facts, or a typed rejection path; this is now encoded by the
  init-projection route connector.
- Encounter composition decides who owns participant membership, subject
  profiles, Initiative/current actor facts, and handoff-to-battle sequencing;
  this is now encoded by the encounter-composition route connector and fresh
  target evidence.

Those are not implementation conveniences. They are cross-layer contracts that
future targets need to share. Encoding them in QNT/guidance keeps route
evidence independent, prevents selected or fixture identity dispatch, and
avoids making dirty or fresh Rust target choices into hidden source semantics.
