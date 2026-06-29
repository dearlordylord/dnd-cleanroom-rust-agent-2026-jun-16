use crate::rules::battle_reducer_spine::{
    discover_generic_route_subject_observed, generic_route_subject_for_current_actor,
    resolve_battle_subject_observed, start_battle_observed, BattleEntrypointTrace,
    BattleGenericRouteFill, BattleResolutionRequest, BattleSetup, BattleState, BattleSubjectKind,
};
use crate::rules::level_one_spatial_spells::{
    project_dancing_lights_movable_dim_light,
    project_faerie_fire_outline_advantage_invisible_dim_light,
    project_feather_fall_reaction_mitigation_landing,
    project_fog_cloud_area_identity_obscurement_strong_wind_cleanup,
    project_grease_cast_ground_hazard_saving_throws, project_grease_movement_and_turn_triggers,
    project_jump_movement_replacement_landing_witness,
    project_light_object_emitter_projection_replacement_cleanup,
    project_produce_flame_held_light_projection_hurl_cleanup,
    project_thunderwave_save_push_objects_boom, AttackRollMode, Illumination,
    LevelOneSpatialProtocol, LevelOneSpatialScenarioOutcome, LevelOneSpatialState,
    SightObscurement,
};

use super::battle_runtime_reducer_route::{
    observed_reducer_route, route_discover_battle_acts_from_route_holes,
    route_resolve_battle_interrupt_from_route_holes,
    route_resolve_battle_subject_from_route_result,
    route_resolve_battle_subject_without_fill_from_route_result, route_start_battle,
    ReducerRouteEvent, ReducerRouteFillKind, ReducerRouteHoleKind, ReducerRouteOwnerGroup,
    ReducerRouteResolutionOutcome, ReducerRouteSubjectFamily,
};

pub const BRANCH_ACTIONS: [&str; 10] = [
    "doDancingLightsMovableDimLight",
    "doFaerieFireOutlineAdvantageInvisibleDimLight",
    "doFeatherFallReactionMitigationLanding",
    "doFogCloudAreaIdentityObscurementStrongWindCleanup",
    "doGreaseCastGroundHazardSavingThrows",
    "doGreaseMovementAndTurnTriggers",
    "doJumpMovementReplacementLandingWitness",
    "doLightObjectEmitterProjectionReplacementCleanup",
    "doProduceFlameHeldLightProjectionHurlCleanup",
    "doThunderwaveSavePushObjectsBoom",
];

pub const ACCEPTED_BRANCH_ACTIONS: [&str; 10] = BRANCH_ACTIONS;

pub fn replay_observed_action(observed_action_taken: &str) -> LevelOneSpatialState {
    match observed_action_taken {
        "doDancingLightsMovableDimLight" => project_dancing_lights_movable_dim_light(),
        "doFaerieFireOutlineAdvantageInvisibleDimLight" => {
            project_faerie_fire_outline_advantage_invisible_dim_light()
        }
        "doFeatherFallReactionMitigationLanding" => {
            project_feather_fall_reaction_mitigation_landing()
        }
        "doFogCloudAreaIdentityObscurementStrongWindCleanup" => {
            project_fog_cloud_area_identity_obscurement_strong_wind_cleanup()
        }
        "doGreaseCastGroundHazardSavingThrows" => project_grease_cast_ground_hazard_saving_throws(),
        "doGreaseMovementAndTurnTriggers" => project_grease_movement_and_turn_triggers(),
        "doJumpMovementReplacementLandingWitness" => {
            project_jump_movement_replacement_landing_witness()
        }
        "doLightObjectEmitterProjectionReplacementCleanup" => {
            project_light_object_emitter_projection_replacement_cleanup()
        }
        "doProduceFlameHeldLightProjectionHurlCleanup" => {
            project_produce_flame_held_light_projection_hurl_cleanup()
        }
        "doThunderwaveSavePushObjectsBoom" => project_thunderwave_save_push_objects_boom(),
        action => panic!("unsupported mbt::actionTaken {action}"),
    }
}

pub fn replay_observed_route(observed_action_taken: &str) -> Vec<ReducerRouteEvent> {
    let mut trace = BattleEntrypointTrace::default();
    let state = start_battle_observed(BattleSetup::standard(), &mut trace).state;
    let route_subjects = selected_composition_route_subjects(observed_action_taken);
    let _state =
        replay_selected_spatial_composition_route(observed_action_taken, state, &mut trace);
    observed_reducer_route(&trace, &route_subjects)
}

pub fn expected_witness(observed_action_taken: &str) -> LevelOneSpatialState {
    replay_observed_action(observed_action_taken)
}

pub fn expected_route(observed_action_taken: &str) -> Vec<ReducerRouteEvent> {
    let mut route = vec![route_start_battle(ReducerRouteOwnerGroup::ActionEconomy)];
    route.extend(selected_spatial_composition_route(observed_action_taken));
    route
}

