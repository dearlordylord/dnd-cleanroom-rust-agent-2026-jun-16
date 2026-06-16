use crate::rules::sorcerer_metamagic::{
    resolve_twinned_target_count, TwinnedSpellProtocol, TwinnedSpellScenarioResult,
    TwinnedSpellState,
};

pub const BRANCH_ACTIONS: [&str; 1] = ["doResolveTwinnedTargetCount"];

pub fn replay_observed_action(observed_action_taken: &str) -> TwinnedSpellState {
    match observed_action_taken {
        "doResolveTwinnedTargetCount" => resolve_twinned_target_count(),
        action => panic!("unsupported mbt::actionTaken {action}"),
    }
}

pub fn expected_witness(observed_action_taken: &str) -> TwinnedSpellState {
    replay_observed_action(observed_action_taken)
}

pub fn projection_payload(state: &TwinnedSpellState) -> String {
    [
        format!("magicActionAvailable={}", state.magic_action_available),
        format!("bonusActionAvailable={}", state.bonus_action_available),
        format!("sorceryPointsRemaining={}", state.sorcery_points_remaining),
        format!("targetHp={}", state.target_hit_points),
        format!(
            "targetActiveEffectCount={}",
            state.target_active_effect_count
        ),
        format!("scenarioResult={}", scenario_ref(state.scenario_result)),
        format!("protocolResult={}", protocol_ref(state.protocol)),
        "protocolHoles=none".to_string(),
    ]
    .join("\n")
}

fn scenario_ref(result: TwinnedSpellScenarioResult) -> &'static str {
    match result {
        TwinnedSpellScenarioResult::Init => "init",
        TwinnedSpellScenarioResult::TwinnedTargetCount => "twinnedTargetCount",
    }
}

fn protocol_ref(protocol: TwinnedSpellProtocol) -> &'static str {
    match protocol {
        TwinnedSpellProtocol::Init => "init",
        TwinnedSpellProtocol::Resolved => "resolved",
    }
}
