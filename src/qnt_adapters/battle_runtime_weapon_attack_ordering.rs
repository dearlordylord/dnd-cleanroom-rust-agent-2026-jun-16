use crate::rules::battle_reducer_spine::{
    discover_battle_acts, resolve_battle_subject_test_fill, start_battle, Actor, AttackRollFacts,
    BattleFill, BattleHoleKind, BattleResolutionInvalidReason, BattleResolutionOutcome,
    BattleSetup, BattleSubject, BattleSubjectKind,
};
use crate::rules::weapon_attack_ordering::{
    discover_weapon_attack, fill_weapon_attack_damage_dice, fill_weapon_attack_roll_hit,
    fill_weapon_attack_roll_miss, fill_weapon_attack_target_choice,
    reject_weapon_attack_roll_before_target_choice, reject_weapon_damage_before_attack_roll,
    weapon_attack_ordering_projection, WeaponAttackFillOrderingError, WeaponAttackFrontierStage,
    WeaponAttackHoleKind, WeaponAttackInvalidReason, WeaponAttackOrderingProjectionFacts,
    WeaponAttackOrderingProtocol, WeaponAttackOrderingState, WeaponAttackRuntimeResult,
};

use super::battle_runtime_reducer_route::{
    route_discover_battle_acts, route_resolve_battle_subject_from_result,
    route_resolve_battle_subject_from_route_result, route_start_battle, ReducerRouteEvent,
    ReducerRouteFillKind, ReducerRouteHoleKind, ReducerRouteOwnerGroup,
    ReducerRouteResolutionOutcome, ReducerRouteResolveConnector, ReducerRouteResolveFill,
    ReducerRouteSubjectFamily,
};

pub const BRANCH_ACTIONS: [&str; 7] = [
    "doDiscoverAttack",
    "doRejectAttackRollBeforeTargetChoice",
    "doFillTargetChoice",
    "doRejectDamageBeforeAttackRoll",
    "doFillAttackRollMiss",
    "doFillAttackRollHit",
    "doFillDamageDice",
];

pub fn replay_observed_action(observed_action_taken: &str) -> WeaponAttackOrderingState {
    replay_observed_action_through_spine(observed_action_taken)
}

pub fn expected_witness(observed_action_taken: &str) -> WeaponAttackOrderingState {
    match observed_action_taken {
        "doDiscoverAttack" => discover_weapon_attack(),
        "doRejectAttackRollBeforeTargetChoice" => reject_weapon_attack_roll_before_target_choice(),
        "doFillTargetChoice" => fill_weapon_attack_target_choice(),
        "doRejectDamageBeforeAttackRoll" => reject_weapon_damage_before_attack_roll(),
        "doFillAttackRollMiss" => fill_weapon_attack_roll_miss(),
        "doFillAttackRollHit" => fill_weapon_attack_roll_hit(),
        "doFillDamageDice" => fill_weapon_attack_damage_dice(),
        action => panic!("unsupported mbt::actionTaken {action}"),
    }
}

pub fn replay_observed_route(observed_action_taken: &str) -> Vec<ReducerRouteEvent> {
    match observed_action_taken {
        "doDiscoverAttack" => weapon_attack_discovery_route().2,
        "doRejectAttackRollBeforeTargetChoice" => {
            let (state, subject, mut route) = weapon_attack_discovery_route();
            let result = resolve_battle_subject_test_fill(
                state,
                subject,
                BattleFill::AttackRoll(AttackRollFacts {
                    total: 16,
                    natural_d20: 12,
                }),
            );
            route.push(weapon_attack_route_event(
                ReducerRouteFillKind::AttackRoll,
                ReducerRouteOwnerGroup::HoleFrontier,
                &result,
            ));
            route
        }
        "doFillTargetChoice" => weapon_attack_target_choice_route().2,
        "doRejectDamageBeforeAttackRoll" => {
            let (state, subject, mut route) = weapon_attack_target_choice_route();
            let result =
                resolve_battle_subject_test_fill(state, subject, BattleFill::DamageRoll(6));
            route.push(weapon_attack_route_event(
                ReducerRouteFillKind::RolledDice,
                ReducerRouteOwnerGroup::HoleFrontier,
                &result,
            ));
            route
        }
        "doFillAttackRollMiss" => {
            let (state, subject, mut route) = weapon_attack_target_choice_route();
            let result = resolve_battle_subject_test_fill(
                state,
                subject,
                BattleFill::AttackRoll(AttackRollFacts {
                    total: 2,
                    natural_d20: 1,
                }),
            );
            route.push(weapon_attack_route_event(
                ReducerRouteFillKind::AttackRoll,
                ReducerRouteOwnerGroup::AttackRoll,
                &result,
            ));
            route
        }
        "doFillAttackRollHit" => weapon_attack_hit_route().2,
        "doFillDamageDice" => {
            let (state, subject, mut route) = weapon_attack_hit_route();
            let result =
                resolve_battle_subject_test_fill(state, subject, BattleFill::DamageRoll(6));
            route.push(weapon_attack_route_event(
                ReducerRouteFillKind::RolledDice,
                ReducerRouteOwnerGroup::HitPoint,
                &result,
            ));
            route
        }
        action => panic!("unsupported mbt::actionTaken {action}"),
    }
}

