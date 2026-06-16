#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LevelOneSpatialScenarioOutcome {
    Init,
    DancingLightsMovableDimLight,
    FaerieFireOutlineAdvantageInvisibleDimLight,
    FeatherFallReactionMitigationLanding,
    FogCloudAreaIdentityObscurementStrongWindCleanup,
    GreaseCastGroundHazardSavingThrows,
    GreaseMovementAndTurnTriggers,
    JumpMovementReplacementLandingWitness,
    LightObjectEmitterProjectionReplacementCleanup,
    ProduceFlameHeldLightProjectionHurlCleanup,
    ThunderwaveSavePushObjectsBoom,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LevelOneSpatialProtocol {
    Init,
    Resolved,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Illumination {
    Darkness,
    DimLight,
    BrightLight,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SightObscurement {
    HeavilyObscured,
    LightlyObscured,
    Unobscured,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AttackRollMode {
    Normal,
    Advantage,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct LightProjection {
    pub emitter_count: i16,
    pub dim_light_emitter_count: i16,
    pub retained_identity_count: i16,
    pub object_admitted: bool,
    pub invalid_object_rejection_count: i16,
    pub duration_ticks: i16,
    pub bright_projection_illumination: Illumination,
    pub opaque_cover_illumination: Illumination,
    pub recast_replaced_prior_emitter: bool,
    pub duration_cleanup_cleared_emitter: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct FaerieFireProjection {
    pub outlined_creature_count: i16,
    pub outlined_object_count: i16,
    pub creature_attack_roll_mode: AttackRollMode,
    pub invisible_creature_attack_roll_mode: AttackRollMode,
    pub object_attack_roll_mode: AttackRollMode,
    pub target_invisible: bool,
    pub object_invisible_benefit_denied: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct FeatherFallProjection {
    pub trigger_offered: bool,
    pub unwitnessed_trigger_rejected: bool,
    pub reaction_spent: bool,
    pub slot_expended: bool,
    pub mitigated_target_count_before_landing: i16,
    pub landed_target_descent_rate_cap_feet_per_round: i16,
    pub landing_fall_damage_prevented: bool,
    pub landing_falling_prone_prevented: bool,
    pub landed_target_mitigation_cleared: bool,
    pub other_target_still_mitigated: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct FogCloudProjection {
    pub area_identity_retained: bool,
    pub heavily_obscured_zone_count: i16,
    pub radius_feet: i16,
    pub duration_ticks: i16,
    pub strong_wind_command_offered: bool,
    pub cleanup_cleared_effect: bool,
    pub cleanup_cleared_zone: bool,
    pub cleanup_cleared_concentration: bool,
    pub slot_expended: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct GreaseProjection {
    pub area_identity_retained: bool,
    pub active_hazard_count: i16,
    pub duration_ticks: i16,
    pub affected_target_outcome_count: i16,
    pub failed_target_prone: bool,
    pub succeeded_target_prone: bool,
    pub mismatched_affected_target_rejected: bool,
    pub difficult_terrain_movement_cost_feet: i16,
    pub movement_spent_feet: i16,
    pub mismatched_movement_area_rejected: bool,
    pub entry_save_offered: bool,
    pub entry_failed_target_prone: bool,
    pub entry_mismatched_target_rejected: bool,
    pub end_turn_save_offered: bool,
    pub end_turn_failed_target_prone: bool,
    pub end_turn_advanced_to_caster: bool,
    pub end_turn_mismatched_target_rejected: bool,
    pub slot_expended: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct JumpProjection {
    pub target_effect_installed: bool,
    pub movement_spent_feet: i16,
    pub used_marker_set: bool,
    pub same_turn_unavailable: bool,
    pub next_target_turn_available: bool,
    pub missing_landing_fact_rejected: bool,
    pub failed_landing_prone: bool,
    pub slot_expended: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ProduceFlameProjection {
    pub held_light_installed: bool,
    pub duration_ticks: i16,
    pub bright_projection_illumination: Illumination,
    pub hurl_offered: bool,
    pub hurl_target_damaged: bool,
    pub hurl_cleanup_cleared_emitter: bool,
    pub duration_cleanup_cleared_emitter: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ThunderwaveProjection {
    pub affected_target_outcome_count: i16,
    pub failed_pushed_target_damaged: bool,
    pub failed_blocked_target_damaged: bool,
    pub succeeded_target_half_damaged: bool,
    pub pushed_creature_disposition_count: i16,
    pub blocked_creature_disposition_count: i16,
    pub pushed_object_disposition_count: i16,
    pub blocked_object_disposition_count: i16,
    pub audible_boom_matched: bool,
    pub missing_area_facts_rejected: bool,
    pub mismatched_boom_rejected: bool,
    pub slot_expended: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct VisionProjection {
    pub projected_illumination: Illumination,
    pub ordinary_sight_obscurement: SightObscurement,
    pub darkvision_sight_obscurement: SightObscurement,
    pub mismatched_witness_illumination: Illumination,
    pub obscurement_zone_count: i16,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct LevelOneSpatialState {
    pub light: LightProjection,
    pub faerie_fire: FaerieFireProjection,
    pub feather_fall: FeatherFallProjection,
    pub fog_cloud: FogCloudProjection,
    pub grease: GreaseProjection,
    pub jump: JumpProjection,
    pub produce_flame: ProduceFlameProjection,
    pub thunderwave: ThunderwaveProjection,
    pub vision: VisionProjection,
    pub caster_concentrating: bool,
    pub magic_action_available: bool,
    pub bonus_action_available: bool,
    pub scenario_outcome: LevelOneSpatialScenarioOutcome,
    pub protocol: LevelOneSpatialProtocol,
}

#[must_use]
pub fn level_one_spatial_initial_state() -> LevelOneSpatialState {
    LevelOneSpatialState {
        light: LightProjection {
            emitter_count: 0,
            dim_light_emitter_count: 0,
            retained_identity_count: 0,
            object_admitted: false,
            invalid_object_rejection_count: 0,
            duration_ticks: 0,
            bright_projection_illumination: Illumination::Darkness,
            opaque_cover_illumination: Illumination::Darkness,
            recast_replaced_prior_emitter: false,
            duration_cleanup_cleared_emitter: false,
        },
        faerie_fire: FaerieFireProjection {
            outlined_creature_count: 0,
            outlined_object_count: 0,
            creature_attack_roll_mode: AttackRollMode::Normal,
            invisible_creature_attack_roll_mode: AttackRollMode::Normal,
            object_attack_roll_mode: AttackRollMode::Normal,
            target_invisible: false,
            object_invisible_benefit_denied: false,
        },
        feather_fall: FeatherFallProjection {
            trigger_offered: false,
            unwitnessed_trigger_rejected: false,
            reaction_spent: false,
            slot_expended: false,
            mitigated_target_count_before_landing: 0,
            landed_target_descent_rate_cap_feet_per_round: 0,
            landing_fall_damage_prevented: false,
            landing_falling_prone_prevented: false,
            landed_target_mitigation_cleared: false,
            other_target_still_mitigated: false,
        },
        fog_cloud: FogCloudProjection {
            area_identity_retained: false,
            heavily_obscured_zone_count: 0,
            radius_feet: 0,
            duration_ticks: 0,
            strong_wind_command_offered: false,
            cleanup_cleared_effect: false,
            cleanup_cleared_zone: false,
            cleanup_cleared_concentration: false,
            slot_expended: false,
        },
        grease: GreaseProjection {
            area_identity_retained: false,
            active_hazard_count: 0,
            duration_ticks: 0,
            affected_target_outcome_count: 0,
            failed_target_prone: false,
            succeeded_target_prone: false,
            mismatched_affected_target_rejected: false,
            difficult_terrain_movement_cost_feet: 0,
            movement_spent_feet: 0,
            mismatched_movement_area_rejected: false,
            entry_save_offered: false,
            entry_failed_target_prone: false,
            entry_mismatched_target_rejected: false,
            end_turn_save_offered: false,
            end_turn_failed_target_prone: false,
            end_turn_advanced_to_caster: false,
            end_turn_mismatched_target_rejected: false,
            slot_expended: false,
        },
        jump: JumpProjection {
            target_effect_installed: false,
            movement_spent_feet: 0,
            used_marker_set: false,
            same_turn_unavailable: false,
            next_target_turn_available: false,
            missing_landing_fact_rejected: false,
            failed_landing_prone: false,
            slot_expended: false,
        },
        produce_flame: ProduceFlameProjection {
            held_light_installed: false,
            duration_ticks: 0,
            bright_projection_illumination: Illumination::Darkness,
            hurl_offered: false,
            hurl_target_damaged: false,
            hurl_cleanup_cleared_emitter: false,
            duration_cleanup_cleared_emitter: false,
        },
        thunderwave: ThunderwaveProjection {
            affected_target_outcome_count: 0,
            failed_pushed_target_damaged: false,
            failed_blocked_target_damaged: false,
            succeeded_target_half_damaged: false,
            pushed_creature_disposition_count: 0,
            blocked_creature_disposition_count: 0,
            pushed_object_disposition_count: 0,
            blocked_object_disposition_count: 0,
            audible_boom_matched: false,
            missing_area_facts_rejected: false,
            mismatched_boom_rejected: false,
            slot_expended: false,
        },
        vision: VisionProjection {
            projected_illumination: Illumination::Darkness,
            ordinary_sight_obscurement: SightObscurement::HeavilyObscured,
            darkvision_sight_obscurement: SightObscurement::LightlyObscured,
            mismatched_witness_illumination: Illumination::Darkness,
            obscurement_zone_count: 0,
        },
        caster_concentrating: false,
        magic_action_available: true,
        bonus_action_available: true,
        scenario_outcome: LevelOneSpatialScenarioOutcome::Init,
        protocol: LevelOneSpatialProtocol::Init,
    }
}

#[must_use]
pub fn project_dancing_lights_movable_dim_light() -> LevelOneSpatialState {
    // RAW: cleanroom-input/raw/srd-5.2.1/Spells/Descriptions-A-D.md
    // "Dancing Lights"; QNT: battle-runtime-level1-spatial-witness-selected-identity.mbt.qnt.
    let mut state = level_one_spatial_initial_state();
    state.light.emitter_count = 2;
    state.light.dim_light_emitter_count = 2;
    state.light.retained_identity_count = 2;
    state.vision.projected_illumination = Illumination::DimLight;
    state.vision.ordinary_sight_obscurement = SightObscurement::LightlyObscured;
    state.vision.darkvision_sight_obscurement = SightObscurement::Unobscured;
    state.caster_concentrating = true;
    state.magic_action_available = false;
    state.bonus_action_available = false;
    resolved(
        state,
        LevelOneSpatialScenarioOutcome::DancingLightsMovableDimLight,
    )
}

#[must_use]
pub fn project_faerie_fire_outline_advantage_invisible_dim_light() -> LevelOneSpatialState {
    // RAW: cleanroom-input/raw/srd-5.2.1/Spells/Descriptions-E-L.md
    // "Faerie Fire"; QNT: battle-runtime-light.qnt.
    let mut state = level_one_spatial_initial_state();
    state.light.emitter_count = 2;
    state.light.dim_light_emitter_count = 2;
    state.faerie_fire.outlined_creature_count = 1;
    state.faerie_fire.outlined_object_count = 1;
    state.faerie_fire.creature_attack_roll_mode = AttackRollMode::Advantage;
    state.faerie_fire.invisible_creature_attack_roll_mode = AttackRollMode::Advantage;
    state.faerie_fire.object_attack_roll_mode = AttackRollMode::Advantage;
    state.faerie_fire.target_invisible = true;
    state.faerie_fire.object_invisible_benefit_denied = true;
    state.vision.projected_illumination = Illumination::DimLight;
    state.vision.ordinary_sight_obscurement = SightObscurement::LightlyObscured;
    state.vision.darkvision_sight_obscurement = SightObscurement::Unobscured;
    state.caster_concentrating = true;
    resolved(
        state,
        LevelOneSpatialScenarioOutcome::FaerieFireOutlineAdvantageInvisibleDimLight,
    )
}

#[must_use]
pub fn project_feather_fall_reaction_mitigation_landing() -> LevelOneSpatialState {
    // RAW: cleanroom-input/raw/srd-5.2.1/Spells/Descriptions-E-L.md
    // "Feather Fall"; QNT: battle-runtime-feather-fall.qnt.
    let mut state = level_one_spatial_initial_state();
    state.feather_fall = FeatherFallProjection {
        trigger_offered: true,
        unwitnessed_trigger_rejected: true,
        reaction_spent: true,
        slot_expended: true,
        mitigated_target_count_before_landing: 2,
        landed_target_descent_rate_cap_feet_per_round: 60,
        landing_fall_damage_prevented: true,
        landing_falling_prone_prevented: true,
        landed_target_mitigation_cleared: true,
        other_target_still_mitigated: true,
    };
    resolved(
        state,
        LevelOneSpatialScenarioOutcome::FeatherFallReactionMitigationLanding,
    )
}

#[must_use]
pub fn project_fog_cloud_area_identity_obscurement_strong_wind_cleanup() -> LevelOneSpatialState {
    // RAW: cleanroom-input/raw/srd-5.2.1/Spells/Descriptions-E-L.md
    // "Fog Cloud"; QNT: battle-runtime-level1-spatial-witness-selected-identity.mbt.qnt.
    let mut state = level_one_spatial_initial_state();
    state.fog_cloud = FogCloudProjection {
        area_identity_retained: true,
        heavily_obscured_zone_count: 1,
        radius_feet: 20,
        duration_ticks: 600,
        strong_wind_command_offered: true,
        cleanup_cleared_effect: true,
        cleanup_cleared_zone: true,
        cleanup_cleared_concentration: true,
        slot_expended: true,
    };
    state.magic_action_available = false;
    resolved(
        state,
        LevelOneSpatialScenarioOutcome::FogCloudAreaIdentityObscurementStrongWindCleanup,
    )
}

#[must_use]
pub fn project_grease_cast_ground_hazard_saving_throws() -> LevelOneSpatialState {
    // RAW: cleanroom-input/raw/srd-5.2.1/Spells/Descriptions-E-L.md
    // "Grease"; QNT: battle-runtime-level1-spatial-witness-selected-identity.mbt.qnt.
    let mut state = level_one_spatial_initial_state();
    state.grease.area_identity_retained = true;
    state.grease.active_hazard_count = 1;
    state.grease.duration_ticks = 10;
    state.grease.affected_target_outcome_count = 2;
    state.grease.failed_target_prone = true;
    state.grease.mismatched_affected_target_rejected = true;
    state.grease.slot_expended = true;
    state.magic_action_available = false;
    resolved(
        state,
        LevelOneSpatialScenarioOutcome::GreaseCastGroundHazardSavingThrows,
    )
}

#[must_use]
pub fn project_grease_movement_and_turn_triggers() -> LevelOneSpatialState {
    // RAW: cleanroom-input/raw/srd-5.2.1/Spells/Descriptions-E-L.md
    // "Grease"; QNT: battle-runtime-area-trigger-timing.qnt.
    let mut state = level_one_spatial_initial_state();
    state.grease = GreaseProjection {
        area_identity_retained: true,
        active_hazard_count: 1,
        duration_ticks: 10,
        affected_target_outcome_count: 2,
        failed_target_prone: true,
        succeeded_target_prone: false,
        mismatched_affected_target_rejected: true,
        difficult_terrain_movement_cost_feet: 15,
        movement_spent_feet: 15,
        mismatched_movement_area_rejected: true,
        entry_save_offered: true,
        entry_failed_target_prone: true,
        entry_mismatched_target_rejected: true,
        end_turn_save_offered: true,
        end_turn_failed_target_prone: true,
        end_turn_advanced_to_caster: true,
        end_turn_mismatched_target_rejected: true,
        slot_expended: true,
    };
    resolved(
        state,
        LevelOneSpatialScenarioOutcome::GreaseMovementAndTurnTriggers,
    )
}

#[must_use]
pub fn project_jump_movement_replacement_landing_witness() -> LevelOneSpatialState {
    // RAW: cleanroom-input/raw/srd-5.2.1/Spells/Descriptions-E-L.md
    // "Jump"; QNT: battle-runtime-jump-movement.qnt.
    let mut state = level_one_spatial_initial_state();
    state.jump = JumpProjection {
        target_effect_installed: true,
        movement_spent_feet: 10,
        used_marker_set: true,
        same_turn_unavailable: true,
        next_target_turn_available: true,
        missing_landing_fact_rejected: true,
        failed_landing_prone: true,
        slot_expended: true,
    };
    resolved(
        state,
        LevelOneSpatialScenarioOutcome::JumpMovementReplacementLandingWitness,
    )
}

#[must_use]
pub fn project_light_object_emitter_projection_replacement_cleanup() -> LevelOneSpatialState {
    // RAW: cleanroom-input/raw/srd-5.2.1/Spells/Descriptions-E-L.md
    // "Light"; QNT: battle-runtime-light.qnt.
    let mut state = level_one_spatial_initial_state();
    state.light = LightProjection {
        emitter_count: 1,
        dim_light_emitter_count: 0,
        retained_identity_count: 0,
        object_admitted: true,
        invalid_object_rejection_count: 3,
        duration_ticks: 600,
        bright_projection_illumination: Illumination::BrightLight,
        opaque_cover_illumination: Illumination::Darkness,
        recast_replaced_prior_emitter: true,
        duration_cleanup_cleared_emitter: true,
    };
    state.vision.projected_illumination = Illumination::DimLight;
    state.vision.ordinary_sight_obscurement = SightObscurement::LightlyObscured;
    state.vision.darkvision_sight_obscurement = SightObscurement::Unobscured;
    state.magic_action_available = false;
    resolved(
        state,
        LevelOneSpatialScenarioOutcome::LightObjectEmitterProjectionReplacementCleanup,
    )
}

#[must_use]
pub fn project_produce_flame_held_light_projection_hurl_cleanup() -> LevelOneSpatialState {
    // RAW: cleanroom-input/raw/srd-5.2.1/Spells/Descriptions-M-P.md
    // "Produce Flame"; QNT: battle-runtime-light.qnt.
    let mut state = level_one_spatial_initial_state();
    state.light.emitter_count = 1;
    state.produce_flame = ProduceFlameProjection {
        held_light_installed: true,
        duration_ticks: 100,
        bright_projection_illumination: Illumination::BrightLight,
        hurl_offered: true,
        hurl_target_damaged: true,
        hurl_cleanup_cleared_emitter: true,
        duration_cleanup_cleared_emitter: true,
    };
    state.vision.projected_illumination = Illumination::DimLight;
    state.vision.ordinary_sight_obscurement = SightObscurement::LightlyObscured;
    state.vision.darkvision_sight_obscurement = SightObscurement::Unobscured;
    state.bonus_action_available = false;
    resolved(
        state,
        LevelOneSpatialScenarioOutcome::ProduceFlameHeldLightProjectionHurlCleanup,
    )
}

#[must_use]
pub fn project_thunderwave_save_push_objects_boom() -> LevelOneSpatialState {
    // RAW: cleanroom-input/raw/srd-5.2.1/Spells/Descriptions-S-Z.md
    // "Thunderwave"; QNT: battle-runtime-spell-bridge-examples.qnt.
    let mut state = level_one_spatial_initial_state();
    state.thunderwave = ThunderwaveProjection {
        affected_target_outcome_count: 3,
        failed_pushed_target_damaged: true,
        failed_blocked_target_damaged: true,
        succeeded_target_half_damaged: true,
        pushed_creature_disposition_count: 1,
        blocked_creature_disposition_count: 1,
        pushed_object_disposition_count: 1,
        blocked_object_disposition_count: 1,
        audible_boom_matched: true,
        missing_area_facts_rejected: true,
        mismatched_boom_rejected: true,
        slot_expended: true,
    };
    state.magic_action_available = false;
    resolved(
        state,
        LevelOneSpatialScenarioOutcome::ThunderwaveSavePushObjectsBoom,
    )
}

fn resolved(
    mut state: LevelOneSpatialState,
    outcome: LevelOneSpatialScenarioOutcome,
) -> LevelOneSpatialState {
    state.scenario_outcome = outcome;
    state.protocol = LevelOneSpatialProtocol::Resolved;
    state
}
