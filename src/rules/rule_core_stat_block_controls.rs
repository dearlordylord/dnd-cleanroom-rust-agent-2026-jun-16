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
pub enum StatBlockAttackSlot {
    Primary,
    Secondary,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct StatBlockMultiattackProfile {
    pub first_attack_slot: StatBlockAttackSlot,
    pub primary_attack_count: i16,
    pub secondary_attack_count: i16,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StatBlockDispatchSubject {
    PrimaryAttackSlot,
    SecondaryAttackSlot,
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
    start_stat_block_multiattack_from(
        stat_block_control_initial_state(),
        StatBlockMultiattackProfile {
            first_attack_slot: StatBlockAttackSlot::Primary,
            primary_attack_count: 2,
            secondary_attack_count: 1,
        },
    )
}

#[must_use]
pub fn start_stat_block_multiattack_from(
    state: StatBlockControlState,
    profile: StatBlockMultiattackProfile,
) -> StatBlockControlState {
    // RAW: cleanroom-input/raw/srd-5.2.1/Monsters/Overview.md
    // "Actions" and "Multiattack"; Playing-the-Game.md "Actions".
    // QNT: rule-core/stat-block-controls.qnt `startStatBlockMultiattack`.
    if !legal_stat_block_multiattack_profile(profile)
        || stat_block_multiattack_continuation_open(&state)
        || !state.attack_action_available
    {
        return state;
    }

    let dispatches = multiattack_dispatches_after_first_attack(profile);
    StatBlockControlState {
        attack_action_available: false,
        pending_primary_dispatches: dispatches.0,
        pending_secondary_dispatches: dispatches.1,
        multiattack_continuation_open: dispatches_open(dispatches.0, dispatches.1),
        protocol: StatBlockControlProtocol::Resolved,
        ..state
    }
}

#[must_use]
pub fn move_during_stat_block_dispatch() -> StatBlockControlState {
    resolve_stat_block_control_subject(
        start_stat_block_multiattack(),
        StatBlockDispatchSubject::Movement,
    )
}

#[must_use]
pub fn resolve_stat_block_control_subject(
    state: StatBlockControlState,
    subject: StatBlockDispatchSubject,
) -> StatBlockControlState {
    // QNT: stat-block-controls.qnt `multiattackDispatchSubjectPermitted`,
    // `resolveStatBlockMultiattackDispatch`, and `endStatBlockTurn`.
    if !multiattack_dispatch_subject_permitted(&state, subject) {
        return invalid_stale_subject(state);
    }

    match subject {
        StatBlockDispatchSubject::Movement => move_during_stat_block_dispatch_from(state),
        StatBlockDispatchSubject::EndTurn => end_stat_block_turn_from(state),
        StatBlockDispatchSubject::PrimaryAttackSlot => {
            resolve_stat_block_multiattack_dispatch_from(state, StatBlockAttackSlot::Primary)
        }
        StatBlockDispatchSubject::SecondaryAttackSlot => {
            resolve_stat_block_multiattack_dispatch_from(state, StatBlockAttackSlot::Secondary)
        }
        StatBlockDispatchSubject::BonusAction | StatBlockDispatchSubject::OrdinaryAction => state,
    }
}

#[must_use]
pub fn move_during_stat_block_dispatch_from(state: StatBlockControlState) -> StatBlockControlState {
    if state.movement_remaining_feet < STAT_BLOCK_INTERLEAVED_MOVEMENT_FEET {
        return state;
    }

    StatBlockControlState {
        movement_spent_feet: state.movement_spent_feet + STAT_BLOCK_INTERLEAVED_MOVEMENT_FEET,
        movement_remaining_feet: state.movement_remaining_feet
            - STAT_BLOCK_INTERLEAVED_MOVEMENT_FEET,
        protocol: StatBlockControlProtocol::Resolved,
        ..state
    }
}

#[must_use]
pub fn reject_bonus_action_during_dispatch() -> StatBlockControlState {
    resolve_stat_block_control_subject(
        start_stat_block_multiattack(),
        StatBlockDispatchSubject::BonusAction,
    )
}

#[must_use]
pub fn reject_ordinary_action_during_dispatch() -> StatBlockControlState {
    resolve_stat_block_control_subject(
        start_stat_block_multiattack(),
        StatBlockDispatchSubject::OrdinaryAction,
    )
}

#[must_use]
pub fn resolve_primary_stat_block_dispatch() -> StatBlockControlState {
    resolve_stat_block_control_subject(
        start_stat_block_multiattack(),
        StatBlockDispatchSubject::PrimaryAttackSlot,
    )
}

#[must_use]
pub fn resolve_secondary_stat_block_dispatch() -> StatBlockControlState {
    resolve_stat_block_control_subject(
        start_stat_block_multiattack(),
        StatBlockDispatchSubject::SecondaryAttackSlot,
    )
}

#[must_use]
pub fn resolve_stat_block_multiattack_dispatch_from(
    state: StatBlockControlState,
    attack_slot: StatBlockAttackSlot,
) -> StatBlockControlState {
    let (primary, secondary) = decrement_dispatch(
        state.pending_primary_dispatches,
        state.pending_secondary_dispatches,
        attack_slot,
    );
    StatBlockControlState {
        pending_primary_dispatches: primary,
        pending_secondary_dispatches: secondary,
        multiattack_continuation_open: dispatches_open(primary, secondary),
        protocol: StatBlockControlProtocol::Resolved,
        ..state
    }
}

#[must_use]
pub fn end_turn_closes_stat_block_dispatches() -> StatBlockControlState {
    end_stat_block_turn_from(start_stat_block_multiattack())
}

#[must_use]
pub fn end_stat_block_turn_from(_state: StatBlockControlState) -> StatBlockControlState {
    StatBlockControlState {
        attack_action_available: true,
        bonus_action_available: true,
        pending_primary_dispatches: 0,
        pending_secondary_dispatches: 0,
        movement_spent_feet: 0,
        movement_remaining_feet: STAT_BLOCK_INITIAL_MOVEMENT_REMAINING_FEET,
        multiattack_continuation_open: false,
        protocol: StatBlockControlProtocol::Resolved,
    }
}

#[must_use]
pub fn multiattack_dispatch_subject_permitted(
    state: &StatBlockControlState,
    subject: StatBlockDispatchSubject,
) -> bool {
    if !stat_block_multiattack_continuation_open(state) {
        return true;
    }

    match subject {
        StatBlockDispatchSubject::Movement | StatBlockDispatchSubject::EndTurn => true,
        StatBlockDispatchSubject::PrimaryAttackSlot => state.pending_primary_dispatches > 0,
        StatBlockDispatchSubject::SecondaryAttackSlot => state.pending_secondary_dispatches > 0,
        StatBlockDispatchSubject::BonusAction | StatBlockDispatchSubject::OrdinaryAction => false,
    }
}

#[must_use]
pub fn stat_block_multiattack_continuation_open(state: &StatBlockControlState) -> bool {
    dispatches_open(
        state.pending_primary_dispatches,
        state.pending_secondary_dispatches,
    )
}

fn legal_stat_block_multiattack_profile(profile: StatBlockMultiattackProfile) -> bool {
    let total_attacks = profile.primary_attack_count + profile.secondary_attack_count;
    let first_attack_count = match profile.first_attack_slot {
        StatBlockAttackSlot::Primary => profile.primary_attack_count,
        StatBlockAttackSlot::Secondary => profile.secondary_attack_count,
    };

    (0..=3).contains(&profile.primary_attack_count)
        && (0..=3).contains(&profile.secondary_attack_count)
        && (1..=4).contains(&total_attacks)
        && first_attack_count >= 1
}

fn multiattack_dispatches_after_first_attack(profile: StatBlockMultiattackProfile) -> (i16, i16) {
    decrement_dispatch(
        profile.primary_attack_count,
        profile.secondary_attack_count,
        profile.first_attack_slot,
    )
}

fn decrement_dispatch(
    primary: i16,
    secondary: i16,
    attack_slot: StatBlockAttackSlot,
) -> (i16, i16) {
    match attack_slot {
        StatBlockAttackSlot::Primary if primary > 0 => (primary - 1, secondary),
        StatBlockAttackSlot::Secondary if secondary > 0 => (primary, secondary - 1),
        StatBlockAttackSlot::Primary | StatBlockAttackSlot::Secondary => (primary, secondary),
    }
}

fn dispatches_open(primary: i16, secondary: i16) -> bool {
    primary + secondary > 0
}

fn invalid_stale_subject(state: StatBlockControlState) -> StatBlockControlState {
    StatBlockControlState {
        protocol: StatBlockControlProtocol::Invalid {
            holes: Vec::new(),
            reason: StatBlockControlInvalidReason::StaleSubject,
        },
        ..state
    }
}
