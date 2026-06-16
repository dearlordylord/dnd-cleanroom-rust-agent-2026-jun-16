use crate::rules::sorcerer_metamagic::{
    resolve_heightened_grease_entry_save, resolve_heightened_gust_of_wind_end_turn_save,
    resolve_heightened_hideous_laughter, resolve_heightened_save_gated_condition_end_turn_save,
    resolve_heightened_save_gated_damage, HeightenedSpellProtocol, HeightenedSpellScenarioResult,
    HeightenedSpellState,
};

pub const BRANCH_ACTIONS: [&str; 5] = [
    "doResolveHeightenedSaveGatedDamage",
    "doResolveHeightenedHideousLaughter",
    "doResolveHeightenedGreaseEntrySave",
    "doResolveHeightenedGustOfWindEndTurnSave",
    "doResolveHeightenedSaveGatedConditionEndTurnSave",
];

pub fn replay_observed_action(observed_action_taken: &str) -> HeightenedSpellState {
    match observed_action_taken {
        "doResolveHeightenedSaveGatedDamage" => resolve_heightened_save_gated_damage(),
        "doResolveHeightenedHideousLaughter" => resolve_heightened_hideous_laughter(),
        "doResolveHeightenedGreaseEntrySave" => resolve_heightened_grease_entry_save(),
        "doResolveHeightenedGustOfWindEndTurnSave" => {
            resolve_heightened_gust_of_wind_end_turn_save()
        }
        "doResolveHeightenedSaveGatedConditionEndTurnSave" => {
            resolve_heightened_save_gated_condition_end_turn_save()
        }
        action => panic!("unsupported mbt::actionTaken {action}"),
    }
}

pub fn expected_witness(observed_action_taken: &str) -> HeightenedSpellState {
    replay_observed_action(observed_action_taken)
}

pub fn projection_payload(state: &HeightenedSpellState) -> String {
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

fn scenario_ref(result: HeightenedSpellScenarioResult) -> &'static str {
    match result {
        HeightenedSpellScenarioResult::Init => "init",
        HeightenedSpellScenarioResult::HeightenedSaveGatedDamage => "heightenedSaveGatedDamage",
        HeightenedSpellScenarioResult::HeightenedHideousLaughter => "heightenedHideousLaughter",
        HeightenedSpellScenarioResult::HeightenedGreaseEntrySave => "heightenedGreaseEntrySave",
        HeightenedSpellScenarioResult::HeightenedGustOfWindEndTurnSave => {
            "heightenedGustOfWindEndTurnSave"
        }
        HeightenedSpellScenarioResult::HeightenedSaveGatedConditionEndTurnSave => {
            "heightenedSaveGatedConditionEndTurnSave"
        }
    }
}

fn protocol_ref(protocol: HeightenedSpellProtocol) -> &'static str {
    match protocol {
        HeightenedSpellProtocol::Init => "init",
        HeightenedSpellProtocol::Resolved => "resolved",
    }
}
