use crate::rules::battle_reducer_spine::{
    discover_battle_acts, resolve_battle_subject_test_fill, start_skeleton_actor_turn,
    start_skeleton_battle, stat_block_multiattack_dispatches_available, Actor, AttackRollFacts,
    BattleFill, BattleHoleKind, BattleResolutionInvalidReason, BattleResolutionOutcome,
    BattleSubject, BattleSubjectKind,
};
use crate::rules::weapon_attack_skeleton::{
    discover_skeleton_weapon_attack, fill_skeleton_weapon_attack_roll_hit,
    fill_skeleton_weapon_attack_roll_miss, fill_skeleton_weapon_attack_target,
    fill_skeleton_weapon_damage_high, fill_skeleton_weapon_damage_high_sneak_attack,
    fill_skeleton_weapon_damage_low, fill_skeleton_weapon_damage_low_sneak_attack,
    reject_recursive_skeleton_multiattack, reject_skeleton_weapon_attack_stale_after_resolved,
    reject_skeleton_weapon_attack_wrong_target, resolve_skeleton_multiattack,
    spend_skeleton_multiattack_dispatch, start_skeleton_turn, WeaponAttackSkeletonHole,
    WeaponAttackSkeletonInvalidReason, WeaponAttackSkeletonProtocol, WeaponAttackSkeletonState,
};

pub const BRANCH_ACTIONS: [&str; 14] = [
    "doDiscoverAttack",
    "doFillTarget",
    "doRejectWrongTarget",
    "doFillAttackRollMiss",
    "doFillAttackRollHit",
    "doFillDamageLow",
    "doFillDamageHigh",
    "doFillDamageLowSneakAttack",
    "doFillDamageHighSneakAttack",
    "doRejectStaleAfterResolved",
    "doStartSkeletonTurn",
    "doResolveSkeletonMultiattack",
    "doRejectRecursiveSkeletonMultiattack",
    "doSpendSkeletonMultiattackDispatch",
];

pub fn replay_observed_action(observed_action_taken: &str) -> WeaponAttackSkeletonState {
    replay_observed_action_through_spine(observed_action_taken)
}

pub fn expected_witness(observed_action_taken: &str) -> WeaponAttackSkeletonState {
    match observed_action_taken {
        "doDiscoverAttack" => discover_skeleton_weapon_attack(),
        "doFillTarget" => fill_skeleton_weapon_attack_target(),
        "doRejectWrongTarget" => reject_skeleton_weapon_attack_wrong_target(),
        "doFillAttackRollMiss" => fill_skeleton_weapon_attack_roll_miss(),
        "doFillAttackRollHit" => fill_skeleton_weapon_attack_roll_hit(),
        "doFillDamageLow" => fill_skeleton_weapon_damage_low(),
        "doFillDamageHigh" => fill_skeleton_weapon_damage_high(),
        "doFillDamageLowSneakAttack" => fill_skeleton_weapon_damage_low_sneak_attack(),
        "doFillDamageHighSneakAttack" => fill_skeleton_weapon_damage_high_sneak_attack(),
        "doRejectStaleAfterResolved" => reject_skeleton_weapon_attack_stale_after_resolved(),
        "doStartSkeletonTurn" => start_skeleton_turn(),
        "doResolveSkeletonMultiattack" => resolve_skeleton_multiattack(),
        "doRejectRecursiveSkeletonMultiattack" => reject_recursive_skeleton_multiattack(),
        "doSpendSkeletonMultiattackDispatch" => spend_skeleton_multiattack_dispatch(),
        action => panic!("unsupported mbt::actionTaken {action}"),
    }
}

