#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SaveGatedSpellFrontierStage {
    ActSelection,
    TargetListAndConditionChoice,
    TargetList,
    ConditionChoice,
    DamageSavingThrowOutcome,
    ConditionSavingThrowOutcome,
    DamageDice,
    Resolved,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SaveGatedSpellHoleKind {
    SpellTargetList,
    ConditionChoice,
    SavingThrowOutcome,
    RolledDice,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SaveGatedSpellFillKind {
    SpellTargetList,
    ConditionChoice,
    SavingThrowOutcome,
    RolledDice,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SaveGatedSpellFillOrderingError {
    TargetOrAreaRequired,
    SavingThrowRequired,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SaveGatedSpellFillOrderResult {
    Accepted(SaveGatedSpellFrontierStage),
    Rejected(SaveGatedSpellFillOrderingError),
    NotOrderingError(SaveGatedSpellFrontierStage),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SaveGatedSpellRuntimeResult {
    Init,
    NeedsHoles,
    Resolved,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SaveGatedSpellOrderingProtocol {
    Init,
    NeedsHoles(Vec<SaveGatedSpellHoleKind>),
    Resolved,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SaveGatedSpellOrderingState {
    pub stage: SaveGatedSpellFrontierStage,
    pub protocol: SaveGatedSpellOrderingProtocol,
    pub last_ordering_error: Option<SaveGatedSpellFillOrderingError>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SaveGatedSpellOrderingProjectionFacts {
    pub stage: SaveGatedSpellFrontierStage,
    pub runtime_result: SaveGatedSpellRuntimeResult,
    pub last_ordering_error: Option<SaveGatedSpellFillOrderingError>,
}

#[must_use]
pub fn save_gated_spell_ordering_initial_state() -> SaveGatedSpellOrderingState {
    save_gated_spell_ordering_projection(SaveGatedSpellOrderingProjectionFacts {
        stage: SaveGatedSpellFrontierStage::ActSelection,
        runtime_result: SaveGatedSpellRuntimeResult::Init,
        last_ordering_error: None,
    })
}

#[must_use]
pub fn save_gated_spell_hole_frontier(
    stage: SaveGatedSpellFrontierStage,
) -> Vec<SaveGatedSpellHoleKind> {
    // QNT: cleanroom-input/qnt/battle-runtime/
    // battle-runtime-save-gated-spell-ordering.qnt.
    match stage {
        SaveGatedSpellFrontierStage::ActSelection | SaveGatedSpellFrontierStage::Resolved => {
            Vec::new()
        }
        SaveGatedSpellFrontierStage::TargetListAndConditionChoice => vec![
            SaveGatedSpellHoleKind::SpellTargetList,
            SaveGatedSpellHoleKind::ConditionChoice,
        ],
        SaveGatedSpellFrontierStage::TargetList => {
            vec![SaveGatedSpellHoleKind::SpellTargetList]
        }
        SaveGatedSpellFrontierStage::ConditionChoice => {
            vec![SaveGatedSpellHoleKind::ConditionChoice]
        }
        SaveGatedSpellFrontierStage::DamageSavingThrowOutcome
        | SaveGatedSpellFrontierStage::ConditionSavingThrowOutcome => {
            vec![SaveGatedSpellHoleKind::SavingThrowOutcome]
        }
        SaveGatedSpellFrontierStage::DamageDice => vec![SaveGatedSpellHoleKind::RolledDice],
    }
}

#[must_use]
pub fn save_gated_spell_after_saving_throw_stage(
    damage_dice_required: bool,
) -> SaveGatedSpellFrontierStage {
    if damage_dice_required {
        SaveGatedSpellFrontierStage::DamageDice
    } else {
        SaveGatedSpellFrontierStage::Resolved
    }
}

#[must_use]
pub fn save_gated_spell_fill_order_result(
    stage: SaveGatedSpellFrontierStage,
    fill_kind: SaveGatedSpellFillKind,
    damage_dice_required: bool,
) -> SaveGatedSpellFillOrderResult {
    match stage {
        SaveGatedSpellFrontierStage::ActSelection => {
            SaveGatedSpellFillOrderResult::NotOrderingError(
                SaveGatedSpellFrontierStage::ActSelection,
            )
        }
        SaveGatedSpellFrontierStage::TargetListAndConditionChoice => match fill_kind {
            SaveGatedSpellFillKind::SpellTargetList => SaveGatedSpellFillOrderResult::Accepted(
                SaveGatedSpellFrontierStage::ConditionChoice,
            ),
            SaveGatedSpellFillKind::ConditionChoice => {
                SaveGatedSpellFillOrderResult::Accepted(SaveGatedSpellFrontierStage::TargetList)
            }
            SaveGatedSpellFillKind::SavingThrowOutcome | SaveGatedSpellFillKind::RolledDice => {
                SaveGatedSpellFillOrderResult::Rejected(
                    SaveGatedSpellFillOrderingError::TargetOrAreaRequired,
                )
            }
        },
        SaveGatedSpellFrontierStage::TargetList => match fill_kind {
            SaveGatedSpellFillKind::SpellTargetList => SaveGatedSpellFillOrderResult::Accepted(
                SaveGatedSpellFrontierStage::ConditionSavingThrowOutcome,
            ),
            SaveGatedSpellFillKind::SavingThrowOutcome | SaveGatedSpellFillKind::RolledDice => {
                SaveGatedSpellFillOrderResult::Rejected(
                    SaveGatedSpellFillOrderingError::TargetOrAreaRequired,
                )
            }
            SaveGatedSpellFillKind::ConditionChoice => {
                SaveGatedSpellFillOrderResult::NotOrderingError(
                    SaveGatedSpellFrontierStage::TargetList,
                )
            }
        },
        SaveGatedSpellFrontierStage::ConditionChoice => match fill_kind {
            SaveGatedSpellFillKind::ConditionChoice => SaveGatedSpellFillOrderResult::Accepted(
                SaveGatedSpellFrontierStage::ConditionSavingThrowOutcome,
            ),
            SaveGatedSpellFillKind::SavingThrowOutcome | SaveGatedSpellFillKind::RolledDice => {
                SaveGatedSpellFillOrderResult::Rejected(
                    SaveGatedSpellFillOrderingError::TargetOrAreaRequired,
                )
            }
            SaveGatedSpellFillKind::SpellTargetList => {
                SaveGatedSpellFillOrderResult::NotOrderingError(
                    SaveGatedSpellFrontierStage::ConditionChoice,
                )
            }
        },
        SaveGatedSpellFrontierStage::DamageSavingThrowOutcome => match fill_kind {
            SaveGatedSpellFillKind::SavingThrowOutcome => SaveGatedSpellFillOrderResult::Accepted(
                save_gated_spell_after_saving_throw_stage(damage_dice_required),
            ),
            SaveGatedSpellFillKind::RolledDice => SaveGatedSpellFillOrderResult::Rejected(
                SaveGatedSpellFillOrderingError::SavingThrowRequired,
            ),
            SaveGatedSpellFillKind::SpellTargetList | SaveGatedSpellFillKind::ConditionChoice => {
                SaveGatedSpellFillOrderResult::NotOrderingError(
                    SaveGatedSpellFrontierStage::DamageSavingThrowOutcome,
                )
            }
        },
        SaveGatedSpellFrontierStage::ConditionSavingThrowOutcome => match fill_kind {
            SaveGatedSpellFillKind::SavingThrowOutcome => {
                SaveGatedSpellFillOrderResult::Accepted(SaveGatedSpellFrontierStage::Resolved)
            }
            SaveGatedSpellFillKind::RolledDice => SaveGatedSpellFillOrderResult::Rejected(
                SaveGatedSpellFillOrderingError::SavingThrowRequired,
            ),
            SaveGatedSpellFillKind::SpellTargetList | SaveGatedSpellFillKind::ConditionChoice => {
                SaveGatedSpellFillOrderResult::NotOrderingError(
                    SaveGatedSpellFrontierStage::ConditionSavingThrowOutcome,
                )
            }
        },
        SaveGatedSpellFrontierStage::DamageDice => match fill_kind {
            SaveGatedSpellFillKind::RolledDice => {
                SaveGatedSpellFillOrderResult::Accepted(SaveGatedSpellFrontierStage::Resolved)
            }
            SaveGatedSpellFillKind::SpellTargetList
            | SaveGatedSpellFillKind::ConditionChoice
            | SaveGatedSpellFillKind::SavingThrowOutcome => {
                SaveGatedSpellFillOrderResult::NotOrderingError(
                    SaveGatedSpellFrontierStage::DamageDice,
                )
            }
        },
        SaveGatedSpellFrontierStage::Resolved => {
            SaveGatedSpellFillOrderResult::NotOrderingError(SaveGatedSpellFrontierStage::Resolved)
        }
    }
}

#[must_use]
pub fn save_gated_spell_fill_order_accepted_stage(
    result: SaveGatedSpellFillOrderResult,
    fallback: SaveGatedSpellFrontierStage,
) -> SaveGatedSpellFrontierStage {
    match result {
        SaveGatedSpellFillOrderResult::Accepted(stage)
        | SaveGatedSpellFillOrderResult::NotOrderingError(stage) => stage,
        SaveGatedSpellFillOrderResult::Rejected(_) => fallback,
    }
}

#[must_use]
pub fn save_gated_spell_fill_order_runtime_result(
    result: SaveGatedSpellFillOrderResult,
) -> SaveGatedSpellRuntimeResult {
    match result {
        SaveGatedSpellFillOrderResult::Accepted(SaveGatedSpellFrontierStage::Resolved) => {
            SaveGatedSpellRuntimeResult::Resolved
        }
        SaveGatedSpellFillOrderResult::Accepted(_)
        | SaveGatedSpellFillOrderResult::Rejected(_)
        | SaveGatedSpellFillOrderResult::NotOrderingError(_) => {
            SaveGatedSpellRuntimeResult::NeedsHoles
        }
    }
}

#[must_use]
pub fn save_gated_spell_fill_order_error(
    result: SaveGatedSpellFillOrderResult,
) -> Option<SaveGatedSpellFillOrderingError> {
    match result {
        SaveGatedSpellFillOrderResult::Rejected(error) => Some(error),
        SaveGatedSpellFillOrderResult::Accepted(_)
        | SaveGatedSpellFillOrderResult::NotOrderingError(_) => None,
    }
}

#[must_use]
pub fn save_gated_spell_ordering_projection(
    facts: SaveGatedSpellOrderingProjectionFacts,
) -> SaveGatedSpellOrderingState {
    let holes = save_gated_spell_hole_frontier(facts.stage);
    SaveGatedSpellOrderingState {
        stage: facts.stage,
        protocol: save_gated_spell_ordering_protocol(holes, facts.runtime_result),
        last_ordering_error: facts.last_ordering_error,
    }
}

#[must_use]
pub fn discover_area_save_damage() -> SaveGatedSpellOrderingState {
    // RAW: cleanroom-input/raw/srd-5.2.1/Playing-the-Game.md
    // "Saving Throws", "Damage Rolls", and "Saving Throws and Damage";
    // QNT: battle-runtime-save-gated-spell-ordering.mbt.qnt.
    projection(
        SaveGatedSpellFrontierStage::DamageSavingThrowOutcome,
        SaveGatedSpellRuntimeResult::NeedsHoles,
        None,
    )
}

#[must_use]
pub fn submit_damage_before_saving_throw() -> SaveGatedSpellOrderingState {
    let stage = SaveGatedSpellFrontierStage::DamageSavingThrowOutcome;
    projection_from_result(
        save_gated_spell_fill_order_result(stage, SaveGatedSpellFillKind::RolledDice, true),
        stage,
    )
}

#[must_use]
pub fn fill_area_save_failed() -> SaveGatedSpellOrderingState {
    let stage = SaveGatedSpellFrontierStage::DamageSavingThrowOutcome;
    projection_from_result(
        save_gated_spell_fill_order_result(stage, SaveGatedSpellFillKind::SavingThrowOutcome, true),
        stage,
    )
}

#[must_use]
pub fn fill_area_damage_dice() -> SaveGatedSpellOrderingState {
    let stage = SaveGatedSpellFrontierStage::DamageDice;
    projection_from_result(
        save_gated_spell_fill_order_result(stage, SaveGatedSpellFillKind::RolledDice, true),
        stage,
    )
}

#[must_use]
pub fn discover_target_list_condition_choice() -> SaveGatedSpellOrderingState {
    // RAW: cleanroom-input/raw/srd-5.2.1/Rules-Glossary.md "Target";
    // QNT: battle-runtime-save-gated-spell-ordering.mbt.qnt.
    projection(
        SaveGatedSpellFrontierStage::TargetListAndConditionChoice,
        SaveGatedSpellRuntimeResult::NeedsHoles,
        None,
    )
}

#[must_use]
pub fn fill_target_list_before_condition_choice() -> SaveGatedSpellOrderingState {
    let stage = SaveGatedSpellFrontierStage::TargetListAndConditionChoice;
    projection_from_result(
        save_gated_spell_fill_order_result(stage, SaveGatedSpellFillKind::SpellTargetList, false),
        stage,
    )
}

#[must_use]
pub fn fill_condition_choice_after_target_list() -> SaveGatedSpellOrderingState {
    let stage = SaveGatedSpellFrontierStage::ConditionChoice;
    projection_from_result(
        save_gated_spell_fill_order_result(stage, SaveGatedSpellFillKind::ConditionChoice, false),
        stage,
    )
}

#[must_use]
pub fn fill_condition_choice_before_target_list() -> SaveGatedSpellOrderingState {
    let stage = SaveGatedSpellFrontierStage::TargetListAndConditionChoice;
    projection_from_result(
        save_gated_spell_fill_order_result(stage, SaveGatedSpellFillKind::ConditionChoice, false),
        stage,
    )
}

#[must_use]
pub fn fill_target_list_after_condition_choice() -> SaveGatedSpellOrderingState {
    let stage = SaveGatedSpellFrontierStage::TargetList;
    projection_from_result(
        save_gated_spell_fill_order_result(stage, SaveGatedSpellFillKind::SpellTargetList, false),
        stage,
    )
}

#[must_use]
pub fn fill_condition_saving_throw() -> SaveGatedSpellOrderingState {
    let stage = SaveGatedSpellFrontierStage::ConditionSavingThrowOutcome;
    projection_from_result(
        save_gated_spell_fill_order_result(
            stage,
            SaveGatedSpellFillKind::SavingThrowOutcome,
            false,
        ),
        stage,
    )
}

fn projection_from_result(
    result: SaveGatedSpellFillOrderResult,
    fallback: SaveGatedSpellFrontierStage,
) -> SaveGatedSpellOrderingState {
    projection(
        save_gated_spell_fill_order_accepted_stage(result, fallback),
        save_gated_spell_fill_order_runtime_result(result),
        save_gated_spell_fill_order_error(result),
    )
}

fn projection(
    stage: SaveGatedSpellFrontierStage,
    runtime_result: SaveGatedSpellRuntimeResult,
    last_ordering_error: Option<SaveGatedSpellFillOrderingError>,
) -> SaveGatedSpellOrderingState {
    save_gated_spell_ordering_projection(SaveGatedSpellOrderingProjectionFacts {
        stage,
        runtime_result,
        last_ordering_error,
    })
}

fn save_gated_spell_ordering_protocol(
    holes: Vec<SaveGatedSpellHoleKind>,
    runtime_result: SaveGatedSpellRuntimeResult,
) -> SaveGatedSpellOrderingProtocol {
    match runtime_result {
        SaveGatedSpellRuntimeResult::Init => SaveGatedSpellOrderingProtocol::Init,
        SaveGatedSpellRuntimeResult::NeedsHoles if !holes.is_empty() => {
            SaveGatedSpellOrderingProtocol::NeedsHoles(holes)
        }
        SaveGatedSpellRuntimeResult::NeedsHoles | SaveGatedSpellRuntimeResult::Resolved => {
            SaveGatedSpellOrderingProtocol::Resolved
        }
    }
}
