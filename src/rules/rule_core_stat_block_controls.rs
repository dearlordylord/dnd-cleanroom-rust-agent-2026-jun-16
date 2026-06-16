#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StatBlockControlHole {
    Movement,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StatBlockControlInvalidReason {
    InvalidFill,
    StaleSubject,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum StatBlockControlProtocol {
    Init,
    NeedsHoles(Vec<StatBlockControlHole>),
    Resolved,
    Invalid {
        holes: Vec<StatBlockControlHole>,
        reason: StatBlockControlInvalidReason,
    },
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StatBlockControlState {
    pub attack_action_available: bool,
    pub bonus_action_available: bool,
    pub pending_primary_dispatches: i16,
    pub pending_secondary_dispatches: i16,
    pub movement_spent_feet: i16,
    pub movement_remaining_feet: i16,
    pub multiattack_continuation_open: bool,
    pub protocol: StatBlockControlProtocol,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StatBlockDispatchSubject {
    PrimaryAttack,
    SecondaryAttack,
    Movement,
    EndTurn,
    BonusAction,
    OrdinaryAction,
}

pub const STAT_BLOCK_INITIAL_MOVEMENT_REMAINING_FEET: i16 = 30;
pub const STAT_BLOCK_INTERLEAVED_MOVEMENT_FEET: i16 = 5;

#[must_use]
pub fn stat_block_control_initial_state() -> StatBlockControlState {
    StatBlockControlState {
        attack_action_available: true,
        bonus_action_available: true,
        pending_primary_dispatches: 0,
        pending_secondary_dispatches: 0,
        movement_spent_feet: 0,
        movement_remaining_feet: STAT_BLOCK_INITIAL_MOVEMENT_REMAINING_FEET,
        multiattack_continuation_open: false,
        protocol: StatBlockControlProtocol::Init,
    }
}

#[must_use]
pub fn start_stat_block_multiattack() -> StatBlockControlState {
    // RAW: cleanroom-input/raw/srd-5.2.1/Monsters/Overview.md
    // "Actions" and "Multiattack"; Playing-the-Game.md "Actions".
    // QNT: rule-core/stat-block-controls.qnt.
    StatBlockControlState {
        attack_action_available: false,
        pending_primary_dispatches: 1,
        pending_secondary_dispatches: 1,
        multiattack_continuation_open: true,
        protocol: StatBlockControlProtocol::Resolved,
        ..stat_block_control_initial_state()
    }
}

#[must_use]
pub fn move_during_stat_block_dispatch() -> StatBlockControlState {
    StatBlockControlState {
        movement_spent_feet: STAT_BLOCK_INTERLEAVED_MOVEMENT_FEET,
        movement_remaining_feet: STAT_BLOCK_INITIAL_MOVEMENT_REMAINING_FEET
            - STAT_BLOCK_INTERLEAVED_MOVEMENT_FEET,
        ..start_stat_block_multiattack()
    }
}

#[must_use]
pub fn reject_bonus_action_during_dispatch() -> StatBlockControlState {
    invalid_stale_subject_from_started_multiattack()
}

#[must_use]
pub fn reject_ordinary_action_during_dispatch() -> StatBlockControlState {
    invalid_stale_subject_from_started_multiattack()
}

#[must_use]
pub fn resolve_primary_stat_block_dispatch() -> StatBlockControlState {
    StatBlockControlState {
        pending_primary_dispatches: 0,
        ..start_stat_block_multiattack()
    }
}

#[must_use]
pub fn resolve_secondary_stat_block_dispatch() -> StatBlockControlState {
    StatBlockControlState {
        pending_secondary_dispatches: 0,
        ..start_stat_block_multiattack()
    }
}

#[must_use]
pub fn end_turn_closes_stat_block_dispatches() -> StatBlockControlState {
    StatBlockControlState {
        protocol: StatBlockControlProtocol::Resolved,
        ..stat_block_control_initial_state()
    }
}

#[must_use]
pub fn multiattack_dispatch_subject_permitted(
    state: &StatBlockControlState,
    subject: StatBlockDispatchSubject,
) -> bool {
    if !state.multiattack_continuation_open {
        return true;
    }

    match subject {
        StatBlockDispatchSubject::Movement | StatBlockDispatchSubject::EndTurn => true,
        StatBlockDispatchSubject::PrimaryAttack => state.pending_primary_dispatches > 0,
        StatBlockDispatchSubject::SecondaryAttack => state.pending_secondary_dispatches > 0,
        StatBlockDispatchSubject::BonusAction | StatBlockDispatchSubject::OrdinaryAction => false,
    }
}

fn invalid_stale_subject_from_started_multiattack() -> StatBlockControlState {
    StatBlockControlState {
        protocol: StatBlockControlProtocol::Invalid {
            holes: Vec::new(),
            reason: StatBlockControlInvalidReason::StaleSubject,
        },
        ..start_stat_block_multiattack()
    }
}
