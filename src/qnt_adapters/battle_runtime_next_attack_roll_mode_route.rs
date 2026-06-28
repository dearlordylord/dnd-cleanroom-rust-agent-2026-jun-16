use crate::rules::battle_reducer_spine::{
    discover_generic_route_subject_observed, discover_spell_attack_damage_subject_observed,
    next_attack_roll_mode_route_subject_for_target, resolve_battle_subject_observed,
    save_gated_next_attack_roll_mode_subject_for_target, start_battle_observed, Actor,
    AttackRollFacts, BattleEntrypointTrace, BattleGenericRouteFill, BattleNextAttackRollModeEffect,
    BattleResolutionRequest, BattleResolutionResult, BattleSetup, BattleSpellActiveEffectKind,
    BattleSpellAttackFill, BattleState, BattleSubject, BattleSubjectKind,
};

use super::battle_runtime_reducer_route::{
    observed_reducer_route, route_discover_battle_acts, route_resolve_battle_subject,
    route_resolve_battle_subject_without_fill, route_start_battle, ReducerRouteEvent,
    ReducerRouteFillKind, ReducerRouteOwnerGroup, ReducerRouteSubjectFamily,
};

pub const CONNECTOR_ACTIONS: [&str; 6] = [
    "doAdmitHostAttackHitNextAttackAdvantageEffect",
    "doAdmitHostEffectNextAttackDisadvantageEffect",
    "doProjectAdvantageOnLaterAttackAgainstAffectedTarget",
    "doProjectDisadvantageOnAffectedTargetNextAttack",
    "doExpireAdvantageAtBoundary",
    "doExpireDisadvantageAtBoundary",
];

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NextAttackRollModeDirection {
    Advantage,
    Disadvantage,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NextAttackRollModeRouteWitness {
    pub direction: NextAttackRollModeDirection,
    pub active_after_admission: bool,
    pub projected: bool,
    pub consumed_after_projection: bool,
    pub expired_at_boundary: bool,
    pub active_after_cleanup: bool,
}

pub fn replay_observed_action(observed_action_taken: &str) -> NextAttackRollModeRouteWitness {
    replay_observed_state_and_route(observed_action_taken).0
}

pub fn replay_observed_route(observed_action_taken: &str) -> Vec<ReducerRouteEvent> {
    replay_observed_state_and_route(observed_action_taken).1
}

pub fn expected_witness(observed_action_taken: &str) -> NextAttackRollModeRouteWitness {
    match observed_action_taken {
        "doAdmitHostAttackHitNextAttackAdvantageEffect" => {
            admission_witness(NextAttackRollModeDirection::Advantage)
        }
        "doAdmitHostEffectNextAttackDisadvantageEffect" => {
            admission_witness(NextAttackRollModeDirection::Disadvantage)
        }
        "doProjectAdvantageOnLaterAttackAgainstAffectedTarget" => {
            projected_witness(NextAttackRollModeDirection::Advantage)
        }
        "doProjectDisadvantageOnAffectedTargetNextAttack" => {
            projected_witness(NextAttackRollModeDirection::Disadvantage)
        }
        "doExpireAdvantageAtBoundary" => expired_witness(NextAttackRollModeDirection::Advantage),
        "doExpireDisadvantageAtBoundary" => {
            expired_witness(NextAttackRollModeDirection::Disadvantage)
        }
        action => panic!("unsupported route connector action {action}"),
    }
}

pub fn expected_route(observed_action_taken: &str) -> Vec<ReducerRouteEvent> {
    match observed_action_taken {
        "doAdmitHostAttackHitNextAttackAdvantageEffect" => advantage_admission_route(),
        "doAdmitHostEffectNextAttackDisadvantageEffect" => disadvantage_admission_route(),
        "doProjectAdvantageOnLaterAttackAgainstAffectedTarget" => {
            let mut route = advantage_admission_route();
            route.extend(project_and_cleanup_route());
            route
        }
        "doProjectDisadvantageOnAffectedTargetNextAttack" => {
            let mut route = disadvantage_admission_route();
            route.extend(project_and_cleanup_route());
            route
        }
        "doExpireAdvantageAtBoundary" => {
            let mut route = advantage_admission_route();
            route.extend(expire_and_cleanup_route());
            route
        }
        "doExpireDisadvantageAtBoundary" => {
            let mut route = disadvantage_admission_route();
            route.extend(expire_and_cleanup_route());
            route
        }
        action => panic!("unsupported route connector action {action}"),
    }
}

pub fn projection_payload(witness: &NextAttackRollModeRouteWitness) -> String {
    [
        format!("direction={}", direction_ref(witness.direction)),
        format!("activeAfterAdmission={}", witness.active_after_admission),
        format!("projected={}", witness.projected),
        format!(
            "consumedAfterProjection={}",
            witness.consumed_after_projection
        ),
        format!("expiredAtBoundary={}", witness.expired_at_boundary),
        format!("activeAfterCleanup={}", witness.active_after_cleanup),
    ]
    .join("\n")
}

fn replay_observed_state_and_route(
    observed_action_taken: &str,
) -> (NextAttackRollModeRouteWitness, Vec<ReducerRouteEvent>) {
    let mut trace = BattleEntrypointTrace::default();
    let state = start_battle_observed(next_attack_roll_mode_setup(), &mut trace).state;
    let (state, direction) = match observed_action_taken {
        "doAdmitHostAttackHitNextAttackAdvantageEffect"
        | "doProjectAdvantageOnLaterAttackAgainstAffectedTarget"
        | "doExpireAdvantageAtBoundary" => {
            let state = admit_advantage_effect(state, &mut trace);
            (state, NextAttackRollModeDirection::Advantage)
        }
        "doAdmitHostEffectNextAttackDisadvantageEffect"
        | "doProjectDisadvantageOnAffectedTargetNextAttack"
        | "doExpireDisadvantageAtBoundary" => {
            let state = admit_disadvantage_effect(state, &mut trace);
            (state, NextAttackRollModeDirection::Disadvantage)
        }
        action => panic!("unsupported route connector action {action}"),
    };
    let active_after_admission = effect_for_direction(&state, direction).projects_roll_mode();

    let (state, projected, consumed_after_projection, expired_at_boundary) =
        match observed_action_taken {
            "doProjectAdvantageOnLaterAttackAgainstAffectedTarget"
            | "doProjectDisadvantageOnAffectedTargetNextAttack" => {
                let state = resolve_next_attack_roll_without_fill(
                    state,
                    BattleSubjectKind::NextAttackRollModeProjection,
                    &mut trace,
                );
                let consumed = effect_for_direction(&state, direction)
                    == BattleNextAttackRollModeEffect::Consumed;
                (state, true, consumed, false)
            }
            "doExpireAdvantageAtBoundary" | "doExpireDisadvantageAtBoundary" => {
                let state = resolve_next_attack_roll_without_fill(
                    state,
                    BattleSubjectKind::NextAttackRollModeDurationExpiry,
                    &mut trace,
                );
                let expired = effect_for_direction(&state, direction)
                    == BattleNextAttackRollModeEffect::Expired;
                (state, false, false, expired)
            }
            _ => (state, false, false, false),
        };

    let state = if consumed_after_projection || expired_at_boundary {
        resolve_next_attack_roll_without_fill(
            state,
            BattleSubjectKind::NextAttackRollModeActiveEffectCleanup,
            &mut trace,
        )
    } else {
        state
    };

    (
        NextAttackRollModeRouteWitness {
            direction,
            active_after_admission,
            projected,
            consumed_after_projection,
            expired_at_boundary,
            active_after_cleanup: effect_for_direction(&state, direction).projects_roll_mode(),
        },
        observed_reducer_route(
            &trace,
            &[
                ReducerRouteSubjectFamily::SpellAttack,
                ReducerRouteSubjectFamily::SaveGatedSpell,
                ReducerRouteSubjectFamily::NextAttackRollMode,
            ],
        ),
    )
}

fn admit_advantage_effect(state: BattleState, trace: &mut BattleEntrypointTrace) -> BattleState {
    let (state, subject) = discover_spell_attack_damage_subject_observed(
        state,
        true,
        BattleSpellActiveEffectKind::NextAttackRollAgainstSelfAdvantage,
        trace,
    );
    let result = resolve_battle_subject_observed(
        state,
        BattleResolutionRequest::spell_attack(
            subject,
            BattleSpellAttackFill::TargetChoice(Actor::Skeleton),
        )
        .expect("spell attack subject should accept target choice"),
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
        .expect("spell attack subject should accept attack roll"),
        trace,
    );
    let (state, subject) = continuation(result, "attack roll");
    let result = resolve_battle_subject_observed(
        state,
        BattleResolutionRequest::spell_attack(subject, BattleSpellAttackFill::DamageRoll(4))
            .expect("spell attack subject should accept damage roll"),
        trace,
    );
    let state = resolved_state(result, "damage roll");
    resolve_next_attack_roll_without_fill(
        state,
        BattleSubjectKind::NextAttackRollModeActiveEffectAdmission,
        trace,
    )
}

fn admit_disadvantage_effect(state: BattleState, trace: &mut BattleEntrypointTrace) -> BattleState {
    let (state, subject) = discover_generic_route_subject_observed(
        state,
        BattleSubjectKind::SaveGatedNextAttackRollModeTargetChoice,
        trace,
    );
    let result = resolve_battle_subject_observed(
        state,
        BattleResolutionRequest::generic_route(subject, BattleGenericRouteFill::TargetChoice)
            .expect("save-gated next-roll mode subject should accept target choice"),
        trace,
    );
    let state = continuation(result, "save-gated target choice").0;
    let subject = save_gated_next_attack_roll_mode_subject_for_target(
        &state,
        BattleSubjectKind::SaveGatedNextAttackRollModeSavingThrow,
        Actor::Skeleton,
    );
    let result = resolve_battle_subject_observed(
        state,
        BattleResolutionRequest::generic_route(subject, BattleGenericRouteFill::SavingThrowOutcome)
            .expect("save-gated next-roll mode subject should accept save outcome"),
        trace,
    );
    let state = resolved_state(result, "save-gated saving throw outcome");
    resolve_next_attack_roll_without_fill(
        state,
        BattleSubjectKind::NextAttackRollModeActiveEffectAdmission,
        trace,
    )
}

fn resolve_next_attack_roll_without_fill(
    state: BattleState,
    kind: BattleSubjectKind,
    trace: &mut BattleEntrypointTrace,
) -> BattleState {
    let subject = next_attack_roll_mode_route_subject_for_target(&state, kind, Actor::Skeleton);
    let result = resolve_battle_subject_observed(
        state,
        BattleResolutionRequest::generic_route(subject, BattleGenericRouteFill::WithoutFill)
            .expect("next-attack-roll mode lifecycle subject should accept without-fill"),
        trace,
    );
    resolved_state(result, "next-attack-roll mode lifecycle")
}

fn effect_for_direction(
    state: &BattleState,
    direction: NextAttackRollModeDirection,
) -> BattleNextAttackRollModeEffect {
    let effects = state.skeleton.spell_active_effects;
    match direction {
        NextAttackRollModeDirection::Advantage => effects.next_attack_roll_against_self_advantage,
        NextAttackRollModeDirection::Disadvantage => effects.next_attack_roll_disadvantage,
    }
}

fn next_attack_roll_mode_setup() -> BattleSetup {
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

fn advantage_admission_route() -> Vec<ReducerRouteEvent> {
    let mut route = vec![
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
    ];
    route.extend(next_attack_roll_admission_route());
    route
}

fn disadvantage_admission_route() -> Vec<ReducerRouteEvent> {
    let mut route = vec![
        route_start_battle(ReducerRouteOwnerGroup::ActionEconomy),
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
    ];
    route.extend(next_attack_roll_admission_route());
    route
}

fn next_attack_roll_admission_route() -> Vec<ReducerRouteEvent> {
    vec![route_resolve_battle_subject_without_fill(
        ReducerRouteSubjectFamily::NextAttackRollMode,
        Vec::new(),
        ReducerRouteOwnerGroup::ActiveEffect,
    )]
}

fn project_and_cleanup_route() -> Vec<ReducerRouteEvent> {
    vec![
        route_resolve_battle_subject_without_fill(
            ReducerRouteSubjectFamily::NextAttackRollMode,
            Vec::new(),
            ReducerRouteOwnerGroup::AttackRollMode,
        ),
        route_resolve_battle_subject_without_fill(
            ReducerRouteSubjectFamily::NextAttackRollMode,
            Vec::new(),
            ReducerRouteOwnerGroup::ActiveEffect,
        ),
    ]
}

fn expire_and_cleanup_route() -> Vec<ReducerRouteEvent> {
    vec![
        route_resolve_battle_subject_without_fill(
            ReducerRouteSubjectFamily::NextAttackRollMode,
            Vec::new(),
            ReducerRouteOwnerGroup::TurnBoundary,
        ),
        route_resolve_battle_subject_without_fill(
            ReducerRouteSubjectFamily::NextAttackRollMode,
            Vec::new(),
            ReducerRouteOwnerGroup::ActiveEffect,
        ),
    ]
}

fn admission_witness(direction: NextAttackRollModeDirection) -> NextAttackRollModeRouteWitness {
    NextAttackRollModeRouteWitness {
        direction,
        active_after_admission: true,
        projected: false,
        consumed_after_projection: false,
        expired_at_boundary: false,
        active_after_cleanup: true,
    }
}

fn projected_witness(direction: NextAttackRollModeDirection) -> NextAttackRollModeRouteWitness {
    NextAttackRollModeRouteWitness {
        direction,
        active_after_admission: true,
        projected: true,
        consumed_after_projection: true,
        expired_at_boundary: false,
        active_after_cleanup: false,
    }
}

fn expired_witness(direction: NextAttackRollModeDirection) -> NextAttackRollModeRouteWitness {
    NextAttackRollModeRouteWitness {
        direction,
        active_after_admission: true,
        projected: false,
        consumed_after_projection: false,
        expired_at_boundary: true,
        active_after_cleanup: false,
    }
}

fn direction_ref(direction: NextAttackRollModeDirection) -> &'static str {
    match direction {
        NextAttackRollModeDirection::Advantage => "NextAttackRollAdvantage",
        NextAttackRollModeDirection::Disadvantage => "NextAttackRollDisadvantage",
    }
}
