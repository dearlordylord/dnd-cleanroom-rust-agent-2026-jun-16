# Dirty Rehearsal Status

This repo is being used as the dirty reducer-spine rehearsal target. It is not
a clean evidentiary run: the worktree had prior implementation and replay
evidence before this batch, and the historical ledger/artifact model remains
incomplete.

- Current manifest source commit SHA: `0e024f089687526c6bc4b9e6f9c7e640414f6486`
- Current source branch inventory SHA: `47b0589f442c0aaff2a814c19384fcaed7a6dbe3e7a78b5d0df8b011f56e7eae`
- Current reducer-route inventory SHA: `f19710f4cbc4d09208d38568c04375f0df4e2a738bbadc76c19f0f7161ffd733`
- Source guidance commits used:
  - `80fd9f9b1a65ebd18142fd62ac920af6f914fc3b` reducer-spine routing package
  - `0a9f139b2af181db3bdfb1879b92b7d2fb942ffd` identity-dispatch boundary guidance
  - `665a9b4ad3cc11c8c16f92126b2a2567355cbcc9` QNT reducer-route connectors
  - `0e024f089687526c6bc4b9e6f9c7e640414f6486` level-5 reducer-route package
- Active assignment: `reducer-spine-diagnostic-battle`
- Diagnostic queue length: 5 drivers

## Reducer-Spine Diagnostic Queue

| Order | Driver | Current dirty-target status |
| ---: | --- | --- |
| 1 | `cleanroom-input/qnt/battle-runtime/battle-runtime-magic-missile.mbt.qnt` | Observed adapter replay matches `battle-runtime-magic-missile.route.mbt.qnt` route events through `discover_battle_acts` and `resolve_battle_subject` with battle-owned slot-spell state. |
| 2 | `cleanroom-input/qnt/battle-runtime/battle-runtime-save-gated-spell-ordering.mbt.qnt` | Observed adapter replay matches `battle-runtime-save-gated-spell-ordering.route.mbt.qnt` route events for area damage and target-list/condition paths through the shared reducer. |
| 3 | `cleanroom-input/qnt/battle-runtime/battle-runtime-hit-point-restoration-ordering.mbt.qnt` | Observed adapter replay matches `battle-runtime-hit-point-restoration-ordering.route.mbt.qnt` route events for single-target spell, target-list spell, and feature healing pool through the shared reducer. |
| 4 | `cleanroom-input/qnt/battle-runtime/battle-runtime-death-saving-throw.mbt.qnt` | Observed adapter replay now matches `battle-runtime-death-saving-throw.route.mbt.qnt` route events through `discover_battle_acts` and `resolve_battle_subject` with battle-owned Hit Point, Stable, Dead, and death-save counter state. |
| 5 | `cleanroom-input/qnt/battle-runtime/battle-runtime-concentration-break-teardown.mbt.qnt` | Observed adapter replay now matches `battle-runtime-concentration-break-teardown.route.mbt.qnt` route events through `discover_battle_acts`, fill-less subject resolution, damage-save request, and saving-throw fill using a battle-owned Concentration owner. |

The earlier "three additional focused battle drivers" threshold was a success
criterion from the source plan, not the count of available drivers. The current
reducer-spine diagnostic queue has five drivers. The broader level-1/2 battle
lane has 65 battle drivers in `tasks/ACTIVE_WORK.json`.

## Boundary Correction

The copied guidance now records the cleanroom rule that reading unapproved
source code and dispatching production reducer behavior on authored or fixture
identity are the same cleanroom-boundary violation class. Production reducer
behavior must route by runtime shape, typed facts, capabilities, procedure
state, and battle-owned state. Fixture names and QNT action names belong in
adapters, tests, evidence, catalog/selection boundaries, or documented
support-profile admission.

Target code was adjusted for the called-out stat-block helper family: the
production reducer no longer exposes named-creature stat-block control helpers
such as `start_goblin_multiattack_control`. The adapter supplies fixture actor
handles and profile facts; production functions route on the current actor,
combatant state, subject, and profile shape.

