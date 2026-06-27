# Fresh SDK Composition Acceptance

Campaign: `level-1-2-runtime-reducer-route`

Status: `accepted`

Fresh target:
`/workspace/typescript/.codex-worktrees/dnd-fresh-cleanroom-dry-run-fc00`

Source input commit:
`0387d29f9282037637b4256c3c7f292bab7ef85c`

Fresh target commit:
`893198ce66a35c8aad007ad8ac7a61c4631c64d9`

Implementation evidence commit:
`ed768b281796887341cbed8eead4a80c05bbd09c`

## Result

The fresh target now proves the full SDK-style programmatic surface:

- create a synthetic `CharacterSheet`;
- project it through character-battle handoff;
- compose it with non-sheet participants, Encounter Side, Initiative count,
  stable Initiative order/current actor, and generic subject profiles;
- enter `BattleState`;
- discover and resolve a generic battle act through reducer entrypoints;
- apply damage, spend the action, and advance the turn.

The previous blocker
`SDK-tracer-handoff-battle-lacks-encounter-composition-surface` is closed.

## Source Inputs

- `cleanroom-input/qnt/character-battle-runtime/character-battle-encounter-composition.route.mbt.qnt`
- `cleanroom-input/qnt/character-battle-runtime/character-battle-reducer-route.qnt`
- `cleanroom-input/guidance/reducer-spine.md`
- `cleanroom-input/raw/srd-5.2.1/Playing-the-Game.md`
- `cleanroom-input/raw/srd-5.2.1/Rules-Glossary.md`
- `cleanroom-input/domain/UBIQUITOUS_LANGUAGE.md`

## Evidence

- `EVIDENCE/sdk-tracer-bullet-programmatic-surface.json` status is `accepted`.
- `BLOCKERS.json` has no SDK tracer blocker.
- `tests/sdk_tracer_bullet.rs` includes
  `sdk_can_project_sheet_compose_encounter_and_drive_one_battle_turn`.
- `tests/reducer_spine.rs` includes
  `character_battle_encounter_composition_matches_route_connector`.
- `STATE_OWNERS.md` records the durable state owners for participant
  membership, Encounter Side, Initiative count/order, current actor, subject
  profile availability, and route observations.

## Verification

- `cargo fmt --check`
- `cargo test --test sdk_tracer_bullet`
- `cargo test --test reducer_spine character_battle_encounter_composition_matches_route_connector`
- `python3 tools/verify_sdk_tracer_bullet.py`
- `cargo test`
- `cargo clippy --all-targets -- -D warnings`
- `git diff --check`

## Remaining Source Feedback

The encounter composition gap from `FC06_SOURCE_FEEDBACK.md` is resolved by the
source connector and this fresh target evidence. The Pact Slot handoff
route-surface gap from that report is also resolved in the copied source
package by source commit `b57772b459f1b75592fd45b9196fd60965b534d3`; this SDK
composition acceptance itself does not claim a fresh target replay for those
Pact Slot branches. The later FC-07 fresh target replay accepts those branches
at commit `f0ee8f8eb95192639afe5b6af17764dfe46c5303`.