pub fn projection_payload(state: &LevelOneSpatialState) -> String {
    [
        format!("qLightEmitterCount={}", state.light.emitter_count),
        format!(
            "qDimLightEmitterCount={}",
            state.light.dim_light_emitter_count
        ),
        format!(
            "qRetainedLightIdentityCount={}",
            state.light.retained_identity_count
        ),
        format!("qLightObjectAdmitted={}", state.light.object_admitted),
        format!(
            "qLightInvalidObjectRejectionCount={}",
            state.light.invalid_object_rejection_count
        ),
        format!("qLightDurationTicks={}", state.light.duration_ticks),
        format!(
            "qLightBrightProjectionIllumination={}",
            illumination_ref(state.light.bright_projection_illumination)
        ),
        format!(
            "qLightOpaqueCoverIllumination={}",
            illumination_ref(state.light.opaque_cover_illumination)
        ),
        format!(
            "qLightRecastReplacedPriorEmitter={}",
            state.light.recast_replaced_prior_emitter
        ),
        format!(
            "qLightDurationCleanupClearedEmitter={}",
            state.light.duration_cleanup_cleared_emitter
        ),
        format!(
            "qFaerieFireOutlinedCreatureCount={}",
            state.faerie_fire.outlined_creature_count
        ),
        format!(
            "qFaerieFireOutlinedObjectCount={}",
            state.faerie_fire.outlined_object_count
        ),
        format!(
            "qFaerieFireCreatureAttackRollMode={}",
            attack_roll_mode_ref(state.faerie_fire.creature_attack_roll_mode)
        ),
        format!(
            "qFaerieFireInvisibleCreatureAttackRollMode={}",
            attack_roll_mode_ref(state.faerie_fire.invisible_creature_attack_roll_mode)
        ),
        format!(
            "qFaerieFireObjectAttackRollMode={}",
            attack_roll_mode_ref(state.faerie_fire.object_attack_roll_mode)
        ),
        format!(
            "qFaerieFireTargetInvisible={}",
            state.faerie_fire.target_invisible
        ),
        format!(
            "qFaerieFireObjectInvisibleBenefitDenied={}",
            state.faerie_fire.object_invisible_benefit_denied
        ),
        format!(
            "qFeatherFallTriggerOffered={}",
            state.feather_fall.trigger_offered
        ),
        format!(
            "qFeatherFallUnwitnessedTriggerRejected={}",
            state.feather_fall.unwitnessed_trigger_rejected
        ),
        format!(
            "qFeatherFallReactionSpent={}",
            state.feather_fall.reaction_spent
        ),
        format!(
            "qFeatherFallSlotExpended={}",
            state.feather_fall.slot_expended
        ),
        format!(
            "qFeatherFallMitigatedTargetCountBeforeLanding={}",
            state.feather_fall.mitigated_target_count_before_landing
        ),
        format!(
            "qFeatherFallLandedTargetDescentRateCapFeetPerRound={}",
            state
                .feather_fall
                .landed_target_descent_rate_cap_feet_per_round
        ),
        format!(
            "qFeatherFallLandingFallDamagePrevented={}",
            state.feather_fall.landing_fall_damage_prevented
        ),
        format!(
            "qFeatherFallLandingFallingPronePrevented={}",
            state.feather_fall.landing_falling_prone_prevented
        ),
        format!(
            "qFeatherFallLandedTargetMitigationCleared={}",
            state.feather_fall.landed_target_mitigation_cleared
        ),
        format!(
            "qFeatherFallOtherTargetStillMitigated={}",
            state.feather_fall.other_target_still_mitigated
        ),
        format!(
            "qFogCloudAreaIdentityRetained={}",
            state.fog_cloud.area_identity_retained
        ),
        format!(
            "qFogCloudHeavilyObscuredZoneCount={}",
            state.fog_cloud.heavily_obscured_zone_count
        ),
        format!("qFogCloudRadiusFeet={}", state.fog_cloud.radius_feet),
        format!("qFogCloudDurationTicks={}", state.fog_cloud.duration_ticks),
        format!(
            "qFogCloudStrongWindCommandOffered={}",
            state.fog_cloud.strong_wind_command_offered
        ),
        format!(
            "qFogCloudCleanupClearedEffect={}",
            state.fog_cloud.cleanup_cleared_effect
        ),
        format!(
            "qFogCloudCleanupClearedZone={}",
            state.fog_cloud.cleanup_cleared_zone
        ),
        format!(
            "qFogCloudCleanupClearedConcentration={}",
            state.fog_cloud.cleanup_cleared_concentration
        ),
        format!("qFogCloudSlotExpended={}", state.fog_cloud.slot_expended),
        format!(
            "qGreaseAreaIdentityRetained={}",
            state.grease.area_identity_retained
        ),
        format!(
            "qGreaseActiveHazardCount={}",
            state.grease.active_hazard_count
        ),
        format!("qGreaseDurationTicks={}", state.grease.duration_ticks),
        format!(
            "qGreaseAffectedTargetOutcomeCount={}",
            state.grease.affected_target_outcome_count
        ),
        format!(
            "qGreaseFailedTargetProne={}",
            state.grease.failed_target_prone
        ),
        format!(
            "qGreaseSucceededTargetProne={}",
            state.grease.succeeded_target_prone
        ),
        format!(
            "qGreaseMismatchedAffectedTargetRejected={}",
            state.grease.mismatched_affected_target_rejected
        ),
        format!(
            "qGreaseDifficultTerrainMovementCostFeet={}",
            state.grease.difficult_terrain_movement_cost_feet
        ),
        format!(
            "qGreaseMovementSpentFeet={}",
            state.grease.movement_spent_feet
        ),
        format!(
            "qGreaseMismatchedMovementAreaRejected={}",
            state.grease.mismatched_movement_area_rejected
        ),
        format!(
            "qGreaseEntrySaveOffered={}",
            state.grease.entry_save_offered
        ),
        format!(
            "qGreaseEntryFailedTargetProne={}",
            state.grease.entry_failed_target_prone
        ),
        format!(
            "qGreaseEntryMismatchedTargetRejected={}",
            state.grease.entry_mismatched_target_rejected
        ),
        format!(
            "qGreaseEndTurnSaveOffered={}",
            state.grease.end_turn_save_offered
        ),
        format!(
            "qGreaseEndTurnFailedTargetProne={}",
            state.grease.end_turn_failed_target_prone
        ),
        format!(
            "qGreaseEndTurnAdvancedToCaster={}",
            state.grease.end_turn_advanced_to_caster
        ),
        format!(
            "qGreaseEndTurnMismatchedTargetRejected={}",
            state.grease.end_turn_mismatched_target_rejected
        ),
        format!("qGreaseSlotExpended={}", state.grease.slot_expended),
        format!(
            "qJumpTargetEffectInstalled={}",
            state.jump.target_effect_installed
        ),
        format!("qJumpMovementSpentFeet={}", state.jump.movement_spent_feet),
        format!("qJumpUsedMarkerSet={}", state.jump.used_marker_set),
        format!(
            "qJumpSameTurnUnavailable={}",
            state.jump.same_turn_unavailable
        ),
        format!(
            "qJumpNextTargetTurnAvailable={}",
            state.jump.next_target_turn_available
        ),
        format!(
            "qJumpMissingLandingFactRejected={}",
            state.jump.missing_landing_fact_rejected
        ),
        format!(
            "qJumpFailedLandingProne={}",
            state.jump.failed_landing_prone
        ),
        format!("qJumpSlotExpended={}", state.jump.slot_expended),
        format!(
            "qProduceFlameHeldLightInstalled={}",
            state.produce_flame.held_light_installed
        ),
        format!(
            "qProduceFlameDurationTicks={}",
            state.produce_flame.duration_ticks
        ),
        format!(
            "qProduceFlameBrightProjectionIllumination={}",
            illumination_ref(state.produce_flame.bright_projection_illumination)
        ),
        format!(
            "qProduceFlameHurlOffered={}",
            state.produce_flame.hurl_offered
        ),
        format!(
            "qProduceFlameHurlTargetDamaged={}",
            state.produce_flame.hurl_target_damaged
        ),
        format!(
            "qProduceFlameHurlCleanupClearedEmitter={}",
            state.produce_flame.hurl_cleanup_cleared_emitter
        ),
        format!(
            "qProduceFlameDurationCleanupClearedEmitter={}",
            state.produce_flame.duration_cleanup_cleared_emitter
        ),
        format!(
            "qThunderwaveAffectedTargetOutcomeCount={}",
            state.thunderwave.affected_target_outcome_count
        ),
        format!(
            "qThunderwaveFailedPushedTargetDamaged={}",
            state.thunderwave.failed_pushed_target_damaged
        ),
        format!(
            "qThunderwaveFailedBlockedTargetDamaged={}",
            state.thunderwave.failed_blocked_target_damaged
        ),
        format!(
            "qThunderwaveSucceededTargetHalfDamaged={}",
            state.thunderwave.succeeded_target_half_damaged
        ),
        format!(
            "qThunderwavePushedCreatureDispositionCount={}",
            state.thunderwave.pushed_creature_disposition_count
        ),
        format!(
            "qThunderwaveBlockedCreatureDispositionCount={}",
            state.thunderwave.blocked_creature_disposition_count
        ),
        format!(
            "qThunderwavePushedObjectDispositionCount={}",
            state.thunderwave.pushed_object_disposition_count
        ),
        format!(
            "qThunderwaveBlockedObjectDispositionCount={}",
            state.thunderwave.blocked_object_disposition_count
        ),
        format!(
            "qThunderwaveAudibleBoomMatched={}",
            state.thunderwave.audible_boom_matched
        ),
        format!(
            "qThunderwaveMissingAreaFactsRejected={}",
            state.thunderwave.missing_area_facts_rejected
        ),
        format!(
            "qThunderwaveMismatchedBoomRejected={}",
            state.thunderwave.mismatched_boom_rejected
        ),
        format!(
            "qThunderwaveSlotExpended={}",
            state.thunderwave.slot_expended
        ),
        format!(
            "qProjectedIllumination={}",
            illumination_ref(state.vision.projected_illumination)
        ),
        format!(
            "qOrdinarySightObscurement={}",
            sight_obscurement_ref(state.vision.ordinary_sight_obscurement)
        ),
        format!(
            "qDarkvisionSightObscurement={}",
            sight_obscurement_ref(state.vision.darkvision_sight_obscurement)
        ),
        format!(
            "qMismatchedWitnessIllumination={}",
            illumination_ref(state.vision.mismatched_witness_illumination)
        ),
        format!(
            "qObscurementZoneCount={}",
            state.vision.obscurement_zone_count
        ),
        format!("qCasterConcentrating={}", state.caster_concentrating),
        format!("qMagicActionAvailable={}", state.magic_action_available),
        format!("qBonusActionAvailable={}", state.bonus_action_available),
        format!(
            "qScenarioOutcome={}",
            scenario_outcome_ref(state.scenario_outcome)
        ),
        format!("protocolResult={}", protocol_ref(state.protocol)),
        "protocolHoles=none".to_string(),
    ]
    .join("\n")
}

