use crate::rules::battle_reducer_spine::{
    discover_battle_acts, resolve_battle_subject, scalar_buff_target_projection_from_battle_result,
    start_standard_battle, Actor, BattleResolutionRequest, BattleScalarBuffFill, BattleState,
    BattleSubject, BattleSubjectKind,
};
use crate::rules::scalar_buff::{
    ScalarBuffTargetHole, ScalarBuffTargetInvalidReason, ScalarBuffTargetProtocol,
    ScalarBuffTargetState,
};

use super::battle_runtime_reducer_route::{
    route_discover_battle_acts, route_resolve_battle_subject_from_result, route_start_battle,
    ReducerRouteEvent, ReducerRouteFillKind, ReducerRouteOwnerGroup, ReducerRouteResolveConnector,
    ReducerRouteResolveFill, ReducerRouteSubjectFamily,
};

pub const BRANCH_ACTIONS: [&str; 2] = ["doFillLongstriderTarget", "doRejectStaleAfterResolved"];

pub fn replay_observed_action(observed_action_taken: &str) -> ScalarBuffTargetState {
    match observed_action_taken {
        "doFillLongstriderTarget" => {
            scalar_buff_target_projection_from_battle_result(&fill_longstrider_target_result())
        }
        "doRejectStaleAfterResolved" => {
            scalar_buff_target_projection_from_battle_result(&reject_stale_scalar_buff_result())
        }
        action => panic!("unsupported mbt::actionTaken {action}"),
    }
}

pub fn expected_witness(observed_action_taken: &str) -> ScalarBuffTargetState {
    replay_observed_action(observed_action_taken)
}

pub fn replay_observed_route(observed_action_taken: &str) -> Vec<ReducerRouteEvent> {
    scalar_buff_route_from_obligation(observed_action_taken)
}

fn scalar_buff_route_from_obligation(observed_action_taken: &str) -> Vec<ReducerRouteEvent> {
    match observed_action_taken {
        "doFillLongstriderTarget" => fill_longstrider_target_route().2,
        "doRejectStaleAfterResolved" => {
            let (state, subject, mut route) = fill_longstrider_target_route();
            let result = resolve_scalar_buff_target(state, subject);
            route.push(scalar_buff_route_event(&result));
            route
        }
        action => panic!("unsupported mbt::actionTaken {action}"),
    }
}

pub fn expected_route(observed_action_taken: &str) -> Vec<ReducerRouteEvent> {
    scalar_buff_route_from_obligation(observed_action_taken)
}

pub fn projection_payload(state: &ScalarBuffTargetState) -> String {
    [
        format!("fighterSpeed={}", state.fighter_speed_feet),
        format!("goblinSpeed={}", state.goblin_speed_feet),
        format!("actionAvailable={}", state.action_available),
        format!("protocolResult={}", protocol_ref(state.protocol)),
        format!(
            "protocolInvalidReason={}",
            invalid_reason_ref(state.protocol)
        ),
        format!("protocolHoles={}", joined_holes(&state.protocol_holes)),
    ]
    .join("\n")
}

fn fill_longstrider_target_route() -> (BattleState, BattleSubject, Vec<ReducerRouteEvent>) {
    let state = start_standard_battle();
    let mut route = vec![route_start_battle(ReducerRouteOwnerGroup::ActionEconomy)];
    let subject = scalar_buff_subject(&state);
    let discovery = discover_battle_acts(&state);
    let holes = discovery
        .available_acts()
        .iter()
        .find(|act| act.subject == subject)
        .map(|act| act.holes.clone())
        .expect("scalar buff target act should be discoverable");
    route.push(route_discover_battle_acts(
        ReducerRouteSubjectFamily::ScalarBuff,
        holes,
        ReducerRouteOwnerGroup::SpellSlotAndActionEconomy,
    ));
    let result = resolve_scalar_buff_target(state, subject);
    route.push(scalar_buff_route_event(&result));
    (result.into_state(), subject, route)
}

fn fill_longstrider_target_result() -> crate::rules::battle_reducer_spine::BattleResolutionResult {
    let state = start_standard_battle();
    let subject = scalar_buff_subject(&state);
    resolve_scalar_buff_target(state, subject)
}

fn reject_stale_scalar_buff_result() -> crate::rules::battle_reducer_spine::BattleResolutionResult {
    let result = fill_longstrider_target_result();
    let state = result.into_state();
    resolve_scalar_buff_target(state, scalar_buff_subject(&start_standard_battle()))
}

fn resolve_scalar_buff_target(
    state: BattleState,
    subject: BattleSubject,
) -> crate::rules::battle_reducer_spine::BattleResolutionResult {
    resolve_battle_subject(
        state,
        BattleResolutionRequest::scalar_buff(
            subject,
            BattleScalarBuffFill::TargetChoice(Actor::Goblin),
        )
        .expect("scalar buff subject kind should match scalar buff request"),
    )
}

fn scalar_buff_subject(state: &BattleState) -> BattleSubject {
    discover_battle_acts(state)
        .available_acts()
        .iter()
        .map(|act| act.subject)
        .find(|subject| subject.kind == BattleSubjectKind::ScalarBuffTargetSpell)
        .unwrap_or(BattleSubject {
            kind: BattleSubjectKind::ScalarBuffTargetSpell,
            actor: Actor::Fighter,
            target: None,
            stage: crate::rules::weapon_attack_ordering::WeaponAttackFrontierStage::Resolved,
            damage_modifier: 0,
        })
}

fn scalar_buff_route_event(
    result: &crate::rules::battle_reducer_spine::BattleResolutionResult,
) -> ReducerRouteEvent {
    route_resolve_battle_subject_from_result(
        ReducerRouteResolveConnector {
            subject: ReducerRouteSubjectFamily::ScalarBuff,
            fill: ReducerRouteResolveFill::Fill(ReducerRouteFillKind::TargetChoice),
            owner: scalar_buff_route_owner(result),
        },
        result,
    )
}

fn scalar_buff_route_owner(
    result: &crate::rules::battle_reducer_spine::BattleResolutionResult,
) -> ReducerRouteOwnerGroup {
    if matches!(
        result.outcome(),
        crate::rules::battle_reducer_spine::BattleResolutionOutcome::Invalid(_)
    ) {
        ReducerRouteOwnerGroup::HoleFrontier
    } else {
        ReducerRouteOwnerGroup::ActiveEffect
    }
}

fn protocol_ref(protocol: ScalarBuffTargetProtocol) -> &'static str {
    match protocol {
        ScalarBuffTargetProtocol::Init => "init",
        ScalarBuffTargetProtocol::Resolved => "resolved",
        ScalarBuffTargetProtocol::Invalid(_) => "invalid",
    }
}

fn invalid_reason_ref(protocol: ScalarBuffTargetProtocol) -> &'static str {
    match protocol {
        ScalarBuffTargetProtocol::Invalid(ScalarBuffTargetInvalidReason::StaleSubject) => {
            "WStaleSubject"
        }
        ScalarBuffTargetProtocol::Init | ScalarBuffTargetProtocol::Resolved => "none",
    }
}

fn joined_holes(holes: &[ScalarBuffTargetHole]) -> String {
    if holes.is_empty() {
        return "none".to_string();
    }

    holes
        .iter()
        .map(|hole| match hole {
            ScalarBuffTargetHole::TargetChoice => "TargetChoice",
        })
        .collect::<Vec<_>>()
        .join(",")
}
