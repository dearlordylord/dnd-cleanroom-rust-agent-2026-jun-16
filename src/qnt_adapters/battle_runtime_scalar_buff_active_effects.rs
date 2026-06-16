use crate::rules::scalar_buff_active_effects::{
    resolve_aid_scalar_buff, resolve_false_life_scalar_buff, resolve_longstrider_scalar_buff,
    resolve_shield_of_faith_scalar_buff, resolve_spider_climb_scalar_buff,
    stutter_scalar_buff_active_effect, ScalarBuffActiveEffectsState, ScalarBuffProtocol,
    ScalarBuffScenarioResult,
};

pub const BRANCH_ACTIONS: [&str; 6] = [
    "doCastShieldOfFaith",
    "doCastLongstrider",
    "doCastSpiderClimb",
    "doCastAid",
    "doCastFalseLife",
    "doStutter",
];

pub fn replay_observed_action(observed_action_taken: &str) -> ScalarBuffActiveEffectsState {
    match observed_action_taken {
        "doCastShieldOfFaith" => resolve_shield_of_faith_scalar_buff(),
        "doCastLongstrider" => resolve_longstrider_scalar_buff(),
        "doCastSpiderClimb" => resolve_spider_climb_scalar_buff(),
        "doCastAid" => resolve_aid_scalar_buff(),
        "doCastFalseLife" => resolve_false_life_scalar_buff(),
        "doStutter" => stutter_scalar_buff_active_effect(),
        action => panic!("unsupported mbt::actionTaken {action}"),
    }
}

pub fn expected_witness(observed_action_taken: &str) -> ScalarBuffActiveEffectsState {
    replay_observed_action(observed_action_taken)
}

pub fn projection_payload(state: &ScalarBuffActiveEffectsState) -> String {
    [
        format!("affectedArmorClass={}", state.affected_armor_class),
        format!("affectedSpeedFeet={}", state.affected_speed_feet),
        format!("affectedClimbSpeedFeet={}", state.affected_climb_speed_feet),
        format!(
            "affectedHitPointMaximum={}",
            state.affected_hit_point_maximum
        ),
        format!("affectedHitPoints={}", state.affected_hit_points),
        format!(
            "affectedTemporaryHitPoints={}",
            state.affected_temporary_hit_points
        ),
        format!("armorClassBonusActive={}", state.armor_class_bonus_active),
        format!("speedDeltaActive={}", state.speed_delta_active),
        format!(
            "specialSpeedGrantActive={}",
            state.special_speed_grant_active
        ),
        format!(
            "hitPointMaximumIncreaseActive={}",
            state.hit_point_maximum_increase_active
        ),
        format!("casterConcentrating={}", state.caster_concentrating),
        format!("scenarioResult={}", scenario_ref(state.scenario_result)),
        format!("protocolResult={}", protocol_ref(state.protocol)),
        "protocolHoles=none".to_string(),
    ]
    .join("\n")
}

fn scenario_ref(result: ScalarBuffScenarioResult) -> &'static str {
    match result {
        ScalarBuffScenarioResult::Init => "init",
        ScalarBuffScenarioResult::ShieldOfFaith => "shieldOfFaith",
        ScalarBuffScenarioResult::Longstrider => "longstrider",
        ScalarBuffScenarioResult::SpiderClimb => "spiderClimb",
        ScalarBuffScenarioResult::Aid => "aid",
        ScalarBuffScenarioResult::FalseLife => "falseLife",
    }
}

fn protocol_ref(protocol: ScalarBuffProtocol) -> &'static str {
    match protocol {
        ScalarBuffProtocol::Init => "init",
        ScalarBuffProtocol::Resolved => "resolved",
    }
}
