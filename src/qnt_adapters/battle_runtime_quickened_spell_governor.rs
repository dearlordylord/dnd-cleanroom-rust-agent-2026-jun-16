use super::battle_runtime_reducer_route::{
    observed_reducer_route, route_discover_battle_acts, route_resolve_battle_subject,
    route_resolve_battle_subject_from_route_result, route_start_battle, ReducerRouteEvent,
    ReducerRouteFillKind, ReducerRouteOwnerGroup, ReducerRouteResolutionOutcome,
    ReducerRouteSubjectFamily,
};
use crate::rules::battle_reducer_spine::{
    discover_battle_acts_observed, quickened_spell_governor_from_battle,
    resolve_battle_subject_observed, start_battle_observed, start_quickened_spell_governor_battle,
    with_battle_sorcery_points, Actor, BattleEntrypointTrace, BattleMetamagicOptionFacts,
    BattleMetamagicOptionSpellEffect, BattleMetamagicOptionSpellFill,
    BattleResolutionInvalidReason, BattleResolutionRequest, BattleSetup, BattleSubjectKind,
};
use crate::rules::quickened_spell_governor::{
    QuickenedSpellGovernorState, QuickenedSpellInvalidKind, QuickenedSpellProtocol,
    QuickenedSpellScenarioOutcome,
};

pub const BRANCH_ACTIONS: [&str; 11] = [
    "doResolveQuickenedRestoration",
    "doResolveQuickenedSaveGatedCondition",
    "doResolveQuickenedSaveGatedConditionImmunity",
    "doResolveQuickenedDirectCondition",
    "doResolveQuickenedRollModifier",
    "doResolveQuickenedAfterMagicActionSpent",
    "doRejectUnaffordable",
    "doRejectUnknownOption",
    "doRejectUnsupportedSecondOption",
    "doRejectOnePerSpell",
    "doRejectPriorLevelOnePlusSpell",
];

pub fn replay_observed_action(observed_action_taken: &str) -> QuickenedSpellGovernorState {
    let (state, _) = replay_reducer_route(observed_action_taken);
    state
}

pub fn replay_observed_route(observed_action_taken: &str) -> Vec<ReducerRouteEvent> {
    let (_, route) = replay_reducer_route(observed_action_taken);
    route
}

pub fn expected_witness(observed_action_taken: &str) -> QuickenedSpellGovernorState {
    let scenario = scenario_for_action(observed_action_taken);
    QuickenedSpellGovernorState {
        quickened_cure_wounds_offered: matches!(
            scenario,
            QuickenedSpellScenarioOutcome::Init
                | QuickenedSpellScenarioOutcome::RejectedUnknownOption
                | QuickenedSpellScenarioOutcome::RejectedUnsupportedSecondOption
                | QuickenedSpellScenarioOutcome::RejectedOnePerSpell
        ),
        color_spray_blinded: scenario
            == QuickenedSpellScenarioOutcome::ResolvedQuickenedSaveGatedCondition,
        calm_emotions_immunity: scenario
            == QuickenedSpellScenarioOutcome::ResolvedQuickenedSaveGatedConditionImmunity,
        invisibility_active: scenario
            == QuickenedSpellScenarioOutcome::ResolvedQuickenedDirectCondition,
        bless_active: scenario == QuickenedSpellScenarioOutcome::ResolvedQuickenedRollModifier,
        magic_action_available: scenario
            != QuickenedSpellScenarioOutcome::ResolvedAfterMagicActionSpent,
        bonus_action_available: matches!(
            scenario,
            QuickenedSpellScenarioOutcome::RejectedUnaffordable
                | QuickenedSpellScenarioOutcome::RejectedUnknownOption
                | QuickenedSpellScenarioOutcome::RejectedUnsupportedSecondOption
                | QuickenedSpellScenarioOutcome::RejectedOnePerSpell
                | QuickenedSpellScenarioOutcome::RejectedPriorLevelOnePlusSpell
        ),
        sorcery_points_remaining: expected_sorcery_points_remaining(scenario),
        target_hit_points: expected_target_hit_points(scenario),
        spell_slot_committed: quickened_success(scenario),
        level_one_plus_cast_this_turn: quickened_success(scenario)
            || scenario == QuickenedSpellScenarioOutcome::RejectedPriorLevelOnePlusSpell,
        quickened_level_one_plus_cast_this_turn: quickened_success(scenario),
        spell_slot_acts_available: !quickened_success(scenario),
        invalid_kind: invalid_kind_for_scenario(scenario),
        scenario_outcome: scenario,
        protocol: QuickenedSpellProtocol::Resolved,
    }
}

