use crate::rules::command_options::{
    cleanup_command_approach_no_movement, cleanup_command_flee_no_movement,
    command_next_turn_initial_state, continue_command_approach,
    end_command_approach_within_five_feet, follow_command_drop, follow_command_grovel,
    open_command_flee_opportunity_attack_window, record_command_failed_save_pending,
    reject_command_flee_partial_movement, resolve_command_flee_full_movement,
    suppress_command_halt, CommandNextTurnOption, CommandNextTurnState, CommandTurnActor,
};
use crate::rules::rule_core_components::{
    rule_core_component_route, rule_core_component_route_event_ref, RuleCoreComponentOwner,
    RuleCoreComponentRouteEvent,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RuleCoreSkill {
    Acrobatics,
    AnimalHandling,
    Arcana,
    Athletics,
    Deception,
    History,
    Insight,
    Intimidation,
    Investigation,
    Medicine,
    Nature,
    Perception,
    Performance,
    Persuasion,
    Religion,
    SleightOfHand,
    Stealth,
    Survival,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RuleCoreAbility {
    Dexterity,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AbilitySkillCommandState {
    pub last_scenario: &'static str,
    pub target_hidden: bool,
    pub target_prone: bool,
    pub target_effect_count: i16,
    pub caster_effect_count: i16,
    pub action_available: bool,
    pub bonus_action_available: bool,
    pub movement_spent_feet: i16,
    pub current_actor: &'static str,
    pub pending_command_option: &'static str,
    pub dropped_object_count: i16,
    pub reaction_window_open: bool,
    pub halt_suppressed: bool,
    pub d20_modifier_skill: &'static str,
    pub ability_check_mode_ability: &'static str,
    pub replay_index: i16,
    pub component_route: Vec<RuleCoreComponentRouteEvent>,
}

pub const BRANCH_ACTIONS: [&str; 32] = [
    "doSearchFails",
    "doSearchSucceeds",
    "doGuidanceSkillAcrobatics",
    "doGuidanceSkillAnimalHandling",
    "doGuidanceSkillArcana",
    "doGuidanceSkillAthletics",
    "doGuidanceSkillDeception",
    "doGuidanceSkillHistory",
    "doGuidanceSkillInsight",
    "doGuidanceSkillIntimidation",
    "doGuidanceSkillInvestigation",
    "doGuidanceSkillMedicine",
    "doGuidanceSkillNature",
    "doGuidanceSkillPerception",
    "doGuidanceSkillPerformance",
    "doGuidanceSkillPersuasion",
    "doGuidanceSkillReligion",
    "doGuidanceSkillSleightOfHand",
    "doGuidanceSkillStealth",
    "doGuidanceSkillSurvival",
    "doEnhanceAbilityChoice",
    "doCommandCastGrovel",
    "doCommandFollowGrovel",
    "doCommandFollowDrop",
    "doCommandHaltSuppresses",
    "doCommandFollowApproachContinues",
    "doCommandFollowApproachWithinFive",
    "doCommandFollowApproachNoMovement",
    "doCommandFollowFlee",
    "doCommandFollowFleePartialRejected",
    "doCommandFollowFleeNoMovement",
    "doCommandFollowFleeOpportunityAttack",
];

pub fn replay_observed_action(observed_action_taken: &str) -> AbilitySkillCommandState {
    match observed_action_taken {
        "doSearchFails" => search_projection("search-fails", true, 1),
        "doSearchSucceeds" => search_projection("search-succeeds", false, 2),
        "doGuidanceSkillAcrobatics" => guidance_projection(RuleCoreSkill::Acrobatics, 3),
        "doGuidanceSkillAnimalHandling" => guidance_projection(RuleCoreSkill::AnimalHandling, 4),
        "doGuidanceSkillArcana" => guidance_projection(RuleCoreSkill::Arcana, 5),
        "doGuidanceSkillAthletics" => guidance_projection(RuleCoreSkill::Athletics, 6),
        "doGuidanceSkillDeception" => guidance_projection(RuleCoreSkill::Deception, 7),
        "doGuidanceSkillHistory" => guidance_projection(RuleCoreSkill::History, 8),
        "doGuidanceSkillInsight" => guidance_projection(RuleCoreSkill::Insight, 9),
        "doGuidanceSkillIntimidation" => guidance_projection(RuleCoreSkill::Intimidation, 10),
        "doGuidanceSkillInvestigation" => guidance_projection(RuleCoreSkill::Investigation, 11),
        "doGuidanceSkillMedicine" => guidance_projection(RuleCoreSkill::Medicine, 12),
        "doGuidanceSkillNature" => guidance_projection(RuleCoreSkill::Nature, 13),
        "doGuidanceSkillPerception" => guidance_projection(RuleCoreSkill::Perception, 14),
        "doGuidanceSkillPerformance" => guidance_projection(RuleCoreSkill::Performance, 15),
        "doGuidanceSkillPersuasion" => guidance_projection(RuleCoreSkill::Persuasion, 16),
        "doGuidanceSkillReligion" => guidance_projection(RuleCoreSkill::Religion, 17),
        "doGuidanceSkillSleightOfHand" => guidance_projection(RuleCoreSkill::SleightOfHand, 18),
        "doGuidanceSkillStealth" => guidance_projection(RuleCoreSkill::Stealth, 19),
        "doGuidanceSkillSurvival" => guidance_projection(RuleCoreSkill::Survival, 20),
        "doEnhanceAbilityChoice" => enhance_ability_projection(RuleCoreAbility::Dexterity, 21),
        "doCommandCastGrovel" => command_projection(
            "command-cast-grovel",
            record_command_failed_save_pending(
                command_next_turn_initial_state(),
                CommandNextTurnOption::Grovel,
            ),
            22,
        ),
        "doCommandFollowGrovel" => command_projection(
            "command-follow-grovel",
            follow_command_grovel(failed_save(CommandNextTurnOption::Grovel)),
            23,
        ),
        "doCommandFollowDrop" => command_projection(
            "command-follow-drop",
            follow_command_drop(failed_save(CommandNextTurnOption::Drop), 1),
            24,
        ),
        "doCommandHaltSuppresses" => command_projection(
            "command-halt-suppresses",
            suppress_command_halt(failed_save(CommandNextTurnOption::Halt), 30),
            25,
        ),
        "doCommandFollowApproachContinues" => command_projection(
            "command-follow-approach-continues",
            continue_command_approach(failed_save(CommandNextTurnOption::Approach), 10),
            26,
        ),
        "doCommandFollowApproachWithinFive" => command_projection(
            "command-follow-approach-within-five",
            end_command_approach_within_five_feet(continue_command_approach(
                failed_save(CommandNextTurnOption::Approach),
                10,
            )),
            27,
        ),
        "doCommandFollowApproachNoMovement" => command_projection(
            "command-follow-approach-no-movement",
            cleanup_command_approach_no_movement(failed_save(CommandNextTurnOption::Approach)),
            28,
        ),
        "doCommandFollowFlee" => command_projection(
            "command-follow-flee",
            resolve_command_flee_full_movement(failed_save(CommandNextTurnOption::Flee), 30),
            29,
        ),
        "doCommandFollowFleePartialRejected" => command_projection(
            "command-follow-flee-partial-rejected",
            reject_command_flee_partial_movement(failed_save(CommandNextTurnOption::Flee)),
            30,
        ),
        "doCommandFollowFleeNoMovement" => command_projection(
            "command-follow-flee-no-movement",
            cleanup_command_flee_no_movement(failed_save(CommandNextTurnOption::Flee)),
            31,
        ),
        "doCommandFollowFleeOpportunityAttack" => command_projection(
            "command-follow-flee-opportunity-attack",
            open_command_flee_opportunity_attack_window(failed_save(CommandNextTurnOption::Flee)),
            32,
        ),
        action => panic!("unsupported mbt::actionTaken {action}"),
    }
}

pub fn expected_witness(observed_action_taken: &str) -> AbilitySkillCommandState {
    match observed_action_taken {
        "doSearchFails" => expected_projection(
            "search-fails",
            true,
            false,
            0,
            0,
            false,
            true,
            0,
            "Goblin",
            "none",
            0,
            false,
            false,
            "none",
            "none",
            1,
        ),
        "doSearchSucceeds" => expected_projection(
            "search-succeeds",
            false,
            false,
            0,
            0,
            false,
            true,
            0,
            "Goblin",
            "none",
            0,
            false,
            false,
            "none",
            "none",
            2,
        ),
        "doGuidanceSkillAcrobatics" => {
            expected_guidance("guidance-skill-acrobatics", "acrobatics", 3)
        }
        "doGuidanceSkillAnimalHandling" => {
            expected_guidance("guidance-skill-animal-handling", "animal_handling", 4)
        }
        "doGuidanceSkillArcana" => expected_guidance("guidance-skill-arcana", "arcana", 5),
        "doGuidanceSkillAthletics" => expected_guidance("guidance-skill-athletics", "athletics", 6),
        "doGuidanceSkillDeception" => expected_guidance("guidance-skill-deception", "deception", 7),
        "doGuidanceSkillHistory" => expected_guidance("guidance-skill-history", "history", 8),
        "doGuidanceSkillInsight" => expected_guidance("guidance-skill-insight", "insight", 9),
        "doGuidanceSkillIntimidation" => {
            expected_guidance("guidance-skill-intimidation", "intimidation", 10)
        }
        "doGuidanceSkillInvestigation" => {
            expected_guidance("guidance-skill-investigation", "investigation", 11)
        }
        "doGuidanceSkillMedicine" => expected_guidance("guidance-skill-medicine", "medicine", 12),
        "doGuidanceSkillNature" => expected_guidance("guidance-skill-nature", "nature", 13),
        "doGuidanceSkillPerception" => {
            expected_guidance("guidance-skill-perception", "perception", 14)
        }
        "doGuidanceSkillPerformance" => {
            expected_guidance("guidance-skill-performance", "performance", 15)
        }
        "doGuidanceSkillPersuasion" => {
            expected_guidance("guidance-skill-persuasion", "persuasion", 16)
        }
        "doGuidanceSkillReligion" => expected_guidance("guidance-skill-religion", "religion", 17),
        "doGuidanceSkillSleightOfHand" => {
            expected_guidance("guidance-skill-sleight-of-hand", "sleight_of_hand", 18)
        }
        "doGuidanceSkillStealth" => expected_guidance("guidance-skill-stealth", "stealth", 19),
        "doGuidanceSkillSurvival" => expected_guidance("guidance-skill-survival", "survival", 20),
        "doEnhanceAbilityChoice" => expected_projection(
            "enhance-ability-choice",
            false,
            false,
            1,
            0,
            false,
            true,
            0,
            "Fighter",
            "none",
            0,
            false,
            false,
            "none",
            "dex",
            21,
        ),
        "doCommandCastGrovel" => expected_command(
            "command-cast-grovel",
            false,
            1,
            false,
            true,
            0,
            "Fighter",
            "grovel",
            0,
            false,
            false,
            22,
        ),
        "doCommandFollowGrovel" => expected_command(
            "command-follow-grovel",
            true,
            0,
            true,
            true,
            0,
            "Fighter",
            "none",
            0,
            false,
            false,
            23,
        ),
        "doCommandFollowDrop" => expected_command(
            "command-follow-drop",
            false,
            0,
            true,
            true,
            0,
            "Fighter",
            "none",
            1,
            false,
            false,
            24,
        ),
        "doCommandHaltSuppresses" => expected_command(
            "command-halt-suppresses",
            false,
            1,
            false,
            false,
            30,
            "Goblin",
            "halt",
            0,
            false,
            true,
            25,
        ),
        "doCommandFollowApproachContinues" => expected_command(
            "command-follow-approach-continues",
            false,
            0,
            true,
            true,
            10,
            "Goblin",
            "none",
            0,
            false,
            false,
            26,
        ),
        "doCommandFollowApproachWithinFive" => expected_command(
            "command-follow-approach-within-five",
            false,
            0,
            true,
            true,
            10,
            "Fighter",
            "none",
            0,
            false,
            false,
            27,
        ),
        "doCommandFollowApproachNoMovement" => expected_command(
            "command-follow-approach-no-movement",
            false,
            0,
            true,
            true,
            0,
            "Goblin",
            "none",
            0,
            false,
            false,
            28,
        ),
        "doCommandFollowFlee" => expected_command(
            "command-follow-flee",
            false,
            0,
            true,
            true,
            30,
            "Fighter",
            "none",
            0,
            false,
            false,
            29,
        ),
        "doCommandFollowFleePartialRejected" => expected_command(
            "command-follow-flee-partial-rejected",
            false,
            1,
            true,
            true,
            0,
            "Goblin",
            "flee",
            0,
            false,
            false,
            30,
        ),
        "doCommandFollowFleeNoMovement" => expected_command(
            "command-follow-flee-no-movement",
            false,
            0,
            true,
            true,
            0,
            "Fighter",
            "none",
            0,
            false,
            false,
            31,
        ),
        "doCommandFollowFleeOpportunityAttack" => expected_command(
            "command-follow-flee-opportunity-attack",
            false,
            1,
            true,
            true,
            0,
            "Goblin",
            "flee",
            0,
            true,
            false,
            32,
        ),
        action => panic!("unsupported mbt::actionTaken {action}"),
    }
}

pub fn expected_component_route() -> Vec<RuleCoreComponentRouteEvent> {
    component_route()
}

pub fn projection_payload(state: &AbilitySkillCommandState) -> String {
    [
        format!("qLastScenario={}", state.last_scenario),
        format!("qTargetHidden={}", state.target_hidden),
        format!("qTargetProne={}", state.target_prone),
        format!("qTargetEffectCount={}", state.target_effect_count),
        format!("qCasterEffectCount={}", state.caster_effect_count),
        format!("qActionAvailable={}", state.action_available),
        format!("qBonusActionAvailable={}", state.bonus_action_available),
        format!("qMovementSpentFeet={}", state.movement_spent_feet),
        format!("qCurrentActor={}", state.current_actor),
        format!("qPendingCommandOption={}", state.pending_command_option),
        format!("qDroppedObjectCount={}", state.dropped_object_count),
        format!("qReactionWindowOpen={}", state.reaction_window_open),
        format!("qHaltSuppressed={}", state.halt_suppressed),
        format!("qD20ModifierSkill={}", state.d20_modifier_skill),
        format!(
            "qAbilityCheckModeAbility={}",
            state.ability_check_mode_ability
        ),
        format!("qReplayIndex={}", state.replay_index),
        format!(
            "qComponentRoute={}",
            component_route_payload(&state.component_route)
        ),
    ]
    .join("\n")
}

pub fn component_route_payload(route: &[RuleCoreComponentRouteEvent]) -> String {
    route
        .iter()
        .map(rule_core_component_route_event_ref)
        .collect::<Vec<_>>()
        .join(">")
}

fn expected_guidance(
    scenario: &'static str,
    skill: &'static str,
    replay_index: i16,
) -> AbilitySkillCommandState {
    expected_projection(
        scenario,
        false,
        false,
        1,
        1,
        false,
        true,
        0,
        "Fighter",
        "none",
        0,
        false,
        false,
        skill,
        "none",
        replay_index,
    )
}

#[allow(clippy::too_many_arguments)]
fn expected_command(
    scenario: &'static str,
    target_prone: bool,
    target_effect_count: i16,
    action_available: bool,
    bonus_action_available: bool,
    movement_spent_feet: i16,
    current_actor: &'static str,
    pending_command_option: &'static str,
    dropped_object_count: i16,
    reaction_window_open: bool,
    halt_suppressed: bool,
    replay_index: i16,
) -> AbilitySkillCommandState {
    expected_projection(
        scenario,
        false,
        target_prone,
        target_effect_count,
        0,
        action_available,
        bonus_action_available,
        movement_spent_feet,
        current_actor,
        pending_command_option,
        dropped_object_count,
        reaction_window_open,
        halt_suppressed,
        "none",
        "none",
        replay_index,
    )
}

#[allow(clippy::too_many_arguments)]
fn expected_projection(
    scenario: &'static str,
    target_hidden: bool,
    target_prone: bool,
    target_effect_count: i16,
    caster_effect_count: i16,
    action_available: bool,
    bonus_action_available: bool,
    movement_spent_feet: i16,
    current_actor: &'static str,
    pending_command_option: &'static str,
    dropped_object_count: i16,
    reaction_window_open: bool,
    halt_suppressed: bool,
    d20_modifier_skill: &'static str,
    ability_check_mode_ability: &'static str,
    replay_index: i16,
) -> AbilitySkillCommandState {
    AbilitySkillCommandState {
        last_scenario: scenario,
        target_hidden,
        target_prone,
        target_effect_count,
        caster_effect_count,
        action_available,
        bonus_action_available,
        movement_spent_feet,
        current_actor,
        pending_command_option,
        dropped_object_count,
        reaction_window_open,
        halt_suppressed,
        d20_modifier_skill,
        ability_check_mode_ability,
        replay_index,
        component_route: component_route(),
    }
}

fn search_projection(
    scenario: &'static str,
    target_hidden: bool,
    replay_index: i16,
) -> AbilitySkillCommandState {
    base_projection(scenario, replay_index, |state| {
        state.target_hidden = target_hidden;
        state.action_available = false;
        state.current_actor = "Goblin";
    })
}

fn guidance_projection(skill: RuleCoreSkill, replay_index: i16) -> AbilitySkillCommandState {
    base_projection(skill_guidance_scenario_ref(skill), replay_index, |state| {
        state.target_effect_count = 1;
        state.caster_effect_count = 1;
        state.action_available = false;
        state.d20_modifier_skill = skill_ref(skill);
    })
}

fn enhance_ability_projection(
    ability: RuleCoreAbility,
    replay_index: i16,
) -> AbilitySkillCommandState {
    base_projection("enhance-ability-choice", replay_index, |state| {
        state.target_effect_count = 1;
        state.action_available = false;
        state.ability_check_mode_ability = ability_ref(ability);
    })
}

fn command_projection(
    scenario: &'static str,
    command: CommandNextTurnState,
    replay_index: i16,
) -> AbilitySkillCommandState {
    base_projection(scenario, replay_index, |state| {
        state.target_prone = command.target_prone;
        state.target_effect_count = command.target_effect_count;
        state.action_available = command.action_available;
        state.bonus_action_available = command.bonus_action_available;
        state.movement_spent_feet = command.movement_spent_feet;
        state.current_actor = actor_ref(command.current_actor);
        state.pending_command_option = option_ref(command.pending_option);
        state.dropped_object_count = command.dropped_object_count;
        state.reaction_window_open = command.reaction_window_open;
        state.halt_suppressed = command.halt_suppressed;
    })
}

fn base_projection(
    scenario: &'static str,
    replay_index: i16,
    apply: impl FnOnce(&mut AbilitySkillCommandState),
) -> AbilitySkillCommandState {
    let mut state = AbilitySkillCommandState {
        last_scenario: scenario,
        target_hidden: false,
        target_prone: false,
        target_effect_count: 0,
        caster_effect_count: 0,
        action_available: true,
        bonus_action_available: true,
        movement_spent_feet: 0,
        current_actor: "Fighter",
        pending_command_option: "none",
        dropped_object_count: 0,
        reaction_window_open: false,
        halt_suppressed: false,
        d20_modifier_skill: "none",
        ability_check_mode_ability: "none",
        replay_index,
        component_route: component_route(),
    };
    apply(&mut state);
    state
}

fn failed_save(option: CommandNextTurnOption) -> CommandNextTurnState {
    record_command_failed_save_pending(command_next_turn_initial_state(), option)
}

fn component_route() -> Vec<RuleCoreComponentRouteEvent> {
    rule_core_component_route(RuleCoreComponentOwner::AbilitySkillCommand)
}

fn actor_ref(actor: Option<CommandTurnActor>) -> &'static str {
    match actor {
        Some(CommandTurnActor::Caster) => "Fighter",
        Some(CommandTurnActor::Target) => "Goblin",
        None => "none",
    }
}

fn option_ref(option: Option<CommandNextTurnOption>) -> &'static str {
    match option {
        Some(CommandNextTurnOption::Approach) => "approach",
        Some(CommandNextTurnOption::Drop) => "drop",
        Some(CommandNextTurnOption::Flee) => "flee",
        Some(CommandNextTurnOption::Grovel) => "grovel",
        Some(CommandNextTurnOption::Halt) => "halt",
        None => "none",
    }
}

fn skill_guidance_scenario_ref(skill: RuleCoreSkill) -> &'static str {
    match skill {
        RuleCoreSkill::Acrobatics => "guidance-skill-acrobatics",
        RuleCoreSkill::AnimalHandling => "guidance-skill-animal-handling",
        RuleCoreSkill::Arcana => "guidance-skill-arcana",
        RuleCoreSkill::Athletics => "guidance-skill-athletics",
        RuleCoreSkill::Deception => "guidance-skill-deception",
        RuleCoreSkill::History => "guidance-skill-history",
        RuleCoreSkill::Insight => "guidance-skill-insight",
        RuleCoreSkill::Intimidation => "guidance-skill-intimidation",
        RuleCoreSkill::Investigation => "guidance-skill-investigation",
        RuleCoreSkill::Medicine => "guidance-skill-medicine",
        RuleCoreSkill::Nature => "guidance-skill-nature",
        RuleCoreSkill::Perception => "guidance-skill-perception",
        RuleCoreSkill::Performance => "guidance-skill-performance",
        RuleCoreSkill::Persuasion => "guidance-skill-persuasion",
        RuleCoreSkill::Religion => "guidance-skill-religion",
        RuleCoreSkill::SleightOfHand => "guidance-skill-sleight-of-hand",
        RuleCoreSkill::Stealth => "guidance-skill-stealth",
        RuleCoreSkill::Survival => "guidance-skill-survival",
    }
}

fn skill_ref(skill: RuleCoreSkill) -> &'static str {
    match skill {
        RuleCoreSkill::Acrobatics => "acrobatics",
        RuleCoreSkill::AnimalHandling => "animal_handling",
        RuleCoreSkill::Arcana => "arcana",
        RuleCoreSkill::Athletics => "athletics",
        RuleCoreSkill::Deception => "deception",
        RuleCoreSkill::History => "history",
        RuleCoreSkill::Insight => "insight",
        RuleCoreSkill::Intimidation => "intimidation",
        RuleCoreSkill::Investigation => "investigation",
        RuleCoreSkill::Medicine => "medicine",
        RuleCoreSkill::Nature => "nature",
        RuleCoreSkill::Perception => "perception",
        RuleCoreSkill::Performance => "performance",
        RuleCoreSkill::Persuasion => "persuasion",
        RuleCoreSkill::Religion => "religion",
        RuleCoreSkill::SleightOfHand => "sleight_of_hand",
        RuleCoreSkill::Stealth => "stealth",
        RuleCoreSkill::Survival => "survival",
    }
}

fn ability_ref(ability: RuleCoreAbility) -> &'static str {
    match ability {
        RuleCoreAbility::Dexterity => "dex",
    }
}
