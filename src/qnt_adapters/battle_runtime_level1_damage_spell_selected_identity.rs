use crate::rules::battle_reducer_spine::{
    discover_save_gated_damage_subject_observed, discover_spell_attack_damage_subject_observed,
    resolve_battle_subject_observed, start_battle_observed, Actor, AttackRollFacts,
    BattleEntrypointTrace, BattleResolutionRequest, BattleResolutionResult,
    BattleSaveGatedDamageApplication, BattleSaveGatedSpellFill, BattleSaveGatedSpellProcedure,
    BattleSetup, BattleSpellActiveEffectKind, BattleSpellAttackFill, BattleSpellAttackProcedure,
    BattleState,
};

use super::battle_runtime_reducer_route::{
    observed_reducer_route, route_discover_battle_acts, route_resolve_battle_subject,
    route_start_battle, ReducerRouteEvent, ReducerRouteFillKind, ReducerRouteOwnerGroup,
    ReducerRouteSubjectFamily,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LevelOneDamageSpellWitness {
    pub action_available: bool,
    pub spell_slot_spent_this_turn: bool,
    pub level1_slots_remaining: u8,
    pub primary_target_hp: i16,
    pub primary_target_poisoned: bool,
    pub primary_target_next_attack_roll_disadvantage: bool,
    pub secondary_target_hp: i16,
    pub scenario_outcome: &'static str,
    pub protocol_result: &'static str,
    pub protocol_holes: Vec<&'static str>,
}

#[derive(Debug, Clone, Copy)]
struct ExpectedLevelOneDamageSpell {
    action_available: bool,
    spell_slot_spent_this_turn: bool,
    level1_slots_remaining: u8,
    primary_target_hp: i16,
    primary_target_poisoned: bool,
    primary_target_next_attack_roll_disadvantage: bool,
    secondary_target_hp: i16,
    scenario_outcome: &'static str,
}

pub const BRANCH_ACTIONS: [&str; 10] = [
    "doResolveBurningHandsMixedConeSavingThrows",
    "doResolveChromaticOrbDuplicateDamageLeap",
    "doResolveIceKnifeHitAttackDamageAndBurstSavingThrows",
    "doResolveIceKnifeMissBurstSavingThrows",
    "doResolvePoisonSpraySpellAttackDamage",
    "doResolveRayOfSicknessSpellAttackDamageAndPoisoned",
    "doResolveSacredFlameDexteritySavingThrowRadiantDamage",
    "doResolveSorcerousBurstSpellAttackDamage",
    "doResolveStarryWispObjectSpellAttackDamageAndDimLight",
    "doResolveViciousMockeryWisdomSavingThrowPsychicDamageAndNextAttackDisadvantage",
];

pub const ACCEPTED_BRANCH_ACTIONS: [&str; 8] = [
    "doResolveBurningHandsMixedConeSavingThrows",
    "doResolveIceKnifeHitAttackDamageAndBurstSavingThrows",
    "doResolveIceKnifeMissBurstSavingThrows",
    "doResolvePoisonSpraySpellAttackDamage",
    "doResolveRayOfSicknessSpellAttackDamageAndPoisoned",
    "doResolveSacredFlameDexteritySavingThrowRadiantDamage",
    "doResolveSorcerousBurstSpellAttackDamage",
    "doResolveViciousMockeryWisdomSavingThrowPsychicDamageAndNextAttackDisadvantage",
];

pub const BLOCKED_BRANCH_ACTIONS: [(&str, &str); 2] = [
    (
        "doResolveChromaticOrbDuplicateDamageLeap",
        "blocked: chained duplicate-damage leap is modeled in ChromaticOrbSequenceState, not production BattleState",
    ),
    (
        "doResolveStarryWispObjectSpellAttackDamageAndDimLight",
        "blocked: object HP and dim-light emitter facts are modeled in StarryWispObjectState, not production BattleState",
    ),
];

pub fn replay_observed_action(observed_action_taken: &str) -> LevelOneDamageSpellWitness {
    match observed_action_taken {
        action if blocker_reason(action).is_some() => {
            panic!("{}: {}", action, blocker_reason(action).unwrap())
        }
        action => witness_from_battle_state(&replay_observed_state_and_route(action).0),
    }
}

pub fn replay_observed_route(observed_action_taken: &str) -> Vec<ReducerRouteEvent> {
    match observed_action_taken {
        action if blocker_reason(action).is_some() => {
            panic!("{}: {}", action, blocker_reason(action).unwrap())
        }
        action => replay_observed_state_and_route(action).1,
    }
}

fn replay_observed_state_and_route(
    observed_action_taken: &str,
) -> (BattleState, Vec<ReducerRouteEvent>) {
    match observed_action_taken {
        "doResolveBurningHandsMixedConeSavingThrows" => save_gated_route(
            BattleSaveGatedDamageApplication {
                primary_target: Actor::Skeleton,
                primary_damage: 6,
                primary_effect: BattleSpellActiveEffectKind::None,
                secondary_target: Some(Actor::Goblin),
                secondary_damage: 3,
                spend_first_level_slot: true,
            },
            "BurningHandsMixedConeSavingThrows",
        ),
        "doResolveIceKnifeHitAttackDamageAndBurstSavingThrows" => save_gated_route(
            BattleSaveGatedDamageApplication {
                primary_target: Actor::Skeleton,
                primary_damage: 8,
                primary_effect: BattleSpellActiveEffectKind::None,
                secondary_target: None,
                secondary_damage: 0,
                spend_first_level_slot: true,
            },
            "IceKnifeHitAttackDamageAndBurstSavingThrows",
        ),
        "doResolveIceKnifeMissBurstSavingThrows" => save_gated_route(
            BattleSaveGatedDamageApplication {
                primary_target: Actor::Skeleton,
                primary_damage: 4,
                primary_effect: BattleSpellActiveEffectKind::None,
                secondary_target: None,
                secondary_damage: 0,
                spend_first_level_slot: true,
            },
            "IceKnifeMissBurstSavingThrows",
        ),
        "doResolvePoisonSpraySpellAttackDamage" => spell_attack_route(
            false,
            BattleSpellActiveEffectKind::None,
            7,
            "PoisonSpraySpellAttackDamage",
        ),
        "doResolveRayOfSicknessSpellAttackDamageAndPoisoned" => spell_attack_route(
            true,
            BattleSpellActiveEffectKind::Poisoned,
            7,
            "RayOfSicknessSpellAttackDamageAndPoisoned",
        ),
        "doResolveSacredFlameDexteritySavingThrowRadiantDamage" => save_gated_route(
            BattleSaveGatedDamageApplication {
                primary_target: Actor::Skeleton,
                primary_damage: 7,
                primary_effect: BattleSpellActiveEffectKind::None,
                secondary_target: None,
                secondary_damage: 0,
                spend_first_level_slot: false,
            },
            "SacredFlameDexteritySavingThrowRadiantDamage",
        ),
        "doResolveSorcerousBurstSpellAttackDamage" => spell_attack_route(
            false,
            BattleSpellActiveEffectKind::None,
            10,
            "SorcerousBurstSpellAttackDamage",
        ),
        "doResolveViciousMockeryWisdomSavingThrowPsychicDamageAndNextAttackDisadvantage" => {
            save_gated_route(
                BattleSaveGatedDamageApplication {
                    primary_target: Actor::Skeleton,
                    primary_damage: 6,
                    primary_effect: BattleSpellActiveEffectKind::NextAttackRollDisadvantage,
                    secondary_target: None,
                    secondary_damage: 0,
                    spend_first_level_slot: false,
                },
                "ViciousMockeryWisdomSavingThrowPsychicDamageAndNextAttackDisadvantage",
            )
        }
        action => panic!("unsupported mbt::actionTaken {action}"),
    }
}

pub fn expected_witness(observed_action_taken: &str) -> LevelOneDamageSpellWitness {
    match observed_action_taken {
        "doResolveBurningHandsMixedConeSavingThrows" => expected(ExpectedLevelOneDamageSpell {
            action_available: false,
            spell_slot_spent_this_turn: true,
            level1_slots_remaining: 0,
            primary_target_hp: 6,
            primary_target_poisoned: false,
            primary_target_next_attack_roll_disadvantage: false,
            secondary_target_hp: 9,
            scenario_outcome: "BurningHandsMixedConeSavingThrows",
        }),
        "doResolveChromaticOrbDuplicateDamageLeap" => expected(ExpectedLevelOneDamageSpell {
            action_available: false,
            spell_slot_spent_this_turn: true,
            level1_slots_remaining: 0,
            primary_target_hp: 3,
            primary_target_poisoned: false,
            primary_target_next_attack_roll_disadvantage: false,
            secondary_target_hp: 9,
            scenario_outcome: "ChromaticOrbDuplicateDamageLeap",
        }),
        "doResolveIceKnifeHitAttackDamageAndBurstSavingThrows" => {
            expected(ExpectedLevelOneDamageSpell {
                action_available: false,
                spell_slot_spent_this_turn: true,
                level1_slots_remaining: 0,
                primary_target_hp: 4,
                primary_target_poisoned: false,
                primary_target_next_attack_roll_disadvantage: false,
                secondary_target_hp: 12,
                scenario_outcome: "IceKnifeHitAttackDamageAndBurstSavingThrows",
            })
        }
        "doResolveIceKnifeMissBurstSavingThrows" => expected(ExpectedLevelOneDamageSpell {
            action_available: false,
            spell_slot_spent_this_turn: true,
            level1_slots_remaining: 0,
            primary_target_hp: 8,
            primary_target_poisoned: false,
            primary_target_next_attack_roll_disadvantage: false,
            secondary_target_hp: 12,
            scenario_outcome: "IceKnifeMissBurstSavingThrows",
        }),
        "doResolvePoisonSpraySpellAttackDamage" => expected(ExpectedLevelOneDamageSpell {
            action_available: false,
            spell_slot_spent_this_turn: false,
            level1_slots_remaining: 1,
            primary_target_hp: 5,
            primary_target_poisoned: false,
            primary_target_next_attack_roll_disadvantage: false,
            secondary_target_hp: 12,
            scenario_outcome: "PoisonSpraySpellAttackDamage",
        }),
        "doResolveRayOfSicknessSpellAttackDamageAndPoisoned" => {
            expected(ExpectedLevelOneDamageSpell {
                action_available: false,
                spell_slot_spent_this_turn: true,
                level1_slots_remaining: 0,
                primary_target_hp: 5,
                primary_target_poisoned: true,
                primary_target_next_attack_roll_disadvantage: false,
                secondary_target_hp: 12,
                scenario_outcome: "RayOfSicknessSpellAttackDamageAndPoisoned",
            })
        }
        "doResolveSacredFlameDexteritySavingThrowRadiantDamage" => {
            expected(ExpectedLevelOneDamageSpell {
                action_available: false,
                spell_slot_spent_this_turn: false,
                level1_slots_remaining: 1,
                primary_target_hp: 5,
                primary_target_poisoned: false,
                primary_target_next_attack_roll_disadvantage: false,
                secondary_target_hp: 12,
                scenario_outcome: "SacredFlameDexteritySavingThrowRadiantDamage",
            })
        }
        "doResolveSorcerousBurstSpellAttackDamage" => expected(ExpectedLevelOneDamageSpell {
            action_available: false,
            spell_slot_spent_this_turn: false,
            level1_slots_remaining: 1,
            primary_target_hp: 2,
            primary_target_poisoned: false,
            primary_target_next_attack_roll_disadvantage: false,
            secondary_target_hp: 12,
            scenario_outcome: "SorcerousBurstSpellAttackDamage",
        }),
        "doResolveStarryWispObjectSpellAttackDamageAndDimLight" => {
            expected(ExpectedLevelOneDamageSpell {
                action_available: false,
                spell_slot_spent_this_turn: false,
                level1_slots_remaining: 1,
                primary_target_hp: 12,
                primary_target_poisoned: false,
                primary_target_next_attack_roll_disadvantage: false,
                secondary_target_hp: 12,
                scenario_outcome: "StarryWispObjectSpellAttackDamageAndDimLight",
            })
        }
        "doResolveViciousMockeryWisdomSavingThrowPsychicDamageAndNextAttackDisadvantage" => {
            expected(ExpectedLevelOneDamageSpell {
                action_available: false,
                spell_slot_spent_this_turn: false,
                level1_slots_remaining: 1,
                primary_target_hp: 6,
                primary_target_poisoned: false,
                primary_target_next_attack_roll_disadvantage: true,
                secondary_target_hp: 12,
                scenario_outcome:
                    "ViciousMockeryWisdomSavingThrowPsychicDamageAndNextAttackDisadvantage",
            })
        }
        action => panic!("unsupported expected mbt::actionTaken {action}"),
    }
}

pub fn expected_route(observed_action_taken: &str) -> Vec<ReducerRouteEvent> {
    match observed_action_taken {
        "doResolvePoisonSpraySpellAttackDamage"
        | "doResolveRayOfSicknessSpellAttackDamageAndPoisoned"
        | "doResolveSorcerousBurstSpellAttackDamage" => expected_spell_attack_route(),
        action if ACCEPTED_BRANCH_ACTIONS.contains(&action) => expected_save_gated_route(),
        action if blocker_reason(action).is_some() => {
            panic!("{}: {}", action, blocker_reason(action).unwrap())
        }
        action => panic!("unsupported expected route mbt::actionTaken {action}"),
    }
}

#[must_use]
pub fn blocker_reason(observed_action_taken: &str) -> Option<&'static str> {
    BLOCKED_BRANCH_ACTIONS
        .iter()
        .find_map(|(action, reason)| (*action == observed_action_taken).then_some(*reason))
}