pub fn expected_route(observed_action_taken: &str) -> Vec<ReducerRouteEvent> {
    let scenario = scenario_for_action(observed_action_taken);
    vec![
        route_start_battle(ReducerRouteOwnerGroup::ActionEconomy),
        route_discover_battle_acts(
            ReducerRouteSubjectFamily::MetamagicOptionSpell,
            Vec::new(),
            ReducerRouteOwnerGroup::FeatureResource,
        ),
        quickened_expected_resolution_event(scenario),
    ]
}

fn quickened_expected_resolution_event(
    scenario: QuickenedSpellScenarioOutcome,
) -> ReducerRouteEvent {
    if quickened_success(scenario) {
        return route_resolve_battle_subject(
            ReducerRouteSubjectFamily::MetamagicOptionSpell,
            ReducerRouteFillKind::UnitFeatureDecision,
            Vec::new(),
            ReducerRouteOwnerGroup::SpellSlotAndActionEconomy,
        );
    }
    route_resolve_battle_subject_from_route_result(
        ReducerRouteSubjectFamily::MetamagicOptionSpell,
        ReducerRouteFillKind::UnitFeatureDecision,
        ReducerRouteResolutionOutcome::Invalid(BattleResolutionInvalidReason::InvalidFill),
        Vec::new(),
        ReducerRouteOwnerGroup::FeatureResource,
    )
}

pub fn projection_payload(state: &QuickenedSpellGovernorState) -> String {
    [
        format!(
            "qQuickenedCureWoundsOffered={}",
            state.quickened_cure_wounds_offered
        ),
        format!("qColorSprayBlinded={}", state.color_spray_blinded),
        format!("qCalmEmotionsImmunity={}", state.calm_emotions_immunity),
        format!("qInvisibilityActive={}", state.invisibility_active),
        format!("qBlessActive={}", state.bless_active),
        format!("qMagicActionAvailable={}", state.magic_action_available),
        format!("qBonusActionAvailable={}", state.bonus_action_available),
        format!("qSorceryPointsRemaining={}", state.sorcery_points_remaining),
        format!("qTargetHp={}", state.target_hit_points),
        format!("qSpellSlotCommitted={}", state.spell_slot_committed),
        format!(
            "qLevelOnePlusCastThisTurn={}",
            state.level_one_plus_cast_this_turn
        ),
        format!(
            "qQuickenedLevelOnePlusCastThisTurn={}",
            state.quickened_level_one_plus_cast_this_turn
        ),
        format!(
            "qSpellSlotActsAvailable={}",
            state.spell_slot_acts_available
        ),
        format!("qInvalidKind={}", invalid_kind_ref(state.invalid_kind)),
        format!(
            "qScenarioOutcome={}",
            scenario_outcome_ref(state.scenario_outcome)
        ),
        format!("protocolResult={}", protocol_ref(state.protocol)),
        "protocolHoles=none".to_string(),
    ]
    .join("\n")
}

fn replay_reducer_route(
    observed_action_taken: &str,
) -> (QuickenedSpellGovernorState, Vec<ReducerRouteEvent>) {
    let mut trace = BattleEntrypointTrace::default();
    let setup = quickened_setup_for_action(observed_action_taken);
    let mut state = start_battle_observed(setup, &mut trace).state;
    state = prepare_state_for_action(state, observed_action_taken);
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
        fill_for_action(observed_action_taken),
    )
    .expect("adapter selects the metamagic option spell subject");
    let result = resolve_battle_subject_observed(state, request, &mut trace);
    (
        quickened_spell_governor_from_battle(result.state()),
        observed_reducer_route(&trace, &[ReducerRouteSubjectFamily::MetamagicOptionSpell]),
    )
}

