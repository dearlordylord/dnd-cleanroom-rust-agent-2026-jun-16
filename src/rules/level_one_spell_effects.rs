use crate::rules::battle_features::SavingThrowAbility;
use crate::rules::spell_shapes::DamageType;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LevelOneSpell {
    DivineFavor,
    DivineSmite,
    EnsnaringStrike,
    FalseLife,
    Heroism,
    HuntersMark,
    Hex,
    Longstrider,
    SearingSmite,
    Shillelagh,
    TrueStrike,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LevelOneSpellEffectScenarioOutcome {
    Init,
    DivineFavor,
    DivineSmite,
    EnsnaringStrike,
    FalseLife,
    Heroism,
    HuntersMark,
    Hex,
    Longstrider,
    SearingSmite,
    Shillelagh,
    TrueStrike,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LevelOneSpellEffectProtocol {
    Init,
    Resolved,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LevelOneEffectTarget {
    None,
    Target,
    TransferTarget,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LevelOneCondition {
    Frightened,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AbilityScore {
    Strength,
    Wisdom,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AbilityCheckSkill {
    Athletics,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MarkTransferAvailability {
    None,
    Available,
    AvailableAfterTurn,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ActiveMarkTransfer {
    None,
    AwaitingTargetDrop,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MarkRetargetTiming {
    None,
    SameTurn,
    LaterTurn,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SaveSuccessEnding {
    None,
    Spell,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SpellWeapon {
    None,
    Quarterstaff,
    Dagger,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SpellWeaponComponent {
    None,
    MainDagger,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WeaponAttackPresentation {
    None,
    QuarterstaffForce,
    Dagger,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DamageDice {
    pub dice: u8,
    pub die_size: u8,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DamageRiderEffect {
    pub spell: Option<LevelOneSpell>,
    pub damage_type: Option<DamageType>,
    pub dice: DamageDice,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TemporaryHitPointsFormula {
    pub spell: Option<LevelOneSpell>,
    pub dice: DamageDice,
    pub flat: i16,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct FrightenedImmunityEffect {
    pub spell: Option<LevelOneSpell>,
    pub condition: Option<LevelOneCondition>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TurnStartTemporaryHitPointsEffect {
    pub spell: Option<LevelOneSpell>,
    pub amount: i16,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RestrainedEscapeEffect {
    pub restrained_before_escape: bool,
    pub target_restrained: bool,
    pub save_spell: Option<LevelOneSpell>,
    pub save_ability: Option<SavingThrowAbility>,
    pub turn_start_damage: DamageRiderEffect,
    pub escape_ability: Option<AbilityScore>,
    pub escape_skill: Option<AbilityCheckSkill>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MarkedDamageEffect {
    pub damage: DamageRiderEffect,
    pub active_mark_spell: Option<LevelOneSpell>,
    pub active_mark_target: LevelOneEffectTarget,
    pub concentration_spell: Option<LevelOneSpell>,
    pub transfer_on_drop_turn: MarkTransferAvailability,
    pub active_mark_transfer: ActiveMarkTransfer,
    pub retarget_timing: MarkRetargetTiming,
    pub transfer_visible_on_drop_turn: bool,
    pub ability_check_ability: Option<AbilityScore>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct LongstriderSpeedEffect {
    pub target_speed_feet: u16,
    pub spell: Option<LevelOneSpell>,
    pub target: LevelOneEffectTarget,
    pub delta_feet: i16,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SearingSmiteLifecycleEffect {
    pub immediate_damage: DamageRiderEffect,
    pub active_before_successful_save: bool,
    pub turn_start_damage: DamageRiderEffect,
    pub turn_start_save_spell: Option<LevelOneSpell>,
    pub turn_start_save_ability: Option<SavingThrowAbility>,
    pub successful_save_ending: SaveSuccessEnding,
    pub active_after_successful_save: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct WeaponAttackOverrideEffect {
    pub spell: Option<LevelOneSpell>,
    pub weapon: SpellWeapon,
    pub spellcasting_ability_modifier: i16,
    pub override_attack_bonus: i16,
    pub override_damage: DamageDice,
    pub attack_presentation: WeaponAttackPresentation,
    pub attack_bonus: i16,
    pub damage_type: Option<DamageType>,
    pub damage: DamageDice,
    pub damage_modifier: i16,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SpellHostedWeaponAttackEffect {
    pub spell: Option<LevelOneSpell>,
    pub component_weapon: SpellWeaponComponent,
    pub weapon: SpellWeapon,
    pub attack_presentation: WeaponAttackPresentation,
    pub attack_bonus: i16,
    pub damage_type: Option<DamageType>,
    pub damage: DamageDice,
    pub damage_modifier: i16,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct LevelOneSpellEffectProjection {
    pub divine_favor_active_rider_count: u8,
    pub target_hit_points: i16,
    pub longstrider: LongstriderSpeedEffect,
    pub caster_temporary_hit_points: i16,
    pub caster_frightened: bool,
    pub spell_slot_spent_this_turn: bool,
    pub level_one_slots_remaining: u8,
    pub damage_rider: DamageRiderEffect,
    pub temporary_hit_points: TemporaryHitPointsFormula,
    pub frightened_immunity: FrightenedImmunityEffect,
    pub turn_start_temporary_hit_points: TurnStartTemporaryHitPointsEffect,
    pub ensnaring_strike: RestrainedEscapeEffect,
    pub caster_concentrating: bool,
    pub hunters_mark: MarkedDamageEffect,
    pub hex: MarkedDamageEffect,
    pub searing_smite: SearingSmiteLifecycleEffect,
    pub shillelagh: WeaponAttackOverrideEffect,
    pub true_strike: SpellHostedWeaponAttackEffect,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct LevelOneSpellEffectState {
    pub projection: LevelOneSpellEffectProjection,
    pub scenario_outcome: LevelOneSpellEffectScenarioOutcome,
    pub protocol: LevelOneSpellEffectProtocol,
}

#[must_use]
pub fn level_one_spell_effects_initial_state() -> LevelOneSpellEffectState {
    state(
        default_projection(),
        LevelOneSpellEffectScenarioOutcome::Init,
    )
}

#[must_use]
pub fn project_divine_favor_weapon_damage_rider() -> LevelOneSpellEffectState {
    // RAW: cleanroom-input/raw/srd-5.2.1/Spells/Descriptions-A-D.md
    // "Divine Favor"; QNT: spell-damage-rider-projection-core.qnt.
    let mut projection = default_projection();
    projection.divine_favor_active_rider_count = 1;
    projection.target_hit_points = 5;
    projection.spell_slot_spent_this_turn = true;
    projection.level_one_slots_remaining = 1;
    projection.damage_rider = damage_rider(LevelOneSpell::DivineFavor, DamageType::Radiant, 1, 4);
    state(projection, LevelOneSpellEffectScenarioOutcome::DivineFavor)
}

#[must_use]
pub fn project_divine_smite_after_hit_damage() -> LevelOneSpellEffectState {
    // RAW: cleanroom-input/raw/srd-5.2.1/Spells/Descriptions-A-D.md
    // "Divine Smite"; QNT: spell-damage-rider-projection-core.qnt.
    let mut projection = default_projection();
    projection.target_hit_points = 1;
    projection.spell_slot_spent_this_turn = true;
    projection.level_one_slots_remaining = 1;
    projection.damage_rider = damage_rider(LevelOneSpell::DivineSmite, DamageType::Radiant, 2, 8);
    state(projection, LevelOneSpellEffectScenarioOutcome::DivineSmite)
}

#[must_use]
pub fn project_ensnaring_strike_restraint_damage_and_escape() -> LevelOneSpellEffectState {
    // RAW: cleanroom-input/raw/srd-5.2.1/Spells/Descriptions-E-L.md
    // "Ensnaring Strike"; QNT: spell-damage-rider-projection-core.qnt.
    let mut projection = default_projection();
    projection.target_hit_points = 5;
    projection.level_one_slots_remaining = 1;
    projection.ensnaring_strike = RestrainedEscapeEffect {
        restrained_before_escape: true,
        target_restrained: false,
        save_spell: Some(LevelOneSpell::EnsnaringStrike),
        save_ability: Some(SavingThrowAbility::Strength),
        turn_start_damage: damage_rider(LevelOneSpell::EnsnaringStrike, DamageType::Piercing, 1, 6),
        escape_ability: Some(AbilityScore::Strength),
        escape_skill: Some(AbilityCheckSkill::Athletics),
    };
    state(
        projection,
        LevelOneSpellEffectScenarioOutcome::EnsnaringStrike,
    )
}

#[must_use]
pub fn project_false_life_temporary_hit_points() -> LevelOneSpellEffectState {
    // RAW: cleanroom-input/raw/srd-5.2.1/Spells/Descriptions-E-L.md
    // "False Life"; QNT: spell-scalar-buff-projection-core.qnt.
    let mut projection = default_projection();
    projection.caster_temporary_hit_points = 11;
    projection.spell_slot_spent_this_turn = true;
    projection.level_one_slots_remaining = 1;
    projection.temporary_hit_points = TemporaryHitPointsFormula {
        spell: Some(LevelOneSpell::FalseLife),
        dice: dice(2, 4),
        flat: 4,
    };
    state(projection, LevelOneSpellEffectScenarioOutcome::FalseLife)
}

#[must_use]
pub fn project_heroism_frightened_immunity_and_turn_start_temporary_hit_points(
) -> LevelOneSpellEffectState {
    // RAW: cleanroom-input/raw/srd-5.2.1/Spells/Descriptions-E-L.md
    // "Heroism"; QNT: spell-scalar-buff-projection-core.qnt.
    let mut projection = default_projection();
    projection.caster_temporary_hit_points = 3;
    projection.level_one_slots_remaining = 1;
    projection.caster_concentrating = true;
    projection.frightened_immunity = FrightenedImmunityEffect {
        spell: Some(LevelOneSpell::Heroism),
        condition: Some(LevelOneCondition::Frightened),
    };
    projection.turn_start_temporary_hit_points = TurnStartTemporaryHitPointsEffect {
        spell: Some(LevelOneSpell::Heroism),
        amount: 3,
    };
    state(projection, LevelOneSpellEffectScenarioOutcome::Heroism)
}

#[must_use]
pub fn project_heroism_cleanup_after_effect_end() -> LevelOneSpellEffectState {
    // RAW: cleanroom-input/raw/srd-5.2.1/Spells/Descriptions-E-L.md
    // "Heroism"; QNT: battle-runtime-restoration-and-buffs.qnt.
    let mut projection = default_projection();
    projection.caster_temporary_hit_points = 3;
    projection.caster_frightened = true;
    projection.level_one_slots_remaining = 1;
    state(projection, LevelOneSpellEffectScenarioOutcome::Heroism)
}

#[must_use]
pub fn project_hunters_mark_damage_rider_and_same_turn_transfer() -> LevelOneSpellEffectState {
    // RAW: cleanroom-input/raw/srd-5.2.1/Spells/Descriptions-E-L.md
    // "Hunter's Mark"; QNT: battle-runtime-marked-spells.qnt.
    let mut projection = default_projection();
    projection.target_hit_points = 0;
    projection.level_one_slots_remaining = 1;
    projection.caster_concentrating = true;
    projection.hunters_mark = MarkedDamageEffect {
        damage: damage_rider(LevelOneSpell::HuntersMark, DamageType::Force, 1, 6),
        active_mark_spell: Some(LevelOneSpell::HuntersMark),
        active_mark_target: LevelOneEffectTarget::TransferTarget,
        concentration_spell: Some(LevelOneSpell::HuntersMark),
        transfer_on_drop_turn: MarkTransferAvailability::Available,
        active_mark_transfer: ActiveMarkTransfer::AwaitingTargetDrop,
        retarget_timing: MarkRetargetTiming::SameTurn,
        transfer_visible_on_drop_turn: true,
        ability_check_ability: None,
    };
    state(projection, LevelOneSpellEffectScenarioOutcome::HuntersMark)
}

#[must_use]
pub fn project_hex_damage_rider_and_later_turn_transfer() -> LevelOneSpellEffectState {
    // RAW: cleanroom-input/raw/srd-5.2.1/Spells/Descriptions-E-L.md "Hex";
    // QNT: battle-runtime-marked-spells.qnt.
    let mut projection = default_projection();
    projection.target_hit_points = 0;
    projection.level_one_slots_remaining = 1;
    projection.caster_concentrating = true;
    projection.hex = MarkedDamageEffect {
        damage: damage_rider(LevelOneSpell::Hex, DamageType::Necrotic, 1, 6),
        active_mark_spell: Some(LevelOneSpell::Hex),
        active_mark_target: LevelOneEffectTarget::TransferTarget,
        concentration_spell: None,
        transfer_on_drop_turn: MarkTransferAvailability::AvailableAfterTurn,
        active_mark_transfer: ActiveMarkTransfer::AwaitingTargetDrop,
        retarget_timing: MarkRetargetTiming::LaterTurn,
        transfer_visible_on_drop_turn: false,
        ability_check_ability: Some(AbilityScore::Wisdom),
    };
    state(projection, LevelOneSpellEffectScenarioOutcome::Hex)
}

#[must_use]
pub fn project_longstrider_speed_increase() -> LevelOneSpellEffectState {
    // RAW: cleanroom-input/raw/srd-5.2.1/Spells/Descriptions-E-L.md
    // "Longstrider"; QNT: spell-scalar-buff-projection-core.qnt.
    let mut projection = default_projection();
    projection.longstrider = LongstriderSpeedEffect {
        target_speed_feet: 40,
        spell: Some(LevelOneSpell::Longstrider),
        target: LevelOneEffectTarget::Target,
        delta_feet: 10,
    };
    projection.spell_slot_spent_this_turn = true;
    projection.level_one_slots_remaining = 1;
    state(projection, LevelOneSpellEffectScenarioOutcome::Longstrider)
}

#[must_use]
pub fn project_searing_smite_timed_damage_and_save_cleanup() -> LevelOneSpellEffectState {
    // RAW: cleanroom-input/raw/srd-5.2.1/Spells/Descriptions-S-Z.md
    // "Searing Smite"; QNT: spell-damage-rider-projection-core.qnt.
    let mut projection = default_projection();
    projection.target_hit_points = 1;
    projection.level_one_slots_remaining = 1;
    projection.searing_smite = SearingSmiteLifecycleEffect {
        immediate_damage: damage_rider(LevelOneSpell::SearingSmite, DamageType::Fire, 1, 6),
        active_before_successful_save: true,
        turn_start_damage: damage_rider(LevelOneSpell::SearingSmite, DamageType::Fire, 1, 6),
        turn_start_save_spell: Some(LevelOneSpell::SearingSmite),
        turn_start_save_ability: Some(SavingThrowAbility::Constitution),
        successful_save_ending: SaveSuccessEnding::Spell,
        active_after_successful_save: false,
    };
    state(projection, LevelOneSpellEffectScenarioOutcome::SearingSmite)
}

#[must_use]
pub fn project_shillelagh_weapon_attack_override() -> LevelOneSpellEffectState {
    // RAW: cleanroom-input/raw/srd-5.2.1/Spells/Descriptions-S-Z.md
    // "Shillelagh"; QNT: battle-runtime-light.qnt.
    let mut projection = default_projection();
    projection.target_hit_points = 5;
    projection.shillelagh = WeaponAttackOverrideEffect {
        spell: Some(LevelOneSpell::Shillelagh),
        weapon: SpellWeapon::Quarterstaff,
        spellcasting_ability_modifier: 3,
        override_attack_bonus: shillelagh_attack_modifier(3, 2),
        override_damage: shillelagh_damage_dice(1),
        attack_presentation: WeaponAttackPresentation::QuarterstaffForce,
        attack_bonus: shillelagh_attack_modifier(3, 2),
        damage_type: Some(DamageType::Force),
        damage: shillelagh_damage_dice(1),
        damage_modifier: 3,
    };
    state(projection, LevelOneSpellEffectScenarioOutcome::Shillelagh)
}

#[must_use]
pub fn project_true_strike_spell_hosted_weapon_attack() -> LevelOneSpellEffectState {
    // RAW: cleanroom-input/raw/srd-5.2.1/Spells/Descriptions-S-Z.md
    // "True Strike"; QNT: battle-runtime-attack-facts.qnt.
    let mut projection = default_projection();
    projection.target_hit_points = 5;
    projection.true_strike = SpellHostedWeaponAttackEffect {
        spell: Some(LevelOneSpell::TrueStrike),
        component_weapon: SpellWeaponComponent::MainDagger,
        weapon: SpellWeapon::Dagger,
        attack_presentation: WeaponAttackPresentation::Dagger,
        attack_bonus: true_strike_weapon_attack_modifier(3, 2),
        damage_type: Some(DamageType::Radiant),
        damage: dice(1, 4),
        damage_modifier: 3,
    };
    state(projection, LevelOneSpellEffectScenarioOutcome::TrueStrike)
}

#[must_use]
pub fn shillelagh_attack_modifier(
    spellcasting_ability_modifier: i16,
    proficiency_bonus: i16,
) -> i16 {
    spellcasting_ability_modifier + proficiency_bonus
}

#[must_use]
pub fn shillelagh_damage_dice(character_level: u8) -> DamageDice {
    if character_level >= 17 {
        dice(2, 6)
    } else if character_level >= 11 {
        dice(1, 12)
    } else if character_level >= 5 {
        dice(1, 10)
    } else {
        dice(1, 8)
    }
}

#[must_use]
pub fn true_strike_weapon_attack_modifier(
    spellcasting_ability_modifier: i16,
    proficiency_bonus: i16,
) -> i16 {
    spellcasting_ability_modifier + proficiency_bonus
}

#[must_use]
pub fn true_strike_bonus_damage_dice(character_level: u8) -> DamageDice {
    if character_level >= 17 {
        dice(3, 6)
    } else if character_level >= 11 {
        dice(2, 6)
    } else if character_level >= 5 {
        dice(1, 6)
    } else {
        dice(0, 0)
    }
}

fn state(
    projection: LevelOneSpellEffectProjection,
    scenario_outcome: LevelOneSpellEffectScenarioOutcome,
) -> LevelOneSpellEffectState {
    let protocol = if scenario_outcome == LevelOneSpellEffectScenarioOutcome::Init {
        LevelOneSpellEffectProtocol::Init
    } else {
        LevelOneSpellEffectProtocol::Resolved
    };
    LevelOneSpellEffectState {
        projection,
        scenario_outcome,
        protocol,
    }
}

fn default_projection() -> LevelOneSpellEffectProjection {
    LevelOneSpellEffectProjection {
        divine_favor_active_rider_count: 0,
        target_hit_points: 12,
        longstrider: LongstriderSpeedEffect {
            target_speed_feet: 30,
            spell: None,
            target: LevelOneEffectTarget::None,
            delta_feet: 0,
        },
        caster_temporary_hit_points: 0,
        caster_frightened: false,
        spell_slot_spent_this_turn: false,
        level_one_slots_remaining: 2,
        damage_rider: no_damage_rider(),
        temporary_hit_points: TemporaryHitPointsFormula {
            spell: None,
            dice: dice(0, 0),
            flat: 0,
        },
        frightened_immunity: FrightenedImmunityEffect {
            spell: None,
            condition: None,
        },
        turn_start_temporary_hit_points: TurnStartTemporaryHitPointsEffect {
            spell: None,
            amount: 0,
        },
        ensnaring_strike: RestrainedEscapeEffect {
            restrained_before_escape: false,
            target_restrained: false,
            save_spell: None,
            save_ability: None,
            turn_start_damage: no_damage_rider(),
            escape_ability: None,
            escape_skill: None,
        },
        caster_concentrating: false,
        hunters_mark: no_marked_damage_effect(),
        hex: no_marked_damage_effect(),
        searing_smite: SearingSmiteLifecycleEffect {
            immediate_damage: no_damage_rider(),
            active_before_successful_save: false,
            turn_start_damage: no_damage_rider(),
            turn_start_save_spell: None,
            turn_start_save_ability: None,
            successful_save_ending: SaveSuccessEnding::None,
            active_after_successful_save: false,
        },
        shillelagh: WeaponAttackOverrideEffect {
            spell: None,
            weapon: SpellWeapon::None,
            spellcasting_ability_modifier: 0,
            override_attack_bonus: 0,
            override_damage: dice(0, 0),
            attack_presentation: WeaponAttackPresentation::None,
            attack_bonus: 0,
            damage_type: None,
            damage: dice(0, 0),
            damage_modifier: 0,
        },
        true_strike: SpellHostedWeaponAttackEffect {
            spell: None,
            component_weapon: SpellWeaponComponent::None,
            weapon: SpellWeapon::None,
            attack_presentation: WeaponAttackPresentation::None,
            attack_bonus: 0,
            damage_type: None,
            damage: dice(0, 0),
            damage_modifier: 0,
        },
    }
}

fn damage_rider(
    spell: LevelOneSpell,
    damage_type: DamageType,
    dice_count: u8,
    die_size: u8,
) -> DamageRiderEffect {
    DamageRiderEffect {
        spell: Some(spell),
        damage_type: Some(damage_type),
        dice: dice(dice_count, die_size),
    }
}

fn no_damage_rider() -> DamageRiderEffect {
    DamageRiderEffect {
        spell: None,
        damage_type: None,
        dice: dice(0, 0),
    }
}

fn no_marked_damage_effect() -> MarkedDamageEffect {
    MarkedDamageEffect {
        damage: no_damage_rider(),
        active_mark_spell: None,
        active_mark_target: LevelOneEffectTarget::None,
        concentration_spell: None,
        transfer_on_drop_turn: MarkTransferAvailability::None,
        active_mark_transfer: ActiveMarkTransfer::None,
        retarget_timing: MarkRetargetTiming::None,
        transfer_visible_on_drop_turn: false,
        ability_check_ability: None,
    }
}

fn dice(dice_count: u8, die_size: u8) -> DamageDice {
    DamageDice {
        dice: dice_count,
        die_size,
    }
}