pub fn projection_payload(witness: &LevelOneDamageSpellWitness) -> String {
    [
        format!("qActionAvailable={}", witness.action_available),
        format!(
            "qSpellSlotSpentThisTurn={}",
            witness.spell_slot_spent_this_turn
        ),
        format!("qLevel1SlotsRemaining={}", witness.level1_slots_remaining),
        format!("qPrimaryTargetHp={}", witness.primary_target_hp),
        format!("qPrimaryTargetPoisoned={}", witness.primary_target_poisoned),
        format!(
            "qPrimaryTargetNextAttackRollDisadvantage={}",
            witness.primary_target_next_attack_roll_disadvantage
        ),
        format!("qSecondaryTargetHp={}", witness.secondary_target_hp),
        format!("qScenarioOutcome={}", witness.scenario_outcome),
        format!("protocolResult={}", witness.protocol_result),
        format!("protocolHoles={}", joined_or_none(&witness.protocol_holes)),
    ]
    .join("\n")
}

fn spell_attack_route(
    requires_spell_slot: bool,
    effect: BattleSpellActiveEffectKind,
    damage: i16,
    scenario_outcome: &'static str,
) -> (BattleState, Vec<ReducerRouteEvent>) {
    let mut observer = BattleEntrypointTrace::default();
    let state = start_battle_observed(level_one_damage_setup(), &mut observer).state;
    let (state, subject) = discover_spell_attack_damage_subject_observed(
        state,
        requires_spell_slot,
        effect,
        &mut observer,
    );
    let result = resolve_battle_subject_observed(
        state,
        BattleResolutionRequest::spell_attack(
            subject,
            BattleSpellAttackFill::TargetChoice(Actor::Skeleton),
        )
        .expect("spell attack subject should accept target choice"),
        &mut observer,
    );
    let (state, subject) = continuation(result, "target choice");
    let result = resolve_battle_subject_observed(
        state,
        BattleResolutionRequest::spell_attack(
            subject,
            BattleSpellAttackFill::AttackRoll(AttackRollFacts {
                total: 20,
                natural_d20: 15,
            }),
        )
        .expect("spell attack subject should accept attack roll"),
        &mut observer,
    );
    let (state, subject) = continuation(result, "attack roll");
    let result = resolve_battle_subject_observed(
        state,
        BattleResolutionRequest::spell_attack(subject, BattleSpellAttackFill::DamageRoll(damage))
            .expect("spell attack subject should accept damage roll"),
        &mut observer,
    );
    (
        resolved_state(result, scenario_outcome),
        observed_reducer_route(&observer, &[ReducerRouteSubjectFamily::SpellAttack]),
    )
}

