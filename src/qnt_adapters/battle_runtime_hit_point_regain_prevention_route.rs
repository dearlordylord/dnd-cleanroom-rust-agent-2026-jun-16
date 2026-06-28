use crate::rules::battle_reducer_spine::{
    discover_spell_attack_damage_subject_observed,
    hit_point_regain_prevention_route_subject_for_target, resolve_battle_subject_observed,
    start_battle_observed, Actor, AttackRollFacts, BattleEntrypointTrace, BattleGenericRouteFill,
    BattleHitPointRegainPreventionEffect, BattleResolutionRequest, BattleResolutionResult,
    BattleSetup, BattleSpellActiveEffectKind, BattleSpellAttackFill, BattleState, BattleSubject,
    BattleSubjectKind,
};

use super::battle_runtime_reducer_route::{
    observed_reducer_route, route_discover_battle_acts, route_resolve_battle_subject,
    route_resolve_battle_subject_without_fill, route_start_battle, ReducerRouteEvent,
    ReducerRouteFillKind, ReducerRouteOwnerGroup, ReducerRouteSubjectFamily,
};

pub const BRANCH_ACTIONS: [&str; 1] = ["doChillTouchHitPointRegainPrevention"];

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HitPointRegainPreventionRouteWitness {
    pub target_hit_points_after_damage: i16,
    pub target_hit_points_after_healing_attempt: i16,
    pub active_after_attack_hit: bool,
    pub active_after_admission: bool,
    pub expired_after_duration: bool,
    pub active_after_cleanup: bool,
}

pub fn replay_observed_action(observed_action_taken: &str) -> HitPointRegainPreventionRouteWitness {
    match observed_action_taken {
        "doChillTouchHitPointRegainPrevention" => replay_observed_state_and_route().0,
        action => panic!("unsupported mbt::actionTaken {action}"),
    }
}

pub fn replay_observed_route(observed_action_taken: &str) -> Vec<ReducerRouteEvent> {
    match observed_action_taken {
        "doChillTouchHitPointRegainPrevention" => replay_observed_state_and_route().1,
        action => panic!("unsupported mbt::actionTaken {action}"),
    }
}

pub fn expected_witness(observed_action_taken: &str) -> HitPointRegainPreventionRouteWitness {
    match observed_action_taken {
        "doChillTouchHitPointRegainPrevention" => HitPointRegainPreventionRouteWitness {
            target_hit_points_after_damage: 8,
            target_hit_points_after_healing_attempt: 8,
            active_after_attack_hit: true,
            active_after_admission: true,
            expired_after_duration: true,
            active_after_cleanup: false,
        },
        action => panic!("unsupported mbt::actionTaken {action}"),
    }
}

pub fn expected_route(observed_action_taken: &str) -> Vec<ReducerRouteEvent> {
    match observed_action_taken {
        "doChillTouchHitPointRegainPrevention" => expected_hit_point_regain_prevention_route(),
        action => panic!("unsupported mbt::actionTaken {action}"),
    }
}

pub fn projection_payload(witness: &HitPointRegainPreventionRouteWitness) -> String {
    [
        format!(
            "targetHitPointsAfterDamage={}",
            witness.target_hit_points_after_damage
        ),
        format!(
            "targetHitPointsAfterHealingAttempt={}",
            witness.target_hit_points_after_healing_attempt
        ),
        format!("activeAfterAttackHit={}", witness.active_after_attack_hit),
        format!("activeAfterAdmission={}", witness.active_after_admission),
        format!("expiredAfterDuration={}", witness.expired_after_duration),
        format!("activeAfterCleanup={}", witness.active_after_cleanup),
    ]
    .join("\n")
}

