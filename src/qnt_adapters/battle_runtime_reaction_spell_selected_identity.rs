use crate::rules::battle_reducer_spine::{
    reaction_spell_selected_identity_projection_from_battle, resolve_battle_subject_observed,
    start_battle_observed, Actor, BattleEntrypointTrace, BattleReactionArmorClassInterruptionFacts,
    BattleReactionFailedSaveDamageFacts, BattleReactionSpellFill,
    BattleReactionSpellSelectedIdentityOutcome, BattleReactionSpellSelectedIdentityProjection,
    BattleResolutionRequest, BattleResolutionResult, BattleSetup, BattleSpellSlotLevel,
    BattleState, BattleSubject, BattleSubjectKind,
};

use super::battle_runtime_reducer_route::{
    observed_reducer_route, route_resolve_battle_subject_from_route_result, route_start_battle,
    ReducerRouteEvent, ReducerRouteFillKind, ReducerRouteOwnerGroup, ReducerRouteResolutionOutcome,
    ReducerRouteSubjectFamily,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ReactionSpellSelectedIdentityWitness {
    pub reactor_hp: i16,
    pub trigger_creature_hp: i16,
    pub reactor_armor_class: i16,
    pub reactor_reaction_available: bool,
    pub trigger_creature_first_level_slots_expended: i16,
    pub first_level_slots_expended: i16,
    pub second_level_slots_expended: i16,
    pub third_level_slots_expended: i16,
    pub scenario_outcome: &'static str,
    pub protocol_result: &'static str,
    pub protocol_holes: Vec<&'static str>,
}

pub const BRANCH_ACTIONS: [&str; 2] = [
    "doResolveShieldReactionSpellHit",
    "doResolveHellishRebukeFailedSavingThrow",
];

pub fn replay_observed_action(observed_action_taken: &str) -> ReactionSpellSelectedIdentityWitness {
    let (state, _route, outcome) = replay_observed_state_route_and_outcome(observed_action_taken);
    witness_from_projection(reaction_spell_selected_identity_projection_from_battle(
        &state, outcome,
    ))
}

pub fn expected_witness(observed_action_taken: &str) -> ReactionSpellSelectedIdentityWitness {
    match observed_action_taken {
        "doResolveShieldReactionSpellHit" => ReactionSpellSelectedIdentityWitness {
            reactor_hp: 12,
            trigger_creature_hp: 12,
            reactor_armor_class: 15,
            reactor_reaction_available: false,
            trigger_creature_first_level_slots_expended: 0,
            first_level_slots_expended: 1,
            second_level_slots_expended: 0,
            third_level_slots_expended: 0,
            scenario_outcome: "Resolved",
            protocol_result: "resolved",
            protocol_holes: Vec::new(),
        },
        "doResolveHellishRebukeFailedSavingThrow" => ReactionSpellSelectedIdentityWitness {
            reactor_hp: 11,
            trigger_creature_hp: 9,
            reactor_armor_class: 10,
            reactor_reaction_available: false,
            trigger_creature_first_level_slots_expended: 0,
            first_level_slots_expended: 0,
            second_level_slots_expended: 1,
            third_level_slots_expended: 0,
            scenario_outcome: "Resolved",
            protocol_result: "resolved",
            protocol_holes: Vec::new(),
        },
        action => panic!("unsupported mbt::actionTaken {action}"),
    }
}

pub fn replay_observed_route(observed_action_taken: &str) -> Vec<ReducerRouteEvent> {
    replay_observed_state_route_and_outcome(observed_action_taken).1
}

pub fn expected_route(observed_action_taken: &str) -> Vec<ReducerRouteEvent> {
    match observed_action_taken {
        "doResolveShieldReactionSpellHit" => {
            reaction_spell_route(ReducerRouteOwnerGroup::ActiveEffect)
        }
        "doResolveHellishRebukeFailedSavingThrow" => {
            reaction_spell_route(ReducerRouteOwnerGroup::HitPoint)
        }
        action => panic!("unsupported expected route mbt::actionTaken {action}"),
    }
}

pub fn projection_payload(witness: &ReactionSpellSelectedIdentityWitness) -> String {
    [
        format!("qReactorHp={}", witness.reactor_hp),
        format!("qTriggerCreatureHp={}", witness.trigger_creature_hp),
        format!("qReactorArmorClass={}", witness.reactor_armor_class),
        format!(
            "qReactorReactionAvailable={}",
            witness.reactor_reaction_available
        ),
        format!(
            "qTriggerCreatureFirstLevelSlotsExpended={}",
            witness.trigger_creature_first_level_slots_expended
        ),
        format!(
            "qFirstLevelSlotsExpended={}",
            witness.first_level_slots_expended
        ),
        format!(
            "qSecondLevelSlotsExpended={}",
            witness.second_level_slots_expended
        ),
        format!(
            "qThirdLevelSlotsExpended={}",
            witness.third_level_slots_expended
        ),
        format!("qScenarioOutcome={}", witness.scenario_outcome),
        format!("protocolResult={}", witness.protocol_result),
        format!("protocolHoles={}", joined_or_none(&witness.protocol_holes)),
    ]
    .join("\n")
}

fn replay_observed_state_route_and_outcome(
    observed_action_taken: &str,
) -> (
    BattleState,
    Vec<ReducerRouteEvent>,
    BattleReactionSpellSelectedIdentityOutcome,
) {
    match observed_action_taken {
        "doResolveShieldReactionSpellHit" => reaction_spell_state_and_route(
            BattleReactionSpellFill::ArmorClassInterruption(
                BattleReactionArmorClassInterruptionFacts {
                    reactor: Actor::Fighter,
                    armor_class_bonus: 5,
                    slot_level: BattleSpellSlotLevel::First,
                },
            ),
            BattleReactionSpellSelectedIdentityOutcome::ShieldReactionSpellHitResolved,
        ),
        "doResolveHellishRebukeFailedSavingThrow" => reaction_spell_state_and_route(
            BattleReactionSpellFill::FailedSaveDamage(BattleReactionFailedSaveDamageFacts {
                reactor: Actor::Fighter,
                trigger_creature: Actor::Goblin,
                reactor_damage_taken: 1,
                damage: 3,
                slot_level: BattleSpellSlotLevel::Second,
            }),
            BattleReactionSpellSelectedIdentityOutcome::HellishRebukeFailedSaveResolved,
        ),
        action => panic!("unsupported routed mbt::actionTaken {action}"),
    }
}

fn reaction_spell_state_and_route(
    fill: BattleReactionSpellFill,
    outcome: BattleReactionSpellSelectedIdentityOutcome,
) -> (
    BattleState,
    Vec<ReducerRouteEvent>,
    BattleReactionSpellSelectedIdentityOutcome,
) {
    let mut observer = BattleEntrypointTrace::default();
    let state = start_battle_observed(reaction_spell_setup(), &mut observer).state;
    let subject = reaction_spell_subject();
    let result = resolve_battle_subject_observed(
        state,
        BattleResolutionRequest::reaction_spell(subject, fill)
            .expect("reaction spell subject should accept reaction spell fill"),
        &mut observer,
    );
    (
        resolved_state(result, "reaction spell selected identity"),
        observed_reducer_route(&observer, &[ReducerRouteSubjectFamily::ReactionSpell]),
        outcome,
    )
}

fn reaction_spell_setup() -> BattleSetup {
    let mut setup = BattleSetup::standard();
    setup.goblin.hp = 12;
    setup.goblin.max_hp = 12;
    setup
}

fn reaction_spell_subject() -> BattleSubject {
    BattleSubject {
        kind: BattleSubjectKind::ReactionSpell,
        actor: Actor::Fighter,
        target: None,
        stage: crate::rules::weapon_attack_ordering::WeaponAttackFrontierStage::Resolved,
        damage_modifier: 0,
    }
}

fn resolved_state(result: BattleResolutionResult, context: &str) -> BattleState {
    let BattleResolutionResult::Resolved { state } = result else {
        panic!("{context} should resolve")
    };
    state
}

fn reaction_spell_route(owner: ReducerRouteOwnerGroup) -> Vec<ReducerRouteEvent> {
    vec![
        route_start_battle(ReducerRouteOwnerGroup::ActionEconomy),
        route_resolve_battle_subject_from_route_result(
            ReducerRouteSubjectFamily::ReactionSpell,
            ReducerRouteFillKind::UnitFeatureDecision,
            ReducerRouteResolutionOutcome::Resolved,
            Vec::new(),
            owner,
        ),
    ]
}

fn witness_from_projection(
    projection: BattleReactionSpellSelectedIdentityProjection,
) -> ReactionSpellSelectedIdentityWitness {
    ReactionSpellSelectedIdentityWitness {
        reactor_hp: projection.reactor_hp,
        trigger_creature_hp: projection.trigger_creature_hp,
        reactor_armor_class: projection.reactor_armor_class,
        reactor_reaction_available: projection.reactor_reaction_available,
        trigger_creature_first_level_slots_expended: projection
            .trigger_creature_first_level_slots_expended,
        first_level_slots_expended: projection.first_level_slots_expended,
        second_level_slots_expended: projection.second_level_slots_expended,
        third_level_slots_expended: projection.third_level_slots_expended,
        scenario_outcome: outcome_ref(projection.outcome),
        protocol_result: protocol_result_ref(projection.outcome),
        protocol_holes: Vec::new(),
    }
}

fn outcome_ref(outcome: BattleReactionSpellSelectedIdentityOutcome) -> &'static str {
    match outcome {
        BattleReactionSpellSelectedIdentityOutcome::Init => "Init",
        BattleReactionSpellSelectedIdentityOutcome::ShieldReactionSpellHitResolved
        | BattleReactionSpellSelectedIdentityOutcome::HellishRebukeFailedSaveResolved => "Resolved",
    }
}

fn protocol_result_ref(outcome: BattleReactionSpellSelectedIdentityOutcome) -> &'static str {
    match outcome {
        BattleReactionSpellSelectedIdentityOutcome::Init => "init",
        BattleReactionSpellSelectedIdentityOutcome::ShieldReactionSpellHitResolved
        | BattleReactionSpellSelectedIdentityOutcome::HellishRebukeFailedSaveResolved => "resolved",
    }
}

fn joined_or_none(values: &[&'static str]) -> String {
    if values.is_empty() {
        "none".to_string()
    } else {
        values.join(",")
    }
}
