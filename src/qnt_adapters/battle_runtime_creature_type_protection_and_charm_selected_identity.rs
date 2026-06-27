use crate::rules::battle_reducer_spine::{BattleGenericRouteFill, BattleSubjectKind};
use crate::rules::creature_type_protection::{
    creature_type_protection_initial_state, discover_animal_friendship_target_admission,
    project_protection_condition_and_possession_prevention, project_protection_scoped_attack_roll,
    resolve_animal_friendship_damage_break, resolve_animal_friendship_failed_save,
    resolve_protection_from_evil_and_good_target, resolve_protection_relevant_charm_save,
    CreatureTypeProtectionProtocol, CreatureTypeProtectionState, ProtectionScenarioOutcome,
};

use super::battle_runtime_reducer_route::{
    replay_generic_battle_route, route_discover_battle_acts_from_route_holes,
    route_resolve_battle_subject_from_route_result,
    route_resolve_battle_subject_without_fill_from_route_result, route_start_battle,
    GenericBattleRouteStep, ReducerRouteEvent, ReducerRouteFillKind, ReducerRouteHoleKind,
    ReducerRouteOwnerGroup, ReducerRouteResolutionOutcome, ReducerRouteSubjectFamily,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CreatureTypeProtectionAndCharmWitness {
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
    pub scenario_outcome: &'static str,
    pub protocol_result: &'static str,
    pub protocol_holes: Vec<&'static str>,
}

pub const BRANCH_ACTIONS: [&str; 7] = [
    "doDiscoverAnimalFriendshipBeastTargetAdmission",
    "doResolveAnimalFriendshipFailedSaveCharmed",
    "doResolveAnimalFriendshipCasterDamageBreak",
    "doResolveProtectionFromEvilAndGoodKnownWillingTargetProtection",
    "doProjectProtectionFromEvilAndGoodScopedAttackDisadvantage",
    "doPreventProtectionFromEvilAndGoodScopedCharmAndPossession",
    "doResolveProtectionFromEvilAndGoodRelevantCharmSaveAdvantage",
];

pub fn replay_observed_action(
    observed_action_taken: &str,
) -> CreatureTypeProtectionAndCharmWitness {
    match observed_action_taken {
        "doDiscoverAnimalFriendshipBeastTargetAdmission" => {
            witness_from_state(discovered_animal_friendship_target())
        }
        "doResolveAnimalFriendshipFailedSaveCharmed" => {
            witness_from_state(animal_friendship_failed_save())
        }
        "doResolveAnimalFriendshipCasterDamageBreak" => {
            witness_from_state(animal_friendship_damage_break())
        }
        "doResolveProtectionFromEvilAndGoodKnownWillingTargetProtection" => {
            witness_from_state(protection_known_willing_target())
        }
        "doProjectProtectionFromEvilAndGoodScopedAttackDisadvantage" => {
            witness_from_state(protection_scoped_attack_projection())
        }
        "doPreventProtectionFromEvilAndGoodScopedCharmAndPossession" => {
            witness_from_state(protection_prevented_charm_and_possession())
        }
        "doResolveProtectionFromEvilAndGoodRelevantCharmSaveAdvantage" => {
            witness_from_state(protection_relevant_charm_save())
        }
        action => panic!("unsupported mbt::actionTaken {action}"),
    }
}

pub fn expected_witness(observed_action_taken: &str) -> CreatureTypeProtectionAndCharmWitness {
    replay_observed_action(observed_action_taken)
}

pub fn replay_observed_route(observed_action_taken: &str) -> Vec<ReducerRouteEvent> {
    replay_generic_battle_route(&observed_route_steps(observed_action_taken))
}

pub fn expected_route(observed_action_taken: &str) -> Vec<ReducerRouteEvent> {
    match observed_action_taken {
        "doDiscoverAnimalFriendshipBeastTargetAdmission" => vec![
            start_route(),
            discover(
                ReducerRouteSubjectFamily::CreatureTypeTargetAdmission,
                vec![ReducerRouteHoleKind::TargetChoice],
                ReducerRouteOwnerGroup::SpellSlotAndActionEconomy,
            ),
        ],
        "doResolveAnimalFriendshipFailedSaveCharmed" => save_charmed_route(),
        "doResolveAnimalFriendshipCasterDamageBreak" => {
            let mut route = save_charmed_route();
            route.extend([
                resolve_without_fill(
                    ReducerRouteSubjectFamily::CharmSourceDamageBreak,
                    Vec::new(),
                    ReducerRouteOwnerGroup::HitPoint,
                ),
                resolve_without_fill(
                    ReducerRouteSubjectFamily::CharmSourceDamageBreak,
                    Vec::new(),
                    ReducerRouteOwnerGroup::ConditionLifecycle,
                ),
                resolve_without_fill(
                    ReducerRouteSubjectFamily::CharmSourceDamageBreak,
                    Vec::new(),
                    ReducerRouteOwnerGroup::ActiveEffect,
                ),
            ]);
            route
        }
        "doResolveProtectionFromEvilAndGoodKnownWillingTargetProtection" => vec![
            start_route(),
            discover(
                ReducerRouteSubjectFamily::ProtectionCharmActiveEffect,
                vec![ReducerRouteHoleKind::TargetChoice],
                ReducerRouteOwnerGroup::SpellSlotAndActionEconomy,
            ),
            resolve(
                ReducerRouteSubjectFamily::ProtectionCharmActiveEffect,
                ReducerRouteFillKind::TargetChoice,
                Vec::new(),
                ReducerRouteOwnerGroup::TargetSelection,
            ),
            resolve_without_fill(
                ReducerRouteSubjectFamily::ProtectionCharmActiveEffect,
                Vec::new(),
                ReducerRouteOwnerGroup::ActiveEffect,
            ),
            resolve_without_fill(
                ReducerRouteSubjectFamily::ProtectionCharmActiveEffect,
                Vec::new(),
                ReducerRouteOwnerGroup::Concentration,
            ),
        ],
        "doProjectProtectionFromEvilAndGoodScopedAttackDisadvantage" => vec![
            start_route(),
            discover(
                ReducerRouteSubjectFamily::ProtectionCharmActiveEffect,
                Vec::new(),
                ReducerRouteOwnerGroup::ActiveEffect,
            ),
            resolve_without_fill(
                ReducerRouteSubjectFamily::ProtectionCharmActiveEffect,
                Vec::new(),
                ReducerRouteOwnerGroup::AttackRoll,
            ),
        ],
        "doPreventProtectionFromEvilAndGoodScopedCharmAndPossession" => vec![
            start_route(),
            discover(
                ReducerRouteSubjectFamily::ProtectionCharmActiveEffect,
                Vec::new(),
                ReducerRouteOwnerGroup::ActiveEffect,
            ),
            resolve_without_fill(
                ReducerRouteSubjectFamily::ProtectionCharmActiveEffect,
                Vec::new(),
                ReducerRouteOwnerGroup::ConditionLifecycle,
            ),
            resolve_without_fill(
                ReducerRouteSubjectFamily::ProtectionCharmActiveEffect,
                Vec::new(),
                ReducerRouteOwnerGroup::CreatureState,
            ),
        ],
        "doResolveProtectionFromEvilAndGoodRelevantCharmSaveAdvantage" => vec![
            start_route(),
            discover(
                ReducerRouteSubjectFamily::ProtectionCharmActiveEffect,
                Vec::new(),
                ReducerRouteOwnerGroup::ActiveEffect,
            ),
            resolve_without_fill(
                ReducerRouteSubjectFamily::ProtectionCharmActiveEffect,
                Vec::new(),
                ReducerRouteOwnerGroup::SavingThrowRollMode,
            ),
        ],
        action => panic!("unsupported route mbt::actionTaken {action}"),
    }
}

pub fn projection_payload(witness: &CreatureTypeProtectionAndCharmWitness) -> String {
    [
        format!("beastTargetAdmitted={}", witness.beast_target_admitted),
        format!(
            "humanoidTargetAdmitted={}",
            witness.humanoid_target_admitted
        ),
        format!(
            "knownWillingProtectionTargetAdmitted={}",
            witness.known_willing_protection_target_admitted
        ),
        format!(
            "plainProtectionTargetRejected={}",
            witness.plain_protection_target_rejected
        ),
        format!(
            "protectionEffectPresent={}",
            witness.protection_effect_present
        ),
        format!(
            "scopedAttackRollDisadvantage={}",
            witness.scoped_attack_roll_disadvantage
        ),
        format!(
            "unscopedAttackRollNormal={}",
            witness.unscoped_attack_roll_normal
        ),
        format!("scopedCharmPrevented={}", witness.scoped_charm_prevented),
        format!("unscopedCharmApplied={}", witness.unscoped_charm_applied),
        format!(
            "scopedPossessionPrevented={}",
            witness.scoped_possession_prevented
        ),
        format!(
            "unscopedPossessionUnprevented={}",
            witness.unscoped_possession_unprevented
        ),
        format!(
            "relevantCharmSaveHasAdvantage={}",
            witness.relevant_charm_save_has_advantage
        ),
        format!(
            "relevantCharmSaveCleared={}",
            witness.relevant_charm_save_cleared
        ),
        format!("targetCharmed={}", witness.target_charmed),
        format!(
            "animalFriendshipEffectPresent={}",
            witness.animal_friendship_effect_present
        ),
        format!("actionAvailable={}", witness.action_available),
        format!(
            "firstLevelSlotsExpended={}",
            witness.first_level_slots_expended
        ),
        format!("scenarioOutcome={}", witness.scenario_outcome),
        format!("protocolResult={}", witness.protocol_result),
        format!("protocolHoles={}", joined_or_none(&witness.protocol_holes)),
    ]
    .join("\n")
}

fn observed_route_steps(observed_action_taken: &str) -> Vec<GenericBattleRouteStep> {
    use BattleGenericRouteFill::{TargetChoice, WithoutFill};
    use BattleSubjectKind::{
        CharmSourceDamageBreakActiveEffect, CharmSourceDamageBreakConditionLifecycle,
        CharmSourceDamageBreakHitPoint, CreatureTypeTargetAdmission, ProtectionCharmActiveEffect,
        ProtectionCharmAttackRollMode, ProtectionCharmConcentration,
        ProtectionCharmConditionLifecycle, ProtectionCharmCreatureState,
        ProtectionCharmSavingThrowRollMode, ProtectionCharmTargetChoice,
    };
    use GenericBattleRouteStep::{Discover, Resolve};

    match observed_action_taken {
        "doDiscoverAnimalFriendshipBeastTargetAdmission" => {
            vec![Discover(CreatureTypeTargetAdmission)]
        }
        "doResolveAnimalFriendshipFailedSaveCharmed" => save_charmed_steps(),
        "doResolveAnimalFriendshipCasterDamageBreak" => {
            let mut steps = save_charmed_steps();
            steps.extend([
                Resolve {
                    kind: CharmSourceDamageBreakHitPoint,
                    fill: WithoutFill,
                },
                Resolve {
                    kind: CharmSourceDamageBreakConditionLifecycle,
                    fill: WithoutFill,
                },
                Resolve {
                    kind: CharmSourceDamageBreakActiveEffect,
                    fill: WithoutFill,
                },
            ]);
            steps
        }
        "doResolveProtectionFromEvilAndGoodKnownWillingTargetProtection" => vec![
            Discover(ProtectionCharmTargetChoice),
            Resolve {
                kind: ProtectionCharmTargetChoice,
                fill: TargetChoice,
            },
            Resolve {
                kind: ProtectionCharmActiveEffect,
                fill: WithoutFill,
            },
            Resolve {
                kind: ProtectionCharmConcentration,
                fill: WithoutFill,
            },
        ],
        "doProjectProtectionFromEvilAndGoodScopedAttackDisadvantage" => vec![
            Discover(ProtectionCharmAttackRollMode),
            Resolve {
                kind: ProtectionCharmAttackRollMode,
                fill: WithoutFill,
            },
        ],
        "doPreventProtectionFromEvilAndGoodScopedCharmAndPossession" => vec![
            Discover(ProtectionCharmConditionLifecycle),
            Resolve {
                kind: ProtectionCharmConditionLifecycle,
                fill: WithoutFill,
            },
            Resolve {
                kind: ProtectionCharmCreatureState,
                fill: WithoutFill,
            },
        ],
        "doResolveProtectionFromEvilAndGoodRelevantCharmSaveAdvantage" => vec![
            Discover(ProtectionCharmSavingThrowRollMode),
            Resolve {
                kind: ProtectionCharmSavingThrowRollMode,
                fill: WithoutFill,
            },
        ],
        action => panic!("unsupported route mbt::actionTaken {action}"),
    }
}

fn save_charmed_steps() -> Vec<GenericBattleRouteStep> {
    use BattleGenericRouteFill::{SavingThrowOutcome, TargetChoice, WithoutFill};
    use BattleSubjectKind::{
        ProtectionCharmActiveEffect, ProtectionCharmConditionLifecycle,
        ProtectionCharmSavingThrowOutcome, ProtectionCharmTargetChoiceThenSave,
    };
    use GenericBattleRouteStep::{Discover, Resolve};

    vec![
        Discover(ProtectionCharmTargetChoiceThenSave),
        Resolve {
            kind: ProtectionCharmTargetChoiceThenSave,
            fill: TargetChoice,
        },
        Resolve {
            kind: ProtectionCharmSavingThrowOutcome,
            fill: SavingThrowOutcome,
        },
        Resolve {
            kind: ProtectionCharmConditionLifecycle,
            fill: WithoutFill,
        },
        Resolve {
            kind: ProtectionCharmActiveEffect,
            fill: WithoutFill,
        },
    ]
}

fn save_charmed_route() -> Vec<ReducerRouteEvent> {
    vec![
        start_route(),
        discover(
            ReducerRouteSubjectFamily::ProtectionCharmActiveEffect,
            vec![
                ReducerRouteHoleKind::TargetChoice,
                ReducerRouteHoleKind::SavingThrowOutcome,
            ],
            ReducerRouteOwnerGroup::SpellSlotAndActionEconomy,
        ),
        resolve(
            ReducerRouteSubjectFamily::ProtectionCharmActiveEffect,
            ReducerRouteFillKind::TargetChoice,
            vec![ReducerRouteHoleKind::SavingThrowOutcome],
            ReducerRouteOwnerGroup::TargetSelection,
        ),
        resolve(
            ReducerRouteSubjectFamily::ProtectionCharmActiveEffect,
            ReducerRouteFillKind::SavingThrowOutcome,
            Vec::new(),
            ReducerRouteOwnerGroup::SavingThrowOutcome,
        ),
        resolve_without_fill(
            ReducerRouteSubjectFamily::ProtectionCharmActiveEffect,
            Vec::new(),
            ReducerRouteOwnerGroup::ConditionLifecycle,
        ),
        resolve_without_fill(
            ReducerRouteSubjectFamily::ProtectionCharmActiveEffect,
            Vec::new(),
            ReducerRouteOwnerGroup::ActiveEffect,
        ),
    ]
}

fn start_route() -> ReducerRouteEvent {
    route_start_battle(ReducerRouteOwnerGroup::ActionEconomy)
}

fn discover(
    subject: ReducerRouteSubjectFamily,
    holes: Vec<ReducerRouteHoleKind>,
    owner: ReducerRouteOwnerGroup,
) -> ReducerRouteEvent {
    route_discover_battle_acts_from_route_holes(subject, holes, owner)
}

fn resolve(
    subject: ReducerRouteSubjectFamily,
    fill: ReducerRouteFillKind,
    holes: Vec<ReducerRouteHoleKind>,
    owner: ReducerRouteOwnerGroup,
) -> ReducerRouteEvent {
    route_resolve_battle_subject_from_route_result(
        subject,
        fill,
        route_outcome(&holes),
        holes,
        owner,
    )
}

fn resolve_without_fill(
    subject: ReducerRouteSubjectFamily,
    holes: Vec<ReducerRouteHoleKind>,
    owner: ReducerRouteOwnerGroup,
) -> ReducerRouteEvent {
    route_resolve_battle_subject_without_fill_from_route_result(
        subject,
        route_outcome(&holes),
        holes,
        owner,
    )
}

fn route_outcome(holes: &[ReducerRouteHoleKind]) -> ReducerRouteResolutionOutcome {
    if holes.is_empty() {
        ReducerRouteResolutionOutcome::Resolved
    } else {
        ReducerRouteResolutionOutcome::NeedsHoles
    }
}

fn discovered_animal_friendship_target() -> CreatureTypeProtectionState {
    discover_animal_friendship_target_admission(creature_type_protection_initial_state())
}

fn animal_friendship_failed_save() -> CreatureTypeProtectionState {
    resolve_animal_friendship_failed_save(creature_type_protection_initial_state())
}

fn animal_friendship_damage_break() -> CreatureTypeProtectionState {
    resolve_animal_friendship_damage_break(creature_type_protection_initial_state())
}

fn protection_known_willing_target() -> CreatureTypeProtectionState {
    resolve_protection_from_evil_and_good_target(creature_type_protection_initial_state())
}

fn protection_scoped_attack_projection() -> CreatureTypeProtectionState {
    project_protection_scoped_attack_roll(creature_type_protection_initial_state())
}

fn protection_prevented_charm_and_possession() -> CreatureTypeProtectionState {
    project_protection_condition_and_possession_prevention(creature_type_protection_initial_state())
}

fn protection_relevant_charm_save() -> CreatureTypeProtectionState {
    resolve_protection_relevant_charm_save(creature_type_protection_initial_state())
}

fn witness_from_state(state: CreatureTypeProtectionState) -> CreatureTypeProtectionAndCharmWitness {
    CreatureTypeProtectionAndCharmWitness {
        beast_target_admitted: state.beast_target_admitted,
        humanoid_target_admitted: state.humanoid_target_admitted,
        known_willing_protection_target_admitted: state.known_willing_protection_target_admitted,
        plain_protection_target_rejected: state.plain_protection_target_rejected,
        protection_effect_present: state.protection_effect_present,
        scoped_attack_roll_disadvantage: state.scoped_attack_roll_disadvantage,
        unscoped_attack_roll_normal: state.unscoped_attack_roll_normal,
        scoped_charm_prevented: state.scoped_charm_prevented,
        unscoped_charm_applied: state.unscoped_charm_applied,
        scoped_possession_prevented: state.scoped_possession_prevented,
        unscoped_possession_unprevented: state.unscoped_possession_unprevented,
        relevant_charm_save_has_advantage: state.relevant_charm_save_has_advantage,
        relevant_charm_save_cleared: state.relevant_charm_save_cleared,
        target_charmed: state.target_charmed,
        animal_friendship_effect_present: state.animal_friendship_effect_present,
        action_available: state.action_available,
        first_level_slots_expended: state.first_level_slots_expended,
        scenario_outcome: scenario_outcome_ref(state.scenario_outcome),
        protocol_result: protocol_result_ref(state.protocol),
        protocol_holes: protocol_holes(state.protocol),
    }
}

fn scenario_outcome_ref(outcome: ProtectionScenarioOutcome) -> &'static str {
    match outcome {
        ProtectionScenarioOutcome::Init => "Init",
        ProtectionScenarioOutcome::Discovered => "Discovered",
        ProtectionScenarioOutcome::Resolved => "Resolved",
        ProtectionScenarioOutcome::DamageBreakResolved => "DamageBreakResolved",
        ProtectionScenarioOutcome::ProtectionResolved => "ProtectionResolved",
        ProtectionScenarioOutcome::ProtectionAttackProjected => "ProtectionAttackProjected",
        ProtectionScenarioOutcome::ProtectionCharmPrevented => "ProtectionCharmPrevented",
        ProtectionScenarioOutcome::ProtectionRelevantSaveResolved => {
            "ProtectionRelevantSaveResolved"
        }
    }
}

fn protocol_result_ref(protocol: CreatureTypeProtectionProtocol) -> &'static str {
    match protocol {
        CreatureTypeProtectionProtocol::Init => "init",
        CreatureTypeProtectionProtocol::Resolved => "resolved",
    }
}

fn protocol_holes(protocol: CreatureTypeProtectionProtocol) -> Vec<&'static str> {
    match protocol {
        CreatureTypeProtectionProtocol::Init => vec!["CreatureTypeProtectionCharm"],
        CreatureTypeProtectionProtocol::Resolved => vec![],
    }
}

fn joined_or_none(values: &[&'static str]) -> String {
    if values.is_empty() {
        "none".to_string()
    } else {
        values.join(",")
    }
}
