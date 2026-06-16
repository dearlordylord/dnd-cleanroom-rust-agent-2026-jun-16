use crate::rules::feature_resources::apply_temporary_hit_points;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BattleTurnFeatureState {
    pub actor_temporary_hit_points: i16,
    pub bonus_action_available: bool,
    pub dash_bonus_feet: i16,
    pub feature_uses_remaining: i16,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct AdrenalineRushFacts {
    pub turn: BattleTurnFeatureState,
    pub speed_feet: i16,
    pub proficiency_bonus: i16,
    pub actor_owns_turn: bool,
    pub actor_unconscious: bool,
    pub actor_incapacitated: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AdrenalineRushRejection {
    WrongActor,
    StaleSubject,
    NoUsesRemaining,
    InvalidFill,
    UnableToAct,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AdrenalineRushResult {
    Resolved,
    Invalid(AdrenalineRushRejection),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct AdrenalineRushProjection {
    pub turn: BattleTurnFeatureState,
    pub result: AdrenalineRushResult,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SavingThrowAbility {
    Strength,
    Dexterity,
    Constitution,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PassiveSavingThrowRollMode {
    Normal,
    Advantage,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DangerSenseScenarioOutcome {
    Init,
    DexterityAdvantage,
    IncapacitatedSuppressed,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DangerSenseProtocol {
    Init,
    Resolved,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DangerSenseState {
    pub scenario_outcome: DangerSenseScenarioOutcome,
    pub feature_present: bool,
    pub dexterity_roll_mode_count: i16,
    pub constitution_roll_mode_count: i16,
    pub suppressed: bool,
    pub accepted: bool,
    pub protocol: DangerSenseProtocol,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DangerSenseFacts {
    pub saving_throw_ability: SavingThrowAbility,
    pub actor_incapacitated: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BreathWeaponAreaShape {
    Cone15Feet,
    Line30By5Feet,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DragonbornBreathWeaponScenarioOutcome {
    Init,
    Resolved,
    OpenedExtraAttack,
    RejectMissingResource,
    RejectMismatchedArea,
    RejectInvalidDamageRoll,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DragonbornBreathWeaponInvalidReason {
    InvalidFill,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DragonbornBreathWeaponProtocol {
    Init,
    Resolved,
    Invalid(DragonbornBreathWeaponInvalidReason),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DragonbornBreathWeaponState {
    pub target_hit_points: i16,
    pub second_target_hit_points: i16,
    pub breath_weapon_uses_remaining: i16,
    pub attack_action_attacks_remaining: i16,
    pub scenario_outcome: DragonbornBreathWeaponScenarioOutcome,
    pub protocol: DragonbornBreathWeaponProtocol,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DragonbornBreathWeaponFacts {
    pub character_level: i16,
    pub damage_roll: i16,
    pub target_saving_throw_succeeded: bool,
    pub second_target_in_area: bool,
    pub second_target_saving_throw_succeeded: bool,
    pub area_shape: BreathWeaponAreaShape,
    pub expected_area_shape: BreathWeaponAreaShape,
    pub opens_extra_attack_slot: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InnateSorceryOccurrence {
    Inactive,
    ActiveUntilEndOfRound(i16),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InnateSorcerySpellAttackRollMode {
    Normal,
    Advantage,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InnateSorcerySpellBenefitEligibility {
    Eligible,
    Ineligible,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InnateSorceryScenarioOutcome {
    Init,
    Activated,
    SpellBenefitsProjected,
    NonSorcererExcluded,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InnateSorceryProtocol {
    Init,
    Resolved,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct InnateSorceryState {
    pub bonus_action_available: bool,
    pub feature_uses_remaining: i16,
    pub occurrence: InnateSorceryOccurrence,
    pub spell_save_dc: i16,
    pub spell_attack_roll_mode: InnateSorcerySpellAttackRollMode,
    pub scenario_outcome: InnateSorceryScenarioOutcome,
    pub protocol: InnateSorceryProtocol,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct InnateSorcerySpellFacts {
    pub benefit_eligibility: InnateSorcerySpellBenefitEligibility,
}

#[must_use]
pub fn resolve_adrenaline_rush_bonus_action_dash(
    facts: AdrenalineRushFacts,
) -> AdrenalineRushProjection {
    // RAW: cleanroom-input/raw/srd-5.2.1/Character-Origins.md "Orc",
    // cleanroom-input/raw/srd-5.2.1/Playing-the-Game.md "Dash",
    // "Bonus Actions", and "Temporary Hit Points"; QNT:
    // cleanroom-input/qnt/battle-runtime/battle-runtime-adrenaline-rush.mbt.qnt
    // and battle-runtime-movement.qnt bonusActionDashTemporaryHitPointsFighter.
    if !facts.actor_owns_turn {
        return rejected(facts.turn, AdrenalineRushRejection::WrongActor);
    }
    if !facts.turn.bonus_action_available {
        return rejected(facts.turn, AdrenalineRushRejection::StaleSubject);
    }
    if facts.turn.feature_uses_remaining <= 0 {
        return rejected(facts.turn, AdrenalineRushRejection::NoUsesRemaining);
    }
    if !(2..=6).contains(&facts.proficiency_bonus) || facts.speed_feet < 0 {
        return rejected(facts.turn, AdrenalineRushRejection::InvalidFill);
    }
    if facts.actor_unconscious || facts.actor_incapacitated {
        return rejected(facts.turn, AdrenalineRushRejection::UnableToAct);
    }

    AdrenalineRushProjection {
        turn: BattleTurnFeatureState {
            actor_temporary_hit_points: apply_temporary_hit_points(
                facts.turn.actor_temporary_hit_points,
                facts.proficiency_bonus,
            ),
            bonus_action_available: false,
            dash_bonus_feet: facts.turn.dash_bonus_feet + facts.speed_feet,
            feature_uses_remaining: facts.turn.feature_uses_remaining - 1,
        },
        result: AdrenalineRushResult::Resolved,
    }
}

fn rejected(
    turn: BattleTurnFeatureState,
    rejection: AdrenalineRushRejection,
) -> AdrenalineRushProjection {
    AdrenalineRushProjection {
        turn,
        result: AdrenalineRushResult::Invalid(rejection),
    }
}

#[must_use]
pub fn danger_sense_initial_state() -> DangerSenseState {
    DangerSenseState {
        scenario_outcome: DangerSenseScenarioOutcome::Init,
        feature_present: false,
        dexterity_roll_mode_count: 0,
        constitution_roll_mode_count: 0,
        suppressed: false,
        accepted: false,
        protocol: DangerSenseProtocol::Init,
    }
}

#[must_use]
pub fn danger_sense_saving_throw_roll_mode(facts: DangerSenseFacts) -> PassiveSavingThrowRollMode {
    // RAW: cleanroom-input/raw/srd-5.2.1/Classes/Barbarian.md
    // "Level 2: Danger Sense"; QNT:
    // unit-feature-save-damage-core-examples.qnt passiveSavingThrowRollMode.
    if facts.saving_throw_ability == SavingThrowAbility::Dexterity && !facts.actor_incapacitated {
        PassiveSavingThrowRollMode::Advantage
    } else {
        PassiveSavingThrowRollMode::Normal
    }
}

#[must_use]
pub fn project_danger_sense_dexterity_advantage(_state: DangerSenseState) -> DangerSenseState {
    let mode = danger_sense_saving_throw_roll_mode(DangerSenseFacts {
        saving_throw_ability: SavingThrowAbility::Dexterity,
        actor_incapacitated: false,
    });
    DangerSenseState {
        scenario_outcome: DangerSenseScenarioOutcome::DexterityAdvantage,
        feature_present: true,
        dexterity_roll_mode_count: i16::from(mode == PassiveSavingThrowRollMode::Advantage),
        constitution_roll_mode_count: 0,
        suppressed: false,
        accepted: true,
        protocol: DangerSenseProtocol::Resolved,
    }
}

#[must_use]
pub fn suppress_danger_sense_while_incapacitated(_state: DangerSenseState) -> DangerSenseState {
    let mode = danger_sense_saving_throw_roll_mode(DangerSenseFacts {
        saving_throw_ability: SavingThrowAbility::Dexterity,
        actor_incapacitated: true,
    });
    DangerSenseState {
        scenario_outcome: DangerSenseScenarioOutcome::IncapacitatedSuppressed,
        feature_present: true,
        dexterity_roll_mode_count: i16::from(mode == PassiveSavingThrowRollMode::Advantage),
        constitution_roll_mode_count: 0,
        suppressed: true,
        accepted: true,
        protocol: DangerSenseProtocol::Resolved,
    }
}

#[must_use]
pub fn dragonborn_breath_weapon_initial_state() -> DragonbornBreathWeaponState {
    DragonbornBreathWeaponState {
        target_hit_points: 20,
        second_target_hit_points: 20,
        breath_weapon_uses_remaining: 3,
        attack_action_attacks_remaining: 1,
        scenario_outcome: DragonbornBreathWeaponScenarioOutcome::Init,
        protocol: DragonbornBreathWeaponProtocol::Init,
    }
}

#[must_use]
pub fn dragonborn_breath_weapon_save_dc(constitution_modifier: i16, proficiency_bonus: i16) -> i16 {
    // RAW: cleanroom-input/raw/srd-5.2.1/Character-Origins.md
    // "Dragonborn", "Breath Weapon".
    8 + constitution_modifier + proficiency_bonus
}

#[must_use]
pub fn dragonborn_breath_weapon_damage_die_count(character_level: i16) -> Option<i16> {
    // RAW: cleanroom-input/raw/srd-5.2.1/Character-Origins.md
    // "Dragonborn", "Breath Weapon".
    match character_level {
        1..=4 => Some(1),
        5..=10 => Some(2),
        11..=16 => Some(3),
        17.. => Some(4),
        _ => None,
    }
}

#[must_use]
pub fn resolve_dragonborn_breath_weapon(
    state: DragonbornBreathWeaponState,
    facts: DragonbornBreathWeaponFacts,
) -> DragonbornBreathWeaponState {
    // RAW: cleanroom-input/raw/srd-5.2.1/Character-Origins.md
    // "Dragonborn", "Breath Weapon"; QNT:
    // cleanroom-input/qnt/battle-runtime/
    // battle-runtime-dragonborn-breath-weapon.mbt.qnt.
    if state.breath_weapon_uses_remaining <= 0 {
        return invalid_dragonborn_breath_weapon(
            state,
            DragonbornBreathWeaponScenarioOutcome::RejectMissingResource,
        );
    }
    if facts.area_shape != facts.expected_area_shape {
        return invalid_dragonborn_breath_weapon(
            state,
            DragonbornBreathWeaponScenarioOutcome::RejectMismatchedArea,
        );
    }

    let Some(damage_die_count) = dragonborn_breath_weapon_damage_die_count(facts.character_level)
    else {
        return invalid_dragonborn_breath_weapon(
            state,
            DragonbornBreathWeaponScenarioOutcome::RejectInvalidDamageRoll,
        );
    };
    let minimum_damage_roll = damage_die_count;
    let maximum_damage_roll = damage_die_count * 10;
    if facts.damage_roll < minimum_damage_roll || facts.damage_roll > maximum_damage_roll {
        return invalid_dragonborn_breath_weapon(
            state,
            DragonbornBreathWeaponScenarioOutcome::RejectInvalidDamageRoll,
        );
    }

    let target_damage =
        breath_weapon_damage_after_save(facts.damage_roll, facts.target_saving_throw_succeeded);
    let second_target_damage = if facts.second_target_in_area {
        breath_weapon_damage_after_save(
            facts.damage_roll,
            facts.second_target_saving_throw_succeeded,
        )
    } else {
        0
    };

    DragonbornBreathWeaponState {
        target_hit_points: apply_breath_weapon_damage(state.target_hit_points, target_damage),
        second_target_hit_points: apply_breath_weapon_damage(
            state.second_target_hit_points,
            second_target_damage,
        ),
        breath_weapon_uses_remaining: state.breath_weapon_uses_remaining - 1,
        attack_action_attacks_remaining: if facts.opens_extra_attack_slot {
            state.attack_action_attacks_remaining
        } else {
            state.attack_action_attacks_remaining.saturating_sub(1)
        },
        scenario_outcome: if facts.opens_extra_attack_slot {
            DragonbornBreathWeaponScenarioOutcome::OpenedExtraAttack
        } else {
            DragonbornBreathWeaponScenarioOutcome::Resolved
        },
        protocol: DragonbornBreathWeaponProtocol::Resolved,
    }
}

#[must_use]
pub fn reject_dragonborn_breath_weapon_missing_resource(
    state: DragonbornBreathWeaponState,
) -> DragonbornBreathWeaponState {
    invalid_dragonborn_breath_weapon(
        DragonbornBreathWeaponState {
            breath_weapon_uses_remaining: 0,
            ..state
        },
        DragonbornBreathWeaponScenarioOutcome::RejectMissingResource,
    )
}

#[must_use]
pub fn reject_dragonborn_breath_weapon_mismatched_area(
    state: DragonbornBreathWeaponState,
) -> DragonbornBreathWeaponState {
    resolve_dragonborn_breath_weapon(
        state,
        DragonbornBreathWeaponFacts {
            character_level: 1,
            damage_roll: 10,
            target_saving_throw_succeeded: false,
            second_target_in_area: true,
            second_target_saving_throw_succeeded: true,
            area_shape: BreathWeaponAreaShape::Line30By5Feet,
            expected_area_shape: BreathWeaponAreaShape::Cone15Feet,
            opens_extra_attack_slot: false,
        },
    )
}

#[must_use]
pub fn reject_dragonborn_breath_weapon_invalid_damage_roll(
    state: DragonbornBreathWeaponState,
) -> DragonbornBreathWeaponState {
    resolve_dragonborn_breath_weapon(
        state,
        DragonbornBreathWeaponFacts {
            character_level: 1,
            damage_roll: 11,
            target_saving_throw_succeeded: false,
            second_target_in_area: true,
            second_target_saving_throw_succeeded: true,
            area_shape: BreathWeaponAreaShape::Cone15Feet,
            expected_area_shape: BreathWeaponAreaShape::Cone15Feet,
            opens_extra_attack_slot: false,
        },
    )
}

#[must_use]
pub fn sorcerer_spell_save_dc(spellcasting_ability_modifier: i16, proficiency_bonus: i16) -> i16 {
    // RAW: cleanroom-input/raw/srd-5.2.1/Classes/Sorcerer.md,
    // "Spellcasting Ability"; Ubiquitous Language "Spell Save DC".
    8 + spellcasting_ability_modifier + proficiency_bonus
}

#[must_use]
pub fn innate_sorcery_initial_state(base_spell_save_dc: i16) -> InnateSorceryState {
    InnateSorceryState {
        bonus_action_available: true,
        feature_uses_remaining: 2,
        occurrence: InnateSorceryOccurrence::Inactive,
        spell_save_dc: base_spell_save_dc,
        spell_attack_roll_mode: InnateSorcerySpellAttackRollMode::Normal,
        scenario_outcome: InnateSorceryScenarioOutcome::Init,
        protocol: InnateSorceryProtocol::Init,
    }
}

#[must_use]
pub fn activate_innate_sorcery(
    state: InnateSorceryState,
    current_round: i16,
) -> InnateSorceryState {
    // RAW: cleanroom-input/raw/srd-5.2.1/Classes/Sorcerer.md
    // "Level 1: Innate Sorcery"; QNT:
    // battle-runtime-feature-selected-identity.mbt.qnt.
    let Some(active) = active_innate_sorcery_state(state, current_round) else {
        return state;
    };
    InnateSorceryState {
        spell_save_dc: state.spell_save_dc + 1,
        scenario_outcome: InnateSorceryScenarioOutcome::Activated,
        ..active
    }
}

#[must_use]
pub fn project_innate_sorcery_spell_benefits(
    state: InnateSorceryState,
    current_round: i16,
    facts: InnateSorcerySpellFacts,
) -> InnateSorceryState {
    // RAW: cleanroom-input/raw/srd-5.2.1/Classes/Sorcerer.md
    // "Level 1: Innate Sorcery".
    let Some(active) = active_innate_sorcery_state(state, current_round) else {
        return state;
    };
    match facts.benefit_eligibility {
        InnateSorcerySpellBenefitEligibility::Eligible => InnateSorceryState {
            spell_save_dc: state.spell_save_dc + 1,
            spell_attack_roll_mode: InnateSorcerySpellAttackRollMode::Advantage,
            scenario_outcome: InnateSorceryScenarioOutcome::SpellBenefitsProjected,
            ..active
        },
        InnateSorcerySpellBenefitEligibility::Ineligible => InnateSorceryState {
            spell_save_dc: state.spell_save_dc,
            spell_attack_roll_mode: InnateSorcerySpellAttackRollMode::Normal,
            scenario_outcome: InnateSorceryScenarioOutcome::NonSorcererExcluded,
            ..active
        },
    }
}

fn breath_weapon_damage_after_save(damage_roll: i16, saving_throw_succeeded: bool) -> i16 {
    if saving_throw_succeeded {
        damage_roll / 2
    } else {
        damage_roll
    }
}

fn apply_breath_weapon_damage(hit_points: i16, damage: i16) -> i16 {
    hit_points.saturating_sub(damage.max(0)).max(0)
}

fn invalid_dragonborn_breath_weapon(
    state: DragonbornBreathWeaponState,
    scenario_outcome: DragonbornBreathWeaponScenarioOutcome,
) -> DragonbornBreathWeaponState {
    DragonbornBreathWeaponState {
        scenario_outcome,
        protocol: DragonbornBreathWeaponProtocol::Invalid(
            DragonbornBreathWeaponInvalidReason::InvalidFill,
        ),
        ..state
    }
}

fn active_innate_sorcery_state(
    state: InnateSorceryState,
    current_round: i16,
) -> Option<InnateSorceryState> {
    let can_activate = state.bonus_action_available && state.feature_uses_remaining > 0;
    if !can_activate {
        return None;
    }

    Some(InnateSorceryState {
        bonus_action_available: false,
        feature_uses_remaining: state.feature_uses_remaining - 1,
        occurrence: InnateSorceryOccurrence::ActiveUntilEndOfRound(current_round + 10),
        spell_save_dc: state.spell_save_dc,
        spell_attack_roll_mode: InnateSorcerySpellAttackRollMode::Normal,
        scenario_outcome: state.scenario_outcome,
        protocol: InnateSorceryProtocol::Resolved,
    })
}
