use crate::rules::ability_check_choice_search::{
    AbilityCheckChoiceSearchProjection, AbilityCheckSearchAbility, AbilityCheckSearchHole,
    AbilityCheckSearchProtocol, AbilityCheckSearchRollMode, AbilityCheckSearchSkill,
    SearchAbilityCheckFacts, SearchTargetChoiceFacts,
};
use crate::rules::battle_reducer_spine::{
    ability_check_choice_search_projection_from_battle,
    ability_check_choice_search_projection_from_battle_result, discover_battle_acts_observed,
    resolve_battle_subject_observed, start_battle_observed,
    start_roll_modifier_skill_choice_battle, start_search_ability_check_battle, Actor,
    BattleAbilityCheckSearchFill, BattleEntrypointTrace, BattleResolutionInvalidReason,
    BattleResolutionRequest, BattleResolutionResult, BattleRollModifierEffectFill, BattleState,
    BattleSubject, BattleSubjectKind,
};

use super::battle_runtime_reducer_route::{
    observed_reducer_route, route_discover_battle_acts_from_route_holes,
    route_resolve_battle_subject_from_route_result,
    route_resolve_battle_subject_with_fill_evidence_from_route_result,
    route_resolve_battle_subject_without_fill_from_route_result, route_start_battle,
    setup_from_battle_state, ReducerRouteAbilityChoice, ReducerRouteEvent,
    ReducerRouteFillEvidence, ReducerRouteFillKind, ReducerRouteHoleKind, ReducerRouteOwnerGroup,
    ReducerRouteResolutionOutcome, ReducerRouteSkillChoice, ReducerRouteSubjectFamily,
};

pub const BRANCH_ACTIONS: [&str; 9] = [
    "doSearchTargetChoiceOpen",
    "doSearchAbilityCheckOpen",
    "doSearchInvalidTargetRejected",
    "doSearchInvalidAbilityFillRejected",
    "doSearchFails",
    "doSearchSucceeds",
    "doGuidanceSkillChoiceOpen",
    "doGuidanceInvalidAbilityFillRejected",
    "doGuidanceSkillAthletics",
];

pub const OUT_OF_SCOPE_BRANCH_ACTIONS: [(&str, &str); 3] = [
    (
        "doEnhanceAbilityChoiceOpen",
        "Enhance Ability is a level-2 spell branch for a later route lane.",
    ),
    (
        "doEnhanceAbilityInvalidSkillFillRejected",
        "Enhance Ability is a level-2 spell branch for a later route lane.",
    ),
    (
        "doEnhanceAbilityDex",
        "Enhance Ability is a level-2 spell branch for a later route lane.",
    ),
];

pub fn replay_observed_action(observed_action_taken: &str) -> AbilityCheckChoiceSearchProjection {
    match observed_action_taken {
        "doSearchTargetChoiceOpen" => {
            let state = start_search_ability_check_battle();
            ability_check_choice_search_projection_from_battle(&state)
        }
        "doSearchAbilityCheckOpen" => {
            let result = resolve_search_target(true);
            ability_check_choice_search_projection_from_battle_result(&result)
        }
        "doSearchInvalidTargetRejected" => {
            let result = resolve_search_target(false);
            ability_check_choice_search_projection_from_battle_result(&result)
        }
        "doSearchInvalidAbilityFillRejected" => {
            let (state, subject) = search_ability_check_open();
            let result = resolve_battle_subject(
                state,
                subject,
                BattleAbilityCheckSearchFill::SkillChoice(AbilityCheckSearchSkill::Athletics),
            );
            ability_check_choice_search_projection_from_battle_result(&result)
        }
        "doSearchFails" => {
            let result = resolve_search_ability_check(8, 12);
            ability_check_choice_search_projection_from_battle_result(&result)
        }
        "doSearchSucceeds" => {
            let result = resolve_search_ability_check(15, 12);
            ability_check_choice_search_projection_from_battle_result(&result)
        }
        "doGuidanceSkillChoiceOpen" => {
            let result = resolve_roll_modifier_target(start_roll_modifier_skill_choice_battle());
            ability_check_choice_search_projection_from_battle_result(&result)
        }
        "doGuidanceInvalidAbilityFillRejected" => {
            let (state, subject) =
                roll_modifier_choice_open(start_roll_modifier_skill_choice_battle());
            let result = resolve_roll_modifier(
                state,
                subject,
                BattleRollModifierEffectFill::AbilityChoice(AbilityCheckSearchAbility::Dexterity),
            );
            ability_check_choice_search_projection_from_battle_result(&result)
        }
        "doGuidanceSkillAthletics" => {
            let (state, subject) =
                roll_modifier_choice_open(start_roll_modifier_skill_choice_battle());
            let result = resolve_roll_modifier(
                state,
                subject,
                BattleRollModifierEffectFill::SkillChoice(AbilityCheckSearchSkill::Athletics),
            );
            ability_check_choice_search_projection_from_battle_result(&result)
        }
        action if out_of_scope_reason(action).is_some() => {
            panic!("out-of-scope ability-check/search choice branch: {action}")
        }
        action => panic!("unsupported mbt::actionTaken {action}"),
    }
}

