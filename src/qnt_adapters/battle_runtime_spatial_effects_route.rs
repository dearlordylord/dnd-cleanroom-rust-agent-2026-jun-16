use crate::rules::battle_reducer_spine::{
    discover_generic_route_subject_observed, generic_route_subject_for_current_actor,
    resolve_battle_subject_observed, start_battle_observed, BattleEntrypointTrace,
    BattleGenericRouteFill, BattleResolutionRequest, BattleSetup, BattleSpatialEffectRouteState,
    BattleState, BattleSubjectKind,
};

use super::battle_runtime_reducer_route::{
    observed_reducer_route, route_discover_battle_acts_from_route_holes,
    route_resolve_battle_subject_from_route_result,
    route_resolve_battle_subject_without_fill_from_route_result, route_start_battle,
    ReducerRouteEvent, ReducerRouteFillKind, ReducerRouteHoleKind, ReducerRouteOwnerGroup,
    ReducerRouteResolutionOutcome, ReducerRouteSubjectFamily,
};

pub const CONNECTOR_ACTIONS: [&str; 13] = [
    "doAdmitMovableMultiEmitterLight",
    "doMoveMovableMultiEmitterLight",
    "doAdmitOutlineSightEffect",
    "doProjectOutlineSightAttackAdvantage",
    "doAdmitAreaObscurement",
    "doCleanupAreaObscurementByDuration",
    "doDisperseAreaObscurementByStrongWind",
    "doAdmitAreaHazard",
    "doResolveAreaHazardSavingThrowTrigger",
    "doResolveAreaHazardDifficultTerrainMovement",
    "doResolveAreaHazardMovementDamageTrigger",
    "doCleanupAreaHazard",
    "doRecordTableOwnedSpatialWitnesses",
];

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SpatialEffectRouteFact {
    BattleEffect(SpatialEffectBattleEffectFact),
    Light(SpatialEffectLightFact),
    Geometry(SpatialEffectGeometryFact),
    Presentation(SpatialEffectPresentationFact),
    Object(SpatialEffectObjectFact),
    Sight(SpatialEffectSightFact),
    Hazard(SpatialEffectHazardFact),
    CleanedUpBy {
        cleanup: SpatialEffectCleanupFact,
        owner: SpatialEffectCleanupOwnerFact,
    },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SpatialEffectBattleEffectFact {
    MovableMultiEmitterEffectAdmitted,
    OutlineEffectAdmitted,
    AreaObscurementEffectAdmitted,
    AreaHazardEffectAdmitted,
    ConcentrationBackedEffect,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SpatialEffectLightFact {
    MultiEmitterDimLightProjection,
    EmitterPositionMoved,
    DimLightProjection,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[allow(clippy::enum_variant_names)]
pub enum SpatialEffectGeometryFact {
    AreaShapeWitness,
    AreaMembershipWitness,
    MovementPathWitness,
    TotalCoverBlockerWitness,
    StrongWindWitness,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[allow(clippy::enum_variant_names)]
pub enum SpatialEffectPresentationFact {
    EmitterAppearancePresentation,
    ColorChoicePresentation,
    VisibleOutlinePresentation,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SpatialEffectObjectFact {
    OutlinedObjectWitness,
    ObjectSightTargetWitness,
    ObjectInvisibleBenefitDeniedProjection,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SpatialEffectSightFact {
    AttackerCanSeeTargetWitness,
    InvisibleBenefitDeniedProjection,
    AttackRollAdvantageProjection,
    HeavilyObscuredProjection,
    LightlyObscuredProjection,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SpatialEffectHazardFact {
    DifficultTerrainProjection,
    EntrySavingThrowTrigger,
    StartTurnSavingThrowTrigger,
    MovementDamageTrigger,
    PerTurnTriggerLimit,
    AreaRemovedCleanup,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SpatialEffectCleanupFact {
    DurationCleanup,
    DispersalCleanup,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[allow(clippy::enum_variant_names)]
pub enum SpatialEffectCleanupOwnerFact {
    BattleActiveEffectCleanupOwner,
    BattleConcentrationCleanupOwner,
    BattleObscurementCleanupOwner,
    BattleAreaHazardCleanupOwner,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SpatialEffectRouteWitness {
    pub route_state: BattleSpatialEffectRouteState,
    pub facts: Vec<SpatialEffectRouteFact>,
}

pub fn replay_observed_action(observed_action_taken: &str) -> SpatialEffectRouteWitness {
    replay_observed_state_and_route(observed_action_taken).0
}

pub fn replay_observed_route(observed_action_taken: &str) -> Vec<ReducerRouteEvent> {
    replay_observed_state_and_route(observed_action_taken).1
}

pub fn expected_witness(observed_action_taken: &str) -> SpatialEffectRouteWitness {
    expected_for_action(observed_action_taken).0
}

pub fn expected_route(observed_action_taken: &str) -> Vec<ReducerRouteEvent> {
    expected_for_action(observed_action_taken).1
}

pub fn projection_payload(witness: &SpatialEffectRouteWitness) -> String {
    [
        format!("routeState={}", route_state_ref(witness.route_state)),
        format!("facts={}", facts_payload(&witness.facts)),
    ]
    .join("\n")
}

fn replay_observed_state_and_route(
    observed_action_taken: &str,
) -> (SpatialEffectRouteWitness, Vec<ReducerRouteEvent>) {
    let mut trace = BattleEntrypointTrace::default();
    let state = start_battle_observed(BattleSetup::standard(), &mut trace).state;
    let state = match observed_action_taken {
        "doAdmitMovableMultiEmitterLight" => admit_movable_light(state, &mut trace),
        "doMoveMovableMultiEmitterLight" => {
            let state = admit_movable_light(state, &mut trace);
            move_movable_light(state, &mut trace)
        }
        "doAdmitOutlineSightEffect" => admit_outline(state, &mut trace),
        "doProjectOutlineSightAttackAdvantage" => {
            let state = admit_outline(state, &mut trace);
            project_outline(state, &mut trace)
        }
        "doAdmitAreaObscurement" => admit_area_obscurement(state, &mut trace),
        "doCleanupAreaObscurementByDuration" => {
            let state = admit_area_obscurement(state, &mut trace);
            cleanup_area_obscurement_duration(state, &mut trace)
        }
        "doDisperseAreaObscurementByStrongWind" => {
            let state = admit_area_obscurement(state, &mut trace);
            cleanup_area_obscurement_dispersal(state, &mut trace)
        }
        "doAdmitAreaHazard" => admit_area_hazard(state, &mut trace),
        "doResolveAreaHazardSavingThrowTrigger" => {
            let state = admit_area_hazard(state, &mut trace);
            resolve_hazard_save(state, &mut trace)
        }
        "doResolveAreaHazardDifficultTerrainMovement" => {
            let state = admit_area_hazard(state, &mut trace);
            resolve_hazard_difficult_movement(state, &mut trace)
        }
        "doResolveAreaHazardMovementDamageTrigger" => {
            let state = admit_area_hazard(state, &mut trace);
            resolve_hazard_movement_damage(state, &mut trace)
        }
        "doCleanupAreaHazard" => {
            let state = admit_area_hazard(state, &mut trace);
            cleanup_area_hazard(state, &mut trace)
        }
        "doRecordTableOwnedSpatialWitnesses" => record_table_witnesses(state, &mut trace),
        action => panic!("unsupported spatial-effects route action {action}"),
    };
    (
        SpatialEffectRouteWitness {
            route_state: state.spatial_effect_route,
            facts: expected_facts(observed_action_taken),
        },
        observed_reducer_route(&trace, &[ReducerRouteSubjectFamily::SpatialEffectRoute]),
    )
}

fn admit_movable_light(state: BattleState, trace: &mut BattleEntrypointTrace) -> BattleState {
    let state = discover_then_resolve_without_fill(
        state,
        BattleSubjectKind::SpatialEffectMovableMultiEmitterAdmission,
        trace,
    );
    let state = resolve_without_fill(
        state,
        BattleSubjectKind::SpatialEffectMovableMultiEmitterConcentration,
        trace,
    );
    resolve_without_fill(
        state,
        BattleSubjectKind::SpatialEffectMovableMultiEmitterLightProjection,
        trace,
    )
}

fn move_movable_light(state: BattleState, trace: &mut BattleEntrypointTrace) -> BattleState {
    let state = discover_then_resolve(
        state,
        BattleSubjectKind::SpatialEffectMovableMultiEmitterMoveTargetChoice,
        BattleGenericRouteFill::TargetChoice,
        trace,
    );
    resolve_without_fill(
        state,
        BattleSubjectKind::SpatialEffectMovableMultiEmitterMoveLightProjection,
        trace,
    )
}

fn admit_outline(state: BattleState, trace: &mut BattleEntrypointTrace) -> BattleState {
    let state = discover_then_resolve(
        state,
        BattleSubjectKind::SpatialEffectOutlineTargetChoice,
        BattleGenericRouteFill::TargetChoice,
        trace,
    );
    let state = resolve(
        state,
        BattleSubjectKind::SpatialEffectOutlineSavingThrowOutcome,
        BattleGenericRouteFill::SavingThrowOutcome,
        trace,
    );
    let state = resolve_without_fill(
        state,
        BattleSubjectKind::SpatialEffectOutlineActiveEffect,
        trace,
    );
    resolve_without_fill(
        state,
        BattleSubjectKind::SpatialEffectOutlineConcentration,
        trace,
    )
}

fn project_outline(state: BattleState, trace: &mut BattleEntrypointTrace) -> BattleState {
    let state = resolve_without_fill(
        state,
        BattleSubjectKind::SpatialEffectOutlineLightProjection,
        trace,
    );
    let state = resolve_without_fill(
        state,
        BattleSubjectKind::SpatialEffectOutlineSightProjection,
        trace,
    );
    resolve_without_fill(
        state,
        BattleSubjectKind::SpatialEffectOutlineAttackRollModeProjection,
        trace,
    )
}

fn admit_area_obscurement(state: BattleState, trace: &mut BattleEntrypointTrace) -> BattleState {
    let state = discover_then_resolve(
        state,
        BattleSubjectKind::SpatialEffectAreaObscurementTargetChoice,
        BattleGenericRouteFill::TargetChoice,
        trace,
    );
    let state = resolve_without_fill(
        state,
        BattleSubjectKind::SpatialEffectAreaObscurementActiveEffect,
        trace,
    );
    let state = resolve_without_fill(
        state,
        BattleSubjectKind::SpatialEffectAreaObscurementConcentration,
        trace,
    );
    let state = resolve_without_fill(
        state,
        BattleSubjectKind::SpatialEffectAreaObscurementProjection,
        trace,
    );
    resolve_without_fill(
        state,
        BattleSubjectKind::SpatialEffectAreaObscurementSightProjection,
        trace,
    )
}

fn cleanup_area_obscurement_duration(
    state: BattleState,
    trace: &mut BattleEntrypointTrace,
) -> BattleState {
    let state = resolve_without_fill(
        state,
        BattleSubjectKind::SpatialEffectAreaObscurementDurationTurnBoundary,
        trace,
    );
    let state = resolve_without_fill(
        state,
        BattleSubjectKind::SpatialEffectAreaObscurementDurationProjectionCleanup,
        trace,
    );
    let state = resolve_without_fill(
        state,
        BattleSubjectKind::SpatialEffectAreaObscurementDurationActiveEffectCleanup,
        trace,
    );
    resolve_without_fill(
        state,
        BattleSubjectKind::SpatialEffectAreaObscurementDurationConcentrationCleanup,
        trace,
    )
}

fn cleanup_area_obscurement_dispersal(
    state: BattleState,
    trace: &mut BattleEntrypointTrace,
) -> BattleState {
    let state = resolve_without_fill(
        state,
        BattleSubjectKind::SpatialEffectAreaObscurementDispersalProjectionCleanup,
        trace,
    );
    let state = resolve_without_fill(
        state,
        BattleSubjectKind::SpatialEffectAreaObscurementDispersalActiveEffectCleanup,
        trace,
    );
    resolve_without_fill(
        state,
        BattleSubjectKind::SpatialEffectAreaObscurementDispersalConcentrationCleanup,
        trace,
    )
}

fn admit_area_hazard(state: BattleState, trace: &mut BattleEntrypointTrace) -> BattleState {
    let state = discover_then_resolve(
        state,
        BattleSubjectKind::SpatialEffectAreaHazardTargetChoice,
        BattleGenericRouteFill::TargetChoice,
        trace,
    );
    let state = resolve_without_fill(
        state,
        BattleSubjectKind::SpatialEffectAreaHazardActiveEffect,
        trace,
    );
    let state = resolve_without_fill(
        state,
        BattleSubjectKind::SpatialEffectAreaHazardProjection,
        trace,
    );
    resolve_without_fill(
        state,
        BattleSubjectKind::SpatialEffectAreaHazardCreatureSpaceMovement,
        trace,
    )
}

fn resolve_hazard_save(state: BattleState, trace: &mut BattleEntrypointTrace) -> BattleState {
    let state = discover_then_resolve(
        state,
        BattleSubjectKind::SpatialEffectAreaHazardSavingThrow,
        BattleGenericRouteFill::SavingThrowOutcome,
        trace,
    );
    resolve_without_fill(
        state,
        BattleSubjectKind::SpatialEffectAreaHazardConditionLifecycle,
        trace,
    )
}

fn resolve_hazard_difficult_movement(
    state: BattleState,
    trace: &mut BattleEntrypointTrace,
) -> BattleState {
    discover_then_resolve(
        state,
        BattleSubjectKind::SpatialEffectAreaHazardDifficultTerrainMovement,
        BattleGenericRouteFill::Movement { accepted: true },
        trace,
    )
}

fn resolve_hazard_movement_damage(
    state: BattleState,
    trace: &mut BattleEntrypointTrace,
) -> BattleState {
    let state = discover_then_resolve(
        state,
        BattleSubjectKind::SpatialEffectAreaHazardMovementDamageMovement,
        BattleGenericRouteFill::Movement { accepted: true },
        trace,
    );
    resolve(
        state,
        BattleSubjectKind::SpatialEffectAreaHazardMovementDamageHitPoint,
        BattleGenericRouteFill::RolledDice,
        trace,
    )
}

fn cleanup_area_hazard(state: BattleState, trace: &mut BattleEntrypointTrace) -> BattleState {
    let state = resolve_without_fill(
        state,
        BattleSubjectKind::SpatialEffectAreaHazardCleanupTurnBoundary,
        trace,
    );
    let state = resolve_without_fill(
        state,
        BattleSubjectKind::SpatialEffectAreaHazardCleanupHazard,
        trace,
    );
    resolve_without_fill(
        state,
        BattleSubjectKind::SpatialEffectAreaHazardCleanupActiveEffect,
        trace,
    )
}

fn record_table_witnesses(state: BattleState, trace: &mut BattleEntrypointTrace) -> BattleState {
    let state = resolve_without_fill(
        state,
        BattleSubjectKind::SpatialEffectTableAreaShapeWitness,
        trace,
    );
    let state = resolve_without_fill(
        state,
        BattleSubjectKind::SpatialEffectTableSightWitness,
        trace,
    );
    resolve_without_fill(
        state,
        BattleSubjectKind::SpatialEffectTableAreaHazardWitness,
        trace,
    )
}

fn discover_then_resolve_without_fill(
    state: BattleState,
    kind: BattleSubjectKind,
    trace: &mut BattleEntrypointTrace,
) -> BattleState {
    let (state, subject) = discover_generic_route_subject_observed(state, kind, trace);
    resolve_subject(state, subject, BattleGenericRouteFill::WithoutFill, trace)
}

fn discover_then_resolve(
    state: BattleState,
    kind: BattleSubjectKind,
    fill: BattleGenericRouteFill,
    trace: &mut BattleEntrypointTrace,
) -> BattleState {
    let (state, subject) = discover_generic_route_subject_observed(state, kind, trace);
    resolve_subject(state, subject, fill, trace)
}

fn resolve_without_fill(
    state: BattleState,
    kind: BattleSubjectKind,
    trace: &mut BattleEntrypointTrace,
) -> BattleState {
    resolve(state, kind, BattleGenericRouteFill::WithoutFill, trace)
}

fn resolve(
    state: BattleState,
    kind: BattleSubjectKind,
    fill: BattleGenericRouteFill,
    trace: &mut BattleEntrypointTrace,
) -> BattleState {
    let subject = generic_route_subject_for_current_actor(&state, kind);
    resolve_subject(state, subject, fill, trace)
}

fn resolve_subject(
    state: BattleState,
    subject: crate::rules::battle_reducer_spine::BattleSubject,
    fill: BattleGenericRouteFill,
    trace: &mut BattleEntrypointTrace,
) -> BattleState {
    resolve_battle_subject_observed(
        state,
        BattleResolutionRequest::generic_route(subject, fill)
            .expect("spatial route subject should accept generic route fill"),
        trace,
    )
    .into_state()
}

fn expected_for_action(action: &str) -> (SpatialEffectRouteWitness, Vec<ReducerRouteEvent>) {
    let mut route = vec![route_start_battle(ReducerRouteOwnerGroup::ActionEconomy)];
    let state = match action {
        "doAdmitMovableMultiEmitterLight" => {
            route.extend(effect_admission_route());
            route.extend(concentration_route());
            route.extend(light_projection_route());
            BattleSpatialEffectRouteState::MovableMultiEmitterLightActive
        }
        "doMoveMovableMultiEmitterLight" => {
            route.extend(effect_admission_route());
            route.extend(concentration_route());
            route.extend(light_projection_route());
            route.extend(move_emitter_route());
            BattleSpatialEffectRouteState::MovableMultiEmitterLightMoved
        }
        "doAdmitOutlineSightEffect" => {
            route.extend(outline_admission_route());
            route.extend(concentration_route());
            BattleSpatialEffectRouteState::OutlineSightEffectActive
        }
        "doProjectOutlineSightAttackAdvantage" => {
            route.extend(outline_admission_route());
            route.extend(concentration_route());
            route.extend(outline_projection_route());
            BattleSpatialEffectRouteState::OutlineSightEffectProjected
        }
        "doAdmitAreaObscurement" => {
            route.extend(area_admission_route());
            route.extend(concentration_route());
            route.extend(obscurement_projection_route());
            BattleSpatialEffectRouteState::AreaObscurementActive
        }
        "doCleanupAreaObscurementByDuration" => {
            route.extend(area_admission_route());
            route.extend(concentration_route());
            route.extend(obscurement_projection_route());
            route.extend(obscurement_duration_cleanup_route());
            BattleSpatialEffectRouteState::AreaObscurementDurationCleaned
        }
        "doDisperseAreaObscurementByStrongWind" => {
            route.extend(area_admission_route());
            route.extend(concentration_route());
            route.extend(obscurement_projection_route());
            route.extend(obscurement_dispersal_cleanup_route());
            BattleSpatialEffectRouteState::AreaObscurementDispersed
        }
        "doAdmitAreaHazard" => {
            route.extend(area_admission_route());
            route.extend(hazard_projection_route());
            BattleSpatialEffectRouteState::AreaHazardActive
        }
        "doResolveAreaHazardSavingThrowTrigger" => {
            route.extend(area_admission_route());
            route.extend(hazard_projection_route());
            route.extend(hazard_saving_throw_route());
            BattleSpatialEffectRouteState::AreaHazardSaveResolved
        }
        "doResolveAreaHazardDifficultTerrainMovement" => {
            route.extend(area_admission_route());
            route.extend(hazard_projection_route());
            route.extend(hazard_difficult_terrain_movement_route());
            BattleSpatialEffectRouteState::AreaHazardDifficultTerrainMovementResolved
        }
        "doResolveAreaHazardMovementDamageTrigger" => {
            route.extend(area_admission_route());
            route.extend(hazard_projection_route());
            route.extend(hazard_movement_damage_route());
            BattleSpatialEffectRouteState::AreaHazardMovementDamageResolved
        }
        "doCleanupAreaHazard" => {
            route.extend(area_admission_route());
            route.extend(hazard_projection_route());
            route.extend(hazard_cleanup_route());
            BattleSpatialEffectRouteState::AreaHazardCleaned
        }
        "doRecordTableOwnedSpatialWitnesses" => {
            route.extend(table_spatial_witness_route());
            BattleSpatialEffectRouteState::TableSpatialWitnessRecorded
        }
        action => panic!("unsupported expected spatial-effects route action {action}"),
    };
    (
        SpatialEffectRouteWitness {
            route_state: state,
            facts: expected_facts(action),
        },
        route,
    )
}

fn effect_admission_route() -> Vec<ReducerRouteEvent> {
    vec![
        discover(
            Vec::new(),
            ReducerRouteOwnerGroup::SpellSlotAndActionEconomy,
        ),
        resolve_without_fill_event(ReducerRouteOwnerGroup::ActiveEffect),
    ]
}

fn concentration_route() -> Vec<ReducerRouteEvent> {
    vec![resolve_without_fill_event(
        ReducerRouteOwnerGroup::Concentration,
    )]
}

fn light_projection_route() -> Vec<ReducerRouteEvent> {
    vec![resolve_without_fill_event(
        ReducerRouteOwnerGroup::LightProjection,
    )]
}

fn move_emitter_route() -> Vec<ReducerRouteEvent> {
    vec![
        discover(
            vec![ReducerRouteHoleKind::TargetChoice],
            ReducerRouteOwnerGroup::SpellSlotAndActionEconomy,
        ),
        resolve_with_fill(
            ReducerRouteFillKind::TargetChoice,
            Vec::new(),
            ReducerRouteOwnerGroup::AreaShape,
        ),
        resolve_without_fill_event(ReducerRouteOwnerGroup::LightProjection),
    ]
}

fn outline_admission_route() -> Vec<ReducerRouteEvent> {
    vec![
        discover(
            vec![
                ReducerRouteHoleKind::SavingThrowOutcome,
                ReducerRouteHoleKind::TargetChoice,
            ],
            ReducerRouteOwnerGroup::SpellSlotAndActionEconomy,
        ),
        resolve_with_fill(
            ReducerRouteFillKind::TargetChoice,
            vec![ReducerRouteHoleKind::SavingThrowOutcome],
            ReducerRouteOwnerGroup::AreaShape,
        ),
        resolve_with_fill(
            ReducerRouteFillKind::SavingThrowOutcome,
            Vec::new(),
            ReducerRouteOwnerGroup::SavingThrowOutcome,
        ),
        resolve_without_fill_event(ReducerRouteOwnerGroup::ActiveEffect),
    ]
}

fn outline_projection_route() -> Vec<ReducerRouteEvent> {
    vec![
        resolve_without_fill_event(ReducerRouteOwnerGroup::LightProjection),
        resolve_without_fill_event(ReducerRouteOwnerGroup::SightProjection),
        resolve_without_fill_event(ReducerRouteOwnerGroup::AttackRollMode),
    ]
}

fn area_admission_route() -> Vec<ReducerRouteEvent> {
    vec![
        discover(
            vec![ReducerRouteHoleKind::TargetChoice],
            ReducerRouteOwnerGroup::SpellSlotAndActionEconomy,
        ),
        resolve_with_fill(
            ReducerRouteFillKind::TargetChoice,
            Vec::new(),
            ReducerRouteOwnerGroup::AreaShape,
        ),
        resolve_without_fill_event(ReducerRouteOwnerGroup::ActiveEffect),
    ]
}

fn obscurement_projection_route() -> Vec<ReducerRouteEvent> {
    vec![
        resolve_without_fill_event(ReducerRouteOwnerGroup::ObscurementProjection),
        resolve_without_fill_event(ReducerRouteOwnerGroup::SightProjection),
    ]
}

fn hazard_projection_route() -> Vec<ReducerRouteEvent> {
    vec![
        resolve_without_fill_event(ReducerRouteOwnerGroup::AreaHazard),
        resolve_without_fill_event(ReducerRouteOwnerGroup::CreatureSpaceMovement),
    ]
}

fn hazard_saving_throw_route() -> Vec<ReducerRouteEvent> {
    vec![
        discover(
            vec![ReducerRouteHoleKind::SavingThrowOutcome],
            ReducerRouteOwnerGroup::AreaHazard,
        ),
        resolve_with_fill(
            ReducerRouteFillKind::SavingThrowOutcome,
            Vec::new(),
            ReducerRouteOwnerGroup::SavingThrowOutcome,
        ),
        resolve_without_fill_event(ReducerRouteOwnerGroup::ConditionLifecycle),
    ]
}

fn hazard_difficult_terrain_movement_route() -> Vec<ReducerRouteEvent> {
    vec![
        discover(
            vec![ReducerRouteHoleKind::Movement],
            ReducerRouteOwnerGroup::AreaHazard,
        ),
        resolve_with_fill(
            ReducerRouteFillKind::Movement,
            Vec::new(),
            ReducerRouteOwnerGroup::MovementResource,
        ),
    ]
}

fn hazard_movement_damage_route() -> Vec<ReducerRouteEvent> {
    vec![
        discover(
            vec![ReducerRouteHoleKind::Movement],
            ReducerRouteOwnerGroup::AreaHazard,
        ),
        resolve_with_fill(
            ReducerRouteFillKind::Movement,
            vec![ReducerRouteHoleKind::RolledDice],
            ReducerRouteOwnerGroup::MovementResource,
        ),
        resolve_with_fill(
            ReducerRouteFillKind::RolledDice,
            Vec::new(),
            ReducerRouteOwnerGroup::HitPoint,
        ),
    ]
}

fn obscurement_duration_cleanup_route() -> Vec<ReducerRouteEvent> {
    vec![
        resolve_without_fill_event(ReducerRouteOwnerGroup::TurnBoundary),
        resolve_without_fill_event(ReducerRouteOwnerGroup::ObscurementProjection),
        resolve_without_fill_event(ReducerRouteOwnerGroup::ActiveEffect),
        resolve_without_fill_event(ReducerRouteOwnerGroup::Concentration),
    ]
}

fn obscurement_dispersal_cleanup_route() -> Vec<ReducerRouteEvent> {
    vec![
        resolve_without_fill_event(ReducerRouteOwnerGroup::ObscurementProjection),
        resolve_without_fill_event(ReducerRouteOwnerGroup::ActiveEffect),
        resolve_without_fill_event(ReducerRouteOwnerGroup::Concentration),
    ]
}

fn hazard_cleanup_route() -> Vec<ReducerRouteEvent> {
    vec![
        resolve_without_fill_event(ReducerRouteOwnerGroup::TurnBoundary),
        resolve_without_fill_event(ReducerRouteOwnerGroup::AreaHazard),
        resolve_without_fill_event(ReducerRouteOwnerGroup::ActiveEffect),
    ]
}

fn table_spatial_witness_route() -> Vec<ReducerRouteEvent> {
    vec![
        resolve_without_fill_event(ReducerRouteOwnerGroup::AreaShape),
        resolve_without_fill_event(ReducerRouteOwnerGroup::SightProjection),
        resolve_without_fill_event(ReducerRouteOwnerGroup::AreaHazard),
    ]
}

fn discover(holes: Vec<ReducerRouteHoleKind>, owner: ReducerRouteOwnerGroup) -> ReducerRouteEvent {
    route_discover_battle_acts_from_route_holes(
        ReducerRouteSubjectFamily::SpatialEffectRoute,
        holes,
        owner,
    )
}

fn resolve_with_fill(
    fill: ReducerRouteFillKind,
    holes: Vec<ReducerRouteHoleKind>,
    owner: ReducerRouteOwnerGroup,
) -> ReducerRouteEvent {
    route_resolve_battle_subject_from_route_result(
        ReducerRouteSubjectFamily::SpatialEffectRoute,
        fill,
        if holes.is_empty() {
            ReducerRouteResolutionOutcome::Resolved
        } else {
            ReducerRouteResolutionOutcome::NeedsHoles
        },
        holes,
        owner,
    )
}

fn resolve_without_fill_event(owner: ReducerRouteOwnerGroup) -> ReducerRouteEvent {
    route_resolve_battle_subject_without_fill_from_route_result(
        ReducerRouteSubjectFamily::SpatialEffectRoute,
        ReducerRouteResolutionOutcome::Resolved,
        Vec::new(),
        owner,
    )
}

fn expected_facts(action: &str) -> Vec<SpatialEffectRouteFact> {
    let mut facts = match action {
        "doAdmitMovableMultiEmitterLight" | "doMoveMovableMultiEmitterLight" => {
            movable_multi_emitter_light_facts()
        }
        "doAdmitOutlineSightEffect" | "doProjectOutlineSightAttackAdvantage" => {
            outline_admission_facts()
        }
        "doAdmitAreaObscurement"
        | "doCleanupAreaObscurementByDuration"
        | "doDisperseAreaObscurementByStrongWind" => area_obscurement_facts(),
        "doAdmitAreaHazard"
        | "doResolveAreaHazardSavingThrowTrigger"
        | "doResolveAreaHazardDifficultTerrainMovement"
        | "doResolveAreaHazardMovementDamageTrigger"
        | "doCleanupAreaHazard" => area_hazard_facts(),
        "doRecordTableOwnedSpatialWitnesses" => table_spatial_witness_facts(),
        action => panic!("unsupported spatial-effects facts action {action}"),
    };

    match action {
        "doMoveMovableMultiEmitterLight" => facts.extend(movable_emitter_moved_facts()),
        "doProjectOutlineSightAttackAdvantage" => facts.extend(outline_projection_facts()),
        "doCleanupAreaObscurementByDuration" => {
            facts.extend(obscurement_duration_cleanup_facts());
        }
        "doDisperseAreaObscurementByStrongWind" => {
            facts.extend(obscurement_dispersal_cleanup_facts());
        }
        "doResolveAreaHazardSavingThrowTrigger" => facts.extend(hazard_save_facts()),
        "doResolveAreaHazardDifficultTerrainMovement" => {
            facts.extend(hazard_difficult_terrain_movement_facts());
        }
        "doResolveAreaHazardMovementDamageTrigger" => {
            facts.extend(hazard_movement_damage_facts());
        }
        "doCleanupAreaHazard" => facts.extend(hazard_cleanup_facts()),
        _ => {}
    }
    facts
}

fn movable_multi_emitter_light_facts() -> Vec<SpatialEffectRouteFact> {
    vec![
        SpatialEffectRouteFact::BattleEffect(
            SpatialEffectBattleEffectFact::MovableMultiEmitterEffectAdmitted,
        ),
        SpatialEffectRouteFact::BattleEffect(
            SpatialEffectBattleEffectFact::ConcentrationBackedEffect,
        ),
        SpatialEffectRouteFact::Light(SpatialEffectLightFact::MultiEmitterDimLightProjection),
        SpatialEffectRouteFact::Presentation(
            SpatialEffectPresentationFact::EmitterAppearancePresentation,
        ),
    ]
}

fn movable_emitter_moved_facts() -> Vec<SpatialEffectRouteFact> {
    vec![
        SpatialEffectRouteFact::Light(SpatialEffectLightFact::EmitterPositionMoved),
        SpatialEffectRouteFact::Geometry(SpatialEffectGeometryFact::MovementPathWitness),
    ]
}

fn outline_admission_facts() -> Vec<SpatialEffectRouteFact> {
    vec![
        SpatialEffectRouteFact::BattleEffect(SpatialEffectBattleEffectFact::OutlineEffectAdmitted),
        SpatialEffectRouteFact::BattleEffect(
            SpatialEffectBattleEffectFact::ConcentrationBackedEffect,
        ),
        SpatialEffectRouteFact::Geometry(SpatialEffectGeometryFact::AreaShapeWitness),
        SpatialEffectRouteFact::Geometry(SpatialEffectGeometryFact::AreaMembershipWitness),
        SpatialEffectRouteFact::Light(SpatialEffectLightFact::DimLightProjection),
        SpatialEffectRouteFact::Presentation(
            SpatialEffectPresentationFact::ColorChoicePresentation,
        ),
        SpatialEffectRouteFact::Presentation(
            SpatialEffectPresentationFact::VisibleOutlinePresentation,
        ),
        SpatialEffectRouteFact::Object(SpatialEffectObjectFact::OutlinedObjectWitness),
    ]
}

fn outline_projection_facts() -> Vec<SpatialEffectRouteFact> {
    vec![
        SpatialEffectRouteFact::Sight(SpatialEffectSightFact::AttackerCanSeeTargetWitness),
        SpatialEffectRouteFact::Sight(SpatialEffectSightFact::InvisibleBenefitDeniedProjection),
        SpatialEffectRouteFact::Sight(SpatialEffectSightFact::AttackRollAdvantageProjection),
        SpatialEffectRouteFact::Object(SpatialEffectObjectFact::ObjectSightTargetWitness),
        SpatialEffectRouteFact::Object(
            SpatialEffectObjectFact::ObjectInvisibleBenefitDeniedProjection,
        ),
    ]
}

fn area_obscurement_facts() -> Vec<SpatialEffectRouteFact> {
    vec![
        SpatialEffectRouteFact::BattleEffect(
            SpatialEffectBattleEffectFact::AreaObscurementEffectAdmitted,
        ),
        SpatialEffectRouteFact::BattleEffect(
            SpatialEffectBattleEffectFact::ConcentrationBackedEffect,
        ),
        SpatialEffectRouteFact::Geometry(SpatialEffectGeometryFact::AreaShapeWitness),
        SpatialEffectRouteFact::Sight(SpatialEffectSightFact::HeavilyObscuredProjection),
    ]
}

fn area_hazard_facts() -> Vec<SpatialEffectRouteFact> {
    vec![
        SpatialEffectRouteFact::BattleEffect(
            SpatialEffectBattleEffectFact::AreaHazardEffectAdmitted,
        ),
        SpatialEffectRouteFact::Geometry(SpatialEffectGeometryFact::AreaShapeWitness),
        SpatialEffectRouteFact::Hazard(SpatialEffectHazardFact::DifficultTerrainProjection),
    ]
}

fn hazard_save_facts() -> Vec<SpatialEffectRouteFact> {
    vec![
        SpatialEffectRouteFact::Geometry(SpatialEffectGeometryFact::AreaMembershipWitness),
        SpatialEffectRouteFact::Hazard(SpatialEffectHazardFact::EntrySavingThrowTrigger),
        SpatialEffectRouteFact::Hazard(SpatialEffectHazardFact::StartTurnSavingThrowTrigger),
        SpatialEffectRouteFact::Hazard(SpatialEffectHazardFact::PerTurnTriggerLimit),
    ]
}

fn hazard_difficult_terrain_movement_facts() -> Vec<SpatialEffectRouteFact> {
    vec![
        SpatialEffectRouteFact::Geometry(SpatialEffectGeometryFact::MovementPathWitness),
        SpatialEffectRouteFact::Hazard(SpatialEffectHazardFact::DifficultTerrainProjection),
    ]
}

fn hazard_movement_damage_facts() -> Vec<SpatialEffectRouteFact> {
    vec![
        SpatialEffectRouteFact::Geometry(SpatialEffectGeometryFact::MovementPathWitness),
        SpatialEffectRouteFact::Hazard(SpatialEffectHazardFact::MovementDamageTrigger),
    ]
}

fn obscurement_duration_cleanup_facts() -> Vec<SpatialEffectRouteFact> {
    vec![
        cleaned(
            SpatialEffectCleanupFact::DurationCleanup,
            SpatialEffectCleanupOwnerFact::BattleObscurementCleanupOwner,
        ),
        cleaned(
            SpatialEffectCleanupFact::DurationCleanup,
            SpatialEffectCleanupOwnerFact::BattleActiveEffectCleanupOwner,
        ),
        cleaned(
            SpatialEffectCleanupFact::DurationCleanup,
            SpatialEffectCleanupOwnerFact::BattleConcentrationCleanupOwner,
        ),
    ]
}

fn obscurement_dispersal_cleanup_facts() -> Vec<SpatialEffectRouteFact> {
    vec![
        SpatialEffectRouteFact::Geometry(SpatialEffectGeometryFact::StrongWindWitness),
        cleaned(
            SpatialEffectCleanupFact::DispersalCleanup,
            SpatialEffectCleanupOwnerFact::BattleObscurementCleanupOwner,
        ),
        cleaned(
            SpatialEffectCleanupFact::DispersalCleanup,
            SpatialEffectCleanupOwnerFact::BattleActiveEffectCleanupOwner,
        ),
        cleaned(
            SpatialEffectCleanupFact::DispersalCleanup,
            SpatialEffectCleanupOwnerFact::BattleConcentrationCleanupOwner,
        ),
    ]
}

fn hazard_cleanup_facts() -> Vec<SpatialEffectRouteFact> {
    vec![
        SpatialEffectRouteFact::Hazard(SpatialEffectHazardFact::AreaRemovedCleanup),
        cleaned(
            SpatialEffectCleanupFact::DurationCleanup,
            SpatialEffectCleanupOwnerFact::BattleAreaHazardCleanupOwner,
        ),
        cleaned(
            SpatialEffectCleanupFact::DurationCleanup,
            SpatialEffectCleanupOwnerFact::BattleActiveEffectCleanupOwner,
        ),
    ]
}

fn table_spatial_witness_facts() -> Vec<SpatialEffectRouteFact> {
    vec![
        SpatialEffectRouteFact::Geometry(SpatialEffectGeometryFact::AreaShapeWitness),
        SpatialEffectRouteFact::Geometry(SpatialEffectGeometryFact::AreaMembershipWitness),
        SpatialEffectRouteFact::Geometry(SpatialEffectGeometryFact::TotalCoverBlockerWitness),
        SpatialEffectRouteFact::Geometry(SpatialEffectGeometryFact::MovementPathWitness),
        SpatialEffectRouteFact::Object(SpatialEffectObjectFact::OutlinedObjectWitness),
        SpatialEffectRouteFact::Sight(SpatialEffectSightFact::AttackerCanSeeTargetWitness),
        SpatialEffectRouteFact::Sight(SpatialEffectSightFact::LightlyObscuredProjection),
        SpatialEffectRouteFact::Hazard(SpatialEffectHazardFact::DifficultTerrainProjection),
        SpatialEffectRouteFact::Presentation(
            SpatialEffectPresentationFact::ColorChoicePresentation,
        ),
    ]
}

fn cleaned(
    cleanup: SpatialEffectCleanupFact,
    owner: SpatialEffectCleanupOwnerFact,
) -> SpatialEffectRouteFact {
    SpatialEffectRouteFact::CleanedUpBy { cleanup, owner }
}

fn route_state_ref(state: BattleSpatialEffectRouteState) -> &'static str {
    match state {
        BattleSpatialEffectRouteState::Inactive => "FreshSpatialEffectRouteSurface",
        BattleSpatialEffectRouteState::MovableMultiEmitterLightActive => {
            "MovableMultiEmitterLightActiveRouteSurface"
        }
        BattleSpatialEffectRouteState::MovableMultiEmitterLightMoved => {
            "MovableMultiEmitterLightMovedRouteSurface"
        }
        BattleSpatialEffectRouteState::OutlineSightEffectActive => {
            "OutlineSightEffectActiveRouteSurface"
        }
        BattleSpatialEffectRouteState::OutlineSightEffectProjected => {
            "OutlineSightEffectProjectedRouteSurface"
        }
        BattleSpatialEffectRouteState::AreaObscurementActive => "AreaObscurementActiveRouteSurface",
        BattleSpatialEffectRouteState::AreaObscurementDurationCleaned => {
            "AreaObscurementDurationCleanedRouteSurface"
        }
        BattleSpatialEffectRouteState::AreaObscurementDispersed => {
            "AreaObscurementDispersedRouteSurface"
        }
        BattleSpatialEffectRouteState::AreaHazardActive => "AreaHazardActiveRouteSurface",
        BattleSpatialEffectRouteState::AreaHazardSaveResolved => {
            "AreaHazardSaveResolvedRouteSurface"
        }
        BattleSpatialEffectRouteState::AreaHazardDifficultTerrainMovementResolved => {
            "AreaHazardDifficultTerrainMovementResolvedRouteSurface"
        }
        BattleSpatialEffectRouteState::AreaHazardMovementDamageResolved => {
            "AreaHazardMovementDamageResolvedRouteSurface"
        }
        BattleSpatialEffectRouteState::AreaHazardCleaned => "AreaHazardCleanedRouteSurface",
        BattleSpatialEffectRouteState::TableSpatialWitnessRecorded => {
            "TableSpatialWitnessRouteSurface"
        }
    }
}

fn facts_payload(facts: &[SpatialEffectRouteFact]) -> String {
    if facts.is_empty() {
        return "none".to_string();
    }
    facts.iter().map(fact_ref).collect::<Vec<_>>().join(",")
}

fn fact_ref(fact: &SpatialEffectRouteFact) -> &'static str {
    match *fact {
        SpatialEffectRouteFact::BattleEffect(fact) => battle_effect_fact_ref(fact),
        SpatialEffectRouteFact::Light(fact) => light_fact_ref(fact),
        SpatialEffectRouteFact::Geometry(fact) => geometry_fact_ref(fact),
        SpatialEffectRouteFact::Presentation(fact) => presentation_fact_ref(fact),
        SpatialEffectRouteFact::Object(fact) => object_fact_ref(fact),
        SpatialEffectRouteFact::Sight(fact) => sight_fact_ref(fact),
        SpatialEffectRouteFact::Hazard(fact) => hazard_fact_ref(fact),
        SpatialEffectRouteFact::CleanedUpBy { cleanup, owner } => match (cleanup, owner) {
            (
                SpatialEffectCleanupFact::DurationCleanup,
                SpatialEffectCleanupOwnerFact::BattleObscurementCleanupOwner,
            ) => "DurationCleanup:BattleObscurementCleanupOwner",
            (
                SpatialEffectCleanupFact::DurationCleanup,
                SpatialEffectCleanupOwnerFact::BattleActiveEffectCleanupOwner,
            ) => "DurationCleanup:BattleActiveEffectCleanupOwner",
            (
                SpatialEffectCleanupFact::DurationCleanup,
                SpatialEffectCleanupOwnerFact::BattleConcentrationCleanupOwner,
            ) => "DurationCleanup:BattleConcentrationCleanupOwner",
            (
                SpatialEffectCleanupFact::DurationCleanup,
                SpatialEffectCleanupOwnerFact::BattleAreaHazardCleanupOwner,
            ) => "DurationCleanup:BattleAreaHazardCleanupOwner",
            (
                SpatialEffectCleanupFact::DispersalCleanup,
                SpatialEffectCleanupOwnerFact::BattleObscurementCleanupOwner,
            ) => "DispersalCleanup:BattleObscurementCleanupOwner",
            (
                SpatialEffectCleanupFact::DispersalCleanup,
                SpatialEffectCleanupOwnerFact::BattleActiveEffectCleanupOwner,
            ) => "DispersalCleanup:BattleActiveEffectCleanupOwner",
            (
                SpatialEffectCleanupFact::DispersalCleanup,
                SpatialEffectCleanupOwnerFact::BattleConcentrationCleanupOwner,
            ) => "DispersalCleanup:BattleConcentrationCleanupOwner",
            (
                SpatialEffectCleanupFact::DispersalCleanup,
                SpatialEffectCleanupOwnerFact::BattleAreaHazardCleanupOwner,
            ) => "DispersalCleanup:BattleAreaHazardCleanupOwner",
        },
    }
}

fn battle_effect_fact_ref(fact: SpatialEffectBattleEffectFact) -> &'static str {
    match fact {
        SpatialEffectBattleEffectFact::MovableMultiEmitterEffectAdmitted => {
            "MovableMultiEmitterEffectAdmitted"
        }
        SpatialEffectBattleEffectFact::OutlineEffectAdmitted => "OutlineEffectAdmitted",
        SpatialEffectBattleEffectFact::AreaObscurementEffectAdmitted => {
            "AreaObscurementEffectAdmitted"
        }
        SpatialEffectBattleEffectFact::AreaHazardEffectAdmitted => "AreaHazardEffectAdmitted",
        SpatialEffectBattleEffectFact::ConcentrationBackedEffect => "ConcentrationBackedEffect",
    }
}

fn light_fact_ref(fact: SpatialEffectLightFact) -> &'static str {
    match fact {
        SpatialEffectLightFact::MultiEmitterDimLightProjection => "MultiEmitterDimLightProjection",
        SpatialEffectLightFact::EmitterPositionMoved => "EmitterPositionMoved",
        SpatialEffectLightFact::DimLightProjection => "DimLightProjection",
    }
}

fn geometry_fact_ref(fact: SpatialEffectGeometryFact) -> &'static str {
    match fact {
        SpatialEffectGeometryFact::AreaShapeWitness => "AreaShapeWitness",
        SpatialEffectGeometryFact::AreaMembershipWitness => "AreaMembershipWitness",
        SpatialEffectGeometryFact::MovementPathWitness => "MovementPathWitness",
        SpatialEffectGeometryFact::TotalCoverBlockerWitness => "TotalCoverBlockerWitness",
        SpatialEffectGeometryFact::StrongWindWitness => "StrongWindWitness",
    }
}

fn presentation_fact_ref(fact: SpatialEffectPresentationFact) -> &'static str {
    match fact {
        SpatialEffectPresentationFact::EmitterAppearancePresentation => {
            "EmitterAppearancePresentation"
        }
        SpatialEffectPresentationFact::ColorChoicePresentation => "ColorChoicePresentation",
        SpatialEffectPresentationFact::VisibleOutlinePresentation => "VisibleOutlinePresentation",
    }
}

fn object_fact_ref(fact: SpatialEffectObjectFact) -> &'static str {
    match fact {
        SpatialEffectObjectFact::OutlinedObjectWitness => "OutlinedObjectWitness",
        SpatialEffectObjectFact::ObjectSightTargetWitness => "ObjectSightTargetWitness",
        SpatialEffectObjectFact::ObjectInvisibleBenefitDeniedProjection => {
            "ObjectInvisibleBenefitDeniedProjection"
        }
    }
}

fn sight_fact_ref(fact: SpatialEffectSightFact) -> &'static str {
    match fact {
        SpatialEffectSightFact::AttackerCanSeeTargetWitness => "AttackerCanSeeTargetWitness",
        SpatialEffectSightFact::InvisibleBenefitDeniedProjection => {
            "InvisibleBenefitDeniedProjection"
        }
        SpatialEffectSightFact::AttackRollAdvantageProjection => "AttackRollAdvantageProjection",
        SpatialEffectSightFact::HeavilyObscuredProjection => "HeavilyObscuredProjection",
        SpatialEffectSightFact::LightlyObscuredProjection => "LightlyObscuredProjection",
    }
}

fn hazard_fact_ref(fact: SpatialEffectHazardFact) -> &'static str {
    match fact {
        SpatialEffectHazardFact::DifficultTerrainProjection => "DifficultTerrainProjection",
        SpatialEffectHazardFact::EntrySavingThrowTrigger => "EntrySavingThrowTrigger",
        SpatialEffectHazardFact::StartTurnSavingThrowTrigger => "StartTurnSavingThrowTrigger",
        SpatialEffectHazardFact::MovementDamageTrigger => "MovementDamageTrigger",
        SpatialEffectHazardFact::PerTurnTriggerLimit => "PerTurnTriggerLimit",
        SpatialEffectHazardFact::AreaRemovedCleanup => "AreaRemovedCleanup",
    }
}
