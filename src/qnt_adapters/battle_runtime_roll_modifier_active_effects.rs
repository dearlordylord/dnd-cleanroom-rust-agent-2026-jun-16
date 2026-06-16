use crate::rules::roll_modifier_active_effects::{
    break_roll_modifier_concentration, cast_bane_failed, cast_bless, cast_enhance_dex,
    cast_enhance_per_target, cast_enthrall, cast_guidance_stealth, cast_pass_without_trace,
    cast_thaumaturgy_booming_voice, cast_thaumaturgy_cancelled, discover_bane_save,
    discover_enhance_ability_choice, discover_enhance_target_ability_choices,
    discover_guidance_skill_choice, discover_thaumaturgy_count,
    stutter_roll_modifier_active_effect, RollModifierActiveEffectsState, RollModifierChoice,
    RollModifierHole, RollModifierProtocol, RollModifierRollMode, RollModifierScenarioOutcome,
    RollModifierSkill, RollModifierValue,
};

pub const BRANCH_ACTIONS: [&str; 16] = [
    "doDiscoverBaneSave",
    "doCastBaneFailed",
    "doCastBless",
    "doDiscoverGuidanceSkillChoice",
    "doCastGuidanceStealth",
    "doCastPassWithoutTrace",
    "doDiscoverEnhanceAbilityChoice",
    "doCastEnhanceDex",
    "doDiscoverEnhanceTargetAbilityChoices",
    "doCastEnhancePerTarget",
    "doCastEnthrall",
    "doDiscoverThaumaturgyCount",
    "doCastThaumaturgyBoomingVoice",
    "doCastThaumaturgyCancelled",
    "doBreakConcentration",
    "doStutter",
];

pub fn replay_observed_action(observed_action_taken: &str) -> RollModifierActiveEffectsState {
    match observed_action_taken {
        "doDiscoverBaneSave" => discover_bane_save(),
        "doCastBaneFailed" => cast_bane_failed(),
        "doCastBless" => cast_bless(),
        "doDiscoverGuidanceSkillChoice" => discover_guidance_skill_choice(),
        "doCastGuidanceStealth" => cast_guidance_stealth(),
        "doCastPassWithoutTrace" => cast_pass_without_trace(),
        "doDiscoverEnhanceAbilityChoice" => discover_enhance_ability_choice(),
        "doCastEnhanceDex" => cast_enhance_dex(),
        "doDiscoverEnhanceTargetAbilityChoices" => discover_enhance_target_ability_choices(),
        "doCastEnhancePerTarget" => cast_enhance_per_target(),
        "doCastEnthrall" => cast_enthrall(),
        "doDiscoverThaumaturgyCount" => discover_thaumaturgy_count(),
        "doCastThaumaturgyBoomingVoice" => cast_thaumaturgy_booming_voice(),
        "doCastThaumaturgyCancelled" => cast_thaumaturgy_cancelled(),
        "doBreakConcentration" => break_roll_modifier_concentration(),
        "doStutter" => stutter_roll_modifier_active_effect(),
        action => panic!("unsupported mbt::actionTaken {action}"),
    }
}

pub fn expected_witness(observed_action_taken: &str) -> RollModifierActiveEffectsState {
    replay_observed_action(observed_action_taken)
}

pub fn projection_payload(state: &RollModifierActiveEffectsState) -> String {
    [
        format!("qActionAvailable={}", state.action_available),
        format!("qSpellAvailable={}", state.spell_available),
        format!("qCasterConcentrating={}", state.caster_concentrating),
        format!(
            "qTargetAttackModifier={}",
            modifier_ref(state.target_attack_modifier)
        ),
        format!(
            "qTargetSavingThrowModifier={}",
            modifier_ref(state.target_saving_throw_modifier)
        ),
        format!(
            "qCasterAbilityCheckModifier={}",
            modifier_ref(state.caster_ability_check_modifier)
        ),
        format!(
            "qTargetAbilityCheckModifier={}",
            modifier_ref(state.target_ability_check_modifier)
        ),
        format!("qCasterSkill={}", skill_ref(state.caster_skill)),
        format!("qTargetSkill={}", skill_ref(state.target_skill)),
        format!(
            "qTargetAbilityChoice={}",
            choice_ref(state.target_ability_choice)
        ),
        format!(
            "qSecondTargetAbilityChoice={}",
            choice_ref(state.second_target_ability_choice)
        ),
        format!(
            "qTargetAbilityCheckRollMode={}",
            roll_mode_ref(state.target_ability_check_roll_mode)
        ),
        format!(
            "qSecondTargetAbilityCheckRollMode={}",
            roll_mode_ref(state.second_target_ability_check_roll_mode)
        ),
        format!(
            "qThaumaturgyIntimidationRollMode={}",
            roll_mode_ref(state.thaumaturgy_intimidation_roll_mode)
        ),
        format!(
            "qThaumaturgyEffectActive={}",
            state.thaumaturgy_effect_active
        ),
        format!("qPassivePerceptionDelta={}", state.passive_perception_delta),
        format!(
            "qScenarioOutcome={}",
            scenario_outcome_ref(state.scenario_outcome)
        ),
        format!("protocolResult={}", protocol_ref(state.protocol)),
        format!("protocolHoles={}", joined_holes(&state.protocol_holes)),
    ]
    .join("\n")
}

