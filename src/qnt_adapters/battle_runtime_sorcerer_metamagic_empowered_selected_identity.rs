use super::battle_runtime_reducer_route::{
    battle_setup_from_state, observed_reducer_route, route_discover_battle_acts,
    route_resolve_battle_subject, route_start_battle, ReducerRouteEvent, ReducerRouteFillKind,
    ReducerRouteOwnerGroup, ReducerRouteSubjectFamily,
};
use crate::rules::battle_reducer_spine::{
    discover_battle_acts_observed, empowered_spell_from_battle, resolve_battle_subject_observed,
    start_battle_observed, start_metamagic_option_spell_battle, BattleEntrypointTrace,
    BattleMetamagicOptionFacts, BattleMetamagicOptionSpellEffect, BattleMetamagicOptionSpellFill,
    BattleResolutionRequest, BattleSubjectKind,
};
use crate::rules::sorcerer_metamagic::{
    EmpoweredSpellProtocol, EmpoweredSpellScenarioResult, EmpoweredSpellState,
};

pub const BRANCH_ACTIONS: [&str; 1] = ["doResolveEmpoweredSpellDamageReroll"];

pub fn replay_observed_action(observed_action_taken: &str) -> EmpoweredSpellState {
    let (state, _) = replay_reducer_route(observed_action_taken);
    state
}

pub fn replay_observed_route(observed_action_taken: &str) -> Vec<ReducerRouteEvent> {
    let (_, route) = replay_reducer_route(observed_action_taken);
    route
}

pub fn expected_witness(observed_action_taken: &str) -> EmpoweredSpellState {
    match observed_action_taken {
        "doResolveEmpoweredSpellDamageReroll" => EmpoweredSpellState {
            magic_action_available: false,
            bonus_action_available: true,
            sorcery_points_remaining: 3,
            target_hit_points: 1,
            target_active_effect_count: 1,
            scenario_result: EmpoweredSpellScenarioResult::EmpoweredSpellDamageReroll,
            protocol: EmpoweredSpellProtocol::Resolved,
        },
        action => panic!("unsupported mbt::actionTaken {action}"),
    }
}

pub fn expected_route(observed_action_taken: &str) -> Vec<ReducerRouteEvent> {
    match observed_action_taken {
        "doResolveEmpoweredSpellDamageReroll" => {
            metamagic_success_route(ReducerRouteOwnerGroup::DamageRoll)
        }
        action => panic!("unsupported mbt::actionTaken {action}"),
    }
}

pub fn projection_payload(state: &EmpoweredSpellState) -> String {
    [
        format!("magicActionAvailable={}", state.magic_action_available),
        format!("bonusActionAvailable={}", state.bonus_action_available),
        format!("sorceryPointsRemaining={}", state.sorcery_points_remaining),
        format!("targetHp={}", state.target_hit_points),
        format!(
            "targetActiveEffectCount={}",
            state.target_active_effect_count
        ),
        format!("scenarioResult={}", scenario_ref(state.scenario_result)),
        format!("protocolResult={}", protocol_ref(state.protocol)),
        "protocolHoles=none".to_string(),
    ]
    .join("\n")
}

fn scenario_ref(result: EmpoweredSpellScenarioResult) -> &'static str {
    match result {
        EmpoweredSpellScenarioResult::Init => "init",
        EmpoweredSpellScenarioResult::EmpoweredSpellDamageReroll => "empoweredSpellDamageReroll",
    }
}

fn protocol_ref(protocol: EmpoweredSpellProtocol) -> &'static str {
    match protocol {
        EmpoweredSpellProtocol::Init => "init",
        EmpoweredSpellProtocol::Resolved => "resolved",
    }
}

fn replay_reducer_route(
    observed_action_taken: &str,
) -> (EmpoweredSpellState, Vec<ReducerRouteEvent>) {
    let mut trace = BattleEntrypointTrace::default();
    let setup = battle_setup_from_state(start_metamagic_option_spell_battle(4));
    let state = start_battle_observed(setup, &mut trace).state;
    let discovery = discover_battle_acts_observed(&state, &mut trace);
    let subject = discovery
        .available_acts()
        .iter()
        .find(|act| act.subject.kind == BattleSubjectKind::MetamagicOptionSpell)
        .map(|act| act.subject)
        .unwrap_or_else(|| {
            panic!("Empowered metamagic subject not discovered for {observed_action_taken}")
        });
    let request = BattleResolutionRequest::metamagic_option_spell(
        subject,
        BattleMetamagicOptionSpellFill {
            option_facts: BattleMetamagicOptionFacts::spell_damage_dice_reroll(),
            effect: effect_for_action(observed_action_taken),
            options_already_applied_to_spell: 0,
            selected_second_option_supported: true,
            spell_uses_level_one_plus_slot: true,
        },
    )
    .expect("adapter selects the metamagic option spell subject");
    let result = resolve_battle_subject_observed(state, request, &mut trace);
    (
        empowered_spell_from_battle(result.state()),
        observed_reducer_route(&trace, &[ReducerRouteSubjectFamily::MetamagicOptionSpell]),
    )
}

fn effect_for_action(observed_action_taken: &str) -> BattleMetamagicOptionSpellEffect {
    match observed_action_taken {
        "doResolveEmpoweredSpellDamageReroll" => BattleMetamagicOptionSpellEffect::DamageReroll {
            target_hit_points_after: 1,
            target_active_effect_count: 1,
        },
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