fn selected_composition_route_subjects(action: &str) -> Vec<ReducerRouteSubjectFamily> {
    match action {
        "doDancingLightsMovableDimLight"
        | "doFaerieFireOutlineAdvantageInvisibleDimLight"
        | "doFogCloudAreaIdentityObscurementStrongWindCleanup"
        | "doGreaseCastGroundHazardSavingThrows"
        | "doGreaseMovementAndTurnTriggers" => {
            vec![ReducerRouteSubjectFamily::SpatialEffectRoute]
        }
        "doFeatherFallReactionMitigationLanding" => {
            vec![ReducerRouteSubjectFamily::ReactionFallMitigation]
        }
        "doJumpMovementReplacementLandingWitness" => {
            vec![ReducerRouteSubjectFamily::MovementPresentationRoute]
        }
        "doLightObjectEmitterProjectionReplacementCleanup"
        | "doProduceFlameHeldLightProjectionHurlCleanup" => {
            vec![ReducerRouteSubjectFamily::ObjectLightRider]
        }
        "doThunderwaveSavePushObjectsBoom" => {
            vec![ReducerRouteSubjectFamily::MovementPresentationRoute]
        }
        action => panic!("unsupported selected spatial composition route action {action}"),
    }
}

fn replay_selected_spatial_composition_route(
    action: &str,
    state: BattleState,
    trace: &mut BattleEntrypointTrace,
) -> BattleState {
    match action {
        "doDancingLightsMovableDimLight" => replay_route_steps(
            state,
            trace,
            &[
                RouteStep::Discover(BattleSubjectKind::SpatialEffectMovableMultiEmitterAdmission),
                RouteStep::Resolve(
                    BattleSubjectKind::SpatialEffectMovableMultiEmitterAdmission,
                    BattleGenericRouteFill::WithoutFill,
                ),
                RouteStep::Resolve(
                    BattleSubjectKind::SpatialEffectMovableMultiEmitterConcentration,
                    BattleGenericRouteFill::WithoutFill,
                ),
                RouteStep::Resolve(
                    BattleSubjectKind::SpatialEffectMovableMultiEmitterLightProjection,
                    BattleGenericRouteFill::WithoutFill,
                ),
                RouteStep::Discover(
                    BattleSubjectKind::SpatialEffectMovableMultiEmitterMoveTargetChoice,
                ),
                RouteStep::Resolve(
                    BattleSubjectKind::SpatialEffectMovableMultiEmitterMoveTargetChoice,
                    BattleGenericRouteFill::TargetChoice,
                ),
                RouteStep::Resolve(
                    BattleSubjectKind::SpatialEffectMovableMultiEmitterMoveLightProjection,
                    BattleGenericRouteFill::WithoutFill,
                ),
            ],
        ),
        "doFaerieFireOutlineAdvantageInvisibleDimLight" => replay_route_steps(
            state,
            trace,
            &[
                RouteStep::Discover(BattleSubjectKind::SpatialEffectOutlineTargetChoice),
                RouteStep::Resolve(
                    BattleSubjectKind::SpatialEffectOutlineTargetChoice,
                    BattleGenericRouteFill::TargetChoice,
                ),
                RouteStep::Resolve(
                    BattleSubjectKind::SpatialEffectOutlineSavingThrowOutcome,
                    BattleGenericRouteFill::SavingThrowOutcome,
                ),
                RouteStep::Resolve(
                    BattleSubjectKind::SpatialEffectOutlineActiveEffect,
                    BattleGenericRouteFill::WithoutFill,
                ),
                RouteStep::Resolve(
                    BattleSubjectKind::SpatialEffectOutlineConcentration,
                    BattleGenericRouteFill::WithoutFill,
                ),
                RouteStep::Resolve(
                    BattleSubjectKind::SpatialEffectOutlineLightProjection,
                    BattleGenericRouteFill::WithoutFill,
                ),
                RouteStep::Resolve(
                    BattleSubjectKind::SpatialEffectOutlineSightProjection,
                    BattleGenericRouteFill::WithoutFill,
                ),
                RouteStep::Resolve(
                    BattleSubjectKind::SpatialEffectOutlineAttackRollModeProjection,
                    BattleGenericRouteFill::WithoutFill,
                ),
            ],
        ),
        "doFeatherFallReactionMitigationLanding" => replay_route_steps(
            state,
            trace,
            &[
                RouteStep::Discover(BattleSubjectKind::ReactionFallMitigationSlotCommit),
                RouteStep::Resolve(
                    BattleSubjectKind::ReactionFallMitigationSlotCommit,
                    BattleGenericRouteFill::InterruptDecision,
                ),
                RouteStep::Resolve(
                    BattleSubjectKind::ReactionFallMitigationActiveEffect,
                    BattleGenericRouteFill::WithoutFill,
                ),
                RouteStep::Resolve(
                    BattleSubjectKind::ReactionFallMitigationMovement,
                    BattleGenericRouteFill::WithoutFill,
                ),
                RouteStep::Resolve(
                    BattleSubjectKind::ReactionFallMitigationHitPoint,
                    BattleGenericRouteFill::WithoutFill,
                ),
            ],
        ),
        "doFogCloudAreaIdentityObscurementStrongWindCleanup" => replay_route_steps(
            state,
            trace,
            &[
                RouteStep::Discover(BattleSubjectKind::SpatialEffectAreaObscurementTargetChoice),
                RouteStep::Resolve(
                    BattleSubjectKind::SpatialEffectAreaObscurementTargetChoice,
                    BattleGenericRouteFill::TargetChoice,
                ),
                RouteStep::Resolve(
                    BattleSubjectKind::SpatialEffectAreaObscurementActiveEffect,
                    BattleGenericRouteFill::WithoutFill,
                ),
                RouteStep::Resolve(
                    BattleSubjectKind::SpatialEffectAreaObscurementConcentration,
                    BattleGenericRouteFill::WithoutFill,
                ),
                RouteStep::Resolve(
                    BattleSubjectKind::SpatialEffectAreaObscurementProjection,
                    BattleGenericRouteFill::WithoutFill,
                ),
                RouteStep::Resolve(
                    BattleSubjectKind::SpatialEffectAreaObscurementSightProjection,
                    BattleGenericRouteFill::WithoutFill,
                ),
                RouteStep::Resolve(
                    BattleSubjectKind::SpatialEffectAreaObscurementDispersalProjectionCleanup,
                    BattleGenericRouteFill::WithoutFill,
                ),
                RouteStep::Resolve(
                    BattleSubjectKind::SpatialEffectAreaObscurementDispersalActiveEffectCleanup,
                    BattleGenericRouteFill::WithoutFill,
                ),
                RouteStep::Resolve(
                    BattleSubjectKind::SpatialEffectAreaObscurementDispersalConcentrationCleanup,
                    BattleGenericRouteFill::WithoutFill,
                ),
            ],
        ),
        "doGreaseCastGroundHazardSavingThrows" => replay_route_steps(
            state,
            trace,
            &[
                RouteStep::Discover(BattleSubjectKind::SpatialEffectAreaHazardTargetChoice),
                RouteStep::Resolve(
                    BattleSubjectKind::SpatialEffectAreaHazardTargetChoice,
                    BattleGenericRouteFill::TargetChoice,
                ),
                RouteStep::Resolve(
                    BattleSubjectKind::SpatialEffectAreaHazardActiveEffect,
                    BattleGenericRouteFill::WithoutFill,
                ),
                RouteStep::Resolve(
                    BattleSubjectKind::SpatialEffectAreaHazardProjection,
                    BattleGenericRouteFill::WithoutFill,
                ),
                RouteStep::Resolve(
                    BattleSubjectKind::SpatialEffectAreaHazardCreatureSpaceMovement,
                    BattleGenericRouteFill::WithoutFill,
                ),
                RouteStep::Discover(BattleSubjectKind::SpatialEffectAreaHazardSavingThrow),
                RouteStep::Resolve(
                    BattleSubjectKind::SpatialEffectAreaHazardSavingThrow,
                    BattleGenericRouteFill::SavingThrowOutcome,
                ),
                RouteStep::Resolve(
                    BattleSubjectKind::SpatialEffectAreaHazardConditionLifecycle,
                    BattleGenericRouteFill::WithoutFill,
                ),
            ],
        ),
        "doGreaseMovementAndTurnTriggers" => replay_route_steps(
            state,
            trace,
            &[
                RouteStep::Discover(BattleSubjectKind::SpatialEffectAreaHazardTargetChoice),
                RouteStep::Resolve(
                    BattleSubjectKind::SpatialEffectAreaHazardTargetChoice,
                    BattleGenericRouteFill::TargetChoice,
                ),
                RouteStep::Resolve(
                    BattleSubjectKind::SpatialEffectAreaHazardActiveEffect,
                    BattleGenericRouteFill::WithoutFill,
                ),
                RouteStep::Resolve(
                    BattleSubjectKind::SpatialEffectAreaHazardProjection,
                    BattleGenericRouteFill::WithoutFill,
                ),
                RouteStep::Resolve(
                    BattleSubjectKind::SpatialEffectAreaHazardCreatureSpaceMovement,
                    BattleGenericRouteFill::WithoutFill,
                ),
                RouteStep::Discover(
                    BattleSubjectKind::SpatialEffectAreaHazardDifficultTerrainMovement,
                ),
                RouteStep::Resolve(
                    BattleSubjectKind::SpatialEffectAreaHazardDifficultTerrainMovement,
                    BattleGenericRouteFill::Movement { accepted: true },
                ),
                RouteStep::Resolve(
                    BattleSubjectKind::SpatialEffectAreaHazardCleanupTurnBoundary,
                    BattleGenericRouteFill::WithoutFill,
                ),
                RouteStep::Resolve(
                    BattleSubjectKind::SpatialEffectAreaHazardCleanupHazard,
                    BattleGenericRouteFill::WithoutFill,
                ),
                RouteStep::Resolve(
                    BattleSubjectKind::SpatialEffectAreaHazardCleanupActiveEffect,
                    BattleGenericRouteFill::WithoutFill,
                ),
            ],
        ),
        "doJumpMovementReplacementLandingWitness" => replay_route_steps(
            state,
            trace,
            &[
                RouteStep::Discover(BattleSubjectKind::MovementPresentationReplacementMovement),
                RouteStep::Resolve(
                    BattleSubjectKind::MovementPresentationReplacementMovement,
                    BattleGenericRouteFill::Movement { accepted: true },
                ),
                RouteStep::Resolve(
                    BattleSubjectKind::MovementPresentationReplacementTable,
                    BattleGenericRouteFill::WithoutFill,
                ),
                RouteStep::Resolve(
                    BattleSubjectKind::MovementPresentationReplacementConditionLifecycle,
                    BattleGenericRouteFill::WithoutFill,
                ),
            ],
        ),
        "doLightObjectEmitterProjectionReplacementCleanup" => replay_route_steps(
            state,
            trace,
            &[
                RouteStep::Discover(BattleSubjectKind::ObjectLightObjectAttachedAdmission),
                RouteStep::Resolve(
                    BattleSubjectKind::ObjectLightObjectAttachedAdmission,
                    BattleGenericRouteFill::TargetChoice,
                ),
                RouteStep::Resolve(
                    BattleSubjectKind::ObjectLightEmitterAdmission,
                    BattleGenericRouteFill::WithoutFill,
                ),
                RouteStep::Resolve(
                    BattleSubjectKind::ObjectLightProjection,
                    BattleGenericRouteFill::WithoutFill,
                ),
                RouteStep::Resolve(
                    BattleSubjectKind::ObjectLightReplacementCleanup,
                    BattleGenericRouteFill::WithoutFill,
                ),
            ],
        ),
        "doProduceFlameHeldLightProjectionHurlCleanup" => replay_route_steps(
            state,
            trace,
            &[
                RouteStep::Resolve(
                    BattleSubjectKind::ObjectLightHeldEmitterAdmission,
                    BattleGenericRouteFill::WithoutFill,
                ),
                RouteStep::Resolve(
                    BattleSubjectKind::ObjectLightProjection,
                    BattleGenericRouteFill::WithoutFill,
                ),
                RouteStep::Resolve(
                    BattleSubjectKind::ObjectLightHurlCleanup,
                    BattleGenericRouteFill::WithoutFill,
                ),
            ],
        ),
        "doThunderwaveSavePushObjectsBoom" => replay_route_steps(
            state,
            trace,
            &[
                RouteStep::Discover(BattleSubjectKind::MovementPresentationForcedSavingThrow),
                RouteStep::Resolve(
                    BattleSubjectKind::MovementPresentationForcedSavingThrow,
                    BattleGenericRouteFill::SavingThrowOutcome,
                ),
                RouteStep::Resolve(
                    BattleSubjectKind::MovementPresentationForcedMovement,
                    BattleGenericRouteFill::Movement { accepted: true },
                ),
                RouteStep::Resolve(
                    BattleSubjectKind::MovementPresentationForcedTable,
                    BattleGenericRouteFill::WithoutFill,
                ),
                RouteStep::Discover(BattleSubjectKind::MovementPresentationObjectBoundary),
                RouteStep::Resolve(
                    BattleSubjectKind::MovementPresentationObjectBoundary,
                    BattleGenericRouteFill::WithoutFill,
                ),
                RouteStep::Resolve(
                    BattleSubjectKind::MovementPresentationObjectTable,
                    BattleGenericRouteFill::WithoutFill,
                ),
            ],
        ),
        action => panic!("unsupported selected spatial composition route action {action}"),
    }
}

