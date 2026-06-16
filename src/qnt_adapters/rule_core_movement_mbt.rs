use crate::rules::rule_core_movement::{
    dash, decline_opportunity_attack, discover_escape_grapple, discover_grapple, discover_movement,
    disengage, move_provokes_opportunity_attack, move_threat_suppressed_by_disengage,
    reject_dash_after_action_spent, reject_movement_overspend, release_grapple,
    resolve_escape_failure, resolve_escape_success, resolve_grapple_failure,
    resolve_grapple_success, select_grapple_target, spend_full_movement, spend_movement,
    spend_short_movement, stand_from_prone, start_grappled_target_turn, MovementActor,
    MovementHole, MovementInvalidReason, MovementProtocol, MovementState,
};

pub const BRANCH_ACTIONS: [&str; 21] = [
    "doDash",
    "doDeclineOpportunityAttack",
    "doDiscoverEscapeGrapple",
    "doDiscoverGrapple",
    "doDiscoverMovement",
    "doDisengage",
    "doMoveProvokesOpportunityAttack",
    "doMoveThreatSuppressedByDisengage",
    "doRejectDashAfterActionSpent",
    "doRejectMovementOverspend",
    "doReleaseGrapple",
    "doResolveEscapeFailure",
    "doResolveEscapeSuccess",
    "doResolveGrappleFailure",
    "doResolveGrappleSuccess",
    "doSelectGrappleTarget",
    "doSpendFullMovement",
    "doSpendMovement",
    "doSpendShortMovement",
    "doStandFromProne",
    "doStartGrappledTargetTurn",
];

pub fn replay_observed_action(observed_action_taken: &str) -> MovementState {
    match observed_action_taken {
        "doDash" => dash(),
        "doDeclineOpportunityAttack" => decline_opportunity_attack(),
        "doDiscoverEscapeGrapple" => discover_escape_grapple(),
        "doDiscoverGrapple" => discover_grapple(),
        "doDiscoverMovement" => discover_movement(),
        "doDisengage" => disengage(),
        "doMoveProvokesOpportunityAttack" => move_provokes_opportunity_attack(),
        "doMoveThreatSuppressedByDisengage" => move_threat_suppressed_by_disengage(),
        "doRejectDashAfterActionSpent" => reject_dash_after_action_spent(),
        "doRejectMovementOverspend" => reject_movement_overspend(),
        "doReleaseGrapple" => release_grapple(),
        "doResolveEscapeFailure" => resolve_escape_failure(),
        "doResolveEscapeSuccess" => resolve_escape_success(),
        "doResolveGrappleFailure" => resolve_grapple_failure(),
        "doResolveGrappleSuccess" => resolve_grapple_success(),
        "doSelectGrappleTarget" => select_grapple_target(),
        "doSpendFullMovement" => spend_full_movement(),
        "doSpendMovement" => spend_movement(),
        "doSpendShortMovement" => spend_short_movement(),
        "doStandFromProne" => stand_from_prone(),
        "doStartGrappledTargetTurn" => start_grappled_target_turn(),
        action => panic!("unsupported mbt::actionTaken {action}"),
    }
}

pub fn expected_witness(observed_action_taken: &str) -> MovementState {
    replay_observed_action(observed_action_taken)
}

pub fn projection_payload(state: &MovementState) -> String {
    let protocol_holes = protocol_holes(&state.protocol);
    [
        format!("qCurrentActor={}", actor_ref(state.current_actor)),
        format!("qMovementSpentFeet={}", state.movement_spent_feet),
        format!("qDashBonusFeet={}", state.dash_bonus_feet),
        format!("qProne={}", state.prone),
        format!("qDisengaged={}", state.disengaged),
        format!("qActionAvailable={}", state.action_available),
        format!("qGrappleActive={}", state.grapple_active),
        format!("qGrappleEscapeDc={}", state.grapple_escape_dc),
        format!("qHoles={}", joined_or_none(&protocol_holes)),
        format!(
            "qPendingOpportunityAttack={}",
            state.pending_opportunity_attack
        ),
        format!("protocolResult={}", protocol_result_ref(&state.protocol)),
        format!(
            "protocolInvalidReason={}",
            protocol_invalid_reason_ref(&state.protocol)
        ),
        format!("protocolHoles={}", joined_or_none(&protocol_holes)),
    ]
    .join("\n")
}

fn actor_ref(actor: MovementActor) -> &'static str {
    match actor {
        MovementActor::Fighter => "Fighter",
        MovementActor::GrappledTarget => "GrappledTarget",
    }
}

fn protocol_result_ref(protocol: &MovementProtocol) -> &'static str {
    match protocol {
        MovementProtocol::Init => "init",
        MovementProtocol::NeedsHoles(_) => "needsHoles",
        MovementProtocol::Resolved => "resolved",
        MovementProtocol::Invalid { .. } => "invalid",
    }
}

fn protocol_invalid_reason_ref(protocol: &MovementProtocol) -> &'static str {
    match protocol {
        MovementProtocol::Invalid {
            reason: MovementInvalidReason::InvalidFill,
            ..
        } => "WInvalidFill",
        MovementProtocol::Invalid {
            reason: MovementInvalidReason::StaleSubject,
            ..
        } => "WStaleSubject",
        MovementProtocol::Init | MovementProtocol::NeedsHoles(_) | MovementProtocol::Resolved => "",
    }
}

fn protocol_holes(protocol: &MovementProtocol) -> Vec<&'static str> {
    match protocol {
        MovementProtocol::NeedsHoles(holes) => holes.iter().map(hole_ref).collect(),
        MovementProtocol::Invalid { holes, .. } => holes.iter().map(hole_ref).collect(),
        MovementProtocol::Init | MovementProtocol::Resolved => Vec::new(),
    }
}

fn hole_ref(hole: &MovementHole) -> &'static str {
    match hole {
        MovementHole::Movement => "Movement",
        MovementHole::ReactionDecision => "ReactionDecision",
        MovementHole::TargetChoice => "TargetChoice",
        MovementHole::GrappleOutcome => "GrappleOutcome",
    }
}

fn joined_or_none(values: &[&str]) -> String {
    if values.is_empty() {
        "none".to_string()
    } else {
        values.join(",")
    }
}
