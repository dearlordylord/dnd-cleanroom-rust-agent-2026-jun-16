use super::battle_runtime_reducer_route::{
    battle_setup_from_state, observed_reducer_route, route_discover_battle_acts,
    route_resolve_battle_subject, route_start_battle, ReducerRouteEvent, ReducerRouteFillKind,
    ReducerRouteOwnerGroup, ReducerRouteSubjectFamily,
};
use crate::rules::battle_reducer_spine::{
    discover_battle_acts_observed, extended_spell_from_battle, resolve_battle_subject_observed,
    start_battle_observed, start_metamagic_option_spell_battle, BattleEntrypointTrace,
    BattleMetamagicOptionFacts, BattleMetamagicOptionSpellEffect, BattleMetamagicOptionSpellFill,
    BattleResolutionRequest, BattleSubjectKind,
};
use crate::rules::sorcerer_metamagic::{
    ExtendedSpellConcentrationSaveMode, ExtendedSpellProtocol, ExtendedSpellScenarioResult,
    ExtendedSpellState,
};

pub const BRANCH_ACTIONS: [&str; 1] = ["doResolveExtendedCreatureSizeIncrease"];

pub fn replay_observed_action(observed_action_taken: &str) -> ExtendedSpellState {
    let (state, _) = replay_reducer_route(observed_action_taken);
    state
}

pub fn replay_observed_route(observed_action_taken: &str) -> Vec<ReducerRouteEvent> {
    let (_, route) = replay_reducer_route(observed_action_taken);
    route
}

pub fn expected_witness(observed_action_taken: &str) -> ExtendedSpellState {
    match observed_action_taken {
        "doResolveExtendedCreatureSizeIncrease" => ExtendedSpellState {
            sorcery_points_remaining: 1,
            duration_ticks: 20,
            concentration_saving_throw_mode: ExtendedSpellConcentrationSaveMode::Advantage,
            scenario_result: ExtendedSpellScenarioResult::ExtendedCreatureSizeIncrease,
            protocol: ExtendedSpellProtocol::Resolved,
        },
        action => panic!("unsupported mbt::actionTaken {action}"),
    }
}

pub fn expected_route(observed_action_taken: &str) -> Vec<ReducerRouteEvent> {
    match observed_action_taken {
        "doResolveExtendedCreatureSizeIncrease" => {
            metamagic_success_route(ReducerRouteOwnerGroup::Concentration)
        }
        action => panic!("unsupported mbt::actionTaken {action}"),
    }
}

pub fn projection_payload(state: &ExtendedSpellState) -> String {
    [
        format!("sorceryPointsRemaining={}", state.sorcery_points_remaining),
        format!("durationTicks={}", state.duration_ticks),
        format!(
            "concentrationSavingThrowMode={}",
            concentration_mode_ref(state.concentration_saving_throw_mode)
        ),
        format!("scenarioResult={}", scenario_ref(state.scenario_result)),
        format!("protocolResult={}", protocol_ref(state.protocol)),
        "protocolHoles=none".to_string(),
    ]
    .join("\n")
}

fn concentration_mode_ref(mode: ExtendedSpellConcentrationSaveMode) -> &'static str {
    match mode {
        ExtendedSpellConcentrationSaveMode::Normal => "normal",
        ExtendedSpellConcentrationSaveMode::Advantage => "advantage",
    }
}

fn scenario_ref(result: ExtendedSpellScenarioResult) -> &'static str {
    match result {
        ExtendedSpellScenarioResult::Init => "init",
        ExtendedSpellScenarioResult::ExtendedCreatureSizeIncrease => "extendedCreatureSizeIncrease",
    }
}

fn protocol_ref(protocol: ExtendedSpellProtocol) -> &'static str {
    match protocol {
        ExtendedSpellProtocol::Init => "init",
        ExtendedSpellProtocol::Resolved => "resolved",
    }
}

fn replay_reducer_route(
    observed_action_taken: &str,
) -> (ExtendedSpellState, Vec<ReducerRouteEvent>) {
    let mut trace = BattleEntrypointTrace::default();
    let setup = battle_setup_from_state(start_metamagic_option_spell_battle(2));
    let state = start_battle_observed(setup, &mut trace).state;
    let discovery = discover_battle_acts_observed(&state, &mut trace);
    let subject = discovery
        .available_acts()
        .iter()
        .find(|act| act.subject.kind == BattleSubjectKind::MetamagicOptionSpell)
        .map(|act| act.subject)
        .unwrap_or_else(|| {
            panic!("Extended metamagic subject not discovered for {observed_action_taken}")
        });
    let request = BattleResolutionRequest::metamagic_option_spell(
        subject,
        BattleMetamagicOptionSpellFill {
            option_facts: BattleMetamagicOptionFacts::spell_duration_extension(),
            effect: effect_for_action(observed_action_taken),
            options_already_applied_to_spell: 0,
            selected_second_option_supported: true,
            spell_uses_level_one_plus_slot: true,
        },
    )
    .expect("adapter selects the metamagic option spell subject");
    let result = resolve_battle_subject_observed(state, request, &mut trace);
    (
        extended_spell_from_battle(result.state()),
        observed_reducer_route(&trace, &[ReducerRouteSubjectFamily::MetamagicOptionSpell]),
    )
}

fn effect_for_action(observed_action_taken: &str) -> BattleMetamagicOptionSpellEffect {
    match observed_action_taken {
        "doResolveExtendedCreatureSizeIncrease" => {
            BattleMetamagicOptionSpellEffect::DurationExtension {
                duration_ticks: 20,
                concentration_saving_throw_mode: ExtendedSpellConcentrationSaveMode::Advantage,
            }
        }
        action => panic!("unsupported mbt::actionTaken {action}"),
    }
}

fn metamagic_success_route(owner: ReducerRouteOwnerGroup) -> Vec<ReducerRouteEvent> {
    vec![
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
            owner,
        ),
    ]
}