fn selected_spatial_composition_route(action: &str) -> Vec<ReducerRouteEvent> {
    match action {
        "doDancingLightsMovableDimLight" => movable_multi_emitter_light_route(),
        "doFaerieFireOutlineAdvantageInvisibleDimLight" => outline_sight_advantage_route(),
        "doFeatherFallReactionMitigationLanding" => fall_mitigation_route(),
        "doFogCloudAreaIdentityObscurementStrongWindCleanup" => area_obscurement_cleanup_route(),
        "doGreaseCastGroundHazardSavingThrows" => area_hazard_save_route(),
        "doGreaseMovementAndTurnTriggers" => area_hazard_movement_route(),
        "doJumpMovementReplacementLandingWitness" => movement_replacement_route(),
        "doLightObjectEmitterProjectionReplacementCleanup" => object_light_emitter_route(),
        "doProduceFlameHeldLightProjectionHurlCleanup" => held_light_emitter_route(),
        "doThunderwaveSavePushObjectsBoom" => save_push_presentation_route(),
        action => panic!("unsupported selected spatial expected route action {action}"),
    }
}

#[derive(Debug, Clone, Copy)]
enum RouteStep {
    Discover(BattleSubjectKind),
    Resolve(BattleSubjectKind, BattleGenericRouteFill),
}

