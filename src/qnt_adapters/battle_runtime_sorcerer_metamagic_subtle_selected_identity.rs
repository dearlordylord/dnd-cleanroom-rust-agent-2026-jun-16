use crate::rules::sorcerer_metamagic::{
    reject_subtle_false_life_without_sorcery_points, resolve_subtle_false_life,
    SubtleSpellProtocol, SubtleSpellScenarioResult, SubtleSpellState,
};

pub const BRANCH_ACTIONS: [&str; 2] = [
    "doRejectSubtleFalseLifeWithoutSorceryPoints",
    "doResolveSubtleFalseLife",
];

pub fn replay_observed_action(observed_action_taken: &str) -> SubtleSpellState {
    match observed_action_taken {
        "doRejectSubtleFalseLifeWithoutSorceryPoints" => {
            reject_subtle_false_life_without_sorcery_points()
        }
        "doResolveSubtleFalseLife" => resolve_subtle_false_life(),
        action => panic!("unsupported mbt::actionTaken {action}"),
    }
}

pub fn expected_witness(observed_action_taken: &str) -> SubtleSpellState {
    replay_observed_action(observed_action_taken)
}

pub fn projection_payload(state: &SubtleSpellState) -> String {
    [
        format!("verbalSuppressed={}", state.verbal_suppressed),
        format!("somaticSuppressed={}", state.somatic_suppressed),
        format!("materialSuppressed={}", state.material_suppressed),
        format!("materialPreserved={}", state.material_preserved),
        format!("sorceryPointsRemaining={}", state.sorcery_points_remaining),
        format!("tempHp={}", state.temporary_hit_points),
        format!("scenarioResult={}", scenario_ref(state.scenario_result)),
        format!("protocolResult={}", protocol_ref(state.protocol)),
        format!(
            "protocolInvalidReason={}",
            invalid_reason_ref(state.protocol)
        ),
        "protocolHoles=none".to_string(),
    ]
    .join("\n")
}

fn scenario_ref(result: SubtleSpellScenarioResult) -> &'static str {
    match result {
        SubtleSpellScenarioResult::Init => "init",
        SubtleSpellScenarioResult::SubtleFalseLife => "subtleFalseLife",
        SubtleSpellScenarioResult::UnaffordableSubtleFalseLife => "unaffordableSubtleFalseLife",
    }
}

fn protocol_ref(protocol: SubtleSpellProtocol) -> &'static str {
    match protocol {
        SubtleSpellProtocol::Init => "init",
        SubtleSpellProtocol::Resolved => "resolved",
        SubtleSpellProtocol::InvalidUnsupportedActOption => "invalid",
    }
}

fn invalid_reason_ref(protocol: SubtleSpellProtocol) -> &'static str {
    match protocol {
        SubtleSpellProtocol::InvalidUnsupportedActOption => "WUnsupportedActOption",
        SubtleSpellProtocol::Init | SubtleSpellProtocol::Resolved => "none",
    }
}