pub fn expected_witness(observed_action_taken: &str) -> AbilityCheckChoiceSearchProjection {
    match observed_action_taken {
        "doSearchTargetChoiceOpen" => expected_projection(
            AbilityCheckSearchProtocol::NeedsHoles,
            vec![AbilityCheckSearchHole::TargetChoice],
            true,
            true,
            ProjectionChoices::none(),
        ),
        "doSearchAbilityCheckOpen" => expected_projection(
            AbilityCheckSearchProtocol::NeedsHoles,
            vec![AbilityCheckSearchHole::AbilityCheck],
            true,
            true,
            ProjectionChoices::none(),
        ),
        "doSearchInvalidTargetRejected" | "doSearchInvalidAbilityFillRejected" => {
            expected_projection(
                AbilityCheckSearchProtocol::Invalid,
                Vec::new(),
                true,
                true,
                ProjectionChoices::none(),
            )
        }
        "doSearchFails" => expected_projection(
            AbilityCheckSearchProtocol::Resolved,
            Vec::new(),
            true,
            false,
            ProjectionChoices::none(),
        ),
        "doSearchSucceeds" => expected_projection(
            AbilityCheckSearchProtocol::Resolved,
            Vec::new(),
            false,
            false,
            ProjectionChoices::none(),
        ),
        "doGuidanceSkillChoiceOpen" => expected_projection(
            AbilityCheckSearchProtocol::NeedsHoles,
            vec![AbilityCheckSearchHole::SkillChoice],
            false,
            true,
            ProjectionChoices::none(),
        ),
        "doGuidanceInvalidAbilityFillRejected" => expected_projection(
            AbilityCheckSearchProtocol::Invalid,
            Vec::new(),
            false,
            true,
            ProjectionChoices::none(),
        ),
        "doGuidanceSkillAthletics" => expected_projection(
            AbilityCheckSearchProtocol::Resolved,
            Vec::new(),
            false,
            false,
            ProjectionChoices {
                target_effect_count: 1,
                caster_effect_count: 1,
                d20_modifier_skill: Some(AbilityCheckSearchSkill::Athletics),
                ..ProjectionChoices::none()
            },
        ),
        action if out_of_scope_reason(action).is_some() => {
            panic!("out-of-scope ability-check/search choice branch: {action}")
        }
        action => panic!("unsupported mbt::actionTaken {action}"),
    }
}