fn replay_route_steps(
    mut state: BattleState,
    trace: &mut BattleEntrypointTrace,
    steps: &[RouteStep],
) -> BattleState {
    for step in steps {
        state = match *step {
            RouteStep::Discover(kind) => {
                let (next_state, _) = discover_generic_route_subject_observed(state, kind, trace);
                next_state
            }
            RouteStep::Resolve(kind, fill) => {
                let subject = generic_route_subject_for_current_actor(&state, kind);
                resolve_battle_subject_observed(
                    state,
                    BattleResolutionRequest::generic_route(subject, fill)
                        .expect("selected spatial route step should accept generic fill"),
                    trace,
                )
                .into_state()
            }
        };
    }
    state
}

fn movable_multi_emitter_light_route() -> Vec<ReducerRouteEvent> {
    vec![
        spatial_discover(
            Vec::new(),
            ReducerRouteOwnerGroup::SpellSlotAndActionEconomy,
        ),
        spatial_resolve_without_fill(ReducerRouteOwnerGroup::ActiveEffect),
        spatial_resolve_without_fill(ReducerRouteOwnerGroup::Concentration),
        spatial_resolve_without_fill(ReducerRouteOwnerGroup::LightProjection),
        spatial_discover(
            vec![ReducerRouteHoleKind::TargetChoice],
            ReducerRouteOwnerGroup::SpellSlotAndActionEconomy,
        ),
        spatial_resolve_with_fill(
            ReducerRouteFillKind::TargetChoice,
            Vec::new(),
            ReducerRouteOwnerGroup::AreaShape,
        ),
        spatial_resolve_without_fill(ReducerRouteOwnerGroup::LightProjection),
    ]
}

