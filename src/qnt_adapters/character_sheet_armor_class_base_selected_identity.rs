use crate::rules::armor_class::{
    armor_class_projection, ArmorClassAbility, ArmorClassFacts, ArmorClassFormula,
    ArmorClassOption, ArmorClassProjection, ShieldArmorClassBonus,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ArmorClassWitness {
    pub source_unit_id: &'static str,
    pub base_source: &'static str,
    pub base_armor_class: i8,
    pub uses_dexterity: bool,
    pub uses_constitution: bool,
    pub uses_wisdom: bool,
    pub shield_bonus: i8,
    pub armor_class: i8,
    pub last_result: &'static str,
}

pub fn replay_observed_action(observed_action_taken: &str) -> ArmorClassWitness {
    match observed_action_taken {
        "doSelectBarbarianUnarmoredDefense" => barbarian_unarmored_defense(false),
        "doSelectBarbarianUnarmoredDefenseWithShield" => barbarian_unarmored_defense(true),
        "doSelectMonkUnarmoredDefense" => monk_unarmored_defense(),
        "doProjectLightArmor" => light_armor(),
        "doProjectMediumArmorDexCap" => medium_armor_dex_cap(),
        "doProjectHeavyArmorWithShield" => heavy_armor_with_shield(),
        action => panic!("unsupported mbt::actionTaken {action}"),
    }
}

pub fn expected_barbarian_unarmored_defense_witness() -> ArmorClassWitness {
    ArmorClassWitness {
        source_unit_id: "barbarian_unarmored_defense",
        base_source: "unarmored_defense",
        base_armor_class: 10,
        uses_dexterity: true,
        uses_constitution: true,
        uses_wisdom: false,
        shield_bonus: 0,
        armor_class: 13,
        last_result: "selected",
    }
}

pub fn expected_barbarian_unarmored_defense_with_shield_witness() -> ArmorClassWitness {
    ArmorClassWitness {
        source_unit_id: "barbarian_unarmored_defense",
        base_source: "unarmored_defense",
        base_armor_class: 10,
        uses_dexterity: true,
        uses_constitution: true,
        uses_wisdom: false,
        shield_bonus: 2,
        armor_class: 15,
        last_result: "selected",
    }
}

pub fn expected_monk_unarmored_defense_witness() -> ArmorClassWitness {
    ArmorClassWitness {
        source_unit_id: "monk_unarmored_defense",
        base_source: "unarmored_defense",
        base_armor_class: 10,
        uses_dexterity: true,
        uses_constitution: false,
        uses_wisdom: true,
        shield_bonus: 0,
        armor_class: 15,
        last_result: "selected",
    }
}

pub fn expected_light_armor_witness() -> ArmorClassWitness {
    ArmorClassWitness {
        source_unit_id: "armor_leather",
        base_source: "armor",
        base_armor_class: 11,
        uses_dexterity: true,
        uses_constitution: false,
        uses_wisdom: false,
        shield_bonus: 0,
        armor_class: 13,
        last_result: "selected",
    }
}

pub fn expected_medium_armor_dex_cap_witness() -> ArmorClassWitness {
    ArmorClassWitness {
        source_unit_id: "armor_chain_shirt",
        base_source: "armor",
        base_armor_class: 13,
        uses_dexterity: true,
        uses_constitution: false,
        uses_wisdom: false,
        shield_bonus: 0,
        armor_class: 15,
        last_result: "selected",
    }
}

pub fn expected_heavy_armor_with_shield_witness() -> ArmorClassWitness {
    ArmorClassWitness {
        source_unit_id: "armor_chain_mail",
        base_source: "armor",
        base_armor_class: 16,
        uses_dexterity: false,
        uses_constitution: false,
        uses_wisdom: false,
        shield_bonus: 2,
        armor_class: 18,
        last_result: "selected",
    }
}

pub fn projection_payload(witness: &ArmorClassWitness) -> String {
    [
        format!("sourceUnitId={}", witness.source_unit_id),
        format!("baseSource={}", witness.base_source),
        format!("baseArmorClass={}", witness.base_armor_class),
        format!("usesDex={}", witness.uses_dexterity),
        format!("usesCon={}", witness.uses_constitution),
        format!("usesWis={}", witness.uses_wisdom),
        format!("shieldBonus={}", witness.shield_bonus),
        format!("armorClass={}", witness.armor_class),
        format!("lastResult={}", witness.last_result),
    ]
    .join("\n")
}

fn barbarian_unarmored_defense(with_shield: bool) -> ArmorClassWitness {
    let projection = armor_class_projection(
        ArmorClassOption::BarbarianUnarmoredDefense,
        facts(
            2,
            ArmorClassFormula::AbilitySum {
                base: 10,
                abilities: vec![
                    ArmorClassAbility::Dexterity,
                    ArmorClassAbility::Constitution,
                ],
            },
            if with_shield {
                Some(shield_bonus())
            } else {
                None
            },
        ),
    );
    armor_class_witness(&projection)
}

fn monk_unarmored_defense() -> ArmorClassWitness {
    let projection = armor_class_projection(
        ArmorClassOption::MonkUnarmoredDefense,
        facts(
            2,
            ArmorClassFormula::AbilitySum {
                base: 10,
                abilities: vec![ArmorClassAbility::Dexterity, ArmorClassAbility::Wisdom],
            },
            None,
        ),
    );
    armor_class_witness(&projection)
}

fn light_armor() -> ArmorClassWitness {
    let projection = armor_class_projection(
        ArmorClassOption::LeatherArmor,
        facts(2, ArmorClassFormula::LightDex { base: 11 }, None),
    );
    armor_class_witness(&projection)
}

fn medium_armor_dex_cap() -> ArmorClassWitness {
    let projection = armor_class_projection(
        ArmorClassOption::ChainShirt,
        facts(3, ArmorClassFormula::MediumDexMax2 { base: 13 }, None),
    );
    armor_class_witness(&projection)
}

fn heavy_armor_with_shield() -> ArmorClassWitness {
    let projection = armor_class_projection(
        ArmorClassOption::ChainMail,
        facts(
            2,
            ArmorClassFormula::HeavyFixed { armor_class: 16 },
            Some(shield_bonus()),
        ),
    );
    armor_class_witness(&projection)
}

fn facts(
    dexterity_modifier: i8,
    formula: ArmorClassFormula,
    shield_bonus: Option<ShieldArmorClassBonus>,
) -> ArmorClassFacts {
    ArmorClassFacts {
        dexterity_modifier,
        constitution_modifier: 1,
        wisdom_modifier: 3,
        charisma_modifier: 0,
        formula,
        shield_bonus,
    }
}

fn shield_bonus() -> ShieldArmorClassBonus {
    ShieldArmorClassBonus {
        bonus: 2,
        wielding_shield: true,
        shield_training: true,
    }
}

fn armor_class_witness(projection: &ArmorClassProjection) -> ArmorClassWitness {
    ArmorClassWitness {
        source_unit_id: source_ref(projection.option),
        base_source: base_source_ref(projection.option),
        base_armor_class: projection.base_armor_class,
        uses_dexterity: projection.uses_dexterity,
        uses_constitution: projection.uses_constitution,
        uses_wisdom: projection.uses_wisdom,
        shield_bonus: projection.shield_bonus,
        armor_class: projection.armor_class,
        last_result: "selected",
    }
}

fn source_ref(source: ArmorClassOption) -> &'static str {
    match source {
        ArmorClassOption::DefaultUnarmored => "none",
        ArmorClassOption::BarbarianUnarmoredDefense => "barbarian_unarmored_defense",
        ArmorClassOption::MonkUnarmoredDefense => "monk_unarmored_defense",
        ArmorClassOption::LeatherArmor => "armor_leather",
        ArmorClassOption::ChainShirt => "armor_chain_shirt",
        ArmorClassOption::ChainMail => "armor_chain_mail",
    }
}

fn base_source_ref(source: ArmorClassOption) -> &'static str {
    match source {
        ArmorClassOption::DefaultUnarmored => "default_unarmored",
        ArmorClassOption::BarbarianUnarmoredDefense | ArmorClassOption::MonkUnarmoredDefense => {
            "unarmored_defense"
        }
        ArmorClassOption::LeatherArmor
        | ArmorClassOption::ChainShirt
        | ArmorClassOption::ChainMail => "armor",
    }
}