fn replay_observed_action_through_spine(observed_action_taken: &str) -> WeaponAttackSkeletonState {
    match observed_action_taken {
        "doDiscoverAttack" => {
            let state = start_skeleton_battle();
            let act = discovered_weapon_attack_act(&state);
            projection_from_spine(
                &state,
                WeaponAttackSkeletonProtocol::NeedsHoles(holes_from_spine(&act.holes)),
            )
        }
        "doFillTarget" => {
            let (state, _subject) = spine_after_target_choice();
            projection_from_spine(
                &state,
                WeaponAttackSkeletonProtocol::NeedsHoles(vec![
                    WeaponAttackSkeletonHole::AttackRoll,
                ]),
            )
        }
        "doRejectWrongTarget" => {
            let state = start_skeleton_battle();
            let act = discovered_weapon_attack_act(&state);
            let result = resolve_battle_subject_test_fill(
                state,
                act.subject,
                BattleFill::TargetChoice(Actor::Rogue),
            );
            let outcome = result.outcome();
            let invalid = result
                .into_invalid()
                .unwrap_or_else(|| panic!("expected invalid reducer result, got {outcome:?}"));
            assert_eq!(invalid.reason, BattleResolutionInvalidReason::WrongTarget);
            projection_from_spine(
                &invalid.state,
                WeaponAttackSkeletonProtocol::Invalid {
                    holes: vec![WeaponAttackSkeletonHole::TargetChoice],
                    reason: WeaponAttackSkeletonInvalidReason::InvalidFill,
                },
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
            let outcome = result.outcome();
            let state = result
                .into_resolved_state()
                .unwrap_or_else(|| panic!("expected resolved reducer result, got {outcome:?}"));
            projection_from_spine(&state, WeaponAttackSkeletonProtocol::Resolved)
        }
        "doFillAttackRollHit" => {
            let (state, _subject) = spine_after_attack_roll_hit();
            projection_from_spine(
                &state,
                WeaponAttackSkeletonProtocol::NeedsHoles(vec![
                    WeaponAttackSkeletonHole::DamageRoll,
                ]),
            )
        }
        "doFillDamageLow" => replay_damage_roll(2, false),
        "doFillDamageHigh" => replay_damage_roll(4, false),
        "doFillDamageLowSneakAttack" => replay_damage_roll(4, true),
        "doFillDamageHighSneakAttack" => replay_damage_roll(8, true),
        "doRejectStaleAfterResolved" => {
            let (state, subject) = spine_after_attack_roll_hit();
            let result = resolve_battle_subject_test_fill(
                state,
                subject,
                BattleFill::SneakAttackDamageRoll(8),
            );
            let outcome = result.outcome();
            let state = result
                .into_resolved_state()
                .unwrap_or_else(|| panic!("expected resolved reducer result, got {outcome:?}"));
            let result =
                resolve_battle_subject_test_fill(state, subject, BattleFill::DamageRoll(1));
            let outcome = result.outcome();
            let invalid = result
                .into_invalid()
                .unwrap_or_else(|| panic!("expected invalid reducer result, got {outcome:?}"));
            assert_eq!(invalid.reason, BattleResolutionInvalidReason::StaleSubject);
            assert!(invalid.holes.is_empty());
            projection_from_spine(
                &invalid.state,
                WeaponAttackSkeletonProtocol::Invalid {
                    holes: Vec::new(),
                    reason: WeaponAttackSkeletonInvalidReason::StaleSubject,
                },
            )
        }
        "doStartSkeletonTurn" => {
            let state = start_skeleton_actor_turn();
            projection_from_spine(&state, WeaponAttackSkeletonProtocol::Resolved)
        }
        "doResolveSkeletonMultiattack" => {
            let (state, _subject) = spine_after_multiattack_resolution();
            projection_from_spine(&state, WeaponAttackSkeletonProtocol::Resolved)
        }
        "doRejectRecursiveSkeletonMultiattack" => {
            let (state, subject) = spine_after_multiattack_resolution();
            let result =
                resolve_battle_subject_test_fill(state, subject, BattleFill::ResolveMultiattack);
            let outcome = result.outcome();
            let invalid = result
                .into_invalid()
                .unwrap_or_else(|| panic!("expected invalid reducer result, got {outcome:?}"));
            assert_eq!(invalid.reason, BattleResolutionInvalidReason::StaleSubject);
            assert!(invalid.holes.is_empty());
            projection_from_spine(
                &invalid.state,
                WeaponAttackSkeletonProtocol::Invalid {
                    holes: Vec::new(),
                    reason: WeaponAttackSkeletonInvalidReason::StaleSubject,
                },
            )
        }
        "doSpendSkeletonMultiattackDispatch" => {
            let (state, subject) = spine_after_multiattack_resolution();
            let result = resolve_battle_subject_test_fill(
                state,
                subject,
                BattleFill::SpendMultiattackDispatch,
            );
            let outcome = result.outcome();
            let state = result
                .into_resolved_state()
                .unwrap_or_else(|| panic!("expected resolved reducer result, got {outcome:?}"));
            projection_from_spine(&state, WeaponAttackSkeletonProtocol::Resolved)
        }
        action => panic!("unsupported mbt::actionTaken {action}"),
    }
}

fn replay_damage_roll(rolled_damage: i16, use_sneak_attack: bool) -> WeaponAttackSkeletonState {
    let (state, subject) = spine_after_attack_roll_hit();
    let fill = if use_sneak_attack {
        BattleFill::SneakAttackDamageRoll(rolled_damage)
    } else {
        BattleFill::DamageRoll(rolled_damage)
    };
    let result = resolve_battle_subject_test_fill(state, subject, fill);
    let outcome = result.outcome();
    let state = result
        .into_resolved_state()
        .unwrap_or_else(|| panic!("expected resolved reducer result, got {outcome:?}"));
    projection_from_spine(&state, WeaponAttackSkeletonProtocol::Resolved)
}

fn discovered_weapon_attack_act(
    state: &crate::rules::battle_reducer_spine::BattleState,
) -> crate::rules::battle_reducer_spine::AvailableBattleAct {
    discover_battle_acts(state)
        .into_available_acts()
        .into_iter()
        .find(|act| {
            act.subject.kind == BattleSubjectKind::WeaponAttack && act.subject.actor == Actor::Rogue
        })
        .expect("QNT skeleton initial state should discover one Rogue weapon attack")
}

fn spine_after_target_choice() -> (
    crate::rules::battle_reducer_spine::BattleState,
    BattleSubject,
) {
    let state = start_skeleton_battle();
    let act = discovered_weapon_attack_act(&state);
    let result = resolve_battle_subject_test_fill(
        state,
        act.subject,
        BattleFill::TargetChoice(Actor::Skeleton),
    );
    let outcome = result.outcome();
    let needs_holes = result
        .into_needs_holes()
        .unwrap_or_else(|| panic!("target choice should need attack-roll holes, got {outcome:?}"));
    (needs_holes.state, needs_holes.subject)
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
        .unwrap_or_else(|| panic!("attack roll hit should need damage holes, got {outcome:?}"));
    (needs_holes.state, needs_holes.subject)
}

fn spine_after_multiattack_resolution() -> (
    crate::rules::battle_reducer_spine::BattleState,
    BattleSubject,
) {
    let state = start_skeleton_actor_turn();
    let act = discover_battle_acts(&state)
        .into_available_acts()
        .into_iter()
        .next()
        .expect("Skeleton turn should discover one multiattack act");
    let result =
        resolve_battle_subject_test_fill(state, act.subject, BattleFill::ResolveMultiattack);
    let outcome = result.outcome();
    if outcome != BattleResolutionOutcome::Resolved {
        panic!("Skeleton multiattack should resolve, got {outcome:?}");
    }
    (result.into_state(), act.subject)
}

fn projection_from_spine(
    state: &crate::rules::battle_reducer_spine::BattleState,
    protocol: WeaponAttackSkeletonProtocol,
) -> WeaponAttackSkeletonState {
    WeaponAttackSkeletonState {
        skeleton_hp: state.skeleton.hp,
        skeleton_dead: crate::rules::battle_reducer_spine::combatant_is_dead(state.skeleton),
        action_available: state.action_available,
        multiattack_dispatches_available: stat_block_multiattack_dispatches_available(state),
        sneak_attack_used_this_turn: state.rogue.sneak_attack_used_this_turn,
        protocol,
    }
}

fn holes_from_spine(holes: &[BattleHoleKind]) -> Vec<WeaponAttackSkeletonHole> {
    holes
        .iter()
        .map(|hole| match hole {
            BattleHoleKind::TargetChoice => WeaponAttackSkeletonHole::TargetChoice,
            BattleHoleKind::AttackRoll => WeaponAttackSkeletonHole::AttackRoll,
            BattleHoleKind::RolledDice => WeaponAttackSkeletonHole::DamageRoll,
            BattleHoleKind::SpellTargetAllocation
            | BattleHoleKind::SpellTargetList
            | BattleHoleKind::ConditionChoice
            | BattleHoleKind::SavingThrowOutcome
            | BattleHoleKind::HitPointHealingDistribution
            | BattleHoleKind::DeathSavingThrow
            | BattleHoleKind::ConcentrationSavingThrow => {
                panic!("weapon projection received non-weapon reducer hole {hole:?}")
            }
        })
        .collect()
}

pub fn projection_payload(state: &WeaponAttackSkeletonState) -> String {
    let protocol_holes = protocol_holes(&state.protocol);
    [
        format!("qSkeletonHp={}", state.skeleton_hp),
        format!("qSkeletonDead={}", state.skeleton_dead),
        format!("qActionAvailable={}", state.action_available),
        format!(
            "qMultiattackDispatchesAvailable={}",
            state.multiattack_dispatches_available
        ),
        format!(
            "qSneakAttackUsedThisTurn={}",
            state.sneak_attack_used_this_turn
        ),
        format!("qHoles={}", joined_or_none(&protocol_holes)),
        format!("protocolResult={}", protocol_result_ref(&state.protocol)),
        format!(
            "protocolInvalidReason={}",
            protocol_invalid_reason_ref(&state.protocol)
        ),
        format!("protocolHoles={}", joined_or_none(&protocol_holes)),
    ]
    .join("\n")
}

fn protocol_result_ref(protocol: &WeaponAttackSkeletonProtocol) -> &'static str {
    match protocol {
        WeaponAttackSkeletonProtocol::Init(_) => "init",
        WeaponAttackSkeletonProtocol::NeedsHoles(_) => "needsHoles",
        WeaponAttackSkeletonProtocol::Resolved => "resolved",
        WeaponAttackSkeletonProtocol::Invalid { .. } => "invalid",
    }
}

