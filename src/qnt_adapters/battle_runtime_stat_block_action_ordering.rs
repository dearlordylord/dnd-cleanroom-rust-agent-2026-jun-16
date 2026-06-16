use crate::rules::stat_block_action_ordering::{
    discover_rolled_action_attack_control, discover_static_action_attack_control,
    fill_rolled_attack_roll_hit, fill_stat_block_attack_roll_miss, fill_stat_block_damage_dice,
    fill_stat_block_recharge_roll, fill_stat_block_target_choice, fill_static_attack_roll_hit,
    reject_attack_roll_before_target_choice, reject_damage_before_attack_roll,
    spend_recharge_gated_rolled_attack, start_multiattack_control,
    StatBlockActionFillOrderingError, StatBlockActionFrontierStage, StatBlockActionHoleKind,
    StatBlockActionInvalidReason, StatBlockActionOrderingProtocol, StatBlockActionOrderingState,
};

pub const BRANCH_ACTIONS: [&str; 12] = [
    "doStartMultiattackControl",
    "doDiscoverRolledActionAttackControl",
    "doDiscoverStaticActionAttackControl",
    "doRejectAttackRollBeforeTargetChoice",
    "doFillTargetChoice",
    "doRejectDamageBeforeAttackRoll",
    "doFillAttackRollMiss",
    "doFillRolledAttackRollHit",
    "doFillStaticAttackRollHit",
    "doFillDamageDice",
    "doSpendRechargeGatedRolledAttack",
    "doFillRechargeRoll",
];

pub fn replay_observed_action(observed_action_taken: &str) -> StatBlockActionOrderingState {
    match observed_action_taken {
        "doStartMultiattackControl" => start_multiattack_control(),
        "doDiscoverRolledActionAttackControl" => discover_rolled_action_attack_control(),
        "doDiscoverStaticActionAttackControl" => discover_static_action_attack_control(),
        "doRejectAttackRollBeforeTargetChoice" => reject_attack_roll_before_target_choice(),
        "doFillTargetChoice" => fill_stat_block_target_choice(),
        "doRejectDamageBeforeAttackRoll" => reject_damage_before_attack_roll(),
        "doFillAttackRollMiss" => fill_stat_block_attack_roll_miss(),
        "doFillRolledAttackRollHit" => fill_rolled_attack_roll_hit(),
        "doFillStaticAttackRollHit" => fill_static_attack_roll_hit(),
        "doFillDamageDice" => fill_stat_block_damage_dice(),
        "doSpendRechargeGatedRolledAttack" => spend_recharge_gated_rolled_attack(),
        "doFillRechargeRoll" => fill_stat_block_recharge_roll(),
        action => panic!("unsupported mbt::actionTaken {action}"),
    }
}

pub fn expected_witness(observed_action_taken: &str) -> StatBlockActionOrderingState {
    replay_observed_action(observed_action_taken)
}

pub fn projection_payload(state: &StatBlockActionOrderingState) -> String {
    let protocol_holes = protocol_holes(&state.protocol);
    [
        format!("qStage={}", stage_ref(state.stage)),
        format!("qHoles={}", joined_or_none(&protocol_holes)),
        format!(
            "qLastOrderingError={}",
            ordering_error_ref(state.last_ordering_error)
        ),
        format!(
            "qMultiattackDispatchesAvailable={}",
            state.multiattack_dispatches_available
        ),
        format!(
            "qRechargeActionAvailable={}",
            state.recharge_action_available
        ),
        format!("qUsesRolledDamage={}", state.uses_rolled_damage),
        format!("protocolResult={}", protocol_result_ref(&state.protocol)),
        format!(
            "protocolInvalidReason={}",
            protocol_invalid_reason_ref(&state.protocol)
        ),
        format!("protocolHoles={}", joined_or_none(&protocol_holes)),
    ]
    .join("\n")
}

fn stage_ref(stage: StatBlockActionFrontierStage) -> &'static str {
    match stage {
        StatBlockActionFrontierStage::ActSelection => "StatBlockActSelectionStage",
        StatBlockActionFrontierStage::AttackTargetChoice => "StatBlockAttackTargetChoiceStage",
        StatBlockActionFrontierStage::AttackRoll => "StatBlockAttackRollStage",
        StatBlockActionFrontierStage::DamageDice => "StatBlockDamageDiceStage",
        StatBlockActionFrontierStage::RechargeRoll => "StatBlockRechargeRollStage",
        StatBlockActionFrontierStage::Resolved => "StatBlockResolvedStage",
    }
}

fn ordering_error_ref(error: Option<StatBlockActionFillOrderingError>) -> &'static str {
    match error {
        Some(StatBlockActionFillOrderingError::TargetChoiceRequired) => {
            "statBlockTargetChoiceRequired"
        }
        Some(StatBlockActionFillOrderingError::AttackRollRequired) => "statBlockAttackRollRequired",
        Some(StatBlockActionFillOrderingError::RechargeRollRequired) => {
            "statBlockRechargeRollRequired"
        }
        None => "",
    }
}

fn protocol_result_ref(protocol: &StatBlockActionOrderingProtocol) -> &'static str {
    match protocol {
        StatBlockActionOrderingProtocol::Init => "init",
        StatBlockActionOrderingProtocol::NeedsHoles(_) => "needsHoles",
        StatBlockActionOrderingProtocol::Resolved => "resolved",
        StatBlockActionOrderingProtocol::Invalid { .. } => "invalid",
    }
}

fn protocol_invalid_reason_ref(protocol: &StatBlockActionOrderingProtocol) -> &'static str {
    match protocol {
        StatBlockActionOrderingProtocol::Invalid {
            reason: StatBlockActionInvalidReason::InvalidFill,
            ..
        } => "WInvalidFill",
        StatBlockActionOrderingProtocol::Init
        | StatBlockActionOrderingProtocol::NeedsHoles(_)
        | StatBlockActionOrderingProtocol::Resolved => "",
    }
}

fn protocol_holes(protocol: &StatBlockActionOrderingProtocol) -> Vec<&'static str> {
    match protocol {
        StatBlockActionOrderingProtocol::NeedsHoles(holes) => holes.iter().map(hole_ref).collect(),
        StatBlockActionOrderingProtocol::Invalid { holes, .. } => {
            holes.iter().map(hole_ref).collect()
        }
        StatBlockActionOrderingProtocol::Init | StatBlockActionOrderingProtocol::Resolved => {
            Vec::new()
        }
    }
}

fn hole_ref(hole: &StatBlockActionHoleKind) -> &'static str {
    match hole {
        StatBlockActionHoleKind::TargetChoice => "TargetChoiceHoleKind",
        StatBlockActionHoleKind::AttackRoll => "AttackRollHoleKind",
        StatBlockActionHoleKind::RolledDice => "RolledDiceHoleKind",
        StatBlockActionHoleKind::StatBlockRechargeRoll => "StatBlockRechargeRollHoleKind",
    }
}

fn joined_or_none(values: &[&str]) -> String {
    if values.is_empty() {
        "none".to_string()
    } else {
        values.join(",")
    }
}
