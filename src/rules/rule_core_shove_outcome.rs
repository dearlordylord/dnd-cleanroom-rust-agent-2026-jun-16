#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ShoveScenario {
    Init,
    SaveSucceeds,
    SaveFailsProne,
    SaveFailsPush,
    SaveFailsPushBlocked,
    SaveFailsPushNoLegalDestination,
    InvalidPushDistance,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ShovePushBlockedReason {
    Blocked,
    NoLegalDestination,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ShovePushDisposition {
    Pushed {
        distance_feet: i16,
    },
    Blocked {
        distance_feet: i16,
        reason: ShovePushBlockedReason,
    },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ShoveFailedEffect {
    Prone,
    PushAway(ShovePushDisposition),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ShoveOutcome {
    SaveSucceeded,
    SaveFailed { failed_effect: ShoveFailedEffect },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ShoveOutcomeResult {
    Rejected {
        target_prone: bool,
    },
    AcceptedNoPush {
        target_prone: bool,
    },
    AcceptedPush {
        target_prone: bool,
        disposition: ShovePushDisposition,
    },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ShovePushDispositionKind {
    None,
    Pushed,
    Blocked,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ShovePushBlockedProjectionReason {
    None,
    Blocked,
    NoLegalDestination,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ShoveOutcomeProjection {
    pub accepted: bool,
    pub target_prone: bool,
    pub push_emitted: bool,
    pub push_disposition_kind: ShovePushDispositionKind,
    pub push_blocked_reason: ShovePushBlockedProjectionReason,
    pub push_distance_feet: i16,
    pub push_provokes_opportunity_attacks: bool,
    pub action_available: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ShoveOutcomeState {
    pub last_scenario: ShoveScenario,
    pub accepted: bool,
    pub target_prone: bool,
    pub push_emitted: bool,
    pub push_disposition_kind: ShovePushDispositionKind,
    pub push_blocked_reason: ShovePushBlockedProjectionReason,
    pub push_distance_feet: i16,
    pub push_provokes_opportunity_attacks: bool,
    pub action_available: bool,
    pub replay_index: i16,
}

pub const SHOVE_LEGAL_PUSH_DISTANCE_FEET: i16 = 5;
pub const SHOVE_INVALID_PUSH_DISTANCE_FEET: i16 = 10;

#[must_use]
pub fn shove_outcome_initial_state() -> ShoveOutcomeState {
    ShoveOutcomeState {
        last_scenario: ShoveScenario::Init,
        accepted: true,
        target_prone: false,
        push_emitted: false,
        push_disposition_kind: ShovePushDispositionKind::None,
        push_blocked_reason: ShovePushBlockedProjectionReason::None,
        push_distance_feet: 0,
        push_provokes_opportunity_attacks: false,
        action_available: true,
        replay_index: 0,
    }
}

#[must_use]
pub fn apply_shove_outcome(target_prone: bool, outcome: ShoveOutcome) -> ShoveOutcomeResult {
    // RAW: cleanroom-input/raw/srd-5.2.1/Rules-Glossary.md
    // "Unarmed Strike" / "Shove"; Playing-the-Game.md "Actions" and
    // "Opportunity Attacks". QNT: rule-core/shove-outcome.qnt.
    match outcome {
        ShoveOutcome::SaveSucceeded => ShoveOutcomeResult::AcceptedNoPush { target_prone },
        ShoveOutcome::SaveFailed { failed_effect } => match failed_effect {
            ShoveFailedEffect::Prone => ShoveOutcomeResult::AcceptedNoPush { target_prone: true },
            ShoveFailedEffect::PushAway(disposition) => {
                if legal_shove_push_disposition(disposition) {
                    ShoveOutcomeResult::AcceptedPush {
                        target_prone,
                        disposition,
                    }
                } else {
                    ShoveOutcomeResult::Rejected { target_prone }
                }
            }
        },
    }
}

#[must_use]
pub fn project_shove_outcome_result(result: ShoveOutcomeResult) -> ShoveOutcomeProjection {
    match result {
        ShoveOutcomeResult::Rejected { target_prone } => ShoveOutcomeProjection {
            accepted: false,
            target_prone,
            push_emitted: false,
            push_disposition_kind: ShovePushDispositionKind::None,
            push_blocked_reason: ShovePushBlockedProjectionReason::None,
            push_distance_feet: 0,
            push_provokes_opportunity_attacks: false,
            action_available: true,
        },
        ShoveOutcomeResult::AcceptedNoPush { target_prone } => ShoveOutcomeProjection {
            accepted: true,
            target_prone,
            push_emitted: false,
            push_disposition_kind: ShovePushDispositionKind::None,
            push_blocked_reason: ShovePushBlockedProjectionReason::None,
            push_distance_feet: 0,
            push_provokes_opportunity_attacks: false,
            action_available: false,
        },
        ShoveOutcomeResult::AcceptedPush {
            target_prone,
            disposition,
        } => ShoveOutcomeProjection {
            accepted: true,
            target_prone,
            push_emitted: true,
            push_disposition_kind: shove_push_disposition_kind(disposition),
            push_blocked_reason: shove_push_blocked_reason(disposition),
            push_distance_feet: shove_push_distance_feet(disposition),
            push_provokes_opportunity_attacks: false,
            action_available: false,
        },
    }
}

#[must_use]
pub fn resolve_shove_save_succeeds() -> ShoveOutcomeState {
    shove_state_from_projection(
        ShoveScenario::SaveSucceeds,
        project_shove_outcome_result(apply_shove_outcome(false, ShoveOutcome::SaveSucceeded)),
        1,
    )
}

#[must_use]
pub fn resolve_shove_save_fails_prone() -> ShoveOutcomeState {
    shove_state_from_projection(
        ShoveScenario::SaveFailsProne,
        project_shove_outcome_result(apply_shove_outcome(
            false,
            ShoveOutcome::SaveFailed {
                failed_effect: ShoveFailedEffect::Prone,
            },
        )),
        2,
    )
}

#[must_use]
pub fn resolve_shove_save_fails_push() -> ShoveOutcomeState {
    shove_state_from_projection(
        ShoveScenario::SaveFailsPush,
        project_shove_outcome_result(apply_shove_outcome(
            false,
            ShoveOutcome::SaveFailed {
                failed_effect: ShoveFailedEffect::PushAway(ShovePushDisposition::Pushed {
                    distance_feet: SHOVE_LEGAL_PUSH_DISTANCE_FEET,
                }),
            },
        )),
        3,
    )
}

#[must_use]
pub fn resolve_shove_save_fails_push_blocked() -> ShoveOutcomeState {
    shove_state_from_projection(
        ShoveScenario::SaveFailsPushBlocked,
        project_shove_outcome_result(apply_shove_outcome(
            false,
            ShoveOutcome::SaveFailed {
                failed_effect: ShoveFailedEffect::PushAway(ShovePushDisposition::Blocked {
                    distance_feet: SHOVE_LEGAL_PUSH_DISTANCE_FEET,
                    reason: ShovePushBlockedReason::Blocked,
                }),
            },
        )),
        4,
    )
}

#[must_use]
pub fn resolve_shove_save_fails_push_no_legal_destination() -> ShoveOutcomeState {
    shove_state_from_projection(
        ShoveScenario::SaveFailsPushNoLegalDestination,
        project_shove_outcome_result(apply_shove_outcome(
            false,
            ShoveOutcome::SaveFailed {
                failed_effect: ShoveFailedEffect::PushAway(ShovePushDisposition::Blocked {
                    distance_feet: SHOVE_LEGAL_PUSH_DISTANCE_FEET,
                    reason: ShovePushBlockedReason::NoLegalDestination,
                }),
            },
        )),
        5,
    )
}

#[must_use]
pub fn reject_shove_invalid_push_distance() -> ShoveOutcomeState {
    shove_state_from_projection(
        ShoveScenario::InvalidPushDistance,
        project_shove_outcome_result(apply_shove_outcome(
            false,
            ShoveOutcome::SaveFailed {
                failed_effect: ShoveFailedEffect::PushAway(ShovePushDisposition::Pushed {
                    distance_feet: SHOVE_INVALID_PUSH_DISTANCE_FEET,
                }),
            },
        )),
        6,
    )
}

#[must_use]
pub fn legal_shove_push_disposition(disposition: ShovePushDisposition) -> bool {
    shove_push_distance_feet(disposition) == SHOVE_LEGAL_PUSH_DISTANCE_FEET
}

#[must_use]
pub fn shove_push_distance_feet(disposition: ShovePushDisposition) -> i16 {
    match disposition {
        ShovePushDisposition::Pushed { distance_feet } => distance_feet,
        ShovePushDisposition::Blocked { distance_feet, .. } => distance_feet,
    }
}

#[must_use]
pub fn shove_push_disposition_kind(disposition: ShovePushDisposition) -> ShovePushDispositionKind {
    match disposition {
        ShovePushDisposition::Pushed { .. } => ShovePushDispositionKind::Pushed,
        ShovePushDisposition::Blocked { .. } => ShovePushDispositionKind::Blocked,
    }
}

#[must_use]
pub fn shove_push_blocked_reason(
    disposition: ShovePushDisposition,
) -> ShovePushBlockedProjectionReason {
    match disposition {
        ShovePushDisposition::Pushed { .. } => ShovePushBlockedProjectionReason::None,
        ShovePushDisposition::Blocked {
            reason: ShovePushBlockedReason::Blocked,
            ..
        } => ShovePushBlockedProjectionReason::Blocked,
        ShovePushDisposition::Blocked {
            reason: ShovePushBlockedReason::NoLegalDestination,
            ..
        } => ShovePushBlockedProjectionReason::NoLegalDestination,
    }
}

fn shove_state_from_projection(
    last_scenario: ShoveScenario,
    projection: ShoveOutcomeProjection,
    replay_index: i16,
) -> ShoveOutcomeState {
    ShoveOutcomeState {
        last_scenario,
        accepted: projection.accepted,
        target_prone: projection.target_prone,
        push_emitted: projection.push_emitted,
        push_disposition_kind: projection.push_disposition_kind,
        push_blocked_reason: projection.push_blocked_reason,
        push_distance_feet: projection.push_distance_feet,
        push_provokes_opportunity_attacks: projection.push_provokes_opportunity_attacks,
        action_available: projection.action_available,
        replay_index,
    }
}
