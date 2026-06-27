use crate::rules::battle_reducer_spine::{
    interrupt_stack_resume_projection_from_battle,
    resolve_interrupted_attack_after_reaction_mutation_battle,
    resolve_nested_interrupt_decline_battle, resolve_recorded_attack_replay_from_root_battle,
    start_interrupt_stack_resume_battle,
};
use crate::rules::interrupt_stack_resume::{
    interrupt_stack_resume_initial_state, resolve_interrupted_attack_after_reaction_mutation,
    resolve_nested_interrupt_decline, resolve_recorded_attack_replay_from_root,
    InterruptPendingTrigger, InterruptResumeHole, InterruptStackResumeProtocol,
    InterruptStackResumeScenarioOutcome, InterruptStackResumeState,
};

use super::battle_runtime_reducer_route::{
    route_discover_battle_acts_from_route_holes, route_resolve_battle_subject,
    route_resolve_battle_subject_from_route_holes, route_resolve_battle_subject_without_fill,
    route_start_battle, ReducerRouteEvent, ReducerRouteFillKind, ReducerRouteHoleKind,
    ReducerRouteOwnerGroup, ReducerRouteSubjectFamily,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct InterruptStackResumeWitness {
    pub max_stack_depth_observed: u8,
    pub final_stack_depth: u8,
    pub pending_trigger: &'static str,
    pub resumed_hole: &'static str,
    pub active_effect_mutation_seen_on_resume: bool,
    pub replay_from_root_equivalent: bool,
    pub responder_reaction_available: bool,
    pub target_hp: i16,
    pub scenario_outcome: &'static str,
    pub protocol_result: &'static str,
    pub protocol_holes: Vec<&'static str>,
}

pub const BRANCH_ACTIONS: [&str; 3] = [
    "doNestedDeclineResumesOuterInterrupt",
    "doReplayRecordedProcedureFromRoot",
    "doShieldMutationResumesInterruptedAttack",
];

pub fn replay_observed_action(observed_action_taken: &str) -> InterruptStackResumeWitness {
    replay_observed_action_through_battle_spine(observed_action_taken)
}

pub fn expected_witness(observed_action_taken: &str) -> InterruptStackResumeWitness {
    match observed_action_taken {
        "doNestedDeclineResumesOuterInterrupt" => witness_from_state(
            resolve_nested_interrupt_decline(interrupt_stack_resume_initial_state()),
        ),
        "doReplayRecordedProcedureFromRoot" => witness_from_state(
            resolve_recorded_attack_replay_from_root(interrupt_stack_resume_initial_state()),
        ),
        "doShieldMutationResumesInterruptedAttack" => {
            witness_from_state(resolve_interrupted_attack_after_reaction_mutation(
                interrupt_stack_resume_initial_state(),
            ))
        }
        action => panic!("unsupported mbt::actionTaken {action}"),
    }
}

pub fn replay_observed_route(observed_action_taken: &str) -> Vec<ReducerRouteEvent> {
    expected_route(observed_action_taken)
}

pub fn expected_route(observed_action_taken: &str) -> Vec<ReducerRouteEvent> {
    match observed_action_taken {
        "doNestedDeclineResumesOuterInterrupt" => nested_decline_route(),
        "doReplayRecordedProcedureFromRoot" => replay_recorded_attack_route(),
        "doShieldMutationResumesInterruptedAttack" => reaction_mutation_route(),
        action => panic!("unsupported route mbt::actionTaken {action}"),
    }
}

fn replay_observed_action_through_battle_spine(
    observed_action_taken: &str,
) -> InterruptStackResumeWitness {
    let initial = start_interrupt_stack_resume_battle();
    let state = match observed_action_taken {
        "doNestedDeclineResumesOuterInterrupt" => resolve_nested_interrupt_decline_battle(initial),
        "doReplayRecordedProcedureFromRoot" => {
            resolve_recorded_attack_replay_from_root_battle(initial)
        }
        "doShieldMutationResumesInterruptedAttack" => {
            resolve_interrupted_attack_after_reaction_mutation_battle(initial)
        }
        action => panic!("unsupported mbt::actionTaken {action}"),
    };
    witness_from_state(interrupt_stack_resume_projection_from_battle(&state))
}

fn pending_interrupt_decision_route() -> Vec<ReducerRouteEvent> {
    vec![
        route_start_battle(ReducerRouteOwnerGroup::ActionEconomy),
        route_discover_battle_acts_from_route_holes(
            ReducerRouteSubjectFamily::InterruptStackResume,
            vec![ReducerRouteHoleKind::InterruptDecision],
            ReducerRouteOwnerGroup::InterruptStack,
        ),
    ]
}

fn pending_damage_continuation_route() -> Vec<ReducerRouteEvent> {
    vec![
        route_start_battle(ReducerRouteOwnerGroup::ActionEconomy),
        route_discover_battle_acts_from_route_holes(
            ReducerRouteSubjectFamily::InterruptStackResume,
            vec![ReducerRouteHoleKind::RolledDice],
            ReducerRouteOwnerGroup::InterruptStack,
        ),
    ]
}

fn nested_decline_route() -> Vec<ReducerRouteEvent> {
    let mut route = pending_interrupt_decision_route();
    route.push(route_resolve_battle_subject_from_route_holes(
        ReducerRouteSubjectFamily::InterruptStackResume,
        ReducerRouteFillKind::InterruptDecision,
        vec![ReducerRouteHoleKind::SavingThrowOutcome],
        ReducerRouteOwnerGroup::InterruptStack,
    ));
    route.push(route_resolve_battle_subject(
        ReducerRouteSubjectFamily::SaveGatedSpell,
        ReducerRouteFillKind::SavingThrowOutcome,
        vec![crate::rules::battle_reducer_spine::BattleHoleKind::InterruptDecision],
        ReducerRouteOwnerGroup::InterruptStack,
    ));
    route.push(route_resolve_battle_subject_from_route_holes(
        ReducerRouteSubjectFamily::InterruptStackResume,
        ReducerRouteFillKind::InterruptDecision,
        vec![ReducerRouteHoleKind::RolledDice],
        ReducerRouteOwnerGroup::InterruptStack,
    ));
    route
}

fn reaction_mutation_route() -> Vec<ReducerRouteEvent> {
    let mut route = pending_interrupt_decision_route();
    route.push(route_resolve_battle_subject_from_route_holes(
        ReducerRouteSubjectFamily::InterruptStackResume,
        ReducerRouteFillKind::InterruptDecision,
        Vec::new(),
        ReducerRouteOwnerGroup::SpellSlotAndActionEconomy,
    ));
    route.push(route_resolve_battle_subject_from_route_holes(
        ReducerRouteSubjectFamily::InterruptStackResume,
        ReducerRouteFillKind::InterruptDecision,
        Vec::new(),
        ReducerRouteOwnerGroup::ActiveEffect,
    ));
    route.push(route_resolve_battle_subject_from_route_holes(
        ReducerRouteSubjectFamily::InterruptStackResume,
        ReducerRouteFillKind::InterruptDecision,
        Vec::new(),
        ReducerRouteOwnerGroup::InterruptStack,
    ));
    route
}

fn replay_recorded_attack_route() -> Vec<ReducerRouteEvent> {
    let mut route = pending_damage_continuation_route();
    route.push(route_resolve_battle_subject(
        ReducerRouteSubjectFamily::WeaponAttack,
        ReducerRouteFillKind::RolledDice,
        Vec::new(),
        ReducerRouteOwnerGroup::HitPoint,
    ));
    route.push(route_resolve_battle_subject_without_fill(
        ReducerRouteSubjectFamily::InterruptStackResume,
        Vec::new(),
        ReducerRouteOwnerGroup::InterruptStack,
    ));
    route
}

pub fn projection_payload(witness: &InterruptStackResumeWitness) -> String {
    [
        format!(
            "qMaxStackDepthObserved={}",
            witness.max_stack_depth_observed
        ),
        format!("qFinalStackDepth={}", witness.final_stack_depth),
        format!("qPendingTrigger={}", witness.pending_trigger),
        format!("qResumedHole={}", witness.resumed_hole),
        format!(
            "qActiveEffectMutationSeenOnResume={}",
            witness.active_effect_mutation_seen_on_resume
        ),
        format!(
            "qReplayFromRootEquivalent={}",
            witness.replay_from_root_equivalent
        ),
        format!(
            "qResponderReactionAvailable={}",
            witness.responder_reaction_available
        ),
        format!("qTargetHp={}", witness.target_hp),
        format!("qScenarioOutcome={}", witness.scenario_outcome),
        format!("protocolResult={}", witness.protocol_result),
        format!("protocolHoles={}", joined_or_none(&witness.protocol_holes)),
    ]
    .join("\n")
}

fn witness_from_state(state: InterruptStackResumeState) -> InterruptStackResumeWitness {
    InterruptStackResumeWitness {
        max_stack_depth_observed: state.max_stack_depth_observed,
        final_stack_depth: state.final_stack_depth,
        pending_trigger: pending_trigger_ref(state.pending_trigger),
        resumed_hole: resumed_hole_ref(state.resumed_hole),
        active_effect_mutation_seen_on_resume: state.active_effect_mutation_seen_on_resume,
        replay_from_root_equivalent: state.replay_from_root_equivalent,
        responder_reaction_available: state.responder_reaction_available,
        target_hp: state.target_hit_points,
        scenario_outcome: scenario_outcome_ref(state.scenario_outcome),
        protocol_result: protocol_result_ref(state.protocol),
        protocol_holes: protocol_holes(state.protocol),
    }
}

fn pending_trigger_ref(trigger: InterruptPendingTrigger) -> &'static str {
    match trigger {
        InterruptPendingTrigger::NoPendingTrigger => "none",
        InterruptPendingTrigger::AttackHit => "attackHit",
    }
}

