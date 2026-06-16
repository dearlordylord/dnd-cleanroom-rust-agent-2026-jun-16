use crate::rules::turn_boundary_effect_lifecycle::{
    resolve_source_next_turn, resolve_target_start_turn, TurnBoundaryActor,
    TurnBoundaryEffectLifecycleState, TurnBoundaryHoleOrder, TurnBoundaryLifecycleProtocol,
    TurnBoundaryLifecycleScenario,
};

pub const BRANCH_ACTIONS: [&str; 2] = ["doResolveTargetStartTurn", "doResolveSourceNextTurn"];

pub fn replay_observed_action(observed_action_taken: &str) -> TurnBoundaryEffectLifecycleState {
    match observed_action_taken {
        "doResolveTargetStartTurn" => resolve_target_start_turn(),
        "doResolveSourceNextTurn" => resolve_source_next_turn(),
        action => panic!("unsupported mbt::actionTaken {action}"),
    }
}

pub fn expected_witness(observed_action_taken: &str) -> TurnBoundaryEffectLifecycleState {
    replay_observed_action(observed_action_taken)
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
