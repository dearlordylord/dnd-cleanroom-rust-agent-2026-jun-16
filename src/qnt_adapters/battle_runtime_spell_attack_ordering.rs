use crate::rules::spell_attack_ordering::{
    discover_single_target_spell_attack, discover_typed_spell_attack, fill_attack_roll_hit,
    fill_attack_roll_miss, fill_damage_dice, fill_damage_type_after_target_choice,
    fill_damage_type_before_target_choice, fill_target_choice,
    fill_target_choice_after_damage_type, fill_target_choice_before_damage_type,
    submit_attack_roll_before_target_choice, submit_damage_before_attack_roll,
    SpellAttackFillOrderingError, SpellAttackFrontierStage, SpellAttackHoleKind,
    SpellAttackOrderingProtocol, SpellAttackOrderingState,
};

pub const BRANCH_ACTIONS: [&str; 12] = [
    "doDiscoverSingleTargetSpellAttack",
    "doSubmitAttackRollBeforeTargetChoice",
    "doFillTargetChoice",
    "doSubmitDamageBeforeAttackRoll",
    "doFillAttackRollMiss",
    "doFillAttackRollHit",
    "doFillDamageDice",
    "doDiscoverTypedSpellAttack",
    "doFillDamageTypeBeforeTargetChoice",
    "doFillTargetChoiceBeforeDamageType",
    "doFillDamageTypeAfterTargetChoice",
    "doFillTargetChoiceAfterDamageType",
];

pub fn replay_observed_action(observed_action_taken: &str) -> SpellAttackOrderingState {
    match observed_action_taken {
        "doDiscoverSingleTargetSpellAttack" => discover_single_target_spell_attack(),
        "doSubmitAttackRollBeforeTargetChoice" => submit_attack_roll_before_target_choice(),
        "doFillTargetChoice" => fill_target_choice(),
        "doSubmitDamageBeforeAttackRoll" => submit_damage_before_attack_roll(),
        "doFillAttackRollMiss" => fill_attack_roll_miss(),
        "doFillAttackRollHit" => fill_attack_roll_hit(),
        "doFillDamageDice" => fill_damage_dice(),
        "doDiscoverTypedSpellAttack" => discover_typed_spell_attack(),
        "doFillDamageTypeBeforeTargetChoice" => fill_damage_type_before_target_choice(),
        "doFillTargetChoiceBeforeDamageType" => fill_target_choice_before_damage_type(),
        "doFillDamageTypeAfterTargetChoice" => fill_damage_type_after_target_choice(),
        "doFillTargetChoiceAfterDamageType" => fill_target_choice_after_damage_type(),
        action => panic!("unsupported mbt::actionTaken {action}"),
    }
}

pub fn expected_witness(observed_action_taken: &str) -> SpellAttackOrderingState {
    replay_observed_action(observed_action_taken)
}

pub fn projection_payload(state: &SpellAttackOrderingState) -> String {
    let protocol_holes = protocol_holes(&state.protocol);
    [
        format!("qStage={}", stage_ref(state.stage)),
        format!("qHoles={}", joined_or_none(&protocol_holes)),
        format!(
            "qLastOrderingError={}",
            ordering_error_ref(state.last_ordering_error)
        ),
        format!("protocolResult={}", protocol_result_ref(&state.protocol)),
        format!("protocolHoles={}", joined_or_none(&protocol_holes)),
    ]
    .join("\n")
}

fn stage_ref(stage: SpellAttackFrontierStage) -> &'static str {
    match stage {
        SpellAttackFrontierStage::ActSelection => "SpellAttackActSelectionStage",
        SpellAttackFrontierStage::TargetChoice => "SpellAttackTargetChoiceStage",
        SpellAttackFrontierStage::TypedTargetChoice => "SpellAttackTypedTargetChoiceStage",
        SpellAttackFrontierStage::TargetList => "SpellAttackTargetListStage",
        SpellAttackFrontierStage::TargetAllocation => "SpellAttackTargetAllocationStage",
        SpellAttackFrontierStage::DamageTypeAndTargetChoice => {
            "SpellAttackDamageTypeAndTargetChoiceStage"
        }
        SpellAttackFrontierStage::DamageTypeChoice => "SpellAttackDamageTypeChoiceStage",
        SpellAttackFrontierStage::AttackRoll => "SpellAttackAttackRollStage",
        SpellAttackFrontierStage::DamageDice => "SpellAttackDamageDiceStage",
        SpellAttackFrontierStage::Resolved => "SpellAttackResolvedStage",
    }
}

fn protocol_result_ref(protocol: &SpellAttackOrderingProtocol) -> &'static str {
    match protocol {
        SpellAttackOrderingProtocol::Init => "init",
        SpellAttackOrderingProtocol::NeedsHoles(_) => "needsHoles",
        SpellAttackOrderingProtocol::Resolved => "resolved",
    }
}

fn protocol_holes(protocol: &SpellAttackOrderingProtocol) -> Vec<&'static str> {
    match protocol {
        SpellAttackOrderingProtocol::NeedsHoles(holes) => holes.iter().map(hole_ref).collect(),
        SpellAttackOrderingProtocol::Init | SpellAttackOrderingProtocol::Resolved => Vec::new(),
    }
}

fn hole_ref(hole: &SpellAttackHoleKind) -> &'static str {
    match hole {
        SpellAttackHoleKind::TargetChoice => "TargetChoiceHoleKind",
        SpellAttackHoleKind::SpellTargetList => "SpellTargetListHoleKind",
        SpellAttackHoleKind::SpellTargetAllocation => "SpellTargetAllocationHoleKind",
        SpellAttackHoleKind::DamageTypeChoice => "DamageTypeChoiceHoleKind",
        SpellAttackHoleKind::AttackRoll => "AttackRollHoleKind",
        SpellAttackHoleKind::RolledDice => "RolledDiceHoleKind",
    }
}

fn ordering_error_ref(error: Option<SpellAttackFillOrderingError>) -> &'static str {
    match error {
        Some(SpellAttackFillOrderingError::TargetRequired) => "targetRequired",
        Some(SpellAttackFillOrderingError::DamageTypeRequired) => "damageTypeRequired",
        Some(SpellAttackFillOrderingError::TargetOrDamageTypeRequired) => {
            "targetOrDamageTypeRequired"
        }
        Some(SpellAttackFillOrderingError::AttackRollRequired) => "attackRollRequired",
        None => "",
    }
}

fn joined_or_none(values: &[&str]) -> String {
    if values.is_empty() {
        "none".to_string()
    } else {
        values.join(",")
    }
}
