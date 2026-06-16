use crate::rules::sorcerer_metamagic::{
    resolve_careful_save_gated_damage, resolve_careful_save_gated_no_effect, CarefulSpellProtocol,
    CarefulSpellScenarioResult, CarefulSpellState,
};

pub const BRANCH_ACTIONS: [&str; 2] = [
    "doResolveCarefulSaveGatedDamage",
    "doResolveCarefulSaveGatedNoEffect",
];

pub fn replay_observed_action(observed_action_taken: &str) -> CarefulSpellState {
    match observed_action_taken {
        "doResolveCarefulSaveGatedDamage" => resolve_careful_save_gated_damage(),
        "doResolveCarefulSaveGatedNoEffect" => resolve_careful_save_gated_no_effect(),
        action => panic!("unsupported mbt::actionTaken {action}"),
    }
}

pub fn expected_witness(observed_action_taken: &str) -> CarefulSpellState {
    replay_observed_action(observed_action_taken)
}

pub fn projection_payload(state: &CarefulSpellState) -> String {
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

fn scenario_ref(result: CarefulSpellScenarioResult) -> &'static str {
    match result {
        CarefulSpellScenarioResult::Init => "init",
        CarefulSpellScenarioResult::CarefulSaveGatedDamage => "carefulSaveGatedDamage",
        CarefulSpellScenarioResult::CarefulSaveGatedNoEffect => "carefulSaveGatedNoEffect",
    }
}

fn protocol_ref(protocol: CarefulSpellProtocol) -> &'static str {
    match protocol {
        CarefulSpellProtocol::Init => "init",
        CarefulSpellProtocol::Resolved => "resolved",
    }
}
