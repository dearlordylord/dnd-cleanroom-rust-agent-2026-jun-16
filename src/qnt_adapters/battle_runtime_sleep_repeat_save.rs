use crate::rules::sleep_repeat_save::{
    break_sleep_concentration_before_repeat, discover_sleep_repeat_save,
    end_caster_turn_after_sleep_concentration_break, end_caster_turn_with_sleep,
    end_target_turn_after_sleep_concentration_break, fill_sleep_initial_save_failure,
    fill_sleep_repeat_save_failure, fill_sleep_repeat_save_success, SleepRepeatSaveHole,
    SleepRepeatSaveProtocol, SleepRepeatSaveState, SleepRepeatSaveTurnRole,
};

pub const BRANCH_ACTIONS: [&str; 8] = [
    "doFillInitialSaveFailure",
    "doBreakConcentrationBeforeRepeat",
    "doEndCasterTurn",
    "doEndCasterTurnAfterConcentrationBreak",
    "doEndTargetTurnAfterConcentrationBreak",
    "doDiscoverRepeatSave",
    "doFillRepeatSaveSuccess",
    "doFillRepeatSaveFailure",
];

pub fn replay_observed_action(observed_action_taken: &str) -> SleepRepeatSaveState {
    match observed_action_taken {
        "doFillInitialSaveFailure" => fill_sleep_initial_save_failure(),
        "doBreakConcentrationBeforeRepeat" => break_sleep_concentration_before_repeat(),
        "doEndCasterTurn" => end_caster_turn_with_sleep(),
        "doEndCasterTurnAfterConcentrationBreak" => {
            end_caster_turn_after_sleep_concentration_break()
        }
        "doEndTargetTurnAfterConcentrationBreak" => {
            end_target_turn_after_sleep_concentration_break()
        }
        "doDiscoverRepeatSave" => discover_sleep_repeat_save(),
        "doFillRepeatSaveSuccess" => fill_sleep_repeat_save_success(),
        "doFillRepeatSaveFailure" => fill_sleep_repeat_save_failure(),
        action => panic!("unsupported mbt::actionTaken {action}"),
    }
}

pub fn expected_witness(observed_action_taken: &str) -> SleepRepeatSaveState {
    replay_observed_action(observed_action_taken)
}

pub fn projection_payload(state: &SleepRepeatSaveState) -> String {
    [
        format!("currentTurnRole={}", turn_role_ref(state.current_turn_role)),
        format!("targetIncapacitated={}", state.target_incapacitated),
        format!("targetUnconscious={}", state.target_unconscious),
        format!("targetProne={}", state.target_prone),
        format!("casterConcentrating={}", state.caster_concentrating),
        format!("actionAvailable={}", state.action_available),
        format!("protocolResult={}", protocol_ref(state.protocol)),
        format!("protocolHoles={}", joined_holes(&state.protocol_holes)),
    ]
    .join("\n")
}

fn turn_role_ref(role: SleepRepeatSaveTurnRole) -> &'static str {
    match role {
        SleepRepeatSaveTurnRole::Caster => "caster",
        SleepRepeatSaveTurnRole::Target => "target",
    }
}

fn protocol_ref(protocol: SleepRepeatSaveProtocol) -> &'static str {
    match protocol {
        SleepRepeatSaveProtocol::Init => "init",
        SleepRepeatSaveProtocol::NeedsHoles => "needsHoles",
        SleepRepeatSaveProtocol::Resolved => "resolved",
    }
}

fn joined_holes(holes: &[SleepRepeatSaveHole]) -> String {
    if holes.is_empty() {
        return "none".to_string();
    }

    holes
        .iter()
        .map(|hole| match hole {
            SleepRepeatSaveHole::SavingThrowOutcome => "SavingThrowOutcome",
        })
        .collect::<Vec<_>>()
        .join(",")
}
