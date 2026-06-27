#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SpeciesPassiveScenarioResult {
    Init,
    DragonbornDamageResistance,
    DwarvenResilience,
    HalflingBrave,
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
    pub halfling_frightened_avoid_save_advantage: bool,
    pub halfling_frightened_end_save_advantage: bool,
    pub halfling_poisoned_save_advantage: bool,
    pub goliath_escape_roll_mode: SpeciesPassiveRollMode,
    pub goliath_poisoned_escape_roll_mode: SpeciesPassiveRollMode,
    pub scenario_result: SpeciesPassiveScenarioResult,
    pub protocol: SpeciesPassiveProtocol,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PassiveDamageType {
    Fire,
    Cold,
    Poison,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PassiveDamageResistance {
    pub damage_type: PassiveDamageType,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PassiveSaveCondition {
    Charmed,
    Frightened,
    Poisoned,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PassiveSavePurpose {
    AvoidCondition,
    EndCondition,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PassiveSaveAdvantage {
    pub condition: PassiveSaveCondition,
    pub purpose: PassiveSavePurpose,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PassiveAbilityCheckKind {
    EscapeGrapple,
    AvoidPoisoned,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PassiveAbilityCheckRollMode {
    pub check: PassiveAbilityCheckKind,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CreatureSpaceTraversalPermission {
    LargerCreatureSpaceNoStop,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CreatureSpaceTraversalFacts {
    pub permission: Option<CreatureSpaceTraversalPermission>,
    pub mover_size_rank: i8,
    pub occupied_creature_size_rank: i8,
    pub stops_in_occupied_space: bool,
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
        halfling_frightened_avoid_save_advantage: false,
        halfling_frightened_end_save_advantage: false,
        halfling_poisoned_save_advantage: false,
        goliath_escape_roll_mode: SpeciesPassiveRollMode::Normal,
        goliath_poisoned_escape_roll_mode: SpeciesPassiveRollMode::Normal,
        scenario_result: SpeciesPassiveScenarioResult::Init,
        protocol: SpeciesPassiveProtocol::Init,
    }
}

#[must_use]
pub fn apply_passive_damage_resistance(
    resistance: PassiveDamageResistance,
    incoming_damage_type: PassiveDamageType,
    damage_before: i16,
) -> i16 {
    // RAW: cleanroom-input/raw/srd-5.2.1/Playing-the-Game.md
    // "Resistance and Vulnerability"; QNT:
    // battle-runtime-species-passive-trait-selected-identity.mbt.qnt.
    if resistance.damage_type == incoming_damage_type {
        damage_before / 2
    } else {
        damage_before
    }
}

#[must_use]
pub fn passive_save_has_advantage(
    profile: PassiveSaveAdvantage,
    condition: PassiveSaveCondition,
    purpose: PassiveSavePurpose,
) -> bool {
    profile.condition == condition
        && matches!(
            (profile.purpose, purpose),
            (
                PassiveSavePurpose::AvoidCondition,
                PassiveSavePurpose::AvoidCondition
            ) | (
                PassiveSavePurpose::EndCondition,
                PassiveSavePurpose::EndCondition
            )
        )
}

#[must_use]
pub fn passive_ability_check_roll_mode(
    profile: PassiveAbilityCheckRollMode,
    check: PassiveAbilityCheckKind,
) -> SpeciesPassiveRollMode {
    if matches!(
        (profile.check, check),
        (
            PassiveAbilityCheckKind::EscapeGrapple,
            PassiveAbilityCheckKind::EscapeGrapple
        )
    ) {
        SpeciesPassiveRollMode::Advantage
    } else {
        SpeciesPassiveRollMode::Normal
    }
}

#[must_use]
pub fn creature_space_traversal_allowed(facts: CreatureSpaceTraversalFacts) -> bool {
    matches!(
        facts.permission,
        Some(CreatureSpaceTraversalPermission::LargerCreatureSpaceNoStop)
    ) && facts.occupied_creature_size_rank > facts.mover_size_rank
        && !facts.stops_in_occupied_space
}

#[must_use]
pub fn project_dragonborn_damage_resistance() -> SpeciesPassiveTraitState {
    // RAW: cleanroom-input/raw/srd-5.2.1/Character-Origins.md
    // "Dragonborn", "Damage Resistance"; RAW:
    // cleanroom-input/raw/srd-5.2.1/Playing-the-Game.md
    // "Resistance and Vulnerability"; QNT:
    // battle-runtime-species-passive-trait-selected-identity.mbt.qnt.
    SpeciesPassiveTraitState {
        dragonborn_fire_damage_after: apply_passive_damage_resistance(
            PassiveDamageResistance {
                damage_type: PassiveDamageType::Fire,
            },
            PassiveDamageType::Fire,
            9,
        ),
        dragonborn_cold_damage_after: apply_passive_damage_resistance(
            PassiveDamageResistance {
                damage_type: PassiveDamageType::Fire,
            },
            PassiveDamageType::Cold,
            9,
        ),
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
    let poison_save_profile = PassiveSaveAdvantage {
        condition: PassiveSaveCondition::Poisoned,
        purpose: PassiveSavePurpose::AvoidCondition,
    };
    SpeciesPassiveTraitState {
        dwarf_poison_damage_after: apply_passive_damage_resistance(
            PassiveDamageResistance {
                damage_type: PassiveDamageType::Poison,
            },
            PassiveDamageType::Poison,
            9,
        ),
        dwarf_fire_damage_after: apply_passive_damage_resistance(
            PassiveDamageResistance {
                damage_type: PassiveDamageType::Poison,
            },
            PassiveDamageType::Fire,
            9,
        ),
        dwarf_poisoned_save_advantage: passive_save_has_advantage(
            poison_save_profile,
            PassiveSaveCondition::Poisoned,
            PassiveSavePurpose::AvoidCondition,
        ),
        dwarf_charmed_save_advantage: passive_save_has_advantage(
            poison_save_profile,
            PassiveSaveCondition::Charmed,
            PassiveSavePurpose::AvoidCondition,
        ),
        scenario_result: SpeciesPassiveScenarioResult::DwarvenResilience,
        protocol: SpeciesPassiveProtocol::Resolved,
        ..species_passive_traits_initial_state()
    }
}

#[must_use]
pub fn project_halfling_brave() -> SpeciesPassiveTraitState {
    // RAW: cleanroom-input/raw/srd-5.2.1/Character-Origins.md
    // "Halfling", "Brave"; QNT:
    // battle-runtime-species-passive-trait-selected-identity.mbt.qnt.
    let avoid_profile = PassiveSaveAdvantage {
        condition: PassiveSaveCondition::Frightened,
        purpose: PassiveSavePurpose::AvoidCondition,
    };
    let end_profile = PassiveSaveAdvantage {
        condition: PassiveSaveCondition::Frightened,
        purpose: PassiveSavePurpose::EndCondition,
    };
    SpeciesPassiveTraitState {
        halfling_frightened_avoid_save_advantage: passive_save_has_advantage(
            avoid_profile,
            PassiveSaveCondition::Frightened,
            PassiveSavePurpose::AvoidCondition,
        ),
        halfling_frightened_end_save_advantage: passive_save_has_advantage(
            end_profile,
            PassiveSaveCondition::Frightened,
            PassiveSavePurpose::EndCondition,
        ),
        halfling_poisoned_save_advantage: passive_save_has_advantage(
            avoid_profile,
            PassiveSaveCondition::Poisoned,
            PassiveSavePurpose::AvoidCondition,
        ),
        scenario_result: SpeciesPassiveScenarioResult::HalflingBrave,
        protocol: SpeciesPassiveProtocol::Resolved,
        ..species_passive_traits_initial_state()
    }
}

#[must_use]
pub fn project_goliath_powerful_build() -> SpeciesPassiveTraitState {
    // RAW: cleanroom-input/raw/srd-5.2.1/Character-Origins.md
    // "Goliath", "Powerful Build"; QNT:
    // battle-runtime-species-passive-trait-selected-identity.mbt.qnt.
    let profile = PassiveAbilityCheckRollMode {
        check: PassiveAbilityCheckKind::EscapeGrapple,
    };
    SpeciesPassiveTraitState {
        goliath_escape_roll_mode: passive_ability_check_roll_mode(
            profile,
            PassiveAbilityCheckKind::EscapeGrapple,
        ),
        goliath_poisoned_escape_roll_mode: passive_ability_check_roll_mode(
            profile,
            PassiveAbilityCheckKind::AvoidPoisoned,
        ),
        scenario_result: SpeciesPassiveScenarioResult::GoliathPowerfulBuild,
        protocol: SpeciesPassiveProtocol::Resolved,
        ..species_passive_traits_initial_state()
    }
}
