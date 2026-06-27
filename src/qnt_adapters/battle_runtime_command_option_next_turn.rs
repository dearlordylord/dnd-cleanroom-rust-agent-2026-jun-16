use crate::rules::battle_reducer_spine::{
    active_command_battle_subject, discover_battle_acts, resolve_battle_subject,
    start_standard_battle, Actor, BattleCommandEffectFill, BattleResolutionInvalidReason,
    BattleResolutionRequest, BattleState, BattleSubject, BattleSubjectKind,
    CommandPendingOptionCleanup, CommandPendingOptionFollow,
};
use crate::rules::command_options::{
    cleanup_command_approach_no_movement, cleanup_command_flee_no_movement,
    cleanup_command_halt_turn, command_next_turn_initial_state, complete_command_option_sequence,
    continue_command_approach, decline_command_flee_opportunity_attack,
    end_command_approach_within_five_feet, follow_command_drop, follow_command_grovel,
    open_command_flee_opportunity_attack_window, record_command_failed_save_pending,
    reject_command_approach_movement, reject_command_flee_partial_movement,
    resolve_command_flee_full_movement, suppress_command_halt, CommandNextTurnInvalidReason,
    CommandNextTurnOption, CommandNextTurnProtocol, CommandNextTurnScenario, CommandNextTurnState,
    CommandTurnActor,
};

