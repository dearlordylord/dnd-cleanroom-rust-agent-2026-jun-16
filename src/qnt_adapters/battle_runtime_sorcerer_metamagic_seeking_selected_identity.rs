use crate::rules::sorcerer_metamagic::{
    resolve_seeking_spell_attack_reroll, SeekingSpellProtocol, SeekingSpellScenarioResult,
    SeekingSpellState,
};

pub const BRANCH_ACTIONS: [&str; 1] = ["doResolveSeekingSpellAttackReroll"];

pub fn replay_observed_action(observed_action_taken: &str) -> SeekingSpellState {
    match observed_action_taken {
        "doResolveSeekingSpellAttackReroll" => resolve_seeking_spell_attack_reroll(),
        action => panic!("unsupported mbt::actionTaken {action}"),
    }
}

pub fn expected_witness(observed_action_taken: &str) -> SeekingSpellState {
    replay_observed_action(observed_action_taken)
}

pub fn projection_payload(state: &SeekingSpellState) -> String {
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

fn scenario_ref(result: SeekingSpellScenarioResult) -> &'static str {
    match result {
        SeekingSpellScenarioResult::Init => "init",
        SeekingSpellScenarioResult::SeekingSpellAttackReroll => "seekingSpellAttackReroll",
    }
}

fn protocol_ref(protocol: SeekingSpellProtocol) -> &'static str {
    match protocol {
        SeekingSpellProtocol::Init => "init",
        SeekingSpellProtocol::Resolved => "resolved",
    }
}