pub fn replay_observed_route(observed_action_taken: &str) -> Vec<ReducerRouteEvent> {
    let (_state, trace) = match observed_action_taken {
        "doSearchTargetChoiceOpen" => observed_search_discovery_route(),
        "doSearchAbilityCheckOpen" => observed_search_target_route(true),
        "doSearchInvalidTargetRejected" => observed_search_target_route(false),
        "doSearchInvalidAbilityFillRejected" => observed_search_ability_fill_route(
            BattleAbilityCheckSearchFill::SkillChoice(AbilityCheckSearchSkill::Athletics),
        ),
        "doSearchFails" => observed_search_ability_fill_route(
            BattleAbilityCheckSearchFill::AbilityCheck(SearchAbilityCheckFacts {
                total: 8,
                difficulty_class: 12,
            }),
        ),
        "doSearchSucceeds" => observed_search_ability_fill_route(
            BattleAbilityCheckSearchFill::AbilityCheck(SearchAbilityCheckFacts {
                total: 15,
                difficulty_class: 12,
            }),
        ),
        "doGuidanceSkillChoiceOpen" => {
            observed_roll_modifier_target_route(start_roll_modifier_skill_choice_battle())
        }
        "doGuidanceInvalidAbilityFillRejected" => observed_roll_modifier_choice_route(
            start_roll_modifier_skill_choice_battle(),
            BattleRollModifierEffectFill::AbilityChoice(AbilityCheckSearchAbility::Dexterity),
            false,
        ),
        "doGuidanceSkillAthletics" => observed_roll_modifier_choice_route(
            start_roll_modifier_skill_choice_battle(),
            BattleRollModifierEffectFill::SkillChoice(AbilityCheckSearchSkill::Athletics),
            true,
        ),
        action if out_of_scope_reason(action).is_some() => return Vec::new(),
        action => panic!("unsupported mbt::actionTaken {action}"),
    };
    observed_reducer_route(
        &trace,
        &[
            ReducerRouteSubjectFamily::AbilityCheckSearch,
            ReducerRouteSubjectFamily::RollModifierEffect,
        ],
    )
}

pub fn expected_route(observed_action_taken: &str) -> Vec<ReducerRouteEvent> {
    match observed_action_taken {
        "doSearchTargetChoiceOpen" => search_target_choice_route(),
        "doSearchAbilityCheckOpen" => search_ability_check_route(),
        "doSearchInvalidTargetRejected" => {
            let mut route = search_target_choice_route();
            route.push(search_resolve(
                ReducerRouteFillKind::TargetChoice,
                ReducerRouteResolutionOutcome::Invalid(BattleResolutionInvalidReason::InvalidFill),
                Vec::new(),
                ReducerRouteOwnerGroup::TargetSelection,
            ));
            route
        }
        "doSearchInvalidAbilityFillRejected" => {
            let mut route = search_ability_check_route();
            route.push(search_resolve_with_fill_evidence(
                ReducerRouteFillEvidence::SkillChoice(ReducerRouteSkillChoice::Athletics),
                ReducerRouteResolutionOutcome::Invalid(BattleResolutionInvalidReason::InvalidFill),
                Vec::new(),
                ReducerRouteOwnerGroup::HoleFrontier,
            ));
            route
        }
        "doSearchFails" | "doSearchSucceeds" => {
            let mut route = search_ability_check_route();
            route.push(search_resolve(
                ReducerRouteFillKind::AbilityCheck,
                ReducerRouteResolutionOutcome::Resolved,
                Vec::new(),
                ReducerRouteOwnerGroup::AbilityCheck,
            ));
            route
        }
        "doGuidanceSkillChoiceOpen" => {
            roll_modifier_opening_route(ReducerRouteHoleKind::SkillChoice)
        }
        "doGuidanceInvalidAbilityFillRejected" => roll_modifier_invalid_choice_route(
            ReducerRouteHoleKind::SkillChoice,
            ReducerRouteFillEvidence::AbilityChoice(ReducerRouteAbilityChoice::Dexterity),
        ),
        "doGuidanceSkillAthletics" => roll_modifier_accepted_choice_route(
            ReducerRouteHoleKind::SkillChoice,
            ReducerRouteFillEvidence::SkillChoice(ReducerRouteSkillChoice::Athletics),
        ),
        action if out_of_scope_reason(action).is_some() => Vec::new(),
        action => panic!("unsupported mbt::actionTaken {action}"),
    }
}

