use super::battle_runtime_reducer_route::{
    replay_generic_battle_route, GenericBattleRouteStep, ReducerRouteEvent,
};
use crate::rules::{
    battle_reducer_spine::{
        advance_turn, start_turn_boundary_effect_lifecycle_battle, Actor, BattleGenericRouteFill,
        BattleState, BattleSubjectKind, BattleTurnAdvanceResult,
    },
    turn_boundary_effect_lifecycle::{
        resolve_source_next_turn, resolve_target_start_turn, TurnBoundaryActor,
        TurnBoundaryEffectLifecycleState, TurnBoundaryHoleOrder, TurnBoundaryLifecycleProtocol,
        TurnBoundaryLifecycleScenario,
    },
};

pub const BRANCH_ACTIONS: [&str; 2] = ["doResolveTargetStartTurn", "doResolveSourceNextTurn"];

pub fn replay_observed_action(observed_action_taken: &str) -> TurnBoundaryEffectLifecycleState {
    match observed_action_taken {
        "doResolveTargetStartTurn" => {
            let result = advance_turn(start_turn_boundary_effect_lifecycle_battle());
            assert_eq!(result.previous_actor, Actor::Fighter);
            assert_eq!(result.next_actor, Actor::Goblin);
            assert_eq!(result.round, 1);
            project_turn_advance(&result)
        }
        "doResolveSourceNextTurn" => {
            let target_start = advance_turn(start_turn_boundary_effect_lifecycle_battle());
            let source_next = advance_turn(target_start.state);
            assert_eq!(source_next.previous_actor, Actor::Goblin);
            assert_eq!(source_next.next_actor, Actor::Fighter);
            assert_eq!(source_next.round, 2);
            project_turn_advance(&source_next)
        }
        action => panic!("unsupported mbt::actionTaken {action}"),
    }
}

pub fn expected_witness(observed_action_taken: &str) -> TurnBoundaryEffectLifecycleState {
    match observed_action_taken {
        "doResolveTargetStartTurn" => resolve_target_start_turn(),
        "doResolveSourceNextTurn" => resolve_source_next_turn(),
        action => panic!("unsupported mbt::actionTaken {action}"),
    }
}

pub fn replay_observed_route(observed_action_taken: &str) -> Vec<ReducerRouteEvent> {
    match observed_action_taken {
        "doResolveTargetStartTurn" => target_start_turn_route(),
        "doResolveSourceNextTurn" => source_next_turn_route(),
        action => panic!("unsupported mbt::actionTaken {action}"),
    }
}

pub fn expected_route(observed_action_taken: &str) -> Vec<ReducerRouteEvent> {
    replay_observed_route(observed_action_taken)
}

pub fn projection_payload(state: &TurnBoundaryEffectLifecycleState) -> String {
    [
        format!("qScenario={}", scenario_ref(state.scenario)),
        format!("qActor={}", actor_ref(state.actor)),
        format!("qRound={}", state.round),
        format!("qTargetHp={}", state.target_hp),
        format!("qTurnStartDamageActive={}", state.turn_start_damage_active),
        format!("qTurnEndDamageActive={}", state.turn_end_damage_active),
        format!("qUntilNextTurnActive={}", state.until_next_turn_active),
        format!(
            "qStartTurnOngoingFeatureActive={}",
            state.start_turn_ongoing_feature_active
        ),
        format!(
            "qEndTurnOngoingFeatureActive={}",
            state.end_turn_ongoing_feature_active
        ),
        format!(
            "qTurnStartDamageAppliedBeforeEndDamage={}",
            state.turn_start_damage_applied_before_end_damage
        ),
        format!(
            "qTurnEndDamageAppliedBeforeExpiry={}",
            state.turn_end_damage_applied_before_expiry
        ),
        format!(
            "qEndTurnOngoingExpiredAtTargetEnd={}",
            state.end_turn_ongoing_expired_at_target_end
        ),
        format!(
            "qUntilNextTurnExpiredAtSourceStart={}",
            state.until_next_turn_expired_at_source_start
        ),
        format!(
            "qStartTurnOngoingExpiredAtSourceStart={}",
            state.start_turn_ongoing_expired_at_source_start
        ),
        format!(
            "qTurnStartDurationExpiredAfterRoundTick={}",
            state.turn_start_duration_expired_after_round_tick
        ),
        format!("qLastHoleOrder={}", hole_order_ref(state.last_hole_order)),
        format!("protocolResult={}", protocol_result_ref(state.protocol)),
        format!(
            "protocolHoles={}",
            joined_or_none(&protocol_holes(state.protocol))
        ),
    ]
    .join("\n")
}

fn scenario_ref(scenario: TurnBoundaryLifecycleScenario) -> &'static str {
    match scenario {
        TurnBoundaryLifecycleScenario::Init => "Init",
        TurnBoundaryLifecycleScenario::TargetStartTurnResolved => "TargetStartTurnResolved",
        TurnBoundaryLifecycleScenario::SourceNextTurnResolved => "SourceNextTurnResolved",
    }
}

fn actor_ref(actor: TurnBoundaryActor) -> &'static str {
    match actor {
        TurnBoundaryActor::SourceTurn => "SourceTurn",
        TurnBoundaryActor::TargetTurn => "TargetTurn",
    }
}

fn hole_order_ref(order: TurnBoundaryHoleOrder) -> &'static str {
    match order {
        TurnBoundaryHoleOrder::NoBoundaryHoles => "NoBoundaryHoles",
        TurnBoundaryHoleOrder::TurnStartDamageThenSave => "TurnStartDamageThenSave",
        TurnBoundaryHoleOrder::TurnEndDamageOnly => "TurnEndDamageOnly",
    }
}