fn save_gated_route(
    application: BattleSaveGatedDamageApplication,
    scenario_outcome: &'static str,
) -> (BattleState, Vec<ReducerRouteEvent>) {
    let mut observer = BattleEntrypointTrace::default();
    let state = start_battle_observed(level_one_damage_setup(), &mut observer).state;
    let (state, subject) =
        discover_save_gated_damage_subject_observed(state, application, &mut observer);
    let result = resolve_battle_subject_observed(
        state,
        BattleResolutionRequest::save_gated_spell(
            subject,
            BattleSaveGatedSpellFill::SavingThrowOutcome,
        )
        .expect("save-gated subject should accept saving throw"),
        &mut observer,
    );
    let (state, subject) = continuation(result, "saving throw");
    let result = resolve_battle_subject_observed(
        state,
        BattleResolutionRequest::save_gated_spell(subject, BattleSaveGatedSpellFill::DamageRoll)
            .expect("save-gated subject should accept damage roll"),
        &mut observer,
    );
    (
        resolved_state(result, scenario_outcome),
        observed_reducer_route(&observer, &[ReducerRouteSubjectFamily::SaveGatedSpell]),
    )
}

fn level_one_damage_setup() -> BattleSetup {
    let mut setup = BattleSetup::standard();
    setup.initiative.still_to_act.waiting = vec![Actor::Skeleton, Actor::Goblin];
    setup.skeleton.hp = 12;
    setup.skeleton.max_hp = 12;
    setup.skeleton.armor_class = 10;
    setup.goblin.hp = 12;
    setup.goblin.max_hp = 12;
    setup
}

