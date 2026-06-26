use crate::rules::battle_reducer_spine::{
    discover_battle_acts, resolve_battle_subject_test_fill, start_battle, Actor, AttackRollFacts,
    BattleFill, BattleResolutionResult, BattleSubject, BattleSubjectKind,
};
use crate::rules::weapon_attack_ordering::{
    discover_weapon_attack, fill_weapon_attack_damage_dice, fill_weapon_attack_roll_hit,
    fill_weapon_attack_roll_miss, fill_weapon_attack_target_choice,
    reject_weapon_attack_roll_before_target_choice, reject_weapon_damage_before_attack_roll,
    weapon_attack_ordering_projection, WeaponAttackFillOrderingError, WeaponAttackFrontierStage,
    WeaponAttackHoleKind, WeaponAttackInvalidReason, WeaponAttackOrderingProjectionFacts,
    WeaponAttackOrderingProtocol, WeaponAttackOrderingState, WeaponAttackRuntimeResult,
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
            let state = start_battle();
            let act = discovered_weapon_attack_act(&state);
            projection(
                act.subject.stage,
                WeaponAttackRuntimeResult::NeedsHoles,
                None,
            )
        }
        "doRejectAttackRollBeforeTargetChoice" => {
            let state = start_battle();
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
        .into_iter()
        .find(|act| act.subject.kind == BattleSubjectKind::WeaponAttack)
        .expect("QNT initialState should discover one Fighter weapon attack")
}

fn spine_after_target_choice() -> (
    crate::rules::battle_reducer_spine::BattleState,
    BattleSubject,
) {
    let state = start_battle();
    let act = discovered_weapon_attack_act(&state);
    match resolve_battle_subject_test_fill(
        state,
        act.subject,
        BattleFill::TargetChoice(Actor::Goblin),
    ) {
        BattleResolutionResult::NeedsHoles { state, subject, .. } => (state, subject),
        other => panic!("target choice should need attack-roll holes, got {other:?}"),
    }
}

fn spine_after_attack_roll_hit() -> (
    crate::rules::battle_reducer_spine::BattleState,
    BattleSubject,
) {
    let (state, subject) = spine_after_target_choice();
    match resolve_battle_subject_test_fill(
        state,
        subject,
        BattleFill::AttackRoll(AttackRollFacts {
            total: 16,
            natural_d20: 12,
        }),
    ) {
        BattleResolutionResult::NeedsHoles { state, subject, .. } => (state, subject),
        other => panic!("attack-roll hit should need damage holes, got {other:?}"),
    }
}

fn assert_invalid(result: BattleResolutionResult) {
    assert!(
        matches!(result, BattleResolutionResult::Invalid { .. }),
        "expected invalid reducer result, got {result:?}"
    );
}

fn assert_resolved(result: BattleResolutionResult) {
    assert!(
        matches!(result, BattleResolutionResult::Resolved { .. }),
        "expected resolved reducer result, got {result:?}"
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
