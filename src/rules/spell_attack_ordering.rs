#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SpellAttackFrontierStage {
    ActSelection,
    TargetChoice,
    TypedTargetChoice,
    TargetList,
    TargetAllocation,
    DamageTypeAndTargetChoice,
    DamageTypeChoice,
    AttackRoll,
    DamageDice,
    Resolved,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SpellAttackHoleKind {
    TargetChoice,
    SpellTargetList,
    SpellTargetAllocation,
    DamageTypeChoice,
    AttackRoll,
    RolledDice,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SpellAttackFillKind {
    TargetChoice,
    SpellTargetList,
    SpellTargetAllocation,
    DamageTypeChoice,
    AttackRoll,
    RolledDice,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SpellAttackFillOrderingError {
    TargetRequired,
    DamageTypeRequired,
    TargetOrDamageTypeRequired,
    AttackRollRequired,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SpellAttackFillOrderResult {
    Accepted(SpellAttackFrontierStage),
    RequestedEarlier {
        error: SpellAttackFillOrderingError,
        stage: SpellAttackFrontierStage,
    },
    NotOrderingError(SpellAttackFrontierStage),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SpellAttackRuntimeResult {
    Init,
    NeedsHoles,
    Resolved,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SpellAttackOrderingProtocol {
    Init,
    NeedsHoles(Vec<SpellAttackHoleKind>),
    Resolved,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SpellAttackOrderingState {
    pub stage: SpellAttackFrontierStage,
    pub protocol: SpellAttackOrderingProtocol,
    pub last_ordering_error: Option<SpellAttackFillOrderingError>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SpellAttackOrderingProjectionFacts {
    pub stage: SpellAttackFrontierStage,
    pub runtime_result: SpellAttackRuntimeResult,
    pub last_ordering_error: Option<SpellAttackFillOrderingError>,
}

#[must_use]
pub fn spell_attack_ordering_initial_state() -> SpellAttackOrderingState {
    spell_attack_ordering_projection(SpellAttackOrderingProjectionFacts {
        stage: SpellAttackFrontierStage::ActSelection,
        runtime_result: SpellAttackRuntimeResult::Init,
        last_ordering_error: None,
    })
}

#[must_use]
pub fn spell_attack_hole_frontier(stage: SpellAttackFrontierStage) -> Vec<SpellAttackHoleKind> {
    // QNT: cleanroom-input/qnt/battle-runtime/
    // battle-runtime-spell-attack-ordering.qnt.
    match stage {
        SpellAttackFrontierStage::ActSelection | SpellAttackFrontierStage::Resolved => Vec::new(),
        SpellAttackFrontierStage::TargetChoice | SpellAttackFrontierStage::TypedTargetChoice => {
            vec![SpellAttackHoleKind::TargetChoice]
        }
        SpellAttackFrontierStage::TargetList => vec![SpellAttackHoleKind::SpellTargetList],
        SpellAttackFrontierStage::TargetAllocation => {
            vec![SpellAttackHoleKind::SpellTargetAllocation]
        }
        SpellAttackFrontierStage::DamageTypeAndTargetChoice => vec![
            SpellAttackHoleKind::DamageTypeChoice,
            SpellAttackHoleKind::TargetChoice,
        ],
        SpellAttackFrontierStage::DamageTypeChoice => {
            vec![SpellAttackHoleKind::DamageTypeChoice]
        }
        SpellAttackFrontierStage::AttackRoll => vec![SpellAttackHoleKind::AttackRoll],
        SpellAttackFrontierStage::DamageDice => vec![SpellAttackHoleKind::RolledDice],
    }
}

#[must_use]
pub fn spell_attack_after_attack_roll_stage(attack_roll_hits: bool) -> SpellAttackFrontierStage {
    if attack_roll_hits {
        SpellAttackFrontierStage::DamageDice
    } else {
        SpellAttackFrontierStage::Resolved
    }
}

#[must_use]
pub fn spell_attack_fill_order_result(
    stage: SpellAttackFrontierStage,
    fill_kind: SpellAttackFillKind,
    attack_roll_hits: bool,
) -> SpellAttackFillOrderResult {
    match stage {
        SpellAttackFrontierStage::ActSelection => {
            SpellAttackFillOrderResult::NotOrderingError(SpellAttackFrontierStage::ActSelection)
        }
        SpellAttackFrontierStage::TargetChoice => match fill_kind {
            SpellAttackFillKind::TargetChoice => {
                SpellAttackFillOrderResult::Accepted(SpellAttackFrontierStage::AttackRoll)
            }
            SpellAttackFillKind::AttackRoll | SpellAttackFillKind::RolledDice => {
                requested_earlier(SpellAttackFillOrderingError::TargetRequired, stage)
            }
            SpellAttackFillKind::SpellTargetList
            | SpellAttackFillKind::SpellTargetAllocation
            | SpellAttackFillKind::DamageTypeChoice => {
                SpellAttackFillOrderResult::NotOrderingError(stage)
            }
        },
        SpellAttackFrontierStage::TypedTargetChoice => match fill_kind {
            SpellAttackFillKind::TargetChoice => {
                SpellAttackFillOrderResult::Accepted(SpellAttackFrontierStage::AttackRoll)
            }
            SpellAttackFillKind::AttackRoll | SpellAttackFillKind::RolledDice => {
                requested_earlier(SpellAttackFillOrderingError::TargetRequired, stage)
            }
            SpellAttackFillKind::SpellTargetList
            | SpellAttackFillKind::SpellTargetAllocation
            | SpellAttackFillKind::DamageTypeChoice => {
                SpellAttackFillOrderResult::NotOrderingError(stage)
            }
        },
        SpellAttackFrontierStage::TargetList => match fill_kind {
            SpellAttackFillKind::SpellTargetList => {
                SpellAttackFillOrderResult::Accepted(SpellAttackFrontierStage::AttackRoll)
            }
            SpellAttackFillKind::AttackRoll | SpellAttackFillKind::RolledDice => {
                requested_earlier(SpellAttackFillOrderingError::TargetRequired, stage)
            }
            SpellAttackFillKind::TargetChoice
            | SpellAttackFillKind::SpellTargetAllocation
            | SpellAttackFillKind::DamageTypeChoice => {
                SpellAttackFillOrderResult::NotOrderingError(stage)
            }
        },
        SpellAttackFrontierStage::TargetAllocation => match fill_kind {
            SpellAttackFillKind::SpellTargetAllocation => {
                SpellAttackFillOrderResult::Accepted(SpellAttackFrontierStage::AttackRoll)
            }
            SpellAttackFillKind::AttackRoll | SpellAttackFillKind::RolledDice => {
                requested_earlier(SpellAttackFillOrderingError::TargetRequired, stage)
            }
            SpellAttackFillKind::TargetChoice
            | SpellAttackFillKind::SpellTargetList
            | SpellAttackFillKind::DamageTypeChoice => {
                SpellAttackFillOrderResult::NotOrderingError(stage)
            }
        },
        SpellAttackFrontierStage::DamageTypeAndTargetChoice => match fill_kind {
            SpellAttackFillKind::DamageTypeChoice => {
                SpellAttackFillOrderResult::Accepted(SpellAttackFrontierStage::TypedTargetChoice)
            }
            SpellAttackFillKind::TargetChoice => {
                SpellAttackFillOrderResult::Accepted(SpellAttackFrontierStage::DamageTypeChoice)
            }
            SpellAttackFillKind::AttackRoll | SpellAttackFillKind::RolledDice => requested_earlier(
                SpellAttackFillOrderingError::TargetOrDamageTypeRequired,
                stage,
            ),
            SpellAttackFillKind::SpellTargetList | SpellAttackFillKind::SpellTargetAllocation => {
                SpellAttackFillOrderResult::NotOrderingError(stage)
            }
        },
        SpellAttackFrontierStage::DamageTypeChoice => match fill_kind {
            SpellAttackFillKind::DamageTypeChoice => {
                SpellAttackFillOrderResult::Accepted(SpellAttackFrontierStage::AttackRoll)
            }
            SpellAttackFillKind::AttackRoll | SpellAttackFillKind::RolledDice => {
                requested_earlier(SpellAttackFillOrderingError::DamageTypeRequired, stage)
            }
            SpellAttackFillKind::TargetChoice
            | SpellAttackFillKind::SpellTargetList
            | SpellAttackFillKind::SpellTargetAllocation => {
                SpellAttackFillOrderResult::NotOrderingError(stage)
            }
        },
        SpellAttackFrontierStage::AttackRoll => match fill_kind {
            SpellAttackFillKind::AttackRoll => SpellAttackFillOrderResult::Accepted(
                spell_attack_after_attack_roll_stage(attack_roll_hits),
            ),
            SpellAttackFillKind::RolledDice => {
                requested_earlier(SpellAttackFillOrderingError::AttackRollRequired, stage)
            }
            SpellAttackFillKind::TargetChoice
            | SpellAttackFillKind::SpellTargetList
            | SpellAttackFillKind::SpellTargetAllocation
            | SpellAttackFillKind::DamageTypeChoice => {
                SpellAttackFillOrderResult::NotOrderingError(stage)
            }
        },
        SpellAttackFrontierStage::DamageDice => match fill_kind {
            SpellAttackFillKind::RolledDice => {
                SpellAttackFillOrderResult::Accepted(SpellAttackFrontierStage::Resolved)
            }
            SpellAttackFillKind::TargetChoice
            | SpellAttackFillKind::SpellTargetList
            | SpellAttackFillKind::SpellTargetAllocation
            | SpellAttackFillKind::DamageTypeChoice
            | SpellAttackFillKind::AttackRoll => {
                SpellAttackFillOrderResult::NotOrderingError(stage)
            }
        },
        SpellAttackFrontierStage::Resolved => {
            SpellAttackFillOrderResult::NotOrderingError(SpellAttackFrontierStage::Resolved)
        }
    }
}

#[must_use]
pub fn spell_attack_fill_order_accepted_stage(
    result: SpellAttackFillOrderResult,
    _fallback: SpellAttackFrontierStage,
) -> SpellAttackFrontierStage {
    match result {
        SpellAttackFillOrderResult::Accepted(stage)
        | SpellAttackFillOrderResult::RequestedEarlier { stage, .. }
        | SpellAttackFillOrderResult::NotOrderingError(stage) => stage,
    }
}

#[must_use]
pub fn spell_attack_fill_order_runtime_result(
    result: SpellAttackFillOrderResult,
) -> SpellAttackRuntimeResult {
    match result {
        SpellAttackFillOrderResult::Accepted(SpellAttackFrontierStage::Resolved) => {
            SpellAttackRuntimeResult::Resolved
        }
        SpellAttackFillOrderResult::Accepted(_)
        | SpellAttackFillOrderResult::RequestedEarlier { .. }
        | SpellAttackFillOrderResult::NotOrderingError(_) => SpellAttackRuntimeResult::NeedsHoles,
    }
}

#[must_use]
pub fn spell_attack_fill_order_error(
    result: SpellAttackFillOrderResult,
) -> Option<SpellAttackFillOrderingError> {
    match result {
        SpellAttackFillOrderResult::RequestedEarlier { error, .. } => Some(error),
        SpellAttackFillOrderResult::Accepted(_)
        | SpellAttackFillOrderResult::NotOrderingError(_) => None,
    }
}

#[must_use]
pub fn spell_attack_ordering_projection(
    facts: SpellAttackOrderingProjectionFacts,
) -> SpellAttackOrderingState {
    let holes = spell_attack_hole_frontier(facts.stage);
    SpellAttackOrderingState {
        stage: facts.stage,
        protocol: spell_attack_ordering_protocol(holes, facts.runtime_result),
        last_ordering_error: facts.last_ordering_error,
    }
}

#[must_use]
pub fn discover_single_target_spell_attack() -> SpellAttackOrderingState {
    // RAW: cleanroom-input/raw/srd-5.2.1/Playing-the-Game.md
    // "Making an Attack" and "Attack Rolls";
    // cleanroom-input/raw/srd-5.2.1/Spells/Gaining-and-Casting.md
    // "Targets" and "Attack Rolls"; QNT:
    // battle-runtime-spell-attack-ordering.mbt.qnt.
    projection(
        SpellAttackFrontierStage::TargetChoice,
        SpellAttackRuntimeResult::NeedsHoles,
        None,
    )
}

#[must_use]
pub fn submit_attack_roll_before_target_choice() -> SpellAttackOrderingState {
    let stage = SpellAttackFrontierStage::TargetChoice;
    projection_from_result(
        spell_attack_fill_order_result(stage, SpellAttackFillKind::AttackRoll, true),
        stage,
    )
}

#[must_use]
pub fn fill_target_choice() -> SpellAttackOrderingState {
    let stage = SpellAttackFrontierStage::TargetChoice;
    projection_from_result(
        spell_attack_fill_order_result(stage, SpellAttackFillKind::TargetChoice, true),
        stage,
    )
}

#[must_use]
pub fn submit_damage_before_attack_roll() -> SpellAttackOrderingState {
    let stage = SpellAttackFrontierStage::AttackRoll;
    projection_from_result(
        spell_attack_fill_order_result(stage, SpellAttackFillKind::RolledDice, true),
        stage,
    )
}

#[must_use]
pub fn fill_attack_roll_miss() -> SpellAttackOrderingState {
    let stage = SpellAttackFrontierStage::AttackRoll;
    projection_from_result(
        spell_attack_fill_order_result(stage, SpellAttackFillKind::AttackRoll, false),
        stage,
    )
}

#[must_use]
pub fn fill_attack_roll_hit() -> SpellAttackOrderingState {
    let stage = SpellAttackFrontierStage::AttackRoll;
    projection_from_result(
        spell_attack_fill_order_result(stage, SpellAttackFillKind::AttackRoll, true),
        stage,
    )
}

#[must_use]
pub fn fill_damage_dice() -> SpellAttackOrderingState {
    let stage = SpellAttackFrontierStage::DamageDice;
    projection_from_result(
        spell_attack_fill_order_result(stage, SpellAttackFillKind::RolledDice, true),
        stage,
    )
}

#[must_use]
pub fn discover_typed_spell_attack() -> SpellAttackOrderingState {
    // RAW: cleanroom-input/raw/srd-5.2.1/Playing-the-Game.md
    // "Making an Attack", "Damage Rolls", and "Damage Types";
    // cleanroom-input/raw/srd-5.2.1/Spells/Gaining-and-Casting.md
    // "Targets" and "Attack Rolls"; QNT:
    // battle-runtime-spell-attack-ordering.mbt.qnt.
    projection(
        SpellAttackFrontierStage::DamageTypeAndTargetChoice,
        SpellAttackRuntimeResult::NeedsHoles,
        None,
    )
}

#[must_use]
pub fn fill_damage_type_before_target_choice() -> SpellAttackOrderingState {
    let stage = SpellAttackFrontierStage::DamageTypeAndTargetChoice;
    projection_from_result(
        spell_attack_fill_order_result(stage, SpellAttackFillKind::DamageTypeChoice, true),
        stage,
    )
}

#[must_use]
pub fn fill_target_choice_before_damage_type() -> SpellAttackOrderingState {
    let stage = SpellAttackFrontierStage::DamageTypeAndTargetChoice;
    projection_from_result(
        spell_attack_fill_order_result(stage, SpellAttackFillKind::TargetChoice, true),
        stage,
    )
}

#[must_use]
pub fn fill_damage_type_after_target_choice() -> SpellAttackOrderingState {
    let stage = SpellAttackFrontierStage::DamageTypeChoice;
    projection_from_result(
        spell_attack_fill_order_result(stage, SpellAttackFillKind::DamageTypeChoice, true),
        stage,
    )
}

#[must_use]
pub fn fill_target_choice_after_damage_type() -> SpellAttackOrderingState {
    let stage = SpellAttackFrontierStage::TypedTargetChoice;
    projection_from_result(
        spell_attack_fill_order_result(stage, SpellAttackFillKind::TargetChoice, true),
        stage,
    )
}

fn requested_earlier(
    error: SpellAttackFillOrderingError,
    stage: SpellAttackFrontierStage,
) -> SpellAttackFillOrderResult {
    SpellAttackFillOrderResult::RequestedEarlier { error, stage }
}

fn projection_from_result(
    result: SpellAttackFillOrderResult,
    fallback: SpellAttackFrontierStage,
) -> SpellAttackOrderingState {
    projection(
        spell_attack_fill_order_accepted_stage(result, fallback),
        spell_attack_fill_order_runtime_result(result),
        spell_attack_fill_order_error(result),
    )
}

fn projection(
    stage: SpellAttackFrontierStage,
    runtime_result: SpellAttackRuntimeResult,
    last_ordering_error: Option<SpellAttackFillOrderingError>,
) -> SpellAttackOrderingState {
    spell_attack_ordering_projection(SpellAttackOrderingProjectionFacts {
        stage,
        runtime_result,
        last_ordering_error,
    })
}

fn spell_attack_ordering_protocol(
    holes: Vec<SpellAttackHoleKind>,
    runtime_result: SpellAttackRuntimeResult,
) -> SpellAttackOrderingProtocol {
    match runtime_result {
        SpellAttackRuntimeResult::Init => SpellAttackOrderingProtocol::Init,
        SpellAttackRuntimeResult::NeedsHoles if !holes.is_empty() => {
            SpellAttackOrderingProtocol::NeedsHoles(holes)
        }
        SpellAttackRuntimeResult::NeedsHoles | SpellAttackRuntimeResult::Resolved => {
            SpellAttackOrderingProtocol::Resolved
        }
    }
}
