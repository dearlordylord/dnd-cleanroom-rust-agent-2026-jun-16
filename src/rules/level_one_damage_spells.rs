#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LevelOneDamageSpellScenarioOutcome {
    Init,
    BurningHandsMixedConeSavingThrows,
    ChromaticOrbDuplicateDamageLeap,
    IceKnifeHitAttackDamageAndBurstSavingThrows,
    IceKnifeMissBurstSavingThrows,
    PoisonSpraySpellAttackDamage,
    RayOfSicknessSpellAttackDamageAndPoisoned,
    SacredFlameDexteritySavingThrowRadiantDamage,
    SorcerousBurstSpellAttackDamage,
    StarryWispObjectSpellAttackDamageAndDimLight,
    ViciousMockeryWisdomSavingThrowPsychicDamageAndNextAttackDisadvantage,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LevelOneDamageSpellProtocol {
    Init,
    Resolved,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct LevelOneDamageSpellState {
    pub action_available: bool,
    pub spell_slot_spent_this_turn: bool,
    pub level_one_slots_remaining: u8,
    pub primary_target_hit_points: i16,
    pub primary_target_poisoned: bool,
    pub primary_target_next_attack_roll_disadvantage: bool,
    pub secondary_target_hit_points: i16,
    pub scenario_outcome: LevelOneDamageSpellScenarioOutcome,
    pub protocol: LevelOneDamageSpellProtocol,
}

#[must_use]
pub fn level_one_damage_spells_initial_state() -> LevelOneDamageSpellState {
    LevelOneDamageSpellState {
        action_available: true,
        spell_slot_spent_this_turn: false,
        level_one_slots_remaining: 1,
        primary_target_hit_points: 12,
        primary_target_poisoned: false,
        primary_target_next_attack_roll_disadvantage: false,
        secondary_target_hit_points: 12,
        scenario_outcome: LevelOneDamageSpellScenarioOutcome::Init,
        protocol: LevelOneDamageSpellProtocol::Init,
    }
}

#[must_use]
pub fn project_burning_hands_mixed_cone_saving_throws() -> LevelOneDamageSpellState {
    // RAW: cleanroom-input/raw/srd-5.2.1/Spells/Descriptions-A-D.md
    // "Burning Hands"; QNT: battle-runtime-level1-damage-spell-selected-identity.mbt.qnt.
    resolved_state(
        true,
        6,
        false,
        false,
        9,
        LevelOneDamageSpellScenarioOutcome::BurningHandsMixedConeSavingThrows,
    )
}

#[must_use]
pub fn project_chromatic_orb_duplicate_damage_leap() -> LevelOneDamageSpellState {
    // RAW: cleanroom-input/raw/srd-5.2.1/Spells/Descriptions-A-D.md
    // "Chromatic Orb"; QNT: battle-runtime-chained-spell-attack.qnt.
    resolved_state(
        true,
        3,
        false,
        false,
        9,
        LevelOneDamageSpellScenarioOutcome::ChromaticOrbDuplicateDamageLeap,
    )
}

#[must_use]
pub fn project_ice_knife_hit_attack_damage_and_burst_saving_throws() -> LevelOneDamageSpellState {
    // RAW: cleanroom-input/raw/srd-5.2.1/Spells/Descriptions-E-L.md
    // "Ice Knife"; QNT: battle-runtime-save-gated-spell.qnt.
    resolved_state(
        true,
        4,
        false,
        false,
        12,
        LevelOneDamageSpellScenarioOutcome::IceKnifeHitAttackDamageAndBurstSavingThrows,
    )
}

#[must_use]
pub fn project_ice_knife_miss_burst_saving_throws() -> LevelOneDamageSpellState {
    // RAW: cleanroom-input/raw/srd-5.2.1/Spells/Descriptions-E-L.md
    // "Ice Knife"; QNT: battle-runtime-save-gated-spell.qnt.
    resolved_state(
        true,
        8,
        false,
        false,
        12,
        LevelOneDamageSpellScenarioOutcome::IceKnifeMissBurstSavingThrows,
    )
}

#[must_use]
pub fn project_poison_spray_spell_attack_damage() -> LevelOneDamageSpellState {
    // RAW: cleanroom-input/raw/srd-5.2.1/Spells/Descriptions-M-P.md
    // "Poison Spray"; QNT: battle-runtime-spell-attack.qnt.
    resolved_state(
        false,
        5,
        false,
        false,
        12,
        LevelOneDamageSpellScenarioOutcome::PoisonSpraySpellAttackDamage,
    )
}

#[must_use]
pub fn project_ray_of_sickness_spell_attack_damage_and_poisoned() -> LevelOneDamageSpellState {
    // RAW: cleanroom-input/raw/srd-5.2.1/Spells/Descriptions-Q-R.md
    // "Ray of Sickness"; QNT: battle-runtime-spell-attack.qnt.
    resolved_state(
        true,
        5,
        true,
        false,
        12,
        LevelOneDamageSpellScenarioOutcome::RayOfSicknessSpellAttackDamageAndPoisoned,
    )
}

#[must_use]
pub fn project_sacred_flame_dexterity_saving_throw_radiant_damage() -> LevelOneDamageSpellState {
    // RAW: cleanroom-input/raw/srd-5.2.1/Spells/Descriptions-S-Z.md
    // "Sacred Flame"; QNT: battle-runtime-save-gated-spell.qnt.
    resolved_state(
        false,
        5,
        false,
        false,
        12,
        LevelOneDamageSpellScenarioOutcome::SacredFlameDexteritySavingThrowRadiantDamage,
    )
}

#[must_use]
pub fn project_sorcerous_burst_spell_attack_damage() -> LevelOneDamageSpellState {
    // RAW: cleanroom-input/raw/srd-5.2.1/Spells/Descriptions-S-Z.md
    // "Sorcerous Burst"; QNT: battle-runtime-sorcerous-burst-damage-choice.qnt.
    resolved_state(
        false,
        2,
        false,
        false,
        12,
        LevelOneDamageSpellScenarioOutcome::SorcerousBurstSpellAttackDamage,
    )
}

#[must_use]
pub fn project_starry_wisp_object_spell_attack_damage_and_dim_light() -> LevelOneDamageSpellState {
    // RAW: cleanroom-input/raw/srd-5.2.1/Spells/Descriptions-S-Z.md
    // "Starry Wisp"; QNT: battle-runtime-spell-attack.qnt.
    resolved_state(
        false,
        12,
        false,
        false,
        12,
        LevelOneDamageSpellScenarioOutcome::StarryWispObjectSpellAttackDamageAndDimLight,
    )
}

#[must_use]
pub fn project_vicious_mockery_wisdom_saving_throw_psychic_damage_and_next_attack_disadvantage(
) -> LevelOneDamageSpellState {
    // RAW: cleanroom-input/raw/srd-5.2.1/Spells/Descriptions-S-Z.md
    // "Vicious Mockery"; QNT: battle-runtime-save-gated-spell.qnt.
    resolved_state(false, 6, false, true, 12, LevelOneDamageSpellScenarioOutcome::ViciousMockeryWisdomSavingThrowPsychicDamageAndNextAttackDisadvantage)
}

fn resolved_state(
    spends_level_one_slot: bool,
    primary_target_hit_points: i16,
    primary_target_poisoned: bool,
    primary_target_next_attack_roll_disadvantage: bool,
    secondary_target_hit_points: i16,
    scenario_outcome: LevelOneDamageSpellScenarioOutcome,
) -> LevelOneDamageSpellState {
    LevelOneDamageSpellState {
        action_available: false,
        spell_slot_spent_this_turn: spends_level_one_slot,
        level_one_slots_remaining: u8::from(!spends_level_one_slot),
        primary_target_hit_points,
        primary_target_poisoned,
        primary_target_next_attack_roll_disadvantage,
        secondary_target_hit_points,
        scenario_outcome,
        protocol: LevelOneDamageSpellProtocol::Resolved,
    }
}
