use crate::rules::hit_point_restoration_ordering::{
    hit_point_restoration_fill_order_result, hit_point_restoration_projection,
    hit_point_restoration_projection_from_result, restored_hit_points_from_zero,
    HitPointRestorationFillKind, HitPointRestorationFillOrderingError,
    HitPointRestorationFrontierStage, HitPointRestorationHoleKind,
    HitPointRestorationProjectionFacts, HitPointRestorationProtocol,
    HitPointRestorationRuntimeResult, HitPointRestorationState,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HitPointRestorationOrderingWitness {
    pub stage: &'static str,
    pub protocol_result: &'static str,
    pub protocol_holes: Vec<&'static str>,
    pub last_ordering_error: &'static str,
    pub spell_target_hp: i16,
    pub spell_target_zero_hp_lifecycle_cleared: bool,
    pub feature_target_hp: i16,
    pub feature_target_zero_hp_lifecycle_cleared: bool,
}

pub const BRANCH_ACTIONS: [&str; 9] = [
    "doDiscoverSingleTargetSpellHealing",
    "doSubmitHealingRollBeforeTargetChoice",
    "doFillSpellHealingTargetChoice",
    "doDiscoverTargetListSpellHealing",
    "doSubmitHealingRollBeforeTargetList",
    "doFillSpellHealingTargetList",
    "doFillSpellHealingRoll",
    "doDiscoverFeatureHealingPool",
    "doFillFeatureHealingDistribution",
];

pub fn replay_observed_action(observed_action_taken: &str) -> HitPointRestorationOrderingWitness {
    match observed_action_taken {
        "doDiscoverSingleTargetSpellHealing" => witness_from_state(discover_single_target_spell()),
        "doSubmitHealingRollBeforeTargetChoice" => {
            witness_from_state(submit_roll_before_target_choice())
        }
        "doFillSpellHealingTargetChoice" => witness_from_state(fill_spell_target_choice()),
        "doDiscoverTargetListSpellHealing" => witness_from_state(discover_target_list_spell()),
        "doSubmitHealingRollBeforeTargetList" => {
            witness_from_state(submit_roll_before_target_list())
        }
        "doFillSpellHealingTargetList" => witness_from_state(fill_spell_target_list()),
        "doFillSpellHealingRoll" => witness_from_state(fill_spell_healing_roll()),
        "doDiscoverFeatureHealingPool" => witness_from_state(discover_feature_healing_pool()),
        "doFillFeatureHealingDistribution" => witness_from_state(fill_feature_distribution()),
        action => panic!("unsupported mbt::actionTaken {action}"),
    }
}

pub fn expected_witness(observed_action_taken: &str) -> HitPointRestorationOrderingWitness {
    replay_observed_action(observed_action_taken)
}

pub fn projection_payload(witness: &HitPointRestorationOrderingWitness) -> String {
    [
        format!("qStage={}", witness.stage),
        format!("protocolResult={}", witness.protocol_result),
        format!("protocolHoles={}", joined_or_none(&witness.protocol_holes)),
        format!("qLastOrderingError={}", witness.last_ordering_error),
        format!("qSpellTargetHp={}", witness.spell_target_hp),
        format!(
            "qSpellTargetZeroHpLifecycleCleared={}",
            witness.spell_target_zero_hp_lifecycle_cleared
        ),
        format!("qFeatureTargetHp={}", witness.feature_target_hp),
        format!(
            "qFeatureTargetZeroHpLifecycleCleared={}",
            witness.feature_target_zero_hp_lifecycle_cleared
        ),
    ]
    .join("\n")
}

fn discover_single_target_spell() -> HitPointRestorationState {
    projection(
        HitPointRestorationFrontierStage::SpellHealingTargetChoice,
        HitPointRestorationRuntimeResult::NeedsHoles,
        None,
        0,
        false,
        0,
        false,
    )
}

fn submit_roll_before_target_choice() -> HitPointRestorationState {
    let stage = HitPointRestorationFrontierStage::SpellHealingTargetChoice;
    let result =
        hit_point_restoration_fill_order_result(stage, HitPointRestorationFillKind::RolledDice);
    hit_point_restoration_projection_from_result(result, 0, false, 0, false)
}

fn fill_spell_target_choice() -> HitPointRestorationState {
    let stage = HitPointRestorationFrontierStage::SpellHealingTargetChoice;
    let result =
        hit_point_restoration_fill_order_result(stage, HitPointRestorationFillKind::TargetChoice);
    hit_point_restoration_projection_from_result(result, 0, false, 0, false)
}

fn discover_target_list_spell() -> HitPointRestorationState {
    projection(
        HitPointRestorationFrontierStage::SpellHealingTargetList,
        HitPointRestorationRuntimeResult::NeedsHoles,
        None,
        0,
        false,
        0,
        false,
    )
}

fn submit_roll_before_target_list() -> HitPointRestorationState {
    let stage = HitPointRestorationFrontierStage::SpellHealingTargetList;
    let result =
        hit_point_restoration_fill_order_result(stage, HitPointRestorationFillKind::RolledDice);
    hit_point_restoration_projection_from_result(result, 0, false, 0, false)
}

fn fill_spell_target_list() -> HitPointRestorationState {
    let stage = HitPointRestorationFrontierStage::SpellHealingTargetList;
    let result = hit_point_restoration_fill_order_result(
        stage,
        HitPointRestorationFillKind::SpellTargetList,
    );
    hit_point_restoration_projection_from_result(result, 0, false, 0, false)
}

fn fill_spell_healing_roll() -> HitPointRestorationState {
    let stage = HitPointRestorationFrontierStage::SpellHealingRoll;
    let result =
        hit_point_restoration_fill_order_result(stage, HitPointRestorationFillKind::RolledDice);
    let (spell_hp, spell_cleared) = restored_hit_points_from_zero(5);
    hit_point_restoration_projection_from_result(result, spell_hp, spell_cleared, 0, false)
}

fn discover_feature_healing_pool() -> HitPointRestorationState {
    projection(
        HitPointRestorationFrontierStage::FeatureHealingPoolDistribution,
        HitPointRestorationRuntimeResult::NeedsHoles,
        None,
        0,
        false,
        0,
        false,
    )
}

fn fill_feature_distribution() -> HitPointRestorationState {
    let stage = HitPointRestorationFrontierStage::FeatureHealingPoolDistribution;
    let result = hit_point_restoration_fill_order_result(
        stage,
        HitPointRestorationFillKind::HitPointHealingDistribution,
    );
    let (feature_hp, feature_cleared) = restored_hit_points_from_zero(8);
    hit_point_restoration_projection_from_result(result, 0, false, feature_hp, feature_cleared)
}

fn projection(
    stage: HitPointRestorationFrontierStage,
    runtime_result: HitPointRestorationRuntimeResult,
    last_ordering_error: Option<HitPointRestorationFillOrderingError>,
    spell_target_hit_points: i16,
    spell_target_zero_hit_point_lifecycle_cleared: bool,
    feature_target_hit_points: i16,
    feature_target_zero_hit_point_lifecycle_cleared: bool,
) -> HitPointRestorationState {
    hit_point_restoration_projection(HitPointRestorationProjectionFacts {
        stage,
        runtime_result,
        last_ordering_error,
        spell_target_hit_points,
        spell_target_zero_hit_point_lifecycle_cleared,
        feature_target_hit_points,
        feature_target_zero_hit_point_lifecycle_cleared,
    })
}

fn witness_from_state(state: HitPointRestorationState) -> HitPointRestorationOrderingWitness {
    HitPointRestorationOrderingWitness {
        stage: stage_ref(state.stage),
        protocol_result: protocol_result_ref(&state.protocol),
        protocol_holes: protocol_holes(&state.protocol),
        last_ordering_error: ordering_error_ref(state.last_ordering_error),
        spell_target_hp: state.spell_target_hit_points,
        spell_target_zero_hp_lifecycle_cleared: state.spell_target_zero_hit_point_lifecycle_cleared,
        feature_target_hp: state.feature_target_hit_points,
        feature_target_zero_hp_lifecycle_cleared: state
            .feature_target_zero_hit_point_lifecycle_cleared,
    }
}

fn stage_ref(stage: HitPointRestorationFrontierStage) -> &'static str {
    match stage {
        HitPointRestorationFrontierStage::ActSelection => "HitPointRestorationActSelectionStage",
        HitPointRestorationFrontierStage::SpellHealingTargetChoice => {
            "SpellHealingTargetChoiceStage"
        }
        HitPointRestorationFrontierStage::SpellHealingTargetList => "SpellHealingTargetListStage",
        HitPointRestorationFrontierStage::SpellHealingRoll => "SpellHealingRollStage",
        HitPointRestorationFrontierStage::FeatureHealingPoolDistribution => {
            "FeatureHealingPoolDistributionStage"
        }
        HitPointRestorationFrontierStage::Resolved => "HitPointRestorationResolvedStage",
    }
}

