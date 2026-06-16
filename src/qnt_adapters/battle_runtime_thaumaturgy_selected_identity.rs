use crate::rules::thaumaturgy_selected_identity::{
    resolve_thaumaturgy_booming_voice, ThaumaturgyRollMode, ThaumaturgySelectedIdentityProtocol,
    ThaumaturgySelectedIdentityScenarioOutcome, ThaumaturgySelectedIdentityState,
};

pub const BRANCH_ACTIONS: [&str; 1] = ["doResolveThaumaturgyBoomingVoice"];

pub fn replay_observed_action(observed_action_taken: &str) -> ThaumaturgySelectedIdentityState {
    match observed_action_taken {
        "doResolveThaumaturgyBoomingVoice" => resolve_thaumaturgy_booming_voice(),
        action => panic!("unsupported mbt::actionTaken {action}"),
    }
}

pub fn expected_witness(observed_action_taken: &str) -> ThaumaturgySelectedIdentityState {
    replay_observed_action(observed_action_taken)
}

pub fn projection_payload(state: &ThaumaturgySelectedIdentityState) -> String {
    [
        format!("qCasterEffectCount={}", state.caster_effect_count),
        format!("qActionAvailable={}", state.action_available),
        format!(
            "qIntimidationRollMode={}",
            roll_mode_ref(state.intimidation_roll_mode)
        ),
        format!(
            "qWisdomIntimidationRollMode={}",
            roll_mode_ref(state.wisdom_intimidation_roll_mode)
        ),
        format!(
            "qPerceptionRollMode={}",
            roll_mode_ref(state.perception_roll_mode)
        ),
        format!(
            "qScenarioOutcome={}",
            scenario_outcome_ref(state.scenario_outcome)
        ),
        format!("protocolResult={}", protocol_result_ref(state.protocol)),
        format!(
            "protocolHoles={}",
            joined_or_none(&protocol_holes(state.protocol))
        ),
    ]
    .join("\n")
}

fn roll_mode_ref(mode: ThaumaturgyRollMode) -> &'static str {
    match mode {
        ThaumaturgyRollMode::Normal => "normal",
        ThaumaturgyRollMode::Advantage => "advantage",
    }
}

fn scenario_outcome_ref(outcome: ThaumaturgySelectedIdentityScenarioOutcome) -> &'static str {
    match outcome {
        ThaumaturgySelectedIdentityScenarioOutcome::Init => "Init",
        ThaumaturgySelectedIdentityScenarioOutcome::Resolved => "Resolved",
    }
}

fn protocol_result_ref(protocol: ThaumaturgySelectedIdentityProtocol) -> &'static str {
    match protocol {
        ThaumaturgySelectedIdentityProtocol::Init => "init",
        ThaumaturgySelectedIdentityProtocol::Resolved => "resolved",
    }
}

fn protocol_holes(protocol: ThaumaturgySelectedIdentityProtocol) -> Vec<&'static str> {
    match protocol {
        ThaumaturgySelectedIdentityProtocol::Init => vec!["WitnessProtocolHole"],
        ThaumaturgySelectedIdentityProtocol::Resolved => vec![],
    }
}

fn joined_or_none(values: &[&'static str]) -> String {
    if values.is_empty() {
        "none".to_string()
    } else {
        values.join(",")
    }
}
