#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AbilityCheckSearchAbility {
    Dexterity,
    Wisdom,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AbilityCheckSearchSkill {
    Athletics,
    Perception,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AbilityCheckSearchRollMode {
    Normal,
    Advantage,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AbilityCheckSearchProtocol {
    Init,
    NeedsHoles,
    Resolved,
    Invalid,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AbilityCheckSearchHole {
    TargetChoice,
    AbilityCheck,
    SkillChoice,
    AbilityChoice,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AbilityCheckSearchStage {
    TargetChoice,
    AbilityCheck,
    Resolved,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RollModifierChoiceKind {
    Skill,
    Ability,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct AbilityCheckSearchState {
    pub stage: AbilityCheckSearchStage,
    pub target_hidden: bool,
    pub target_admitted: bool,
}

impl AbilityCheckSearchState {
    #[must_use]
    pub const fn target_choice_open(target_hidden: bool) -> Self {
        Self {
            stage: AbilityCheckSearchStage::TargetChoice,
            target_hidden,
            target_admitted: false,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RollModifierChoiceState {
    pub choice_kind: RollModifierChoiceKind,
    pub target_selected: bool,
    pub target_effect_count: i16,
    pub caster_effect_count: i16,
    pub d20_modifier_skill: Option<AbilityCheckSearchSkill>,
    pub ability_check_mode_ability: Option<AbilityCheckSearchAbility>,
    pub target_dexterity_roll_mode: AbilityCheckSearchRollMode,
}

impl RollModifierChoiceState {
    #[must_use]
    pub const fn skill_choice_open() -> Self {
        Self {
            choice_kind: RollModifierChoiceKind::Skill,
            target_selected: false,
            target_effect_count: 0,
            caster_effect_count: 0,
            d20_modifier_skill: None,
            ability_check_mode_ability: None,
            target_dexterity_roll_mode: AbilityCheckSearchRollMode::Normal,
        }
    }

    #[must_use]
    pub const fn ability_choice_open() -> Self {
        Self {
            choice_kind: RollModifierChoiceKind::Ability,
            target_selected: false,
            target_effect_count: 0,
            caster_effect_count: 0,
            d20_modifier_skill: None,
            ability_check_mode_ability: None,
            target_dexterity_roll_mode: AbilityCheckSearchRollMode::Normal,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BattleAbilityCheckChoiceSearchState {
    Inactive,
    Search(AbilityCheckSearchState),
    RollModifierChoice(RollModifierChoiceState),
}

impl BattleAbilityCheckChoiceSearchState {
    #[must_use]
    pub const fn inactive() -> Self {
        Self::Inactive
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SearchTargetChoiceFacts {
    pub target_admitted: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SearchAbilityCheckFacts {
    pub total: i16,
    pub difficulty_class: i16,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AbilityCheckChoiceSearchProjection {
    pub protocol: AbilityCheckSearchProtocol,
    pub protocol_holes: Vec<AbilityCheckSearchHole>,
    pub target_hidden: bool,
    pub action_available: bool,
    pub target_effect_count: i16,
    pub caster_effect_count: i16,
    pub d20_modifier_skill: Option<AbilityCheckSearchSkill>,
    pub ability_check_mode_ability: Option<AbilityCheckSearchAbility>,
    pub target_dexterity_roll_mode: AbilityCheckSearchRollMode,
}

#[must_use]
pub fn projection_from_state(
    state: BattleAbilityCheckChoiceSearchState,
    action_available: bool,
) -> AbilityCheckChoiceSearchProjection {
    match state {
        BattleAbilityCheckChoiceSearchState::Inactive => AbilityCheckChoiceSearchProjection {
            protocol: AbilityCheckSearchProtocol::Init,
            protocol_holes: Vec::new(),
            target_hidden: false,
            action_available,
            target_effect_count: 0,
            caster_effect_count: 0,
            d20_modifier_skill: None,
            ability_check_mode_ability: None,
            target_dexterity_roll_mode: AbilityCheckSearchRollMode::Normal,
        },
        BattleAbilityCheckChoiceSearchState::Search(search) => {
            let holes = match search.stage {
                AbilityCheckSearchStage::TargetChoice => {
                    vec![AbilityCheckSearchHole::TargetChoice]
                }
                AbilityCheckSearchStage::AbilityCheck => {
                    vec![AbilityCheckSearchHole::AbilityCheck]
                }
                AbilityCheckSearchStage::Resolved => Vec::new(),
            };
            AbilityCheckChoiceSearchProjection {
                protocol: if holes.is_empty() {
                    AbilityCheckSearchProtocol::Resolved
                } else {
                    AbilityCheckSearchProtocol::NeedsHoles
                },
                protocol_holes: holes,
                target_hidden: search.target_hidden,
                action_available,
                target_effect_count: 0,
                caster_effect_count: 0,
                d20_modifier_skill: None,
                ability_check_mode_ability: None,
                target_dexterity_roll_mode: AbilityCheckSearchRollMode::Normal,
            }
        }
        BattleAbilityCheckChoiceSearchState::RollModifierChoice(choice) => {
            let choice_resolved = choice.target_effect_count > 0
                || choice.d20_modifier_skill.is_some()
                || choice.ability_check_mode_ability.is_some()
                || choice.target_dexterity_roll_mode == AbilityCheckSearchRollMode::Advantage;
            let protocol_holes = if choice_resolved {
                Vec::new()
            } else if choice.target_selected {
                match choice.choice_kind {
                    RollModifierChoiceKind::Skill => vec![AbilityCheckSearchHole::SkillChoice],
                    RollModifierChoiceKind::Ability => vec![AbilityCheckSearchHole::AbilityChoice],
                }
            } else {
                match choice.choice_kind {
                    RollModifierChoiceKind::Skill => vec![
                        AbilityCheckSearchHole::TargetChoice,
                        AbilityCheckSearchHole::SkillChoice,
                    ],
                    RollModifierChoiceKind::Ability => vec![
                        AbilityCheckSearchHole::TargetChoice,
                        AbilityCheckSearchHole::AbilityChoice,
                    ],
                }
            };
            AbilityCheckChoiceSearchProjection {
                protocol: if protocol_holes.is_empty() {
                    AbilityCheckSearchProtocol::Resolved
                } else {
                    AbilityCheckSearchProtocol::NeedsHoles
                },
                protocol_holes,
                target_hidden: false,
                action_available,
                target_effect_count: choice.target_effect_count,
                caster_effect_count: choice.caster_effect_count,
                d20_modifier_skill: choice.d20_modifier_skill,
                ability_check_mode_ability: choice.ability_check_mode_ability,
                target_dexterity_roll_mode: choice.target_dexterity_roll_mode,
            }
        }
    }
}

#[must_use]
pub const fn invalid_projection(
    state: BattleAbilityCheckChoiceSearchState,
    action_available: bool,
) -> AbilityCheckChoiceSearchProjection {
    match state {
        BattleAbilityCheckChoiceSearchState::Search(search) => AbilityCheckChoiceSearchProjection {
            protocol: AbilityCheckSearchProtocol::Invalid,
            protocol_holes: Vec::new(),
            target_hidden: search.target_hidden,
            action_available,
            target_effect_count: 0,
            caster_effect_count: 0,
            d20_modifier_skill: None,
            ability_check_mode_ability: None,
            target_dexterity_roll_mode: AbilityCheckSearchRollMode::Normal,
        },
        BattleAbilityCheckChoiceSearchState::RollModifierChoice(choice) => {
            AbilityCheckChoiceSearchProjection {
                protocol: AbilityCheckSearchProtocol::Invalid,
                protocol_holes: Vec::new(),
                target_hidden: false,
                action_available,
                target_effect_count: choice.target_effect_count,
                caster_effect_count: choice.caster_effect_count,
                d20_modifier_skill: choice.d20_modifier_skill,
                ability_check_mode_ability: choice.ability_check_mode_ability,
                target_dexterity_roll_mode: choice.target_dexterity_roll_mode,
            }
        }
        BattleAbilityCheckChoiceSearchState::Inactive => AbilityCheckChoiceSearchProjection {
            protocol: AbilityCheckSearchProtocol::Invalid,
            protocol_holes: Vec::new(),
            target_hidden: false,
            action_available,
            target_effect_count: 0,
            caster_effect_count: 0,
            d20_modifier_skill: None,
            ability_check_mode_ability: None,
            target_dexterity_roll_mode: AbilityCheckSearchRollMode::Normal,
        },
    }
}
