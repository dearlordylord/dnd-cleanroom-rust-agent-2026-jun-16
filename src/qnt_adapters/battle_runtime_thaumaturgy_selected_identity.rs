use crate::rules::roll_modifier_active_effects::{
    cast_thaumaturgy_booming_voice, RollModifierRollMode,
};
use crate::rules::thaumaturgy_selected_identity::{
    ThaumaturgyRollMode, ThaumaturgySelectedIdentityProtocol,
    ThaumaturgySelectedIdentityScenarioOutcome, ThaumaturgySelectedIdentityState,
};

use super::battle_runtime_reducer_route::{
    route_discover_battle_acts_from_route_holes,
    route_resolve_battle_subject_without_fill_from_route_result, route_start_battle,
    ReducerRouteEvent, ReducerRouteOwnerGroup, ReducerRouteResolutionOutcome,
    ReducerRouteSubjectFamily,
};
use super::battle_runtime_roll_modifier_active_effects;

pub const BRANCH_ACTIONS: [&str; 1] = ["doResolveThaumaturgyBoomingVoice"];

pub fn replay_observed_action(observed_action_taken: &str) -> ThaumaturgySelectedIdentityState {
    match observed_action_taken {
        "doResolveThaumaturgyBoomingVoice" => {
            let substrate = cast_thaumaturgy_booming_voice();
            ThaumaturgySelectedIdentityState {
                caster_effect_count: i16::from(substrate.thaumaturgy_effect_active),
                action_available: substrate.action_available,
                intimidation_roll_mode: thaumaturgy_roll_mode(
                    substrate.thaumaturgy_intimidation_roll_mode,
                ),
                wisdom_intimidation_roll_mode: ThaumaturgyRollMode::Normal,
                perception_roll_mode: ThaumaturgyRollMode::Normal,
                scenario_outcome: ThaumaturgySelectedIdentityScenarioOutcome::Resolved,
                protocol: ThaumaturgySelectedIdentityProtocol::Resolved,
            }
        }
        action => panic!("unsupported mbt::actionTaken {action}"),
    }
}

pub fn expected_witness(observed_action_taken: &str) -> ThaumaturgySelectedIdentityState {
    match observed_action_taken {
        "doResolveThaumaturgyBoomingVoice" => ThaumaturgySelectedIdentityState {
            caster_effect_count: 1,
            action_available: false,
            intimidation_roll_mode: ThaumaturgyRollMode::Advantage,
            wisdom_intimidation_roll_mode: ThaumaturgyRollMode::Normal,
            perception_roll_mode: ThaumaturgyRollMode::Normal,
            scenario_outcome: ThaumaturgySelectedIdentityScenarioOutcome::Resolved,
            protocol: ThaumaturgySelectedIdentityProtocol::Resolved,
        },
        action => panic!("unsupported mbt::actionTaken {action}"),
    }
}

pub fn replay_observed_route(observed_action_taken: &str) -> Vec<ReducerRouteEvent> {
    match observed_action_taken {
        "doResolveThaumaturgyBoomingVoice" => {
            battle_runtime_roll_modifier_active_effects::replay_observed_route(
                "doCastThaumaturgyBoomingVoice",
            )
        }
        action => panic!("unsupported mbt::actionTaken {action}"),
    }
}

pub fn expected_route(observed_action_taken: &str) -> Vec<ReducerRouteEvent> {
    match observed_action_taken {
        "doResolveThaumaturgyBoomingVoice" => vec![
            route_start_battle(ReducerRouteOwnerGroup::ActionEconomy),
            route_discover_battle_acts_from_route_holes(
                ReducerRouteSubjectFamily::RollModifierEffect,
                Vec::new(),
                ReducerRouteOwnerGroup::ActiveEffect,
            ),
            route_resolve_battle_subject_without_fill_from_route_result(
                ReducerRouteSubjectFamily::RollModifierEffect,
                ReducerRouteResolutionOutcome::Resolved,
                Vec::new(),
                ReducerRouteOwnerGroup::ActiveEffect,
            ),
        ],
        action => panic!("unsupported mbt::actionTaken {action}"),
    }
}

pub fn projection_payload(state: &ThaumaturgySelectedIdentityState) -> String {
    [
        format!("qCasterEffectCount={}", state.caster_effect_count),
        format!("qActionAvailable={}", state.action_available),
        format!(
            "qIntimidationRollMode={}",
            roll_mode_ref(state.intimidation_roll_mode)
        ),
        format!(
            "qWisdomIntimidationRollMode={}",
            roll_mode_ref(state.wisdom_intimidation_roll_mode)
        ),
        format!(
            "qPerceptionRollMode={}",
            roll_mode_ref(state.perception_roll_mode)
        ),
        format!(
            "qScenarioOutcome={}",
            scenario_outcome_ref(state.scenario_outcome)
        ),
        format!("protocolResult={}", protocol_result_ref(state.protocol)),
        format!(
            "protocolHoles={}",
            joined_or_none(&protocol_holes(state.protocol))
        ),
    ]
    .join("\n")
}

fn roll_mode_ref(mode: ThaumaturgyRollMode) -> &'static str {
    match mode {
        ThaumaturgyRollMode::Normal => "normal",
        ThaumaturgyRollMode::Advantage => "advantage",
    }
}

fn thaumaturgy_roll_mode(mode: RollModifierRollMode) -> ThaumaturgyRollMode {
    match mode {
        RollModifierRollMode::Normal => ThaumaturgyRollMode::Normal,
        RollModifierRollMode::Advantage => ThaumaturgyRollMode::Advantage,
    }
}

fn scenario_outcome_ref(outcome: ThaumaturgySelectedIdentityScenarioOutcome) -> &'static str {
    match outcome {
        ThaumaturgySelectedIdentityScenarioOutcome::Init => "Init",
        ThaumaturgySelectedIdentityScenarioOutcome::Resolved => "Resolved",
    }
}

fn protocol_result_ref(protocol: ThaumaturgySelectedIdentityProtocol) -> &'static str {
    match protocol {
        ThaumaturgySelectedIdentityProtocol::Init => "init",
        ThaumaturgySelectedIdentityProtocol::Resolved => "resolved",
    }
}

fn protocol_holes(protocol: ThaumaturgySelectedIdentityProtocol) -> Vec<&'static str> {
    match protocol {
        ThaumaturgySelectedIdentityProtocol::Init => vec!["WitnessProtocolHole"],
        ThaumaturgySelectedIdentityProtocol::Resolved => vec![],
    }
}

fn joined_or_none(values: &[&'static str]) -> String {
    if values.is_empty() {
        "none".to_string()
    } else {
        values.join(",")
    }
}
