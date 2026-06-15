#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AbilityCheckSkillTraining {
    Unproficient,
    Proficient,
    Expertise,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum JackOfAllTradesState {
    Absent,
    Present,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OtherProficiencyBonusState {
    NoOtherProficiencyBonus,
    OtherProficiencyBonusApplies,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AbilityCheckProficiencyBonusKind {
    None,
    SkillProficiency,
    Expertise,
    JackOfAllTrades,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct AbilityCheckProficiencyBonusProjection {
    pub kind: AbilityCheckProficiencyBonusKind,
    pub bonus: u8,
}

#[must_use]
pub fn ability_check_proficiency_bonus(
    proficiency_bonus: u8,
    training: AbilityCheckSkillTraining,
    jack_of_all_trades: JackOfAllTradesState,
    other_proficiency_bonus: OtherProficiencyBonusState,
) -> AbilityCheckProficiencyBonusProjection {
    // cleanroom-input/qnt/shared-algebras/proofs/rule-core/
    // ability-check-proficiency-bonus.qnt; RAW:
    // cleanroom-input/raw/srd-5.2.1/Playing-the-Game.md "Proficiency Bonus",
    // cleanroom-input/raw/srd-5.2.1/Classes/Bard.md "Level 2: Jack of All Trades",
    // and cleanroom-input/raw/srd-5.2.1/Rules-Glossary.md "Expertise".
    match training {
        AbilityCheckSkillTraining::Expertise => AbilityCheckProficiencyBonusProjection {
            kind: AbilityCheckProficiencyBonusKind::Expertise,
            bonus: proficiency_bonus * 2,
        },
        AbilityCheckSkillTraining::Proficient => AbilityCheckProficiencyBonusProjection {
            kind: AbilityCheckProficiencyBonusKind::SkillProficiency,
            bonus: proficiency_bonus,
        },
        AbilityCheckSkillTraining::Unproficient => jack_of_all_trades_bonus(
            proficiency_bonus,
            jack_of_all_trades,
            other_proficiency_bonus,
        ),
    }
}

fn jack_of_all_trades_bonus(
    proficiency_bonus: u8,
    jack_of_all_trades: JackOfAllTradesState,
    other_proficiency_bonus: OtherProficiencyBonusState,
) -> AbilityCheckProficiencyBonusProjection {
    match (jack_of_all_trades, other_proficiency_bonus) {
        (JackOfAllTradesState::Present, OtherProficiencyBonusState::NoOtherProficiencyBonus) => {
            AbilityCheckProficiencyBonusProjection {
                kind: AbilityCheckProficiencyBonusKind::JackOfAllTrades,
                bonus: proficiency_bonus / 2,
            }
        }
        _ => AbilityCheckProficiencyBonusProjection {
            kind: AbilityCheckProficiencyBonusKind::None,
            bonus: 0,
        },
    }
}
