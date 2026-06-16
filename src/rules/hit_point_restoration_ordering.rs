use crate::rules::feature_resources::{
    apply_feature_resource_hit_point_healing, FeatureResourceHitPoints,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HitPointRestorationFrontierStage {
    ActSelection,
    SpellHealingTargetChoice,
    SpellHealingTargetList,
    SpellHealingRoll,
    FeatureHealingPoolDistribution,
    Resolved,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HitPointRestorationHoleKind {
    TargetChoice,
    SpellTargetList,
    RolledDice,
    HitPointHealingDistribution,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HitPointRestorationFillKind {
    TargetChoice,
    SpellTargetList,
    RolledDice,
    HitPointHealingDistribution,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HitPointRestorationFillOrderingError {
    HealingTargetRequired,
    HealingAmountRequired,
    HealingDistributionRequired,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HitPointRestorationFillOrderResult {
    Accepted(HitPointRestorationFrontierStage),
    RequestedEarlier {
        error: HitPointRestorationFillOrderingError,
        stage: HitPointRestorationFrontierStage,
    },
    NotOrderingError(HitPointRestorationFrontierStage),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HitPointRestorationRuntimeResult {
    Init,
    NeedsHoles,
    Resolved,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum HitPointRestorationProtocol {
    Init,
    NeedsHoles(Vec<HitPointRestorationHoleKind>),
    Resolved,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HitPointRestorationState {
    pub stage: HitPointRestorationFrontierStage,
    pub protocol: HitPointRestorationProtocol,
    pub last_ordering_error: Option<HitPointRestorationFillOrderingError>,
    pub spell_target_hit_points: i16,
    pub spell_target_zero_hit_point_lifecycle_cleared: bool,
    pub feature_target_hit_points: i16,
    pub feature_target_zero_hit_point_lifecycle_cleared: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct HitPointRestorationProjectionFacts {
    pub stage: HitPointRestorationFrontierStage,
    pub runtime_result: HitPointRestorationRuntimeResult,
    pub last_ordering_error: Option<HitPointRestorationFillOrderingError>,
    pub spell_target_hit_points: i16,
    pub spell_target_zero_hit_point_lifecycle_cleared: bool,
    pub feature_target_hit_points: i16,
    pub feature_target_zero_hit_point_lifecycle_cleared: bool,
}

#[must_use]
pub fn hit_point_restoration_initial_state() -> HitPointRestorationState {
    hit_point_restoration_projection(HitPointRestorationProjectionFacts {
        stage: HitPointRestorationFrontierStage::ActSelection,
        runtime_result: HitPointRestorationRuntimeResult::Init,
        last_ordering_error: None,
        spell_target_hit_points: 0,
        spell_target_zero_hit_point_lifecycle_cleared: false,
        feature_target_hit_points: 0,
        feature_target_zero_hit_point_lifecycle_cleared: false,
    })
}

#[must_use]
pub fn hit_point_restoration_hole_frontier(
    stage: HitPointRestorationFrontierStage,
) -> Vec<HitPointRestorationHoleKind> {
    // QNT: cleanroom-input/qnt/battle-runtime/
    // battle-runtime-hit-point-restoration-ordering.qnt.
    match stage {
        HitPointRestorationFrontierStage::ActSelection
        | HitPointRestorationFrontierStage::Resolved => vec![],
        HitPointRestorationFrontierStage::SpellHealingTargetChoice => {
            vec![HitPointRestorationHoleKind::TargetChoice]
        }
        HitPointRestorationFrontierStage::SpellHealingTargetList => {
            vec![HitPointRestorationHoleKind::SpellTargetList]
        }
        HitPointRestorationFrontierStage::SpellHealingRoll => {
            vec![HitPointRestorationHoleKind::RolledDice]
        }
        HitPointRestorationFrontierStage::FeatureHealingPoolDistribution => {
            vec![HitPointRestorationHoleKind::HitPointHealingDistribution]
        }
    }
}

#[must_use]
pub fn hit_point_restoration_fill_order_result(
    stage: HitPointRestorationFrontierStage,
    fill_kind: HitPointRestorationFillKind,
) -> HitPointRestorationFillOrderResult {
    match stage {
        HitPointRestorationFrontierStage::ActSelection => {
            HitPointRestorationFillOrderResult::NotOrderingError(
                HitPointRestorationFrontierStage::ActSelection,
            )
        }
        HitPointRestorationFrontierStage::SpellHealingTargetChoice => match fill_kind {
            HitPointRestorationFillKind::TargetChoice => {
                HitPointRestorationFillOrderResult::Accepted(
                    HitPointRestorationFrontierStage::SpellHealingRoll,
                )
            }
            HitPointRestorationFillKind::RolledDice => {
                HitPointRestorationFillOrderResult::RequestedEarlier {
                    error: HitPointRestorationFillOrderingError::HealingTargetRequired,
                    stage: HitPointRestorationFrontierStage::SpellHealingTargetChoice,
                }
            }
            HitPointRestorationFillKind::SpellTargetList
            | HitPointRestorationFillKind::HitPointHealingDistribution => {
                HitPointRestorationFillOrderResult::NotOrderingError(
                    HitPointRestorationFrontierStage::SpellHealingTargetChoice,
                )
            }
        },
        HitPointRestorationFrontierStage::SpellHealingTargetList => match fill_kind {
            HitPointRestorationFillKind::SpellTargetList => {
                HitPointRestorationFillOrderResult::Accepted(
                    HitPointRestorationFrontierStage::SpellHealingRoll,
                )
            }
            HitPointRestorationFillKind::RolledDice => {
                HitPointRestorationFillOrderResult::RequestedEarlier {
                    error: HitPointRestorationFillOrderingError::HealingTargetRequired,
                    stage: HitPointRestorationFrontierStage::SpellHealingTargetList,
                }
            }
            HitPointRestorationFillKind::TargetChoice
            | HitPointRestorationFillKind::HitPointHealingDistribution => {
                HitPointRestorationFillOrderResult::NotOrderingError(
                    HitPointRestorationFrontierStage::SpellHealingTargetList,
                )
            }
        },
        HitPointRestorationFrontierStage::SpellHealingRoll => match fill_kind {
            HitPointRestorationFillKind::RolledDice => {
                HitPointRestorationFillOrderResult::Accepted(
                    HitPointRestorationFrontierStage::Resolved,
                )
            }
            HitPointRestorationFillKind::TargetChoice
            | HitPointRestorationFillKind::SpellTargetList => {
                HitPointRestorationFillOrderResult::RequestedEarlier {
                    error: HitPointRestorationFillOrderingError::HealingAmountRequired,
                    stage: HitPointRestorationFrontierStage::SpellHealingRoll,
                }
            }
            HitPointRestorationFillKind::HitPointHealingDistribution => {
                HitPointRestorationFillOrderResult::NotOrderingError(
                    HitPointRestorationFrontierStage::SpellHealingRoll,
                )
            }
        },
        HitPointRestorationFrontierStage::FeatureHealingPoolDistribution => match fill_kind {
            HitPointRestorationFillKind::HitPointHealingDistribution => {
                HitPointRestorationFillOrderResult::Accepted(
                    HitPointRestorationFrontierStage::Resolved,
                )
            }
            HitPointRestorationFillKind::RolledDice => {
                HitPointRestorationFillOrderResult::RequestedEarlier {
                    error: HitPointRestorationFillOrderingError::HealingDistributionRequired,
                    stage: HitPointRestorationFrontierStage::FeatureHealingPoolDistribution,
                }
            }
            HitPointRestorationFillKind::TargetChoice
            | HitPointRestorationFillKind::SpellTargetList => {
                HitPointRestorationFillOrderResult::NotOrderingError(
                    HitPointRestorationFrontierStage::FeatureHealingPoolDistribution,
                )
            }
        },
        HitPointRestorationFrontierStage::Resolved => {
            HitPointRestorationFillOrderResult::NotOrderingError(
                HitPointRestorationFrontierStage::Resolved,
            )
        }
    }
}

#[must_use]
pub fn hit_point_restoration_fill_order_accepted_stage(
    result: HitPointRestorationFillOrderResult,
) -> HitPointRestorationFrontierStage {
    match result {
        HitPointRestorationFillOrderResult::Accepted(stage)
        | HitPointRestorationFillOrderResult::RequestedEarlier { stage, .. }
        | HitPointRestorationFillOrderResult::NotOrderingError(stage) => stage,
    }
}

#[must_use]
pub fn hit_point_restoration_fill_order_error(
    result: HitPointRestorationFillOrderResult,
) -> Option<HitPointRestorationFillOrderingError> {
    match result {
        HitPointRestorationFillOrderResult::RequestedEarlier { error, .. } => Some(error),
        HitPointRestorationFillOrderResult::Accepted(_)
        | HitPointRestorationFillOrderResult::NotOrderingError(_) => None,
    }
}

#[must_use]
pub fn hit_point_restoration_fill_order_runtime_result(
    result: HitPointRestorationFillOrderResult,
) -> HitPointRestorationRuntimeResult {
    match result {
        HitPointRestorationFillOrderResult::Accepted(
            HitPointRestorationFrontierStage::Resolved,
        ) => HitPointRestorationRuntimeResult::Resolved,
        HitPointRestorationFillOrderResult::Accepted(_)
        | HitPointRestorationFillOrderResult::RequestedEarlier { .. }
        | HitPointRestorationFillOrderResult::NotOrderingError(_) => {
            HitPointRestorationRuntimeResult::NeedsHoles
        }
    }
}

#[must_use]
pub fn hit_point_restoration_projection(
    facts: HitPointRestorationProjectionFacts,
) -> HitPointRestorationState {
    let holes = hit_point_restoration_hole_frontier(facts.stage);
    HitPointRestorationState {
        stage: facts.stage,
        protocol: hit_point_restoration_protocol(holes, facts.runtime_result),
        last_ordering_error: facts.last_ordering_error,
        spell_target_hit_points: facts.spell_target_hit_points.max(0),
        spell_target_zero_hit_point_lifecycle_cleared: facts
            .spell_target_zero_hit_point_lifecycle_cleared,
        feature_target_hit_points: facts.feature_target_hit_points.max(0),
        feature_target_zero_hit_point_lifecycle_cleared: facts
            .feature_target_zero_hit_point_lifecycle_cleared,
    }
}

#[must_use]
pub fn hit_point_restoration_projection_from_result(
    result: HitPointRestorationFillOrderResult,
    spell_target_hit_points: i16,
    spell_target_zero_hit_point_lifecycle_cleared: bool,
    feature_target_hit_points: i16,
    feature_target_zero_hit_point_lifecycle_cleared: bool,
) -> HitPointRestorationState {
    hit_point_restoration_projection(HitPointRestorationProjectionFacts {
        stage: hit_point_restoration_fill_order_accepted_stage(result),
        runtime_result: hit_point_restoration_fill_order_runtime_result(result),
        last_ordering_error: hit_point_restoration_fill_order_error(result),
        spell_target_hit_points,
        spell_target_zero_hit_point_lifecycle_cleared,
        feature_target_hit_points,
        feature_target_zero_hit_point_lifecycle_cleared,
    })
}

#[must_use]
pub fn restored_hit_points_from_zero(raw_healing: i16) -> (i16, bool) {
    // RAW: cleanroom-input/raw/srd-5.2.1/Playing-the-Game.md "Healing",
    // "Death Saving Throws", and "Falling Unconscious".
    let healed = apply_feature_resource_hit_point_healing(
        FeatureResourceHitPoints {
            current_hit_points: 0,
            hit_point_maximum: 20,
            temporary_hit_points: 0,
        },
        raw_healing,
    );
    (healed.current_hit_points, healed.current_hit_points > 0)
}

fn hit_point_restoration_protocol(
    holes: Vec<HitPointRestorationHoleKind>,
    runtime_result: HitPointRestorationRuntimeResult,
) -> HitPointRestorationProtocol {
    match runtime_result {
        HitPointRestorationRuntimeResult::Init => HitPointRestorationProtocol::Init,
        HitPointRestorationRuntimeResult::NeedsHoles if !holes.is_empty() => {
            HitPointRestorationProtocol::NeedsHoles(holes)
        }
        HitPointRestorationRuntimeResult::NeedsHoles
        | HitPointRestorationRuntimeResult::Resolved => HitPointRestorationProtocol::Resolved,
    }
}
