use crate::rules::save_gated_spell_ordering::{
    discover_area_save_damage, discover_target_list_condition_choice, fill_area_damage_dice,
    fill_area_save_failed, fill_condition_choice_after_target_list,
    fill_condition_choice_before_target_list, fill_condition_saving_throw,
    fill_target_list_after_condition_choice, fill_target_list_before_condition_choice,
    submit_damage_before_saving_throw, SaveGatedSpellFillOrderingError,
    SaveGatedSpellFrontierStage, SaveGatedSpellHoleKind, SaveGatedSpellOrderingProtocol,
    SaveGatedSpellOrderingState,
};

pub const BRANCH_ACTIONS: [&str; 10] = [
    "doDiscoverAreaSaveDamage",
    "doSubmitDamageBeforeSavingThrow",
    "doFillAreaSaveFailed",
    "doFillAreaDamageDice",
    "doDiscoverTargetListConditionChoice",
    "doFillTargetListBeforeConditionChoice",
    "doFillConditionChoiceAfterTargetList",
    "doFillConditionChoiceBeforeTargetList",
    "doFillTargetListAfterConditionChoice",
    "doFillConditionSavingThrow",
];

pub fn replay_observed_action(observed_action_taken: &str) -> SaveGatedSpellOrderingState {
    match observed_action_taken {
        "doDiscoverAreaSaveDamage" => discover_area_save_damage(),
        "doSubmitDamageBeforeSavingThrow" => submit_damage_before_saving_throw(),
        "doFillAreaSaveFailed" => fill_area_save_failed(),
        "doFillAreaDamageDice" => fill_area_damage_dice(),
        "doDiscoverTargetListConditionChoice" => discover_target_list_condition_choice(),
        "doFillTargetListBeforeConditionChoice" => fill_target_list_before_condition_choice(),
        "doFillConditionChoiceAfterTargetList" => fill_condition_choice_after_target_list(),
        "doFillConditionChoiceBeforeTargetList" => fill_condition_choice_before_target_list(),
        "doFillTargetListAfterConditionChoice" => fill_target_list_after_condition_choice(),
        "doFillConditionSavingThrow" => fill_condition_saving_throw(),
        action => panic!("unsupported mbt::actionTaken {action}"),
    }
}

pub fn expected_witness(observed_action_taken: &str) -> SaveGatedSpellOrderingState {
    replay_observed_action(observed_action_taken)
}

pub fn projection_payload(state: &SaveGatedSpellOrderingState) -> String {
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

fn stage_ref(stage: SaveGatedSpellFrontierStage) -> &'static str {
    match stage {
        SaveGatedSpellFrontierStage::ActSelection => "SaveGatedSpellActSelectionStage",
        SaveGatedSpellFrontierStage::TargetListAndConditionChoice => {
            "SaveGatedSpellTargetListAndConditionChoiceStage"
        }
        SaveGatedSpellFrontierStage::TargetList => "SaveGatedSpellTargetListStage",
        SaveGatedSpellFrontierStage::ConditionChoice => "SaveGatedSpellConditionChoiceStage",
        SaveGatedSpellFrontierStage::DamageSavingThrowOutcome => {
            "SaveGatedSpellDamageSavingThrowOutcomeStage"
        }
        SaveGatedSpellFrontierStage::ConditionSavingThrowOutcome => {
            "SaveGatedSpellConditionSavingThrowOutcomeStage"
        }
        SaveGatedSpellFrontierStage::DamageDice => "SaveGatedSpellDamageDiceStage",
        SaveGatedSpellFrontierStage::Resolved => "SaveGatedSpellResolvedStage",
    }
}

fn protocol_result_ref(protocol: &SaveGatedSpellOrderingProtocol) -> &'static str {
    match protocol {
        SaveGatedSpellOrderingProtocol::Init => "init",
        SaveGatedSpellOrderingProtocol::NeedsHoles(_) => "needsHoles",
        SaveGatedSpellOrderingProtocol::Resolved => "resolved",
    }
}

fn protocol_holes(protocol: &SaveGatedSpellOrderingProtocol) -> Vec<&'static str> {
    match protocol {
        SaveGatedSpellOrderingProtocol::NeedsHoles(holes) => holes
            .iter()
            .map(|hole| match hole {
                SaveGatedSpellHoleKind::SpellTargetList => "SpellTargetListHoleKind",
                SaveGatedSpellHoleKind::ConditionChoice => "ConditionChoiceHoleKind",
                SaveGatedSpellHoleKind::SavingThrowOutcome => "SavingThrowOutcomeHoleKind",
                SaveGatedSpellHoleKind::RolledDice => "RolledDiceHoleKind",
            })
            .collect(),
        SaveGatedSpellOrderingProtocol::Init | SaveGatedSpellOrderingProtocol::Resolved => {
            Vec::new()
        }
    }
}

fn ordering_error_ref(error: Option<SaveGatedSpellFillOrderingError>) -> &'static str {
    match error {
        Some(SaveGatedSpellFillOrderingError::TargetOrAreaRequired) => "targetOrAreaRequired",
        Some(SaveGatedSpellFillOrderingError::SavingThrowRequired) => "savingThrowRequired",
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
