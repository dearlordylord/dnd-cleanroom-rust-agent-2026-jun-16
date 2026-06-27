# Blockers

- L15-RR07-FU01A-CATALOG-READY-SPELL-SUBSTRATES: no accepted-branch blockers. Find Familiar rows remain accepted after moving observed route evidence to production `BattleReducerRouteTrace` emission; no Find Familiar rows were demoted. Level-2/3 condition-save branches are intentionally out of scope for this lane: doResolveBlindnessDeafnessBlindedSavingThrow, doResolveBlindnessDeafnessDeafenedSavingThrow, doResolveHoldPersonFailedSavingThrow, doResolveHoldPersonRepeatSavingThrowSuccess, doResolveHypnoticPatternFailedSavingThrow.
- L15-RR07-FU01B-SPELL-ATTACK-SAVE-DAMAGE-SUBSTRATES: accepted attack/save damage rows are routed through production `BattleState` spell attack or save-gated reducer entrypoints and observed reducer-route events. The blocked rows are executable target blockers in `src/qnt_adapters/battle_runtime_level1_damage_spell_selected_identity.rs`: observed replay, observed route, and expected witness entrypoints reject them with blocker reasons, and `tasks/target-replay-evidence/L15-RR07-FU01B-SPELL-ATTACK-SAVE-DAMAGE-SUBSTRATES.json` records `target-blocker` runs rather than pass refs.
  - `doResolveChromaticOrbDuplicateDamageLeap`: chained duplicate-damage leap facts are modeled in `ChromaticOrbSequenceState`, not production `BattleState`; accepting this row would require a production chained/object-target route subject.
  - `doResolveStarryWispObjectSpellAttackDamageAndDimLight`: object HP and dim-light emitter facts are modeled in `StarryWispObjectState`, not production `BattleState`; accepting this row would require production object-target damage/light state.
- L15-RR07-FU01F-SPATIAL-LIGHT-AREA-MOVEMENT-SUBSTRATES: no accepted-branch blockers. All ten spatial witness rows route through generic `BattleSubjectKind::Spatial` values carried by `BattleState.spatial_route_subjects` and observed through public `start_battle_observed`, `discover_battle_acts_observed`, and `resolve_battle_subject_observed` entrypoints. No row is accepted through authored spell identity or adapter-local observed-route replay.

## L15-RR07-FU01C-WEAPON-BUFF-MARK-SMITE-SUBSTRATES

- doHeroismFrightenedImmunityTurnStartTemporaryHitPoints: condition-immunity plus turn-start temporary Hit Points needs a generic active-effect route subject.
- doHeroismFrightenedImmunityTurnStartTemporaryHitPointsCleanup: condition-immunity cleanup needs a generic active-effect route subject.
- doHuntersMarkMarkedDamageRiderConcentrationAndSameTurnTransfer: marked-target damage rider plus transfer needs a generic marked-effect route subject.
- doHexMarkedDamageRiderAndLaterTurnTransfer: marked-target damage rider plus transfer needs a generic marked-effect route subject.

## L15-RRCP5-B-ACTIVE-EFFECT-LIFECYCLE-ROUTES

- No target blockers. All selected CP5 active-effect lifecycle route rows are covered by tasks/target-replay-evidence/L15-RRCP5-B-active-effect-lifecycle-routes.json.
