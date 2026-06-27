use super::battle_runtime_reducer_route::{
    observed_reducer_route, route_discover_battle_acts, route_resolve_battle_subject,
    route_start_battle, setup_from_battle_state, ReducerRouteEvent, ReducerRouteFillKind,
    ReducerRouteOwnerGroup, ReducerRouteSubjectFamily,
};
use crate::rules::battle_reducer_spine::{
    discover_battle_acts_observed, heightened_spell_from_battle, resolve_battle_subject_observed,
    start_battle_observed, start_metamagic_option_spell_battle, BattleEntrypointTrace,
    BattleMetamagicOptionFacts, BattleMetamagicOptionSpellEffect, BattleMetamagicOptionSpellFill,
    BattleResolutionRequest, BattleSubjectKind,
};
use crate::rules::sorcerer_metamagic::{
    HeightenedSpellProtocol, HeightenedSpellScenarioResult, HeightenedSpellState,
};

pub const BRANCH_ACTIONS: [&str; 5] = [
    "doResolveHeightenedSaveGatedDamage",
    "doResolveHeightenedHideousLaughter",
    "doResolveHeightenedGreaseEntrySave",
    "doResolveHeightenedGustOfWindEndTurnSave",
    "doResolveHeightenedSaveGatedConditionEndTurnSave",
];

pub fn replay_observed_action(observed_action_taken: &str) -> HeightenedSpellState {
    let (state, _) = replay_reducer_route(observed_action_taken);
    state
}

pub fn replay_observed_route(observed_action_taken: &str) -> Vec<ReducerRouteEvent> {
    let (_, route) = replay_reducer_route(observed_action_taken);
    route
}

pub fn expected_witness(observed_action_taken: &str) -> HeightenedSpellState {
    match observed_action_taken {
        "doResolveHeightenedSaveGatedDamage" => expected_resolved(
            false,
            1,
            0,
            HeightenedSpellScenarioResult::HeightenedSaveGatedDamage,
        ),
        "doResolveHeightenedHideousLaughter" => expected_resolved(
            false,
            10,
            1,
            HeightenedSpellScenarioResult::HeightenedHideousLaughter,
        ),
        "doResolveHeightenedGreaseEntrySave" => expected_resolved(
            true,
            10,
            0,
            HeightenedSpellScenarioResult::HeightenedGreaseEntrySave,
        ),
        "doResolveHeightenedGustOfWindEndTurnSave" => expected_resolved(
            true,
            10,
            0,
            HeightenedSpellScenarioResult::HeightenedGustOfWindEndTurnSave,
        ),
        "doResolveHeightenedSaveGatedConditionEndTurnSave" => expected_resolved(
            true,
            10,
            0,
            HeightenedSpellScenarioResult::HeightenedSaveGatedConditionEndTurnSave,
        ),
        action => panic!("unsupported mbt::actionTaken {action}"),
    }
}

pub fn expected_route(observed_action_taken: &str) -> Vec<ReducerRouteEvent> {
    let owner = match observed_action_taken {
        "doResolveHeightenedHideousLaughter" => ReducerRouteOwnerGroup::ConditionLifecycle,
        "doResolveHeightenedSaveGatedDamage"
        | "doResolveHeightenedGreaseEntrySave"
        | "doResolveHeightenedGustOfWindEndTurnSave"
        | "doResolveHeightenedSaveGatedConditionEndTurnSave" => {
            ReducerRouteOwnerGroup::SavingThrowRollMode
        }
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

pub fn projection_payload(state: &HeightenedSpellState) -> String {
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
) -> (HeightenedSpellState, Vec<ReducerRouteEvent>) {
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
            option_facts: metamagic_option_facts(2),
            effect: effect_for_action(observed_action_taken),
            options_already_applied_to_spell: 0,
            selected_second_option_supported: true,
            spell_uses_level_one_plus_slot: false,
            spell_consumes_magic_action: spell_consumes_magic_action(observed_action_taken),
        },
    )
    .expect("adapter selects the metamagic option spell subject");
    let result = resolve_battle_subject_observed(state, request, &mut trace);
    (
        heightened_spell_from_battle(result.state()),
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
        "doResolveHeightenedSaveGatedDamage" => {
            BattleMetamagicOptionSpellEffect::FirstTargetDisadvantageSaveGatedDamage {
                target_hit_points_after: 1,
            }
        }
        "doResolveHeightenedHideousLaughter" => {
            BattleMetamagicOptionSpellEffect::FirstTargetDisadvantageCondition {
                target_active_effect_count: 1,
            }
        }
        "doResolveHeightenedGreaseEntrySave" => {
            BattleMetamagicOptionSpellEffect::FirstTargetDisadvantageEntrySave
        }
        "doResolveHeightenedGustOfWindEndTurnSave" => {
            BattleMetamagicOptionSpellEffect::FirstTargetDisadvantageEndTurnSave
        }
        "doResolveHeightenedSaveGatedConditionEndTurnSave" => {
            BattleMetamagicOptionSpellEffect::FirstTargetDisadvantageConditionEndTurnSave
        }
        action => panic!("unsupported mbt::actionTaken {action}"),
    }
}

fn spell_consumes_magic_action(observed_action_taken: &str) -> bool {
    match observed_action_taken {
        "doResolveHeightenedSaveGatedDamage" | "doResolveHeightenedHideousLaughter" => true,
        "doResolveHeightenedGreaseEntrySave"
        | "doResolveHeightenedGustOfWindEndTurnSave"
        | "doResolveHeightenedSaveGatedConditionEndTurnSave" => false,
        action => panic!("unsupported mbt::actionTaken {action}"),
    }
}

const fn expected_resolved(
    magic_action_available: bool,
    target_hit_points: i16,
    target_active_effect_count: i16,
    scenario_result: HeightenedSpellScenarioResult,
) -> HeightenedSpellState {
    HeightenedSpellState {
        magic_action_available,
        bonus_action_available: true,
        sorcery_points_remaining: 2,
        target_hit_points,
        target_active_effect_count,
        scenario_result,
        protocol: HeightenedSpellProtocol::Resolved,
    }
}

fn scenario_ref(result: HeightenedSpellScenarioResult) -> &'static str {
    match result {
        HeightenedSpellScenarioResult::Init => "init",
        HeightenedSpellScenarioResult::HeightenedSaveGatedDamage => "heightenedSaveGatedDamage",
        HeightenedSpellScenarioResult::HeightenedHideousLaughter => "heightenedHideousLaughter",
        HeightenedSpellScenarioResult::HeightenedGreaseEntrySave => "heightenedGreaseEntrySave",
        HeightenedSpellScenarioResult::HeightenedGustOfWindEndTurnSave => {
            "heightenedGustOfWindEndTurnSave"
        }
        HeightenedSpellScenarioResult::HeightenedSaveGatedConditionEndTurnSave => {
            "heightenedSaveGatedConditionEndTurnSave"
        }
    }
}

fn protocol_ref(protocol: HeightenedSpellProtocol) -> &'static str {
    match protocol {
        HeightenedSpellProtocol::Init => "init",
        HeightenedSpellProtocol::Resolved => "resolved",
    }
}