fn outline_sight_advantage_route() -> Vec<ReducerRouteEvent> {
    vec![
        spatial_discover(
            vec![
                ReducerRouteHoleKind::TargetChoice,
                ReducerRouteHoleKind::SavingThrowOutcome,
            ],
            ReducerRouteOwnerGroup::SpellSlotAndActionEconomy,
        ),
        spatial_resolve_with_fill(
            ReducerRouteFillKind::TargetChoice,
            vec![ReducerRouteHoleKind::SavingThrowOutcome],
            ReducerRouteOwnerGroup::AreaShape,
        ),
        spatial_resolve_with_fill(
            ReducerRouteFillKind::SavingThrowOutcome,
            Vec::new(),
            ReducerRouteOwnerGroup::SavingThrowOutcome,
        ),
        spatial_resolve_without_fill(ReducerRouteOwnerGroup::ActiveEffect),
        spatial_resolve_without_fill(ReducerRouteOwnerGroup::Concentration),
        spatial_resolve_without_fill(ReducerRouteOwnerGroup::LightProjection),
        spatial_resolve_without_fill(ReducerRouteOwnerGroup::SightProjection),
        spatial_resolve_without_fill(ReducerRouteOwnerGroup::AttackRollMode),
    ]
}

fn fall_mitigation_route() -> Vec<ReducerRouteEvent> {
    let subject = ReducerRouteSubjectFamily::ReactionFallMitigation;
    vec![
        route_discover_battle_acts_from_route_holes(
            subject,
            vec![ReducerRouteHoleKind::InterruptDecision],
            ReducerRouteOwnerGroup::InterruptStack,
        ),
        route_resolve_battle_interrupt_from_route_holes(
            subject,
            ReducerRouteFillKind::InterruptDecision,
            Vec::new(),
            ReducerRouteOwnerGroup::SpellSlotAndActionEconomy,
        ),
        route_resolve_battle_subject_without_fill_from_route_result(
            subject,
            ReducerRouteResolutionOutcome::Resolved,
            Vec::new(),
            ReducerRouteOwnerGroup::ActiveEffect,
        ),
        route_resolve_battle_subject_without_fill_from_route_result(
            subject,
            ReducerRouteResolutionOutcome::Resolved,
            Vec::new(),
            ReducerRouteOwnerGroup::MovementResource,
        ),
        route_resolve_battle_subject_without_fill_from_route_result(
            subject,
            ReducerRouteResolutionOutcome::Resolved,
            Vec::new(),
            ReducerRouteOwnerGroup::HitPoint,
        ),
    ]
}

fn area_obscurement_cleanup_route() -> Vec<ReducerRouteEvent> {
    vec![
        spatial_discover(
            vec![ReducerRouteHoleKind::TargetChoice],
            ReducerRouteOwnerGroup::SpellSlotAndActionEconomy,
        ),
        spatial_resolve_with_fill(
            ReducerRouteFillKind::TargetChoice,
            Vec::new(),
            ReducerRouteOwnerGroup::AreaShape,
        ),
        spatial_resolve_without_fill(ReducerRouteOwnerGroup::ActiveEffect),
        spatial_resolve_without_fill(ReducerRouteOwnerGroup::Concentration),
        spatial_resolve_without_fill(ReducerRouteOwnerGroup::ObscurementProjection),
        spatial_resolve_without_fill(ReducerRouteOwnerGroup::SightProjection),
        spatial_resolve_without_fill(ReducerRouteOwnerGroup::ObscurementProjection),
        spatial_resolve_without_fill(ReducerRouteOwnerGroup::ActiveEffect),
        spatial_resolve_without_fill(ReducerRouteOwnerGroup::Concentration),
    ]
}

