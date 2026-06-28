use crate::rules::battle_reducer_spine::{
    discover_battle_acts_observed, resolve_battle_subject_observed, spatial_route_fill,
    start_battle_observed, BattleEntrypointTrace, BattleResolutionRequest, BattleSetup,
    BattleSpatialRouteSubject, BattleSubjectKind,
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
    observed_reducer_route, ReducerRouteEvent, ReducerRouteFillEvidence, ReducerRouteFillKind,
    ReducerRouteHoleKind, ReducerRouteOwnerGroup, ReducerRouteResolutionOutcome,
    ReducerRouteSubjectFamily,
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
    let subject = spatial_route_subject(observed_action_taken);
    let mut trace = BattleEntrypointTrace::default();
    let mut setup = BattleSetup::standard();
    setup.spatial_route_subjects = vec![subject];
    let state = start_battle_observed(setup, &mut trace).state;
    let discovery = discover_battle_acts_observed(&state, &mut trace);
    let battle_subject = discovery
        .available_acts()
        .iter()
        .find(|act| act.subject.kind == BattleSubjectKind::Spatial(subject))
        .map(|act| act.subject)
        .unwrap_or_else(|| {
            panic!("spatial route subject not discovered for {observed_action_taken}")
        });
    let request =
        BattleResolutionRequest::spatial_route(battle_subject, spatial_route_fill(subject))
            .expect("adapter selects a spatial route subject");
    let _result = resolve_battle_subject_observed(state, request, &mut trace);
    observed_reducer_route(&trace, &[expected_spatial_route_subject_family(subject)])
}

pub fn expected_witness(observed_action_taken: &str) -> LevelOneSpatialState {
    replay_observed_action(observed_action_taken)
}

