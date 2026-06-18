use crate::rules::battle_reducer_spine::{
    discover_goblin_rolled_action_attack_control, discover_goblin_static_action_attack_control,
    resolve_stat_block_action_subject, spend_goblin_recharge_gated_rolled_attack,
    start_goblin_multiattack_control, start_goblin_stat_block_battle,
    stat_block_action_projection_from_result, Actor, AttackRollFacts, BattleState,
    StatBlockActionFill, StatBlockActionResolutionResult, StatBlockActionSubject,
};
use crate::rules::stat_block_action_ordering::{
    self, StatBlockActionFillOrderingError, StatBlockActionFrontierStage, StatBlockActionHoleKind,
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
        "doStartMultiattackControl" => project(start_goblin_multiattack_control(
            start_goblin_stat_block_battle(),
        )),
        "doDiscoverRolledActionAttackControl" => project(
            discover_goblin_rolled_action_attack_control(start_goblin_stat_block_battle()),
        ),
        "doDiscoverStaticActionAttackControl" => project(
            discover_goblin_static_action_attack_control(start_goblin_stat_block_battle()),
        ),
        "doRejectAttackRollBeforeTargetChoice" => {
            let (state, subject) = rolled_action_subject();
            project(resolve_stat_block_action_subject(
                state,
                subject,
                StatBlockActionFill::AttackRoll(hit_roll()),
            ))
        }
        "doFillTargetChoice" => {
            let (state, subject) = rolled_action_subject();
            project(resolve_stat_block_action_subject(
                state,
                subject,
                StatBlockActionFill::TargetChoice(Actor::Fighter),
            ))
        }
        "doRejectDamageBeforeAttackRoll" => {
            let (state, subject) = rolled_action_target_chosen_subject();
            project(resolve_stat_block_action_subject(
                state,
                subject,
                StatBlockActionFill::DamageDice(4),
            ))
        }
        "doFillAttackRollMiss" => {
            let (state, subject) = rolled_action_target_chosen_subject();
            project(resolve_stat_block_action_subject(
                state,
                subject,
                StatBlockActionFill::AttackRoll(miss_roll()),
            ))
        }
        "doFillRolledAttackRollHit" => {
            let (state, subject) = rolled_action_target_chosen_subject();
            project(resolve_stat_block_action_subject(
                state,
                subject,
                StatBlockActionFill::AttackRoll(hit_roll()),
            ))
        }
        "doFillStaticAttackRollHit" => {
            let (state, subject) = static_action_target_chosen_subject();
            project(resolve_stat_block_action_subject(
                state,
                subject,
                StatBlockActionFill::AttackRoll(hit_roll()),
            ))
        }
        "doFillDamageDice" => {
            let (state, subject) = rolled_action_attack_hit_subject();
            project(resolve_stat_block_action_subject(
                state,
                subject,
                StatBlockActionFill::DamageDice(4),
            ))
        }
        "doSpendRechargeGatedRolledAttack" => project(spend_goblin_recharge_gated_rolled_attack(
            start_goblin_stat_block_battle(),
        )),
        "doFillRechargeRoll" => {
            let (state, subject) = recharge_roll_subject();
            project(resolve_stat_block_action_subject(
                state,
                subject,
                StatBlockActionFill::RechargeRoll(5),
            ))
        }
        action => panic!("unsupported mbt::actionTaken {action}"),
    }
}

pub fn expected_witness(observed_action_taken: &str) -> StatBlockActionOrderingState {
    match observed_action_taken {
        "doStartMultiattackControl" => stat_block_action_ordering::start_multiattack_control(),
        "doDiscoverRolledActionAttackControl" => {
            stat_block_action_ordering::discover_rolled_action_attack_control()
        }
        "doDiscoverStaticActionAttackControl" => {
            stat_block_action_ordering::discover_static_action_attack_control()
        }
        "doRejectAttackRollBeforeTargetChoice" => {
            stat_block_action_ordering::reject_attack_roll_before_target_choice()
        }
        "doFillTargetChoice" => stat_block_action_ordering::fill_stat_block_target_choice(),
        "doRejectDamageBeforeAttackRoll" => {
            stat_block_action_ordering::reject_damage_before_attack_roll()
        }
        "doFillAttackRollMiss" => stat_block_action_ordering::fill_stat_block_attack_roll_miss(),
        "doFillRolledAttackRollHit" => stat_block_action_ordering::fill_rolled_attack_roll_hit(),
        "doFillStaticAttackRollHit" => stat_block_action_ordering::fill_static_attack_roll_hit(),
        "doFillDamageDice" => stat_block_action_ordering::fill_stat_block_damage_dice(),
        "doSpendRechargeGatedRolledAttack" => {
            stat_block_action_ordering::spend_recharge_gated_rolled_attack()
        }
        "doFillRechargeRoll" => stat_block_action_ordering::fill_stat_block_recharge_roll(),
        action => panic!("unsupported mbt::actionTaken {action}"),
    }
}

fn project(result: StatBlockActionResolutionResult) -> StatBlockActionOrderingState {
    stat_block_action_projection_from_result(&result)
}

fn rolled_action_subject() -> (BattleState, StatBlockActionSubject) {
    expect_needs_holes(discover_goblin_rolled_action_attack_control(
        start_goblin_stat_block_battle(),
    ))
}

fn static_action_subject() -> (BattleState, StatBlockActionSubject) {
    expect_needs_holes(discover_goblin_static_action_attack_control(
        start_goblin_stat_block_battle(),
    ))
}

fn rolled_action_target_chosen_subject() -> (BattleState, StatBlockActionSubject) {
    let (state, subject) = rolled_action_subject();
    expect_needs_holes(resolve_stat_block_action_subject(
        state,
        subject,
        StatBlockActionFill::TargetChoice(Actor::Fighter),
    ))
}

fn static_action_target_chosen_subject() -> (BattleState, StatBlockActionSubject) {
    let (state, subject) = static_action_subject();
    expect_needs_holes(resolve_stat_block_action_subject(
        state,
        subject,
        StatBlockActionFill::TargetChoice(Actor::Fighter),
    ))
}

fn rolled_action_attack_hit_subject() -> (BattleState, StatBlockActionSubject) {
    let (state, subject) = rolled_action_target_chosen_subject();
    expect_needs_holes(resolve_stat_block_action_subject(
        state,
        subject,
        StatBlockActionFill::AttackRoll(hit_roll()),
    ))
}

fn recharge_roll_subject() -> (BattleState, StatBlockActionSubject) {
    expect_needs_holes(spend_goblin_recharge_gated_rolled_attack(
        start_goblin_stat_block_battle(),
    ))
}

fn expect_needs_holes(
    result: StatBlockActionResolutionResult,
) -> (BattleState, StatBlockActionSubject) {
    match result {
        StatBlockActionResolutionResult::NeedsHoles { state, subject, .. } => (state, subject),
        other => panic!("expected stat-block action subject with holes, got {other:?}"),
    }
}

fn hit_roll() -> AttackRollFacts {
    AttackRollFacts {
        total: 16,
        natural_d20: 12,
    }
}

fn miss_roll() -> AttackRollFacts {
    AttackRollFacts {
        total: 2,
        natural_d20: 1,
    }
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