fn replay_observed_state_and_route(
) -> (HitPointRegainPreventionRouteWitness, Vec<ReducerRouteEvent>) {
    let mut trace = BattleEntrypointTrace::default();
    let setup = hit_point_regain_prevention_setup();
    let state = start_battle_observed(setup, &mut trace).state;
    let (state, subject) = discover_spell_attack_damage_subject_observed(
        state,
        false,
        BattleSpellActiveEffectKind::HitPointRegainPrevented,
        &mut trace,
    );
    let result = resolve_battle_subject_observed(
        state,
        BattleResolutionRequest::spell_attack(
            subject,
            BattleSpellAttackFill::TargetChoice(Actor::Skeleton),
        )
        .expect("spell attack subject should accept target choice"),
        &mut trace,
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
        .expect("spell attack subject should accept attack roll"),
        &mut trace,
    );
    let (state, subject) = continuation(result, "attack roll");
    let result = resolve_battle_subject_observed(
        state,
        BattleResolutionRequest::spell_attack(subject, BattleSpellAttackFill::DamageRoll(4))
            .expect("spell attack subject should accept damage roll"),
        &mut trace,
    );
    let state = resolved_state(result, "damage roll");
    let target_hit_points_after_damage = state.skeleton.hp;
    let active_after_attack_hit = state
        .skeleton
        .spell_active_effects
        .hit_point_regain_prevention
        .prevents_hit_point_regain();

    let result = resolve_hit_point_regain_prevention_without_fill(
        state,
        BattleSubjectKind::HitPointRegainPreventionActiveEffectAdmission,
        &mut trace,
    );
    let state = resolved_state(result, "active-effect admission");
    let active_after_admission = state
        .skeleton
        .spell_active_effects
        .hit_point_regain_prevention
        .prevents_hit_point_regain();

    let subject = hit_point_regain_prevention_subject(
        &state,
        BattleSubjectKind::HitPointRegainPreventionHealingInterdiction,
    );
    let result = resolve_battle_subject_observed(
        state,
        BattleResolutionRequest::generic_route(
            subject,
            BattleGenericRouteFill::HitPointHealingDistribution {
                target: Actor::Skeleton,
                amount: 3,
            },
        )
        .expect("hit-point-regain prevention should accept healing distribution"),
        &mut trace,
    );
    let state = resolved_state(result, "healing interdiction");
    let target_hit_points_after_healing_attempt = state.skeleton.hp;

    let result = resolve_hit_point_regain_prevention_without_fill(
        state,
        BattleSubjectKind::HitPointRegainPreventionDurationExpiry,
        &mut trace,
    );
    let state = resolved_state(result, "duration expiry");
    let expired_after_duration = state
        .skeleton
        .spell_active_effects
        .hit_point_regain_prevention
        == BattleHitPointRegainPreventionEffect::Expired;

    let result = resolve_hit_point_regain_prevention_without_fill(
        state,
        BattleSubjectKind::HitPointRegainPreventionActiveEffectCleanup,
        &mut trace,
    );
    let state = resolved_state(result, "active-effect cleanup");

    (
        HitPointRegainPreventionRouteWitness {
            target_hit_points_after_damage,
            target_hit_points_after_healing_attempt,
            active_after_attack_hit,
            active_after_admission,
            expired_after_duration,
            active_after_cleanup: state
                .skeleton
                .spell_active_effects
                .hit_point_regain_prevention
                .prevents_hit_point_regain(),
        },
        observed_reducer_route(
            &trace,
            &[
                ReducerRouteSubjectFamily::SpellAttack,
                ReducerRouteSubjectFamily::HitPointRegainPrevention,
            ],
        ),
    )
}

fn resolve_hit_point_regain_prevention_without_fill(
    state: BattleState,
    kind: BattleSubjectKind,
    trace: &mut BattleEntrypointTrace,
) -> BattleResolutionResult {
    let subject = hit_point_regain_prevention_subject(&state, kind);
    resolve_battle_subject_observed(
        state,
        BattleResolutionRequest::generic_route(subject, BattleGenericRouteFill::WithoutFill)
            .expect("hit-point-regain prevention without-fill subject should be accepted"),
        trace,
    )
}

fn hit_point_regain_prevention_subject(
    state: &BattleState,
    kind: BattleSubjectKind,
) -> BattleSubject {
    hit_point_regain_prevention_route_subject_for_target(state, kind, Actor::Skeleton)
}

fn hit_point_regain_prevention_setup() -> BattleSetup {
    let mut setup = BattleSetup::standard();
    setup.initiative.still_to_act.waiting = vec![Actor::Skeleton];
    setup.skeleton.hp = 12;
    setup.skeleton.max_hp = 12;
    setup.skeleton.armor_class = 10;
    setup
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

fn expected_hit_point_regain_prevention_route() -> Vec<ReducerRouteEvent> {
    vec![
        route_start_battle(ReducerRouteOwnerGroup::ActionEconomy),
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
        route_resolve_battle_subject_without_fill(
            ReducerRouteSubjectFamily::HitPointRegainPrevention,
            Vec::new(),
            ReducerRouteOwnerGroup::ActiveEffect,
        ),
        route_resolve_battle_subject(
            ReducerRouteSubjectFamily::HitPointRegainPrevention,
            ReducerRouteFillKind::HitPointHealingDistribution,
            Vec::new(),
            ReducerRouteOwnerGroup::HitPoint,
        ),
        route_resolve_battle_subject_without_fill(
            ReducerRouteSubjectFamily::HitPointRegainPrevention,
            Vec::new(),
            ReducerRouteOwnerGroup::TurnBoundary,
        ),
        route_resolve_battle_subject_without_fill(
            ReducerRouteSubjectFamily::HitPointRegainPrevention,
            Vec::new(),
            ReducerRouteOwnerGroup::ActiveEffect,
        ),
    ]
}
