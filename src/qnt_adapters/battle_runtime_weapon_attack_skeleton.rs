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

pub fn expected_witness(observed_action_taken: &str) -> WeaponAttackSkeletonState {
    replay_observed_action(observed_action_taken)
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
