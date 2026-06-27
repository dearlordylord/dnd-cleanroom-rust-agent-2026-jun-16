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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RollModifierBuffTarget {
    Caster,
    PrimaryTarget,
    PrimaryAndSecondaryTargets,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct D20RollModifierBuff {
    pub target: RollModifierBuffTarget,
    pub sign: RollModifierBuffSign,
    pub attack_roll: bool,
    pub saving_throw: bool,
    pub ability_check: bool,
    pub skill: RollModifierBuffSkill,
    pub rejects_unselected_target: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DamageReductionBuff {
    pub target: RollModifierBuffTarget,
    pub damage_type: RollModifierBuffDamageType,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ArmorClassBuff {
    pub target: RollModifierBuffTarget,
    pub bonus: u8,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RollModifierBuffSubstrate {
    D20Modifier(D20RollModifierBuff),
    DamageReduction(DamageReductionBuff),
    ArmorClass(ArmorClassBuff),
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
pub fn project_roll_modifier_buff_substrate(
    substrate: RollModifierBuffSubstrate,
    scenario_outcome: RollModifierBuffScenarioOutcome,
) -> RollModifierBuffSelectedIdentityState {
    // QNT: battle-runtime-roll-modifier-buff-selected-identity.mbt.qnt.
    // The selected identity boundary chooses one of these typed substrates; the
    // projection below dispatches only on the executable substrate shape.
    let mut state = base_projection(scenario_outcome, RollModifierBuffProtocol::Resolved);
    state.caster_concentrating = true;
    match substrate {
        RollModifierBuffSubstrate::D20Modifier(buff) => {
            apply_effect_count(&mut state, buff.target);
            state.d20_modifier_sign = buff.sign;
            state.d20_modifier_attack_roll = buff.attack_roll;
            state.d20_modifier_saving_throw = buff.saving_throw;
            state.d20_modifier_ability_check = buff.ability_check;
            state.d20_modifier_skill = buff.skill;
            state.invalid_target_rejected = buff.rejects_unselected_target;
        }
        RollModifierBuffSubstrate::DamageReduction(buff) => {
            apply_effect_count(&mut state, buff.target);
            state.damage_reduction_type = buff.damage_type;
            state.damage_reduction_used = buff.damage_type != RollModifierBuffDamageType::None;
        }
        RollModifierBuffSubstrate::ArmorClass(buff) => {
            apply_effect_count(&mut state, buff.target);
            state.primary_target_armor_class += buff.bonus;
        }
    }
    state
}

#[must_use]
pub fn project_bless_attack_and_save_modifier() -> RollModifierBuffSelectedIdentityState {
    // RAW: cleanroom-input/raw/srd-5.2.1/Spells/Descriptions-A-D.md
    // "Bless"; QNT:
    // battle-runtime-roll-modifier-buff-selected-identity.mbt.qnt.
    project_roll_modifier_buff_substrate(
        RollModifierBuffSubstrate::D20Modifier(D20RollModifierBuff {
            target: RollModifierBuffTarget::PrimaryAndSecondaryTargets,
            sign: RollModifierBuffSign::Plus,
            attack_roll: true,
            saving_throw: true,
            ability_check: false,
            skill: RollModifierBuffSkill::None,
            rejects_unselected_target: false,
        }),
        RollModifierBuffScenarioOutcome::Bless,
    )
}

#[must_use]
pub fn project_bane_failed_save_penalty() -> RollModifierBuffSelectedIdentityState {
    // RAW: cleanroom-input/raw/srd-5.2.1/Spells/Descriptions-A-D.md
    // "Bane"; QNT:
    // battle-runtime-roll-modifier-buff-selected-identity.mbt.qnt.
    project_roll_modifier_buff_substrate(
        RollModifierBuffSubstrate::D20Modifier(D20RollModifierBuff {
            target: RollModifierBuffTarget::PrimaryTarget,
            sign: RollModifierBuffSign::Minus,
            attack_roll: true,
            saving_throw: true,
            ability_check: false,
            skill: RollModifierBuffSkill::None,
            rejects_unselected_target: false,
        }),
        RollModifierBuffScenarioOutcome::Bane,
    )
}

#[must_use]
pub fn project_guidance_skill_ability_check_modifier() -> RollModifierBuffSelectedIdentityState {
    // RAW: cleanroom-input/raw/srd-5.2.1/Spells/Descriptions-E-L.md
    // "Guidance"; QNT:
    // battle-runtime-roll-modifier-buff-selected-identity.mbt.qnt.
    project_roll_modifier_buff_substrate(
        RollModifierBuffSubstrate::D20Modifier(D20RollModifierBuff {
            target: RollModifierBuffTarget::Caster,
            sign: RollModifierBuffSign::Plus,
            attack_roll: false,
            saving_throw: false,
            ability_check: true,
            skill: RollModifierBuffSkill::Stealth,
            rejects_unselected_target: true,
        }),
        RollModifierBuffScenarioOutcome::Guidance,
    )
}

#[must_use]
pub fn project_resistance_matching_damage_reduction() -> RollModifierBuffSelectedIdentityState {
    // RAW: cleanroom-input/raw/srd-5.2.1/Spells/Descriptions-Q-R.md
    // "Resistance"; QNT:
    // battle-runtime-roll-modifier-buff-selected-identity.mbt.qnt.
    project_roll_modifier_buff_substrate(
        RollModifierBuffSubstrate::DamageReduction(DamageReductionBuff {
            target: RollModifierBuffTarget::Caster,
            damage_type: RollModifierBuffDamageType::Bludgeoning,
        }),
        RollModifierBuffScenarioOutcome::Resistance,
    )
}

#[must_use]
pub fn project_shield_of_faith_armor_class_bonus() -> RollModifierBuffSelectedIdentityState {
    // RAW: cleanroom-input/raw/srd-5.2.1/Spells/Descriptions-S-Z.md
    // "Shield of Faith"; QNT:
    // battle-runtime-roll-modifier-buff-selected-identity.mbt.qnt.
    project_roll_modifier_buff_substrate(
        RollModifierBuffSubstrate::ArmorClass(ArmorClassBuff {
            target: RollModifierBuffTarget::PrimaryTarget,
            bonus: 2,
        }),
        RollModifierBuffScenarioOutcome::ShieldOfFaith,
    )
}

fn apply_effect_count(
    state: &mut RollModifierBuffSelectedIdentityState,
    target: RollModifierBuffTarget,
) {
    match target {
        RollModifierBuffTarget::Caster => state.caster_effect_count = 1,
        RollModifierBuffTarget::PrimaryTarget => state.primary_target_effect_count = 1,
        RollModifierBuffTarget::PrimaryAndSecondaryTargets => {
            state.primary_target_effect_count = 1;
            state.secondary_target_effect_count = 1;
        }
    }
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