pub fn expected_route(observed_action_taken: &str) -> Vec<ReducerRouteEvent> {
    match observed_action_taken {
        "doDiscoverAttack" => expected_weapon_discovery_route(),
        "doRejectAttackRollBeforeTargetChoice" => {
            let mut route = expected_weapon_discovery_route();
            route.push(route_resolve_battle_subject_from_route_result(
                ReducerRouteSubjectFamily::WeaponAttack,
                ReducerRouteFillKind::AttackRoll,
                ReducerRouteResolutionOutcome::Invalid(BattleResolutionInvalidReason::InvalidFill),
                vec![ReducerRouteHoleKind::TargetChoice],
                ReducerRouteOwnerGroup::HoleFrontier,
            ));
            route
        }
        "doFillTargetChoice" => expected_weapon_route(&[(
            ReducerRouteFillKind::TargetChoice,
            vec![BattleHoleKind::AttackRoll],
            ReducerRouteOwnerGroup::TargetSelection,
        )]),
        "doRejectDamageBeforeAttackRoll" => {
            let mut route = expected_weapon_route(&[(
                ReducerRouteFillKind::TargetChoice,
                vec![BattleHoleKind::AttackRoll],
                ReducerRouteOwnerGroup::TargetSelection,
            )]);
            route.push(route_resolve_battle_subject_from_route_result(
                ReducerRouteSubjectFamily::WeaponAttack,
                ReducerRouteFillKind::RolledDice,
                ReducerRouteResolutionOutcome::Invalid(BattleResolutionInvalidReason::InvalidFill),
                vec![ReducerRouteHoleKind::AttackRoll],
                ReducerRouteOwnerGroup::HoleFrontier,
            ));
            route
        }
        "doFillAttackRollMiss" => expected_weapon_route(&[
            (
                ReducerRouteFillKind::TargetChoice,
                vec![BattleHoleKind::AttackRoll],
                ReducerRouteOwnerGroup::TargetSelection,
            ),
            (
                ReducerRouteFillKind::AttackRoll,
                Vec::new(),
                ReducerRouteOwnerGroup::AttackRoll,
            ),
        ]),
        "doFillAttackRollHit" => expected_weapon_route(&[
            (
                ReducerRouteFillKind::TargetChoice,
                vec![BattleHoleKind::AttackRoll],
                ReducerRouteOwnerGroup::TargetSelection,
            ),
            (
                ReducerRouteFillKind::AttackRoll,
                vec![BattleHoleKind::RolledDice],
                ReducerRouteOwnerGroup::AttackRoll,
            ),
        ]),
        "doFillDamageDice" => expected_weapon_route(&[
            (
                ReducerRouteFillKind::TargetChoice,
                vec![BattleHoleKind::AttackRoll],
                ReducerRouteOwnerGroup::TargetSelection,
            ),
            (
                ReducerRouteFillKind::AttackRoll,
                vec![BattleHoleKind::RolledDice],
                ReducerRouteOwnerGroup::AttackRoll,
            ),
            (
                ReducerRouteFillKind::RolledDice,
                Vec::new(),
                ReducerRouteOwnerGroup::HitPoint,
            ),
        ]),
        action => panic!("unsupported mbt::actionTaken {action}"),
    }
}

