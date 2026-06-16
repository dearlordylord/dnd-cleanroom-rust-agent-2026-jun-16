use crate::rules::rule_core_shove_outcome::{
    reject_shove_invalid_push_distance, resolve_shove_save_fails_prone,
    resolve_shove_save_fails_push, resolve_shove_save_fails_push_blocked,
    resolve_shove_save_fails_push_no_legal_destination, resolve_shove_save_succeeds,
    ShoveOutcomeState, ShovePushBlockedProjectionReason, ShovePushDispositionKind, ShoveScenario,
};

pub const BRANCH_ACTIONS: [&str; 6] = [
    "doInvalidPushDistance",
    "doSaveFailsProne",
    "doSaveFailsPush",
    "doSaveFailsPushBlocked",
    "doSaveFailsPushNoLegalDestination",
    "doSaveSucceeds",
];

pub fn replay_observed_action(observed_action_taken: &str) -> ShoveOutcomeState {
    match observed_action_taken {
        "doInvalidPushDistance" => reject_shove_invalid_push_distance(),
        "doSaveFailsProne" => resolve_shove_save_fails_prone(),
        "doSaveFailsPush" => resolve_shove_save_fails_push(),
        "doSaveFailsPushBlocked" => resolve_shove_save_fails_push_blocked(),
        "doSaveFailsPushNoLegalDestination" => resolve_shove_save_fails_push_no_legal_destination(),
        "doSaveSucceeds" => resolve_shove_save_succeeds(),
        action => panic!("unsupported mbt::actionTaken {action}"),
    }
}

pub fn expected_witness(observed_action_taken: &str) -> ShoveOutcomeState {
    replay_observed_action(observed_action_taken)
}

pub fn projection_payload(state: &ShoveOutcomeState) -> String {
    [
        format!("qLastScenario={}", scenario_ref(state.last_scenario)),
        format!("qAccepted={}", state.accepted),
        format!("qTargetProne={}", state.target_prone),
        format!("qPushEmitted={}", state.push_emitted),
        format!(
            "qPushDispositionKind={}",
            push_disposition_kind_ref(state.push_disposition_kind)
        ),
        format!(
            "qPushBlockedReason={}",
            push_blocked_reason_ref(state.push_blocked_reason)
        ),
        format!("qPushDistanceFeet={}", state.push_distance_feet),
        format!(
            "qPushProvokesOpportunityAttacks={}",
            state.push_provokes_opportunity_attacks
        ),
        format!("qActionAvailable={}", state.action_available),
        format!("qReplayIndex={}", state.replay_index),
    ]
    .join("\n")
}

fn scenario_ref(scenario: ShoveScenario) -> &'static str {
    match scenario {
        ShoveScenario::Init => "init",
        ShoveScenario::SaveSucceeds => "save-succeeds",
        ShoveScenario::SaveFailsProne => "save-fails-prone",
        ShoveScenario::SaveFailsPush => "save-fails-push",
        ShoveScenario::SaveFailsPushBlocked => "save-fails-push-blocked",
        ShoveScenario::SaveFailsPushNoLegalDestination => "save-fails-push-no-legal-destination",
        ShoveScenario::InvalidPushDistance => "invalid-push-distance",
    }
}

fn push_disposition_kind_ref(kind: ShovePushDispositionKind) -> &'static str {
    match kind {
        ShovePushDispositionKind::None => "none",
        ShovePushDispositionKind::Pushed => "pushed",
        ShovePushDispositionKind::Blocked => "blocked",
    }
}

fn push_blocked_reason_ref(reason: ShovePushBlockedProjectionReason) -> &'static str {
    match reason {
        ShovePushBlockedProjectionReason::None => "none",
        ShovePushBlockedProjectionReason::Blocked => "blocked",
        ShovePushBlockedProjectionReason::NoLegalDestination => "noLegalDestination",
    }
}
