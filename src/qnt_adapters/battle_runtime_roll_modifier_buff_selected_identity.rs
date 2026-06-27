use crate::rules::roll_modifier_buff_selected_identity::{
    project_bane_failed_save_penalty, project_bless_attack_and_save_modifier,
    project_guidance_skill_ability_check_modifier, project_resistance_matching_damage_reduction,
    project_shield_of_faith_armor_class_bonus, RollModifierBuffDamageType,
    RollModifierBuffProtocol, RollModifierBuffScenarioOutcome,
    RollModifierBuffSelectedIdentityState, RollModifierBuffSign, RollModifierBuffSkill,
};

pub const BRANCH_ACTIONS: [&str; 5] = [
    "doBlessAttackAndSaveModifier",
    "doBaneFailedSavePenalty",
    "doGuidanceSkillAbilityCheckModifier",
    "doResistanceReducesMatchingDamage",
    "doShieldOfFaithArmorClassBonus",
];

pub fn replay_observed_action(
    observed_action_taken: &str,
) -> RollModifierBuffSelectedIdentityState {
    match observed_action_taken {
        "doBlessAttackAndSaveModifier" => project_bless_attack_and_save_modifier(),
        "doBaneFailedSavePenalty" => project_bane_failed_save_penalty(),
        "doGuidanceSkillAbilityCheckModifier" => project_guidance_skill_ability_check_modifier(),
        "doResistanceReducesMatchingDamage" => project_resistance_matching_damage_reduction(),
        "doShieldOfFaithArmorClassBonus" => project_shield_of_faith_armor_class_bonus(),
        action => panic!("unsupported mbt::actionTaken {action}"),
    }
}

pub fn expected_witness(observed_action_taken: &str) -> RollModifierBuffSelectedIdentityState {
    match observed_action_taken {
        "doBlessAttackAndSaveModifier" => RollModifierBuffSelectedIdentityState {
            caster_concentrating: true,
            primary_target_effect_count: 1,
            secondary_target_effect_count: 1,
            d20_modifier_sign: RollModifierBuffSign::Plus,
            d20_modifier_attack_roll: true,
            d20_modifier_saving_throw: true,
            scenario_outcome: RollModifierBuffScenarioOutcome::Bless,
            protocol: RollModifierBuffProtocol::Resolved,
            ..initial_expected()
        },
        "doBaneFailedSavePenalty" => RollModifierBuffSelectedIdentityState {
            caster_concentrating: true,
            primary_target_effect_count: 1,
            d20_modifier_sign: RollModifierBuffSign::Minus,
            d20_modifier_attack_roll: true,
            d20_modifier_saving_throw: true,
            scenario_outcome: RollModifierBuffScenarioOutcome::Bane,
            protocol: RollModifierBuffProtocol::Resolved,
            ..initial_expected()
        },
        "doGuidanceSkillAbilityCheckModifier" => RollModifierBuffSelectedIdentityState {
            caster_concentrating: true,
            caster_effect_count: 1,
            d20_modifier_sign: RollModifierBuffSign::Plus,
            d20_modifier_ability_check: true,
            d20_modifier_skill: RollModifierBuffSkill::Stealth,
            invalid_target_rejected: true,
            scenario_outcome: RollModifierBuffScenarioOutcome::Guidance,
            protocol: RollModifierBuffProtocol::Resolved,
            ..initial_expected()
        },
        "doResistanceReducesMatchingDamage" => RollModifierBuffSelectedIdentityState {
            caster_concentrating: true,
            caster_effect_count: 1,
            damage_reduction_type: RollModifierBuffDamageType::Bludgeoning,
            damage_reduction_used: true,
            scenario_outcome: RollModifierBuffScenarioOutcome::Resistance,
            protocol: RollModifierBuffProtocol::Resolved,
            ..initial_expected()
        },
        "doShieldOfFaithArmorClassBonus" => RollModifierBuffSelectedIdentityState {
            caster_concentrating: true,
            primary_target_effect_count: 1,
            primary_target_armor_class: 12,
            scenario_outcome: RollModifierBuffScenarioOutcome::ShieldOfFaith,
            protocol: RollModifierBuffProtocol::Resolved,
            ..initial_expected()
        },
        action => panic!("unsupported mbt::actionTaken {action}"),
    }
}