use super::battle_runtime_reducer_route::{
    battle_resolution_continuation, route_discover_battle_acts,
    route_discover_battle_acts_from_route_holes, route_resolve_battle_subject_from_result,
    route_resolve_battle_subject_from_route_result,
    route_resolve_battle_subject_without_fill_from_route_result, route_start_battle,
    ReducerRouteEvent, ReducerRouteFillKind, ReducerRouteHoleKind, ReducerRouteOwnerGroup,
    ReducerRouteResolutionOutcome, ReducerRouteResolveConnector, ReducerRouteResolveFill,
    ReducerRouteSubjectFamily,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CommandOptionNextTurnWitness {
    pub scenario: &'static str,
    pub protocol_result: &'static str,
    pub protocol_holes: Vec<&'static str>,
    pub invalid_reason: &'static str,
    pub target_prone: bool,
    pub target_effect_count: i16,
    pub action_available: bool,
    pub bonus_action_available: bool,
    pub movement_spent_feet: i16,
    pub current_actor: &'static str,
    pub pending_command_option: &'static str,
    pub dropped_object_count: i16,
    pub reaction_window_open: bool,
    pub halt_suppressed: bool,
}

pub const BRANCH_ACTIONS: [&str; 15] = [
    "doFailedSaveRecordsPending",
    "doFollowGrovel",
    "doFollowDrop",
    "doHaltSuppresses",
    "doHaltEndTurnCleanup",
    "doApproachContinues",
    "doApproachWithinFiveEndsTurn",
    "doApproachMovementRejected",
    "doApproachNoMovementCleanup",
    "doFleeFullMovementEndsTurn",
    "doFleePartialMovementRejected",
    "doFleeNoMovementCleanup",
    "doFleeOpportunityAttackWindow",
    "doFleeOpportunityAttackDeclinedContinuation",
    "doComplete",
];

pub fn replay_observed_action(observed_action_taken: &str) -> CommandOptionNextTurnWitness {
    match observed_action_taken {
        "doFailedSaveRecordsPending" => {
            witness_from_state(failed_save(CommandNextTurnOption::Grovel))
        }
        "doFollowGrovel" => witness_from_state(follow_command_grovel(failed_save(
            CommandNextTurnOption::Grovel,
        ))),
        "doFollowDrop" => witness_from_state(follow_command_drop(
            failed_save(CommandNextTurnOption::Drop),
            1,
        )),
        "doHaltSuppresses" => witness_from_state(halt_suppressed()),
        "doHaltEndTurnCleanup" => witness_from_state(cleanup_command_halt_turn(halt_suppressed())),
        "doApproachContinues" => witness_from_state(approach_continues()),
        "doApproachWithinFiveEndsTurn" => {
            witness_from_state(end_command_approach_within_five_feet(approach_continues()))
        }
        "doApproachMovementRejected" => witness_from_state(approach_rejected()),
        "doApproachNoMovementCleanup" => {
            witness_from_state(cleanup_command_approach_no_movement(approach_rejected()))
        }
        "doFleeFullMovementEndsTurn" => witness_from_state(resolve_command_flee_full_movement(
            failed_save(CommandNextTurnOption::Flee),
            30,
        )),
        "doFleePartialMovementRejected" => witness_from_state(flee_rejected()),
        "doFleeNoMovementCleanup" => {
            witness_from_state(cleanup_command_flee_no_movement(flee_rejected()))
        }
        "doFleeOpportunityAttackWindow" => witness_from_state(flee_opportunity_window()),
        "doFleeOpportunityAttackDeclinedContinuation" => {
            witness_from_state(flee_opportunity_declined())
        }
        "doComplete" => {
            witness_from_state(complete_command_option_sequence(flee_opportunity_declined()))
        }
        action => panic!("unsupported mbt::actionTaken {action}"),
    }
}

pub fn expected_witness(observed_action_taken: &str) -> CommandOptionNextTurnWitness {
    replay_observed_action(observed_action_taken)
}

pub fn projection_payload(witness: &CommandOptionNextTurnWitness) -> String {
    [
        format!("scenario={}", witness.scenario),
        format!("protocolResult={}", witness.protocol_result),
        format!("protocolHoles={}", joined_or_none(&witness.protocol_holes)),
        format!("invalidReason={}", witness.invalid_reason),
        format!("targetProne={}", witness.target_prone),
        format!("targetEffectCount={}", witness.target_effect_count),
        format!("actionAvailable={}", witness.action_available),
        format!("bonusActionAvailable={}", witness.bonus_action_available),
        format!("movementSpentFeet={}", witness.movement_spent_feet),
        format!("currentActor={}", witness.current_actor),
        format!("pendingCommandOption={}", witness.pending_command_option),
        format!("droppedObjectCount={}", witness.dropped_object_count),
        format!("reactionWindowOpen={}", witness.reaction_window_open),
        format!("haltSuppressed={}", witness.halt_suppressed),
    ]
    .join("\n")
}

pub fn replay_observed_route(observed_action_taken: &str) -> Vec<ReducerRouteEvent> {
    match observed_action_taken {
        "doFailedSaveRecordsPending" => command_route_from_start(
            BattleCommandEffectFill::SavingThrowOutcome {
                option: CommandNextTurnOption::Grovel,
                failed: true,
                movement_available: false,
                held_object_facts_required: false,
            },
            ReducerRouteResolveFill::Fill(ReducerRouteFillKind::SavingThrowOutcome),
            ReducerRouteOwnerGroup::ActiveEffect,
        ),
        "doFollowGrovel" => command_follow_route_from_failed_save(
            CommandNextTurnOption::Grovel,
            BattleCommandEffectFill::FollowPendingOption(CommandPendingOptionFollow::Grovel),
            ReducerRouteResolveFill::WithoutFill,
            ReducerRouteOwnerGroup::ConditionLifecycle,
        ),
        "doFollowDrop" => command_follow_route_from_failed_save(
            CommandNextTurnOption::Drop,
            BattleCommandEffectFill::FollowPendingOption(CommandPendingOptionFollow::Drop {
                dropped_object_count: 1,
            }),
            ReducerRouteResolveFill::WithoutFill,
            ReducerRouteOwnerGroup::ActiveEffect,
        ),
        "doHaltSuppresses" => command_follow_route_from_failed_save(
            CommandNextTurnOption::Halt,
            BattleCommandEffectFill::FollowPendingOption(CommandPendingOptionFollow::Halt {
                movement_spent_feet: 30,
            }),
            ReducerRouteResolveFill::WithoutFill,
            ReducerRouteOwnerGroup::ActiveEffect,
        ),
        "doHaltEndTurnCleanup" => command_route_from_start(
            BattleCommandEffectFill::CleanupPendingOption(CommandPendingOptionCleanup::Halt),
            ReducerRouteResolveFill::WithoutFill,
            ReducerRouteOwnerGroup::ActiveEffect,
        ),
        "doApproachContinues" => command_follow_route_from_failed_save(
            CommandNextTurnOption::Approach,
            BattleCommandEffectFill::FollowPendingOption(CommandPendingOptionFollow::Approach {
                movement_spent_feet: 10,
            }),
            ReducerRouteResolveFill::WithoutFill,
            ReducerRouteOwnerGroup::MovementResource,
        ),
        "doApproachWithinFiveEndsTurn" => command_movement_route_from_failed_save(
            CommandNextTurnOption::Approach,
            true,
            true,
            false,
            10,
            ReducerRouteOwnerGroup::MovementResource,
        ),
        "doApproachMovementRejected" => command_movement_route_from_failed_save(
            CommandNextTurnOption::Approach,
            false,
            false,
            false,
            0,
            ReducerRouteOwnerGroup::HoleFrontier,
        ),
        "doApproachNoMovementCleanup" => command_route_from_start(
            BattleCommandEffectFill::CleanupPendingOption(
                CommandPendingOptionCleanup::ApproachNoMovement,
            ),
            ReducerRouteResolveFill::WithoutFill,
            ReducerRouteOwnerGroup::MovementResource,
        ),
        "doFleeFullMovementEndsTurn" => command_follow_route_from_failed_save(
            CommandNextTurnOption::Flee,
            BattleCommandEffectFill::FollowPendingOption(CommandPendingOptionFollow::Flee {
                movement_spent_feet: 30,
            }),
            ReducerRouteResolveFill::WithoutFill,
            ReducerRouteOwnerGroup::MovementResource,
        ),
        "doFleePartialMovementRejected" => command_movement_route_from_failed_save(
            CommandNextTurnOption::Flee,
            false,
            false,
            false,
            0,
            ReducerRouteOwnerGroup::HoleFrontier,
        ),
        "doFleeNoMovementCleanup" => command_route_from_start(
            BattleCommandEffectFill::CleanupPendingOption(
                CommandPendingOptionCleanup::FleeNoMovement,
            ),
            ReducerRouteResolveFill::WithoutFill,
            ReducerRouteOwnerGroup::MovementResource,
        ),
        "doFleeOpportunityAttackWindow" => command_movement_route_from_failed_save(
            CommandNextTurnOption::Flee,
            true,
            false,
            true,
            0,
            ReducerRouteOwnerGroup::InterruptStack,
        ),
        "doFleeOpportunityAttackDeclinedContinuation" => command_follow_route_from_failed_save(
            CommandNextTurnOption::Flee,
            BattleCommandEffectFill::FollowPendingOption(
                CommandPendingOptionFollow::FleeOpportunityAttackDeclined {
                    movement_spent_feet: 30,
                },
            ),
            ReducerRouteResolveFill::WithoutFill,
            ReducerRouteOwnerGroup::MovementResource,
        ),
        "doComplete" => command_route_from_start(
            BattleCommandEffectFill::Complete,
            ReducerRouteResolveFill::WithoutFill,
            ReducerRouteOwnerGroup::ActiveEffect,
        ),
        action => panic!("unsupported mbt::actionTaken {action}"),
    }
}

pub fn expected_route(observed_action_taken: &str) -> Vec<ReducerRouteEvent> {
    expected_command_option_next_turn_route(observed_action_taken)
}

fn expected_command_option_next_turn_route(observed_action_taken: &str) -> Vec<ReducerRouteEvent> {
    match observed_action_taken {
        "doFailedSaveRecordsPending" => {
            let mut route = expected_command_discovery_route();
            route.push(expected_command_fill(
                ReducerRouteFillKind::SavingThrowOutcome,
                ReducerRouteResolutionOutcome::NeedsHoles,
                vec![ReducerRouteHoleKind::SpellTargetList],
                ReducerRouteOwnerGroup::ActiveEffect,
            ));
            route
        }
        "doFollowGrovel" => expected_command_follow_route_from_failed_save(
            CommandNextTurnOption::Grovel,
            ReducerRouteOwnerGroup::ConditionLifecycle,
        ),
        "doFollowDrop" => expected_command_follow_route_from_failed_save(
            CommandNextTurnOption::Drop,
            ReducerRouteOwnerGroup::ActiveEffect,
        ),
        "doHaltSuppresses" => expected_command_follow_route_from_failed_save(
            CommandNextTurnOption::Halt,
            ReducerRouteOwnerGroup::ActiveEffect,
        ),
        "doHaltEndTurnCleanup" => expected_command_without_fill_from_start(
            ReducerRouteResolutionOutcome::Resolved,
            Vec::new(),
            ReducerRouteOwnerGroup::ActiveEffect,
        ),
        "doApproachContinues" => expected_command_follow_route_from_failed_save(
            CommandNextTurnOption::Approach,
            ReducerRouteOwnerGroup::MovementResource,
        ),
        "doApproachWithinFiveEndsTurn" => expected_command_movement_route_from_failed_save(
            CommandNextTurnOption::Approach,
            ReducerRouteResolutionOutcome::Resolved,
            Vec::new(),
            ReducerRouteOwnerGroup::MovementResource,
        ),
        "doApproachMovementRejected" => expected_command_movement_route_from_failed_save(
            CommandNextTurnOption::Approach,
            ReducerRouteResolutionOutcome::Invalid(BattleResolutionInvalidReason::InvalidFill),
            vec![ReducerRouteHoleKind::Movement],
            ReducerRouteOwnerGroup::HoleFrontier,
        ),
        "doApproachNoMovementCleanup" => expected_command_without_fill_from_start(
            ReducerRouteResolutionOutcome::Resolved,
            Vec::new(),
            ReducerRouteOwnerGroup::MovementResource,
        ),
        "doFleeFullMovementEndsTurn" => expected_command_follow_route_from_failed_save(
            CommandNextTurnOption::Flee,
            ReducerRouteOwnerGroup::MovementResource,
        ),
        "doFleePartialMovementRejected" => expected_command_movement_route_from_failed_save(
            CommandNextTurnOption::Flee,
            ReducerRouteResolutionOutcome::Invalid(BattleResolutionInvalidReason::InvalidFill),
            vec![ReducerRouteHoleKind::Movement],
            ReducerRouteOwnerGroup::HoleFrontier,
        ),
        "doFleeNoMovementCleanup" => expected_command_without_fill_from_start(
            ReducerRouteResolutionOutcome::Resolved,
            Vec::new(),
            ReducerRouteOwnerGroup::MovementResource,
        ),
        "doFleeOpportunityAttackWindow" => expected_command_movement_route_from_failed_save(
            CommandNextTurnOption::Flee,
            ReducerRouteResolutionOutcome::Resolved,
            Vec::new(),
            ReducerRouteOwnerGroup::InterruptStack,
        ),
        "doFleeOpportunityAttackDeclinedContinuation" => {
            expected_command_follow_route_from_failed_save(
                CommandNextTurnOption::Flee,
                ReducerRouteOwnerGroup::MovementResource,
            )
        }
        "doComplete" => expected_command_without_fill_from_start(
            ReducerRouteResolutionOutcome::NeedsHoles,
            vec![
                ReducerRouteHoleKind::CommandOptionChoice,
                ReducerRouteHoleKind::SpellTargetList,
            ],
            ReducerRouteOwnerGroup::ActiveEffect,
        ),
        action => panic!("unsupported mbt::actionTaken {action}"),
    }
}

fn expected_command_follow_route_from_failed_save(
    option: CommandNextTurnOption,
    owner: ReducerRouteOwnerGroup,
) -> Vec<ReducerRouteEvent> {
    let mut route = expected_command_failed_save_route(option);
    route.push(expected_command_without_fill(
        ReducerRouteResolutionOutcome::Resolved,
        Vec::new(),
        owner,
    ));
    route
}

fn expected_command_movement_route_from_failed_save(
    option: CommandNextTurnOption,
    outcome: ReducerRouteResolutionOutcome,
    holes: Vec<ReducerRouteHoleKind>,
    owner: ReducerRouteOwnerGroup,
) -> Vec<ReducerRouteEvent> {
    let mut route = expected_command_failed_save_route(option);
    route.push(expected_command_fill(
        ReducerRouteFillKind::Movement,
        outcome,
        holes,
        owner,
    ));
    route
}

fn expected_command_failed_save_route(option: CommandNextTurnOption) -> Vec<ReducerRouteEvent> {
    let mut route = expected_command_option_choice_route();
    let (outcome, holes) = match option {
        CommandNextTurnOption::Approach | CommandNextTurnOption::Flee => (
            ReducerRouteResolutionOutcome::NeedsHoles,
            vec![ReducerRouteHoleKind::Movement],
        ),
        CommandNextTurnOption::Drop
        | CommandNextTurnOption::Grovel
        | CommandNextTurnOption::Halt => (ReducerRouteResolutionOutcome::Resolved, Vec::new()),
    };
    route.push(expected_command_fill(
        ReducerRouteFillKind::SavingThrowOutcome,
        outcome,
        holes,
        ReducerRouteOwnerGroup::ActiveEffect,
    ));
    route
}

fn expected_command_option_choice_route() -> Vec<ReducerRouteEvent> {
    let mut route = expected_command_target_list_route();
    route.push(expected_command_fill(
        ReducerRouteFillKind::CommandOptionChoice,
        ReducerRouteResolutionOutcome::NeedsHoles,
        vec![ReducerRouteHoleKind::SavingThrowOutcome],
        ReducerRouteOwnerGroup::HoleFrontier,
    ));
    route
}

fn expected_command_target_list_route() -> Vec<ReducerRouteEvent> {
    let mut route = expected_command_discovery_route();
    route.push(expected_command_fill(
        ReducerRouteFillKind::SpellTargetList,
        ReducerRouteResolutionOutcome::NeedsHoles,
        vec![ReducerRouteHoleKind::CommandOptionChoice],
        ReducerRouteOwnerGroup::HoleFrontier,
    ));
    route
}

fn expected_command_without_fill_from_start(
    outcome: ReducerRouteResolutionOutcome,
    holes: Vec<ReducerRouteHoleKind>,
    owner: ReducerRouteOwnerGroup,
) -> Vec<ReducerRouteEvent> {
    let mut route = expected_command_discovery_route();
    route.push(expected_command_without_fill(outcome, holes, owner));
    route
}

fn expected_command_discovery_route() -> Vec<ReducerRouteEvent> {
    vec![
        route_start_battle(ReducerRouteOwnerGroup::ActionEconomy),
        route_discover_battle_acts_from_route_holes(
            ReducerRouteSubjectFamily::CommandSpell,
            vec![
                ReducerRouteHoleKind::CommandOptionChoice,
                ReducerRouteHoleKind::SpellTargetList,
            ],
            ReducerRouteOwnerGroup::SpellSlotAndActionEconomy,
        ),
    ]
}

fn expected_command_fill(
    fill: ReducerRouteFillKind,
    outcome: ReducerRouteResolutionOutcome,
    holes: Vec<ReducerRouteHoleKind>,
    owner: ReducerRouteOwnerGroup,
) -> ReducerRouteEvent {
    route_resolve_battle_subject_from_route_result(
        ReducerRouteSubjectFamily::CommandSpell,
        fill,
        outcome,
        holes,
        owner,
    )
}

fn expected_command_without_fill(
    outcome: ReducerRouteResolutionOutcome,
    holes: Vec<ReducerRouteHoleKind>,
    owner: ReducerRouteOwnerGroup,
) -> ReducerRouteEvent {
    route_resolve_battle_subject_without_fill_from_route_result(
        ReducerRouteSubjectFamily::CommandSpell,
        outcome,
        holes,
        owner,
    )
}

fn command_route_from_start(
    fill: BattleCommandEffectFill,
    route_fill: ReducerRouteResolveFill,
    owner: ReducerRouteOwnerGroup,
) -> Vec<ReducerRouteEvent> {
    let state = start_standard_battle();
    let mut route = vec![route_start_battle(ReducerRouteOwnerGroup::ActionEconomy)];
    let subject = command_subject(&state);
    let discovery = discover_battle_acts(&state);
    let holes = discovery
        .available_acts()
        .iter()
        .find(|act| act.subject == subject)
        .map(|act| act.holes.clone())
        .expect("command act should be discoverable");
    route.push(route_discover_battle_acts(
        ReducerRouteSubjectFamily::CommandSpell,
        holes,
        ReducerRouteOwnerGroup::SpellSlotAndActionEconomy,
    ));
    let result = resolve_battle_subject(
        state,
        BattleResolutionRequest::command_effect(subject, fill)
            .expect("command subject kind should match command request"),
    );
    route.push(route_resolve_battle_subject_from_result(
        ReducerRouteResolveConnector {
            subject: ReducerRouteSubjectFamily::CommandSpell,
            fill: route_fill,
            owner,
        },
        &result,
    ));
    route
}

fn command_movement_route_from_failed_save(
    option: CommandNextTurnOption,
    movement_available: bool,
    moved_within_five_feet_of_caster: bool,
    opened_opportunity_attack: bool,
    movement_spent_feet: i16,
    owner: ReducerRouteOwnerGroup,
) -> Vec<ReducerRouteEvent> {
    let (state, subject, route) = command_after_failed_save_route(option);
    append_command_route(
        state,
        subject,
        BattleCommandEffectFill::Movement {
            option,
            movement_available,
            moved_within_five_feet_of_caster,
            opened_opportunity_attack,
            movement_spent_feet,
        },
        ReducerRouteResolveFill::Fill(ReducerRouteFillKind::Movement),
        owner,
        route,
    )
    .1
}

fn command_follow_route_from_failed_save(
    option: CommandNextTurnOption,
    fill: BattleCommandEffectFill,
    route_fill: ReducerRouteResolveFill,
    owner: ReducerRouteOwnerGroup,
) -> Vec<ReducerRouteEvent> {
    let (state, subject, route) = command_after_failed_save_route(option);
    append_command_route(state, subject, fill, route_fill, owner, route).1
}

fn command_after_failed_save_route(
    option: CommandNextTurnOption,
) -> (BattleState, BattleSubject, Vec<ReducerRouteEvent>) {
    let state = start_standard_battle();
    let mut route = vec![route_start_battle(ReducerRouteOwnerGroup::ActionEconomy)];
    let subject = command_subject(&state);
    let discovery = discover_battle_acts(&state);
    let holes = discovery
        .available_acts()
        .iter()
        .find(|act| act.subject == subject)
        .map(|act| act.holes.clone())
        .expect("command act should be discoverable");
    route.push(route_discover_battle_acts(
        ReducerRouteSubjectFamily::CommandSpell,
        holes,
        ReducerRouteOwnerGroup::SpellSlotAndActionEconomy,
    ));
    let (target_result, route) = append_command_route(
        state,
        subject,
        BattleCommandEffectFill::SpellTargetList(Actor::Goblin),
        ReducerRouteResolveFill::Fill(ReducerRouteFillKind::SpellTargetList),
        ReducerRouteOwnerGroup::HoleFrontier,
        route,
    );
    let (state, subject) = battle_resolution_continuation(target_result, "command target list");
    let (option_result, route) = append_command_route(
        state,
        subject,
        BattleCommandEffectFill::CommandOptionChoice(option),
        ReducerRouteResolveFill::Fill(ReducerRouteFillKind::CommandOptionChoice),
        ReducerRouteOwnerGroup::HoleFrontier,
        route,
    );
    let (state, subject) = battle_resolution_continuation(option_result, "command option choice");
    let (save_result, route) = append_command_route(
        state,
        subject,
        BattleCommandEffectFill::SavingThrowOutcome {
            option,
            failed: true,
            movement_available: true,
            held_object_facts_required: false,
        },
        ReducerRouteResolveFill::Fill(ReducerRouteFillKind::SavingThrowOutcome),
        ReducerRouteOwnerGroup::ActiveEffect,
        route,
    );
    match save_result.continuing_subject() {
        Some(_) => {
            let (state, subject) =
                battle_resolution_continuation(save_result, "command failed save");
            (state, subject, route)
        }
        None => {
            let state = save_result.into_state();
            let subject = command_subject_from_active_state(&state, "command failed save");
            (state, subject, route)
        }
    }
}

fn append_command_route(
    state: BattleState,
    subject: BattleSubject,
    fill: BattleCommandEffectFill,
    route_fill: ReducerRouteResolveFill,
    owner: ReducerRouteOwnerGroup,
    mut route: Vec<ReducerRouteEvent>,
) -> (
    crate::rules::battle_reducer_spine::BattleResolutionResult,
    Vec<ReducerRouteEvent>,
) {
    let result = resolve_battle_subject(
        state,
        BattleResolutionRequest::command_effect(subject, fill)
            .expect("command subject kind should match command request"),
    );
    route.push(route_resolve_battle_subject_from_result(
        ReducerRouteResolveConnector {
            subject: ReducerRouteSubjectFamily::CommandSpell,
            fill: route_fill,
            owner,
        },
        &result,
    ));
    (result, route)
}

fn command_subject(state: &BattleState) -> BattleSubject {
    discover_battle_acts(state)
        .available_acts()
        .iter()
        .map(|act| act.subject)
        .find(|subject| subject.kind == BattleSubjectKind::CommandSpell)
        .expect("command subject should be discoverable from standard battle")
}

fn command_subject_from_active_state(state: &BattleState, context: &str) -> BattleSubject {
    active_command_battle_subject(state)
        .unwrap_or_else(|| panic!("{context} should leave an active command subject in state"))
}

fn failed_save(option: CommandNextTurnOption) -> CommandNextTurnState {
    record_command_failed_save_pending(command_next_turn_initial_state(), option)
}

fn halt_suppressed() -> CommandNextTurnState {
    suppress_command_halt(failed_save(CommandNextTurnOption::Halt), 30)
}

fn approach_continues() -> CommandNextTurnState {
    continue_command_approach(failed_save(CommandNextTurnOption::Approach), 10)
}

fn approach_rejected() -> CommandNextTurnState {
    reject_command_approach_movement(failed_save(CommandNextTurnOption::Approach))
}

fn flee_rejected() -> CommandNextTurnState {
    reject_command_flee_partial_movement(failed_save(CommandNextTurnOption::Flee))
}

fn flee_opportunity_window() -> CommandNextTurnState {
    open_command_flee_opportunity_attack_window(failed_save(CommandNextTurnOption::Flee))
}

fn flee_opportunity_declined() -> CommandNextTurnState {
    decline_command_flee_opportunity_attack(flee_opportunity_window(), 30)
}

fn witness_from_state(state: CommandNextTurnState) -> CommandOptionNextTurnWitness {
    CommandOptionNextTurnWitness {
        scenario: scenario_ref(state.scenario),
        protocol_result: protocol_result_ref(state.protocol),
        protocol_holes: protocol_holes(state.protocol),
        invalid_reason: invalid_reason_ref(state.protocol),
        target_prone: state.target_prone,
        target_effect_count: state.target_effect_count,
        action_available: state.action_available,
        bonus_action_available: state.bonus_action_available,
        movement_spent_feet: state.movement_spent_feet,
        current_actor: actor_ref(state.current_actor),
        pending_command_option: option_ref(state.pending_option),
        dropped_object_count: state.dropped_object_count,
        reaction_window_open: state.reaction_window_open,
        halt_suppressed: state.halt_suppressed,
    }
}

fn scenario_ref(scenario: CommandNextTurnScenario) -> &'static str {
    match scenario {
        CommandNextTurnScenario::Init => "CommandOptionNextTurnInit",
        CommandNextTurnScenario::FailedSaveRecordsPending => "CommandFailedSaveRecordsPending",
        CommandNextTurnScenario::FollowGrovel => "CommandFollowGrovel",
        CommandNextTurnScenario::FollowDrop => "CommandFollowDrop",
        CommandNextTurnScenario::HaltSuppresses => "CommandHaltSuppresses",
        CommandNextTurnScenario::HaltEndTurnCleanup => "CommandHaltEndTurnCleanup",
        CommandNextTurnScenario::ApproachContinues => "CommandApproachContinues",
        CommandNextTurnScenario::ApproachWithinFiveEndsTurn => "CommandApproachWithinFiveEndsTurn",
        CommandNextTurnScenario::ApproachMovementRejected => "CommandApproachMovementRejected",
        CommandNextTurnScenario::ApproachNoMovementCleanup => "CommandApproachNoMovementCleanup",
        CommandNextTurnScenario::FleeFullMovementEndsTurn => "CommandFleeFullMovementEndsTurn",
        CommandNextTurnScenario::FleePartialMovementRejected => {
            "CommandFleePartialMovementRejected"
        }
        CommandNextTurnScenario::FleeNoMovementCleanup => "CommandFleeNoMovementCleanup",
        CommandNextTurnScenario::FleeOpportunityAttackWindow => {
            "CommandFleeOpportunityAttackWindow"
        }
        CommandNextTurnScenario::FleeOpportunityAttackDeclinedContinuation => {
            "CommandFleeOpportunityAttackDeclinedContinuation"
        }
    }
}