fn quickened_setup_for_action(observed_action_taken: &str) -> BattleSetup {
    let mut state = start_quickened_spell_governor_battle();
    if observed_action_taken == "doResolveQuickenedAfterMagicActionSpent" {
        state.action_available = false;
    }
    if observed_action_taken == "doRejectPriorLevelOnePlusSpell" {
        state
            .level_one_plus_spell_casters_this_turn
            .push(Actor::Fighter);
    }
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

fn prepare_state_for_action(
    state: crate::rules::battle_reducer_spine::BattleState,
    observed_action_taken: &str,
) -> crate::rules::battle_reducer_spine::BattleState {
    match observed_action_taken {
        "doRejectUnaffordable" => with_battle_sorcery_points(state, 1),
        "doRejectUnsupportedSecondOption" | "doRejectOnePerSpell" => {
            with_battle_sorcery_points(state, 5)
        }
        _ => state,
    }
}

fn fill_for_action(observed_action_taken: &str) -> BattleMetamagicOptionSpellFill {
    let mut fill = BattleMetamagicOptionSpellFill {
        option_facts: BattleMetamagicOptionFacts::quickened_action_to_bonus_action(),
        effect: effect_for_action(observed_action_taken),
        options_already_applied_to_spell: 0,
        selected_second_option_supported: true,
        spell_uses_level_one_plus_slot: true,
        spell_consumes_magic_action: false,
    };
    match observed_action_taken {
        "doRejectUnknownOption" => {
            fill.option_facts.selected_option_admitted = false;
        }
        "doRejectUnsupportedSecondOption" => {
            fill.options_already_applied_to_spell = 1;
            fill.selected_second_option_supported = false;
        }
        "doRejectOnePerSpell" => {
            fill.options_already_applied_to_spell = 1;
        }
        _ => {}
    }
    fill
}

fn effect_for_action(observed_action_taken: &str) -> BattleMetamagicOptionSpellEffect {
    match observed_action_taken {
        "doResolveQuickenedRestoration"
        | "doResolveQuickenedAfterMagicActionSpent"
        | "doRejectUnaffordable"
        | "doRejectUnknownOption"
        | "doRejectUnsupportedSecondOption"
        | "doRejectOnePerSpell"
        | "doRejectPriorLevelOnePlusSpell" => {
            BattleMetamagicOptionSpellEffect::HitPointRestoration {
                target_hit_points_after: 14,
            }
        }
        "doResolveQuickenedSaveGatedCondition" => {
            BattleMetamagicOptionSpellEffect::SaveGatedCondition
        }
        "doResolveQuickenedSaveGatedConditionImmunity" => {
            BattleMetamagicOptionSpellEffect::SaveGatedConditionImmunity
        }
        "doResolveQuickenedDirectCondition" => BattleMetamagicOptionSpellEffect::DirectCondition,
        "doResolveQuickenedRollModifier" => BattleMetamagicOptionSpellEffect::RollModifier,
        action => panic!("unsupported mbt::actionTaken {action}"),
    }
}

fn scenario_for_action(observed_action_taken: &str) -> QuickenedSpellScenarioOutcome {
    match observed_action_taken {
        "doResolveQuickenedRestoration" => {
            QuickenedSpellScenarioOutcome::ResolvedQuickenedRestoration
        }
        "doResolveQuickenedSaveGatedCondition" => {
            QuickenedSpellScenarioOutcome::ResolvedQuickenedSaveGatedCondition
        }
        "doResolveQuickenedSaveGatedConditionImmunity" => {
            QuickenedSpellScenarioOutcome::ResolvedQuickenedSaveGatedConditionImmunity
        }
        "doResolveQuickenedDirectCondition" => {
            QuickenedSpellScenarioOutcome::ResolvedQuickenedDirectCondition
        }
        "doResolveQuickenedRollModifier" => {
            QuickenedSpellScenarioOutcome::ResolvedQuickenedRollModifier
        }
        "doResolveQuickenedAfterMagicActionSpent" => {
            QuickenedSpellScenarioOutcome::ResolvedAfterMagicActionSpent
        }
        "doRejectUnaffordable" => QuickenedSpellScenarioOutcome::RejectedUnaffordable,
        "doRejectUnknownOption" => QuickenedSpellScenarioOutcome::RejectedUnknownOption,
        "doRejectUnsupportedSecondOption" => {
            QuickenedSpellScenarioOutcome::RejectedUnsupportedSecondOption
        }
        "doRejectOnePerSpell" => QuickenedSpellScenarioOutcome::RejectedOnePerSpell,
        "doRejectPriorLevelOnePlusSpell" => {
            QuickenedSpellScenarioOutcome::RejectedPriorLevelOnePlusSpell
        }
        action => panic!("unsupported mbt::actionTaken {action}"),
    }
}

fn quickened_success(scenario: QuickenedSpellScenarioOutcome) -> bool {
    matches!(
        scenario,
        QuickenedSpellScenarioOutcome::ResolvedQuickenedRestoration
            | QuickenedSpellScenarioOutcome::ResolvedQuickenedSaveGatedCondition
            | QuickenedSpellScenarioOutcome::ResolvedQuickenedSaveGatedConditionImmunity
            | QuickenedSpellScenarioOutcome::ResolvedQuickenedDirectCondition
            | QuickenedSpellScenarioOutcome::ResolvedQuickenedRollModifier
            | QuickenedSpellScenarioOutcome::ResolvedAfterMagicActionSpent
    )
}

fn expected_sorcery_points_remaining(scenario: QuickenedSpellScenarioOutcome) -> i16 {
    match scenario {
        QuickenedSpellScenarioOutcome::RejectedUnaffordable => 1,
        QuickenedSpellScenarioOutcome::RejectedUnsupportedSecondOption
        | QuickenedSpellScenarioOutcome::RejectedOnePerSpell => 5,
        _ if quickened_success(scenario) => 2,
        _ => 4,
    }
}

fn expected_target_hit_points(scenario: QuickenedSpellScenarioOutcome) -> i16 {
    match scenario {
        QuickenedSpellScenarioOutcome::ResolvedQuickenedRestoration
        | QuickenedSpellScenarioOutcome::ResolvedAfterMagicActionSpent => 14,
        _ => 4,
    }
}

fn invalid_kind_for_scenario(scenario: QuickenedSpellScenarioOutcome) -> QuickenedSpellInvalidKind {
    match scenario {
        QuickenedSpellScenarioOutcome::RejectedUnaffordable => {
            QuickenedSpellInvalidKind::Unaffordable
        }
        QuickenedSpellScenarioOutcome::RejectedUnknownOption => {
            QuickenedSpellInvalidKind::UnknownOption
        }
        QuickenedSpellScenarioOutcome::RejectedUnsupportedSecondOption => {
            QuickenedSpellInvalidKind::UnsupportedSecondOption
        }
        QuickenedSpellScenarioOutcome::RejectedOnePerSpell => {
            QuickenedSpellInvalidKind::OnePerSpell
        }
        QuickenedSpellScenarioOutcome::RejectedPriorLevelOnePlusSpell => {
            QuickenedSpellInvalidKind::SameTurnLevelOnePlus
        }
        _ => QuickenedSpellInvalidKind::None,
    }
}

fn invalid_kind_ref(invalid_kind: QuickenedSpellInvalidKind) -> &'static str {
    match invalid_kind {
        QuickenedSpellInvalidKind::None => "none",
        QuickenedSpellInvalidKind::Unaffordable => "unaffordable",
        QuickenedSpellInvalidKind::UnknownOption => "unknownOption",
        QuickenedSpellInvalidKind::UnsupportedSecondOption => "unsupportedSecondOption",
        QuickenedSpellInvalidKind::OnePerSpell => "onePerSpell",
        QuickenedSpellInvalidKind::SameTurnLevelOnePlus => "sameTurnLevelOnePlus",
    }
}