fn continuation(
    result: BattleResolutionResult,
    context: &str,
) -> (
    BattleState,
    crate::rules::battle_reducer_spine::BattleSubject,
) {
    result
        .into_needs_holes()
        .map(|needs| (needs.state, needs.subject))
        .unwrap_or_else(|| panic!("{context} should continue"))
}

fn resolved_state(result: BattleResolutionResult, scenario_outcome: &'static str) -> BattleState {
    let BattleResolutionResult::Resolved { state } = result else {
        panic!("{scenario_outcome} should resolve")
    };
    state
}

fn witness_from_battle_state(state: &BattleState) -> LevelOneDamageSpellWitness {
    LevelOneDamageSpellWitness {
        action_available: state.action_available,
        spell_slot_spent_this_turn: state.fighter.spell_slots.first_level_expended > 0,
        level1_slots_remaining: (1 - state.fighter.spell_slots.first_level_expended).max(0) as u8,
        primary_target_hp: state.skeleton.hp,
        primary_target_poisoned: state.skeleton.spell_active_effects.poisoned,
        primary_target_next_attack_roll_disadvantage: state
            .skeleton
            .spell_active_effects
            .next_attack_roll_disadvantage,
        secondary_target_hp: state.goblin.hp,
        scenario_outcome: scenario_from_battle_state(state),
        protocol_result: "resolved",
        protocol_holes: Vec::new(),
    }
}

