use crate::rules::ability_checks::{
    ability_check_proficiency_bonus, AbilityCheckProficiencyBonusKind,
    AbilityCheckProficiencyBonusProjection, AbilityCheckSkillTraining, JackOfAllTradesState,
    OtherProficiencyBonusState,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AbilityCheckProficiencyWitness {
    pub last_result: &'static str,
    pub projection_tag: &'static str,
    pub source_unit_id: &'static str,
    pub skill: &'static str,
    pub bonus: u8,
    pub replay_index: u8,
}

pub fn replay_observed_action(observed_action_taken: &str) -> AbilityCheckProficiencyWitness {
    match observed_action_taken {
        "doProjectJackOfAllTradesLevelTwo" => jack_of_all_trades_level_two(),
        "doProjectJackOfAllTradesRoundedDown" => jack_of_all_trades_rounded_down(),
        "doProjectSkillProficiency" => skill_proficiency(),
        "doProjectExpertise" => expertise(),
        "doRejectOtherProficiencyBonus" => other_proficiency_bonus_applies(),
        "doRejectMissingBardLevelTwo" => missing_bard_level_two(),
        action => panic!("unsupported mbt::actionTaken {action}"),
    }
}

pub fn expected_jack_of_all_trades_level_two_witness() -> AbilityCheckProficiencyWitness {
    AbilityCheckProficiencyWitness {
        last_result: "jack-of-all-trades-level-two",
        projection_tag: "jackOfAllTrades",
        source_unit_id: "bard_jack_of_all_trades",
        skill: "performance",
        bonus: 1,
        replay_index: 1,
    }
}

pub fn expected_jack_of_all_trades_rounded_down_witness() -> AbilityCheckProficiencyWitness {
    AbilityCheckProficiencyWitness {
        last_result: "jack-of-all-trades-rounded-down",
        projection_tag: "jackOfAllTrades",
        source_unit_id: "bard_jack_of_all_trades",
        skill: "performance",
        bonus: 1,
        replay_index: 2,
    }
}

pub fn expected_skill_proficiency_witness() -> AbilityCheckProficiencyWitness {
    AbilityCheckProficiencyWitness {
        last_result: "skill-proficiency",
        projection_tag: "skillProficiency",
        source_unit_id: "none",
        skill: "performance",
        bonus: 3,
        replay_index: 3,
    }
}

pub fn expected_expertise_witness() -> AbilityCheckProficiencyWitness {
    AbilityCheckProficiencyWitness {
        last_result: "expertise",
        projection_tag: "expertise",
        source_unit_id: "none",
        skill: "performance",
        bonus: 6,
        replay_index: 4,
    }
}

pub fn expected_other_proficiency_bonus_witness() -> AbilityCheckProficiencyWitness {
    AbilityCheckProficiencyWitness {
        last_result: "other-proficiency-bonus-applies",
        projection_tag: "none",
        source_unit_id: "none",
        skill: "performance",
        bonus: 0,
        replay_index: 5,
    }
}

pub fn expected_missing_bard_level_two_witness() -> AbilityCheckProficiencyWitness {
    AbilityCheckProficiencyWitness {
        last_result: "missing-bard-level-two",
        projection_tag: "none",
        source_unit_id: "none",
        skill: "performance",
        bonus: 0,
        replay_index: 6,
    }
}

pub fn projection_payload(witness: &AbilityCheckProficiencyWitness) -> String {
    [
        format!("lastResult={}", witness.last_result),
        format!("projectionTag={}", witness.projection_tag),
        format!("sourceUnitId={}", witness.source_unit_id),
        format!("skill={}", witness.skill),
        format!("bonus={}", witness.bonus),
        format!("replayIndex={}", witness.replay_index),
    ]
    .join("\n")
}

fn jack_of_all_trades_level_two() -> AbilityCheckProficiencyWitness {
    let projection = ability_check_proficiency_bonus(
        2,
        AbilityCheckSkillTraining::Unproficient,
        JackOfAllTradesState::Present,
        OtherProficiencyBonusState::NoOtherProficiencyBonus,
    );
    ability_check_witness(
        "jack-of-all-trades-level-two",
        "bard_jack_of_all_trades",
        1,
        projection,
    )
}

fn jack_of_all_trades_rounded_down() -> AbilityCheckProficiencyWitness {
    let projection = ability_check_proficiency_bonus(
        3,
        AbilityCheckSkillTraining::Unproficient,
        JackOfAllTradesState::Present,
        OtherProficiencyBonusState::NoOtherProficiencyBonus,
    );
    ability_check_witness(
        "jack-of-all-trades-rounded-down",
        "bard_jack_of_all_trades",
        2,
        projection,
    )
}

fn skill_proficiency() -> AbilityCheckProficiencyWitness {
    let projection = ability_check_proficiency_bonus(
        3,
        AbilityCheckSkillTraining::Proficient,
        JackOfAllTradesState::Present,
        OtherProficiencyBonusState::NoOtherProficiencyBonus,
    );
    ability_check_witness("skill-proficiency", "none", 3, projection)
}

fn expertise() -> AbilityCheckProficiencyWitness {
    let projection = ability_check_proficiency_bonus(
        3,
        AbilityCheckSkillTraining::Expertise,
        JackOfAllTradesState::Present,
        OtherProficiencyBonusState::NoOtherProficiencyBonus,
    );
    ability_check_witness("expertise", "none", 4, projection)
}

fn other_proficiency_bonus_applies() -> AbilityCheckProficiencyWitness {
    let projection = ability_check_proficiency_bonus(
        3,
        AbilityCheckSkillTraining::Unproficient,
        JackOfAllTradesState::Present,
        OtherProficiencyBonusState::OtherProficiencyBonusApplies,
    );
    ability_check_witness("other-proficiency-bonus-applies", "none", 5, projection)
}

fn missing_bard_level_two() -> AbilityCheckProficiencyWitness {
    let projection = ability_check_proficiency_bonus(
        2,
        AbilityCheckSkillTraining::Unproficient,
        JackOfAllTradesState::Absent,
        OtherProficiencyBonusState::NoOtherProficiencyBonus,
    );
    ability_check_witness("missing-bard-level-two", "none", 6, projection)
}

fn ability_check_witness(
    last_result: &'static str,
    source_unit_id: &'static str,
    replay_index: u8,
    projection: AbilityCheckProficiencyBonusProjection,
) -> AbilityCheckProficiencyWitness {
    AbilityCheckProficiencyWitness {
        last_result,
        projection_tag: projection_tag(projection.kind),
        source_unit_id,
        skill: "performance",
        bonus: projection.bonus,
        replay_index,
    }
}

fn projection_tag(kind: AbilityCheckProficiencyBonusKind) -> &'static str {
    match kind {
        AbilityCheckProficiencyBonusKind::None => "none",
        AbilityCheckProficiencyBonusKind::SkillProficiency => "skillProficiency",
        AbilityCheckProficiencyBonusKind::Expertise => "expertise",
        AbilityCheckProficiencyBonusKind::JackOfAllTrades => "jackOfAllTrades",
    }
}
