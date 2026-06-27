use super::battle_runtime_reducer_route::{
    battle_setup_from_state, observed_reducer_route, route_discover_battle_acts,
    route_resolve_battle_subject, route_start_battle, ReducerRouteEvent, ReducerRouteFillKind,
    ReducerRouteOwnerGroup, ReducerRouteSubjectFamily,
};
use crate::rules::battle_reducer_spine::{
    discover_battle_acts_observed, resolve_battle_subject_observed, seeking_spell_from_battle,
    start_battle_observed, start_metamagic_option_spell_battle, BattleEntrypointTrace,
    BattleMetamagicOptionFacts, BattleMetamagicOptionSpellEffect, BattleMetamagicOptionSpellFill,
    BattleResolutionRequest, BattleSubjectKind,
};
use crate::rules::sorcerer_metamagic::{
    SeekingSpellProtocol, SeekingSpellScenarioResult, SeekingSpellState,
};

pub const BRANCH_ACTIONS: [&str; 1] = ["doResolveSeekingSpellAttackReroll"];

pub fn replay_observed_action(observed_action_taken: &str) -> SeekingSpellState {
    let (state, _) = replay_reducer_route(observed_action_taken);
    state
}

pub fn replay_observed_route(observed_action_taken: &str) -> Vec<ReducerRouteEvent> {
    let (_, route) = replay_reducer_route(observed_action_taken);
    route
}

pub fn expected_witness(observed_action_taken: &str) -> SeekingSpellState {
    match observed_action_taken {
        "doResolveSeekingSpellAttackReroll" => SeekingSpellState {
            magic_action_available: false,
            bonus_action_available: true,
            sorcery_points_remaining: 3,
            target_hit_points: 3,
            target_active_effect_count: 1,
            scenario_result: SeekingSpellScenarioResult::SeekingSpellAttackReroll,
            protocol: SeekingSpellProtocol::Resolved,
        },
        action => panic!("unsupported mbt::actionTaken {action}"),
    }
}

pub fn expected_route(observed_action_taken: &str) -> Vec<ReducerRouteEvent> {
    match observed_action_taken {
        "doResolveSeekingSpellAttackReroll" => {
            metamagic_success_route(ReducerRouteOwnerGroup::AttackRoll)
        }
        action => panic!("unsupported mbt::actionTaken {action}"),
    }
}

pub fn projection_payload(state: &SeekingSpellState) -> String {
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

fn scenario_ref(result: SeekingSpellScenarioResult) -> &'static str {
    match result {
        SeekingSpellScenarioResult::Init => "init",
        SeekingSpellScenarioResult::SeekingSpellAttackReroll => "seekingSpellAttackReroll",
    }
}

fn protocol_ref(protocol: SeekingSpellProtocol) -> &'static str {
    match protocol {
        SeekingSpellProtocol::Init => "init",
        SeekingSpellProtocol::Resolved => "resolved",
    }
}

fn replay_reducer_route(
    observed_action_taken: &str,
) -> (SeekingSpellState, Vec<ReducerRouteEvent>) {
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
            panic!("Seeking metamagic subject not discovered for {observed_action_taken}")
        });
    let request = BattleResolutionRequest::metamagic_option_spell(
        subject,
        BattleMetamagicOptionSpellFill {
            option_facts: BattleMetamagicOptionFacts::missed_spell_attack_d20_reroll(),
            effect: effect_for_action(observed_action_taken),
            options_already_applied_to_spell: 0,
            selected_second_option_supported: true,
            spell_uses_level_one_plus_slot: true,
            spell_consumes_magic_action: true,
        },
    )
    .expect("adapter selects the metamagic option spell subject");
    let result = resolve_battle_subject_observed(state, request, &mut trace);
    (
        seeking_spell_from_battle(result.state()),
        observed_reducer_route(&trace, &[ReducerRouteSubjectFamily::MetamagicOptionSpell]),
    )
}

fn effect_for_action(observed_action_taken: &str) -> BattleMetamagicOptionSpellEffect {
    match observed_action_taken {
        "doResolveSeekingSpellAttackReroll" => {
            BattleMetamagicOptionSpellEffect::SpellAttackReroll {
                target_hit_points_after: 3,
                target_active_effect_count: 1,
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