fn protocol_result_ref(protocol: &HitPointRestorationProtocol) -> &'static str {
    match protocol {
        HitPointRestorationProtocol::Init => "init",
        HitPointRestorationProtocol::NeedsHoles(_) => "needsHoles",
        HitPointRestorationProtocol::Resolved => "resolved",
    }
}

fn protocol_holes(protocol: &HitPointRestorationProtocol) -> Vec<&'static str> {
    match protocol {
        HitPointRestorationProtocol::NeedsHoles(holes) => {
            holes.iter().copied().map(hole_ref).collect()
        }
        HitPointRestorationProtocol::Init | HitPointRestorationProtocol::Resolved => vec![],
    }
}

fn hole_ref(hole: HitPointRestorationHoleKind) -> &'static str {
    match hole {
        HitPointRestorationHoleKind::TargetChoice => "TargetChoiceHoleKind",
        HitPointRestorationHoleKind::SpellTargetList => "SpellTargetListHoleKind",
        HitPointRestorationHoleKind::RolledDice => "RolledDiceHoleKind",
        HitPointRestorationHoleKind::HitPointHealingDistribution => {
            "HitPointHealingDistributionHoleKind"
        }
    }
}

fn ordering_error_ref(error: Option<HitPointRestorationFillOrderingError>) -> &'static str {
    match error {
        Some(HitPointRestorationFillOrderingError::HealingTargetRequired) => {
            "healingTargetRequired"
        }
        Some(HitPointRestorationFillOrderingError::HealingAmountRequired) => {
            "healingAmountRequired"
        }
        Some(HitPointRestorationFillOrderingError::HealingDistributionRequired) => {
            "healingDistributionRequired"
        }
        None => "",
    }
}

fn joined_or_none(values: &[&'static str]) -> String {
    if values.is_empty() {
        "none".to_string()
    } else {
        values.join(",")
    }
}