pub fn out_of_scope_reason(observed_action_taken: &str) -> Option<&'static str> {
    OUT_OF_SCOPE_BRANCH_ACTIONS
        .iter()
        .find_map(|(action, reason)| (*action == observed_action_taken).then_some(*reason))
}

pub fn projection_payload(state: &AbilityCheckChoiceSearchProjection) -> String {
    [
        format!("protocolResult={}", protocol_ref(state.protocol)),
        format!("protocolHoles={}", joined_holes(&state.protocol_holes)),
        format!("targetHidden={}", state.target_hidden),
        format!("actionAvailable={}", state.action_available),
        format!("targetEffectCount={}", state.target_effect_count),
        format!("casterEffectCount={}", state.caster_effect_count),
        format!("d20ModifierSkill={}", skill_ref(state.d20_modifier_skill)),
        format!(
            "abilityCheckModeAbility={}",
            ability_ref(state.ability_check_mode_ability)
        ),
        format!(
            "targetDexRollMode={}",
            roll_mode_ref(state.target_dexterity_roll_mode)
        ),
    ]
    .join("\n")
}

fn observed_search_discovery_route() -> (BattleState, BattleEntrypointTrace) {
    let mut trace = BattleEntrypointTrace::default();
    let state = start_observed(start_search_ability_check_battle(), &mut trace);
    let _ = discover_battle_acts_observed(&state, &mut trace);
    (state, trace)
}

fn observed_search_target_route(target_admitted: bool) -> (BattleState, BattleEntrypointTrace) {
    let mut trace = BattleEntrypointTrace::default();
    let state = start_observed(start_search_ability_check_battle(), &mut trace);
    let subject = discover_subject(&state, BattleSubjectKind::AbilityCheckSearch, &mut trace);
    let result = resolve_battle_subject_observed(
        state,
        BattleResolutionRequest::ability_check_search(
            subject,
            BattleAbilityCheckSearchFill::TargetChoice {
                target: Actor::Goblin,
                facts: SearchTargetChoiceFacts { target_admitted },
            },
        )
        .expect("ability-check search subject should accept search fill"),
        &mut trace,
    );
    (result.into_state(), trace)
}

fn observed_search_ability_fill_route(
    fill: BattleAbilityCheckSearchFill,
) -> (BattleState, BattleEntrypointTrace) {
    let mut trace = BattleEntrypointTrace::default();
    let state = start_observed(start_search_ability_check_battle(), &mut trace);
    let subject = discover_subject(&state, BattleSubjectKind::AbilityCheckSearch, &mut trace);
    let result = resolve_battle_subject_observed(
        state,
        BattleResolutionRequest::ability_check_search(
            subject,
            BattleAbilityCheckSearchFill::TargetChoice {
                target: Actor::Goblin,
                facts: SearchTargetChoiceFacts {
                    target_admitted: true,
                },
            },
        )
        .expect("ability-check search subject should accept target fill"),
        &mut trace,
    );
    let (state, subject) = continuation(result, "search ability check");
    let result = resolve_battle_subject_observed(
        state,
        BattleResolutionRequest::ability_check_search(subject, fill)
            .expect("ability-check search subject should accept ability fill"),
        &mut trace,
    );
    (result.into_state(), trace)
}

fn observed_roll_modifier_target_route(state: BattleState) -> (BattleState, BattleEntrypointTrace) {
    let mut trace = BattleEntrypointTrace::default();
    let state = start_observed(state, &mut trace);
    let subject = discover_subject(&state, BattleSubjectKind::RollModifierEffect, &mut trace);
    let result = resolve_battle_subject_observed(
        state,
        BattleResolutionRequest::roll_modifier_effect(
            subject,
            BattleRollModifierEffectFill::TargetChoice(Actor::Goblin),
        )
        .expect("roll-modifier subject should accept target fill"),
        &mut trace,
    );
    (result.into_state(), trace)
}

