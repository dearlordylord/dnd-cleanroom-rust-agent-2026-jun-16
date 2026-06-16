use crate::rules::weapon_attack_ordering::{
    discover_weapon_attack, fill_weapon_attack_damage_dice, fill_weapon_attack_roll_hit,
    fill_weapon_attack_roll_miss, fill_weapon_attack_target_choice,
    reject_weapon_attack_roll_before_target_choice, reject_weapon_damage_before_attack_roll,
    WeaponAttackFillOrderingError, WeaponAttackFrontierStage, WeaponAttackHoleKind,
    WeaponAttackInvalidReason, WeaponAttackOrderingProtocol, WeaponAttackOrderingState,
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

pub fn expected_witness(observed_action_taken: &str) -> WeaponAttackOrderingState {
    replay_observed_action(observed_action_taken)
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
