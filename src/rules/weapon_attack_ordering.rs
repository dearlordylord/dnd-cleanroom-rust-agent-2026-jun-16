#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WeaponAttackFrontierStage {
    ActSelection,
    TargetChoice,
    AttackRoll,
    DamageDice,
    Resolved,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WeaponAttackHoleKind {
    TargetChoice,
    AttackRoll,
    RolledDice,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WeaponAttackFillKind {
    TargetChoice,
    AttackRoll,
    RolledDice,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WeaponAttackFillOrderingError {
    TargetChoiceRequired,
    AttackRollRequired,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WeaponAttackFillOrderResult {
    Accepted(WeaponAttackFrontierStage),
    Rejected(WeaponAttackFillOrderingError),
    NotOrderingError(WeaponAttackFrontierStage),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WeaponAttackRuntimeResult {
    Init,
    NeedsHoles,
    Resolved,
    Invalid,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WeaponAttackInvalidReason {
    InvalidFill,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum WeaponAttackOrderingProtocol {
    Init,
    NeedsHoles(Vec<WeaponAttackHoleKind>),
    Resolved,
    Invalid {
        holes: Vec<WeaponAttackHoleKind>,
        reason: WeaponAttackInvalidReason,
    },
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WeaponAttackOrderingState {
    pub stage: WeaponAttackFrontierStage,
    pub protocol: WeaponAttackOrderingProtocol,
    pub last_ordering_error: Option<WeaponAttackFillOrderingError>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct WeaponAttackOrderingProjectionFacts {
    pub stage: WeaponAttackFrontierStage,
    pub runtime_result: WeaponAttackRuntimeResult,
    pub last_ordering_error: Option<WeaponAttackFillOrderingError>,
}

#[must_use]
pub fn weapon_attack_ordering_initial_state() -> WeaponAttackOrderingState {
    weapon_attack_ordering_projection(WeaponAttackOrderingProjectionFacts {
        stage: WeaponAttackFrontierStage::ActSelection,
        runtime_result: WeaponAttackRuntimeResult::Init,
        last_ordering_error: None,
    })
}

#[must_use]
pub fn weapon_attack_hole_frontier(stage: WeaponAttackFrontierStage) -> Vec<WeaponAttackHoleKind> {
    // QNT: cleanroom-input/qnt/battle-runtime/
    // battle-runtime-weapon-attack-ordering.qnt.
    match stage {
        WeaponAttackFrontierStage::ActSelection | WeaponAttackFrontierStage::Resolved => Vec::new(),
        WeaponAttackFrontierStage::TargetChoice => vec![WeaponAttackHoleKind::TargetChoice],
        WeaponAttackFrontierStage::AttackRoll => vec![WeaponAttackHoleKind::AttackRoll],
        WeaponAttackFrontierStage::DamageDice => vec![WeaponAttackHoleKind::RolledDice],
    }
}

#[must_use]
pub fn weapon_attack_after_attack_roll_stage(attack_roll_hits: bool) -> WeaponAttackFrontierStage {
    if attack_roll_hits {
        WeaponAttackFrontierStage::DamageDice
    } else {
        WeaponAttackFrontierStage::Resolved
    }
}

#[must_use]
pub fn weapon_attack_fill_order_result(
    stage: WeaponAttackFrontierStage,
    fill_kind: WeaponAttackFillKind,
    attack_roll_hits: bool,
) -> WeaponAttackFillOrderResult {
    match stage {
        WeaponAttackFrontierStage::ActSelection => {
            WeaponAttackFillOrderResult::NotOrderingError(WeaponAttackFrontierStage::ActSelection)
        }
        WeaponAttackFrontierStage::TargetChoice => match fill_kind {
            WeaponAttackFillKind::TargetChoice => {
                WeaponAttackFillOrderResult::Accepted(WeaponAttackFrontierStage::AttackRoll)
            }
            WeaponAttackFillKind::AttackRoll | WeaponAttackFillKind::RolledDice => {
                WeaponAttackFillOrderResult::Rejected(
                    WeaponAttackFillOrderingError::TargetChoiceRequired,
                )
            }
        },
        WeaponAttackFrontierStage::AttackRoll => match fill_kind {
            WeaponAttackFillKind::AttackRoll => WeaponAttackFillOrderResult::Accepted(
                weapon_attack_after_attack_roll_stage(attack_roll_hits),
            ),
            WeaponAttackFillKind::RolledDice => WeaponAttackFillOrderResult::Rejected(
                WeaponAttackFillOrderingError::AttackRollRequired,
            ),
            WeaponAttackFillKind::TargetChoice => {
                WeaponAttackFillOrderResult::NotOrderingError(stage)
            }
        },
        WeaponAttackFrontierStage::DamageDice => match fill_kind {
            WeaponAttackFillKind::RolledDice => {
                WeaponAttackFillOrderResult::Accepted(WeaponAttackFrontierStage::Resolved)
            }
            WeaponAttackFillKind::TargetChoice | WeaponAttackFillKind::AttackRoll => {
                WeaponAttackFillOrderResult::NotOrderingError(stage)
            }
        },
        WeaponAttackFrontierStage::Resolved => {
            WeaponAttackFillOrderResult::NotOrderingError(WeaponAttackFrontierStage::Resolved)
        }
    }
}

#[must_use]
pub fn weapon_attack_ordering_projection(
    facts: WeaponAttackOrderingProjectionFacts,
) -> WeaponAttackOrderingState {
    let holes = weapon_attack_hole_frontier(facts.stage);
    WeaponAttackOrderingState {
        stage: facts.stage,
        protocol: weapon_attack_ordering_protocol(holes, facts.runtime_result),
        last_ordering_error: facts.last_ordering_error,
    }
}

#[must_use]
pub fn discover_weapon_attack() -> WeaponAttackOrderingState {
    // RAW: cleanroom-input/raw/srd-5.2.1/Playing-the-Game.md
    // "Making an Attack"; QNT:
    // battle-runtime-weapon-attack-ordering.mbt.qnt.
    projection(WeaponAttackOrderingProjectionFacts {
        stage: WeaponAttackFrontierStage::TargetChoice,
        runtime_result: WeaponAttackRuntimeResult::NeedsHoles,
        last_ordering_error: None,
    })
}

#[must_use]
pub fn reject_weapon_attack_roll_before_target_choice() -> WeaponAttackOrderingState {
    let current = discover_weapon_attack();
    projection_from_result(
        weapon_attack_fill_order_result(current.stage, WeaponAttackFillKind::AttackRoll, false),
        current,
    )
}

#[must_use]
pub fn fill_weapon_attack_target_choice() -> WeaponAttackOrderingState {
    let current = discover_weapon_attack();
    projection_from_result(
        weapon_attack_fill_order_result(current.stage, WeaponAttackFillKind::TargetChoice, false),
        current,
    )
}

#[must_use]
pub fn reject_weapon_damage_before_attack_roll() -> WeaponAttackOrderingState {
    let current = fill_weapon_attack_target_choice();
    projection_from_result(
        weapon_attack_fill_order_result(current.stage, WeaponAttackFillKind::RolledDice, false),
        current,
    )
}

#[must_use]
pub fn fill_weapon_attack_roll_miss() -> WeaponAttackOrderingState {
    let current = fill_weapon_attack_target_choice();
    projection_from_result(
        weapon_attack_fill_order_result(current.stage, WeaponAttackFillKind::AttackRoll, false),
        current,
    )
}

#[must_use]
pub fn fill_weapon_attack_roll_hit() -> WeaponAttackOrderingState {
    let current = fill_weapon_attack_target_choice();
    projection_from_result(
        weapon_attack_fill_order_result(current.stage, WeaponAttackFillKind::AttackRoll, true),
        current,
    )
}

#[must_use]
pub fn fill_weapon_attack_damage_dice() -> WeaponAttackOrderingState {
    let current = fill_weapon_attack_roll_hit();
    projection_from_result(
        weapon_attack_fill_order_result(current.stage, WeaponAttackFillKind::RolledDice, true),
        current,
    )
}

fn projection_from_result(
    result: WeaponAttackFillOrderResult,
    current: WeaponAttackOrderingState,
) -> WeaponAttackOrderingState {
    let stage = match result {
        WeaponAttackFillOrderResult::Accepted(stage)
        | WeaponAttackFillOrderResult::NotOrderingError(stage) => stage,
        WeaponAttackFillOrderResult::Rejected(_) => current.stage,
    };
    let runtime_result = match result {
        WeaponAttackFillOrderResult::Accepted(WeaponAttackFrontierStage::Resolved) => {
            WeaponAttackRuntimeResult::Resolved
        }
        WeaponAttackFillOrderResult::Accepted(_)
        | WeaponAttackFillOrderResult::NotOrderingError(_) => WeaponAttackRuntimeResult::NeedsHoles,
        WeaponAttackFillOrderResult::Rejected(_) => WeaponAttackRuntimeResult::Invalid,
    };
    let last_ordering_error = match result {
        WeaponAttackFillOrderResult::Rejected(error) => Some(error),
        WeaponAttackFillOrderResult::Accepted(_)
        | WeaponAttackFillOrderResult::NotOrderingError(_) => None,
    };

    projection(WeaponAttackOrderingProjectionFacts {
        stage,
        runtime_result,
        last_ordering_error,
    })
}

fn projection(facts: WeaponAttackOrderingProjectionFacts) -> WeaponAttackOrderingState {
    weapon_attack_ordering_projection(facts)
}

fn weapon_attack_ordering_protocol(
    holes: Vec<WeaponAttackHoleKind>,
    runtime_result: WeaponAttackRuntimeResult,
) -> WeaponAttackOrderingProtocol {
    match runtime_result {
        WeaponAttackRuntimeResult::Init => WeaponAttackOrderingProtocol::Init,
        WeaponAttackRuntimeResult::NeedsHoles => WeaponAttackOrderingProtocol::NeedsHoles(holes),
        WeaponAttackRuntimeResult::Resolved => WeaponAttackOrderingProtocol::Resolved,
        WeaponAttackRuntimeResult::Invalid => WeaponAttackOrderingProtocol::Invalid {
            holes,
            reason: WeaponAttackInvalidReason::InvalidFill,
        },
    }
}
