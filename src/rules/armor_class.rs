#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ArmorClassAbility {
    Dexterity,
    Constitution,
    Wisdom,
    Charisma,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ArmorClassFormula {
    AbilitySum {
        base: i8,
        abilities: Vec<ArmorClassAbility>,
    },
    LightDex {
        base: i8,
    },
    MediumDexMax2 {
        base: i8,
    },
    HeavyFixed {
        armor_class: i8,
    },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ShieldArmorClassBonus {
    pub bonus: i8,
    pub wielding_shield: bool,
    pub shield_training: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ArmorClassFacts {
    pub dexterity_modifier: i8,
    pub constitution_modifier: i8,
    pub wisdom_modifier: i8,
    pub charisma_modifier: i8,
    pub formula: ArmorClassFormula,
    pub shield_bonus: Option<ShieldArmorClassBonus>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ArmorClassOption {
    DefaultUnarmored,
    BarbarianUnarmoredDefense,
    MonkUnarmoredDefense,
    LeatherArmor,
    ChainShirt,
    ChainMail,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ArmorClassProjection {
    pub option: ArmorClassOption,
    pub formula: ArmorClassFormula,
    pub base_armor_class: i8,
    pub uses_dexterity: bool,
    pub uses_constitution: bool,
    pub uses_wisdom: bool,
    pub shield_bonus: i8,
    pub armor_class: i8,
}

#[must_use]
pub fn armor_class_projection(
    option: ArmorClassOption,
    facts: ArmorClassFacts,
) -> ArmorClassProjection {
    // cleanroom-input/qnt/shared-algebras/proofs/rule-core/armor-class-base.qnt;
    // RAW: cleanroom-input/raw/srd-5.2.1/Equipment.md "Armor" and "Shield";
    // cleanroom-input/raw/srd-5.2.1/Classes/Barbarian.md and Classes/Monk.md
    // "Level 1: Unarmored Defense".
    let base_armor_class = base_armor_class_formula_value(&facts.formula);
    let shield_bonus = facts.shield_bonus.map_or(0, shield_armor_class_bonus_value);
    let armor_class = current_armor_class(&facts);
    let formula = facts.formula;

    ArmorClassProjection {
        option,
        uses_dexterity: uses_ability(&formula, ArmorClassAbility::Dexterity),
        uses_constitution: uses_ability(&formula, ArmorClassAbility::Constitution),
        uses_wisdom: uses_ability(&formula, ArmorClassAbility::Wisdom),
        formula,
        base_armor_class,
        shield_bonus,
        armor_class,
    }
}

#[must_use]
pub fn current_armor_class(facts: &ArmorClassFacts) -> i8 {
    let total = base_armor_class_value(facts)
        + facts.shield_bonus.map_or(0, shield_armor_class_bonus_value);
    total.max(1)
}

fn base_armor_class_value(facts: &ArmorClassFacts) -> i8 {
    match &facts.formula {
        ArmorClassFormula::AbilitySum { base, abilities } => {
            *base
                + abilities
                    .iter()
                    .map(|ability| armor_class_ability_modifier(facts, *ability))
                    .sum::<i8>()
        }
        ArmorClassFormula::LightDex { base } => *base + facts.dexterity_modifier,
        ArmorClassFormula::MediumDexMax2 { base } => *base + facts.dexterity_modifier.min(2),
        ArmorClassFormula::HeavyFixed { armor_class } => *armor_class,
    }
}

fn base_armor_class_formula_value(formula: &ArmorClassFormula) -> i8 {
    match formula {
        ArmorClassFormula::AbilitySum { base, .. }
        | ArmorClassFormula::LightDex { base }
        | ArmorClassFormula::MediumDexMax2 { base } => *base,
        ArmorClassFormula::HeavyFixed { armor_class } => *armor_class,
    }
}

fn armor_class_ability_modifier(facts: &ArmorClassFacts, ability: ArmorClassAbility) -> i8 {
    match ability {
        ArmorClassAbility::Dexterity => facts.dexterity_modifier,
        ArmorClassAbility::Constitution => facts.constitution_modifier,
        ArmorClassAbility::Wisdom => facts.wisdom_modifier,
        ArmorClassAbility::Charisma => facts.charisma_modifier,
    }
}

fn shield_armor_class_bonus_value(bonus: ShieldArmorClassBonus) -> i8 {
    if bonus.wielding_shield && bonus.shield_training {
        bonus.bonus
    } else {
        0
    }
}

fn uses_ability(formula: &ArmorClassFormula, ability: ArmorClassAbility) -> bool {
    match formula {
        ArmorClassFormula::AbilitySum { abilities, .. } => abilities.contains(&ability),
        ArmorClassFormula::LightDex { .. } | ArmorClassFormula::MediumDexMax2 { .. } => {
            ability == ArmorClassAbility::Dexterity
        }
        ArmorClassFormula::HeavyFixed { .. } => false,
    }
}