fn expected(fields: ExpectedLevelOneDamageSpell) -> LevelOneDamageSpellWitness {
    LevelOneDamageSpellWitness {
        action_available: fields.action_available,
        spell_slot_spent_this_turn: fields.spell_slot_spent_this_turn,
        level1_slots_remaining: fields.level1_slots_remaining,
        primary_target_hp: fields.primary_target_hp,
        primary_target_poisoned: fields.primary_target_poisoned,
        primary_target_next_attack_roll_disadvantage: fields
            .primary_target_next_attack_roll_disadvantage,
        secondary_target_hp: fields.secondary_target_hp,
        scenario_outcome: fields.scenario_outcome,
        protocol_result: "resolved",
        protocol_holes: Vec::new(),
    }
}

fn scenario_from_battle_state(state: &BattleState) -> &'static str {
    match (
        state.skeleton.hp,
        state.goblin.hp,
        state.fighter.spell_slots.first_level_expended,
        state.skeleton.spell_active_effects.poisoned,
        state.spell_attack_procedure,
        state.save_gated_spell_procedure,
        state
            .skeleton
            .spell_active_effects
            .next_attack_roll_disadvantage,
    ) {
        (
            6,
            9,
            1,
            false,
            BattleSpellAttackProcedure::Inactive,
            BattleSaveGatedSpellProcedure::Active(_),
            false,
        ) => "BurningHandsMixedConeSavingThrows",
        (
            4,
            12,
            1,
            false,
            BattleSpellAttackProcedure::Inactive,
            BattleSaveGatedSpellProcedure::Active(_),
            false,
        ) => "IceKnifeHitAttackDamageAndBurstSavingThrows",
        (
            8,
            12,
            1,
            false,
            BattleSpellAttackProcedure::Inactive,
            BattleSaveGatedSpellProcedure::Active(_),
            false,
        ) => "IceKnifeMissBurstSavingThrows",
        (
            5,
            12,
            0,
            false,
            BattleSpellAttackProcedure::Active(_),
            BattleSaveGatedSpellProcedure::Inactive,
            false,
        ) => "PoisonSpraySpellAttackDamage",
        (
            5,
            12,
            1,
            true,
            BattleSpellAttackProcedure::Active(_),
            BattleSaveGatedSpellProcedure::Inactive,
            false,
        ) => "RayOfSicknessSpellAttackDamageAndPoisoned",
        (
            5,
            12,
            0,
            false,
            BattleSpellAttackProcedure::Inactive,
            BattleSaveGatedSpellProcedure::Active(_),
            false,
        ) => "SacredFlameDexteritySavingThrowRadiantDamage",
        (
            2,
            12,
            0,
            false,
            BattleSpellAttackProcedure::Active(_),
            BattleSaveGatedSpellProcedure::Inactive,
            false,
        ) => "SorcerousBurstSpellAttackDamage",
        (
            6,
            12,
            0,
            false,
            BattleSpellAttackProcedure::Inactive,
            BattleSaveGatedSpellProcedure::Active(_),
            true,
        ) => "ViciousMockeryWisdomSavingThrowPsychicDamageAndNextAttackDisadvantage",
        _ => "Unknown",
    }
}

