use crate::rules::battle_reducer_spine::{
    project_bane_failed_save_penalty_observed, project_bless_attack_and_save_modifier_observed,
    project_guidance_skill_ability_check_modifier_observed,
    project_resistance_matching_damage_reduction_observed,
    project_shield_of_faith_armor_class_bonus_observed,
};
use crate::rules::roll_modifier_buff_selected_identity::{
    project_bane_failed_save_penalty, project_bless_attack_and_save_modifier,
    project_guidance_skill_ability_check_modifier, project_resistance_matching_damage_reduction,
    project_shield_of_faith_armor_class_bonus, RollModifierBuffDamageType,
    RollModifierBuffProtocol, RollModifierBuffScenarioOutcome,
    RollModifierBuffSelectedIdentityState, RollModifierBuffSign, RollModifierBuffSkill,
};

use super::battle_runtime_reducer_route::{
    reducer_route_events_from_battle_trace, route_discover_battle_acts_from_route_holes,
    route_resolve_battle_subject_from_route_result,
    route_resolve_battle_subject_without_fill_from_route_result, route_start_battle,
    ReducerRouteEvent, ReducerRouteFillKind, ReducerRouteHoleKind, ReducerRouteOwnerGroup,
    ReducerRouteResolutionOutcome, ReducerRouteSubjectFamily,
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

pub fn replay_observed_route(observed_action_taken: &str) -> Vec<ReducerRouteEvent> {
    let (_state, trace) = match observed_action_taken {
        "doBlessAttackAndSaveModifier" => project_bless_attack_and_save_modifier_observed(),
        "doBaneFailedSavePenalty" => project_bane_failed_save_penalty_observed(),
        "doGuidanceSkillAbilityCheckModifier" => {
            project_guidance_skill_ability_check_modifier_observed()
        }
        "doResistanceReducesMatchingDamage" => {
            project_resistance_matching_damage_reduction_observed()
        }
        "doShieldOfFaithArmorClassBonus" => project_shield_of_faith_armor_class_bonus_observed(),
        action => panic!("unsupported mbt::actionTaken {action}"),
    };
    reducer_route_events_from_battle_trace(&trace)
}

pub fn expected_route(observed_action_taken: &str) -> Vec<ReducerRouteEvent> {
    match observed_action_taken {
        "doBlessAttackAndSaveModifier" => bless_route(),
        "doBaneFailedSavePenalty" => bane_failed_save_route(),
        "doGuidanceSkillAbilityCheckModifier" => guidance_skill_route(),
        "doResistanceReducesMatchingDamage" => resistance_damage_reduction_route(),
        "doShieldOfFaithArmorClassBonus" => shield_of_faith_route(),
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

fn bless_route() -> Vec<ReducerRouteEvent> {
    vec![
        route_start_battle(ReducerRouteOwnerGroup::ActionEconomy),
        discover_roll_modifier(Vec::new()),
        resolve_roll_modifier_without_fill(ReducerRouteOwnerGroup::ActiveEffect),
        resolve_roll_modifier_without_fill(ReducerRouteOwnerGroup::Concentration),
    ]
}

fn bane_failed_save_route() -> Vec<ReducerRouteEvent> {
    vec![
        route_start_battle(ReducerRouteOwnerGroup::ActionEconomy),
        discover_roll_modifier(vec![ReducerRouteHoleKind::SavingThrowOutcome]),
        route_resolve_battle_subject_from_route_result(
            ReducerRouteSubjectFamily::RollModifierEffect,
            ReducerRouteFillKind::SavingThrowOutcome,
            ReducerRouteResolutionOutcome::Resolved,
            Vec::new(),
            ReducerRouteOwnerGroup::ActiveEffect,
        ),
        resolve_roll_modifier_without_fill(ReducerRouteOwnerGroup::Concentration),
    ]
}

fn guidance_skill_route() -> Vec<ReducerRouteEvent> {
    vec![
        route_start_battle(ReducerRouteOwnerGroup::ActionEconomy),
        discover_roll_modifier(vec![ReducerRouteHoleKind::SkillChoice]),
        route_resolve_battle_subject_from_route_result(
            ReducerRouteSubjectFamily::RollModifierEffect,
            ReducerRouteFillKind::SkillChoice,
            ReducerRouteResolutionOutcome::Resolved,
            Vec::new(),
            ReducerRouteOwnerGroup::ActiveEffect,
        ),
        resolve_roll_modifier_without_fill(ReducerRouteOwnerGroup::Concentration),
    ]
}

fn resistance_damage_reduction_route() -> Vec<ReducerRouteEvent> {
    vec![
        route_start_battle(ReducerRouteOwnerGroup::ActionEconomy),
        route_discover_battle_acts_from_route_holes(
            ReducerRouteSubjectFamily::SpellDamageReduction,
            vec![ReducerRouteHoleKind::TargetChoice],
            ReducerRouteOwnerGroup::SpellSlotAndActionEconomy,
        ),
        route_resolve_battle_subject_from_route_result(
            ReducerRouteSubjectFamily::SpellDamageReduction,
            ReducerRouteFillKind::TargetChoice,
            ReducerRouteResolutionOutcome::NeedsHoles,
            vec![ReducerRouteHoleKind::DamageTypeChoice],
            ReducerRouteOwnerGroup::TargetSelection,
        ),
        route_resolve_battle_subject_from_route_result(
            ReducerRouteSubjectFamily::SpellDamageReduction,
            ReducerRouteFillKind::DamageTypeChoice,
            ReducerRouteResolutionOutcome::Resolved,
            Vec::new(),
            ReducerRouteOwnerGroup::ActiveEffect,
        ),
        route_resolve_battle_subject_without_fill_from_route_result(
            ReducerRouteSubjectFamily::SpellDamageReduction,
            ReducerRouteResolutionOutcome::Resolved,
            Vec::new(),
            ReducerRouteOwnerGroup::Concentration,
        ),
        route_discover_battle_acts_from_route_holes(
            ReducerRouteSubjectFamily::SpellDamageReduction,
            vec![ReducerRouteHoleKind::RolledDice],
            ReducerRouteOwnerGroup::DamageAdjustment,
        ),
        route_resolve_battle_subject_from_route_result(
            ReducerRouteSubjectFamily::SpellDamageReduction,
            ReducerRouteFillKind::RolledDice,
            ReducerRouteResolutionOutcome::Resolved,
            Vec::new(),
            ReducerRouteOwnerGroup::DamageAdjustment,
        ),
        route_resolve_battle_subject_without_fill_from_route_result(
            ReducerRouteSubjectFamily::SpellDamageReduction,
            ReducerRouteResolutionOutcome::Resolved,
            Vec::new(),
            ReducerRouteOwnerGroup::ActiveEffect,
        ),
    ]
}

fn shield_of_faith_route() -> Vec<ReducerRouteEvent> {
    vec![
        route_start_battle(ReducerRouteOwnerGroup::ActionEconomy),
        route_discover_battle_acts_from_route_holes(
            ReducerRouteSubjectFamily::ScalarBuffEffect,
            Vec::new(),
            ReducerRouteOwnerGroup::SpellSlotAndActionEconomy,
        ),
        route_resolve_battle_subject_without_fill_from_route_result(
            ReducerRouteSubjectFamily::ScalarBuffEffect,
            ReducerRouteResolutionOutcome::Resolved,
            Vec::new(),
            ReducerRouteOwnerGroup::ActiveEffect,
        ),
        route_resolve_battle_subject_without_fill_from_route_result(
            ReducerRouteSubjectFamily::ScalarBuffEffect,
            ReducerRouteResolutionOutcome::Resolved,
            Vec::new(),
            ReducerRouteOwnerGroup::Concentration,
        ),
    ]
}

fn discover_roll_modifier(holes: Vec<ReducerRouteHoleKind>) -> ReducerRouteEvent {
    route_discover_battle_acts_from_route_holes(
        ReducerRouteSubjectFamily::RollModifierEffect,
        holes,
        ReducerRouteOwnerGroup::SpellSlotAndActionEconomy,
    )
}

fn resolve_roll_modifier_without_fill(owner: ReducerRouteOwnerGroup) -> ReducerRouteEvent {
    route_resolve_battle_subject_without_fill_from_route_result(
        ReducerRouteSubjectFamily::RollModifierEffect,
        ReducerRouteResolutionOutcome::Resolved,
        Vec::new(),
        owner,
    )
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