fn observed_roll_modifier_choice_route(
    state: BattleState,
    fill: BattleRollModifierEffectFill,
    follow_concentration: bool,
) -> (BattleState, BattleEntrypointTrace) {
    let mut trace = BattleEntrypointTrace::default();
    let state = start_observed(state, &mut trace);
    let subject = discover_subject(&state, BattleSubjectKind::RollModifierEffect, &mut trace);
    let result = resolve_battle_subject_observed(
        state,
        BattleResolutionRequest::roll_modifier_effect(
            subject,
            BattleRollModifierEffectFill::TargetChoice(Actor::Goblin),
        )
        .expect("roll-modifier subject should accept target fill"),
        &mut trace,
    );
    let (state, subject) = continuation(result, "roll modifier choice");
    let result = resolve_battle_subject_observed(
        state,
        BattleResolutionRequest::roll_modifier_effect(subject, fill)
            .expect("roll-modifier subject should accept choice fill"),
        &mut trace,
    );
    if !follow_concentration {
        return (result.into_state(), trace);
    }
    let state = result.into_state();
    let result = resolve_battle_subject_observed(
        state,
        BattleResolutionRequest::roll_modifier_effect_without_fill(subject)
            .expect("roll-modifier subject should accept concentration no-fill"),
        &mut trace,
    );
    (result.into_state(), trace)
}

fn start_observed(state: BattleState, trace: &mut BattleEntrypointTrace) -> BattleState {
    start_battle_observed(setup_from_battle_state(state), trace).state
}

fn discover_subject(
    state: &BattleState,
    kind: BattleSubjectKind,
    trace: &mut BattleEntrypointTrace,
) -> BattleSubject {
    discover_battle_acts_observed(state, trace)
        .available_acts()
        .iter()
        .map(|act| act.subject)
        .find(|subject| subject.kind == kind)
        .expect("expected route subject should be discoverable")
}

fn resolve_search_target(target_admitted: bool) -> BattleResolutionResult {
    let state = start_search_ability_check_battle();
    let subject = plain_subject(&state, BattleSubjectKind::AbilityCheckSearch);
    resolve_battle_subject(
        state,
        subject,
        BattleAbilityCheckSearchFill::TargetChoice {
            target: Actor::Goblin,
            facts: SearchTargetChoiceFacts { target_admitted },
        },
    )
}

fn search_ability_check_open() -> (BattleState, BattleSubject) {
    let result = resolve_search_target(true);
    continuation(result, "search ability check")
}

fn resolve_search_ability_check(total: i16, difficulty_class: i16) -> BattleResolutionResult {
    let (state, subject) = search_ability_check_open();
    resolve_battle_subject(
        state,
        subject,
        BattleAbilityCheckSearchFill::AbilityCheck(SearchAbilityCheckFacts {
            total,
            difficulty_class,
        }),
    )
}

fn roll_modifier_choice_open(state: BattleState) -> (BattleState, BattleSubject) {
    let result = resolve_roll_modifier_target(state);
    continuation(result, "roll modifier choice")
}

fn resolve_roll_modifier_target(state: BattleState) -> BattleResolutionResult {
    let subject = plain_subject(&state, BattleSubjectKind::RollModifierEffect);
    resolve_roll_modifier(
        state,
        subject,
        BattleRollModifierEffectFill::TargetChoice(Actor::Goblin),
    )
}

fn resolve_battle_subject(
    state: BattleState,
    subject: BattleSubject,
    fill: BattleAbilityCheckSearchFill,
) -> BattleResolutionResult {
    resolve_battle_subject_observed(
        state,
        BattleResolutionRequest::ability_check_search(subject, fill)
            .expect("ability-check search subject should match"),
        &mut BattleEntrypointTrace::default(),
    )
}

fn resolve_roll_modifier(
    state: BattleState,
    subject: BattleSubject,
    fill: BattleRollModifierEffectFill,
) -> BattleResolutionResult {
    resolve_battle_subject_observed(
        state,
        BattleResolutionRequest::roll_modifier_effect(subject, fill)
            .expect("roll modifier subject should match"),
        &mut BattleEntrypointTrace::default(),
    )
}

