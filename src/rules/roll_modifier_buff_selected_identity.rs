#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RollModifierBuffSign {
    None,
    Plus,
    Minus,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RollModifierBuffSkill {
    None,
    Stealth,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RollModifierBuffDamageType {
    None,
    Bludgeoning,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RollModifierBuffScenarioOutcome {
    Init,
    Bless,
    Bane,
    Guidance,
    Resistance,
    ShieldOfFaith,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RollModifierBuffProtocol {
    Init,
    Resolved,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RollModifierBuffSelectedIdentityState {
    pub caster_concentrating: bool,
    pub caster_hp: i16,
    pub caster_effect_count: u8,
    pub primary_target_effect_count: u8,
    pub secondary_target_effect_count: u8,
    pub primary_target_armor_class: u8,
    pub primary_target_hp: i16,
    pub d20_modifier_sign: RollModifierBuffSign,
    pub d20_modifier_attack_roll: bool,
    pub d20_modifier_saving_throw: bool,
    pub d20_modifier_ability_check: bool,
    pub d20_modifier_skill: RollModifierBuffSkill,
    pub invalid_target_rejected: bool,
    pub damage_reduction_type: RollModifierBuffDamageType,
    pub damage_reduction_used: bool,
    pub scenario_outcome: RollModifierBuffScenarioOutcome,
    pub protocol: RollModifierBuffProtocol,
}

#[must_use]
pub fn roll_modifier_buff_selected_identity_initial_state() -> RollModifierBuffSelectedIdentityState
{
    base_projection(
        RollModifierBuffScenarioOutcome::Init,
        RollModifierBuffProtocol::Init,
    )
}

#[must_use]
pub fn project_bless_attack_and_save_modifier() -> RollModifierBuffSelectedIdentityState {
    // RAW: cleanroom-input/raw/srd-5.2.1/Spells/Descriptions-A-D.md
    // "Bless"; QNT:
    // battle-runtime-roll-modifier-buff-selected-identity.mbt.qnt.
    let mut state = base_projection(
        RollModifierBuffScenarioOutcome::Bless,
        RollModifierBuffProtocol::Resolved,
    );
    state.caster_concentrating = true;
    state.primary_target_effect_count = 1;
    state.secondary_target_effect_count = 1;
    state.d20_modifier_sign = RollModifierBuffSign::Plus;
    state.d20_modifier_attack_roll = true;
    state.d20_modifier_saving_throw = true;
    state
}

#[must_use]
pub fn project_bane_failed_save_penalty() -> RollModifierBuffSelectedIdentityState {
    // RAW: cleanroom-input/raw/srd-5.2.1/Spells/Descriptions-A-D.md
    // "Bane"; QNT:
    // battle-runtime-roll-modifier-buff-selected-identity.mbt.qnt.
    let mut state = base_projection(
        RollModifierBuffScenarioOutcome::Bane,
        RollModifierBuffProtocol::Resolved,
    );
    state.caster_concentrating = true;
    state.primary_target_effect_count = 1;
    state.d20_modifier_sign = RollModifierBuffSign::Minus;
    state.d20_modifier_attack_roll = true;
    state.d20_modifier_saving_throw = true;
    state
}

#[must_use]
pub fn project_guidance_skill_ability_check_modifier() -> RollModifierBuffSelectedIdentityState {
    // RAW: cleanroom-input/raw/srd-5.2.1/Spells/Descriptions-E-L.md
    // "Guidance"; QNT:
    // battle-runtime-roll-modifier-buff-selected-identity.mbt.qnt.
    let mut state = base_projection(
        RollModifierBuffScenarioOutcome::Guidance,
        RollModifierBuffProtocol::Resolved,
    );
    state.caster_concentrating = true;
    state.caster_effect_count = 1;
    state.d20_modifier_sign = RollModifierBuffSign::Plus;
    state.d20_modifier_ability_check = true;
    state.d20_modifier_skill = RollModifierBuffSkill::Stealth;
    state.invalid_target_rejected = true;
    state
}

#[must_use]
pub fn project_resistance_matching_damage_reduction() -> RollModifierBuffSelectedIdentityState {
    // RAW: cleanroom-input/raw/srd-5.2.1/Spells/Descriptions-Q-R.md
    // "Resistance"; QNT:
    // battle-runtime-roll-modifier-buff-selected-identity.mbt.qnt.
    let mut state = base_projection(
        RollModifierBuffScenarioOutcome::Resistance,
        RollModifierBuffProtocol::Resolved,
    );
    state.caster_concentrating = true;
    state.caster_effect_count = 1;
    state.damage_reduction_type = RollModifierBuffDamageType::Bludgeoning;
    state.damage_reduction_used = true;
    state
}

#[must_use]
pub fn project_shield_of_faith_armor_class_bonus() -> RollModifierBuffSelectedIdentityState {
    // RAW: cleanroom-input/raw/srd-5.2.1/Spells/Descriptions-S-Z.md
    // "Shield of Faith"; QNT:
    // battle-runtime-roll-modifier-buff-selected-identity.mbt.qnt.
    let mut state = base_projection(
        RollModifierBuffScenarioOutcome::ShieldOfFaith,
        RollModifierBuffProtocol::Resolved,
    );
    state.caster_concentrating = true;
    state.primary_target_effect_count = 1;
    state.primary_target_armor_class = 12;
    state
}

fn base_projection(
    scenario_outcome: RollModifierBuffScenarioOutcome,
    protocol: RollModifierBuffProtocol,
) -> RollModifierBuffSelectedIdentityState {
    RollModifierBuffSelectedIdentityState {
        caster_concentrating: false,
        caster_hp: 12,
        caster_effect_count: 0,
        primary_target_effect_count: 0,
        secondary_target_effect_count: 0,
        primary_target_armor_class: 10,
        primary_target_hp: 12,
        d20_modifier_sign: RollModifierBuffSign::None,
        d20_modifier_attack_roll: false,
        d20_modifier_saving_throw: false,
        d20_modifier_ability_check: false,
        d20_modifier_skill: RollModifierBuffSkill::None,
        invalid_target_rejected: false,
        damage_reduction_type: RollModifierBuffDamageType::None,
        damage_reduction_used: false,
        scenario_outcome,
        protocol,
    }
}