fn expected_spell_attack_route() -> Vec<ReducerRouteEvent> {
    vec![
        route_start_battle(ReducerRouteOwnerGroup::ActionEconomy),
        route_discover_battle_acts(
            ReducerRouteSubjectFamily::SpellAttack,
            vec![crate::rules::battle_reducer_spine::BattleHoleKind::TargetChoice],
            ReducerRouteOwnerGroup::SpellSlotAndActionEconomy,
        ),
        route_resolve_battle_subject(
            ReducerRouteSubjectFamily::SpellAttack,
            ReducerRouteFillKind::TargetChoice,
            vec![crate::rules::battle_reducer_spine::BattleHoleKind::AttackRoll],
            ReducerRouteOwnerGroup::TargetSelection,
        ),
        route_resolve_battle_subject(
            ReducerRouteSubjectFamily::SpellAttack,
            ReducerRouteFillKind::AttackRoll,
            vec![crate::rules::battle_reducer_spine::BattleHoleKind::RolledDice],
            ReducerRouteOwnerGroup::AttackRoll,
        ),
        route_resolve_battle_subject(
            ReducerRouteSubjectFamily::SpellAttack,
            ReducerRouteFillKind::RolledDice,
            Vec::new(),
            ReducerRouteOwnerGroup::HitPoint,
        ),
    ]
}

fn expected_save_gated_route() -> Vec<ReducerRouteEvent> {
    vec![
        route_start_battle(ReducerRouteOwnerGroup::ActionEconomy),
        route_discover_battle_acts(
            ReducerRouteSubjectFamily::SaveGatedSpell,
            vec![crate::rules::battle_reducer_spine::BattleHoleKind::SavingThrowOutcome],
            ReducerRouteOwnerGroup::SpellSlotAndActionEconomy,
        ),
        route_resolve_battle_subject(
            ReducerRouteSubjectFamily::SaveGatedSpell,
            ReducerRouteFillKind::SavingThrowOutcome,
            vec![crate::rules::battle_reducer_spine::BattleHoleKind::RolledDice],
            ReducerRouteOwnerGroup::HoleFrontier,
        ),
        route_resolve_battle_subject(
            ReducerRouteSubjectFamily::SaveGatedSpell,
            ReducerRouteFillKind::RolledDice,
            Vec::new(),
            ReducerRouteOwnerGroup::HitPoint,
        ),
    ]
}

fn joined_or_none(values: &[&'static str]) -> String {
    if values.is_empty() {
        "none".to_string()
    } else {
        values.join(",")
    }
}
