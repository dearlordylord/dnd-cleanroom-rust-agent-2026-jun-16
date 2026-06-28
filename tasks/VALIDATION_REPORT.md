# Validation Report

## SQNT-07A-SPATIAL-MOVEMENT-DIRTY-REPLAY

- Lane: `SQNT-07A-SPATIAL-MOVEMENT-DIRTY-REPLAY`
- Base SHA: `2645478`
- Source package commit: `e9f75e22a10891cd438fb06f6ea1ca666f79aaeb`
- Evidence file: `tasks/target-replay-evidence/SQNT-07A-spatial-movement-dirty-replay.json`
- Accepted rows: 16 route connector rows.
- Blocked rows: 2.
- Scope note: dirty target replay only; this is not fresh target acceptance.

Drivers and route connectors:

- `cleanroom-input/qnt/battle-runtime/battle-runtime-spatial-effects.route.mbt.qnt`
- `cleanroom-input/qnt/battle-runtime/battle-runtime-spatial-effect-route-facts.qnt`
- `cleanroom-input/qnt/battle-runtime/battle-runtime-movement-presentation.route.mbt.qnt`
- `cleanroom-input/qnt/battle-runtime/battle-runtime-movement-presentation-route-facts.qnt`

Accepted route rows:

- Spatial effects: `doAdmitMovableMultiEmitterLight`, `doMoveMovableMultiEmitterLight`, `doAdmitOutlineSightEffect`, `doProjectOutlineSightAttackAdvantage`, `doAdmitAreaObscurement`, `doCleanupAreaObscurementByDuration`, `doDisperseAreaObscurementByStrongWind`, `doAdmitAreaHazard`, `doResolveAreaHazardSavingThrowTrigger`, `doResolveAreaHazardDifficultTerrainMovement`, `doResolveAreaHazardMovementDamageTrigger`, `doCleanupAreaHazard`, `doRecordTableOwnedSpatialWitnesses`.
- Movement presentation: `doRouteMovementReplacementLandingWitness`, `doRouteForcedCreatureMovementPresentation`, `doRouteObjectPushPresentation`.

Blocked rows preserved:

- `doJumpMovementReplacementLandingWitness`: movement replacement budget/distance and table landing presentation are accepted, but landing legality and failed-landing Prone consequence are not supported by copied route facts.
- Concentration-backed Sleet Storm / Spike Growth / Web-like area hazards: copied spatial route facts cover non-Concentration area hazard lifecycle plus separate movement-damage trigger, not concentration-backed hazard rows.

Behavior implemented:

- Added battle-owned route-surface state for generic spatial effects and movement presentation.
- Added public reducer subject/owner/fill/hole shapes for spatial-effect and movement-presentation route connectors.
- Added focused adapters/tests that obtain observed routes only through public battle start/discover/resolve entrypoints.
- Kept selected spell identity out of production dispatch; adapter action names are copied QNT row names only.

Verification results:

- `git merge-base --is-ancestor 2645478 HEAD` passed.
- `cargo test spatial_effects_dirty_replay_routes_through_reducer` passed.
- `cargo test movement_presentation_dirty_replay_routes_through_reducer` passed.
- `cargo fmt --check` passed.
- `cargo test` passed.
- `cargo clippy --all-targets -- -D warnings` passed.
- `git diff --check` passed.
- `node scripts/check-target-replay-evidence-file.cjs --driver cleanroom-input/qnt/battle-runtime/battle-runtime-spatial-effects.route.mbt.qnt --evidence tasks/target-replay-evidence/SQNT-07A-spatial-movement-dirty-replay.json` failed because route connector rows are not source-branch-inventory obligations; all 16 connector runs are reported as unknown source obligations.
- `node scripts/check-target-replay-evidence-file.cjs --driver cleanroom-input/qnt/battle-runtime/battle-runtime-movement-presentation.route.mbt.qnt --evidence tasks/target-replay-evidence/SQNT-07A-spatial-movement-dirty-replay.json` failed for the same source-inventory limitation.
- `node scripts/check-cleanroom-harness.cjs` failed on existing stale ledger/evidence debt under the refreshed manifest, plus the known broad harness scan; the lane-specific focused Rust tests and JSON syntax checks pass.