fn expected_weapon_route(
    steps: &[(
        ReducerRouteFillKind,
        Vec<BattleHoleKind>,
        ReducerRouteOwnerGroup,
    )],
) -> Vec<ReducerRouteEvent> {
    let mut route = expected_weapon_discovery_route();
    for (fill, holes, owner) in steps {
        route.push(
            super::battle_runtime_reducer_route::route_resolve_battle_subject(
                ReducerRouteSubjectFamily::WeaponAttack,
                *fill,
                holes.clone(),
                *owner,
            ),
        );
    }
    route
}

fn expected_weapon_discovery_route() -> Vec<ReducerRouteEvent> {
    vec![
        route_start_battle(ReducerRouteOwnerGroup::ActionEconomy),
        route_discover_battle_acts(
            ReducerRouteSubjectFamily::WeaponAttack,
            vec![BattleHoleKind::TargetChoice],
            ReducerRouteOwnerGroup::ActionEconomy,
        ),
    ]
}

pub fn projection_payload(state: &WeaponAttackOrderingState) -> String {
    let protocol_holes = protocol_holes(&state.protocol);
    [
        format!("qStage={}", stage_ref(state.stage)),
        format!("qHoles={}", joined_or_none(&protocol_holes)),
        format!(
            "qLastOrderingError={}",
            ordering_error_ref(state.last_ordering_error)
        ),
        format!("protocolResult={}", protocol_result_ref(&state.protocol)),
        format!(
            "protocolInvalidReason={}",
            protocol_invalid_reason_ref(&state.protocol)
        ),
        format!("protocolHoles={}", joined_or_none(&protocol_holes)),
    ]
    .join("\n")
}

fn replay_observed_action_through_spine(observed_action_taken: &str) -> WeaponAttackOrderingState {
    match observed_action_taken {
        "doDiscoverAttack" => {
            let state = standard_battle_state();
            let act = discovered_weapon_attack_act(&state);
            projection(
                act.subject.stage,
                WeaponAttackRuntimeResult::NeedsHoles,
                None,
            )
        }
        "doRejectAttackRollBeforeTargetChoice" => {
            let state = standard_battle_state();
            let act = discovered_weapon_attack_act(&state);
            let result = resolve_battle_subject_test_fill(
                state,
                act.subject,
                BattleFill::AttackRoll(AttackRollFacts {
                    total: 16,
                    natural_d20: 12,
                }),
            );
            assert_invalid(result);
            projection(
                WeaponAttackFrontierStage::TargetChoice,
                WeaponAttackRuntimeResult::Invalid,
                Some(WeaponAttackFillOrderingError::TargetChoiceRequired),
            )
        }
        "doFillTargetChoice" => {
            let (state, subject) = spine_after_target_choice();
            let _ = state;
            projection(subject.stage, WeaponAttackRuntimeResult::NeedsHoles, None)
        }
        "doRejectDamageBeforeAttackRoll" => {
            let (state, subject) = spine_after_target_choice();
            let result =
                resolve_battle_subject_test_fill(state, subject, BattleFill::DamageRoll(6));
            assert_invalid(result);
            projection(
                WeaponAttackFrontierStage::AttackRoll,
                WeaponAttackRuntimeResult::Invalid,
                Some(WeaponAttackFillOrderingError::AttackRollRequired),
            )
        }
        "doFillAttackRollMiss" => {
            let (state, subject) = spine_after_target_choice();
            let result = resolve_battle_subject_test_fill(
                state,
                subject,
                BattleFill::AttackRoll(AttackRollFacts {
                    total: 2,
                    natural_d20: 1,
                }),
            );
            assert_resolved(result);
            projection(
                WeaponAttackFrontierStage::Resolved,
                WeaponAttackRuntimeResult::Resolved,
                None,
            )
        }
        "doFillAttackRollHit" => {
            let (_state, subject) = spine_after_attack_roll_hit();
            projection(subject.stage, WeaponAttackRuntimeResult::NeedsHoles, None)
        }
        "doFillDamageDice" => {
            let (state, subject) = spine_after_attack_roll_hit();
            let result =
                resolve_battle_subject_test_fill(state, subject, BattleFill::DamageRoll(6));
            assert_resolved(result);
            projection(
                WeaponAttackFrontierStage::Resolved,
                WeaponAttackRuntimeResult::Resolved,
                None,
            )
        }
        action => panic!("unsupported mbt::actionTaken {action}"),
    }
}