fn protocol_result_ref(protocol: CommandNextTurnProtocol) -> &'static str {
    match protocol {
        CommandNextTurnProtocol::Init => "init",
        CommandNextTurnProtocol::Resolved => "resolved",
        CommandNextTurnProtocol::NeedsInterruptDecision => "needsHoles",
        CommandNextTurnProtocol::Invalid(_) => "invalid",
    }
}

fn protocol_holes(protocol: CommandNextTurnProtocol) -> Vec<&'static str> {
    match protocol {
        CommandNextTurnProtocol::NeedsInterruptDecision => vec!["InterruptDecision"],
        CommandNextTurnProtocol::Init
        | CommandNextTurnProtocol::Resolved
        | CommandNextTurnProtocol::Invalid(_) => vec![],
    }
}

fn invalid_reason_ref(protocol: CommandNextTurnProtocol) -> &'static str {
    match protocol {
        CommandNextTurnProtocol::Invalid(CommandNextTurnInvalidReason::InvalidFill) => {
            "invalidFill"
        }
        CommandNextTurnProtocol::Init
        | CommandNextTurnProtocol::Resolved
        | CommandNextTurnProtocol::NeedsInterruptDecision => "none",
    }
}

fn actor_ref(actor: Option<CommandTurnActor>) -> &'static str {
    match actor {
        Some(CommandTurnActor::Caster) => "Fighter",
        Some(CommandTurnActor::Target) => "Goblin",
        None => "none",
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

fn joined_or_none(values: &[&'static str]) -> String {
    if values.is_empty() {
        "none".to_string()
    } else {
        values.join(",")
    }
}
