use crate::rules::sorcerer_metamagic::{
    resolve_empowered_spell_damage_reroll, EmpoweredSpellProtocol, EmpoweredSpellScenarioResult,
    EmpoweredSpellState,
};

pub const BRANCH_ACTIONS: [&str; 1] = ["doResolveEmpoweredSpellDamageReroll"];

pub fn replay_observed_action(observed_action_taken: &str) -> EmpoweredSpellState {
    match observed_action_taken {
        "doResolveEmpoweredSpellDamageReroll" => resolve_empowered_spell_damage_reroll(),
        action => panic!("unsupported mbt::actionTaken {action}"),
    }
}

pub fn expected_witness(observed_action_taken: &str) -> EmpoweredSpellState {
    replay_observed_action(observed_action_taken)
}

pub fn projection_payload(state: &EmpoweredSpellState) -> String {
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

fn scenario_ref(result: EmpoweredSpellScenarioResult) -> &'static str {
    match result {
        EmpoweredSpellScenarioResult::Init => "init",
        EmpoweredSpellScenarioResult::EmpoweredSpellDamageReroll => "empoweredSpellDamageReroll",
    }
}

fn protocol_ref(protocol: EmpoweredSpellProtocol) -> &'static str {
    match protocol {
        EmpoweredSpellProtocol::Init => "init",
        EmpoweredSpellProtocol::Resolved => "resolved",
    }
}
