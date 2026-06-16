use crate::rules::scalar_buff::{
    fill_longstrider_target_choice, reject_stale_scalar_buff_after_resolved, ScalarBuffTargetHole,
    ScalarBuffTargetInvalidReason, ScalarBuffTargetProtocol, ScalarBuffTargetState,
};

pub const BRANCH_ACTIONS: [&str; 2] = ["doFillLongstriderTarget", "doRejectStaleAfterResolved"];

pub fn replay_observed_action(observed_action_taken: &str) -> ScalarBuffTargetState {
    match observed_action_taken {
        "doFillLongstriderTarget" => fill_longstrider_target_choice(),
        "doRejectStaleAfterResolved" => reject_stale_scalar_buff_after_resolved(),
        action => panic!("unsupported mbt::actionTaken {action}"),
    }
}

pub fn expected_witness(observed_action_taken: &str) -> ScalarBuffTargetState {
    replay_observed_action(observed_action_taken)
}

pub fn projection_payload(state: &ScalarBuffTargetState) -> String {
    [
        format!("fighterSpeed={}", state.fighter_speed_feet),
        format!("goblinSpeed={}", state.goblin_speed_feet),
        format!("actionAvailable={}", state.action_available),
        format!("protocolResult={}", protocol_ref(state.protocol)),
        format!(
            "protocolInvalidReason={}",
            invalid_reason_ref(state.protocol)
        ),
        format!("protocolHoles={}", joined_holes(&state.protocol_holes)),
    ]
    .join("\n")
}

fn protocol_ref(protocol: ScalarBuffTargetProtocol) -> &'static str {
    match protocol {
        ScalarBuffTargetProtocol::Init => "init",
        ScalarBuffTargetProtocol::Resolved => "resolved",
        ScalarBuffTargetProtocol::Invalid(_) => "invalid",
    }
}

fn invalid_reason_ref(protocol: ScalarBuffTargetProtocol) -> &'static str {
    match protocol {
        ScalarBuffTargetProtocol::Invalid(ScalarBuffTargetInvalidReason::StaleSubject) => {
            "WStaleSubject"
        }
        ScalarBuffTargetProtocol::Init | ScalarBuffTargetProtocol::Resolved => "none",
    }
}

fn joined_holes(holes: &[ScalarBuffTargetHole]) -> String {
    if holes.is_empty() {
        return "none".to_string();
    }

    holes
        .iter()
        .map(|hole| match hole {
            ScalarBuffTargetHole::TargetChoice => "TargetChoice",
        })
        .collect::<Vec<_>>()
        .join(",")
}
