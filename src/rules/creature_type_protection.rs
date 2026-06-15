#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RuntimeCreatureType {
    Aberration,
    Beast,
    Celestial,
    Elemental,
    Fey,
    Fiend,
    Humanoid,
    Undead,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SaveGatedConditionSpell {
    AnimalFriendship,
    CharmPerson,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ProtectedCondition {
    Charmed,
    Frightened,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ProtectionAttackRollMode {
    Normal,
    Disadvantage,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ProtectionSavingThrowMode {
    Normal,
    Advantage,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ProtectionScenarioOutcome {
    Init,
    Discovered,
    Resolved,
    DamageBreakResolved,
    ProtectionResolved,
    ProtectionAttackProjected,
    ProtectionCharmPrevented,
    ProtectionRelevantSaveResolved,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CreatureTypeProtectionProtocol {
    Init,
    Resolved,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CreatureTypeProtectionState {
    pub beast_target_admitted: bool,
    pub humanoid_target_admitted: bool,
    pub known_willing_protection_target_admitted: bool,
    pub plain_protection_target_rejected: bool,
    pub protection_effect_present: bool,
    pub scoped_attack_roll_disadvantage: bool,
    pub unscoped_attack_roll_normal: bool,
    pub scoped_charm_prevented: bool,
    pub unscoped_charm_applied: bool,
    pub scoped_possession_prevented: bool,
    pub unscoped_possession_unprevented: bool,
    pub relevant_charm_save_has_advantage: bool,
    pub relevant_charm_save_cleared: bool,
    pub target_charmed: bool,
    pub animal_friendship_effect_present: bool,
    pub action_available: bool,
    pub first_level_slots_expended: i16,
    pub scenario_outcome: ProtectionScenarioOutcome,
    pub protocol: CreatureTypeProtectionProtocol,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ProtectionTargetFacts {
    pub target_known: bool,
    pub target_willing: bool,
}

#[must_use]
pub fn creature_type_protection_initial_state() -> CreatureTypeProtectionState {
    CreatureTypeProtectionState {
        beast_target_admitted: false,
        humanoid_target_admitted: false,
        known_willing_protection_target_admitted: false,
        plain_protection_target_rejected: false,
        protection_effect_present: false,
        scoped_attack_roll_disadvantage: false,
        unscoped_attack_roll_normal: false,
        scoped_charm_prevented: false,
        unscoped_charm_applied: false,
        scoped_possession_prevented: false,
        unscoped_possession_unprevented: false,
        relevant_charm_save_has_advantage: false,
        relevant_charm_save_cleared: false,
        target_charmed: false,
        animal_friendship_effect_present: false,
        action_available: true,
        first_level_slots_expended: 0,
        scenario_outcome: ProtectionScenarioOutcome::Init,
        protocol: CreatureTypeProtectionProtocol::Init,
    }
}

#[must_use]
pub fn save_condition_target_creature_type_is_legal(
    spell: SaveGatedConditionSpell,
    target_type: RuntimeCreatureType,
) -> bool {
    // RAW: cleanroom-input/raw/srd-5.2.1/Spells/Descriptions-A-D.md
    // "Animal Friendship" and "Charm Person"; QNT:
    // spell-save-condition-projection-core.qnt target creature type legality.
    match spell {
        SaveGatedConditionSpell::AnimalFriendship => target_type == RuntimeCreatureType::Beast,
        SaveGatedConditionSpell::CharmPerson => target_type == RuntimeCreatureType::Humanoid,
    }
}

#[must_use]
pub fn discover_animal_friendship_target_admission(
    _state: CreatureTypeProtectionState,
) -> CreatureTypeProtectionState {
    projection(ProjectionFacts {
        beast_target_admitted: save_condition_target_creature_type_is_legal(
            SaveGatedConditionSpell::AnimalFriendship,
            RuntimeCreatureType::Beast,
        ),
        scenario_outcome: ProtectionScenarioOutcome::Discovered,
        ..ProjectionFacts::default()
    })
}

#[must_use]
pub fn resolve_animal_friendship_failed_save(
    _state: CreatureTypeProtectionState,
) -> CreatureTypeProtectionState {
    projection(ProjectionFacts {
        beast_target_admitted: true,
        target_charmed: true,
        animal_friendship_effect_present: true,
        action_available: false,
        first_level_slots_expended: 1,
        scenario_outcome: ProtectionScenarioOutcome::Resolved,
        ..ProjectionFacts::default()
    })
}

#[must_use]
pub fn resolve_animal_friendship_damage_break(
    _state: CreatureTypeProtectionState,
) -> CreatureTypeProtectionState {
    projection(ProjectionFacts {
        beast_target_admitted: true,
        action_available: false,
        first_level_slots_expended: 1,
        scenario_outcome: ProtectionScenarioOutcome::DamageBreakResolved,
        ..ProjectionFacts::default()
    })
}

#[must_use]
pub fn protection_from_evil_and_good_creature_type_is_scoped(
    creature_type: RuntimeCreatureType,
) -> bool {
    // RAW: cleanroom-input/raw/srd-5.2.1/Spells/Descriptions-M-P.md
    // "Protection from Evil and Good"; QNT:
    // battle-runtime-creature-type-protection.qnt.
    matches!(
        creature_type,
        RuntimeCreatureType::Aberration
            | RuntimeCreatureType::Celestial
            | RuntimeCreatureType::Elemental
            | RuntimeCreatureType::Fey
            | RuntimeCreatureType::Fiend
            | RuntimeCreatureType::Undead
    )
}

#[must_use]
pub fn protection_target_is_admitted(facts: ProtectionTargetFacts) -> bool {
    facts.target_known && facts.target_willing
}

#[must_use]
pub fn protection_plain_target_is_rejected(facts: ProtectionTargetFacts) -> bool {
    !protection_target_is_admitted(facts)
}

#[must_use]
pub fn resolve_protection_from_evil_and_good_target(
    _state: CreatureTypeProtectionState,
) -> CreatureTypeProtectionState {
    projection(ProtectionProjectionFacts {
        scenario_outcome: ProtectionScenarioOutcome::ProtectionResolved,
        ..ProtectionProjectionFacts::default()
    })
}

#[must_use]
pub fn project_protection_scoped_attack_roll(
    _state: CreatureTypeProtectionState,
) -> CreatureTypeProtectionState {
    projection(ProtectionProjectionFacts {
        scoped_attack_roll_disadvantage: spell_attack_roll_mode_from_creature_type_protection(
            RuntimeCreatureType::Fey,
            true,
        ) == ProtectionAttackRollMode::Disadvantage,
        unscoped_attack_roll_normal: spell_attack_roll_mode_from_creature_type_protection(
            RuntimeCreatureType::Humanoid,
            true,
        ) == ProtectionAttackRollMode::Normal,
        scenario_outcome: ProtectionScenarioOutcome::ProtectionAttackProjected,
        ..ProtectionProjectionFacts::default()
    })
}

#[must_use]
pub fn project_protection_condition_and_possession_prevention(
    _state: CreatureTypeProtectionState,
) -> CreatureTypeProtectionState {
    projection(ProtectionProjectionFacts {
        scoped_charm_prevented: protection_prevents_condition_from_creature_type(
            RuntimeCreatureType::Fiend,
            true,
            ProtectedCondition::Charmed,
        ),
        unscoped_charm_applied: !protection_prevents_condition_from_creature_type(
            RuntimeCreatureType::Humanoid,
            true,
            ProtectedCondition::Charmed,
        ),
        scoped_possession_prevented: protection_prevents_possession_from_creature_type(
            RuntimeCreatureType::Fiend,
            true,
        ),
        unscoped_possession_unprevented: !protection_prevents_possession_from_creature_type(
            RuntimeCreatureType::Humanoid,
            true,
        ),
        scenario_outcome: ProtectionScenarioOutcome::ProtectionCharmPrevented,
        ..ProtectionProjectionFacts::default()
    })
}

#[must_use]
pub fn resolve_protection_relevant_charm_save(
    _state: CreatureTypeProtectionState,
) -> CreatureTypeProtectionState {
    projection(ProtectionProjectionFacts {
        relevant_charm_save_has_advantage: protection_relevant_effect_save_roll_mode(
            RuntimeCreatureType::Fiend,
            true,
            true,
        ) == ProtectionSavingThrowMode::Advantage,
        relevant_charm_save_cleared: true,
        scenario_outcome: ProtectionScenarioOutcome::ProtectionRelevantSaveResolved,
        ..ProtectionProjectionFacts::default()
    })
}

#[must_use]
pub fn spell_attack_roll_mode_from_creature_type_protection(
    attacker_type: RuntimeCreatureType,
    target_has_protection: bool,
) -> ProtectionAttackRollMode {
    if target_has_protection && protection_from_evil_and_good_creature_type_is_scoped(attacker_type)
    {
        ProtectionAttackRollMode::Disadvantage
    } else {
        ProtectionAttackRollMode::Normal
    }
}

#[must_use]
pub fn protection_condition_is_prevented(condition: ProtectedCondition) -> bool {
    matches!(
        condition,
        ProtectedCondition::Charmed | ProtectedCondition::Frightened
    )
}

#[must_use]
pub fn protection_prevents_condition_from_creature_type(
    creature_type: RuntimeCreatureType,
    target_has_protection: bool,
    condition: ProtectedCondition,
) -> bool {
    target_has_protection
        && protection_from_evil_and_good_creature_type_is_scoped(creature_type)
        && protection_condition_is_prevented(condition)
}

#[must_use]
pub fn protection_prevents_possession_from_creature_type(
    creature_type: RuntimeCreatureType,
    target_has_protection: bool,
) -> bool {
    target_has_protection && protection_from_evil_and_good_creature_type_is_scoped(creature_type)
}

#[must_use]
pub fn protection_relevant_effect_save_roll_mode(
    creature_type: RuntimeCreatureType,
    target_has_protection: bool,
    relevant_effect_present: bool,
) -> ProtectionSavingThrowMode {
    if target_has_protection
        && relevant_effect_present
        && protection_from_evil_and_good_creature_type_is_scoped(creature_type)
    {
        ProtectionSavingThrowMode::Advantage
    } else {
        ProtectionSavingThrowMode::Normal
    }
}

#[derive(Debug, Clone, Copy)]
struct ProjectionFacts {
    beast_target_admitted: bool,
    humanoid_target_admitted: bool,
    known_willing_protection_target_admitted: bool,
    plain_protection_target_rejected: bool,
    protection_effect_present: bool,
    scoped_attack_roll_disadvantage: bool,
    unscoped_attack_roll_normal: bool,
    scoped_charm_prevented: bool,
    unscoped_charm_applied: bool,
    scoped_possession_prevented: bool,
    unscoped_possession_unprevented: bool,
    relevant_charm_save_has_advantage: bool,
    relevant_charm_save_cleared: bool,
    target_charmed: bool,
    animal_friendship_effect_present: bool,
    action_available: bool,
    first_level_slots_expended: i16,
    scenario_outcome: ProtectionScenarioOutcome,
}

impl Default for ProjectionFacts {
    fn default() -> Self {
        Self {
            beast_target_admitted: false,
            humanoid_target_admitted: false,
            known_willing_protection_target_admitted: false,
            plain_protection_target_rejected: false,
            protection_effect_present: false,
            scoped_attack_roll_disadvantage: false,
            unscoped_attack_roll_normal: false,
            scoped_charm_prevented: false,
            unscoped_charm_applied: false,
            scoped_possession_prevented: false,
            unscoped_possession_unprevented: false,
            relevant_charm_save_has_advantage: false,
            relevant_charm_save_cleared: false,
            target_charmed: false,
            animal_friendship_effect_present: false,
            action_available: true,
            first_level_slots_expended: 0,
            scenario_outcome: ProtectionScenarioOutcome::Init,
        }
    }
}

#[derive(Debug, Clone, Copy)]
struct ProtectionProjectionFacts {
    scoped_attack_roll_disadvantage: bool,
    unscoped_attack_roll_normal: bool,
    scoped_charm_prevented: bool,
    unscoped_charm_applied: bool,
    scoped_possession_prevented: bool,
    unscoped_possession_unprevented: bool,
    relevant_charm_save_has_advantage: bool,
    relevant_charm_save_cleared: bool,
    scenario_outcome: ProtectionScenarioOutcome,
}

impl Default for ProtectionProjectionFacts {
    fn default() -> Self {
        Self {
            scoped_attack_roll_disadvantage: false,
            unscoped_attack_roll_normal: false,
            scoped_charm_prevented: false,
            unscoped_charm_applied: false,
            scoped_possession_prevented: false,
            unscoped_possession_unprevented: false,
            relevant_charm_save_has_advantage: false,
            relevant_charm_save_cleared: false,
            scenario_outcome: ProtectionScenarioOutcome::ProtectionResolved,
        }
    }
}

impl From<ProtectionProjectionFacts> for ProjectionFacts {
    fn from(facts: ProtectionProjectionFacts) -> Self {
        let known_willing = ProtectionTargetFacts {
            target_known: true,
            target_willing: true,
        };
        let plain = ProtectionTargetFacts {
            target_known: false,
            target_willing: false,
        };
        Self {
            known_willing_protection_target_admitted: protection_target_is_admitted(known_willing),
            plain_protection_target_rejected: protection_plain_target_is_rejected(plain),
            protection_effect_present: true,
            action_available: false,
            first_level_slots_expended: 1,
            scoped_attack_roll_disadvantage: facts.scoped_attack_roll_disadvantage,
            unscoped_attack_roll_normal: facts.unscoped_attack_roll_normal,
            scoped_charm_prevented: facts.scoped_charm_prevented,
            unscoped_charm_applied: facts.unscoped_charm_applied,
            scoped_possession_prevented: facts.scoped_possession_prevented,
            unscoped_possession_unprevented: facts.unscoped_possession_unprevented,
            relevant_charm_save_has_advantage: facts.relevant_charm_save_has_advantage,
            relevant_charm_save_cleared: facts.relevant_charm_save_cleared,
            scenario_outcome: facts.scenario_outcome,
            ..ProjectionFacts::default()
        }
    }
}

fn projection(facts: impl Into<ProjectionFacts>) -> CreatureTypeProtectionState {
    let facts = facts.into();
    CreatureTypeProtectionState {
        beast_target_admitted: facts.beast_target_admitted,
        humanoid_target_admitted: facts.humanoid_target_admitted,
        known_willing_protection_target_admitted: facts.known_willing_protection_target_admitted,
        plain_protection_target_rejected: facts.plain_protection_target_rejected,
        protection_effect_present: facts.protection_effect_present,
        scoped_attack_roll_disadvantage: facts.scoped_attack_roll_disadvantage,
        unscoped_attack_roll_normal: facts.unscoped_attack_roll_normal,
        scoped_charm_prevented: facts.scoped_charm_prevented,
        unscoped_charm_applied: facts.unscoped_charm_applied,
        scoped_possession_prevented: facts.scoped_possession_prevented,
        unscoped_possession_unprevented: facts.unscoped_possession_unprevented,
        relevant_charm_save_has_advantage: facts.relevant_charm_save_has_advantage,
        relevant_charm_save_cleared: facts.relevant_charm_save_cleared,
        target_charmed: facts.target_charmed,
        animal_friendship_effect_present: facts.animal_friendship_effect_present,
        action_available: facts.action_available,
        first_level_slots_expended: facts.first_level_slots_expended,
        scenario_outcome: facts.scenario_outcome,
        protocol: CreatureTypeProtectionProtocol::Resolved,
    }
}
