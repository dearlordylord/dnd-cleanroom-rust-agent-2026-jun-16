use crate::rules::battle_reducer_spine::{BattleGenericRouteFill, BattleSubjectKind};
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

use super::battle_runtime_reducer_route::{
    replay_generic_battle_route, route_discover_battle_acts_from_route_holes,
    route_resolve_battle_subject_from_route_holes,
    route_resolve_battle_subject_without_fill_from_route_holes, route_start_battle,
    GenericBattleRouteStep, ReducerRouteEvent, ReducerRouteFillKind, ReducerRouteHoleKind,
    ReducerRouteOwnerGroup, ReducerRouteSubjectFamily,
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

pub fn replay_observed_route(observed_action_taken: &str) -> Vec<ReducerRouteEvent> {
    let discover = GenericBattleRouteStep::Discover;
    let resolve = |kind, fill| GenericBattleRouteStep::Resolve { kind, fill };

    match observed_action_taken {
        "doDiscoverBaneSave" => replay_generic_battle_route(&[discover(
            BattleSubjectKind::RollModifierEffectSavingThrow,
        )]),
        "doCastBaneFailed" => replay_generic_battle_route(&[
            discover(BattleSubjectKind::RollModifierEffectSavingThrow),
            resolve(
                BattleSubjectKind::RollModifierEffectSavingThrow,
                BattleGenericRouteFill::SavingThrowOutcome,
            ),
            resolve(
                BattleSubjectKind::RollModifierEffectConcentrationCleanup,
                BattleGenericRouteFill::WithoutFill,
            ),
        ]),
        "doCastBless" | "doCastPassWithoutTrace" => direct_concentration_observed_route(),
        "doDiscoverGuidanceSkillChoice" => replay_generic_battle_route(&[discover(
            BattleSubjectKind::RollModifierEffectSkillChoice,
        )]),
        "doCastGuidanceStealth" => replay_generic_battle_route(&[
            discover(BattleSubjectKind::RollModifierEffectSkillChoice),
            resolve(
                BattleSubjectKind::RollModifierEffectSkillChoice,
                BattleGenericRouteFill::SkillChoice,
            ),
            resolve(
                BattleSubjectKind::RollModifierEffectConcentrationCleanup,
                BattleGenericRouteFill::WithoutFill,
            ),
        ]),
        "doDiscoverEnhanceAbilityChoice" => replay_generic_battle_route(&[discover(
            BattleSubjectKind::RollModifierEffectAbilityChoice,
        )]),
        "doCastEnhanceDex" => replay_generic_battle_route(&[
            discover(BattleSubjectKind::RollModifierEffectAbilityChoice),
            resolve(
                BattleSubjectKind::RollModifierEffectAbilityChoice,
                BattleGenericRouteFill::AbilityChoice,
            ),
            resolve(
                BattleSubjectKind::RollModifierEffectConcentrationCleanup,
                BattleGenericRouteFill::WithoutFill,
            ),
        ]),
        "doDiscoverEnhanceTargetAbilityChoices" => replay_generic_battle_route(&[discover(
            BattleSubjectKind::RollModifierEffectTargetAbilityChoices,
        )]),
        "doCastEnhancePerTarget" => replay_generic_battle_route(&[
            discover(BattleSubjectKind::RollModifierEffectTargetAbilityChoices),
            resolve(
                BattleSubjectKind::RollModifierEffectTargetAbilityChoices,
                BattleGenericRouteFill::TargetAbilityChoices,
            ),
            resolve(
                BattleSubjectKind::RollModifierEffectConcentrationCleanup,
                BattleGenericRouteFill::WithoutFill,
            ),
        ]),
        "doCastEnthrall" => replay_generic_battle_route(&[
            discover(BattleSubjectKind::RollModifierEffectSavingThrow),
            resolve(
                BattleSubjectKind::RollModifierEffectSavingThrow,
                BattleGenericRouteFill::SavingThrowOutcome,
            ),
            resolve(
                BattleSubjectKind::RollModifierEffectConcentrationCleanup,
                BattleGenericRouteFill::WithoutFill,
            ),
        ]),
        "doDiscoverThaumaturgyCount" => replay_generic_battle_route(&[discover(
            BattleSubjectKind::RollModifierEffectActiveOneMinuteCount,
        )]),
        "doCastThaumaturgyBoomingVoice" | "doCastThaumaturgyCancelled" => {
            replay_generic_battle_route(&[
                discover(BattleSubjectKind::RollModifierEffectActiveOneMinuteCount),
                resolve(
                    BattleSubjectKind::RollModifierEffectActiveOneMinuteCount,
                    BattleGenericRouteFill::WithoutFill,
                ),
            ])
        }
        "doBreakConcentration" => {
            let mut route = direct_concentration_observed_route();
            route.extend(
                replay_generic_battle_route(&[
                    resolve(
                        BattleSubjectKind::RollModifierEffectConcentrationCleanup,
                        BattleGenericRouteFill::WithoutFill,
                    ),
                    resolve(
                        BattleSubjectKind::RollModifierEffectActiveEffectCleanup,
                        BattleGenericRouteFill::WithoutFill,
                    ),
                ])
                .into_iter()
                .skip(1),
            );
            route
        }
        "doStutter" => direct_concentration_observed_route(),
        action => panic!("unsupported mbt::actionTaken {action}"),
    }
}

pub fn expected_route(observed_action_taken: &str) -> Vec<ReducerRouteEvent> {
    match observed_action_taken {
        "doDiscoverBaneSave" => vec![
            start(),
            discover(
                vec![ReducerRouteHoleKind::SavingThrowOutcome],
                spell_owner(),
            ),
        ],
        "doCastBaneFailed" => saving_throw_modifier_route(),
        "doCastBless" | "doCastPassWithoutTrace" | "doStutter" => {
            direct_concentration_expected_route()
        }
        "doDiscoverGuidanceSkillChoice" => vec![
            start(),
            discover(vec![ReducerRouteHoleKind::SkillChoice], spell_owner()),
        ],
        "doCastGuidanceStealth" => choice_modifier_route(
            ReducerRouteHoleKind::SkillChoice,
            ReducerRouteFillKind::SkillChoice,
        ),
        "doDiscoverEnhanceAbilityChoice" => vec![
            start(),
            discover(vec![ReducerRouteHoleKind::AbilityChoice], spell_owner()),
        ],
        "doCastEnhanceDex" => choice_modifier_route(
            ReducerRouteHoleKind::AbilityChoice,
            ReducerRouteFillKind::AbilityChoice,
        ),
        "doDiscoverEnhanceTargetAbilityChoices" => vec![
            start(),
            discover(
                vec![ReducerRouteHoleKind::TargetAbilityChoices],
                spell_owner(),
            ),
        ],
        "doCastEnhancePerTarget" => choice_modifier_route(
            ReducerRouteHoleKind::TargetAbilityChoices,
            ReducerRouteFillKind::TargetAbilityChoices,
        ),
        "doCastEnthrall" => saving_throw_modifier_route(),
        "doDiscoverThaumaturgyCount" => vec![
            start(),
            discover(Vec::new(), ReducerRouteOwnerGroup::ActiveEffect),
        ],
        "doCastThaumaturgyBoomingVoice" | "doCastThaumaturgyCancelled" => vec![
            start(),
            discover(Vec::new(), ReducerRouteOwnerGroup::ActiveEffect),
            resolve_without_fill(ReducerRouteOwnerGroup::ActiveEffect),
        ],
        "doBreakConcentration" => {
            let mut route = direct_concentration_expected_route();
            route.push(resolve_without_fill(ReducerRouteOwnerGroup::Concentration));
            route.push(resolve_without_fill(ReducerRouteOwnerGroup::ActiveEffect));
            route
        }
        action => panic!("unsupported mbt::actionTaken {action}"),
    }
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

fn direct_concentration_observed_route() -> Vec<ReducerRouteEvent> {
    replay_generic_battle_route(&[
        GenericBattleRouteStep::Discover(BattleSubjectKind::RollModifierEffectNoChoice),
        GenericBattleRouteStep::Resolve {
            kind: BattleSubjectKind::RollModifierEffectNoChoice,
            fill: BattleGenericRouteFill::WithoutFill,
        },
        GenericBattleRouteStep::Resolve {
            kind: BattleSubjectKind::RollModifierEffectConcentrationCleanup,
            fill: BattleGenericRouteFill::WithoutFill,
        },
    ])
}

fn saving_throw_modifier_route() -> Vec<ReducerRouteEvent> {
    vec![
        start(),
        discover(
            vec![ReducerRouteHoleKind::SavingThrowOutcome],
            spell_owner(),
        ),
        resolve(
            ReducerRouteFillKind::SavingThrowOutcome,
            Vec::new(),
            ReducerRouteOwnerGroup::ActiveEffect,
        ),
        resolve_without_fill(ReducerRouteOwnerGroup::Concentration),
    ]
}

fn direct_concentration_expected_route() -> Vec<ReducerRouteEvent> {
    vec![
        start(),
        discover(Vec::new(), spell_owner()),
        resolve_without_fill(ReducerRouteOwnerGroup::ActiveEffect),
        resolve_without_fill(ReducerRouteOwnerGroup::Concentration),
    ]
}

fn choice_modifier_route(
    hole: ReducerRouteHoleKind,
    fill: ReducerRouteFillKind,
) -> Vec<ReducerRouteEvent> {
    vec![
        start(),
        discover(vec![hole], spell_owner()),
        resolve(fill, Vec::new(), ReducerRouteOwnerGroup::ActiveEffect),
        resolve_without_fill(ReducerRouteOwnerGroup::Concentration),
    ]
}

fn start() -> ReducerRouteEvent {
    route_start_battle(ReducerRouteOwnerGroup::ActionEconomy)
}

fn discover(holes: Vec<ReducerRouteHoleKind>, owner: ReducerRouteOwnerGroup) -> ReducerRouteEvent {
    route_discover_battle_acts_from_route_holes(
        ReducerRouteSubjectFamily::RollModifierEffect,
        holes,
        owner,
    )
}

fn resolve(
    fill: ReducerRouteFillKind,
    holes: Vec<ReducerRouteHoleKind>,
    owner: ReducerRouteOwnerGroup,
) -> ReducerRouteEvent {
    route_resolve_battle_subject_from_route_holes(
        ReducerRouteSubjectFamily::RollModifierEffect,
        fill,
        holes,
        owner,
    )
}

fn resolve_without_fill(owner: ReducerRouteOwnerGroup) -> ReducerRouteEvent {
    route_resolve_battle_subject_without_fill_from_route_holes(
        ReducerRouteSubjectFamily::RollModifierEffect,
        Vec::new(),
        owner,
    )
}

fn spell_owner() -> ReducerRouteOwnerGroup {
    ReducerRouteOwnerGroup::SpellSlotAndActionEconomy
}