fn plain_subject(state: &BattleState, kind: BattleSubjectKind) -> BattleSubject {
    crate::rules::battle_reducer_spine::discover_battle_acts(state)
        .available_acts()
        .iter()
        .map(|act| act.subject)
        .find(|subject| subject.kind == kind)
        .expect("expected route subject should be discoverable")
}

fn continuation(result: BattleResolutionResult, context: &str) -> (BattleState, BattleSubject) {
    let outcome = result.outcome();
    result
        .into_needs_holes()
        .map(|needs| (needs.state, needs.subject))
        .unwrap_or_else(|| panic!("{context} should need holes, got {outcome:?}"))
}

fn search_target_choice_route() -> Vec<ReducerRouteEvent> {
    vec![
        route_start_battle(ReducerRouteOwnerGroup::ActionEconomy),
        route_discover_battle_acts_from_route_holes(
            ReducerRouteSubjectFamily::AbilityCheckSearch,
            vec![ReducerRouteHoleKind::TargetChoice],
            ReducerRouteOwnerGroup::ActionEconomy,
        ),
    ]
}

fn search_ability_check_route() -> Vec<ReducerRouteEvent> {
    let mut route = search_target_choice_route();
    route.push(search_resolve(
        ReducerRouteFillKind::TargetChoice,
        ReducerRouteResolutionOutcome::NeedsHoles,
        vec![ReducerRouteHoleKind::AbilityCheck],
        ReducerRouteOwnerGroup::TargetSelection,
    ));
    route
}

fn search_resolve(
    fill: ReducerRouteFillKind,
    outcome: ReducerRouteResolutionOutcome,
    holes: Vec<ReducerRouteHoleKind>,
    owner: ReducerRouteOwnerGroup,
) -> ReducerRouteEvent {
    route_resolve_battle_subject_from_route_result(
        ReducerRouteSubjectFamily::AbilityCheckSearch,
        fill,
        outcome,
        holes,
        owner,
    )
}

fn search_resolve_with_fill_evidence(
    fill: ReducerRouteFillEvidence,
    outcome: ReducerRouteResolutionOutcome,
    holes: Vec<ReducerRouteHoleKind>,
    owner: ReducerRouteOwnerGroup,
) -> ReducerRouteEvent {
    route_resolve_battle_subject_with_fill_evidence_from_route_result(
        ReducerRouteSubjectFamily::AbilityCheckSearch,
        fill,
        outcome,
        holes,
        owner,
    )
}

fn roll_modifier_opening_route(choice_hole: ReducerRouteHoleKind) -> Vec<ReducerRouteEvent> {
    vec![
        route_start_battle(ReducerRouteOwnerGroup::ActionEconomy),
        route_discover_battle_acts_from_route_holes(
            ReducerRouteSubjectFamily::RollModifierEffect,
            vec![ReducerRouteHoleKind::TargetChoice, choice_hole],
            ReducerRouteOwnerGroup::SpellSlotAndActionEconomy,
        ),
        route_resolve_battle_subject_from_route_result(
            ReducerRouteSubjectFamily::RollModifierEffect,
            ReducerRouteFillKind::TargetChoice,
            ReducerRouteResolutionOutcome::NeedsHoles,
            vec![choice_hole],
            ReducerRouteOwnerGroup::TargetSelection,
        ),
    ]
}

fn roll_modifier_invalid_choice_route(
    choice_hole: ReducerRouteHoleKind,
    fill: ReducerRouteFillEvidence,
) -> Vec<ReducerRouteEvent> {
    let mut route = roll_modifier_opening_route(choice_hole);
    route.push(
        route_resolve_battle_subject_with_fill_evidence_from_route_result(
            ReducerRouteSubjectFamily::RollModifierEffect,
            fill,
            ReducerRouteResolutionOutcome::Invalid(BattleResolutionInvalidReason::InvalidFill),
            Vec::new(),
            ReducerRouteOwnerGroup::HoleFrontier,
        ),
    );
    route
}

