use crate::rules::sorcerer_metamagic::{
    resolve_transmuted_save_gated_damage, resolve_transmuted_spell_attack, TransmutedSpellProtocol,
    TransmutedSpellScenarioResult, TransmutedSpellState,
};

pub const BRANCH_ACTIONS: [&str; 2] = [
    "doResolveTransmutedSaveGatedDamage",
    "doResolveTransmutedSpellAttack",
];

pub fn replay_observed_action(observed_action_taken: &str) -> TransmutedSpellState {
    match observed_action_taken {
        "doResolveTransmutedSaveGatedDamage" => resolve_transmuted_save_gated_damage(),
        "doResolveTransmutedSpellAttack" => resolve_transmuted_spell_attack(),
        action => panic!("unsupported mbt::actionTaken {action}"),
    }
}

pub fn expected_witness(observed_action_taken: &str) -> TransmutedSpellState {
    replay_observed_action(observed_action_taken)
}

pub fn projection_payload(state: &TransmutedSpellState) -> String {
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

fn scenario_ref(result: TransmutedSpellScenarioResult) -> &'static str {
    match result {
        TransmutedSpellScenarioResult::Init => "init",
        TransmutedSpellScenarioResult::TransmutedSaveGatedDamage => "transmutedSaveGatedDamage",
        TransmutedSpellScenarioResult::TransmutedSpellAttack => "transmutedSpellAttack",
    }
}

fn protocol_ref(protocol: TransmutedSpellProtocol) -> &'static str {
    match protocol {
        TransmutedSpellProtocol::Init => "init",
        TransmutedSpellProtocol::Resolved => "resolved",
    }
}
