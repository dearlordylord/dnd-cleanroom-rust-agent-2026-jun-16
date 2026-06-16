#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ThaumaturgyAbility {
    Charisma,
    Wisdom,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ThaumaturgySkill {
    Intimidation,
    Perception,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ThaumaturgyRollMode {
    Normal,
    Advantage,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ThaumaturgySelectedIdentityScenarioOutcome {
    Init,
    Resolved,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ThaumaturgySelectedIdentityProtocol {
    Init,
    Resolved,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ThaumaturgySelectedIdentityState {
    pub caster_effect_count: i16,
    pub action_available: bool,
    pub intimidation_roll_mode: ThaumaturgyRollMode,
    pub wisdom_intimidation_roll_mode: ThaumaturgyRollMode,
    pub perception_roll_mode: ThaumaturgyRollMode,
    pub scenario_outcome: ThaumaturgySelectedIdentityScenarioOutcome,
    pub protocol: ThaumaturgySelectedIdentityProtocol,
}

pub const THAUMATURGY_BOOMING_VOICE_DURATION_TICKS: i16 = 10;
pub const THAUMATURGY_MAX_ACTIVE_ONE_MINUTE_EFFECTS: i16 = 3;

#[must_use]
pub fn thaumaturgy_selected_identity_initial_state() -> ThaumaturgySelectedIdentityState {
    ThaumaturgySelectedIdentityState {
        caster_effect_count: 0,
        action_available: true,
        intimidation_roll_mode: ThaumaturgyRollMode::Normal,
        wisdom_intimidation_roll_mode: ThaumaturgyRollMode::Normal,
        perception_roll_mode: ThaumaturgyRollMode::Normal,
        scenario_outcome: ThaumaturgySelectedIdentityScenarioOutcome::Init,
        protocol: ThaumaturgySelectedIdentityProtocol::Init,
    }
}

#[must_use]
pub fn resolve_thaumaturgy_booming_voice() -> ThaumaturgySelectedIdentityState {
    // RAW: cleanroom-input/raw/srd-5.2.1/Spells/Descriptions-S-Z.md
    // "Thaumaturgy", "Booming Voice"; QNT:
    // battle-runtime-thaumaturgy-selected-identity.mbt.qnt.
    ThaumaturgySelectedIdentityState {
        caster_effect_count: 1,
        action_available: false,
        intimidation_roll_mode: thaumaturgy_booming_voice_roll_mode(
            true,
            ThaumaturgyAbility::Charisma,
            ThaumaturgySkill::Intimidation,
        ),
        wisdom_intimidation_roll_mode: thaumaturgy_booming_voice_roll_mode(
            true,
            ThaumaturgyAbility::Wisdom,
            ThaumaturgySkill::Intimidation,
        ),
        perception_roll_mode: thaumaturgy_booming_voice_roll_mode(
            true,
            ThaumaturgyAbility::Charisma,
            ThaumaturgySkill::Perception,
        ),
        scenario_outcome: ThaumaturgySelectedIdentityScenarioOutcome::Resolved,
        protocol: ThaumaturgySelectedIdentityProtocol::Resolved,
    }
}

#[must_use]
pub fn thaumaturgy_booming_voice_roll_mode(
    effect_active: bool,
    ability: ThaumaturgyAbility,
    skill: ThaumaturgySkill,
) -> ThaumaturgyRollMode {
    if effect_active
        && ability == ThaumaturgyAbility::Charisma
        && skill == ThaumaturgySkill::Intimidation
    {
        ThaumaturgyRollMode::Advantage
    } else {
        ThaumaturgyRollMode::Normal
    }
}