fn roll_modifier_accepted_choice_route(
    choice_hole: ReducerRouteHoleKind,
    fill: ReducerRouteFillEvidence,
) -> Vec<ReducerRouteEvent> {
    let mut route = roll_modifier_opening_route(choice_hole);
    route.push(
        route_resolve_battle_subject_with_fill_evidence_from_route_result(
            ReducerRouteSubjectFamily::RollModifierEffect,
            fill,
            ReducerRouteResolutionOutcome::Resolved,
            Vec::new(),
            ReducerRouteOwnerGroup::ActiveEffect,
        ),
    );
    route.push(route_resolve_battle_subject_without_fill_from_route_result(
        ReducerRouteSubjectFamily::RollModifierEffect,
        ReducerRouteResolutionOutcome::Resolved,
        Vec::new(),
        ReducerRouteOwnerGroup::Concentration,
    ));
    route
}

#[derive(Debug, Clone, Copy)]
struct ProjectionChoices {
    target_effect_count: i16,
    caster_effect_count: i16,
    d20_modifier_skill: Option<AbilityCheckSearchSkill>,
    ability_check_mode_ability: Option<AbilityCheckSearchAbility>,
    target_dexterity_roll_mode: AbilityCheckSearchRollMode,
}

impl ProjectionChoices {
    const fn none() -> Self {
        Self {
            target_effect_count: 0,
            caster_effect_count: 0,
            d20_modifier_skill: None,
            ability_check_mode_ability: None,
            target_dexterity_roll_mode: AbilityCheckSearchRollMode::Normal,
        }
    }
}

fn expected_projection(
    protocol: AbilityCheckSearchProtocol,
    protocol_holes: Vec<AbilityCheckSearchHole>,
    target_hidden: bool,
    action_available: bool,
    choices: ProjectionChoices,
) -> AbilityCheckChoiceSearchProjection {
    AbilityCheckChoiceSearchProjection {
        protocol,
        protocol_holes,
        target_hidden,
        action_available,
        target_effect_count: choices.target_effect_count,
        caster_effect_count: choices.caster_effect_count,
        d20_modifier_skill: choices.d20_modifier_skill,
        ability_check_mode_ability: choices.ability_check_mode_ability,
        target_dexterity_roll_mode: choices.target_dexterity_roll_mode,
    }
}

fn protocol_ref(protocol: AbilityCheckSearchProtocol) -> &'static str {
    match protocol {
        AbilityCheckSearchProtocol::Init => "init",
        AbilityCheckSearchProtocol::NeedsHoles => "needsHoles",
        AbilityCheckSearchProtocol::Resolved => "resolved",
        AbilityCheckSearchProtocol::Invalid => "invalid",
    }
}

fn joined_holes(holes: &[AbilityCheckSearchHole]) -> String {
    if holes.is_empty() {
        return "none".to_string();
    }
    holes
        .iter()
        .map(|hole| match hole {
            AbilityCheckSearchHole::TargetChoice => "targetChoice",
            AbilityCheckSearchHole::AbilityCheck => "abilityCheck",
            AbilityCheckSearchHole::SkillChoice => "skillChoice",
            AbilityCheckSearchHole::AbilityChoice => "abilityChoice",
        })
        .collect::<Vec<_>>()
        .join(",")
}

fn skill_ref(skill: Option<AbilityCheckSearchSkill>) -> &'static str {
    match skill {
        Some(AbilityCheckSearchSkill::Athletics) => "athletics",
        Some(AbilityCheckSearchSkill::Perception) => "perception",
        None => "none",
    }
}

fn ability_ref(ability: Option<AbilityCheckSearchAbility>) -> &'static str {
    match ability {
        Some(AbilityCheckSearchAbility::Dexterity) => "dex",
        Some(AbilityCheckSearchAbility::Wisdom) => "wis",
        None => "none",
    }
}

fn roll_mode_ref(mode: AbilityCheckSearchRollMode) -> &'static str {
    match mode {
        AbilityCheckSearchRollMode::Normal => "normal",
        AbilityCheckSearchRollMode::Advantage => "advantage",
    }
}
