use crate::rules::battle_reducer_spine::{
    death_saving_throw_state_from_combatant, discover_battle_acts, resolve_battle_subject,
    start_death_saving_throw_battle, Actor, BattleHoleKind, BattleResolutionInvalidReason,
    BattleResolutionOutcome, BattleResolutionRequest, BattleResolutionResult, BattleState,
    BattleSubject, BattleSubjectKind,
};
use crate::rules::hit_points::{
    death_saving_throw_initial_state, discover_death_saving_throw, fill_death_saving_throw,
    reject_wrong_actor_after_resolved, DeathSavingThrowFacts, DeathSavingThrowInvalidReason,
    DeathSavingThrowProtocol, DeathSavingThrowState, DeathSavingThrowTurnRole,
};

use super::battle_runtime_reducer_route::{
    route_discover_battle_acts, route_resolve_battle_subject,
    route_resolve_battle_subject_from_result, route_resolve_battle_subject_from_route_result,
    route_start_battle, ReducerRouteEvent, ReducerRouteFillKind, ReducerRouteOwnerGroup,
    ReducerRouteResolutionOutcome, ReducerRouteResolveConnector, ReducerRouteResolveFill,
    ReducerRouteSubjectFamily,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DeathSavingThrowWitness {
    pub current_turn_role: &'static str,
    pub target_hp: i16,
    pub target_unconscious: bool,
    pub target_stable: bool,
    pub target_dead: bool,
    pub target_death_successes: i16,
    pub target_death_failures: i16,
    pub protocol_result: &'static str,
    pub protocol_invalid_reason: &'static str,
    pub protocol_holes: Vec<&'static str>,
}

pub const BRANCH_ACTIONS: [&str; 3] = [
    "doDiscoverEndTurnDeathSavingThrow",
    "doFillDeathSavingThrow",
    "doRejectWrongActorEndTurnAfterResolved",
];

pub const FILL_SAMPLE_NATURAL_D20S: [i16; 4] = [1, 5, 10, 20];

pub fn replay_observed_action(observed_action_taken: &str) -> DeathSavingThrowWitness {
    match observed_action_taken {
        "doDiscoverEndTurnDeathSavingThrow" => {
            witness_from_battle_state(&discovered_death_saving_throw_battle().0)
        }
        "doFillDeathSavingThrow" => replay_fill_death_saving_throw(10),
        "doRejectWrongActorEndTurnAfterResolved" => replay_reject_wrong_actor_after_resolved(),
        action => panic!("unsupported mbt::actionTaken {action}"),
    }
}

pub fn replay_fill_death_saving_throw(natural_d20: i16) -> DeathSavingThrowWitness {
    let result = death_saving_throw_fill_result(natural_d20).0;
    witness_from_death_saving_throw_result(result)
}

pub fn expected_witness(observed_action_taken: &str) -> DeathSavingThrowWitness {
    match observed_action_taken {
        "doDiscoverEndTurnDeathSavingThrow" => witness_from_state(discovered()),
        "doFillDeathSavingThrow" => witness_from_state(filled(10)),
        "doRejectWrongActorEndTurnAfterResolved" => witness_from_state(reject_wrong_actor()),
        action => panic!("unsupported mbt::actionTaken {action}"),
    }
}

pub fn replay_observed_route(observed_action_taken: &str) -> Vec<ReducerRouteEvent> {
    match observed_action_taken {
        "doDiscoverEndTurnDeathSavingThrow" => discovered_death_saving_throw_battle().1,
        "doFillDeathSavingThrow" => death_saving_throw_fill_result(10).1,
        "doRejectWrongActorEndTurnAfterResolved" => reject_wrong_actor_battle_result().1,
        action => panic!("unsupported mbt::actionTaken {action}"),
    }
}

pub fn expected_route(observed_action_taken: &str) -> Vec<ReducerRouteEvent> {
    match observed_action_taken {
        "doDiscoverEndTurnDeathSavingThrow" => initial_death_saving_throw_route(),
        "doFillDeathSavingThrow" => {
            let mut route = initial_death_saving_throw_route();
            route.push(route_resolve_battle_subject(
                ReducerRouteSubjectFamily::DeathSavingThrow,
                ReducerRouteFillKind::DeathSavingThrow,
                Vec::new(),
                ReducerRouteOwnerGroup::HitPointAndZeroHpLifecycle,
            ));
            route
        }
        "doRejectWrongActorEndTurnAfterResolved" => {
            let mut route = expected_route("doFillDeathSavingThrow");
            route.push(route_resolve_battle_subject_from_route_result(
                ReducerRouteSubjectFamily::DeathSavingThrow,
                ReducerRouteFillKind::DeathSavingThrow,
                ReducerRouteResolutionOutcome::Invalid(BattleResolutionInvalidReason::WrongActor),
                Vec::new(),
                ReducerRouteOwnerGroup::HitPointAndZeroHpLifecycle,
            ));
            route
        }
        action => panic!("unsupported mbt::actionTaken {action}"),
    }
}

pub fn projection_payload(witness: &DeathSavingThrowWitness) -> String {
    [
        format!("currentTurnRole={}", witness.current_turn_role),
        format!("targetHp={}", witness.target_hp),
        format!("targetUnconscious={}", witness.target_unconscious),
        format!("targetStable={}", witness.target_stable),
        format!("targetDead={}", witness.target_dead),
        format!("targetDeathSuccesses={}", witness.target_death_successes),
        format!("targetDeathFailures={}", witness.target_death_failures),
        format!("protocolResult={}", witness.protocol_result),
        format!("protocolInvalidReason={}", witness.protocol_invalid_reason),
        format!("protocolHoles={}", joined_or_none(&witness.protocol_holes)),
    ]
    .join("\n")
}

fn discovered() -> DeathSavingThrowState {
    discover_death_saving_throw(death_saving_throw_initial_state())
}

fn filled(natural_d20: i16) -> DeathSavingThrowState {
    fill_death_saving_throw(discovered(), DeathSavingThrowFacts { natural_d20 })
}

fn reject_wrong_actor() -> DeathSavingThrowState {
    reject_wrong_actor_after_resolved(filled(20))
}

fn discovered_death_saving_throw_battle() -> (BattleState, Vec<ReducerRouteEvent>) {
    let state = start_death_saving_throw_battle();
    let mut route = vec![route_start_battle(ReducerRouteOwnerGroup::ActionEconomy)];
    let act = discovered_death_saving_throw_act(&state);
    route.push(route_discover_battle_acts(
        ReducerRouteSubjectFamily::DeathSavingThrow,
        act.holes,
        ReducerRouteOwnerGroup::HitPointAndZeroHpLifecycle,
    ));
    (state, route)
}

fn death_saving_throw_fill_result(
    natural_d20: i16,
) -> (BattleResolutionResult, Vec<ReducerRouteEvent>) {
    let (state, mut route) = discovered_death_saving_throw_battle();
    let subject = discovered_death_saving_throw_subject(&state);
    let result =
        resolve_death_saving_throw_subject(state, subject, DeathSavingThrowFacts { natural_d20 });
    route.push(death_saving_throw_route_event(&result));
    (result, route)
}

fn reject_wrong_actor_battle_result() -> (BattleResolutionResult, Vec<ReducerRouteEvent>) {
    let (result, mut route) = death_saving_throw_fill_result(20);
    let outcome = result.outcome();
    if outcome != BattleResolutionOutcome::Resolved {
        panic!("natural 20 death saving throw should resolve before wrong-actor rejection, got {outcome:?}");
    }
    let state = result.into_state();
    let stale_subject = BattleSubject {
        kind: BattleSubjectKind::DeathSavingThrow,
        actor: Actor::Fighter,
        target: Some(Actor::Fighter),
        stage: crate::rules::weapon_attack_ordering::WeaponAttackFrontierStage::Resolved,
        damage_modifier: 0,
    };
    let result = resolve_death_saving_throw_subject(
        state,
        stale_subject,
        DeathSavingThrowFacts { natural_d20: 10 },
    );
    route.push(death_saving_throw_route_event(&result));
    (result, route)
}

fn resolve_death_saving_throw_subject(
    state: BattleState,
    subject: BattleSubject,
    facts: DeathSavingThrowFacts,
) -> BattleResolutionResult {
    let request = BattleResolutionRequest::death_saving_throw(subject, facts)
        .expect("death-saving-throw adapter should build a matching typed request");
    resolve_battle_subject(state, request)
}

fn death_saving_throw_route_event(result: &BattleResolutionResult) -> ReducerRouteEvent {
    route_resolve_battle_subject_from_result(
        ReducerRouteResolveConnector {
            subject: ReducerRouteSubjectFamily::DeathSavingThrow,
            fill: ReducerRouteResolveFill::Fill(ReducerRouteFillKind::DeathSavingThrow),
            owner: ReducerRouteOwnerGroup::HitPointAndZeroHpLifecycle,
        },
        result,
    )
}

fn replay_reject_wrong_actor_after_resolved() -> DeathSavingThrowWitness {
    witness_from_death_saving_throw_result(reject_wrong_actor_battle_result().0)
}

fn initial_death_saving_throw_route() -> Vec<ReducerRouteEvent> {
    vec![
        route_start_battle(ReducerRouteOwnerGroup::ActionEconomy),
        route_discover_battle_acts(
            ReducerRouteSubjectFamily::DeathSavingThrow,
            vec![BattleHoleKind::DeathSavingThrow],
            ReducerRouteOwnerGroup::HitPointAndZeroHpLifecycle,
        ),
    ]
}

fn discovered_death_saving_throw_act(
    state: &BattleState,
) -> crate::rules::battle_reducer_spine::AvailableBattleAct {
    discover_battle_acts(state)
        .into_available_acts()
        .into_iter()
        .find(|act| act.subject.kind == BattleSubjectKind::DeathSavingThrow)
        .expect("death saving throw route should discover one death-save act")
}

fn discovered_death_saving_throw_subject(state: &BattleState) -> BattleSubject {
    discovered_death_saving_throw_act(state).subject
}

fn witness_from_death_saving_throw_result(
    result: BattleResolutionResult,
) -> DeathSavingThrowWitness {
    match result.outcome() {
        BattleResolutionOutcome::Resolved => witness_from_battle_state(result.state()),
        BattleResolutionOutcome::Invalid(reason) => {
            witness_from_battle_state_with_protocol(result.state(), invalid_protocol(reason))
        }
        BattleResolutionOutcome::NeedsHoles => witness_from_battle_state_with_protocol(
            result.state(),
            DeathSavingThrowProtocol::NeedsSavingThrow,
        ),
    }
}

fn witness_from_battle_state(state: &BattleState) -> DeathSavingThrowWitness {
    let mut death_save = fighter_death_saving_throw_state(state);
    death_save.current_turn_role = if state.initiative.still_to_act.actor == Actor::Fighter {
        DeathSavingThrowTurnRole::Actor
    } else {
        DeathSavingThrowTurnRole::Target
    };
    death_save.protocol = if death_saving_throw_route_holes(state).is_empty() {
        DeathSavingThrowProtocol::Resolved
    } else {
        DeathSavingThrowProtocol::NeedsSavingThrow
    };
    witness_from_state(death_save)
}

fn witness_from_battle_state_with_protocol(
    state: &BattleState,
    protocol: DeathSavingThrowProtocol,
) -> DeathSavingThrowWitness {
    let mut death_save = fighter_death_saving_throw_state(state);
    death_save.current_turn_role = if state.initiative.still_to_act.actor == Actor::Fighter {
        DeathSavingThrowTurnRole::Actor
    } else {
        DeathSavingThrowTurnRole::Target
    };
    death_save.protocol = protocol;
    witness_from_state(death_save)
}

fn fighter_death_saving_throw_state(state: &BattleState) -> DeathSavingThrowState {
    death_saving_throw_state_from_combatant(state.fighter)
        .expect("death-saving-throw adapter fixture fighter uses death saving throws")
}

fn death_saving_throw_route_holes(state: &BattleState) -> Vec<BattleHoleKind> {
    discover_battle_acts(state)
        .into_available_acts()
        .into_iter()
        .find(|act| act.subject.kind == BattleSubjectKind::DeathSavingThrow)
        .map(|act| act.holes)
        .unwrap_or_default()
}

fn invalid_protocol(reason: BattleResolutionInvalidReason) -> DeathSavingThrowProtocol {
    match reason {
        BattleResolutionInvalidReason::WrongActor => {
            DeathSavingThrowProtocol::Invalid(DeathSavingThrowInvalidReason::WrongActor)
        }
        BattleResolutionInvalidReason::InvalidFill
        | BattleResolutionInvalidReason::StaleSubject
        | BattleResolutionInvalidReason::WrongTarget => {
            DeathSavingThrowProtocol::Invalid(DeathSavingThrowInvalidReason::InvalidFill)
        }
    }
}

fn witness_from_state(state: DeathSavingThrowState) -> DeathSavingThrowWitness {
    DeathSavingThrowWitness {
        current_turn_role: turn_role_ref(state.current_turn_role),
        target_hp: state.target_hp,
        target_unconscious: state.target_unconscious,
        target_stable: state.target_stable,
        target_dead: state.target_dead,
        target_death_successes: state.target_death_successes,
        target_death_failures: state.target_death_failures,
        protocol_result: protocol_result_ref(state.protocol),
        protocol_invalid_reason: protocol_invalid_reason_ref(state.protocol),
        protocol_holes: protocol_holes(state.protocol),
    }
}

fn turn_role_ref(role: DeathSavingThrowTurnRole) -> &'static str {
    match role {
        DeathSavingThrowTurnRole::Actor => "actor",
        DeathSavingThrowTurnRole::Target => "target",
    }
}