fn area_hazard_save_route() -> Vec<ReducerRouteEvent> {
    vec![
        spatial_discover(
            vec![ReducerRouteHoleKind::TargetChoice],
            ReducerRouteOwnerGroup::SpellSlotAndActionEconomy,
        ),
        spatial_resolve_with_fill(
            ReducerRouteFillKind::TargetChoice,
            Vec::new(),
            ReducerRouteOwnerGroup::AreaShape,
        ),
        spatial_resolve_without_fill(ReducerRouteOwnerGroup::ActiveEffect),
        spatial_resolve_without_fill(ReducerRouteOwnerGroup::AreaHazard),
        spatial_resolve_without_fill(ReducerRouteOwnerGroup::CreatureSpaceMovement),
        spatial_discover(
            vec![ReducerRouteHoleKind::SavingThrowOutcome],
            ReducerRouteOwnerGroup::AreaHazard,
        ),
        spatial_resolve_with_fill(
            ReducerRouteFillKind::SavingThrowOutcome,
            Vec::new(),
            ReducerRouteOwnerGroup::SavingThrowOutcome,
        ),
        spatial_resolve_without_fill(ReducerRouteOwnerGroup::ConditionLifecycle),
    ]
}

fn area_hazard_movement_route() -> Vec<ReducerRouteEvent> {
    vec![
        spatial_discover(
            vec![ReducerRouteHoleKind::TargetChoice],
            ReducerRouteOwnerGroup::SpellSlotAndActionEconomy,
        ),
        spatial_resolve_with_fill(
            ReducerRouteFillKind::TargetChoice,
            Vec::new(),
            ReducerRouteOwnerGroup::AreaShape,
        ),
        spatial_resolve_without_fill(ReducerRouteOwnerGroup::ActiveEffect),
        spatial_resolve_without_fill(ReducerRouteOwnerGroup::AreaHazard),
        spatial_resolve_without_fill(ReducerRouteOwnerGroup::CreatureSpaceMovement),
        spatial_discover(
            vec![ReducerRouteHoleKind::Movement],
            ReducerRouteOwnerGroup::AreaHazard,
        ),
        spatial_resolve_with_fill(
            ReducerRouteFillKind::Movement,
            Vec::new(),
            ReducerRouteOwnerGroup::MovementResource,
        ),
        spatial_resolve_without_fill(ReducerRouteOwnerGroup::TurnBoundary),
        spatial_resolve_without_fill(ReducerRouteOwnerGroup::AreaHazard),
        spatial_resolve_without_fill(ReducerRouteOwnerGroup::ActiveEffect),
    ]
}

fn movement_replacement_route() -> Vec<ReducerRouteEvent> {
    vec![
        movement_discover(
            vec![ReducerRouteHoleKind::Movement],
            ReducerRouteOwnerGroup::MovementResource,
        ),
        movement_resolve_with_fill(
            ReducerRouteFillKind::Movement,
            Vec::new(),
            ReducerRouteOwnerGroup::MovementResource,
        ),
        movement_resolve_without_fill(ReducerRouteOwnerGroup::TablePresentation),
        movement_resolve_without_fill(ReducerRouteOwnerGroup::ConditionLifecycle),
    ]
}

fn object_light_emitter_route() -> Vec<ReducerRouteEvent> {
    vec![
        object_light_discover(
            vec![ReducerRouteHoleKind::TargetChoice],
            ReducerRouteOwnerGroup::SpellSlotAndActionEconomy,
        ),
        object_light_resolve_with_fill(
            ReducerRouteFillKind::TargetChoice,
            Vec::new(),
            ReducerRouteOwnerGroup::ObjectTargetBoundary,
        ),
        object_light_resolve_without_fill(ReducerRouteOwnerGroup::ActiveEffect),
        object_light_resolve_without_fill(ReducerRouteOwnerGroup::LightProjection),
        object_light_resolve_without_fill(ReducerRouteOwnerGroup::ActiveEffect),
    ]
}

fn held_light_emitter_route() -> Vec<ReducerRouteEvent> {
    vec![
        object_light_resolve_without_fill(ReducerRouteOwnerGroup::ActiveEffect),
        object_light_resolve_without_fill(ReducerRouteOwnerGroup::LightProjection),
        object_light_resolve_without_fill(ReducerRouteOwnerGroup::ActiveEffect),
    ]
}

fn save_push_presentation_route() -> Vec<ReducerRouteEvent> {
    vec![
        movement_discover(
            vec![
                ReducerRouteHoleKind::SavingThrowOutcome,
                ReducerRouteHoleKind::Movement,
            ],
            ReducerRouteOwnerGroup::SavingThrowOutcome,
        ),
        movement_resolve_with_fill(
            ReducerRouteFillKind::SavingThrowOutcome,
            vec![ReducerRouteHoleKind::Movement],
            ReducerRouteOwnerGroup::SavingThrowOutcome,
        ),
        movement_resolve_with_fill(
            ReducerRouteFillKind::Movement,
            Vec::new(),
            ReducerRouteOwnerGroup::MovementResource,
        ),
        movement_resolve_without_fill(ReducerRouteOwnerGroup::TablePresentation),
        movement_discover(Vec::new(), ReducerRouteOwnerGroup::ObjectTargetBoundary),
        movement_resolve_without_fill(ReducerRouteOwnerGroup::ObjectTargetBoundary),
        movement_resolve_without_fill(ReducerRouteOwnerGroup::TablePresentation),
    ]
}

fn spatial_discover(
    holes: Vec<ReducerRouteHoleKind>,
    owner: ReducerRouteOwnerGroup,
) -> ReducerRouteEvent {
    route_discover_battle_acts_from_route_holes(
        ReducerRouteSubjectFamily::SpatialEffectRoute,
        holes,
        owner,
    )
}