fn protocol_invalid_reason_ref(protocol: &WeaponAttackSkeletonProtocol) -> &'static str {
    match protocol {
        WeaponAttackSkeletonProtocol::Invalid {
            reason: WeaponAttackSkeletonInvalidReason::InvalidFill,
            ..
        } => "WInvalidFill",
        WeaponAttackSkeletonProtocol::Invalid {
            reason: WeaponAttackSkeletonInvalidReason::StaleSubject,
            ..
        } => "WStaleSubject",
        WeaponAttackSkeletonProtocol::Init(_)
        | WeaponAttackSkeletonProtocol::NeedsHoles(_)
        | WeaponAttackSkeletonProtocol::Resolved => "",
    }
}

fn protocol_holes(protocol: &WeaponAttackSkeletonProtocol) -> Vec<&'static str> {
    match protocol {
        WeaponAttackSkeletonProtocol::Init(holes)
        | WeaponAttackSkeletonProtocol::NeedsHoles(holes) => holes.iter().map(hole_ref).collect(),
        WeaponAttackSkeletonProtocol::Invalid { holes, .. } => holes.iter().map(hole_ref).collect(),
        WeaponAttackSkeletonProtocol::Resolved => Vec::new(),
    }
}

fn hole_ref(hole: &WeaponAttackSkeletonHole) -> &'static str {
    match hole {
        WeaponAttackSkeletonHole::TargetChoice => "TargetChoice",
        WeaponAttackSkeletonHole::AttackRoll => "AttackRoll",
        WeaponAttackSkeletonHole::DamageRoll => "DamageRoll",
    }
}

fn joined_or_none(values: &[&str]) -> String {
    if values.is_empty() {
        "none".to_string()
    } else {
        values.join(",")
    }
}