fn protocol_result_ref(protocol: DeathSavingThrowProtocol) -> &'static str {
    match protocol {
        DeathSavingThrowProtocol::Init => "init",
        DeathSavingThrowProtocol::NeedsSavingThrow => "needsHoles",
        DeathSavingThrowProtocol::Resolved => "resolved",
        DeathSavingThrowProtocol::Invalid(_) => "invalid",
    }
}

fn protocol_invalid_reason_ref(protocol: DeathSavingThrowProtocol) -> &'static str {
    match protocol {
        DeathSavingThrowProtocol::Invalid(DeathSavingThrowInvalidReason::WrongActor) => {
            "WWrongActor"
        }
        DeathSavingThrowProtocol::Invalid(DeathSavingThrowInvalidReason::InvalidFill) => {
            "WInvalidFill"
        }
        DeathSavingThrowProtocol::Init
        | DeathSavingThrowProtocol::NeedsSavingThrow
        | DeathSavingThrowProtocol::Resolved => "",
    }
}

fn protocol_holes(protocol: DeathSavingThrowProtocol) -> Vec<&'static str> {
    match protocol {
        DeathSavingThrowProtocol::NeedsSavingThrow => vec!["DeathSavingThrow"],
        DeathSavingThrowProtocol::Init
        | DeathSavingThrowProtocol::Resolved
        | DeathSavingThrowProtocol::Invalid(_) => vec![],
    }
}

fn joined_or_none(values: &[&'static str]) -> String {
    if values.is_empty() {
        "none".to_string()
    } else {
        values.join(",")
    }
}
