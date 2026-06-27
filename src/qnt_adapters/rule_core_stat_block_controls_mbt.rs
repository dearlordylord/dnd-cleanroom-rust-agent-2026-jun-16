use crate::rules::rule_core_stat_block_controls::{
    end_turn_closes_stat_block_dispatches, move_during_stat_block_dispatch,
    reject_bonus_action_during_dispatch, reject_ordinary_action_during_dispatch,
    resolve_primary_stat_block_dispatch, resolve_secondary_stat_block_dispatch,
    resolve_stat_block_control_subject, start_stat_block_multiattack,
    start_stat_block_multiattack_from, stat_block_control_initial_state, StatBlockAttackSlot,
    StatBlockControlHole, StatBlockControlInvalidReason, StatBlockControlProtocol,
    StatBlockControlState, StatBlockDispatchSubject, StatBlockMultiattackProfile,
};

pub const BRANCH_ACTIONS: [&str; 7] = [
    "doEndTurnClosesDispatches",
    "doMoveDuringDispatch",
    "doRejectBonusActionDuringDispatch",
    "doRejectOrdinaryActionDuringDispatch",
    "doResolvePrimaryDispatch",
    "doResolveSecondaryDispatch",
    "doStartMultiattack",
];

pub fn replay_observed_action(observed_action_taken: &str) -> StatBlockControlState {
    replay_observed_action_through_control_transition(observed_action_taken)
}

pub fn expected_witness(observed_action_taken: &str) -> StatBlockControlState {
    match observed_action_taken {
        "doEndTurnClosesDispatches" => end_turn_closes_stat_block_dispatches(),
        "doMoveDuringDispatch" => move_during_stat_block_dispatch(),
        "doRejectBonusActionDuringDispatch" => reject_bonus_action_during_dispatch(),
        "doRejectOrdinaryActionDuringDispatch" => reject_ordinary_action_during_dispatch(),
        "doResolvePrimaryDispatch" => resolve_primary_stat_block_dispatch(),
        "doResolveSecondaryDispatch" => resolve_secondary_stat_block_dispatch(),
        "doStartMultiattack" => start_stat_block_multiattack(),
        action => panic!("unsupported mbt::actionTaken {action}"),
    }
}

fn replay_observed_action_through_control_transition(
    observed_action_taken: &str,
) -> StatBlockControlState {
    match observed_action_taken {
        "doEndTurnClosesDispatches" => resolve_stat_block_control_subject(
            started_multiattack_control_state(),
            StatBlockDispatchSubject::EndTurn,
        ),
        "doMoveDuringDispatch" => resolve_stat_block_control_subject(
            started_multiattack_control_state(),
            StatBlockDispatchSubject::Movement,
        ),
        "doRejectBonusActionDuringDispatch" => resolve_stat_block_control_subject(
            started_multiattack_control_state(),
            StatBlockDispatchSubject::BonusAction,
        ),
        "doRejectOrdinaryActionDuringDispatch" => resolve_stat_block_control_subject(
            started_multiattack_control_state(),
            StatBlockDispatchSubject::OrdinaryAction,
        ),
        "doResolvePrimaryDispatch" => resolve_stat_block_control_subject(
            started_multiattack_control_state(),
            StatBlockDispatchSubject::PrimaryAttackSlot,
        ),
        "doResolveSecondaryDispatch" => resolve_stat_block_control_subject(
            started_multiattack_control_state(),
            StatBlockDispatchSubject::SecondaryAttackSlot,
        ),
        "doStartMultiattack" => started_multiattack_control_state(),
        action => panic!("unsupported mbt::actionTaken {action}"),
    }
}

fn started_multiattack_control_state() -> StatBlockControlState {
    start_stat_block_multiattack_from(stat_block_control_initial_state(), fixture_profile())
}

fn fixture_profile() -> StatBlockMultiattackProfile {
    StatBlockMultiattackProfile {
        first_attack_slot: StatBlockAttackSlot::Primary,
        primary_attack_count: 2,
        secondary_attack_count: 1,
    }
}

pub fn projection_payload(state: &StatBlockControlState) -> String {
    let protocol_holes = protocol_holes(&state.protocol);
    [
        format!("qAttackActionAvailable={}", state.attack_action_available),
        format!("qBonusActionAvailable={}", state.bonus_action_available),
        format!(
            "qPendingPrimaryDispatches={}",
            state.pending_primary_dispatches
        ),
        format!(
            "qPendingSecondaryDispatches={}",
            state.pending_secondary_dispatches
        ),
        format!("qMovementSpentFeet={}", state.movement_spent_feet),
        format!("qMovementRemainingFeet={}", state.movement_remaining_feet),
        format!(
            "qMultiattackContinuationOpen={}",
            state.multiattack_continuation_open
        ),
        format!("qHoles={}", joined_or_none(&protocol_holes)),
        format!("protocolResult={}", protocol_result_ref(&state.protocol)),
        format!(
            "protocolInvalidReason={}",
            protocol_invalid_reason_ref(&state.protocol)
        ),
        format!("protocolHoles={}", joined_or_none(&protocol_holes)),
        format!("qComponentRoute={}", component_route_payload()),
    ]
    .join("\n")
}

pub fn component_route_payload() -> String {
    [
        "RuleCoreComponentParseInput(RuleCoreStatBlockControlOwner)",
        "RuleCoreComponentAdmitInput(RuleCoreStatBlockControlOwner)",
        "RuleCoreComponentCall(RuleCoreStatBlockControlOwner)",
        "RuleCoreComponentProjectResult(RuleCoreStatBlockControlOwner)",
    ]
    .join(",")
}

fn protocol_result_ref(protocol: &StatBlockControlProtocol) -> &'static str {
    match protocol {
        StatBlockControlProtocol::Init => "init",
        StatBlockControlProtocol::NeedsHoles(_) => "needsHoles",
        StatBlockControlProtocol::Resolved => "resolved",
        StatBlockControlProtocol::Invalid { .. } => "invalid",
    }
}

fn protocol_invalid_reason_ref(protocol: &StatBlockControlProtocol) -> &'static str {
    match protocol {
        StatBlockControlProtocol::Invalid {
            reason: StatBlockControlInvalidReason::InvalidFill,
            ..
        } => "WInvalidFill",
        StatBlockControlProtocol::Invalid {
            reason: StatBlockControlInvalidReason::StaleSubject,
            ..
        } => "WStaleSubject",
        StatBlockControlProtocol::Init
        | StatBlockControlProtocol::NeedsHoles(_)
        | StatBlockControlProtocol::Resolved => "",
    }
}

fn protocol_holes(protocol: &StatBlockControlProtocol) -> Vec<&'static str> {
    match protocol {
        StatBlockControlProtocol::NeedsHoles(holes) => holes.iter().map(hole_ref).collect(),
        StatBlockControlProtocol::Invalid { holes, .. } => holes.iter().map(hole_ref).collect(),
        StatBlockControlProtocol::Init | StatBlockControlProtocol::Resolved => Vec::new(),
    }
}

fn hole_ref(hole: &StatBlockControlHole) -> &'static str {
    match hole {
        StatBlockControlHole::Movement => "Movement",
    }
}

fn joined_or_none(values: &[&str]) -> String {
    if values.is_empty() {
        "none".to_string()
    } else {
        values.join(",")
    }
}
