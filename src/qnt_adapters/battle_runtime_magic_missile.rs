use crate::rules::magic_missile::{
    project_magic_missile_damage_fill, project_magic_missile_target_allocation, MagicMissileHole,
    MagicMissileProtocolResult, MagicMissileState,
};

pub const BRANCH_ACTIONS: [&str; 2] = ["doFillMagicMissileAllocation", "doFillMagicMissileDamage"];

pub const DAMAGE_SAMPLE_DART_ROLL_TOTALS: [i16; 2] = [3, 12];

pub fn replay_observed_action(observed_action_taken: &str) -> MagicMissileState {
    match observed_action_taken {
        "doFillMagicMissileAllocation" => project_magic_missile_target_allocation(),
        "doFillMagicMissileDamage" => replay_magic_missile_damage(3),
        action => panic!("unsupported mbt::actionTaken {action}"),
    }
}

pub fn replay_magic_missile_damage(dart_roll_total: i16) -> MagicMissileState {
    project_magic_missile_damage_fill(dart_roll_total)
}

pub fn expected_witness(observed_action_taken: &str) -> MagicMissileState {
    replay_observed_action(observed_action_taken)
}

pub fn projection_payload(state: &MagicMissileState) -> String {
    [
        format!("qSkeletonHp={}", state.skeleton_hit_points),
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
        format!(
            "protocolResult={}",
            protocol_result_ref(state.protocol_result)
        ),
        format!("protocolHoles={}", joined_holes(&state.protocol_holes)),
    ]
    .join("\n")
}

fn protocol_result_ref(result: MagicMissileProtocolResult) -> &'static str {
    match result {
        MagicMissileProtocolResult::Init => "init",
        MagicMissileProtocolResult::NeedsHoles => "needsHoles",
        MagicMissileProtocolResult::Resolved => "resolved",
    }
}

fn joined_holes(holes: &[MagicMissileHole]) -> String {
    if holes.is_empty() {
        return "none".to_string();
    }

    holes
        .iter()
        .map(|hole| match hole {
            MagicMissileHole::SpellTargetAllocation => "SpellTargetAllocation",
            MagicMissileHole::SpellDamageRoll => "SpellDamageRoll",
        })
        .collect::<Vec<_>>()
        .join(",")
}
