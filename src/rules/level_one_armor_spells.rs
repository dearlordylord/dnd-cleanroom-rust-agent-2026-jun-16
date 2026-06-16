use crate::rules::armor_class::{
    armor_class_projection, ArmorClassAbility, ArmorClassFacts, ArmorClassFormula, ArmorClassOption,
};

pub const DEFAULT_UNARMORED_BASE_ARMOR_CLASS: i8 = 10;
pub const MAGE_ARMOR_BASE_ARMOR_CLASS: i8 = 13;
pub const MAGE_ARMOR_DEXTERITY_MODIFIER: i8 = 2;
pub const ELAPSED_TIME_TICKS_PER_HOUR: u16 = 600;
pub const MAGE_ARMOR_DURATION_TICKS: u16 = 8 * ELAPSED_TIME_TICKS_PER_HOUR;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MageArmorScenarioOutcome {
    Init,
    Discovered,
    ArmoredRejected,
    Resolved,
    DurationExpired,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MageArmorProtocol {
    Init,
    Resolved,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MageArmorState {
    pub self_target_admitted: bool,
    pub armored_target_rejected: bool,
    pub mage_armor_effect_present: bool,
    pub stored_armor_base_still_unarmored: bool,
    pub projected_base_is_mage_armor: bool,
    pub armor_class: i8,
    pub mage_armor_duration_ticks: u16,
    pub level_one_slots_expended: u8,
    pub action_available: bool,
    pub scenario_outcome: MageArmorScenarioOutcome,
    pub protocol: MageArmorProtocol,
}

#[must_use]
pub fn mage_armor_initial_state() -> MageArmorState {
    MageArmorState {
        self_target_admitted: true,
        armored_target_rejected: false,
        mage_armor_effect_present: false,
        stored_armor_base_still_unarmored: true,
        projected_base_is_mage_armor: false,
        armor_class: unarmored_armor_class(),
        mage_armor_duration_ticks: 0,
        level_one_slots_expended: 0,
        action_available: true,
        scenario_outcome: MageArmorScenarioOutcome::Init,
        protocol: MageArmorProtocol::Init,
    }
}

#[must_use]
pub fn discover_mage_armor_unarmored_self_target() -> MageArmorState {
    // RAW: cleanroom-input/raw/srd-5.2.1/Spells/Descriptions-M-P.md
    // "Mage Armor"; QNT: battle-runtime-mage-armor-selected-identity.mbt.qnt.
    MageArmorState {
        scenario_outcome: MageArmorScenarioOutcome::Discovered,
        protocol: MageArmorProtocol::Resolved,
        ..mage_armor_initial_state()
    }
}

#[must_use]
pub fn reject_mage_armor_armored_target() -> MageArmorState {
    // RAW: cleanroom-input/raw/srd-5.2.1/Spells/Descriptions-M-P.md
    // "Mage Armor"; QNT: battle-runtime-armor-spell-resolution.qnt.
    MageArmorState {
        armored_target_rejected: true,
        scenario_outcome: MageArmorScenarioOutcome::ArmoredRejected,
        protocol: MageArmorProtocol::Resolved,
        ..mage_armor_initial_state()
    }
}

#[must_use]
pub fn resolve_mage_armor_base_armor_class() -> MageArmorState {
    // RAW: cleanroom-input/raw/srd-5.2.1/Playing-the-Game.md "Armor Class"
    // and Spells/Descriptions-M-P.md "Mage Armor"; QNT:
    // battle-runtime-armor-class.qnt.
    MageArmorState {
        mage_armor_effect_present: true,
        projected_base_is_mage_armor: true,
        armor_class: mage_armor_armor_class(),
        mage_armor_duration_ticks: MAGE_ARMOR_DURATION_TICKS,
        level_one_slots_expended: 1,
        action_available: false,
        scenario_outcome: MageArmorScenarioOutcome::Resolved,
        protocol: MageArmorProtocol::Resolved,
        ..mage_armor_initial_state()
    }
}

#[must_use]
pub fn expire_mage_armor_duration() -> MageArmorState {
    // RAW: cleanroom-input/raw/srd-5.2.1/Spells/Descriptions-M-P.md
    // "Mage Armor"; QNT: battle-runtime-armor-class.qnt.
    MageArmorState {
        level_one_slots_expended: 1,
        action_available: false,
        scenario_outcome: MageArmorScenarioOutcome::DurationExpired,
        protocol: MageArmorProtocol::Resolved,
        ..mage_armor_initial_state()
    }
}

#[must_use]
pub fn unarmored_armor_class() -> i8 {
    armor_class_projection(
        ArmorClassOption::DefaultUnarmored,
        armor_class_facts(DEFAULT_UNARMORED_BASE_ARMOR_CLASS),
    )
    .armor_class
}

#[must_use]
pub fn mage_armor_armor_class() -> i8 {
    armor_class_projection(
        ArmorClassOption::DefaultUnarmored,
        armor_class_facts(MAGE_ARMOR_BASE_ARMOR_CLASS),
    )
    .armor_class
}

fn armor_class_facts(base_armor_class: i8) -> ArmorClassFacts {
    ArmorClassFacts {
        dexterity_modifier: MAGE_ARMOR_DEXTERITY_MODIFIER,
        constitution_modifier: 0,
        wisdom_modifier: 0,
        charisma_modifier: 0,
        formula: ArmorClassFormula::AbilitySum {
            base: base_armor_class,
            abilities: vec![ArmorClassAbility::Dexterity],
        },
        shield_bonus: None,
    }
}
