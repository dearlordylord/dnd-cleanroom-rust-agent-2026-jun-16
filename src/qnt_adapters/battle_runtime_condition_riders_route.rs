use crate::rules::battle_reducer_spine::{
    condition_rider_route_subject_for_target, discover_condition_rider_route_subject_observed,
    discover_save_gated_condition_rider_subject_observed,
    discover_spell_attack_damage_subject_observed, resolve_battle_subject_observed,
    save_gated_condition_rider_subject_for_target, start_battle_observed, Actor, AttackRollFacts,
    BattleConditionImmunities, BattleConditionRiderBoundary, BattleConditionRiderCondition,
    BattleConditionRiderEffect, BattleEntrypointTrace, BattleGenericRouteFill,
    BattleResolutionRequest, BattleResolutionResult, BattleSetup, BattleSpellActiveEffectKind,
    BattleSpellAttackFill, BattleState, BattleSubject, BattleSubjectKind,
};

use super::battle_runtime_reducer_route::{
    observed_reducer_route, route_discover_battle_acts, route_resolve_battle_subject,
    route_resolve_battle_subject_without_fill, route_start_battle, ReducerRouteEvent,
    ReducerRouteFillKind, ReducerRouteOwnerGroup, ReducerRouteSubjectFamily,
};

pub const CONNECTOR_ACTIONS: [&str; 16] = [
    "doAdmitAttackHitPoisonConditionRider",
    "doRejectAttackHitPoisonConditionRiderByImmunity",
    "doExpireAttackHitPoisonConditionRider",
    "doAdmitFailedSaveBlindedNextTurnConditionRider",
    "doExpireFailedSaveBlindedNextTurnConditionRider",
    "doRejectFailedSaveBlindedConditionRiderByImmunity",
    "doAdmitFailedSaveBlindedRepeatSaveConditionRider",
    "doAdmitFailedSaveDeafenedRepeatSaveConditionRider",
    "doOpenFailedSaveConditionRepeatSaveFrontier",
    "doResolveFailedSaveConditionRepeatSaveSuccessCleanup",
    "doAdmitFailedSaveRestrainedUntilSpellEndEscapeConditionRider",
    "doOpenRestrainedAthleticsEscapeFrontier",
    "doResolveRestrainedAthleticsEscapeSuccessCleanup",
    "doAdmitFailedSaveSleepIncapacitatedConditionRider",
    "doOpenSleepRepeatSaveFrontier",
    "doResolveSleepRepeatSaveFailureTransitionToUnconscious",
];

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ConditionRiderRouteFact {
    HostAttackHitDamageOutcome,
    HostFailedSavingThrowOutcome,
    AffectedTargetConditionCarrier,
    Condition(BattleConditionRiderCondition),
    ConditionApplied,
    ConditionRejectedByImmunity,
    Boundary(BattleConditionRiderBoundary),
    AthleticsActionEscapeCheck,
    RepeatSaveFrontierOpened,
    RepeatSaveSucceeded,
    RepeatSaveFailed,
    Transition(BattleConditionRiderCondition),
    EscapeCheckFrontierOpened,
    EscapeCheckSucceeded,
    Expired,
    CleanedUpBy(ReducerRouteOwnerGroup),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ConditionRiderRouteWitness {
    pub condition_effect: BattleConditionRiderEffect,
    pub rejected_by_immunity: bool,
    pub active_after_host: bool,
    pub active_after_admission: bool,
    pub expired: bool,
    pub cleaned_up: bool,
    pub repeat_save_frontier_opened: bool,
    pub escape_check_frontier_opened: bool,
    pub transitioned_to_unconscious: bool,
    pub facts: Vec<ConditionRiderRouteFact>,
}

pub fn replay_observed_action(observed_action_taken: &str) -> ConditionRiderRouteWitness {
    replay_observed_state_and_route(observed_action_taken).0
}

pub fn replay_observed_route(observed_action_taken: &str) -> Vec<ReducerRouteEvent> {
    replay_observed_state_and_route(observed_action_taken).1
}

pub fn expected_witness(observed_action_taken: &str) -> ConditionRiderRouteWitness {
    expected_for_action(observed_action_taken).0
}

pub fn expected_route(observed_action_taken: &str) -> Vec<ReducerRouteEvent> {
    expected_for_action(observed_action_taken).1
}

pub fn projection_payload(witness: &ConditionRiderRouteWitness) -> String {
    [
        format!(
            "conditionEffect={}",
            condition_effect_ref(witness.condition_effect)
        ),
        format!("rejectedByImmunity={}", witness.rejected_by_immunity),
        format!("activeAfterHost={}", witness.active_after_host),
        format!("activeAfterAdmission={}", witness.active_after_admission),
        format!("expired={}", witness.expired),
        format!("cleanedUp={}", witness.cleaned_up),
        format!(
            "repeatSaveFrontierOpened={}",
            witness.repeat_save_frontier_opened
        ),
        format!(
            "escapeCheckFrontierOpened={}",
            witness.escape_check_frontier_opened
        ),
        format!(
            "transitionedToUnconscious={}",
            witness.transitioned_to_unconscious
        ),
        format!("facts={}", facts_payload(&witness.facts)),
    ]
    .join("\n")
}

fn replay_observed_state_and_route(
    observed_action_taken: &str,
) -> (ConditionRiderRouteWitness, Vec<ReducerRouteEvent>) {
    let mut trace = BattleEntrypointTrace::default();
    let mut setup = condition_rider_setup();
    if rejected_action(observed_action_taken) {
        setup.skeleton.condition_immunities =
            BattleConditionImmunities::only(seed_condition(observed_action_taken));
    }
    let state = start_battle_observed(setup, &mut trace).state;
    let (state, active_after_host) = match host_kind(observed_action_taken) {
        HostKind::AttackHit => {
            let state = resolve_attack_hit_host(state, &mut trace);
            let active = condition_effect(&state).is_present();
            (state, active)
        }
        HostKind::FailedSave => {
            let state = resolve_failed_save_host(
                state,
                seed_condition(observed_action_taken),
                seed_boundary(observed_action_taken),
                &mut trace,
            );
            let active = condition_effect(&state).is_present();
            (state, active)
        }
    };

    let mut state = if rejected_action(observed_action_taken) {
        resolve_condition_rider_without_fill(
            state,
            BattleSubjectKind::ConditionRiderConditionLifecycleRejection,
            &mut trace,
        )
    } else {
        let state = resolve_condition_rider_without_fill(
            state,
            BattleSubjectKind::ConditionRiderConditionLifecycleAdmission,
            &mut trace,
        );
        resolve_condition_rider_without_fill(
            state,
            BattleSubjectKind::ConditionRiderActiveEffectAdmission,
            &mut trace,
        )
    };
    let active_after_admission = condition_effect(&state).is_present();

    match observed_action_taken {
        "doExpireAttackHitPoisonConditionRider"
        | "doExpireFailedSaveBlindedNextTurnConditionRider" => {
            state = resolve_condition_rider_without_fill(
                state,
                BattleSubjectKind::ConditionRiderDurationExpiry,
                &mut trace,
            );
            state = resolve_condition_rider_without_fill(
                state,
                BattleSubjectKind::ConditionRiderConditionLifecycleCleanup,
                &mut trace,
            );
            state = resolve_condition_rider_without_fill(
                state,
                BattleSubjectKind::ConditionRiderActiveEffectCleanup,
                &mut trace,
            );
        }
        "doOpenFailedSaveConditionRepeatSaveFrontier" => {
            state = open_condition_rider_frontier(
                state,
                BattleSubjectKind::ConditionRiderRepeatSave,
                &mut trace,
            );
        }
        "doResolveFailedSaveConditionRepeatSaveSuccessCleanup" => {
            state = open_condition_rider_frontier(
                state,
                BattleSubjectKind::ConditionRiderRepeatSave,
                &mut trace,
            );
            state = resolve_condition_rider_with_fill(
                state,
                BattleSubjectKind::ConditionRiderRepeatSaveSuccess,
                BattleGenericRouteFill::SavingThrowOutcome,
                &mut trace,
            );
            state = resolve_condition_rider_without_fill(
                state,
                BattleSubjectKind::ConditionRiderActiveEffectCleanup,
                &mut trace,
            );
        }
        "doOpenRestrainedAthleticsEscapeFrontier" => {
            state = open_condition_rider_frontier(
                state,
                BattleSubjectKind::ConditionRiderEscapeCheck,
                &mut trace,
            );
        }
        "doResolveRestrainedAthleticsEscapeSuccessCleanup" => {
            state = open_condition_rider_frontier(
                state,
                BattleSubjectKind::ConditionRiderEscapeCheck,
                &mut trace,
            );
            state = resolve_condition_rider_with_fill(
                state,
                BattleSubjectKind::ConditionRiderEscapeCheckSuccess,
                BattleGenericRouteFill::AbilityCheck,
                &mut trace,
            );
            state = resolve_condition_rider_without_fill(
                state,
                BattleSubjectKind::ConditionRiderConditionLifecycleCleanup,
                &mut trace,
            );
            state = resolve_condition_rider_without_fill(
                state,
                BattleSubjectKind::ConditionRiderActiveEffectCleanup,
                &mut trace,
            );
        }
        "doOpenSleepRepeatSaveFrontier" => {
            state = open_condition_rider_frontier(
                state,
                BattleSubjectKind::ConditionRiderRepeatSave,
                &mut trace,
            );
        }
        "doResolveSleepRepeatSaveFailureTransitionToUnconscious" => {
            state = open_condition_rider_frontier(
                state,
                BattleSubjectKind::ConditionRiderRepeatSave,
                &mut trace,
            );
            state = resolve_condition_rider_with_fill(
                state,
                BattleSubjectKind::ConditionRiderRepeatSaveFailureTransition,
                BattleGenericRouteFill::SavingThrowOutcome,
                &mut trace,
            );
            state = resolve_condition_rider_without_fill(
                state,
                BattleSubjectKind::ConditionRiderActiveEffectAdmission,
                &mut trace,
            );
        }
        _ => {}
    }

    let effect = condition_effect(&state);
    (
        ConditionRiderRouteWitness {
            condition_effect: effect,
            rejected_by_immunity: rejected_action(observed_action_taken),
            active_after_host,
            active_after_admission,
            expired: matches!(effect, BattleConditionRiderEffect::Expired { .. }),
            cleaned_up: effect == BattleConditionRiderEffect::Inactive
                && terminal_cleanup_action(observed_action_taken),
            repeat_save_frontier_opened: repeat_frontier_action(observed_action_taken),
            escape_check_frontier_opened: escape_frontier_action(observed_action_taken),
            transitioned_to_unconscious: effect
                .active_condition(BattleConditionRiderCondition::Unconscious),
            facts: expected_facts(observed_action_taken),
        },
        observed_reducer_route(
            &trace,
            &[
                ReducerRouteSubjectFamily::SpellAttack,
                ReducerRouteSubjectFamily::SaveGatedSpell,
                ReducerRouteSubjectFamily::ConditionRider,
            ],
        ),
    )
}

fn resolve_attack_hit_host(state: BattleState, trace: &mut BattleEntrypointTrace) -> BattleState {
    let (state, subject) = discover_spell_attack_damage_subject_observed(
        state,
        true,
        BattleSpellActiveEffectKind::ConditionRider {
            condition: BattleConditionRiderCondition::Poisoned,
            boundary: BattleConditionRiderBoundary::UntilSourceNextTurnEnds,
        },
        trace,
    );
    let result = resolve_battle_subject_observed(
        state,
        BattleResolutionRequest::spell_attack(
            subject,
            BattleSpellAttackFill::TargetChoice(Actor::Skeleton),
        )
        .expect("condition-rider spell attack should accept target choice"),
        trace,
    );
    let (state, subject) = continuation(result, "target choice");
    let result = resolve_battle_subject_observed(
        state,
        BattleResolutionRequest::spell_attack(
            subject,
            BattleSpellAttackFill::AttackRoll(AttackRollFacts {
                total: 20,
                natural_d20: 15,
            }),
        )
        .expect("condition-rider spell attack should accept attack roll"),
        trace,
    );
    let (state, subject) = continuation(result, "attack roll");
    let result = resolve_battle_subject_observed(
        state,
        BattleResolutionRequest::spell_attack(subject, BattleSpellAttackFill::DamageRoll(4))
            .expect("condition-rider spell attack should accept damage roll"),
        trace,
    );
    resolved_state(result, "damage roll")
}

fn resolve_failed_save_host(
    state: BattleState,
    condition: BattleConditionRiderCondition,
    boundary: BattleConditionRiderBoundary,
    trace: &mut BattleEntrypointTrace,
) -> BattleState {
    let (state, subject) = discover_save_gated_condition_rider_subject_observed(
        state,
        BattleSpellActiveEffectKind::ConditionRider {
            condition,
            boundary,
        },
        trace,
    );
    let result = resolve_battle_subject_observed(
        state,
        BattleResolutionRequest::generic_route(subject, BattleGenericRouteFill::TargetChoice)
            .expect("save-gated condition rider should accept target choice"),
        trace,
    );
    let state = continuation(result, "save-gated target choice").0;
    let subject = save_gated_condition_rider_subject_for_target(
        &state,
        BattleSubjectKind::SaveGatedConditionRiderSavingThrow,
        Actor::Skeleton,
    );
    let result = resolve_battle_subject_observed(
        state,
        BattleResolutionRequest::generic_route(subject, BattleGenericRouteFill::SavingThrowOutcome)
            .expect("save-gated condition rider should accept saving throw"),
        trace,
    );
    resolved_state(result, "save-gated saving throw")
}

fn resolve_condition_rider_without_fill(
    state: BattleState,
    kind: BattleSubjectKind,
    trace: &mut BattleEntrypointTrace,
) -> BattleState {
    resolve_condition_rider_with_fill(state, kind, BattleGenericRouteFill::WithoutFill, trace)
}

fn resolve_condition_rider_with_fill(
    state: BattleState,
    kind: BattleSubjectKind,
    fill: BattleGenericRouteFill,
    trace: &mut BattleEntrypointTrace,
) -> BattleState {
    let subject = condition_rider_route_subject_for_target(&state, kind, Actor::Skeleton);
    let result = resolve_battle_subject_observed(
        state,
        BattleResolutionRequest::generic_route(subject, fill)
            .expect("condition-rider route subject should accept generic fill"),
        trace,
    );
    resolved_state(result, "condition-rider route")
}

fn open_condition_rider_frontier(
    state: BattleState,
    kind: BattleSubjectKind,
    trace: &mut BattleEntrypointTrace,
) -> BattleState {
    discover_condition_rider_route_subject_observed(state, kind, Actor::Skeleton, trace).0
}

fn continuation(result: BattleResolutionResult, context: &str) -> (BattleState, BattleSubject) {
    result
        .into_needs_holes()
        .map(|needs| (needs.state, needs.subject))
        .unwrap_or_else(|| panic!("{context} should continue"))
}

fn resolved_state(result: BattleResolutionResult, context: &str) -> BattleState {
    let BattleResolutionResult::Resolved { state } = result else {
        panic!("{context} should resolve")
    };
    state
}

fn condition_rider_setup() -> BattleSetup {
    let mut setup = BattleSetup::standard();
    setup.initiative.still_to_act.waiting = vec![Actor::Skeleton];
    setup.skeleton.hp = 12;
    setup.skeleton.max_hp = 12;
    setup.skeleton.armor_class = 10;
    setup
}

fn condition_effect(state: &BattleState) -> BattleConditionRiderEffect {
    state.skeleton.spell_active_effects.condition_rider
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum HostKind {
    AttackHit,
    FailedSave,
}

fn host_kind(action: &str) -> HostKind {
    if action.contains("AttackHit") {
        HostKind::AttackHit
    } else {
        HostKind::FailedSave
    }
}

fn seed_condition(action: &str) -> BattleConditionRiderCondition {
    match action {
        action if action.contains("Poison") => BattleConditionRiderCondition::Poisoned,
        action if action.contains("FailedSaveConditionRepeatSave") => {
            BattleConditionRiderCondition::Blinded
        }
        action if action.contains("Deafened") => BattleConditionRiderCondition::Deafened,
        action if action.contains("Restrained") => BattleConditionRiderCondition::Restrained,
        action if action.contains("Sleep") => BattleConditionRiderCondition::Incapacitated,
        action if action.contains("Blinded") => BattleConditionRiderCondition::Blinded,
        action => panic!("unsupported condition-rider action {action}"),
    }
}

fn seed_boundary(action: &str) -> BattleConditionRiderBoundary {
    match action {
        action if action.contains("Poison") || action.contains("BlindedNextTurn") => {
            BattleConditionRiderBoundary::UntilSourceNextTurnEnds
        }
        action if action.contains("FailedSaveConditionRepeatSave") => {
            BattleConditionRiderBoundary::AffectedTargetEndTurnRepeatSave
        }
        action if action.contains("RepeatSave") && !action.contains("Sleep") => {
            BattleConditionRiderBoundary::AffectedTargetEndTurnRepeatSave
        }
        action if action.contains("Restrained") => BattleConditionRiderBoundary::UntilSpellEnds,
        action if action.contains("Sleep") => {
            BattleConditionRiderBoundary::UntilAffectedTargetNextTurnEnds
        }
        action if action.contains("Blinded") => {
            BattleConditionRiderBoundary::UntilSourceNextTurnEnds
        }
        action => panic!("unsupported condition-rider action {action}"),
    }
}

fn rejected_action(action: &str) -> bool {
    action.contains("Reject")
}

fn terminal_cleanup_action(action: &str) -> bool {
    action.contains("Expire")
        || action.contains("SuccessCleanup")
        || action.contains("EscapeSuccessCleanup")
}

fn repeat_frontier_action(action: &str) -> bool {
    action.contains("RepeatSaveFrontier")
        || action.contains("RepeatSaveSuccess")
        || action.contains("SleepRepeatSave")
}

fn escape_frontier_action(action: &str) -> bool {
    action.contains("EscapeFrontier") || action.contains("EscapeSuccess")
}

fn expected_for_action(action: &str) -> (ConditionRiderRouteWitness, Vec<ReducerRouteEvent>) {
    let mut route = vec![route_start_battle(ReducerRouteOwnerGroup::ActionEconomy)];
    route.extend(match host_kind(action) {
        HostKind::AttackHit => attack_hit_host_route(),
        HostKind::FailedSave => failed_save_host_route(),
    });
    let mut facts = host_facts(action);
    let mut effect = if rejected_action(action) {
        route.extend(condition_rejection_route());
        facts.push(ConditionRiderRouteFact::ConditionRejectedByImmunity);
        BattleConditionRiderEffect::Inactive
    } else {
        route.extend(condition_admission_route());
        facts.push(ConditionRiderRouteFact::ConditionApplied);
        facts.push(ConditionRiderRouteFact::Boundary(seed_boundary(action)));
        BattleConditionRiderEffect::Active {
            condition: seed_condition(action),
            boundary: seed_boundary(action),
        }
    };

    match action {
        "doExpireAttackHitPoisonConditionRider"
        | "doExpireFailedSaveBlindedNextTurnConditionRider" => {
            route.extend(duration_cleanup_route());
            facts.extend(duration_cleanup_facts());
            effect = BattleConditionRiderEffect::Inactive;
        }
        "doOpenFailedSaveConditionRepeatSaveFrontier" | "doOpenSleepRepeatSaveFrontier" => {
            route.extend(repeat_save_frontier_route());
            facts.push(ConditionRiderRouteFact::RepeatSaveFrontierOpened);
            effect = BattleConditionRiderEffect::RepeatSaveFrontier {
                condition: seed_condition(action),
                boundary: seed_boundary(action),
            };
        }
        "doResolveFailedSaveConditionRepeatSaveSuccessCleanup" => {
            route.extend(repeat_save_frontier_route());
            route.extend(repeat_save_success_cleanup_route());
            facts.push(ConditionRiderRouteFact::RepeatSaveFrontierOpened);
            facts.extend(repeat_save_success_cleanup_facts());
            effect = BattleConditionRiderEffect::Inactive;
        }
        "doOpenRestrainedAthleticsEscapeFrontier" => {
            route.extend(escape_frontier_route());
            facts.push(ConditionRiderRouteFact::EscapeCheckFrontierOpened);
            effect = BattleConditionRiderEffect::EscapeCheckFrontier {
                condition: BattleConditionRiderCondition::Restrained,
                boundary: BattleConditionRiderBoundary::UntilSpellEnds,
            };
        }
        "doResolveRestrainedAthleticsEscapeSuccessCleanup" => {
            route.extend(escape_frontier_route());
            route.extend(escape_success_cleanup_route());
            facts.push(ConditionRiderRouteFact::EscapeCheckFrontierOpened);
            facts.extend(escape_success_cleanup_facts());
            effect = BattleConditionRiderEffect::Inactive;
        }
        "doResolveSleepRepeatSaveFailureTransitionToUnconscious" => {
            route.extend(repeat_save_frontier_route());
            route.extend(sleep_repeat_save_failure_transition_route());
            facts.push(ConditionRiderRouteFact::RepeatSaveFrontierOpened);
            facts.extend(sleep_failure_transition_facts());
            effect = BattleConditionRiderEffect::Active {
                condition: BattleConditionRiderCondition::Unconscious,
                boundary: BattleConditionRiderBoundary::UntilSpellEnds,
            };
        }
        _ => {}
    }

    (
        ConditionRiderRouteWitness {
            condition_effect: effect,
            rejected_by_immunity: rejected_action(action),
            active_after_host: !rejected_action(action),
            active_after_admission: !rejected_action(action),
            expired: false,
            cleaned_up: effect == BattleConditionRiderEffect::Inactive
                && terminal_cleanup_action(action),
            repeat_save_frontier_opened: repeat_frontier_action(action),
            escape_check_frontier_opened: escape_frontier_action(action),
            transitioned_to_unconscious: effect
                .active_condition(BattleConditionRiderCondition::Unconscious),
            facts,
        },
        route,
    )
}

fn attack_hit_host_route() -> Vec<ReducerRouteEvent> {
    vec![
        route_discover_battle_acts(
            ReducerRouteSubjectFamily::SpellAttack,
            vec![crate::rules::battle_reducer_spine::BattleHoleKind::TargetChoice],
            ReducerRouteOwnerGroup::ActionEconomy,
        ),
        route_resolve_battle_subject(
            ReducerRouteSubjectFamily::SpellAttack,
            ReducerRouteFillKind::TargetChoice,
            vec![crate::rules::battle_reducer_spine::BattleHoleKind::AttackRoll],
            ReducerRouteOwnerGroup::TargetSelection,
        ),
        route_resolve_battle_subject(
            ReducerRouteSubjectFamily::SpellAttack,
            ReducerRouteFillKind::AttackRoll,
            vec![crate::rules::battle_reducer_spine::BattleHoleKind::RolledDice],
            ReducerRouteOwnerGroup::AttackRoll,
        ),
        route_resolve_battle_subject(
            ReducerRouteSubjectFamily::SpellAttack,
            ReducerRouteFillKind::RolledDice,
            Vec::new(),
            ReducerRouteOwnerGroup::HitPoint,
        ),
    ]
}

fn failed_save_host_route() -> Vec<ReducerRouteEvent> {
    vec![
        route_discover_battle_acts(
            ReducerRouteSubjectFamily::SaveGatedSpell,
            vec![crate::rules::battle_reducer_spine::BattleHoleKind::TargetChoice],
            ReducerRouteOwnerGroup::SpellSlotAndActionEconomy,
        ),
        route_resolve_battle_subject(
            ReducerRouteSubjectFamily::SaveGatedSpell,
            ReducerRouteFillKind::TargetChoice,
            vec![crate::rules::battle_reducer_spine::BattleHoleKind::SavingThrowOutcome],
            ReducerRouteOwnerGroup::TargetSelection,
        ),
        route_resolve_battle_subject(
            ReducerRouteSubjectFamily::SaveGatedSpell,
            ReducerRouteFillKind::SavingThrowOutcome,
            Vec::new(),
            ReducerRouteOwnerGroup::SavingThrowOutcome,
        ),
    ]
}

fn condition_admission_route() -> Vec<ReducerRouteEvent> {
    vec![
        route_resolve_battle_subject_without_fill(
            ReducerRouteSubjectFamily::ConditionRider,
            Vec::new(),
            ReducerRouteOwnerGroup::ConditionLifecycle,
        ),
        route_resolve_battle_subject_without_fill(
            ReducerRouteSubjectFamily::ConditionRider,
            Vec::new(),
            ReducerRouteOwnerGroup::ActiveEffect,
        ),
    ]
}

fn condition_rejection_route() -> Vec<ReducerRouteEvent> {
    vec![route_resolve_battle_subject_without_fill(
        ReducerRouteSubjectFamily::ConditionRider,
        Vec::new(),
        ReducerRouteOwnerGroup::ConditionLifecycle,
    )]
}

fn duration_cleanup_route() -> Vec<ReducerRouteEvent> {
    vec![
        route_resolve_battle_subject_without_fill(
            ReducerRouteSubjectFamily::ConditionRider,
            Vec::new(),
            ReducerRouteOwnerGroup::TurnBoundary,
        ),
        route_resolve_battle_subject_without_fill(
            ReducerRouteSubjectFamily::ConditionRider,
            Vec::new(),
            ReducerRouteOwnerGroup::ConditionLifecycle,
        ),
        route_resolve_battle_subject_without_fill(
            ReducerRouteSubjectFamily::ConditionRider,
            Vec::new(),
            ReducerRouteOwnerGroup::ActiveEffect,
        ),
    ]
}

fn repeat_save_frontier_route() -> Vec<ReducerRouteEvent> {
    vec![route_discover_battle_acts(
        ReducerRouteSubjectFamily::ConditionRider,
        vec![crate::rules::battle_reducer_spine::BattleHoleKind::SavingThrowOutcome],
        ReducerRouteOwnerGroup::TurnBoundary,
    )]
}

fn repeat_save_success_cleanup_route() -> Vec<ReducerRouteEvent> {
    vec![
        route_resolve_battle_subject(
            ReducerRouteSubjectFamily::ConditionRider,
            ReducerRouteFillKind::SavingThrowOutcome,
            Vec::new(),
            ReducerRouteOwnerGroup::ConditionLifecycle,
        ),
        route_resolve_battle_subject_without_fill(
            ReducerRouteSubjectFamily::ConditionRider,
            Vec::new(),
            ReducerRouteOwnerGroup::ActiveEffect,
        ),
    ]
}

fn escape_frontier_route() -> Vec<ReducerRouteEvent> {
    vec![route_discover_battle_acts(
        ReducerRouteSubjectFamily::ConditionRider,
        vec![crate::rules::battle_reducer_spine::BattleHoleKind::AbilityCheck],
        ReducerRouteOwnerGroup::AbilityCheck,
    )]
}

fn escape_success_cleanup_route() -> Vec<ReducerRouteEvent> {
    vec![
        route_resolve_battle_subject(
            ReducerRouteSubjectFamily::ConditionRider,
            ReducerRouteFillKind::AbilityCheck,
            Vec::new(),
            ReducerRouteOwnerGroup::AbilityCheck,
        ),
        route_resolve_battle_subject_without_fill(
            ReducerRouteSubjectFamily::ConditionRider,
            Vec::new(),
            ReducerRouteOwnerGroup::ConditionLifecycle,
        ),
        route_resolve_battle_subject_without_fill(
            ReducerRouteSubjectFamily::ConditionRider,
            Vec::new(),
            ReducerRouteOwnerGroup::ActiveEffect,
        ),
    ]
}

fn sleep_repeat_save_failure_transition_route() -> Vec<ReducerRouteEvent> {
    vec![
        route_resolve_battle_subject(
            ReducerRouteSubjectFamily::ConditionRider,
            ReducerRouteFillKind::SavingThrowOutcome,
            Vec::new(),
            ReducerRouteOwnerGroup::ConditionLifecycle,
        ),
        route_resolve_battle_subject_without_fill(
            ReducerRouteSubjectFamily::ConditionRider,
            Vec::new(),
            ReducerRouteOwnerGroup::ActiveEffect,
        ),
    ]
}

fn host_facts(action: &str) -> Vec<ConditionRiderRouteFact> {
    let mut facts = vec![
        match host_kind(action) {
            HostKind::AttackHit => ConditionRiderRouteFact::HostAttackHitDamageOutcome,
            HostKind::FailedSave => ConditionRiderRouteFact::HostFailedSavingThrowOutcome,
        },
        ConditionRiderRouteFact::AffectedTargetConditionCarrier,
        ConditionRiderRouteFact::Condition(seed_condition(action)),
    ];
    if action.contains("Restrained") {
        facts.push(ConditionRiderRouteFact::AthleticsActionEscapeCheck);
    }
    facts
}

fn expected_facts(action: &str) -> Vec<ConditionRiderRouteFact> {
    expected_for_action(action).0.facts
}

fn duration_cleanup_facts() -> Vec<ConditionRiderRouteFact> {
    vec![
        ConditionRiderRouteFact::Expired,
        ConditionRiderRouteFact::CleanedUpBy(ReducerRouteOwnerGroup::TurnBoundary),
        ConditionRiderRouteFact::CleanedUpBy(ReducerRouteOwnerGroup::ConditionLifecycle),
        ConditionRiderRouteFact::CleanedUpBy(ReducerRouteOwnerGroup::ActiveEffect),
    ]
}

fn repeat_save_success_cleanup_facts() -> Vec<ConditionRiderRouteFact> {
    vec![
        ConditionRiderRouteFact::RepeatSaveSucceeded,
        ConditionRiderRouteFact::CleanedUpBy(ReducerRouteOwnerGroup::ConditionLifecycle),
        ConditionRiderRouteFact::CleanedUpBy(ReducerRouteOwnerGroup::ActiveEffect),
    ]
}

fn escape_success_cleanup_facts() -> Vec<ConditionRiderRouteFact> {
    vec![
        ConditionRiderRouteFact::EscapeCheckSucceeded,
        ConditionRiderRouteFact::CleanedUpBy(ReducerRouteOwnerGroup::ConditionLifecycle),
        ConditionRiderRouteFact::CleanedUpBy(ReducerRouteOwnerGroup::ActiveEffect),
    ]
}

fn sleep_failure_transition_facts() -> Vec<ConditionRiderRouteFact> {
    vec![
        ConditionRiderRouteFact::RepeatSaveFailed,
        ConditionRiderRouteFact::Transition(BattleConditionRiderCondition::Unconscious),
        ConditionRiderRouteFact::Boundary(BattleConditionRiderBoundary::UntilSpellEnds),
    ]
}

fn facts_payload(facts: &[ConditionRiderRouteFact]) -> String {
    facts.iter().map(fact_ref).collect::<Vec<_>>().join(",")
}

fn fact_ref(fact: &ConditionRiderRouteFact) -> &'static str {
    match fact {
        ConditionRiderRouteFact::HostAttackHitDamageOutcome => "HostAttackHitDamageOutcome",
        ConditionRiderRouteFact::HostFailedSavingThrowOutcome => "HostFailedSavingThrowOutcome",
        ConditionRiderRouteFact::AffectedTargetConditionCarrier => "AffectedTargetConditionCarrier",
        ConditionRiderRouteFact::Condition(condition) => condition_ref(*condition),
        ConditionRiderRouteFact::ConditionApplied => "ConditionApplied",
        ConditionRiderRouteFact::ConditionRejectedByImmunity => "ConditionRejectedByImmunity",
        ConditionRiderRouteFact::Boundary(boundary) => boundary_ref(*boundary),
        ConditionRiderRouteFact::AthleticsActionEscapeCheck => "AthleticsActionEscapeCheck",
        ConditionRiderRouteFact::RepeatSaveFrontierOpened => "RepeatSaveFrontierOpened",
        ConditionRiderRouteFact::RepeatSaveSucceeded => "RepeatSaveSucceeded",
        ConditionRiderRouteFact::RepeatSaveFailed => "RepeatSaveFailed",
        ConditionRiderRouteFact::Transition(condition) => condition_ref(*condition),
        ConditionRiderRouteFact::EscapeCheckFrontierOpened => "EscapeCheckFrontierOpened",
        ConditionRiderRouteFact::EscapeCheckSucceeded => "EscapeCheckSucceeded",
        ConditionRiderRouteFact::Expired => "Expired",
        ConditionRiderRouteFact::CleanedUpBy(owner) => owner_fact_ref(*owner),
    }
}

fn condition_effect_ref(effect: BattleConditionRiderEffect) -> &'static str {
    match effect {
        BattleConditionRiderEffect::Inactive => "Inactive",
        BattleConditionRiderEffect::Active { condition, .. } => condition_ref(condition),
        BattleConditionRiderEffect::RepeatSaveFrontier { condition, .. } => {
            condition_ref(condition)
        }
        BattleConditionRiderEffect::EscapeCheckFrontier { condition, .. } => {
            condition_ref(condition)
        }
        BattleConditionRiderEffect::Expired { condition, .. } => condition_ref(condition),
        BattleConditionRiderEffect::CleanupReady { condition, .. } => condition_ref(condition),
    }
}

