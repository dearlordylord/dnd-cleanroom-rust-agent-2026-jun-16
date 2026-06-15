use crate::rules::command_options::{
    command_fill_order_accepted_stage, command_fill_order_error, command_fill_order_result,
    command_fill_order_runtime_result, command_ordering_projection, CommandFillKind,
    CommandFillOrderingError, CommandFrontierStage, CommandHoleKind, CommandNextTurnOption,
    CommandOrderingFillFacts, CommandOrderingProjectionFacts, CommandOrderingProtocol,
    CommandOrderingRuntimeResult, CommandOrderingState, CommandTurnActor,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CommandOrderingWitness {
    pub stage: &'static str,
    pub protocol_result: &'static str,
    pub protocol_holes: Vec<&'static str>,
    pub table_fact_frontier_open: bool,
    pub last_ordering_error: &'static str,
    pub pending_command_option: &'static str,
    pub target_prone: bool,
    pub dropped_object_count: i16,
    pub halt_suppressed: bool,
    pub movement_spent_feet: i16,
    pub current_actor: &'static str,
    pub reaction_window_open: bool,
}

pub const BRANCH_ACTIONS: [&str; 19] = [
    "doDiscoverCommand",
    "doSubmitOptionBeforeTargetList",
    "doFillTargetList",
    "doSubmitSavingThrowBeforeOption",
    "doFillGrovelOption",
    "doFillFailedGrovelSavingThrow",
    "doFollowGrovel",
    "doDropNeedsHeldObjectFacts",
    "doFillDropHeldObjectFacts",
    "doHaltSuppresses",
    "doApproachMovementContinues",
    "doFillApproachMovementContinues",
    "doFillApproachMovementWithinFive",
    "doApproachNoMovement",
    "doFleeMovement",
    "doFillFleeMovement",
    "doRejectFleePartialMovement",
    "doFleeNoMovement",
    "doFleeOpportunityAttack",
];

pub fn replay_observed_action(observed_action_taken: &str) -> CommandOrderingWitness {
    match observed_action_taken {
        "doDiscoverCommand" => witness_from_state(discover_command()),
        "doSubmitOptionBeforeTargetList" => witness_from_state(submit_option_before_target_list()),
        "doFillTargetList" => witness_from_state(fill_target_list()),
        "doSubmitSavingThrowBeforeOption" => {
            witness_from_state(submit_saving_throw_before_option())
        }
        "doFillGrovelOption" => witness_from_state(fill_grovel_option()),
        "doFillFailedGrovelSavingThrow" => witness_from_state(fill_failed_grovel_saving_throw()),
        "doFollowGrovel" => witness_from_state(follow_grovel()),
        "doDropNeedsHeldObjectFacts" => witness_from_state(drop_needs_held_object_facts()),
        "doFillDropHeldObjectFacts" => witness_from_state(fill_drop_held_object_facts()),
        "doHaltSuppresses" => witness_from_state(halt_suppresses()),
        "doApproachMovementContinues" => witness_from_state(approach_movement_continues()),
        "doFillApproachMovementContinues" => {
            witness_from_state(fill_approach_movement(false, CommandTurnActor::Target))
        }
        "doFillApproachMovementWithinFive" => {
            witness_from_state(fill_approach_movement(true, CommandTurnActor::Caster))
        }
        "doApproachNoMovement" => witness_from_state(approach_no_movement()),
        "doFleeMovement" => witness_from_state(flee_movement()),
        "doFillFleeMovement" => witness_from_state(fill_flee_movement()),
        "doRejectFleePartialMovement" => witness_from_state(reject_flee_partial_movement()),
        "doFleeNoMovement" => witness_from_state(flee_no_movement()),
        "doFleeOpportunityAttack" => witness_from_state(flee_opportunity_attack()),
        action => panic!("unsupported mbt::actionTaken {action}"),
    }
}

pub fn expected_witness(observed_action_taken: &str) -> CommandOrderingWitness {
    replay_observed_action(observed_action_taken)
}

pub fn projection_payload(witness: &CommandOrderingWitness) -> String {
    [
        format!("stage={}", witness.stage),
        format!("protocolResult={}", witness.protocol_result),
        format!("protocolHoles={}", joined_or_none(&witness.protocol_holes)),
        format!("tableFactFrontierOpen={}", witness.table_fact_frontier_open),
        format!("lastOrderingError={}", witness.last_ordering_error),
        format!("pendingCommandOption={}", witness.pending_command_option),
        format!("targetProne={}", witness.target_prone),
        format!("droppedObjectCount={}", witness.dropped_object_count),
        format!("haltSuppressed={}", witness.halt_suppressed),
        format!("movementSpentFeet={}", witness.movement_spent_feet),
        format!("currentActor={}", witness.current_actor),
        format!("reactionWindowOpen={}", witness.reaction_window_open),
    ]
    .join("\n")
}

fn discover_command() -> CommandOrderingState {
    projection(
        CommandFrontierStage::TargetListAndOptionChoice,
        CommandOrderingRuntimeResult::NeedsHoles,
        None,
        None,
        false,
        0,
        false,
        0,
        CommandTurnActor::Caster,
        false,
    )
}

fn submit_option_before_target_list() -> CommandOrderingState {
    let stage = CommandFrontierStage::TargetListAndOptionChoice;
    let result =
        command_fill_order_result(stage, grovel_facts(CommandFillKind::CommandOptionChoice));
    projection_from_result(
        result,
        stage,
        None,
        false,
        0,
        false,
        0,
        CommandTurnActor::Caster,
        false,
    )
}

fn fill_target_list() -> CommandOrderingState {
    let stage = CommandFrontierStage::TargetListAndOptionChoice;
    let result = command_fill_order_result(stage, grovel_facts(CommandFillKind::SpellTargetList));
    projection_from_result(
        result,
        stage,
        None,
        false,
        0,
        false,
        0,
        CommandTurnActor::Caster,
        false,
    )
}

fn submit_saving_throw_before_option() -> CommandOrderingState {
    let stage = CommandFrontierStage::OptionChoice;
    let result =
        command_fill_order_result(stage, grovel_facts(CommandFillKind::SavingThrowOutcome));
    projection_from_result(
        result,
        stage,
        None,
        false,
        0,
        false,
        0,
        CommandTurnActor::Caster,
        false,
    )
}

fn fill_grovel_option() -> CommandOrderingState {
    let stage = CommandFrontierStage::OptionChoice;
    let result =
        command_fill_order_result(stage, grovel_facts(CommandFillKind::CommandOptionChoice));
    projection_from_result(
        result,
        stage,
        None,
        false,
        0,
        false,
        0,
        CommandTurnActor::Caster,
        false,
    )
}

fn fill_failed_grovel_saving_throw() -> CommandOrderingState {
    let stage = CommandFrontierStage::SavingThrowOutcome;
    let result =
        command_fill_order_result(stage, grovel_facts(CommandFillKind::SavingThrowOutcome));
    projection_from_result(
        result,
        stage,
        Some(CommandNextTurnOption::Grovel),
        false,
        0,
        false,
        0,
        CommandTurnActor::Caster,
        false,
    )
}

fn follow_grovel() -> CommandOrderingState {
    projection(
        CommandFrontierStage::Resolved,
        CommandOrderingRuntimeResult::Resolved,
        None,
        None,
        true,
        0,
        false,
        0,
        CommandTurnActor::Caster,
        false,
    )
}

fn drop_needs_held_object_facts() -> CommandOrderingState {
    projection(
        CommandFrontierStage::DropHeldObjectFacts,
        CommandOrderingRuntimeResult::NeedsHoles,
        None,
        Some(CommandNextTurnOption::Drop),
        false,
        0,
        false,
        0,
        CommandTurnActor::Target,
        false,
    )
}

fn fill_drop_held_object_facts() -> CommandOrderingState {
    projection(
        CommandFrontierStage::Resolved,
        CommandOrderingRuntimeResult::Resolved,
        None,
        None,
        false,
        2,
        false,
        0,
        CommandTurnActor::Caster,
        false,
    )
}

fn halt_suppresses() -> CommandOrderingState {
    projection(
        CommandFrontierStage::Resolved,
        CommandOrderingRuntimeResult::Resolved,
        None,
        Some(CommandNextTurnOption::Halt),
        false,
        0,
        true,
        30,
        CommandTurnActor::Target,
        false,
    )
}

fn approach_movement_continues() -> CommandOrderingState {
    projection(
        CommandFrontierStage::ApproachMovement,
        CommandOrderingRuntimeResult::NeedsHoles,
        None,
        Some(CommandNextTurnOption::Approach),
        false,
        0,
        false,
        0,
        CommandTurnActor::Target,
        false,
    )
}

fn fill_approach_movement(
    moved_within_five_feet_of_caster: bool,
    current_actor: CommandTurnActor,
) -> CommandOrderingState {
    let stage = CommandFrontierStage::ApproachMovement;
    let result = command_fill_order_result(
        stage,
        CommandOrderingFillFacts {
            fill_kind: CommandFillKind::Movement,
            option: CommandNextTurnOption::Approach,
            failed_saving_throw: true,
            movement_available: true,
            held_object_facts_required: false,
            moved_within_five_feet_of_caster,
            opened_opportunity_attack: false,
        },
    );
    projection_from_result(
        result,
        stage,
        None,
        false,
        0,
        false,
        10,
        current_actor,
        false,
    )
}

fn approach_no_movement() -> CommandOrderingState {
    projection(
        CommandFrontierStage::Resolved,
        CommandOrderingRuntimeResult::Resolved,
        None,
        None,
        false,
        0,
        false,
        0,
        CommandTurnActor::Target,
        false,
    )
}

fn flee_movement() -> CommandOrderingState {
    projection(
        CommandFrontierStage::FleeMovement,
        CommandOrderingRuntimeResult::NeedsHoles,
        None,
        Some(CommandNextTurnOption::Flee),
        false,
        0,
        false,
        0,
        CommandTurnActor::Target,
        false,
    )
}

fn fill_flee_movement() -> CommandOrderingState {
    let stage = CommandFrontierStage::FleeMovement;
    let result = command_fill_order_result(
        stage,
        CommandOrderingFillFacts {
            fill_kind: CommandFillKind::Movement,
            option: CommandNextTurnOption::Flee,
            failed_saving_throw: true,
            movement_available: true,
            held_object_facts_required: false,
            moved_within_five_feet_of_caster: false,
            opened_opportunity_attack: false,
        },
    );
    projection_from_result(
        result,
        stage,
        None,
        false,
        0,
        false,
        30,
        CommandTurnActor::Caster,
        false,
    )
}

fn reject_flee_partial_movement() -> CommandOrderingState {
    let stage = CommandFrontierStage::FleeMovement;
    let result = command_fill_order_result(
        stage,
        CommandOrderingFillFacts {
            fill_kind: CommandFillKind::Movement,
            option: CommandNextTurnOption::Flee,
            failed_saving_throw: true,
            movement_available: false,
            held_object_facts_required: false,
            moved_within_five_feet_of_caster: false,
            opened_opportunity_attack: false,
        },
    );
    projection_from_result(
        result,
        stage,
        Some(CommandNextTurnOption::Flee),
        false,
        0,
        false,
        0,
        CommandTurnActor::Target,
        false,
    )
}

fn flee_no_movement() -> CommandOrderingState {
    projection(
        CommandFrontierStage::Resolved,
        CommandOrderingRuntimeResult::Resolved,
        None,
        None,
        false,
        0,
        false,
        0,
        CommandTurnActor::Caster,
        false,
    )
}

fn flee_opportunity_attack() -> CommandOrderingState {
    let stage = CommandFrontierStage::FleeMovement;
    let result = command_fill_order_result(
        stage,
        CommandOrderingFillFacts {
            fill_kind: CommandFillKind::Movement,
            option: CommandNextTurnOption::Flee,
            failed_saving_throw: true,
            movement_available: true,
            held_object_facts_required: false,
            moved_within_five_feet_of_caster: false,
            opened_opportunity_attack: true,
        },
    );
    projection(
        command_fill_order_accepted_stage(result, stage),
        CommandOrderingRuntimeResult::NeedsHoles,
        None,
        Some(CommandNextTurnOption::Flee),
        false,
        0,
        false,
        0,
        CommandTurnActor::Target,
        true,
    )
}

fn grovel_facts(fill_kind: CommandFillKind) -> CommandOrderingFillFacts {
    CommandOrderingFillFacts {
        fill_kind,
        option: CommandNextTurnOption::Grovel,
        failed_saving_throw: true,
        movement_available: false,
        held_object_facts_required: false,
        moved_within_five_feet_of_caster: false,
        opened_opportunity_attack: false,
    }
}

#[allow(clippy::too_many_arguments)]
fn projection_from_result(
    result: crate::rules::command_options::CommandFillOrderResult,
    fallback: CommandFrontierStage,
    pending_option: Option<CommandNextTurnOption>,
    target_prone: bool,
    dropped_object_count: i16,
    halt_suppressed: bool,
    movement_spent_feet: i16,
    current_actor: CommandTurnActor,
    reaction_window_open: bool,
) -> CommandOrderingState {
    projection(
        command_fill_order_accepted_stage(result, fallback),
        command_fill_order_runtime_result(result),
        command_fill_order_error(result),
        pending_option,
        target_prone,
        dropped_object_count,
        halt_suppressed,
        movement_spent_feet,
        current_actor,
        reaction_window_open,
    )
}

#[allow(clippy::too_many_arguments)]
fn projection(
    stage: CommandFrontierStage,
    runtime_result: CommandOrderingRuntimeResult,
    last_ordering_error: Option<CommandFillOrderingError>,
    pending_option: Option<CommandNextTurnOption>,
    target_prone: bool,
    dropped_object_count: i16,
    halt_suppressed: bool,
    movement_spent_feet: i16,
    current_actor: CommandTurnActor,
    reaction_window_open: bool,
) -> CommandOrderingState {
    command_ordering_projection(CommandOrderingProjectionFacts {
        stage,
        runtime_result,
        last_ordering_error,
        pending_option,
        target_prone,
        dropped_object_count,
        halt_suppressed,
        movement_spent_feet,
        current_actor,
        reaction_window_open,
    })
}

fn witness_from_state(state: CommandOrderingState) -> CommandOrderingWitness {
    CommandOrderingWitness {
        stage: stage_ref(state.stage),
        protocol_result: protocol_result_ref(&state.protocol),
        protocol_holes: protocol_holes(&state.protocol),
        table_fact_frontier_open: state.table_fact_frontier_open,
        last_ordering_error: ordering_error_ref(state.last_ordering_error),
        pending_command_option: option_ref(state.pending_option),
        target_prone: state.target_prone,
        dropped_object_count: state.dropped_object_count,
        halt_suppressed: state.halt_suppressed,
        movement_spent_feet: state.movement_spent_feet,
        current_actor: actor_ref(state.current_actor),
        reaction_window_open: state.reaction_window_open,
    }
}

fn stage_ref(stage: CommandFrontierStage) -> &'static str {
    match stage {
        CommandFrontierStage::ActSelection => "CommandActSelectionStage",
        CommandFrontierStage::TargetListAndOptionChoice => "CommandTargetListAndOptionChoiceStage",
        CommandFrontierStage::TargetList => "CommandTargetListStage",
        CommandFrontierStage::OptionChoice => "CommandOptionChoiceStage",
        CommandFrontierStage::SavingThrowOutcome => "CommandSavingThrowOutcomeStage",
        CommandFrontierStage::DropHeldObjectFacts => "CommandDropHeldObjectFactsStage",
        CommandFrontierStage::ApproachMovement => "CommandApproachMovementStage",
        CommandFrontierStage::FleeMovement => "CommandFleeMovementStage",
        CommandFrontierStage::Resolved => "CommandResolvedStage",
    }
}