fn scenario_outcome_ref(outcome: QuickenedSpellScenarioOutcome) -> &'static str {
    match outcome {
        QuickenedSpellScenarioOutcome::Init => "Init",
        QuickenedSpellScenarioOutcome::ResolvedQuickenedRestoration => {
            "ResolvedQuickenedRestoration"
        }
        QuickenedSpellScenarioOutcome::ResolvedQuickenedSaveGatedCondition => {
            "ResolvedQuickenedSaveGatedCondition"
        }
        QuickenedSpellScenarioOutcome::ResolvedQuickenedSaveGatedConditionImmunity => {
            "ResolvedQuickenedSaveGatedConditionImmunity"
        }
        QuickenedSpellScenarioOutcome::ResolvedQuickenedDirectCondition => {
            "ResolvedQuickenedDirectCondition"
        }
        QuickenedSpellScenarioOutcome::ResolvedQuickenedRollModifier => {
            "ResolvedQuickenedRollModifier"
        }
        QuickenedSpellScenarioOutcome::ResolvedAfterMagicActionSpent => {
            "ResolvedAfterMagicActionSpent"
        }
        QuickenedSpellScenarioOutcome::RejectedUnaffordable => "RejectedUnaffordable",
        QuickenedSpellScenarioOutcome::RejectedUnknownOption => "RejectedUnknownOption",
        QuickenedSpellScenarioOutcome::RejectedUnsupportedSecondOption => {
            "RejectedUnsupportedSecondOption"
        }
        QuickenedSpellScenarioOutcome::RejectedOnePerSpell => "RejectedOnePerSpell",
        QuickenedSpellScenarioOutcome::RejectedPriorLevelOnePlusSpell => {
            "RejectedPriorLevelOnePlusSpell"
        }
    }
}

fn protocol_ref(protocol: QuickenedSpellProtocol) -> &'static str {
    match protocol {
        QuickenedSpellProtocol::Init => "init",
        QuickenedSpellProtocol::Resolved => "resolved",
    }
}
