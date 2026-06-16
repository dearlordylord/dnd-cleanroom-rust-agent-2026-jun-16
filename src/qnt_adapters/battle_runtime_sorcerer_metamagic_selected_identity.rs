use crate::rules::sorcerer_metamagic::{
    resolve_quickened_save_gated_damage, QuickenedMetamagicProtocol,
    QuickenedMetamagicScenarioResult, QuickenedMetamagicState,
};

pub const BRANCH_ACTIONS: [&str; 1] = ["doResolveQuickenedSaveGatedDamage"];

pub fn replay_observed_action(observed_action_taken: &str) -> QuickenedMetamagicState {
    match observed_action_taken {
        "doResolveQuickenedSaveGatedDamage" => resolve_quickened_save_gated_damage(),
        action => panic!("unsupported mbt::actionTaken {action}"),
    }
}

pub fn expected_witness(observed_action_taken: &str) -> QuickenedMetamagicState {
    replay_observed_action(observed_action_taken)
}

pub fn projection_payload(state: &QuickenedMetamagicState) -> String {
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

fn scenario_ref(result: QuickenedMetamagicScenarioResult) -> &'static str {
    match result {
        QuickenedMetamagicScenarioResult::Init => "init",
        QuickenedMetamagicScenarioResult::QuickenedSaveGatedDamage => "quickenedSaveGatedDamage",
        QuickenedMetamagicScenarioResult::QuickenedSpellAttack => "quickenedSpellAttack",
        QuickenedMetamagicScenarioResult::QuickenedSpellAttackSequence => {
            "quickenedSpellAttackSequence"
        }
    }
}

fn protocol_ref(protocol: QuickenedMetamagicProtocol) -> &'static str {
    match protocol {
        QuickenedMetamagicProtocol::Init => "init",
        QuickenedMetamagicProtocol::Resolved => "resolved",
    }
}
