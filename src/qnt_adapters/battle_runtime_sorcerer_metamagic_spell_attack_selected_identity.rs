use super::battle_runtime_reducer_route::{
    observed_reducer_route, route_discover_battle_acts, route_resolve_battle_subject,
    route_start_battle, ReducerRouteEvent, ReducerRouteFillKind, ReducerRouteOwnerGroup,
    ReducerRouteSubjectFamily,
};
use crate::rules::battle_reducer_spine::{
    discover_battle_acts_observed, quickened_metamagic_from_battle,
    resolve_battle_subject_observed, start_battle_observed, start_quickened_metamagic_battle,
    BattleEntrypointTrace, BattleMetamagicOptionFacts, BattleMetamagicOptionSpellEffect,
    BattleMetamagicOptionSpellFill, BattleResolutionRequest, BattleSetup, BattleSubjectKind,
};
use crate::rules::sorcerer_metamagic::{
    QuickenedMetamagicProtocol, QuickenedMetamagicScenarioResult, QuickenedMetamagicState,
};

pub const BRANCH_ACTIONS: [&str; 1] = ["doResolveQuickenedSpellAttack"];

pub fn replay_observed_action(observed_action_taken: &str) -> QuickenedMetamagicState {
    let (state, _) = replay_reducer_route(observed_action_taken);
    state
}

pub fn replay_observed_route(observed_action_taken: &str) -> Vec<ReducerRouteEvent> {
    let (_, route) = replay_reducer_route(observed_action_taken);
    route
}

pub fn expected_witness(observed_action_taken: &str) -> QuickenedMetamagicState {
    match observed_action_taken {
        "doResolveQuickenedSpellAttack" => QuickenedMetamagicState {
            magic_action_available: true,
            bonus_action_available: false,
            sorcery_points_remaining: 2,
            target_hit_points: 3,
            target_active_effect_count: 1,
            scenario_result: QuickenedMetamagicScenarioResult::QuickenedSpellAttack,
            protocol: QuickenedMetamagicProtocol::Resolved,
        },
        action => panic!("unsupported mbt::actionTaken {action}"),
    }
}

pub fn expected_route(observed_action_taken: &str) -> Vec<ReducerRouteEvent> {
    match observed_action_taken {
        "doResolveQuickenedSpellAttack" => quickened_success_route(),
        action => panic!("unsupported mbt::actionTaken {action}"),
    }
}

pub fn projection_payload(state: &QuickenedMetamagicState) -> String {
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
) -> (QuickenedMetamagicState, Vec<ReducerRouteEvent>) {
    let mut trace = BattleEntrypointTrace::default();
    let state = start_battle_observed(
        setup_from_state(start_quickened_metamagic_battle()),
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
            panic!("Quickened metamagic subject not discovered for {observed_action_taken}")
        });
    let request = BattleResolutionRequest::metamagic_option_spell(
        subject,
        BattleMetamagicOptionSpellFill {
            option_facts: BattleMetamagicOptionFacts::quickened_action_to_bonus_action(),
            effect: effect_for_action(observed_action_taken),
            options_already_applied_to_spell: 0,
            selected_second_option_supported: true,
            spell_uses_level_one_plus_slot: true,
            spell_consumes_magic_action: false,
        },
    )
    .expect("adapter selects the metamagic option spell subject");
    let result = resolve_battle_subject_observed(state, request, &mut trace);
    (
        quickened_metamagic_from_battle(result.state()),
        observed_reducer_route(&trace, &[ReducerRouteSubjectFamily::MetamagicOptionSpell]),
    )
}

fn effect_for_action(observed_action_taken: &str) -> BattleMetamagicOptionSpellEffect {
    match observed_action_taken {
        "doResolveQuickenedSpellAttack" => BattleMetamagicOptionSpellEffect::SpellAttack {
            target_hit_points_after: 3,
            target_active_effect_count: 1,
        },
        action => panic!("unsupported mbt::actionTaken {action}"),
    }
}

fn quickened_success_route() -> Vec<ReducerRouteEvent> {
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
            ReducerRouteOwnerGroup::SpellSlotAndActionEconomy,
        ),
    ]
}

fn setup_from_state(state: crate::rules::battle_reducer_spine::BattleState) -> BattleSetup {
    BattleSetup {
        initiative: state.initiative,
        fighter: state.fighter,
        goblin: state.goblin,
        rogue: state.rogue,
        skeleton: state.skeleton,
        stat_block_control: state.stat_block_control,
        turn_boundary_effects: state.turn_boundary_effects,
        interrupt_resume: state.interrupt_resume,
        reaction_casting_time: state.reaction_casting_time,
        concentration: state.concentration,
        slot_spell_procedure: state.slot_spell_procedure,
        save_gated_spell_procedure: state.save_gated_spell_procedure,
        hit_point_restoration_procedure: state.hit_point_restoration_procedure,
        spell_attack_procedure: state.spell_attack_procedure,
        command_effect_procedure: state.command_effect_procedure,
        spatial_route_subjects: state.spatial_route_subjects,
        object_light_route: state.object_light_route,
        ability_check_choice_search: state.ability_check_choice_search,
        feature_substrates: state.feature_substrates,
        feature_resources: state.feature_resources,
        spell_slot_uses_this_turn: state.spell_slot_uses_this_turn,
        level_one_plus_spell_casters_this_turn: state.level_one_plus_spell_casters_this_turn,
        quickened_level_one_plus_spell_casters_this_turn: state
            .quickened_level_one_plus_spell_casters_this_turn,
        action_available: state.action_available,
        bonus_action_available: state.bonus_action_available,
        attack_roll_made_this_turn: state.attack_roll_made_this_turn,
        dash_movement_bonus_feet: state.dash_movement_bonus_feet,
        disengaged: state.disengaged,
    }
}

fn scenario_ref(result: QuickenedMetamagicScenarioResult) -> &'static str {
    match result {
        QuickenedMetamagicScenarioResult::Init => "init",
        QuickenedMetamagicScenarioResult::QuickenedSaveGatedDamage => "quickenedSaveGatedDamage",
        QuickenedMetamagicScenarioResult::QuickenedSpellAttack => "quickenedSpellAttack",
        QuickenedMetamagicScenarioResult::QuickenedSpellAttackSequence => {
            "quickenedSpellAttackSequence"
        }
    }
}

fn protocol_ref(protocol: QuickenedMetamagicProtocol) -> &'static str {
    match protocol {
        QuickenedMetamagicProtocol::Init => "init",
        QuickenedMetamagicProtocol::Resolved => "resolved",
    }
}
