use crate::rules::sorcerer_metamagic::{
    resolve_distant_object_light, DistantSpellProtocol, DistantSpellScenarioResult,
    DistantSpellState,
};

pub const BRANCH_ACTIONS: [&str; 1] = ["doResolveDistantObjectLight"];

pub fn replay_observed_action(observed_action_taken: &str) -> DistantSpellState {
    match observed_action_taken {
        "doResolveDistantObjectLight" => resolve_distant_object_light(),
        action => panic!("unsupported mbt::actionTaken {action}"),
    }
}

pub fn expected_witness(observed_action_taken: &str) -> DistantSpellState {
    replay_observed_action(observed_action_taken)
}

pub fn projection_payload(state: &DistantSpellState) -> String {
    [
        format!("sorceryPointsRemaining={}", state.sorcery_points_remaining),
        format!("lightEmitterCount={}", state.light_emitter_count),
        format!("brightRadiusFeet={}", state.bright_radius_feet),
        format!("dimAdditionalFeet={}", state.dim_additional_feet),
        format!("scenarioResult={}", scenario_ref(state.scenario_result)),
        format!("protocolResult={}", protocol_ref(state.protocol)),
        "protocolHoles=none".to_string(),
    ]
    .join("\n")
}

fn scenario_ref(result: DistantSpellScenarioResult) -> &'static str {
    match result {
        DistantSpellScenarioResult::Init => "init",
        DistantSpellScenarioResult::DistantObjectLight => "distantObjectLight",
    }
}

fn protocol_ref(protocol: DistantSpellProtocol) -> &'static str {
    match protocol {
        DistantSpellProtocol::Init => "init",
        DistantSpellProtocol::Resolved => "resolved",
    }
}
