#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SpeciesPassiveScenarioResult {
    Init,
    DragonbornDamageResistance,
    DwarvenResilience,
    GoliathPowerfulBuild,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SpeciesPassiveProtocol {
    Init,
    Resolved,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SpeciesPassiveRollMode {
    Normal,
    Advantage,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SpeciesPassiveTraitState {
    pub dragonborn_fire_damage_after: i16,
    pub dragonborn_cold_damage_after: i16,
    pub dwarf_poison_damage_after: i16,
    pub dwarf_fire_damage_after: i16,
    pub dwarf_poisoned_save_advantage: bool,
    pub dwarf_charmed_save_advantage: bool,
    pub goliath_escape_roll_mode: SpeciesPassiveRollMode,
    pub goliath_poisoned_escape_roll_mode: SpeciesPassiveRollMode,
    pub scenario_result: SpeciesPassiveScenarioResult,
    pub protocol: SpeciesPassiveProtocol,
}

#[must_use]
pub fn species_passive_traits_initial_state() -> SpeciesPassiveTraitState {
    SpeciesPassiveTraitState {
        dragonborn_fire_damage_after: 9,
        dragonborn_cold_damage_after: 9,
        dwarf_poison_damage_after: 9,
        dwarf_fire_damage_after: 9,
        dwarf_poisoned_save_advantage: false,
        dwarf_charmed_save_advantage: false,
        goliath_escape_roll_mode: SpeciesPassiveRollMode::Normal,
        goliath_poisoned_escape_roll_mode: SpeciesPassiveRollMode::Normal,
        scenario_result: SpeciesPassiveScenarioResult::Init,
        protocol: SpeciesPassiveProtocol::Init,
    }
}

#[must_use]
pub fn project_dragonborn_damage_resistance() -> SpeciesPassiveTraitState {
    // RAW: cleanroom-input/raw/srd-5.2.1/Character-Origins.md
    // "Dragonborn", "Damage Resistance"; RAW:
    // cleanroom-input/raw/srd-5.2.1/Playing-the-Game.md
    // "Resistance and Vulnerability"; QNT:
    // battle-runtime-species-passive-trait-selected-identity.mbt.qnt.
    SpeciesPassiveTraitState {
        dragonborn_fire_damage_after: 4,
        scenario_result: SpeciesPassiveScenarioResult::DragonbornDamageResistance,
        protocol: SpeciesPassiveProtocol::Resolved,
        ..species_passive_traits_initial_state()
    }
}

#[must_use]
pub fn project_dwarven_resilience() -> SpeciesPassiveTraitState {
    // RAW: cleanroom-input/raw/srd-5.2.1/Character-Origins.md
    // "Dwarf", "Dwarven Resilience"; RAW:
    // cleanroom-input/raw/srd-5.2.1/Playing-the-Game.md
    // "Resistance and Vulnerability"; QNT:
    // battle-runtime-species-passive-trait-selected-identity.mbt.qnt.
    SpeciesPassiveTraitState {
        dwarf_poison_damage_after: 4,
        dwarf_poisoned_save_advantage: true,
        scenario_result: SpeciesPassiveScenarioResult::DwarvenResilience,
        protocol: SpeciesPassiveProtocol::Resolved,
        ..species_passive_traits_initial_state()
    }
}

#[must_use]
pub fn project_goliath_powerful_build() -> SpeciesPassiveTraitState {
    // RAW: cleanroom-input/raw/srd-5.2.1/Character-Origins.md
    // "Goliath", "Powerful Build"; QNT:
    // battle-runtime-species-passive-trait-selected-identity.mbt.qnt.
    SpeciesPassiveTraitState {
        goliath_escape_roll_mode: SpeciesPassiveRollMode::Advantage,
        scenario_result: SpeciesPassiveScenarioResult::GoliathPowerfulBuild,
        protocol: SpeciesPassiveProtocol::Resolved,
        ..species_passive_traits_initial_state()
    }
}
