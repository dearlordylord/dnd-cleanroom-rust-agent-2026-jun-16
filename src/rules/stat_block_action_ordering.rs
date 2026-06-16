#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StatBlockActionFrontierStage {
    ActSelection,
    AttackTargetChoice,
    AttackRoll,
    DamageDice,
    RechargeRoll,
    Resolved,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StatBlockActionHoleKind {
    TargetChoice,
    AttackRoll,
    RolledDice,
    StatBlockRechargeRoll,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StatBlockActionFillKind {
    TargetChoice,
    AttackRoll,
    RolledDice,
    StatBlockRechargeRoll,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StatBlockActionFillOrderingError {
    TargetChoiceRequired,
    AttackRollRequired,
    RechargeRollRequired,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StatBlockActionFillOrderResult {
    Accepted(StatBlockActionFrontierStage),
    Rejected(StatBlockActionFillOrderingError),
    NotOrderingError(StatBlockActionFrontierStage),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StatBlockActionRuntimeResult {
    Init,
    NeedsHoles,
    Resolved,
    Invalid,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StatBlockActionInvalidReason {
    InvalidFill,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum StatBlockActionOrderingProtocol {
    Init,
    NeedsHoles(Vec<StatBlockActionHoleKind>),
    Resolved,
    Invalid {
        holes: Vec<StatBlockActionHoleKind>,
        reason: StatBlockActionInvalidReason,
    },
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StatBlockActionOrderingState {
    pub stage: StatBlockActionFrontierStage,
    pub protocol: StatBlockActionOrderingProtocol,
    pub last_ordering_error: Option<StatBlockActionFillOrderingError>,
    pub multiattack_dispatches_available: i16,
    pub recharge_action_available: bool,
    pub uses_rolled_damage: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct StatBlockActionOrderingProjectionFacts {
    pub stage: StatBlockActionFrontierStage,
    pub runtime_result: StatBlockActionRuntimeResult,
    pub last_ordering_error: Option<StatBlockActionFillOrderingError>,
    pub multiattack_dispatches_available: i16,
    pub recharge_action_available: bool,
    pub uses_rolled_damage: bool,
}

#[must_use]
pub fn stat_block_action_ordering_initial_state() -> StatBlockActionOrderingState {
    stat_block_action_ordering_projection(StatBlockActionOrderingProjectionFacts {
        stage: StatBlockActionFrontierStage::ActSelection,
        runtime_result: StatBlockActionRuntimeResult::Init,
        last_ordering_error: None,
        multiattack_dispatches_available: 0,
        recharge_action_available: true,
        uses_rolled_damage: true,
    })
}

#[must_use]
pub fn stat_block_action_hole_frontier(
    stage: StatBlockActionFrontierStage,
) -> Vec<StatBlockActionHoleKind> {
    // QNT: cleanroom-input/qnt/battle-runtime/
    // battle-runtime-stat-block-action-ordering.qnt.
    match stage {
        StatBlockActionFrontierStage::ActSelection | StatBlockActionFrontierStage::Resolved => {
            Vec::new()
        }
        StatBlockActionFrontierStage::AttackTargetChoice => {
            vec![StatBlockActionHoleKind::TargetChoice]
        }
        StatBlockActionFrontierStage::AttackRoll => vec![StatBlockActionHoleKind::AttackRoll],
        StatBlockActionFrontierStage::DamageDice => vec![StatBlockActionHoleKind::RolledDice],
        StatBlockActionFrontierStage::RechargeRoll => {
            vec![StatBlockActionHoleKind::StatBlockRechargeRoll]
        }
    }
}

#[must_use]
pub fn stat_block_action_after_attack_roll_stage(
    attack_roll_hits: bool,
    uses_rolled_damage: bool,
) -> StatBlockActionFrontierStage {
    if attack_roll_hits && uses_rolled_damage {
        StatBlockActionFrontierStage::DamageDice
    } else {
        StatBlockActionFrontierStage::Resolved
    }
}

#[must_use]
pub fn stat_block_action_fill_order_result(
    stage: StatBlockActionFrontierStage,
    fill_kind: StatBlockActionFillKind,
    attack_roll_hits: bool,
    uses_rolled_damage: bool,
) -> StatBlockActionFillOrderResult {
    match stage {
        StatBlockActionFrontierStage::ActSelection => {
            StatBlockActionFillOrderResult::NotOrderingError(
                StatBlockActionFrontierStage::ActSelection,
            )
        }
        StatBlockActionFrontierStage::AttackTargetChoice => match fill_kind {
            StatBlockActionFillKind::TargetChoice => {
                StatBlockActionFillOrderResult::Accepted(StatBlockActionFrontierStage::AttackRoll)
            }
            StatBlockActionFillKind::AttackRoll | StatBlockActionFillKind::RolledDice => {
                StatBlockActionFillOrderResult::Rejected(
                    StatBlockActionFillOrderingError::TargetChoiceRequired,
                )
            }
            StatBlockActionFillKind::StatBlockRechargeRoll => {
                StatBlockActionFillOrderResult::NotOrderingError(stage)
            }
        },
        StatBlockActionFrontierStage::AttackRoll => match fill_kind {
            StatBlockActionFillKind::AttackRoll => StatBlockActionFillOrderResult::Accepted(
                stat_block_action_after_attack_roll_stage(attack_roll_hits, uses_rolled_damage),
            ),
            StatBlockActionFillKind::RolledDice => StatBlockActionFillOrderResult::Rejected(
                StatBlockActionFillOrderingError::AttackRollRequired,
            ),
            StatBlockActionFillKind::TargetChoice
            | StatBlockActionFillKind::StatBlockRechargeRoll => {
                StatBlockActionFillOrderResult::NotOrderingError(stage)
            }
        },
        StatBlockActionFrontierStage::DamageDice => match fill_kind {
            StatBlockActionFillKind::RolledDice => {
                StatBlockActionFillOrderResult::Accepted(StatBlockActionFrontierStage::Resolved)
            }
            StatBlockActionFillKind::TargetChoice
            | StatBlockActionFillKind::AttackRoll
            | StatBlockActionFillKind::StatBlockRechargeRoll => {
                StatBlockActionFillOrderResult::NotOrderingError(stage)
            }
        },
        StatBlockActionFrontierStage::RechargeRoll => match fill_kind {
            StatBlockActionFillKind::StatBlockRechargeRoll => {
                StatBlockActionFillOrderResult::Accepted(StatBlockActionFrontierStage::Resolved)
            }
            StatBlockActionFillKind::TargetChoice
            | StatBlockActionFillKind::AttackRoll
            | StatBlockActionFillKind::RolledDice => StatBlockActionFillOrderResult::Rejected(
                StatBlockActionFillOrderingError::RechargeRollRequired,
            ),
        },
        StatBlockActionFrontierStage::Resolved => {
            StatBlockActionFillOrderResult::NotOrderingError(StatBlockActionFrontierStage::Resolved)
        }
    }
}

#[must_use]
pub fn stat_block_action_ordering_projection(
    facts: StatBlockActionOrderingProjectionFacts,
) -> StatBlockActionOrderingState {
    let holes = stat_block_action_hole_frontier(facts.stage);
    StatBlockActionOrderingState {
        stage: facts.stage,
        protocol: stat_block_action_ordering_protocol(holes, facts.runtime_result),
        last_ordering_error: facts.last_ordering_error,
        multiattack_dispatches_available: facts.multiattack_dispatches_available,
        recharge_action_available: facts.recharge_action_available,
        uses_rolled_damage: facts.uses_rolled_damage,
    }
}

#[must_use]
pub fn start_multiattack_control() -> StatBlockActionOrderingState {
    // RAW: cleanroom-input/raw/srd-5.2.1/Monsters/Overview.md "Multiattack";
    // QNT: battle-runtime-stat-block-action-ordering.mbt.qnt.
    projection(StatBlockActionOrderingProjectionFacts {
        stage: StatBlockActionFrontierStage::AttackTargetChoice,
        runtime_result: StatBlockActionRuntimeResult::NeedsHoles,
        last_ordering_error: None,
        multiattack_dispatches_available: 2,
        recharge_action_available: false,
        uses_rolled_damage: true,
    })
}

#[must_use]
pub fn discover_rolled_action_attack_control() -> StatBlockActionOrderingState {
    // RAW: cleanroom-input/raw/srd-5.2.1/Monsters/Overview.md
    // "Damage Notation"; QNT:
    // battle-runtime-stat-block-action-ordering.mbt.qnt.
    projection(StatBlockActionOrderingProjectionFacts {
        stage: StatBlockActionFrontierStage::AttackTargetChoice,
        runtime_result: StatBlockActionRuntimeResult::NeedsHoles,
        last_ordering_error: None,
        multiattack_dispatches_available: 0,
        recharge_action_available: true,
        uses_rolled_damage: true,
    })
}

#[must_use]
pub fn discover_static_action_attack_control() -> StatBlockActionOrderingState {
    projection(StatBlockActionOrderingProjectionFacts {
        stage: StatBlockActionFrontierStage::AttackTargetChoice,
        runtime_result: StatBlockActionRuntimeResult::NeedsHoles,
        last_ordering_error: None,
        multiattack_dispatches_available: 0,
        recharge_action_available: false,
        uses_rolled_damage: false,
    })
}

#[must_use]
pub fn reject_attack_roll_before_target_choice() -> StatBlockActionOrderingState {
    let current = discover_rolled_action_attack_control();
    projection_from_result(
        stat_block_action_fill_order_result(
            current.stage,
            StatBlockActionFillKind::AttackRoll,
            true,
            true,
        ),
        current,
    )
}

#[must_use]
pub fn fill_stat_block_target_choice() -> StatBlockActionOrderingState {
    let current = discover_rolled_action_attack_control();
    projection_from_result(
        stat_block_action_fill_order_result(
            current.stage,
            StatBlockActionFillKind::TargetChoice,
            true,
            true,
        ),
        current,
    )
}

#[must_use]
pub fn reject_damage_before_attack_roll() -> StatBlockActionOrderingState {
    let current = fill_stat_block_target_choice();
    projection_from_result(
        stat_block_action_fill_order_result(
            current.stage,
            StatBlockActionFillKind::RolledDice,
            true,
            true,
        ),
        current,
    )
}

#[must_use]
pub fn fill_stat_block_attack_roll_miss() -> StatBlockActionOrderingState {
    let current = fill_stat_block_target_choice();
    let result = stat_block_action_fill_order_result(
        current.stage,
        StatBlockActionFillKind::AttackRoll,
        false,
        current.uses_rolled_damage,
    );
    projection_from_result(
        result,
        StatBlockActionOrderingState {
            recharge_action_available: false,
            ..current
        },
    )
}

#[must_use]
pub fn fill_rolled_attack_roll_hit() -> StatBlockActionOrderingState {
    let current = fill_stat_block_target_choice();
    projection_from_result(
        stat_block_action_fill_order_result(
            current.stage,
            StatBlockActionFillKind::AttackRoll,
            true,
            true,
        ),
        current,
    )
}

#[must_use]
pub fn fill_static_attack_roll_hit() -> StatBlockActionOrderingState {
    let target_choice = fill_static_target_choice();
    let result = stat_block_action_fill_order_result(
        target_choice.stage,
        StatBlockActionFillKind::AttackRoll,
        true,
        false,
    );
    projection_from_result(
        result,
        StatBlockActionOrderingState {
            recharge_action_available: false,
            ..target_choice
        },
    )
}

#[must_use]
pub fn fill_stat_block_damage_dice() -> StatBlockActionOrderingState {
    let current = fill_rolled_attack_roll_hit();
    let result = stat_block_action_fill_order_result(
        current.stage,
        StatBlockActionFillKind::RolledDice,
        true,
        true,
    );
    projection_from_result(
        result,
        StatBlockActionOrderingState {
            recharge_action_available: false,
            ..current
        },
    )
}

#[must_use]
pub fn spend_recharge_gated_rolled_attack() -> StatBlockActionOrderingState {
    // RAW: cleanroom-input/raw/srd-5.2.1/Monsters/Overview.md
    // "Recharge X-Y"; QNT:
    // battle-runtime-stat-block-action-ordering.mbt.qnt.
    projection(StatBlockActionOrderingProjectionFacts {
        stage: StatBlockActionFrontierStage::RechargeRoll,
        runtime_result: StatBlockActionRuntimeResult::NeedsHoles,
        last_ordering_error: None,
        multiattack_dispatches_available: 0,
        recharge_action_available: false,
        uses_rolled_damage: true,
    })
}

#[must_use]
pub fn fill_stat_block_recharge_roll() -> StatBlockActionOrderingState {
    let current = spend_recharge_gated_rolled_attack();
    projection_from_result(
        stat_block_action_fill_order_result(
            current.stage,
            StatBlockActionFillKind::StatBlockRechargeRoll,
            false,
            false,
        ),
        StatBlockActionOrderingState {
            recharge_action_available: true,
            ..current
        },
    )
}

fn fill_static_target_choice() -> StatBlockActionOrderingState {
    let current = discover_static_action_attack_control();
    projection_from_result(
        stat_block_action_fill_order_result(
            current.stage,
            StatBlockActionFillKind::TargetChoice,
            true,
            false,
        ),
        current,
    )
}

fn projection_from_result(
    result: StatBlockActionFillOrderResult,
    current: StatBlockActionOrderingState,
) -> StatBlockActionOrderingState {
    let stage = match result {
        StatBlockActionFillOrderResult::Accepted(stage)
        | StatBlockActionFillOrderResult::NotOrderingError(stage) => stage,
        StatBlockActionFillOrderResult::Rejected(_) => current.stage,
    };
    let runtime_result = match result {
        StatBlockActionFillOrderResult::Accepted(StatBlockActionFrontierStage::Resolved) => {
            StatBlockActionRuntimeResult::Resolved
        }
        StatBlockActionFillOrderResult::Accepted(_)
        | StatBlockActionFillOrderResult::NotOrderingError(_) => {
            StatBlockActionRuntimeResult::NeedsHoles
        }
        StatBlockActionFillOrderResult::Rejected(_) => StatBlockActionRuntimeResult::Invalid,
    };
    let last_ordering_error = match result {
        StatBlockActionFillOrderResult::Rejected(error) => Some(error),
        StatBlockActionFillOrderResult::Accepted(_)
        | StatBlockActionFillOrderResult::NotOrderingError(_) => None,
    };

    projection(StatBlockActionOrderingProjectionFacts {
        stage,
        runtime_result,
        last_ordering_error,
        multiattack_dispatches_available: current.multiattack_dispatches_available,
        recharge_action_available: current.recharge_action_available,
        uses_rolled_damage: current.uses_rolled_damage,
    })
}

fn projection(facts: StatBlockActionOrderingProjectionFacts) -> StatBlockActionOrderingState {
    stat_block_action_ordering_projection(facts)
}

fn stat_block_action_ordering_protocol(
    holes: Vec<StatBlockActionHoleKind>,
    runtime_result: StatBlockActionRuntimeResult,
) -> StatBlockActionOrderingProtocol {
    match runtime_result {
        StatBlockActionRuntimeResult::Init => StatBlockActionOrderingProtocol::Init,
        StatBlockActionRuntimeResult::NeedsHoles => {
            StatBlockActionOrderingProtocol::NeedsHoles(holes)
        }
        StatBlockActionRuntimeResult::Resolved => StatBlockActionOrderingProtocol::Resolved,
        StatBlockActionRuntimeResult::Invalid => StatBlockActionOrderingProtocol::Invalid {
            holes,
            reason: StatBlockActionInvalidReason::InvalidFill,
        },
    }
}