fn discovered_weapon_attack_act(
    state: &crate::rules::battle_reducer_spine::BattleState,
) -> crate::rules::battle_reducer_spine::AvailableBattleAct {
    discover_battle_acts(state)
        .into_available_acts()
        .into_iter()
        .find(|act| act.subject.kind == BattleSubjectKind::WeaponAttack)
        .expect("QNT initialState should discover one Fighter weapon attack")
}

fn weapon_attack_discovery_route() -> (
    crate::rules::battle_reducer_spine::BattleState,
    BattleSubject,
    Vec<ReducerRouteEvent>,
) {
    let state = standard_battle_state();
    let act = discovered_weapon_attack_act(&state);
    let route = vec![
        route_start_battle(ReducerRouteOwnerGroup::ActionEconomy),
        route_discover_battle_acts(
            ReducerRouteSubjectFamily::WeaponAttack,
            act.holes,
            ReducerRouteOwnerGroup::ActionEconomy,
        ),
    ];
    (state, act.subject, route)
}

fn spine_after_target_choice() -> (
    crate::rules::battle_reducer_spine::BattleState,
    BattleSubject,
) {
    let state = standard_battle_state();
    let act = discovered_weapon_attack_act(&state);
    let result = resolve_battle_subject_test_fill(
        state,
        act.subject,
        BattleFill::TargetChoice(Actor::Goblin),
    );
    let outcome = result.outcome();
    let needs_holes = result
        .into_needs_holes()
        .unwrap_or_else(|| panic!("target choice should need attack-roll holes, got {outcome:?}"));
    (needs_holes.state, needs_holes.subject)
}

fn weapon_attack_target_choice_route() -> (
    crate::rules::battle_reducer_spine::BattleState,
    BattleSubject,
    Vec<ReducerRouteEvent>,
) {
    let (state, subject, mut route) = weapon_attack_discovery_route();
    let result =
        resolve_battle_subject_test_fill(state, subject, BattleFill::TargetChoice(Actor::Goblin));
    route.push(weapon_attack_route_event(
        ReducerRouteFillKind::TargetChoice,
        ReducerRouteOwnerGroup::TargetSelection,
        &result,
    ));
    let outcome = result.outcome();
    let needs_holes = result
        .into_needs_holes()
        .unwrap_or_else(|| panic!("target choice should need attack-roll holes, got {outcome:?}"));
    (needs_holes.state, needs_holes.subject, route)
}

fn standard_battle_state() -> crate::rules::battle_reducer_spine::BattleState {
    start_battle(BattleSetup::standard()).state
}

fn spine_after_attack_roll_hit() -> (
    crate::rules::battle_reducer_spine::BattleState,
    BattleSubject,
) {
    let (state, subject) = spine_after_target_choice();
    let result = resolve_battle_subject_test_fill(
        state,
        subject,
        BattleFill::AttackRoll(AttackRollFacts {
            total: 16,
            natural_d20: 12,
        }),
    );
    let outcome = result.outcome();
    let needs_holes = result
        .into_needs_holes()
        .unwrap_or_else(|| panic!("attack-roll hit should need damage holes, got {outcome:?}"));
    (needs_holes.state, needs_holes.subject)
}

fn weapon_attack_hit_route() -> (
    crate::rules::battle_reducer_spine::BattleState,
    BattleSubject,
    Vec<ReducerRouteEvent>,
) {
    let (state, subject, mut route) = weapon_attack_target_choice_route();
    let result = resolve_battle_subject_test_fill(
        state,
        subject,
        BattleFill::AttackRoll(AttackRollFacts {
            total: 16,
            natural_d20: 12,
        }),
    );
    route.push(weapon_attack_route_event(
        ReducerRouteFillKind::AttackRoll,
        ReducerRouteOwnerGroup::AttackRoll,
        &result,
    ));
    let outcome = result.outcome();
    let needs_holes = result
        .into_needs_holes()
        .unwrap_or_else(|| panic!("attack-roll hit should need damage holes, got {outcome:?}"));
    (needs_holes.state, needs_holes.subject, route)
}