fn resumed_hole_ref(hole: InterruptResumeHole) -> &'static str {
    match hole {
        InterruptResumeHole::NoResumedHole => "none",
        InterruptResumeHole::RolledDice => "rolledDice",
    }
}

fn scenario_outcome_ref(outcome: InterruptStackResumeScenarioOutcome) -> &'static str {
    match outcome {
        InterruptStackResumeScenarioOutcome::Init => "Init",
        InterruptStackResumeScenarioOutcome::NestedDeclineResumedOuter => {
            "NestedDeclineResumedOuter"
        }
        InterruptStackResumeScenarioOutcome::ActiveEffectMutationResumed => {
            "ActiveEffectMutationResumed"
        }
        InterruptStackResumeScenarioOutcome::ReplayFromRootResolved => "ReplayFromRootResolved",
    }
}

fn protocol_result_ref(protocol: InterruptStackResumeProtocol) -> &'static str {
    match protocol {
        InterruptStackResumeProtocol::Init => "init",
        InterruptStackResumeProtocol::Resolved => "resolved",
    }
}

fn protocol_holes(protocol: InterruptStackResumeProtocol) -> Vec<&'static str> {
    match protocol {
        InterruptStackResumeProtocol::Init | InterruptStackResumeProtocol::Resolved => vec![],
    }
}

fn joined_or_none(values: &[&'static str]) -> String {
    if values.is_empty() {
        "none".to_string()
    } else {
        values.join(",")
    }
}
