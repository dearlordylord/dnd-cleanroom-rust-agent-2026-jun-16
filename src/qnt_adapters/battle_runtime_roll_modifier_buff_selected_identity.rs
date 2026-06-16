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
    replay_observed_action(observed_action_taken)
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
