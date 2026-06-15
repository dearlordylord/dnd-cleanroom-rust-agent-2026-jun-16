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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CommandFrontierStage {
    ActSelection,
    TargetListAndOptionChoice,
    TargetList,
    OptionChoice,
    SavingThrowOutcome,
    DropHeldObjectFacts,
    ApproachMovement,
    FleeMovement,
    Resolved,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CommandHoleKind {
    TargetChoice,
    SpellTargetList,
    CommandOptionChoice,
    SavingThrowOutcome,
    Movement,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CommandFillKind {
    SpellTargetList,
    CommandOptionChoice,
    SavingThrowOutcome,
    Movement,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CommandFillOrderingError {
    TargetListRequired,
    OptionChoiceRequired,
    SavingThrowRequired,
    HeldObjectFactsRequired,
    MovementRequired,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CommandFillOrderResult {
    Accepted(CommandFrontierStage),
    RequestedEarlier {
        error: CommandFillOrderingError,
        stage: CommandFrontierStage,
    },
    Rejected(CommandFillOrderingError),
    NotOrderingError(CommandFrontierStage),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CommandOrderingProtocol {
    Init,
    NeedsHoles(Vec<CommandHoleKind>),
    Resolved,
    Invalid {
        holes: Vec<CommandHoleKind>,
        reason: CommandNextTurnInvalidReason,
    },
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CommandOrderingState {
    pub stage: CommandFrontierStage,
    pub protocol: CommandOrderingProtocol,
    pub table_fact_frontier_open: bool,
    pub last_ordering_error: Option<CommandFillOrderingError>,
    pub pending_option: Option<CommandNextTurnOption>,
    pub target_prone: bool,
    pub dropped_object_count: i16,
    pub halt_suppressed: bool,
    pub movement_spent_feet: i16,
    pub current_actor: CommandTurnActor,
    pub reaction_window_open: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CommandOrderingFillFacts {
    pub fill_kind: CommandFillKind,
    pub option: CommandNextTurnOption,
    pub failed_saving_throw: bool,
    pub movement_available: bool,
    pub held_object_facts_required: bool,
    pub moved_within_five_feet_of_caster: bool,
    pub opened_opportunity_attack: bool,
}

#[must_use]
pub fn command_ordering_initial_state() -> CommandOrderingState {
    command_ordering_projection(CommandOrderingProjectionFacts {
        stage: CommandFrontierStage::ActSelection,
        runtime_result: CommandOrderingRuntimeResult::Init,
        last_ordering_error: None,
        pending_option: None,
        target_prone: false,
        dropped_object_count: 0,
        halt_suppressed: false,
        movement_spent_feet: 0,
        current_actor: CommandTurnActor::Caster,
        reaction_window_open: false,
    })
}

#[must_use]
pub fn command_hole_frontier(stage: CommandFrontierStage) -> Vec<CommandHoleKind> {
    // QNT: cleanroom-input/qnt/battle-runtime/battle-runtime-command-ordering.qnt.
    match stage {
        CommandFrontierStage::ActSelection
        | CommandFrontierStage::DropHeldObjectFacts
        | CommandFrontierStage::Resolved => vec![],
        CommandFrontierStage::TargetListAndOptionChoice => vec![
            CommandHoleKind::SpellTargetList,
            CommandHoleKind::CommandOptionChoice,
        ],
        CommandFrontierStage::TargetList => vec![CommandHoleKind::SpellTargetList],
        CommandFrontierStage::OptionChoice => vec![CommandHoleKind::CommandOptionChoice],
        CommandFrontierStage::SavingThrowOutcome => vec![CommandHoleKind::SavingThrowOutcome],
        CommandFrontierStage::ApproachMovement | CommandFrontierStage::FleeMovement => {
            vec![CommandHoleKind::Movement]
        }
    }
}

#[must_use]
pub fn command_hole_frontier_includes_table_facts(stage: CommandFrontierStage) -> bool {
    stage == CommandFrontierStage::DropHeldObjectFacts
}

#[must_use]
pub fn command_after_saving_throw_stage(
    option: CommandNextTurnOption,
    failed_saving_throw: bool,
    movement_available: bool,
    held_object_facts_required: bool,
) -> CommandFrontierStage {
    if !failed_saving_throw {
        return CommandFrontierStage::Resolved;
    }

    match option {
        CommandNextTurnOption::Grovel | CommandNextTurnOption::Halt => {
            CommandFrontierStage::Resolved
        }
        CommandNextTurnOption::Drop if held_object_facts_required => {
            CommandFrontierStage::DropHeldObjectFacts
        }
        CommandNextTurnOption::Drop => CommandFrontierStage::Resolved,
        CommandNextTurnOption::Approach if movement_available => {
            CommandFrontierStage::ApproachMovement
        }
        CommandNextTurnOption::Approach => CommandFrontierStage::Resolved,
        CommandNextTurnOption::Flee if movement_available => CommandFrontierStage::FleeMovement,
        CommandNextTurnOption::Flee => CommandFrontierStage::Resolved,
    }
}

#[must_use]
pub fn command_after_movement_stage(
    _option: CommandNextTurnOption,
    _moved_within_five_feet_of_caster: bool,
    _opened_opportunity_attack: bool,
) -> CommandFrontierStage {
    CommandFrontierStage::Resolved
}

#[must_use]
pub fn command_fill_order_result(
    stage: CommandFrontierStage,
    facts: CommandOrderingFillFacts,
) -> CommandFillOrderResult {
    match stage {
        CommandFrontierStage::ActSelection => {
            CommandFillOrderResult::NotOrderingError(CommandFrontierStage::ActSelection)
        }
        CommandFrontierStage::TargetListAndOptionChoice => match facts.fill_kind {
            CommandFillKind::SpellTargetList => {
                CommandFillOrderResult::Accepted(CommandFrontierStage::OptionChoice)
            }
            CommandFillKind::CommandOptionChoice | CommandFillKind::SavingThrowOutcome => {
                CommandFillOrderResult::RequestedEarlier {
                    error: CommandFillOrderingError::TargetListRequired,
                    stage: CommandFrontierStage::TargetList,
                }
            }
            CommandFillKind::Movement => CommandFillOrderResult::NotOrderingError(
                CommandFrontierStage::TargetListAndOptionChoice,
            ),
        },
        CommandFrontierStage::TargetList => {
            if facts.fill_kind == CommandFillKind::SpellTargetList {
                CommandFillOrderResult::Accepted(CommandFrontierStage::OptionChoice)
            } else {
                CommandFillOrderResult::NotOrderingError(CommandFrontierStage::TargetList)
            }
        }
        CommandFrontierStage::OptionChoice => match facts.fill_kind {
            CommandFillKind::CommandOptionChoice => {
                CommandFillOrderResult::Accepted(CommandFrontierStage::SavingThrowOutcome)
            }
            CommandFillKind::SavingThrowOutcome => CommandFillOrderResult::RequestedEarlier {
                error: CommandFillOrderingError::OptionChoiceRequired,
                stage: CommandFrontierStage::OptionChoice,
            },
            CommandFillKind::SpellTargetList | CommandFillKind::Movement => {
                CommandFillOrderResult::NotOrderingError(CommandFrontierStage::OptionChoice)
            }
        },
        CommandFrontierStage::SavingThrowOutcome => match facts.fill_kind {
            CommandFillKind::SavingThrowOutcome => {
                CommandFillOrderResult::Accepted(command_after_saving_throw_stage(
                    facts.option,
                    facts.failed_saving_throw,
                    facts.movement_available,
                    facts.held_object_facts_required,
                ))
            }
            CommandFillKind::Movement => CommandFillOrderResult::RequestedEarlier {
                error: CommandFillOrderingError::SavingThrowRequired,
                stage: CommandFrontierStage::SavingThrowOutcome,
            },
            CommandFillKind::SpellTargetList | CommandFillKind::CommandOptionChoice => {
                CommandFillOrderResult::NotOrderingError(CommandFrontierStage::SavingThrowOutcome)
            }
        },
        CommandFrontierStage::DropHeldObjectFacts => {
            CommandFillOrderResult::NotOrderingError(CommandFrontierStage::DropHeldObjectFacts)
        }
        CommandFrontierStage::ApproachMovement => {
            if facts.fill_kind == CommandFillKind::Movement {
                CommandFillOrderResult::Accepted(command_after_movement_stage(
                    facts.option,
                    facts.moved_within_five_feet_of_caster,
                    facts.opened_opportunity_attack,
                ))
            } else {
                CommandFillOrderResult::NotOrderingError(CommandFrontierStage::ApproachMovement)
            }
        }
        CommandFrontierStage::FleeMovement => {
            if facts.fill_kind == CommandFillKind::Movement && facts.movement_available {
                CommandFillOrderResult::Accepted(command_after_movement_stage(
                    facts.option,
                    false,
                    facts.opened_opportunity_attack,
                ))
            } else if facts.fill_kind == CommandFillKind::Movement {
                CommandFillOrderResult::Rejected(CommandFillOrderingError::MovementRequired)
            } else {
                CommandFillOrderResult::NotOrderingError(CommandFrontierStage::FleeMovement)
            }
        }
        CommandFrontierStage::Resolved => {
            CommandFillOrderResult::NotOrderingError(CommandFrontierStage::Resolved)
        }
    }
}

#[must_use]
pub fn command_fill_order_accepted_stage(
    result: CommandFillOrderResult,
    fallback: CommandFrontierStage,
) -> CommandFrontierStage {
    match result {
        CommandFillOrderResult::Accepted(stage) => stage,
        CommandFillOrderResult::RequestedEarlier { stage, .. } => stage,
        CommandFillOrderResult::Rejected(_) => fallback,
        CommandFillOrderResult::NotOrderingError(stage) => stage,
    }
}

#[must_use]
pub fn command_fill_order_error(
    result: CommandFillOrderResult,
) -> Option<CommandFillOrderingError> {
    match result {
        CommandFillOrderResult::RequestedEarlier { error, .. }
        | CommandFillOrderResult::Rejected(error) => Some(error),
        CommandFillOrderResult::Accepted(_) | CommandFillOrderResult::NotOrderingError(_) => None,
    }
}

#[must_use]
pub fn command_fill_order_runtime_result(
    result: CommandFillOrderResult,
) -> CommandOrderingRuntimeResult {
    match result {
        CommandFillOrderResult::Accepted(CommandFrontierStage::Resolved) => {
            CommandOrderingRuntimeResult::Resolved
        }
        CommandFillOrderResult::Accepted(_)
        | CommandFillOrderResult::RequestedEarlier { .. }
        | CommandFillOrderResult::NotOrderingError(_) => CommandOrderingRuntimeResult::NeedsHoles,
        CommandFillOrderResult::Rejected(_) => CommandOrderingRuntimeResult::Invalid,
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CommandOrderingRuntimeResult {
    Init,
    NeedsHoles,
    Resolved,
    Invalid,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CommandOrderingProjectionFacts {
    pub stage: CommandFrontierStage,
    pub runtime_result: CommandOrderingRuntimeResult,
    pub last_ordering_error: Option<CommandFillOrderingError>,
    pub pending_option: Option<CommandNextTurnOption>,
    pub target_prone: bool,
    pub dropped_object_count: i16,
    pub halt_suppressed: bool,
    pub movement_spent_feet: i16,
    pub current_actor: CommandTurnActor,
    pub reaction_window_open: bool,
}

#[must_use]
pub fn command_ordering_projection(facts: CommandOrderingProjectionFacts) -> CommandOrderingState {
    let holes = command_hole_frontier(facts.stage);
    CommandOrderingState {
        stage: facts.stage,
        protocol: command_ordering_protocol(holes, facts.runtime_result),
        table_fact_frontier_open: command_hole_frontier_includes_table_facts(facts.stage),
        last_ordering_error: facts.last_ordering_error,
        pending_option: facts.pending_option,
        target_prone: facts.target_prone,
        dropped_object_count: facts.dropped_object_count.max(0),
        halt_suppressed: facts.halt_suppressed,
        movement_spent_feet: facts.movement_spent_feet.max(0),
        current_actor: facts.current_actor,
        reaction_window_open: facts.reaction_window_open,
    }
}

fn command_ordering_protocol(
    holes: Vec<CommandHoleKind>,
    runtime_result: CommandOrderingRuntimeResult,
) -> CommandOrderingProtocol {
    match runtime_result {
        CommandOrderingRuntimeResult::Init => CommandOrderingProtocol::Init,
        CommandOrderingRuntimeResult::Invalid => CommandOrderingProtocol::Invalid {
            holes,
            reason: CommandNextTurnInvalidReason::InvalidFill,
        },
        CommandOrderingRuntimeResult::NeedsHoles if !holes.is_empty() => {
            CommandOrderingProtocol::NeedsHoles(holes)
        }
        CommandOrderingRuntimeResult::NeedsHoles | CommandOrderingRuntimeResult::Resolved => {
            CommandOrderingProtocol::Resolved
        }
    }
}
