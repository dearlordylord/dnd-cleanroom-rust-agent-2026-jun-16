#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RollModifierValue {
    None,
    PlusD4,
    MinusD4,
    Plus10,
    Minus10,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RollModifierChoice {
    None,
    Dexterity,
    Wisdom,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RollModifierSkill {
    None,
    Stealth,
    Perception,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RollModifierRollMode {
    Normal,
    Advantage,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RollModifierProtocol {
    Init,
    NeedsHoles,
    Resolved,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RollModifierHole {
    SavingThrowOutcome,
    SkillChoice,
    AbilityChoice,
    TargetAbilityChoices,
    ThaumaturgyActiveOneMinuteEffectCount,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RollModifierScenarioOutcome {
    Init,
    NeedsBaneSave,
    BaneFailedTarget,
    BlessTarget,
    NeedsGuidanceSkill,
    GuidanceStealth,
    PassWithoutTraceStealth,
    NeedsEnhanceAbility,
    EnhanceDex,
    NeedsEnhanceTargetAbilities,
    EnhancePerTarget,
    EnthrallPerception,
    NeedsThaumaturgyCount,
    ThaumaturgyBoomingVoice,
    ThaumaturgyCancelled,
    ConcentrationBroken,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RollModifierActiveEffectsState {
    pub action_available: bool,
    pub spell_available: bool,
    pub caster_concentrating: bool,
    pub target_attack_modifier: RollModifierValue,
    pub target_saving_throw_modifier: RollModifierValue,
    pub caster_ability_check_modifier: RollModifierValue,
    pub target_ability_check_modifier: RollModifierValue,
    pub caster_skill: RollModifierSkill,
    pub target_skill: RollModifierSkill,
    pub target_ability_choice: RollModifierChoice,
    pub second_target_ability_choice: RollModifierChoice,
    pub target_ability_check_roll_mode: RollModifierRollMode,
    pub second_target_ability_check_roll_mode: RollModifierRollMode,
    pub thaumaturgy_intimidation_roll_mode: RollModifierRollMode,
    pub thaumaturgy_effect_active: bool,
    pub passive_perception_delta: i16,
    pub scenario_outcome: RollModifierScenarioOutcome,
    pub protocol: RollModifierProtocol,
    pub protocol_holes: Vec<RollModifierHole>,
}

#[must_use]
pub fn roll_modifier_active_effects_initial_state() -> RollModifierActiveEffectsState {
    no_effect_state(
        true,
        true,
        RollModifierScenarioOutcome::Init,
        RollModifierProtocol::Init,
        Vec::new(),
    )
}

#[must_use]
pub fn discover_bane_save() -> RollModifierActiveEffectsState {
    // RAW: cleanroom-input/raw/srd-5.2.1/Spells/Descriptions-A-D.md "Bane";
    // QNT: battle-runtime-roll-modifier-active-effects.mbt.qnt.
    no_effect_state(
        true,
        true,
        RollModifierScenarioOutcome::NeedsBaneSave,
        RollModifierProtocol::NeedsHoles,
        vec![RollModifierHole::SavingThrowOutcome],
    )
}

#[must_use]
pub fn cast_bane_failed() -> RollModifierActiveEffectsState {
    active_effect_state(RollModifierScenarioOutcome::BaneFailedTarget, |state| {
        state.target_attack_modifier = RollModifierValue::MinusD4;
        state.target_saving_throw_modifier = RollModifierValue::MinusD4;
    })
}

#[must_use]
pub fn cast_bless() -> RollModifierActiveEffectsState {
    // RAW: cleanroom-input/raw/srd-5.2.1/Spells/Descriptions-A-D.md "Bless";
    // QNT: battle-runtime-roll-modifier-active-effects.mbt.qnt.
    active_effect_state(RollModifierScenarioOutcome::BlessTarget, |state| {
        state.target_attack_modifier = RollModifierValue::PlusD4;
        state.target_saving_throw_modifier = RollModifierValue::PlusD4;
    })
}

#[must_use]
pub fn discover_guidance_skill_choice() -> RollModifierActiveEffectsState {
    // RAW: cleanroom-input/raw/srd-5.2.1/Spells/Descriptions-E-L.md
    // "Guidance"; QNT: battle-runtime-roll-modifier-active-effects.mbt.qnt.
    no_effect_state(
        true,
        true,
        RollModifierScenarioOutcome::NeedsGuidanceSkill,
        RollModifierProtocol::NeedsHoles,
        vec![RollModifierHole::SkillChoice],
    )
}

#[must_use]
pub fn cast_guidance_stealth() -> RollModifierActiveEffectsState {
    active_effect_state(RollModifierScenarioOutcome::GuidanceStealth, |state| {
        state.caster_ability_check_modifier = RollModifierValue::PlusD4;
        state.caster_skill = RollModifierSkill::Stealth;
    })
}

#[must_use]
pub fn cast_pass_without_trace() -> RollModifierActiveEffectsState {
    // RAW: cleanroom-input/raw/srd-5.2.1/Spells/Descriptions-M-P.md
    // "Pass without Trace"; QNT: battle-runtime-roll-modifier-active-effects.mbt.qnt.
    active_effect_state(
        RollModifierScenarioOutcome::PassWithoutTraceStealth,
        |state| {
            state.caster_ability_check_modifier = RollModifierValue::Plus10;
            state.target_ability_check_modifier = RollModifierValue::Plus10;
            state.caster_skill = RollModifierSkill::Stealth;
            state.target_skill = RollModifierSkill::Stealth;
        },
    )
}

#[must_use]
pub fn discover_enhance_ability_choice() -> RollModifierActiveEffectsState {
    // RAW: cleanroom-input/raw/srd-5.2.1/Spells/Descriptions-E-L.md
    // "Enhance Ability"; QNT: battle-runtime-roll-modifier-active-effects.mbt.qnt.
    no_effect_state(
        true,
        true,
        RollModifierScenarioOutcome::NeedsEnhanceAbility,
        RollModifierProtocol::NeedsHoles,
        vec![RollModifierHole::AbilityChoice],
    )
}

#[must_use]
pub fn cast_enhance_dex() -> RollModifierActiveEffectsState {
    active_effect_state(RollModifierScenarioOutcome::EnhanceDex, |state| {
        state.target_ability_choice = RollModifierChoice::Dexterity;
        state.target_ability_check_roll_mode = RollModifierRollMode::Advantage;
    })
}

#[must_use]
pub fn discover_enhance_target_ability_choices() -> RollModifierActiveEffectsState {
    no_effect_state(
        true,
        true,
        RollModifierScenarioOutcome::NeedsEnhanceTargetAbilities,
        RollModifierProtocol::NeedsHoles,
        vec![RollModifierHole::TargetAbilityChoices],
    )
}

#[must_use]
pub fn cast_enhance_per_target() -> RollModifierActiveEffectsState {
    active_effect_state(RollModifierScenarioOutcome::EnhancePerTarget, |state| {
        state.target_ability_choice = RollModifierChoice::Dexterity;
        state.second_target_ability_choice = RollModifierChoice::Wisdom;
        state.target_ability_check_roll_mode = RollModifierRollMode::Advantage;
        state.second_target_ability_check_roll_mode = RollModifierRollMode::Advantage;
    })
}

#[must_use]
pub fn cast_enthrall() -> RollModifierActiveEffectsState {
    // RAW: cleanroom-input/raw/srd-5.2.1/Spells/Descriptions-E-L.md
    // "Enthrall"; QNT: battle-runtime-roll-modifier-active-effects.mbt.qnt.
    active_effect_state(RollModifierScenarioOutcome::EnthrallPerception, |state| {
        state.target_ability_check_modifier = RollModifierValue::Minus10;
        state.target_skill = RollModifierSkill::Perception;
        state.passive_perception_delta = -10;
    })
}

#[must_use]
pub fn discover_thaumaturgy_count() -> RollModifierActiveEffectsState {
    // RAW: cleanroom-input/raw/srd-5.2.1/Spells/Descriptions-S-Z.md
    // "Thaumaturgy"; QNT: battle-runtime-roll-modifier-active-effects.mbt.qnt.
    no_effect_state(
        true,
        true,
        RollModifierScenarioOutcome::NeedsThaumaturgyCount,
        RollModifierProtocol::NeedsHoles,
        vec![RollModifierHole::ThaumaturgyActiveOneMinuteEffectCount],
    )
}

#[must_use]
pub fn cast_thaumaturgy_booming_voice() -> RollModifierActiveEffectsState {
    let mut state = no_effect_state(
        false,
        false,
        RollModifierScenarioOutcome::ThaumaturgyBoomingVoice,
        RollModifierProtocol::Resolved,
        Vec::new(),
    );
    state.thaumaturgy_intimidation_roll_mode = RollModifierRollMode::Advantage;
    state.thaumaturgy_effect_active = true;
    state
}

#[must_use]
pub fn cast_thaumaturgy_cancelled() -> RollModifierActiveEffectsState {
    let mut state = no_effect_state(
        false,
        false,
        RollModifierScenarioOutcome::ThaumaturgyCancelled,
        RollModifierProtocol::Resolved,
        Vec::new(),
    );
    state.thaumaturgy_effect_active = true;
    state
}

#[must_use]
pub fn break_roll_modifier_concentration() -> RollModifierActiveEffectsState {
    no_effect_state(
        false,
        false,
        RollModifierScenarioOutcome::ConcentrationBroken,
        RollModifierProtocol::Resolved,
        Vec::new(),
    )
}

#[must_use]
pub fn stutter_roll_modifier_active_effect() -> RollModifierActiveEffectsState {
    cast_bless()
}

fn active_effect_state(
    scenario_outcome: RollModifierScenarioOutcome,
    apply: impl FnOnce(&mut RollModifierActiveEffectsState),
) -> RollModifierActiveEffectsState {
    let mut state = no_effect_state(
        false,
        false,
        scenario_outcome,
        RollModifierProtocol::Resolved,
        Vec::new(),
    );
    state.caster_concentrating = true;
    apply(&mut state);
    state
}

fn no_effect_state(
    action_available: bool,
    spell_available: bool,
    scenario_outcome: RollModifierScenarioOutcome,
    protocol: RollModifierProtocol,
    protocol_holes: Vec<RollModifierHole>,
) -> RollModifierActiveEffectsState {
    RollModifierActiveEffectsState {
        action_available,
        spell_available,
        caster_concentrating: false,
        target_attack_modifier: RollModifierValue::None,
        target_saving_throw_modifier: RollModifierValue::None,
        caster_ability_check_modifier: RollModifierValue::None,
        target_ability_check_modifier: RollModifierValue::None,
        caster_skill: RollModifierSkill::None,
        target_skill: RollModifierSkill::None,
        target_ability_choice: RollModifierChoice::None,
        second_target_ability_choice: RollModifierChoice::None,
        target_ability_check_roll_mode: RollModifierRollMode::Normal,
        second_target_ability_check_roll_mode: RollModifierRollMode::Normal,
        thaumaturgy_intimidation_roll_mode: RollModifierRollMode::Normal,
        thaumaturgy_effect_active: false,
        passive_perception_delta: 0,
        scenario_outcome,
        protocol,
        protocol_holes,
    }
}