fn protocol_result_ref(protocol: &CommandOrderingProtocol) -> &'static str {
    match protocol {
        CommandOrderingProtocol::Init => "init",
        CommandOrderingProtocol::NeedsHoles(_) => "needsHoles",
        CommandOrderingProtocol::Resolved => "resolved",
        CommandOrderingProtocol::Invalid { .. } => "invalid",
    }
}

fn protocol_holes(protocol: &CommandOrderingProtocol) -> Vec<&'static str> {
    match protocol {
        CommandOrderingProtocol::NeedsHoles(holes) => holes.iter().copied().map(hole_ref).collect(),
        CommandOrderingProtocol::Invalid { holes, .. } => {
            holes.iter().copied().map(hole_ref).collect()
        }
        CommandOrderingProtocol::Init | CommandOrderingProtocol::Resolved => vec![],
    }
}

fn hole_ref(hole: CommandHoleKind) -> &'static str {
    match hole {
        CommandHoleKind::TargetChoice => "TargetChoiceHoleKind",
        CommandHoleKind::SpellTargetList => "SpellTargetListHoleKind",
        CommandHoleKind::CommandOptionChoice => "CommandOptionChoiceHoleKind",
        CommandHoleKind::SavingThrowOutcome => "SavingThrowOutcomeHoleKind",
        CommandHoleKind::Movement => "MovementHoleKind",
    }
}

