use crate::rules::sorcerer_metamagic::{
    resolve_extended_creature_size_increase, ExtendedSpellConcentrationSaveMode,
    ExtendedSpellProtocol, ExtendedSpellScenarioResult, ExtendedSpellState,
};

pub const BRANCH_ACTIONS: [&str; 1] = ["doResolveExtendedCreatureSizeIncrease"];

pub fn replay_observed_action(observed_action_taken: &str) -> ExtendedSpellState {
    match observed_action_taken {
        "doResolveExtendedCreatureSizeIncrease" => resolve_extended_creature_size_increase(),
        action => panic!("unsupported mbt::actionTaken {action}"),
    }
}

pub fn expected_witness(observed_action_taken: &str) -> ExtendedSpellState {
    replay_observed_action(observed_action_taken)
}

pub fn projection_payload(state: &ExtendedSpellState) -> String {
    [
        format!("sorceryPointsRemaining={}", state.sorcery_points_remaining),
        format!("durationTicks={}", state.duration_ticks),
        format!(
            "concentrationSavingThrowMode={}",
            concentration_mode_ref(state.concentration_saving_throw_mode)
        ),
        format!("scenarioResult={}", scenario_ref(state.scenario_result)),
        format!("protocolResult={}", protocol_ref(state.protocol)),
        "protocolHoles=none".to_string(),
    ]
    .join("\n")
}

fn concentration_mode_ref(mode: ExtendedSpellConcentrationSaveMode) -> &'static str {
    match mode {
        ExtendedSpellConcentrationSaveMode::Normal => "normal",
        ExtendedSpellConcentrationSaveMode::Advantage => "advantage",
    }
}

fn scenario_ref(result: ExtendedSpellScenarioResult) -> &'static str {
    match result {
        ExtendedSpellScenarioResult::Init => "init",
        ExtendedSpellScenarioResult::ExtendedCreatureSizeIncrease => "extendedCreatureSizeIncrease",
    }
}

fn protocol_ref(protocol: ExtendedSpellProtocol) -> &'static str {
    match protocol {
        ExtendedSpellProtocol::Init => "init",
        ExtendedSpellProtocol::Resolved => "resolved",
    }
}