fn modifier_ref(value: RollModifierValue) -> &'static str {
    match value {
        RollModifierValue::None => "none",
        RollModifierValue::PlusD4 => "+1d4",
        RollModifierValue::MinusD4 => "-1d4",
        RollModifierValue::Plus10 => "+10",
        RollModifierValue::Minus10 => "-10",
    }
}

fn choice_ref(choice: RollModifierChoice) -> &'static str {
    match choice {
        RollModifierChoice::None => "none",
        RollModifierChoice::Dexterity => "dex",
        RollModifierChoice::Wisdom => "wis",
    }
}

fn skill_ref(skill: RollModifierSkill) -> &'static str {
    match skill {
        RollModifierSkill::None => "none",
        RollModifierSkill::Stealth => "stealth",
        RollModifierSkill::Perception => "perception",
    }
}

fn roll_mode_ref(mode: RollModifierRollMode) -> &'static str {
    match mode {
        RollModifierRollMode::Normal => "normal",
        RollModifierRollMode::Advantage => "advantage",
    }
}

fn scenario_outcome_ref(outcome: RollModifierScenarioOutcome) -> &'static str {
    match outcome {
        RollModifierScenarioOutcome::Init => "Init",
        RollModifierScenarioOutcome::NeedsBaneSave => "NeedsBaneSave",
        RollModifierScenarioOutcome::BaneFailedTarget => "BaneFailedTarget",
        RollModifierScenarioOutcome::BlessTarget => "BlessTarget",
        RollModifierScenarioOutcome::NeedsGuidanceSkill => "NeedsGuidanceSkill",
        RollModifierScenarioOutcome::GuidanceStealth => "GuidanceStealth",
        RollModifierScenarioOutcome::PassWithoutTraceStealth => "PassWithoutTraceStealth",
        RollModifierScenarioOutcome::NeedsEnhanceAbility => "NeedsEnhanceAbility",
        RollModifierScenarioOutcome::EnhanceDex => "EnhanceDex",
        RollModifierScenarioOutcome::NeedsEnhanceTargetAbilities => "NeedsEnhanceTargetAbilities",
        RollModifierScenarioOutcome::EnhancePerTarget => "EnhancePerTarget",
        RollModifierScenarioOutcome::EnthrallPerception => "EnthrallPerception",
        RollModifierScenarioOutcome::NeedsThaumaturgyCount => "NeedsThaumaturgyCount",
        RollModifierScenarioOutcome::ThaumaturgyBoomingVoice => "ThaumaturgyBoomingVoice",
        RollModifierScenarioOutcome::ThaumaturgyCancelled => "ThaumaturgyCancelled",
        RollModifierScenarioOutcome::ConcentrationBroken => "ConcentrationBroken",
    }
}

fn protocol_ref(protocol: RollModifierProtocol) -> &'static str {
    match protocol {
        RollModifierProtocol::Init => "init",
        RollModifierProtocol::NeedsHoles => "needsHoles",
        RollModifierProtocol::Resolved => "resolved",
    }
}

fn joined_holes(holes: &[RollModifierHole]) -> String {
    if holes.is_empty() {
        return "none".to_string();
    }

    holes
        .iter()
        .map(|hole| match hole {
            RollModifierHole::SavingThrowOutcome => "SavingThrowOutcome",
            RollModifierHole::SkillChoice => "SkillChoice",
            RollModifierHole::AbilityChoice => "AbilityChoice",
            RollModifierHole::TargetAbilityChoices => "TargetAbilityChoices",
            RollModifierHole::ThaumaturgyActiveOneMinuteEffectCount => {
                "ThaumaturgyActiveOneMinuteEffectCount"
            }
        })
        .collect::<Vec<_>>()
        .join(",")
}