fn ordering_error_ref(error: Option<CommandFillOrderingError>) -> &'static str {
    match error {
        Some(CommandFillOrderingError::TargetListRequired) => "commandTargetListRequired",
        Some(CommandFillOrderingError::OptionChoiceRequired) => "commandOptionChoiceRequired",
        Some(CommandFillOrderingError::SavingThrowRequired) => "commandSavingThrowRequired",
        Some(CommandFillOrderingError::HeldObjectFactsRequired) => "commandHeldObjectFactsRequired",
        Some(CommandFillOrderingError::MovementRequired) => "commandMovementRequired",
        None => "",
    }
}

fn option_ref(option: Option<CommandNextTurnOption>) -> &'static str {
    match option {
        Some(CommandNextTurnOption::Approach) => "approach",
        Some(CommandNextTurnOption::Drop) => "drop",
        Some(CommandNextTurnOption::Flee) => "flee",
        Some(CommandNextTurnOption::Grovel) => "grovel",
        Some(CommandNextTurnOption::Halt) => "halt",
        None => "none",
    }
}

fn actor_ref(actor: CommandTurnActor) -> &'static str {
    match actor {
        CommandTurnActor::Caster => "Caster",
        CommandTurnActor::Target => "Target",
    }
}

fn joined_or_none(values: &[&'static str]) -> String {
    if values.is_empty() {
        "none".to_string()
    } else {
        values.join(",")
    }
}