fn weapon_attack_route_event(
    fill: ReducerRouteFillKind,
    owner: ReducerRouteOwnerGroup,
    result: &crate::rules::battle_reducer_spine::BattleResolutionResult,
) -> ReducerRouteEvent {
    route_resolve_battle_subject_from_result(
        ReducerRouteResolveConnector {
            subject: ReducerRouteSubjectFamily::WeaponAttack,
            fill: ReducerRouteResolveFill::Fill(fill),
            owner,
        },
        result,
    )
}

fn assert_invalid(result: crate::rules::battle_reducer_spine::BattleResolutionResult) {
    let outcome = result.outcome();
    assert!(
        matches!(outcome, BattleResolutionOutcome::Invalid(_)),
        "expected invalid reducer result, got {outcome:?}"
    );
}

fn assert_resolved(result: crate::rules::battle_reducer_spine::BattleResolutionResult) {
    let outcome = result.outcome();
    assert!(
        outcome == BattleResolutionOutcome::Resolved,
        "expected resolved reducer result, got {outcome:?}"
    );
}

fn projection(
    stage: WeaponAttackFrontierStage,
    runtime_result: WeaponAttackRuntimeResult,
    last_ordering_error: Option<WeaponAttackFillOrderingError>,
) -> WeaponAttackOrderingState {
    weapon_attack_ordering_projection(WeaponAttackOrderingProjectionFacts {
        stage,
        runtime_result,
        last_ordering_error,
    })
}

fn stage_ref(stage: WeaponAttackFrontierStage) -> &'static str {
    match stage {
        WeaponAttackFrontierStage::ActSelection => "WeaponAttackActSelectionStage",
        WeaponAttackFrontierStage::TargetChoice => "WeaponAttackTargetChoiceStage",
        WeaponAttackFrontierStage::AttackRoll => "WeaponAttackAttackRollStage",
        WeaponAttackFrontierStage::DamageDice => "WeaponAttackDamageDiceStage",
        WeaponAttackFrontierStage::Resolved => "WeaponAttackResolvedStage",
    }
}

fn ordering_error_ref(error: Option<WeaponAttackFillOrderingError>) -> &'static str {
    match error {
        Some(WeaponAttackFillOrderingError::TargetChoiceRequired) => "targetChoiceRequired",
        Some(WeaponAttackFillOrderingError::AttackRollRequired) => "attackRollRequired",
        None => "",
    }
}

fn protocol_result_ref(protocol: &WeaponAttackOrderingProtocol) -> &'static str {
    match protocol {
        WeaponAttackOrderingProtocol::Init => "init",
        WeaponAttackOrderingProtocol::NeedsHoles(_) => "needsHoles",
        WeaponAttackOrderingProtocol::Resolved => "resolved",
        WeaponAttackOrderingProtocol::Invalid { .. } => "invalid",
    }
}

fn protocol_invalid_reason_ref(protocol: &WeaponAttackOrderingProtocol) -> &'static str {
    match protocol {
        WeaponAttackOrderingProtocol::Invalid {
            reason: WeaponAttackInvalidReason::InvalidFill,
            ..
        } => "WInvalidFill",
        WeaponAttackOrderingProtocol::Init
        | WeaponAttackOrderingProtocol::NeedsHoles(_)
        | WeaponAttackOrderingProtocol::Resolved => "",
    }
}

fn protocol_holes(protocol: &WeaponAttackOrderingProtocol) -> Vec<&'static str> {
    match protocol {
        WeaponAttackOrderingProtocol::NeedsHoles(holes) => holes.iter().map(hole_ref).collect(),
        WeaponAttackOrderingProtocol::Invalid { holes, .. } => holes.iter().map(hole_ref).collect(),
        WeaponAttackOrderingProtocol::Init | WeaponAttackOrderingProtocol::Resolved => Vec::new(),
    }
}

fn hole_ref(hole: &WeaponAttackHoleKind) -> &'static str {
    match hole {
        WeaponAttackHoleKind::TargetChoice => "TargetChoiceHoleKind",
        WeaponAttackHoleKind::AttackRoll => "AttackRollHoleKind",
        WeaponAttackHoleKind::RolledDice => "RolledDiceHoleKind",
    }
}

fn joined_or_none(values: &[&str]) -> String {
    if values.is_empty() {
        "none".to_string()
    } else {
        values.join(",")
    }
}
