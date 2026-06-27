use super::battle_runtime_reducer_route::{
    observed_reducer_route, route_discover_battle_acts, route_resolve_battle_subject,
    route_start_battle, setup_from_battle_state, ReducerRouteEvent, ReducerRouteFillKind,
    ReducerRouteOwnerGroup, ReducerRouteSubjectFamily,
};
use crate::rules::battle_reducer_spine::{
    discover_battle_acts_observed, distant_spell_from_battle, resolve_battle_subject_observed,
    start_battle_observed, start_metamagic_option_spell_battle, BattleEntrypointTrace,
    BattleMetamagicOptionFacts, BattleMetamagicOptionSpellEffect, BattleMetamagicOptionSpellFill,
    BattleMetamagicSpellModification, BattleResolutionRequest, BattleSubjectKind,
};
use crate::rules::sorcerer_metamagic::{
    DistantSpellProtocol, DistantSpellScenarioResult, DistantSpellState,
};

pub const BRANCH_ACTIONS: [&str; 1] = ["doResolveDistantObjectLight"];

pub fn replay_observed_action(observed_action_taken: &str) -> DistantSpellState {
    let (state, _) = replay_reducer_route(observed_action_taken);
    state
}

pub fn replay_observed_route(observed_action_taken: &str) -> Vec<ReducerRouteEvent> {
    let (_, route) = replay_reducer_route(observed_action_taken);
    route
}

pub fn expected_witness(observed_action_taken: &str) -> DistantSpellState {
    match observed_action_taken {
        "doResolveDistantObjectLight" => DistantSpellState {
            sorcery_points_remaining: 1,
            light_emitter_count: 1,
            bright_radius_feet: 20,
            dim_additional_feet: 20,
            scenario_result: DistantSpellScenarioResult::DistantObjectLight,
            protocol: DistantSpellProtocol::Resolved,
        },
        action => panic!("unsupported mbt::actionTaken {action}"),
    }
}

pub fn expected_route(observed_action_taken: &str) -> Vec<ReducerRouteEvent> {
    match observed_action_taken {
        "doResolveDistantObjectLight" => vec![
            route_start_battle(ReducerRouteOwnerGroup::ActionEconomy),
            route_discover_battle_acts(
                ReducerRouteSubjectFamily::MetamagicOptionSpell,
                Vec::new(),
                ReducerRouteOwnerGroup::FeatureResource,
            ),
            route_resolve_battle_subject(
                ReducerRouteSubjectFamily::MetamagicOptionSpell,
                ReducerRouteFillKind::UnitFeatureDecision,
                Vec::new(),
                ReducerRouteOwnerGroup::ObjectBoundary,
            ),
        ],
        action => panic!("unsupported mbt::actionTaken {action}"),
    }
}

pub fn projection_payload(state: &DistantSpellState) -> String {
    [
        format!("sorceryPointsRemaining={}", state.sorcery_points_remaining),
        format!("lightEmitterCount={}", state.light_emitter_count),
        format!("brightRadiusFeet={}", state.bright_radius_feet),
        format!("dimAdditionalFeet={}", state.dim_additional_feet),
        format!("scenarioResult={}", scenario_ref(state.scenario_result)),
        format!("protocolResult={}", protocol_ref(state.protocol)),
        "protocolHoles=none".to_string(),
    ]
    .join("\n")
}

fn replay_reducer_route(
    observed_action_taken: &str,
) -> (DistantSpellState, Vec<ReducerRouteEvent>) {
    let mut trace = BattleEntrypointTrace::default();
    let state = start_battle_observed(
        setup_from_battle_state(start_metamagic_option_spell_battle(2)),
        &mut trace,
    )
    .state;
    let discovery = discover_battle_acts_observed(&state, &mut trace);
    let subject = discovery
        .available_acts()
        .iter()
        .find(|act| act.subject.kind == BattleSubjectKind::MetamagicOptionSpell)
        .map(|act| act.subject)
        .unwrap_or_else(|| {
            panic!("metamagic option spell subject not discovered for {observed_action_taken}")
        });
    let request = BattleResolutionRequest::metamagic_option_spell(
        subject,
        BattleMetamagicOptionSpellFill {
            option_facts: metamagic_option_facts(1),
            effect: effect_for_action(observed_action_taken),
            options_already_applied_to_spell: 0,
            selected_second_option_supported: true,
            spell_uses_level_one_plus_slot: false,
            spell_consumes_magic_action: true,
        },
    )
    .expect("adapter selects the metamagic option spell subject");
    let result = resolve_battle_subject_observed(state, request, &mut trace);
    (
        distant_spell_from_battle(result.state()),
        observed_reducer_route(&trace, &[ReducerRouteSubjectFamily::MetamagicOptionSpell]),
    )
}

const fn metamagic_option_facts(sorcery_point_cost: i16) -> BattleMetamagicOptionFacts {
    BattleMetamagicOptionFacts {
        selected_option_admitted: true,
        sorcery_point_cost,
        modification: BattleMetamagicSpellModification::SpellRangeExtension,
        changes_action_casting_time_to_bonus_action: false,
        permits_multiple_options_for_spell: false,
    }
}

fn effect_for_action(observed_action_taken: &str) -> BattleMetamagicOptionSpellEffect {
    match observed_action_taken {
        "doResolveDistantObjectLight" => BattleMetamagicOptionSpellEffect::ObjectRangeLight {
            bright_radius_feet: 20,
            dim_additional_feet: 20,
        },
        action => panic!("unsupported mbt::actionTaken {action}"),
    }
}

fn scenario_ref(result: DistantSpellScenarioResult) -> &'static str {
    match result {
        DistantSpellScenarioResult::Init => "init",
        DistantSpellScenarioResult::DistantObjectLight => "distantObjectLight",
    }
}

fn protocol_ref(protocol: DistantSpellProtocol) -> &'static str {
    match protocol {
        DistantSpellProtocol::Init => "init",
        DistantSpellProtocol::Resolved => "resolved",
    }
}