fn condition_ref(condition: BattleConditionRiderCondition) -> &'static str {
    match condition {
        BattleConditionRiderCondition::Poisoned => "PoisonedCondition",
        BattleConditionRiderCondition::Blinded => "BlindedCondition",
        BattleConditionRiderCondition::Deafened => "DeafenedCondition",
        BattleConditionRiderCondition::Restrained => "RestrainedCondition",
        BattleConditionRiderCondition::Incapacitated => "IncapacitatedCondition",
        BattleConditionRiderCondition::Unconscious => "UnconsciousCondition",
    }
}

fn boundary_ref(boundary: BattleConditionRiderBoundary) -> &'static str {
    match boundary {
        BattleConditionRiderBoundary::UntilSourceNextTurnEnds => "UntilSourceNextTurnEnds",
        BattleConditionRiderBoundary::UntilAffectedTargetNextTurnEnds => {
            "UntilAffectedTargetNextTurnEnds"
        }
        BattleConditionRiderBoundary::UntilSpellEnds => "UntilSpellEnds",
        BattleConditionRiderBoundary::AffectedTargetEndTurnRepeatSave => {
            "AffectedTargetEndTurnRepeatSave"
        }
    }
}

fn owner_fact_ref(owner: ReducerRouteOwnerGroup) -> &'static str {
    match owner {
        ReducerRouteOwnerGroup::TurnBoundary => "BattleTurnBoundaryCleanupOwner",
        ReducerRouteOwnerGroup::ConditionLifecycle => "BattleConditionLifecycleCleanupOwner",
        ReducerRouteOwnerGroup::ActiveEffect => "BattleActiveEffectCleanupOwner",
        _ => "BattleOtherCleanupOwner",
    }
}
