use crate::rules::battle_reducer_spine::{
    discover_generic_route_subject_observed, generic_route_subject,
    reaction_casting_time_projection_from_battle, resolve_battle_subject_observed,
    resolve_hellish_rebuke_after_damage_battle, start_battle_observed,
    start_reaction_casting_time_battle, Actor, BattleEntrypointTrace, BattleGenericRouteFill,
    BattleReactionCastingContinuation, BattleReactionCastingOutcome,
    BattleReactionCastingTimeProjection, BattleReactionCastingTrigger, BattleResolutionRequest,
    BattleSetup, BattleSubjectKind,
};

use super::battle_runtime_reducer_route::{
    observed_reducer_route, route_discover_battle_acts_from_route_holes,
    route_resolve_battle_subject_from_route_holes, route_start_battle, ReducerRouteEvent,
    ReducerRouteFillKind, ReducerRouteHoleKind, ReducerRouteOwnerGroup, ReducerRouteSubjectFamily,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ReactionCastingTimeWitness {
    pub trigger_kind: &'static str,
    pub continuation_kind: &'static str,
    pub reactor_hp: i16,
    pub trigger_creature_hp: i16,
    pub reactor_reaction_available: bool,
    pub trigger_creature_first_level_slots_expended: i16,
    pub trigger_creature_fourth_level_slots_expended: i16,
    pub reactor_second_level_slots_expended: i16,
    pub reactor_third_level_slots_expended: i16,
    pub reaction_window_cleared: bool,
    pub scenario_outcome: &'static str,
    pub protocol_result: &'static str,
    pub protocol_holes: Vec<&'static str>,
}

pub const BRANCH_ACTIONS: [&str; 1] = ["doHellishRebukeAfterDamage"];

pub const TARGET_BLOCKED_BRANCH_ACTIONS: [&str; 2] = [
    "doCounterspellAllowsSpellCastResume",
    "doCounterspellEndsSpellCast",
];

pub fn replay_observed_action(observed_action_taken: &str) -> ReactionCastingTimeWitness {
    match observed_action_taken {
        "doHellishRebukeAfterDamage" => {
            witness_from_projection(reaction_casting_time_projection_from_battle(
                &resolve_hellish_rebuke_after_damage_battle(start_reaction_casting_time_battle()),
            ))
        }
        action => panic!("unsupported mbt::actionTaken {action}"),
    }
}

pub fn expected_witness(observed_action_taken: &str) -> ReactionCastingTimeWitness {
    match observed_action_taken {
        "doHellishRebukeAfterDamage" => ReactionCastingTimeWitness {
            trigger_kind: "afterDamage",
            continuation_kind: "afterDamageResolved",
            reactor_hp: 29,
            trigger_creature_hp: 27,
            reactor_reaction_available: false,
            trigger_creature_first_level_slots_expended: 0,
            trigger_creature_fourth_level_slots_expended: 0,
            reactor_second_level_slots_expended: 1,
            reactor_third_level_slots_expended: 0,
            reaction_window_cleared: true,
            scenario_outcome: "HellishRebukeAfterDamage",
            protocol_result: "resolved",
            protocol_holes: Vec::new(),
        },
        action => panic!("unsupported mbt::actionTaken {action}"),
    }
}

pub fn replay_observed_route(observed_action_taken: &str) -> Vec<ReducerRouteEvent> {
    match observed_action_taken {
        "doHellishRebukeAfterDamage" => replay_generic_route(
            BattleSubjectKind::ReactionSpellInterruptDecision,
            &[
                (
                    BattleSubjectKind::ReactionSpellInterruptDecision,
                    BattleGenericRouteFill::InterruptDecision,
                ),
                (
                    BattleSubjectKind::ReactionSpellSlotCommit,
                    BattleGenericRouteFill::InterruptDecision,
                ),
                (
                    BattleSubjectKind::ReactionSpellDamage,
                    BattleGenericRouteFill::InterruptDecision,
                ),
            ],
            &[ReducerRouteSubjectFamily::ReactionSpell],
        ),
        action => panic!("unsupported route mbt::actionTaken {action}"),
    }
}

pub fn expected_route(observed_action_taken: &str) -> Vec<ReducerRouteEvent> {
    match observed_action_taken {
        "doHellishRebukeAfterDamage" => hellish_rebuke_after_damage_route(),
        action => panic!("unsupported route mbt::actionTaken {action}"),
    }
}

pub fn projection_payload(witness: &ReactionCastingTimeWitness) -> String {
    [
        format!("qTriggerKind={}", witness.trigger_kind),
        format!("qContinuationKind={}", witness.continuation_kind),
        format!("qReactorHp={}", witness.reactor_hp),
        format!("qTriggerCreatureHp={}", witness.trigger_creature_hp),
        format!(
            "qReactorReactionAvailable={}",
            witness.reactor_reaction_available
        ),
        format!(
            "qTriggerCreatureFirstLevelSlotsExpended={}",
            witness.trigger_creature_first_level_slots_expended
        ),
        format!(
            "qTriggerCreatureFourthLevelSlotsExpended={}",
            witness.trigger_creature_fourth_level_slots_expended
        ),
        format!(
            "qReactorSecondLevelSlotsExpended={}",
            witness.reactor_second_level_slots_expended
        ),
        format!(
            "qReactorThirdLevelSlotsExpended={}",
            witness.reactor_third_level_slots_expended
        ),
        format!("qReactionWindowCleared={}", witness.reaction_window_cleared),
        format!("qScenarioOutcome={}", witness.scenario_outcome),
        format!("protocolResult={}", witness.protocol_result),
        format!("protocolHoles={}", joined_or_none(&witness.protocol_holes)),
    ]
    .join("\n")
}

fn replay_generic_route(
    discovery_subject: BattleSubjectKind,
    steps: &[(BattleSubjectKind, BattleGenericRouteFill)],
    route_subjects: &[ReducerRouteSubjectFamily],
) -> Vec<ReducerRouteEvent> {
    let mut trace = BattleEntrypointTrace::default();
    let state = start_battle_observed(BattleSetup::standard(), &mut trace).state;
    let (mut state, _) =
        discover_generic_route_subject_observed(state, discovery_subject, &mut trace);
    for (subject_kind, fill) in steps {
        let request = BattleResolutionRequest::generic_route(
            generic_route_subject(*subject_kind, Actor::Fighter),
            *fill,
        )
        .expect("generic route subject should accept generic route fills");
        state = resolve_battle_subject_observed(state, request, &mut trace).into_state();
    }
    observed_reducer_route(&trace, route_subjects)
}

fn pending_reaction_decision_route() -> Vec<ReducerRouteEvent> {
    vec![
        route_start_battle(ReducerRouteOwnerGroup::ActionEconomy),
        route_discover_battle_acts_from_route_holes(
            ReducerRouteSubjectFamily::ReactionSpell,
            vec![ReducerRouteHoleKind::InterruptDecision],
            ReducerRouteOwnerGroup::InterruptStack,
        ),
    ]
}

fn hellish_rebuke_after_damage_route() -> Vec<ReducerRouteEvent> {
    let mut route = pending_reaction_decision_route();
    route.push(route_resolve_battle_subject_from_route_holes(
        ReducerRouteSubjectFamily::ReactionSpell,
        ReducerRouteFillKind::InterruptDecision,
        Vec::new(),
        ReducerRouteOwnerGroup::InterruptStack,
    ));
    route.push(route_resolve_battle_subject_from_route_holes(
        ReducerRouteSubjectFamily::ReactionSpell,
        ReducerRouteFillKind::InterruptDecision,
        Vec::new(),
        ReducerRouteOwnerGroup::SpellSlotAndActionEconomy,
    ));
    route.push(route_resolve_battle_subject_from_route_holes(
        ReducerRouteSubjectFamily::ReactionSpell,
        ReducerRouteFillKind::InterruptDecision,
        Vec::new(),
        ReducerRouteOwnerGroup::HitPoint,
    ));
    route
}

fn witness_from_projection(
    projection: BattleReactionCastingTimeProjection,
) -> ReactionCastingTimeWitness {
    ReactionCastingTimeWitness {
        trigger_kind: trigger_ref(projection.trigger),
        continuation_kind: continuation_ref(projection.continuation),
        reactor_hp: projection.reactor_hp,
        trigger_creature_hp: projection.trigger_creature_hp,
        reactor_reaction_available: projection.reactor_reaction_available,
        trigger_creature_first_level_slots_expended: projection
            .trigger_creature_first_level_slots_expended,
        trigger_creature_fourth_level_slots_expended: projection
            .trigger_creature_fourth_level_slots_expended,
        reactor_second_level_slots_expended: projection.reactor_second_level_slots_expended,
        reactor_third_level_slots_expended: projection.reactor_third_level_slots_expended,
        reaction_window_cleared: projection.reaction_window_cleared,
        scenario_outcome: outcome_ref(projection.outcome),
        protocol_result: protocol_result_ref(projection.outcome),
        protocol_holes: Vec::new(),
    }
}

fn trigger_ref(trigger: BattleReactionCastingTrigger) -> &'static str {
    match trigger {
        BattleReactionCastingTrigger::None => "none",
        BattleReactionCastingTrigger::AfterDamage => "afterDamage",
    }
}

fn continuation_ref(continuation: BattleReactionCastingContinuation) -> &'static str {
    match continuation {
        BattleReactionCastingContinuation::None => "none",
        BattleReactionCastingContinuation::AfterDamageResolved => "afterDamageResolved",
    }
}

fn outcome_ref(outcome: BattleReactionCastingOutcome) -> &'static str {
    match outcome {
        BattleReactionCastingOutcome::Init => "Init",
        BattleReactionCastingOutcome::AfterDamageReactionResolved => "HellishRebukeAfterDamage",
    }
}

fn protocol_result_ref(outcome: BattleReactionCastingOutcome) -> &'static str {
    match outcome {
        BattleReactionCastingOutcome::Init => "init",
        BattleReactionCastingOutcome::AfterDamageReactionResolved => "resolved",
    }
}

fn joined_or_none(values: &[&'static str]) -> String {
    if values.is_empty() {
        "none".to_string()
    } else {
        values.join(",")
    }
}
