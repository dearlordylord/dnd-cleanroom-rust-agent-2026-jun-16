use super::battle_runtime_reducer_route::{
    battle_setup_from_state, observed_reducer_route, route_discover_battle_acts,
    route_resolve_battle_subject, route_resolve_battle_subject_from_route_result,
    route_start_battle, ReducerRouteEvent, ReducerRouteFillKind, ReducerRouteOwnerGroup,
    ReducerRouteResolutionOutcome, ReducerRouteSubjectFamily,
};
use crate::rules::battle_reducer_spine::{
    discover_battle_acts_observed, resolve_battle_subject_observed, start_battle_observed,
    start_metamagic_option_spell_battle, subtle_spell_from_battle, BattleEntrypointTrace,
    BattleMetamagicOptionFacts, BattleMetamagicOptionSpellEffect, BattleMetamagicOptionSpellFill,
    BattleResolutionInvalidReason, BattleResolutionRequest, BattleSubjectKind,
};
use crate::rules::sorcerer_metamagic::{
    SubtleSpellProtocol, SubtleSpellScenarioResult, SubtleSpellState,
};

pub const BRANCH_ACTIONS: [&str; 2] = [
    "doRejectSubtleFalseLifeWithoutSorceryPoints",
    "doResolveSubtleFalseLife",
];

pub fn replay_observed_action(observed_action_taken: &str) -> SubtleSpellState {
    let (state, _) = replay_reducer_route(observed_action_taken);
    state
}

pub fn replay_observed_route(observed_action_taken: &str) -> Vec<ReducerRouteEvent> {
    let (_, route) = replay_reducer_route(observed_action_taken);
    route
}

pub fn expected_witness(observed_action_taken: &str) -> SubtleSpellState {
    match observed_action_taken {
        "doRejectSubtleFalseLifeWithoutSorceryPoints" => SubtleSpellState {
            verbal_suppressed: false,
            somatic_suppressed: false,
            material_suppressed: false,
            material_preserved: false,
            sorcery_points_remaining: 0,
            temporary_hit_points: 0,
            scenario_result: SubtleSpellScenarioResult::UnaffordableSubtleFalseLife,
            protocol: SubtleSpellProtocol::InvalidUnsupportedActOption,
        },
        "doResolveSubtleFalseLife" => SubtleSpellState {
            verbal_suppressed: true,
            somatic_suppressed: true,
            material_suppressed: true,
            material_preserved: false,
            sorcery_points_remaining: 1,
            temporary_hit_points: 11,
            scenario_result: SubtleSpellScenarioResult::SubtleFalseLife,
            protocol: SubtleSpellProtocol::Resolved,
        },
        action => panic!("unsupported mbt::actionTaken {action}"),
    }
}

pub fn expected_route(observed_action_taken: &str) -> Vec<ReducerRouteEvent> {
    match observed_action_taken {
        "doResolveSubtleFalseLife" => metamagic_success_route(ReducerRouteOwnerGroup::Component),
        "doRejectSubtleFalseLifeWithoutSorceryPoints" => metamagic_invalid_route(),
        action => panic!("unsupported mbt::actionTaken {action}"),
    }
}

pub fn projection_payload(state: &SubtleSpellState) -> String {
    [
        format!("verbalSuppressed={}", state.verbal_suppressed),
        format!("somaticSuppressed={}", state.somatic_suppressed),
        format!("materialSuppressed={}", state.material_suppressed),
        format!("materialPreserved={}", state.material_preserved),
        format!("sorceryPointsRemaining={}", state.sorcery_points_remaining),
        format!("tempHp={}", state.temporary_hit_points),
        format!("scenarioResult={}", scenario_ref(state.scenario_result)),
        format!("protocolResult={}", protocol_ref(state.protocol)),
        format!(
            "protocolInvalidReason={}",
            invalid_reason_ref(state.protocol)
        ),
        "protocolHoles=none".to_string(),
    ]
    .join("\n")
}

fn scenario_ref(result: SubtleSpellScenarioResult) -> &'static str {
    match result {
        SubtleSpellScenarioResult::Init => "init",
        SubtleSpellScenarioResult::SubtleFalseLife => "subtleFalseLife",
        SubtleSpellScenarioResult::UnaffordableSubtleFalseLife => "unaffordableSubtleFalseLife",
    }
}

fn protocol_ref(protocol: SubtleSpellProtocol) -> &'static str {
    match protocol {
        SubtleSpellProtocol::Init => "init",
        SubtleSpellProtocol::Resolved => "resolved",
        SubtleSpellProtocol::InvalidUnsupportedActOption => "invalid",
    }
}

fn invalid_reason_ref(protocol: SubtleSpellProtocol) -> &'static str {
    match protocol {
        SubtleSpellProtocol::InvalidUnsupportedActOption => "WUnsupportedActOption",
        SubtleSpellProtocol::Init | SubtleSpellProtocol::Resolved => "none",
    }
}

fn replay_reducer_route(observed_action_taken: &str) -> (SubtleSpellState, Vec<ReducerRouteEvent>) {
    let mut trace = BattleEntrypointTrace::default();
    let setup = battle_setup_from_state(start_metamagic_option_spell_battle(
        starting_sorcery_points(observed_action_taken),
    ));
    let state = start_battle_observed(setup, &mut trace).state;
    let discovery = discover_battle_acts_observed(&state, &mut trace);
    let subject = discovery
        .available_acts()
        .iter()
        .find(|act| act.subject.kind == BattleSubjectKind::MetamagicOptionSpell)
        .map(|act| act.subject)
        .unwrap_or_else(|| {
            panic!("Subtle metamagic subject not discovered for {observed_action_taken}")
        });
    let request = BattleResolutionRequest::metamagic_option_spell(
        subject,
        BattleMetamagicOptionSpellFill {
            option_facts: BattleMetamagicOptionFacts::spell_component_suppression(),
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
        subtle_spell_from_battle(result.state()),
        observed_reducer_route(&trace, &[ReducerRouteSubjectFamily::MetamagicOptionSpell]),
    )
}

fn starting_sorcery_points(observed_action_taken: &str) -> i16 {
    match observed_action_taken {
        "doResolveSubtleFalseLife" => 2,
        "doRejectSubtleFalseLifeWithoutSorceryPoints" => 0,
        action => panic!("unsupported mbt::actionTaken {action}"),
    }
}

fn effect_for_action(observed_action_taken: &str) -> BattleMetamagicOptionSpellEffect {
    match observed_action_taken {
        "doResolveSubtleFalseLife" | "doRejectSubtleFalseLifeWithoutSorceryPoints" => {
            BattleMetamagicOptionSpellEffect::ComponentSuppressedHitPointBuff {
                temporary_hit_points: 11,
                verbal_suppressed: true,
                somatic_suppressed: true,
                material_suppressed: true,
                material_preserved: false,
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

fn metamagic_invalid_route() -> Vec<ReducerRouteEvent> {
    vec![
        route_start_battle(ReducerRouteOwnerGroup::ActionEconomy),
        route_discover_battle_acts(
            ReducerRouteSubjectFamily::MetamagicOptionSpell,
            Vec::new(),
            ReducerRouteOwnerGroup::FeatureResource,
        ),
        route_resolve_battle_subject_from_route_result(
            ReducerRouteSubjectFamily::MetamagicOptionSpell,
            ReducerRouteFillKind::UnitFeatureDecision,
            ReducerRouteResolutionOutcome::Invalid(BattleResolutionInvalidReason::InvalidFill),
            Vec::new(),
            ReducerRouteOwnerGroup::FeatureResource,
        ),
    ]
}
