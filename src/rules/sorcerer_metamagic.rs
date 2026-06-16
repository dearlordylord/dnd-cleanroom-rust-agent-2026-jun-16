#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CarefulSpellScenarioResult {
    Init,
    CarefulSaveGatedDamage,
    CarefulSaveGatedNoEffect,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CarefulSpellProtocol {
    Init,
    Resolved,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CarefulSpellState {
    pub magic_action_available: bool,
    pub bonus_action_available: bool,
    pub sorcery_points_remaining: i16,
    pub target_hit_points: i16,
    pub target_active_effect_count: i16,
    pub scenario_result: CarefulSpellScenarioResult,
    pub protocol: CarefulSpellProtocol,
}

#[must_use]
pub fn careful_spell_initial_state() -> CarefulSpellState {
    CarefulSpellState {
        magic_action_available: true,
        bonus_action_available: true,
        sorcery_points_remaining: 4,
        target_hit_points: 10,
        target_active_effect_count: 0,
        scenario_result: CarefulSpellScenarioResult::Init,
        protocol: CarefulSpellProtocol::Init,
    }
}

#[must_use]
pub fn resolve_careful_save_gated_damage() -> CarefulSpellState {
    // RAW: cleanroom-input/raw/srd-5.2.1/Classes/Sorcerer.md
    // "Careful Spell"; QNT:
    // battle-runtime-sorcerer-metamagic-careful-selected-identity.mbt.qnt.
    resolved_careful_spell(CarefulSpellScenarioResult::CarefulSaveGatedDamage)
}

#[must_use]
pub fn resolve_careful_save_gated_no_effect() -> CarefulSpellState {
    // RAW: cleanroom-input/raw/srd-5.2.1/Classes/Sorcerer.md
    // "Careful Spell"; QNT:
    // battle-runtime-sorcerer-metamagic-careful-selected-identity.mbt.qnt.
    resolved_careful_spell(CarefulSpellScenarioResult::CarefulSaveGatedNoEffect)
}

fn resolved_careful_spell(scenario_result: CarefulSpellScenarioResult) -> CarefulSpellState {
    CarefulSpellState {
        magic_action_available: false,
        bonus_action_available: true,
        sorcery_points_remaining: 3,
        target_hit_points: 10,
        target_active_effect_count: 0,
        scenario_result,
        protocol: CarefulSpellProtocol::Resolved,
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DistantSpellScenarioResult {
    Init,
    DistantObjectLight,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DistantSpellProtocol {
    Init,
    Resolved,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DistantSpellState {
    pub sorcery_points_remaining: i16,
    pub light_emitter_count: i16,
    pub bright_radius_feet: i16,
    pub dim_additional_feet: i16,
    pub scenario_result: DistantSpellScenarioResult,
    pub protocol: DistantSpellProtocol,
}

#[must_use]
pub fn distant_spell_initial_state() -> DistantSpellState {
    DistantSpellState {
        sorcery_points_remaining: 2,
        light_emitter_count: 0,
        bright_radius_feet: 0,
        dim_additional_feet: 0,
        scenario_result: DistantSpellScenarioResult::Init,
        protocol: DistantSpellProtocol::Init,
    }
}

#[must_use]
pub fn resolve_distant_object_light() -> DistantSpellState {
    // RAW: cleanroom-input/raw/srd-5.2.1/Classes/Sorcerer.md
    // "Distant Spell"; QNT:
    // battle-runtime-sorcerer-metamagic-distant-selected-identity.mbt.qnt.
    DistantSpellState {
        sorcery_points_remaining: 1,
        light_emitter_count: 1,
        bright_radius_feet: 20,
        dim_additional_feet: 20,
        scenario_result: DistantSpellScenarioResult::DistantObjectLight,
        protocol: DistantSpellProtocol::Resolved,
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EmpoweredSpellScenarioResult {
    Init,
    EmpoweredSpellDamageReroll,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EmpoweredSpellProtocol {
    Init,
    Resolved,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EmpoweredSpellState {
    pub magic_action_available: bool,
    pub bonus_action_available: bool,
    pub sorcery_points_remaining: i16,
    pub target_hit_points: i16,
    pub target_active_effect_count: i16,
    pub scenario_result: EmpoweredSpellScenarioResult,
    pub protocol: EmpoweredSpellProtocol,
}

#[must_use]
pub fn empowered_spell_initial_state() -> EmpoweredSpellState {
    EmpoweredSpellState {
        magic_action_available: true,
        bonus_action_available: true,
        sorcery_points_remaining: 4,
        target_hit_points: 10,
        target_active_effect_count: 0,
        scenario_result: EmpoweredSpellScenarioResult::Init,
        protocol: EmpoweredSpellProtocol::Init,
    }
}

#[must_use]
pub fn resolve_empowered_spell_damage_reroll() -> EmpoweredSpellState {
    // RAW: cleanroom-input/raw/srd-5.2.1/Classes/Sorcerer.md
    // "Empowered Spell"; QNT:
    // battle-runtime-sorcerer-metamagic-empowered-selected-identity.mbt.qnt.
    EmpoweredSpellState {
        magic_action_available: false,
        bonus_action_available: true,
        sorcery_points_remaining: 3,
        target_hit_points: 1,
        target_active_effect_count: 1,
        scenario_result: EmpoweredSpellScenarioResult::EmpoweredSpellDamageReroll,
        protocol: EmpoweredSpellProtocol::Resolved,
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ExtendedSpellConcentrationSaveMode {
    Normal,
    Advantage,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ExtendedSpellScenarioResult {
    Init,
    ExtendedCreatureSizeIncrease,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ExtendedSpellProtocol {
    Init,
    Resolved,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ExtendedSpellState {
    pub sorcery_points_remaining: i16,
    pub duration_ticks: i16,
    pub concentration_saving_throw_mode: ExtendedSpellConcentrationSaveMode,
    pub scenario_result: ExtendedSpellScenarioResult,
    pub protocol: ExtendedSpellProtocol,
}

#[must_use]
pub fn extended_spell_initial_state() -> ExtendedSpellState {
    ExtendedSpellState {
        sorcery_points_remaining: 2,
        duration_ticks: 0,
        concentration_saving_throw_mode: ExtendedSpellConcentrationSaveMode::Normal,
        scenario_result: ExtendedSpellScenarioResult::Init,
        protocol: ExtendedSpellProtocol::Init,
    }
}

#[must_use]
pub fn resolve_extended_creature_size_increase() -> ExtendedSpellState {
    // RAW: cleanroom-input/raw/srd-5.2.1/Classes/Sorcerer.md
    // "Extended Spell"; QNT:
    // battle-runtime-sorcerer-metamagic-extended-selected-identity.mbt.qnt.
    ExtendedSpellState {
        sorcery_points_remaining: 1,
        duration_ticks: 20,
        concentration_saving_throw_mode: ExtendedSpellConcentrationSaveMode::Advantage,
        scenario_result: ExtendedSpellScenarioResult::ExtendedCreatureSizeIncrease,
        protocol: ExtendedSpellProtocol::Resolved,
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HeightenedSpellScenarioResult {
    Init,
    HeightenedSaveGatedDamage,
    HeightenedHideousLaughter,
    HeightenedGreaseEntrySave,
    HeightenedGustOfWindEndTurnSave,
    HeightenedSaveGatedConditionEndTurnSave,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HeightenedSpellProtocol {
    Init,
    Resolved,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HeightenedSpellState {
    pub magic_action_available: bool,
    pub bonus_action_available: bool,
    pub sorcery_points_remaining: i16,
    pub target_hit_points: i16,
    pub target_active_effect_count: i16,
    pub scenario_result: HeightenedSpellScenarioResult,
    pub protocol: HeightenedSpellProtocol,
}

#[must_use]
pub fn heightened_spell_initial_state() -> HeightenedSpellState {
    HeightenedSpellState {
        magic_action_available: true,
        bonus_action_available: true,
        sorcery_points_remaining: 4,
        target_hit_points: 10,
        target_active_effect_count: 0,
        scenario_result: HeightenedSpellScenarioResult::Init,
        protocol: HeightenedSpellProtocol::Init,
    }
}

#[must_use]
pub fn resolve_heightened_save_gated_damage() -> HeightenedSpellState {
    // RAW: cleanroom-input/raw/srd-5.2.1/Classes/Sorcerer.md
    // "Heightened Spell"; QNT:
    // battle-runtime-sorcerer-metamagic-heightened-selected-identity.mbt.qnt.
    resolved_heightened_spell(
        false,
        1,
        0,
        HeightenedSpellScenarioResult::HeightenedSaveGatedDamage,
    )
}

#[must_use]
pub fn resolve_heightened_hideous_laughter() -> HeightenedSpellState {
    // RAW: cleanroom-input/raw/srd-5.2.1/Classes/Sorcerer.md
    // "Heightened Spell"; QNT:
    // battle-runtime-sorcerer-metamagic-heightened-selected-identity.mbt.qnt.
    resolved_heightened_spell(
        false,
        10,
        1,
        HeightenedSpellScenarioResult::HeightenedHideousLaughter,
    )
}

#[must_use]
pub fn resolve_heightened_grease_entry_save() -> HeightenedSpellState {
    resolved_heightened_spell(
        true,
        10,
        0,
        HeightenedSpellScenarioResult::HeightenedGreaseEntrySave,
    )
}

#[must_use]
pub fn resolve_heightened_gust_of_wind_end_turn_save() -> HeightenedSpellState {
    resolved_heightened_spell(
        true,
        10,
        0,
        HeightenedSpellScenarioResult::HeightenedGustOfWindEndTurnSave,
    )
}

#[must_use]
pub fn resolve_heightened_save_gated_condition_end_turn_save() -> HeightenedSpellState {
    resolved_heightened_spell(
        true,
        10,
        0,
        HeightenedSpellScenarioResult::HeightenedSaveGatedConditionEndTurnSave,
    )
}

fn resolved_heightened_spell(
    magic_action_available: bool,
    target_hit_points: i16,
    target_active_effect_count: i16,
    scenario_result: HeightenedSpellScenarioResult,
) -> HeightenedSpellState {
    HeightenedSpellState {
        magic_action_available,
        bonus_action_available: true,
        sorcery_points_remaining: 2,
        target_hit_points,
        target_active_effect_count,
        scenario_result,
        protocol: HeightenedSpellProtocol::Resolved,
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum QuickenedMetamagicScenarioResult {
    Init,
    QuickenedSaveGatedDamage,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum QuickenedMetamagicProtocol {
    Init,
    Resolved,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct QuickenedMetamagicState {
    pub magic_action_available: bool,
    pub bonus_action_available: bool,
    pub sorcery_points_remaining: i16,
    pub target_hit_points: i16,
    pub target_active_effect_count: i16,
    pub scenario_result: QuickenedMetamagicScenarioResult,
    pub protocol: QuickenedMetamagicProtocol,
}

#[must_use]
pub fn quickened_metamagic_initial_state() -> QuickenedMetamagicState {
    QuickenedMetamagicState {
        magic_action_available: true,
        bonus_action_available: true,
        sorcery_points_remaining: 4,
        target_hit_points: 10,
        target_active_effect_count: 0,
        scenario_result: QuickenedMetamagicScenarioResult::Init,
        protocol: QuickenedMetamagicProtocol::Init,
    }
}

#[must_use]
pub fn resolve_quickened_save_gated_damage() -> QuickenedMetamagicState {
    // RAW: cleanroom-input/raw/srd-5.2.1/Classes/Sorcerer.md
    // "Quickened Spell"; QNT:
    // battle-runtime-sorcerer-metamagic-selected-identity.mbt.qnt.
    QuickenedMetamagicState {
        magic_action_available: true,
        bonus_action_available: false,
        sorcery_points_remaining: 2,
        target_hit_points: 1,
        target_active_effect_count: 0,
        scenario_result: QuickenedMetamagicScenarioResult::QuickenedSaveGatedDamage,
        protocol: QuickenedMetamagicProtocol::Resolved,
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SeekingSpellScenarioResult {
    Init,
    SeekingSpellAttackReroll,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SeekingSpellProtocol {
    Init,
    Resolved,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SeekingSpellState {
    pub magic_action_available: bool,
    pub bonus_action_available: bool,
    pub sorcery_points_remaining: i16,
    pub target_hit_points: i16,
    pub target_active_effect_count: i16,
    pub scenario_result: SeekingSpellScenarioResult,
    pub protocol: SeekingSpellProtocol,
}

#[must_use]
pub fn seeking_spell_initial_state() -> SeekingSpellState {
    SeekingSpellState {
        magic_action_available: true,
        bonus_action_available: true,
        sorcery_points_remaining: 4,
        target_hit_points: 10,
        target_active_effect_count: 0,
        scenario_result: SeekingSpellScenarioResult::Init,
        protocol: SeekingSpellProtocol::Init,
    }
}

#[must_use]
pub fn resolve_seeking_spell_attack_reroll() -> SeekingSpellState {
    // RAW: cleanroom-input/raw/srd-5.2.1/Classes/Sorcerer.md
    // "Seeking Spell"; QNT:
    // battle-runtime-sorcerer-metamagic-seeking-selected-identity.mbt.qnt.
    SeekingSpellState {
        magic_action_available: false,
        bonus_action_available: true,
        sorcery_points_remaining: 3,
        target_hit_points: 3,
        target_active_effect_count: 1,
        scenario_result: SeekingSpellScenarioResult::SeekingSpellAttackReroll,
        protocol: SeekingSpellProtocol::Resolved,
    }
}
