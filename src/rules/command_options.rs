#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CommandNextTurnOption {
    Approach,
    Drop,
    Flee,
    Grovel,
    Halt,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CommandTurnActor {
    Caster,
    Target,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CommandNextTurnInvalidReason {
    InvalidFill,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CommandNextTurnProtocol {
    Init,
    Resolved,
    NeedsInterruptDecision,
    Invalid(CommandNextTurnInvalidReason),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CommandNextTurnScenario {
    Init,
    FailedSaveRecordsPending,
    FollowGrovel,
    FollowDrop,
    HaltSuppresses,
    HaltEndTurnCleanup,
    ApproachContinues,
    ApproachWithinFiveEndsTurn,
    ApproachMovementRejected,
    ApproachNoMovementCleanup,
    FleeFullMovementEndsTurn,
    FleePartialMovementRejected,
    FleeNoMovementCleanup,
    FleeOpportunityAttackWindow,
    FleeOpportunityAttackDeclinedContinuation,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CommandNextTurnState {
    pub scenario: CommandNextTurnScenario,
    pub protocol: CommandNextTurnProtocol,
    pub target_prone: bool,
    pub target_effect_count: i16,
    pub action_available: bool,
    pub bonus_action_available: bool,
    pub movement_spent_feet: i16,
    pub current_actor: CommandTurnActor,
    pub pending_option: Option<CommandNextTurnOption>,
    pub dropped_object_count: i16,
    pub reaction_window_open: bool,
    pub halt_suppressed: bool,
}

#[must_use]
pub fn command_next_turn_initial_state() -> CommandNextTurnState {
    CommandNextTurnState {
        scenario: CommandNextTurnScenario::Init,
        protocol: CommandNextTurnProtocol::Init,
        target_prone: false,
        target_effect_count: 0,
        action_available: true,
        bonus_action_available: true,
        movement_spent_feet: 0,
        current_actor: CommandTurnActor::Caster,
        pending_option: None,
        dropped_object_count: 0,
        reaction_window_open: false,
        halt_suppressed: false,
    }
}

#[must_use]
pub fn record_command_failed_save_pending(
    _state: CommandNextTurnState,
    option: CommandNextTurnOption,
) -> CommandNextTurnState {
    // RAW: cleanroom-input/raw/srd-5.2.1/Spells/Descriptions-A-D.md
    // "Command"; QNT: battle-runtime-command-option-next-turn.mbt.qnt.
    CommandNextTurnState {
        scenario: CommandNextTurnScenario::FailedSaveRecordsPending,
        protocol: CommandNextTurnProtocol::Resolved,
        target_prone: false,
        target_effect_count: 1,
        action_available: false,
        bonus_action_available: true,
        movement_spent_feet: 0,
        current_actor: CommandTurnActor::Caster,
        pending_option: Some(option),
        dropped_object_count: 0,
        reaction_window_open: false,
        halt_suppressed: false,
    }
}

#[must_use]
pub fn follow_command_grovel(_state: CommandNextTurnState) -> CommandNextTurnState {
    CommandNextTurnState {
        scenario: CommandNextTurnScenario::FollowGrovel,
        protocol: CommandNextTurnProtocol::Resolved,
        target_prone: true,
        target_effect_count: 0,
        action_available: true,
        bonus_action_available: true,
        movement_spent_feet: 0,
        current_actor: CommandTurnActor::Caster,
        pending_option: None,
        dropped_object_count: 0,
        reaction_window_open: false,
        halt_suppressed: false,
    }
}

#[must_use]
pub fn follow_command_drop(
    _state: CommandNextTurnState,
    dropped_object_count: i16,
) -> CommandNextTurnState {
    CommandNextTurnState {
        scenario: CommandNextTurnScenario::FollowDrop,
        protocol: CommandNextTurnProtocol::Resolved,
        target_prone: false,
        target_effect_count: 0,
        action_available: true,
        bonus_action_available: true,
        movement_spent_feet: 0,
        current_actor: CommandTurnActor::Caster,
        pending_option: None,
        dropped_object_count: dropped_object_count.max(0),
        reaction_window_open: false,
        halt_suppressed: false,
    }
}

#[must_use]
pub fn suppress_command_halt(
    _state: CommandNextTurnState,
    movement_spent_feet: i16,
) -> CommandNextTurnState {
    CommandNextTurnState {
        scenario: CommandNextTurnScenario::HaltSuppresses,
        protocol: CommandNextTurnProtocol::Resolved,
        target_prone: false,
        target_effect_count: 1,
        action_available: false,
        bonus_action_available: false,
        movement_spent_feet: movement_spent_feet.max(0),
        current_actor: CommandTurnActor::Target,
        pending_option: Some(CommandNextTurnOption::Halt),
        dropped_object_count: 0,
        reaction_window_open: false,
        halt_suppressed: true,
    }
}

#[must_use]
pub fn cleanup_command_halt_turn(_state: CommandNextTurnState) -> CommandNextTurnState {
    CommandNextTurnState {
        scenario: CommandNextTurnScenario::HaltEndTurnCleanup,
        protocol: CommandNextTurnProtocol::Resolved,
        target_prone: false,
        target_effect_count: 0,
        action_available: true,
        bonus_action_available: true,
        movement_spent_feet: 30,
        current_actor: CommandTurnActor::Caster,
        pending_option: None,
        dropped_object_count: 0,
        reaction_window_open: false,
        halt_suppressed: false,
    }
}

#[must_use]
pub fn continue_command_approach(
    _state: CommandNextTurnState,
    movement_spent_feet: i16,
) -> CommandNextTurnState {
    CommandNextTurnState {
        scenario: CommandNextTurnScenario::ApproachContinues,
        protocol: CommandNextTurnProtocol::Resolved,
        target_prone: false,
        target_effect_count: 0,
        action_available: true,
        bonus_action_available: true,
        movement_spent_feet: movement_spent_feet.max(0),
        current_actor: CommandTurnActor::Target,
        pending_option: None,
        dropped_object_count: 0,
        reaction_window_open: false,
        halt_suppressed: false,
    }
}

#[must_use]
pub fn end_command_approach_within_five_feet(
    mut state: CommandNextTurnState,
) -> CommandNextTurnState {
    state.scenario = CommandNextTurnScenario::ApproachWithinFiveEndsTurn;
    state.current_actor = CommandTurnActor::Caster;
    state.protocol = CommandNextTurnProtocol::Resolved;
    state
}

#[must_use]
pub fn reject_command_approach_movement(_state: CommandNextTurnState) -> CommandNextTurnState {
    CommandNextTurnState {
        scenario: CommandNextTurnScenario::ApproachMovementRejected,
        protocol: CommandNextTurnProtocol::Invalid(CommandNextTurnInvalidReason::InvalidFill),
        target_prone: false,
        target_effect_count: 1,
        action_available: true,
        bonus_action_available: true,
        movement_spent_feet: 0,
        current_actor: CommandTurnActor::Target,
        pending_option: Some(CommandNextTurnOption::Approach),
        dropped_object_count: 0,
        reaction_window_open: false,
        halt_suppressed: false,
    }
}

#[must_use]
pub fn cleanup_command_approach_no_movement(_state: CommandNextTurnState) -> CommandNextTurnState {
    CommandNextTurnState {
        scenario: CommandNextTurnScenario::ApproachNoMovementCleanup,
        protocol: CommandNextTurnProtocol::Resolved,
        target_prone: false,
        target_effect_count: 0,
        action_available: true,
        bonus_action_available: true,
        movement_spent_feet: 0,
        current_actor: CommandTurnActor::Target,
        pending_option: None,
        dropped_object_count: 0,
        reaction_window_open: false,
        halt_suppressed: false,
    }
}

#[must_use]
pub fn resolve_command_flee_full_movement(
    _state: CommandNextTurnState,
    movement_spent_feet: i16,
) -> CommandNextTurnState {
    CommandNextTurnState {
        scenario: CommandNextTurnScenario::FleeFullMovementEndsTurn,
        protocol: CommandNextTurnProtocol::Resolved,
        target_prone: false,
        target_effect_count: 0,
        action_available: true,
        bonus_action_available: true,
        movement_spent_feet: movement_spent_feet.max(0),
        current_actor: CommandTurnActor::Caster,
        pending_option: None,
        dropped_object_count: 0,
        reaction_window_open: false,
        halt_suppressed: false,
    }
}

#[must_use]
pub fn reject_command_flee_partial_movement(_state: CommandNextTurnState) -> CommandNextTurnState {
    CommandNextTurnState {
        scenario: CommandNextTurnScenario::FleePartialMovementRejected,
        protocol: CommandNextTurnProtocol::Invalid(CommandNextTurnInvalidReason::InvalidFill),
        target_prone: false,
        target_effect_count: 1,
        action_available: true,
        bonus_action_available: true,
        movement_spent_feet: 0,
        current_actor: CommandTurnActor::Target,
        pending_option: Some(CommandNextTurnOption::Flee),
        dropped_object_count: 0,
        reaction_window_open: false,
        halt_suppressed: false,
    }
}

#[must_use]
pub fn cleanup_command_flee_no_movement(_state: CommandNextTurnState) -> CommandNextTurnState {
    CommandNextTurnState {
        scenario: CommandNextTurnScenario::FleeNoMovementCleanup,
        protocol: CommandNextTurnProtocol::Resolved,
        target_prone: false,
        target_effect_count: 0,
        action_available: true,
        bonus_action_available: true,
        movement_spent_feet: 0,
        current_actor: CommandTurnActor::Caster,
        pending_option: None,
        dropped_object_count: 0,
        reaction_window_open: false,
        halt_suppressed: false,
    }
}

#[must_use]
pub fn open_command_flee_opportunity_attack_window(
    _state: CommandNextTurnState,
) -> CommandNextTurnState {
    // RAW: cleanroom-input/raw/srd-5.2.1/Playing-the-Game.md
    // "Opportunity Attacks"; Rules-Glossary.md "Opportunity Attacks".
    CommandNextTurnState {
        scenario: CommandNextTurnScenario::FleeOpportunityAttackWindow,
        protocol: CommandNextTurnProtocol::NeedsInterruptDecision,
        target_prone: false,
        target_effect_count: 1,
        action_available: true,
        bonus_action_available: true,
        movement_spent_feet: 0,
        current_actor: CommandTurnActor::Target,
        pending_option: Some(CommandNextTurnOption::Flee),
        dropped_object_count: 0,
        reaction_window_open: true,
        halt_suppressed: false,
    }
}

#[must_use]
pub fn decline_command_flee_opportunity_attack(
    _state: CommandNextTurnState,
    movement_spent_feet: i16,
) -> CommandNextTurnState {
    CommandNextTurnState {
        scenario: CommandNextTurnScenario::FleeOpportunityAttackDeclinedContinuation,
        protocol: CommandNextTurnProtocol::Resolved,
        target_prone: false,
        target_effect_count: 0,
        action_available: true,
        bonus_action_available: true,
        movement_spent_feet: movement_spent_feet.max(0),
        current_actor: CommandTurnActor::Caster,
        pending_option: None,
        dropped_object_count: 0,
        reaction_window_open: false,
        halt_suppressed: false,
    }
}

#[must_use]
pub fn complete_command_option_sequence(state: CommandNextTurnState) -> CommandNextTurnState {
    state
}