pub fn projection_payload(state: &RollModifierBuffSelectedIdentityState) -> String {
    [
        format!("qCasterConcentrating={}", state.caster_concentrating),
        format!("qCasterHp={}", state.caster_hp),
        format!("qCasterEffectCount={}", state.caster_effect_count),
        format!(
            "qPrimaryTargetEffectCount={}",
            state.primary_target_effect_count
        ),
        format!(
            "qSecondaryTargetEffectCount={}",
            state.secondary_target_effect_count
        ),
        format!(
            "qPrimaryTargetArmorClass={}",
            state.primary_target_armor_class
        ),
        format!("qPrimaryTargetHp={}", state.primary_target_hp),
        format!("qD20ModifierSign={}", sign_ref(state.d20_modifier_sign)),
        format!("qD20ModifierAttackRoll={}", state.d20_modifier_attack_roll),
        format!(
            "qD20ModifierSavingThrow={}",
            state.d20_modifier_saving_throw
        ),
        format!(
            "qD20ModifierAbilityCheck={}",
            state.d20_modifier_ability_check
        ),
        format!("qD20ModifierSkill={}", skill_ref(state.d20_modifier_skill)),
        format!("qInvalidTargetRejected={}", state.invalid_target_rejected),
        format!(
            "qDamageReductionType={}",
            damage_type_ref(state.damage_reduction_type)
        ),
        format!("qDamageReductionUsed={}", state.damage_reduction_used),
        format!(
            "qScenarioOutcome={}",
            scenario_outcome_ref(state.scenario_outcome)
        ),
        format!("protocolResult={}", protocol_ref(state.protocol)),
        "protocolHoles=none".to_string(),
    ]
    .join("\n")
}

fn sign_ref(sign: RollModifierBuffSign) -> &'static str {
    match sign {
        RollModifierBuffSign::None => "none",
        RollModifierBuffSign::Plus => "+",
        RollModifierBuffSign::Minus => "-",
    }
}

fn skill_ref(skill: RollModifierBuffSkill) -> &'static str {
    match skill {
        RollModifierBuffSkill::None => "none",
        RollModifierBuffSkill::Stealth => "stealth",
    }
}

fn damage_type_ref(damage_type: RollModifierBuffDamageType) -> &'static str {
    match damage_type {
        RollModifierBuffDamageType::None => "none",
        RollModifierBuffDamageType::Bludgeoning => "bludgeoning",
    }
}

fn scenario_outcome_ref(outcome: RollModifierBuffScenarioOutcome) -> &'static str {
    match outcome {
        RollModifierBuffScenarioOutcome::Init => "Init",
        RollModifierBuffScenarioOutcome::Bless => "Bless",
        RollModifierBuffScenarioOutcome::Bane => "Bane",
        RollModifierBuffScenarioOutcome::Guidance => "Guidance",
        RollModifierBuffScenarioOutcome::Resistance => "Resistance",
        RollModifierBuffScenarioOutcome::ShieldOfFaith => "ShieldOfFaith",
    }
}

fn protocol_ref(protocol: RollModifierBuffProtocol) -> &'static str {
    match protocol {
        RollModifierBuffProtocol::Init => "init",
        RollModifierBuffProtocol::Resolved => "resolved",
    }
}

fn initial_expected() -> RollModifierBuffSelectedIdentityState {
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
        scenario_outcome: RollModifierBuffScenarioOutcome::Init,
        protocol: RollModifierBuffProtocol::Init,
    }
}
