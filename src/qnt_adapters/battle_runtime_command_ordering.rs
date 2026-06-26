use crate::rules::battle_reducer_spine::{
    discover_battle_acts, resolve_battle_subject, start_command_effect_battle_at_actor,
    start_standard_battle, Actor, BattleCommandEffectFill, BattleCommandEffectProcedure,
    BattleCommandEffectSubject, BattleResolutionRequest, BattleState, BattleSubject,
    BattleSubjectKind,
};
use crate::rules::command_options::{
    command_fill_order_accepted_stage, command_fill_order_error, command_fill_order_result,
    command_fill_order_runtime_result, command_hole_frontier, command_ordering_projection,
    CommandFillKind, CommandFillOrderingError, CommandFrontierStage, CommandHoleKind,
    CommandNextTurnOption, CommandNextTurnScenario, CommandOrderingFillFacts,
    CommandOrderingProjectionFacts, CommandOrderingProtocol, CommandOrderingRuntimeResult,
    CommandOrderingState, CommandTurnActor,
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

pub fn replay_observed_route(observed_action_taken: &str) -> Vec<ReducerRouteEvent> {
    command_ordering_route_from_obligation(observed_action_taken)
}

fn command_ordering_route_from_obligation(observed_action_taken: &str) -> Vec<ReducerRouteEvent> {
    match observed_action_taken {
        "doDiscoverCommand" => command_discovery_route().2,
        "doSubmitOptionBeforeTargetList" => command_route_from_start(
            BattleCommandEffectFill::CommandOptionChoice(CommandNextTurnOption::Grovel),
            ReducerRouteResolveFill::Fill(ReducerRouteFillKind::CommandOptionChoice),
            ReducerRouteOwnerGroup::HoleFrontier,
        ),
        "doFillTargetList" => command_route_from_start(
            BattleCommandEffectFill::SpellTargetList(
                crate::rules::battle_reducer_spine::Actor::Goblin,
            ),
            ReducerRouteResolveFill::Fill(ReducerRouteFillKind::SpellTargetList),
            ReducerRouteOwnerGroup::HoleFrontier,
        ),
        "doSubmitSavingThrowBeforeOption" => {
            let (state, subject, route) = command_after_target_list_route();
            append_command_route(
                state,
                subject,
                BattleCommandEffectFill::SavingThrowOutcome {
                    option: CommandNextTurnOption::Grovel,
                    failed: true,
                    movement_available: false,
                    held_object_facts_required: false,
                },
                ReducerRouteResolveFill::Fill(ReducerRouteFillKind::SavingThrowOutcome),
                ReducerRouteOwnerGroup::HoleFrontier,
                route,
            )
            .1
        }
        "doFillGrovelOption" => {
            let (state, subject, route) = command_after_target_list_route();
            append_command_route(
                state,
                subject,
                BattleCommandEffectFill::CommandOptionChoice(CommandNextTurnOption::Grovel),
                ReducerRouteResolveFill::Fill(ReducerRouteFillKind::CommandOptionChoice),
                ReducerRouteOwnerGroup::HoleFrontier,
                route,
            )
            .1
        }
        "doFillFailedGrovelSavingThrow" => {
            let (state, subject, route) = command_after_grovel_option_route();
            append_command_route(
                state,
                subject,
                BattleCommandEffectFill::SavingThrowOutcome {
                    option: CommandNextTurnOption::Grovel,
                    failed: true,
                    movement_available: false,
                    held_object_facts_required: false,
                },
                ReducerRouteResolveFill::Fill(ReducerRouteFillKind::SavingThrowOutcome),
                ReducerRouteOwnerGroup::ActiveEffect,
                route,
            )
            .1
        }
        "doFollowGrovel" => {
            let (state, subject, route) = command_after_failed_grovel_save_route();
            append_command_route(
                state,
                subject,
                BattleCommandEffectFill::FollowPendingOption(
                    crate::rules::battle_reducer_spine::CommandPendingOptionFollow::Grovel,
                ),
                ReducerRouteResolveFill::WithoutFill,
                ReducerRouteOwnerGroup::ConditionLifecycle,
                route,
            )
            .1
        }
        "doDropNeedsHeldObjectFacts" => {
            let (state, subject, route) = command_after_target_list_route();
            append_command_route(
                state,
                subject,
                BattleCommandEffectFill::SavingThrowOutcome {
                    option: CommandNextTurnOption::Drop,
                    failed: true,
                    movement_available: false,
                    held_object_facts_required: true,
                },
                ReducerRouteResolveFill::Fill(ReducerRouteFillKind::SavingThrowOutcome),
                ReducerRouteOwnerGroup::ActiveEffect,
                route,
            )
            .1
        }
        "doFillDropHeldObjectFacts" => command_route_from_start(
            BattleCommandEffectFill::DropHeldObjectFacts {
                dropped_object_count: 2,
            },
            ReducerRouteResolveFill::WithoutFill,
            ReducerRouteOwnerGroup::ActiveEffect,
        ),
        "doHaltSuppresses" => command_route_from_start(
            BattleCommandEffectFill::FollowPendingOption(
                crate::rules::battle_reducer_spine::CommandPendingOptionFollow::Halt {
                    movement_spent_feet: 30,
                },
            ),
            ReducerRouteResolveFill::WithoutFill,
            ReducerRouteOwnerGroup::ActiveEffect,
        ),
        "doApproachMovementContinues" => command_active_discovery_route(
            CommandFrontierStage::ApproachMovement,
            Some(CommandNextTurnOption::Approach),
            ReducerRouteOwnerGroup::ActiveEffect,
        ),
        "doFillApproachMovementContinues" | "doFillApproachMovementWithinFive" => {
            command_route_from_start(
                BattleCommandEffectFill::Movement {
                    option: CommandNextTurnOption::Approach,
                    movement_available: true,
                    moved_within_five_feet_of_caster: observed_action_taken
                        == "doFillApproachMovementWithinFive",
                    opened_opportunity_attack: false,
                    movement_spent_feet: 10,
                },
                ReducerRouteResolveFill::Fill(ReducerRouteFillKind::Movement),
                ReducerRouteOwnerGroup::MovementResource,
            )
        }
        "doApproachNoMovement" => command_route_from_start(
            BattleCommandEffectFill::CleanupPendingOption(
                crate::rules::battle_reducer_spine::CommandPendingOptionCleanup::ApproachNoMovement,
            ),
            ReducerRouteResolveFill::WithoutFill,
            ReducerRouteOwnerGroup::MovementResource,
        ),
        "doFleeMovement" => command_route_from_start(
            BattleCommandEffectFill::SavingThrowOutcome {
                option: CommandNextTurnOption::Flee,
                failed: true,
                movement_available: true,
                held_object_facts_required: false,
            },
            ReducerRouteResolveFill::Fill(ReducerRouteFillKind::SavingThrowOutcome),
            ReducerRouteOwnerGroup::ActiveEffect,
        ),
        "doFillFleeMovement" => command_route_from_start(
            BattleCommandEffectFill::Movement {
                option: CommandNextTurnOption::Flee,
                movement_available: true,
                moved_within_five_feet_of_caster: false,
                opened_opportunity_attack: false,
                movement_spent_feet: 30,
            },
            ReducerRouteResolveFill::Fill(ReducerRouteFillKind::Movement),
            ReducerRouteOwnerGroup::MovementResource,
        ),
        "doRejectFleePartialMovement" => command_route_from_start(
            BattleCommandEffectFill::Movement {
                option: CommandNextTurnOption::Flee,
                movement_available: false,
                moved_within_five_feet_of_caster: false,
                opened_opportunity_attack: false,
                movement_spent_feet: 0,
            },
            ReducerRouteResolveFill::Fill(ReducerRouteFillKind::Movement),
            ReducerRouteOwnerGroup::HoleFrontier,
        ),
        "doFleeNoMovement" => command_route_from_start(
            BattleCommandEffectFill::CleanupPendingOption(
                crate::rules::battle_reducer_spine::CommandPendingOptionCleanup::FleeNoMovement,
            ),
            ReducerRouteResolveFill::WithoutFill,
            ReducerRouteOwnerGroup::MovementResource,
        ),
        "doFleeOpportunityAttack" => command_route_from_start(
            BattleCommandEffectFill::Movement {
                option: CommandNextTurnOption::Flee,
                movement_available: true,
                moved_within_five_feet_of_caster: false,
                opened_opportunity_attack: true,
                movement_spent_feet: 0,
            },
            ReducerRouteResolveFill::Fill(ReducerRouteFillKind::Movement),
            ReducerRouteOwnerGroup::InterruptStack,
        ),
        action => panic!("unsupported mbt::actionTaken {action}"),
    }
}

pub fn expected_route(observed_action_taken: &str) -> Vec<ReducerRouteEvent> {
    expected_command_ordering_route(observed_action_taken)
}

fn expected_command_ordering_route(observed_action_taken: &str) -> Vec<ReducerRouteEvent> {
    match observed_action_taken {
        "doDiscoverCommand" => expected_command_discovery_route(),
        "doSubmitOptionBeforeTargetList" => {
            let mut route = expected_command_discovery_route();
            route.push(expected_command_fill(
                ReducerRouteFillKind::CommandOptionChoice,
                ReducerRouteResolutionOutcome::NeedsHoles,
                vec![ReducerRouteHoleKind::SpellTargetList],
                ReducerRouteOwnerGroup::HoleFrontier,
            ));
            route
        }
        "doFillTargetList" => expected_command_target_list_route(),
        "doSubmitSavingThrowBeforeOption" => {
            let mut route = expected_command_target_list_route();
            route.push(expected_command_fill(
                ReducerRouteFillKind::SavingThrowOutcome,
                ReducerRouteResolutionOutcome::NeedsHoles,
                vec![ReducerRouteHoleKind::CommandOptionChoice],
                ReducerRouteOwnerGroup::HoleFrontier,
            ));
            route
        }
        "doFillGrovelOption" => expected_command_grovel_option_route(),
        "doFillFailedGrovelSavingThrow" => expected_command_failed_grovel_save_route(),
        "doFollowGrovel" => {
            let mut route = expected_command_failed_grovel_save_route();
            route.push(expected_command_without_fill(
                ReducerRouteResolutionOutcome::Resolved,
                Vec::new(),
                ReducerRouteOwnerGroup::ConditionLifecycle,
            ));
            route
        }
        "doDropNeedsHeldObjectFacts" => {
            let mut route = expected_command_target_list_route();
            route.push(expected_command_fill(
                ReducerRouteFillKind::SavingThrowOutcome,
                ReducerRouteResolutionOutcome::NeedsHoles,
                vec![ReducerRouteHoleKind::CommandOptionChoice],
                ReducerRouteOwnerGroup::ActiveEffect,
            ));
            route
        }
        "doFillDropHeldObjectFacts" => {
            let mut route = expected_command_discovery_route();
            route.push(expected_command_without_fill(
                ReducerRouteResolutionOutcome::Resolved,
                Vec::new(),
                ReducerRouteOwnerGroup::ActiveEffect,
            ));
            route
        }
        "doHaltSuppresses" => {
            let mut route = expected_command_discovery_route();
            route.push(expected_command_without_fill(
                ReducerRouteResolutionOutcome::Invalid(
                    crate::rules::battle_reducer_spine::BattleResolutionInvalidReason::InvalidFill,
                ),
                vec![
                    ReducerRouteHoleKind::CommandOptionChoice,
                    ReducerRouteHoleKind::SpellTargetList,
                ],
                ReducerRouteOwnerGroup::ActiveEffect,
            ));
            route
        }
        "doApproachMovementContinues" => expected_command_active_discovery_route(
            CommandFrontierStage::ApproachMovement,
            ReducerRouteOwnerGroup::ActiveEffect,
        ),
        "doFillApproachMovementContinues" | "doFillApproachMovementWithinFive" => {
            let mut route = expected_command_discovery_route();
            route.push(expected_command_fill(
                ReducerRouteFillKind::Movement,
                ReducerRouteResolutionOutcome::Invalid(
                    crate::rules::battle_reducer_spine::BattleResolutionInvalidReason::InvalidFill,
                ),
                vec![
                    ReducerRouteHoleKind::CommandOptionChoice,
                    ReducerRouteHoleKind::SpellTargetList,
                ],
                ReducerRouteOwnerGroup::MovementResource,
            ));
            route
        }
        "doApproachNoMovement" => {
            let mut route = expected_command_discovery_route();
            route.push(expected_command_without_fill(
                ReducerRouteResolutionOutcome::Resolved,
                Vec::new(),
                ReducerRouteOwnerGroup::MovementResource,
            ));
            route
        }
        "doFleeMovement" => {
            let mut route = expected_command_discovery_route();
            route.push(expected_command_fill(
                ReducerRouteFillKind::SavingThrowOutcome,
                ReducerRouteResolutionOutcome::NeedsHoles,
                vec![ReducerRouteHoleKind::SpellTargetList],
                ReducerRouteOwnerGroup::ActiveEffect,
            ));
            route
        }
        "doFillFleeMovement" => {
            let mut route = expected_command_discovery_route();
            route.push(expected_command_fill(
                ReducerRouteFillKind::Movement,
                ReducerRouteResolutionOutcome::Invalid(
                    crate::rules::battle_reducer_spine::BattleResolutionInvalidReason::InvalidFill,
                ),
                vec![
                    ReducerRouteHoleKind::CommandOptionChoice,
                    ReducerRouteHoleKind::SpellTargetList,
                ],
                ReducerRouteOwnerGroup::MovementResource,
            ));
            route
        }
        "doRejectFleePartialMovement" => {
            let mut route = expected_command_discovery_route();
            route.push(expected_command_fill(
                ReducerRouteFillKind::Movement,
                ReducerRouteResolutionOutcome::Invalid(
                    crate::rules::battle_reducer_spine::BattleResolutionInvalidReason::InvalidFill,
                ),
                vec![
                    ReducerRouteHoleKind::CommandOptionChoice,
                    ReducerRouteHoleKind::SpellTargetList,
                ],
                ReducerRouteOwnerGroup::HoleFrontier,
            ));
            route
        }
        "doFleeNoMovement" => {
            let mut route = expected_command_discovery_route();
            route.push(expected_command_without_fill(
                ReducerRouteResolutionOutcome::Resolved,
                Vec::new(),
                ReducerRouteOwnerGroup::MovementResource,
            ));
            route
        }
        "doFleeOpportunityAttack" => {
            let mut route = expected_command_discovery_route();
            route.push(expected_command_fill(
                ReducerRouteFillKind::Movement,
                ReducerRouteResolutionOutcome::Invalid(
                    crate::rules::battle_reducer_spine::BattleResolutionInvalidReason::InvalidFill,
                ),
                vec![
                    ReducerRouteHoleKind::CommandOptionChoice,
                    ReducerRouteHoleKind::SpellTargetList,
                ],
                ReducerRouteOwnerGroup::InterruptStack,
            ));
            route
        }
        action => panic!("unsupported mbt::actionTaken {action}"),
    }
}

fn command_route_from_start(
    fill: BattleCommandEffectFill,
    route_fill: ReducerRouteResolveFill,
    owner: ReducerRouteOwnerGroup,
) -> Vec<ReducerRouteEvent> {
    let (state, subject, route) = command_discovery_route();
    append_command_route(state, subject, fill, route_fill, owner, route).1
}

fn command_discovery_route() -> (BattleState, BattleSubject, Vec<ReducerRouteEvent>) {
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
    (state, subject, route)
}

fn command_after_target_list_route() -> (BattleState, BattleSubject, Vec<ReducerRouteEvent>) {
    let (state, subject, route) = command_discovery_route();
    let (result, route) = append_command_route(
        state,
        subject,
        BattleCommandEffectFill::SpellTargetList(crate::rules::battle_reducer_spine::Actor::Goblin),
        ReducerRouteResolveFill::Fill(ReducerRouteFillKind::SpellTargetList),
        ReducerRouteOwnerGroup::HoleFrontier,
        route,
    );
    let (state, subject) = battle_resolution_continuation(result, "command target list");
    (state, subject, route)
}

fn command_after_grovel_option_route() -> (BattleState, BattleSubject, Vec<ReducerRouteEvent>) {
    let (state, subject, route) = command_after_target_list_route();
    let (result, route) = append_command_route(
        state,
        subject,
        BattleCommandEffectFill::CommandOptionChoice(CommandNextTurnOption::Grovel),
        ReducerRouteResolveFill::Fill(ReducerRouteFillKind::CommandOptionChoice),
        ReducerRouteOwnerGroup::HoleFrontier,
        route,
    );
    let (state, subject) = battle_resolution_continuation(result, "command option choice");
    (state, subject, route)
}

fn command_after_failed_grovel_save_route() -> (BattleState, BattleSubject, Vec<ReducerRouteEvent>)
{
    let (state, subject, route) = command_after_grovel_option_route();
    let (result, route) = append_command_route(
        state,
        subject,
        BattleCommandEffectFill::SavingThrowOutcome {
            option: CommandNextTurnOption::Grovel,
            failed: true,
            movement_available: false,
            held_object_facts_required: false,
        },
        ReducerRouteResolveFill::Fill(ReducerRouteFillKind::SavingThrowOutcome),
        ReducerRouteOwnerGroup::ActiveEffect,
        route,
    );
    let state = result.into_state();
    let subject = command_subject_from_active_state(&state, "command failed grovel save");
    (state, subject, route)
}

fn command_active_discovery_route(
    stage: CommandFrontierStage,
    pending_option: Option<CommandNextTurnOption>,
    owner: ReducerRouteOwnerGroup,
) -> Vec<ReducerRouteEvent> {
    let state = command_active_state(stage, pending_option);
    let subject = command_subject(&state);
    let holes = discover_battle_acts(&state)
        .available_acts()
        .iter()
        .find(|act| act.subject == subject)
        .map(|act| act.holes.clone())
        .expect("active command subject should be discoverable");
    vec![
        route_start_battle(ReducerRouteOwnerGroup::ActionEconomy),
        route_discover_battle_acts(ReducerRouteSubjectFamily::CommandSpell, holes, owner),
    ]
}

fn command_active_state(
    stage: CommandFrontierStage,
    pending_option: Option<CommandNextTurnOption>,
) -> BattleState {
    let mut state = start_command_effect_battle_at_actor(CommandTurnActor::Target);
    state.command_effect_procedure =
        BattleCommandEffectProcedure::Active(BattleCommandEffectSubject {
            actor: Actor::Goblin,
            target: Some(Actor::Fighter),
            stage,
            pending_option,
            last_ordering_error: None,
            scenario: CommandNextTurnScenario::FailedSaveRecordsPending,
            dropped_object_count: 0,
            halt_suppressed: false,
        });
    state
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

fn expected_command_grovel_option_route() -> Vec<ReducerRouteEvent> {
    let mut route = expected_command_target_list_route();
    route.push(expected_command_fill(
        ReducerRouteFillKind::CommandOptionChoice,
        ReducerRouteResolutionOutcome::NeedsHoles,
        vec![ReducerRouteHoleKind::SavingThrowOutcome],
        ReducerRouteOwnerGroup::HoleFrontier,
    ));
    route
}

fn expected_command_failed_grovel_save_route() -> Vec<ReducerRouteEvent> {
    let mut route = expected_command_grovel_option_route();
    route.push(expected_command_fill(
        ReducerRouteFillKind::SavingThrowOutcome,
        ReducerRouteResolutionOutcome::Resolved,
        Vec::new(),
        ReducerRouteOwnerGroup::ActiveEffect,
    ));
    route
}

fn expected_command_active_discovery_route(
    stage: CommandFrontierStage,
    owner: ReducerRouteOwnerGroup,
) -> Vec<ReducerRouteEvent> {
    vec![
        route_start_battle(ReducerRouteOwnerGroup::ActionEconomy),
        route_discover_battle_acts_from_route_holes(
            ReducerRouteSubjectFamily::CommandSpell,
            command_stage_route_holes(stage),
            owner,
        ),
    ]
}

fn command_stage_route_holes(stage: CommandFrontierStage) -> Vec<ReducerRouteHoleKind> {
    command_hole_frontier(stage)
        .into_iter()
        .map(command_route_hole)
        .collect()
}

fn command_route_hole(hole: CommandHoleKind) -> ReducerRouteHoleKind {
    match hole {
        CommandHoleKind::TargetChoice => ReducerRouteHoleKind::TargetChoice,
        CommandHoleKind::SpellTargetList => ReducerRouteHoleKind::SpellTargetList,
        CommandHoleKind::CommandOptionChoice => ReducerRouteHoleKind::CommandOptionChoice,
        CommandHoleKind::SavingThrowOutcome => ReducerRouteHoleKind::SavingThrowOutcome,
        CommandHoleKind::Movement => ReducerRouteHoleKind::Movement,
    }
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
    crate::rules::battle_reducer_spine::active_command_battle_subject(state)
        .unwrap_or_else(|| panic!("{context} should leave an active command subject in state"))
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
        current_actor: Some(current_actor),
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

fn actor_ref(actor: Option<CommandTurnActor>) -> &'static str {
    match actor {
        Some(CommandTurnActor::Caster) => "Caster",
        Some(CommandTurnActor::Target) => "Target",
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