pub fn expected_route(observed_action_taken: &str) -> Vec<ReducerRouteEvent> {
    let subject = spatial_route_subject(observed_action_taken);
    vec![
        ReducerRouteEvent::StartBattle {
            owner: ReducerRouteOwnerGroup::ActionEconomy,
        },
        ReducerRouteEvent::DiscoverBattleActs {
            subject: expected_spatial_route_subject_family(subject),
            holes: expected_spatial_route_holes(subject),
            owner: expected_spatial_route_discovery_owner(subject),
        },
        ReducerRouteEvent::ResolveBattleSubject {
            subject: expected_spatial_route_subject_family(subject),
            fill: ReducerRouteFillEvidence::FillKind(expected_spatial_route_fill_kind(subject)),
            outcome: ReducerRouteResolutionOutcome::Resolved,
            holes: Vec::new(),
            owner: expected_spatial_route_resolution_owner(subject),
        },
    ]
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

fn spatial_route_subject(observed_action_taken: &str) -> BattleSpatialRouteSubject {
    match observed_action_taken {
        "doDancingLightsMovableDimLight" => BattleSpatialRouteSubject::LightProjection,
        "doFaerieFireOutlineAdvantageInvisibleDimLight" => BattleSpatialRouteSubject::OutlineEffect,
        "doFeatherFallReactionMitigationLanding" => BattleSpatialRouteSubject::FallingMitigation,
        "doFogCloudAreaIdentityObscurementStrongWindCleanup" => {
            BattleSpatialRouteSubject::AreaObscurement
        }
        "doGreaseCastGroundHazardSavingThrows" => BattleSpatialRouteSubject::AreaHazard,
        "doGreaseMovementAndTurnTriggers" => BattleSpatialRouteSubject::MovementReplacement,
        "doJumpMovementReplacementLandingWitness" => BattleSpatialRouteSubject::MovementReplacement,
        "doLightObjectEmitterProjectionReplacementCleanup" => {
            BattleSpatialRouteSubject::ObjectBoundary
        }
        "doProduceFlameHeldLightProjectionHurlCleanup" => {
            BattleSpatialRouteSubject::LightProjection
        }
        "doThunderwaveSavePushObjectsBoom" => BattleSpatialRouteSubject::ForcedMovement,
        action => panic!("unsupported spatial route mbt::actionTaken {action}"),
    }
}

fn expected_spatial_route_subject_family(
    subject: BattleSpatialRouteSubject,
) -> ReducerRouteSubjectFamily {
    match subject {
        BattleSpatialRouteSubject::AreaHazard => ReducerRouteSubjectFamily::AreaHazard,
        BattleSpatialRouteSubject::AreaObscurement => ReducerRouteSubjectFamily::AreaObscurement,
        BattleSpatialRouteSubject::FallingMitigation => {
            ReducerRouteSubjectFamily::FallingMitigation
        }
        BattleSpatialRouteSubject::ForcedMovement => ReducerRouteSubjectFamily::ForcedMovement,
        BattleSpatialRouteSubject::LightProjection => ReducerRouteSubjectFamily::LightProjection,
        BattleSpatialRouteSubject::MovementReplacement => {
            ReducerRouteSubjectFamily::MovementResource
        }
        BattleSpatialRouteSubject::ObjectBoundary => {
            ReducerRouteSubjectFamily::ObjectBoundaryEffect
        }
        BattleSpatialRouteSubject::OutlineEffect => ReducerRouteSubjectFamily::OutlineEffect,
    }
}

fn expected_spatial_route_holes(subject: BattleSpatialRouteSubject) -> Vec<ReducerRouteHoleKind> {
    match subject {
        BattleSpatialRouteSubject::AreaHazard => {
            vec![ReducerRouteHoleKind::SavingThrowOutcome]
        }
        BattleSpatialRouteSubject::AreaObscurement
        | BattleSpatialRouteSubject::LightProjection
        | BattleSpatialRouteSubject::ObjectBoundary
        | BattleSpatialRouteSubject::OutlineEffect => vec![ReducerRouteHoleKind::TargetChoice],
        BattleSpatialRouteSubject::FallingMitigation => {
            vec![ReducerRouteHoleKind::InterruptDecision]
        }
        BattleSpatialRouteSubject::ForcedMovement
        | BattleSpatialRouteSubject::MovementReplacement => vec![ReducerRouteHoleKind::Movement],
    }
}

fn expected_spatial_route_fill_kind(subject: BattleSpatialRouteSubject) -> ReducerRouteFillKind {
    match subject {
        BattleSpatialRouteSubject::AreaHazard => ReducerRouteFillKind::SavingThrowOutcome,
        BattleSpatialRouteSubject::AreaObscurement
        | BattleSpatialRouteSubject::LightProjection
        | BattleSpatialRouteSubject::ObjectBoundary
        | BattleSpatialRouteSubject::OutlineEffect => ReducerRouteFillKind::TargetChoice,
        BattleSpatialRouteSubject::FallingMitigation => ReducerRouteFillKind::UnitFeatureDecision,
        BattleSpatialRouteSubject::ForcedMovement
        | BattleSpatialRouteSubject::MovementReplacement => ReducerRouteFillKind::Movement,
    }
}

fn expected_spatial_route_discovery_owner(
    subject: BattleSpatialRouteSubject,
) -> ReducerRouteOwnerGroup {
    match subject {
        BattleSpatialRouteSubject::AreaHazard | BattleSpatialRouteSubject::AreaObscurement => {
            ReducerRouteOwnerGroup::AreaShape
        }
        BattleSpatialRouteSubject::FallingMitigation => ReducerRouteOwnerGroup::InterruptStack,
        BattleSpatialRouteSubject::ForcedMovement
        | BattleSpatialRouteSubject::MovementReplacement => {
            ReducerRouteOwnerGroup::MovementResource
        }
        BattleSpatialRouteSubject::LightProjection | BattleSpatialRouteSubject::OutlineEffect => {
            ReducerRouteOwnerGroup::ActiveEffect
        }
        BattleSpatialRouteSubject::ObjectBoundary => ReducerRouteOwnerGroup::ObjectBoundary,
    }
}

fn expected_spatial_route_resolution_owner(
    subject: BattleSpatialRouteSubject,
) -> ReducerRouteOwnerGroup {
    match subject {
        BattleSpatialRouteSubject::AreaHazard => ReducerRouteOwnerGroup::ConditionLifecycle,
        BattleSpatialRouteSubject::AreaObscurement
        | BattleSpatialRouteSubject::FallingMitigation
        | BattleSpatialRouteSubject::LightProjection => ReducerRouteOwnerGroup::ActiveEffect,
        BattleSpatialRouteSubject::ForcedMovement
        | BattleSpatialRouteSubject::MovementReplacement => {
            ReducerRouteOwnerGroup::MovementResource
        }
        BattleSpatialRouteSubject::ObjectBoundary => ReducerRouteOwnerGroup::ObjectBoundary,
        BattleSpatialRouteSubject::OutlineEffect => ReducerRouteOwnerGroup::AttackRoll,
    }
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