fn spatial_resolve_with_fill(
    fill: ReducerRouteFillKind,
    holes: Vec<ReducerRouteHoleKind>,
    owner: ReducerRouteOwnerGroup,
) -> ReducerRouteEvent {
    resolve_with_subject_fill(
        ReducerRouteSubjectFamily::SpatialEffectRoute,
        fill,
        holes,
        owner,
    )
}

fn spatial_resolve_without_fill(owner: ReducerRouteOwnerGroup) -> ReducerRouteEvent {
    resolve_without_subject_fill(ReducerRouteSubjectFamily::SpatialEffectRoute, owner)
}

fn movement_discover(
    holes: Vec<ReducerRouteHoleKind>,
    owner: ReducerRouteOwnerGroup,
) -> ReducerRouteEvent {
    route_discover_battle_acts_from_route_holes(
        ReducerRouteSubjectFamily::MovementPresentationRoute,
        holes,
        owner,
    )
}

fn movement_resolve_with_fill(
    fill: ReducerRouteFillKind,
    holes: Vec<ReducerRouteHoleKind>,
    owner: ReducerRouteOwnerGroup,
) -> ReducerRouteEvent {
    resolve_with_subject_fill(
        ReducerRouteSubjectFamily::MovementPresentationRoute,
        fill,
        holes,
        owner,
    )
}

fn movement_resolve_without_fill(owner: ReducerRouteOwnerGroup) -> ReducerRouteEvent {
    resolve_without_subject_fill(ReducerRouteSubjectFamily::MovementPresentationRoute, owner)
}

fn object_light_discover(
    holes: Vec<ReducerRouteHoleKind>,
    owner: ReducerRouteOwnerGroup,
) -> ReducerRouteEvent {
    route_discover_battle_acts_from_route_holes(
        ReducerRouteSubjectFamily::ObjectLightRider,
        holes,
        owner,
    )
}

fn object_light_resolve_with_fill(
    fill: ReducerRouteFillKind,
    holes: Vec<ReducerRouteHoleKind>,
    owner: ReducerRouteOwnerGroup,
) -> ReducerRouteEvent {
    resolve_with_subject_fill(
        ReducerRouteSubjectFamily::ObjectLightRider,
        fill,
        holes,
        owner,
    )
}

fn object_light_resolve_without_fill(owner: ReducerRouteOwnerGroup) -> ReducerRouteEvent {
    resolve_without_subject_fill(ReducerRouteSubjectFamily::ObjectLightRider, owner)
}

fn resolve_with_subject_fill(
    subject: ReducerRouteSubjectFamily,
    fill: ReducerRouteFillKind,
    holes: Vec<ReducerRouteHoleKind>,
    owner: ReducerRouteOwnerGroup,
) -> ReducerRouteEvent {
    route_resolve_battle_subject_from_route_result(
        subject,
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

fn resolve_without_subject_fill(
    subject: ReducerRouteSubjectFamily,
    owner: ReducerRouteOwnerGroup,
) -> ReducerRouteEvent {
    route_resolve_battle_subject_without_fill_from_route_result(
        subject,
        ReducerRouteResolutionOutcome::Resolved,
        Vec::new(),
        owner,
    )
}

fn illumination_ref(value: Illumination) -> &'static str {
    match value {
        Illumination::Darkness => "darkness",
        Illumination::DimLight => "dimLight",
        Illumination::BrightLight => "brightLight",
    }
}

fn sight_obscurement_ref(value: SightObscurement) -> &'static str {
    match value {
        SightObscurement::HeavilyObscured => "heavilyObscured",
        SightObscurement::LightlyObscured => "lightlyObscured",
        SightObscurement::Unobscured => "unobscured",
    }
}

fn attack_roll_mode_ref(value: AttackRollMode) -> &'static str {
    match value {
        AttackRollMode::Normal => "normal",
        AttackRollMode::Advantage => "advantage",
    }
}

fn scenario_outcome_ref(outcome: LevelOneSpatialScenarioOutcome) -> &'static str {
    match outcome {
        LevelOneSpatialScenarioOutcome::Init => "Init",
        LevelOneSpatialScenarioOutcome::DancingLightsMovableDimLight => {
            "DancingLightsMovableDimLight"
        }
        LevelOneSpatialScenarioOutcome::FaerieFireOutlineAdvantageInvisibleDimLight => {
            "FaerieFireOutlineAdvantageInvisibleDimLight"
        }
        LevelOneSpatialScenarioOutcome::FeatherFallReactionMitigationLanding => {
            "FeatherFallReactionMitigationLanding"
        }
        LevelOneSpatialScenarioOutcome::FogCloudAreaIdentityObscurementStrongWindCleanup => {
            "FogCloudAreaIdentityObscurementStrongWindCleanup"
        }
        LevelOneSpatialScenarioOutcome::GreaseCastGroundHazardSavingThrows => {
            "GreaseCastGroundHazardSavingThrows"
        }
        LevelOneSpatialScenarioOutcome::GreaseMovementAndTurnTriggers => {
            "GreaseMovementAndTurnTriggers"
        }
        LevelOneSpatialScenarioOutcome::JumpMovementReplacementLandingWitness => {
            "JumpMovementReplacementLandingWitness"
        }
        LevelOneSpatialScenarioOutcome::LightObjectEmitterProjectionReplacementCleanup => {
            "LightObjectEmitterProjectionReplacementCleanup"
        }
        LevelOneSpatialScenarioOutcome::ProduceFlameHeldLightProjectionHurlCleanup => {
            "ProduceFlameHeldLightProjectionHurlCleanup"
        }
        LevelOneSpatialScenarioOutcome::ThunderwaveSavePushObjectsBoom => {
            "ThunderwaveSavePushObjectsBoom"
        }
    }
}

fn protocol_ref(protocol: LevelOneSpatialProtocol) -> &'static str {
    match protocol {
        LevelOneSpatialProtocol::Init => "init",
        LevelOneSpatialProtocol::Resolved => "resolved",
    }
}