fn protocol_result_ref(protocol: TurnBoundaryLifecycleProtocol) -> &'static str {
    match protocol {
        TurnBoundaryLifecycleProtocol::Init => "init",
        TurnBoundaryLifecycleProtocol::Resolved => "resolved",
    }
}

fn protocol_holes(protocol: TurnBoundaryLifecycleProtocol) -> Vec<&'static str> {
    match protocol {
        TurnBoundaryLifecycleProtocol::Init => vec!["TurnBoundaryLifecycle"],
        TurnBoundaryLifecycleProtocol::Resolved => vec![],
    }
}

fn joined_or_none(values: &[&'static str]) -> String {
    if values.is_empty() {
        "none".to_string()
    } else {
        values.join(",")
    }
}

fn project_turn_advance(result: &BattleTurnAdvanceResult) -> TurnBoundaryEffectLifecycleState {
    let scenario = match (result.previous_actor, result.next_actor, result.round) {
        (Actor::Fighter, Actor::Goblin, 1) => {
            TurnBoundaryLifecycleScenario::TargetStartTurnResolved
        }
        (Actor::Goblin, Actor::Fighter, 2) => TurnBoundaryLifecycleScenario::SourceNextTurnResolved,
        (previous_actor, next_actor, round) => panic!(
            "unsupported turn-boundary advance metadata {previous_actor:?}->{next_actor:?} round {round}"
        ),
    };
    let actor = match result.next_actor {
        Actor::Fighter => TurnBoundaryActor::SourceTurn,
        Actor::Goblin => TurnBoundaryActor::TargetTurn,
        Actor::Rogue | Actor::Skeleton => TurnBoundaryActor::TargetTurn,
    };
    project_battle_state_after_turn_advance(&result.state, scenario, actor, result.round)
}

fn project_battle_state_after_turn_advance(
    state: &BattleState,
    scenario: TurnBoundaryLifecycleScenario,
    actor: TurnBoundaryActor,
    round: i16,
) -> TurnBoundaryEffectLifecycleState {
    let target_start_resolved = scenario == TurnBoundaryLifecycleScenario::TargetStartTurnResolved
        || scenario == TurnBoundaryLifecycleScenario::SourceNextTurnResolved;
    let source_next_resolved = scenario == TurnBoundaryLifecycleScenario::SourceNextTurnResolved;

    TurnBoundaryEffectLifecycleState {
        scenario,
        actor,
        round,
        target_hp: state.goblin.hp,
        turn_start_damage_active: state.turn_boundary_effects.turn_start_damage_active,
        turn_end_damage_active: state.turn_boundary_effects.turn_end_damage_active,
        until_next_turn_active: state.turn_boundary_effects.until_next_turn_active,
        start_turn_ongoing_feature_active: state
            .turn_boundary_effects
            .start_turn_ongoing_feature_active,
        end_turn_ongoing_feature_active: state
            .turn_boundary_effects
            .end_turn_ongoing_feature_active,
        turn_start_damage_applied_before_end_damage: target_start_resolved,
        turn_end_damage_applied_before_expiry: source_next_resolved,
        end_turn_ongoing_expired_at_target_end: source_next_resolved,
        until_next_turn_expired_at_source_start: source_next_resolved,
        start_turn_ongoing_expired_at_source_start: source_next_resolved,
        turn_start_duration_expired_after_round_tick: source_next_resolved,
        last_hole_order: if source_next_resolved {
            TurnBoundaryHoleOrder::TurnEndDamageOnly
        } else if target_start_resolved {
            TurnBoundaryHoleOrder::TurnStartDamageThenSave
        } else {
            TurnBoundaryHoleOrder::NoBoundaryHoles
        },
        protocol: if scenario == TurnBoundaryLifecycleScenario::Init {
            TurnBoundaryLifecycleProtocol::Init
        } else {
            TurnBoundaryLifecycleProtocol::Resolved
        },
    }
}

fn target_start_turn_route() -> Vec<ReducerRouteEvent> {
    replay_generic_battle_route(&[
        discover(BattleSubjectKind::TurnBoundaryEffectLifecycleTargetStartDamage),
        resolve(
            BattleSubjectKind::TurnBoundaryEffectLifecycleTargetStartDamage,
            BattleGenericRouteFill::RolledDice,
        ),
        resolve(
            BattleSubjectKind::TurnBoundaryEffectLifecycleTargetStartSave,
            BattleGenericRouteFill::SavingThrowOutcome,
        ),
    ])
}

fn source_next_turn_route() -> Vec<ReducerRouteEvent> {
    let mut route = target_start_turn_route();
    route.extend(
        replay_generic_battle_route(&[
            discover(BattleSubjectKind::TurnBoundaryEffectLifecycleSourceNextDamage),
            resolve(
                BattleSubjectKind::TurnBoundaryEffectLifecycleSourceNextDamage,
                BattleGenericRouteFill::RolledDice,
            ),
            resolve(
                BattleSubjectKind::TurnBoundaryEffectLifecycleSourceNextActiveEffect,
                BattleGenericRouteFill::WithoutFill,
            ),
            resolve(
                BattleSubjectKind::TurnBoundaryEffectLifecycleSourceNextTurnBoundary,
                BattleGenericRouteFill::WithoutFill,
            ),
        ])
        .into_iter()
        .skip(1),
    );
    route
}

fn discover(kind: BattleSubjectKind) -> GenericBattleRouteStep {
    GenericBattleRouteStep::Discover(kind)
}

fn resolve(kind: BattleSubjectKind, fill: BattleGenericRouteFill) -> GenericBattleRouteStep {
    GenericBattleRouteStep::Resolve { kind, fill }
}
