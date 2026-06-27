use super::battle_runtime_reducer_route::{
    observed_reducer_route, route_discover_battle_acts, route_resolve_battle_subject,
    route_start_battle, setup_from_battle_state, ReducerRouteEvent, ReducerRouteFillKind,
    ReducerRouteOwnerGroup, ReducerRouteSubjectFamily,
};
use crate::rules::battle_reducer_spine::{
    careful_spell_from_battle, discover_battle_acts_observed, resolve_battle_subject_observed,
    start_battle_observed, start_metamagic_option_spell_battle, BattleEntrypointTrace,
    BattleMetamagicOptionFacts, BattleMetamagicOptionSpellEffect, BattleMetamagicOptionSpellFill,
    BattleResolutionRequest, BattleSubjectKind,
};
use crate::rules::sorcerer_metamagic::{
    CarefulSpellProtocol, CarefulSpellScenarioResult, CarefulSpellState,
};

pub const BRANCH_ACTIONS: [&str; 2] = [
    "doResolveCarefulSaveGatedDamage",
    "doResolveCarefulSaveGatedNoEffect",
];

pub fn replay_observed_action(observed_action_taken: &str) -> CarefulSpellState {
    let (state, _) = replay_reducer_route(observed_action_taken);
    state
}

pub fn replay_observed_route(observed_action_taken: &str) -> Vec<ReducerRouteEvent> {
    let (_, route) = replay_reducer_route(observed_action_taken);
    route
}

pub fn expected_witness(observed_action_taken: &str) -> CarefulSpellState {
    match observed_action_taken {
        "doResolveCarefulSaveGatedDamage" => CarefulSpellState {
            magic_action_available: false,
            bonus_action_available: true,
            sorcery_points_remaining: 3,
            target_hit_points: 10,
            target_active_effect_count: 0,
            scenario_result: CarefulSpellScenarioResult::CarefulSaveGatedDamage,
            protocol: CarefulSpellProtocol::Resolved,
        },
        "doResolveCarefulSaveGatedNoEffect" => CarefulSpellState {
            magic_action_available: false,
            bonus_action_available: true,
            sorcery_points_remaining: 3,
            target_hit_points: 10,
            target_active_effect_count: 0,
            scenario_result: CarefulSpellScenarioResult::CarefulSaveGatedNoEffect,
            protocol: CarefulSpellProtocol::Resolved,
        },
        action => panic!("unsupported mbt::actionTaken {action}"),
    }
}

pub fn expected_route(observed_action_taken: &str) -> Vec<ReducerRouteEvent> {
    let owner = match observed_action_taken {
        "doResolveCarefulSaveGatedDamage" => ReducerRouteOwnerGroup::DamageAdjustment,
        "doResolveCarefulSaveGatedNoEffect" => ReducerRouteOwnerGroup::SavingThrowOutcome,
        action => panic!("unsupported mbt::actionTaken {action}"),
    };
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

pub fn projection_payload(state: &CarefulSpellState) -> String {
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

fn replay_reducer_route(
    observed_action_taken: &str,
) -> (CarefulSpellState, Vec<ReducerRouteEvent>) {
    let mut trace = BattleEntrypointTrace::default();
    let state = start_battle_observed(
        setup_from_battle_state(start_metamagic_option_spell_battle(4)),
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
        careful_spell_from_battle(result.state()),
        observed_reducer_route(&trace, &[ReducerRouteSubjectFamily::MetamagicOptionSpell]),
    )
}

const fn metamagic_option_facts(sorcery_point_cost: i16) -> BattleMetamagicOptionFacts {
    BattleMetamagicOptionFacts {
        selected_option_admitted: true,
        sorcery_point_cost,
        changes_action_casting_time_to_bonus_action: false,
        permits_multiple_options_for_spell: false,
    }
}

fn effect_for_action(observed_action_taken: &str) -> BattleMetamagicOptionSpellEffect {
    match observed_action_taken {
        "doResolveCarefulSaveGatedDamage" => {
            BattleMetamagicOptionSpellEffect::ProtectedSaveGatedDamage {
                target_hit_points_after: 10,
                protected_save_targets: 1,
            }
        }
        "doResolveCarefulSaveGatedNoEffect" => {
            BattleMetamagicOptionSpellEffect::ProtectedSaveGatedNoEffect {
                protected_save_targets: 1,
            }
        }
        action => panic!("unsupported mbt::actionTaken {action}"),
    }
}

fn scenario_ref(result: CarefulSpellScenarioResult) -> &'static str {
    match result {
        CarefulSpellScenarioResult::Init => "init",
        CarefulSpellScenarioResult::CarefulSaveGatedDamage => "carefulSaveGatedDamage",
        CarefulSpellScenarioResult::CarefulSaveGatedNoEffect => "carefulSaveGatedNoEffect",
    }
}

fn protocol_ref(protocol: CarefulSpellProtocol) -> &'static str {
    match protocol {
        CarefulSpellProtocol::Init => "init",
        CarefulSpellProtocol::Resolved => "resolved",
    }
}