Remaining dirty-target identity debt still exists outside this focused
correction, especially because this target's existing `Actor` enum and many
older adapters use fixture names. That is not accepted as cleanroom evidence;
it is tolerated here only because this is a dirty rehearsal target.

## State-Owner Notes

- `CombatantLifecycle::UsesDeathSavingThrows` owns the dirty target's
  battle-owned zero-Hit-Point lifecycle facts for the Death Saving Throw
  diagnostic route. Active death-save success/failure counts, Stable, and Dead
  are separate variants, so stable-and-dead or stable-with-counter states cannot
  be represented. Combatants with `DiesAtZeroHitPoints` cannot carry those
  facts, and the zero-Hit-Point helper does not rewrite monster lifecycles.
- `BattleState.save_gated_spell_procedure` is the dirty target's battle-owned
  route owner for save-gated spell area-damage and target-list/condition-choice
  ordering.
- `BattleState.hit_point_restoration_procedure` is the dirty target's
  battle-owned route owner for single-target spell restoration, target-list
  spell restoration, and feature healing-pool ordering. Its stored subject uses
  a mode-specific targeting variant so spell and feature targets cannot coexist.
  Discovery and resolution both require a death-save-capable zero-Hit-Point
  target; RAW-dead monsters with `DiesAtZeroHitPoints` are not ordinary
  restoration targets.
- `BattleState.concentration` is the dirty target's battle-owned Concentration
  teardown owner for concentration cast, damage-save request, failed save,
  voluntary end, and replacement Concentration paths.
- Slot-spell, save-gated spell, Hit Point restoration, and Concentration
  teardown diagnostic subjects now reject stale caller-supplied subjects when
  the subject is no longer discoverable from the current state or, for
  procedure-backed routes, no longer matches active procedure state.
- The adapter-owned route vocabulary remains quarantined under
  `src/qnt_adapters/battle_runtime_reducer_route.rs`.

## Verification

- `cargo fmt --check` passed.
- `cargo test stale_action_unavailable` passed: focused stale diagnostic subject
  coverage now includes Concentration teardown `CastSpell` and
  `CastReplacementSpell` when action availability is gone.
- `cargo test concentration_break_teardown_adapter_replays_all_branches` passed:
  valid Concentration teardown route subjects still replay through the
  connector surface.
- `cargo test adapter_replays_all_branches` passed: 70 filtered adapter tests with route checks for all five diagnostic drivers.
- `cargo test` passed: 173 tests.
- `cargo clippy --all-targets -- -D warnings` passed.
- `node scripts/check-cleanroom-harness.cjs` failed.

Harness failure is repository-artifact debt, not a failing Rust reducer test.
For manifest source SHA `0e024f089687526c6bc4b9e6f9c7e640414f6486`, the
main blockers reported were:

- missing `tasks/RUN_LEDGER.json`;
- `tasks/LEVEL_1_2_SCOPE.md` mismatch with the source-owned snapshot;
- stale or undeclared historical `tasks/target-replay-evidence/*.json`;
- selected reducer-spine diagnostic evidence not current for manifest
  `0e024f089687526c6bc4b9e6f9c7e640414f6486`;
- broad pre-existing adapter quarantine/static-gate findings in older dirty
  target files;
- `tasks/VALIDATION_REPORT.md` missing current manifest/source-inventory rows
  and selected reducer-spine-contract covered rows.

Current branch coverage evidence JSON for the five routed drivers is still
stale relative to the manifest. The passing adapter tests prove dirty target
behavior through the copied route connector surfaces, but accepted cleanroom
evidence still requires regenerated replay evidence, ledger entries, history
artifacts, and validation report rows.

## Next Useful Step

For this dirty rehearsal, the next useful source-planning step is to treat the
five-driver route surface as proven by dirty target tests and decide whether
Task 14 should require fresh evidence regeneration from a clean target rather
than trying to repair this target's stale historical ledger.
