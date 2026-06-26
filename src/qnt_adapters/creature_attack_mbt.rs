use crate::rules::battle_reducer_spine::{
    discover_battle_acts, resolve_battle_subject, start_battle, Actor, AttackRollFacts,
    BattleHoleKind, BattleResolutionRequest, BattleSetup, BattleState, BattleSubject,
    BattleSubjectKind, BattleWeaponAttackFill, Combatant, CombatantLifecycle, Initiative,
    InitiativeStillToAct,
};
use crate::rules::creature_attack::{
    creature_attack_initial_state, resolve_creature_attack, CreatureAttackActor,
    CreatureAttackFills, CreatureAttackState, CREATURE_ATTACK_MAX_DAMAGE_ROLL,
};
use crate::rules::creature_size::CreatureSize;

use super::battle_runtime_reducer_route::{
    battle_resolution_continuation, route_discover_battle_acts,
    route_resolve_battle_subject_from_result, route_start_battle, ReducerRouteEvent,
    ReducerRouteFillKind, ReducerRouteOwnerGroup, ReducerRouteResolveConnector,
    ReducerRouteResolveFill, ReducerRouteSubjectFamily,
};

pub const BRANCH_ACTIONS: [&str; 2] = ["doAttackerAAttacks", "doAttackerBAttacks"];
pub const REPLAY_DAMAGE_SAMPLE: i16 = CREATURE_ATTACK_MAX_DAMAGE_ROLL;
pub const REPLAY_HIT_SAMPLE: bool = true;

pub fn replay_observed_action(observed_action_taken: &str) -> CreatureAttackState {
    match observed_action_taken {
        "doAttackerAAttacks" => replay_attack_through_battle(CreatureAttackActor::CreatureA),
        "doAttackerBAttacks" => replay_attack_through_battle(CreatureAttackActor::CreatureB),
        action => panic!("unsupported mbt::actionTaken {action}"),
    }
}

pub fn expected_witness(observed_action_taken: &str) -> CreatureAttackState {
    match observed_action_taken {
        "doAttackerAAttacks" => replay_attack(CreatureAttackActor::CreatureA),
        "doAttackerBAttacks" => replay_attack(CreatureAttackActor::CreatureB),
        action => panic!("unsupported mbt::actionTaken {action}"),
    }
}

pub fn replay_observed_route(observed_action_taken: &str) -> Vec<ReducerRouteEvent> {
    match observed_action_taken {
        "doAttackerAAttacks" => creature_attack_route_for_hit(CreatureAttackActor::CreatureA),
        "doAttackerBAttacks" => creature_attack_route_for_hit(CreatureAttackActor::CreatureB),
        action => panic!("unsupported mbt::actionTaken {action}"),
    }
}

pub fn expected_route(observed_action_taken: &str) -> Vec<ReducerRouteEvent> {
    match observed_action_taken {
        "doAttackerAAttacks" | "doAttackerBAttacks" => expected_creature_attack_route(),
        action => panic!("unsupported mbt::actionTaken {action}"),
    }
}

fn expected_creature_attack_route() -> Vec<ReducerRouteEvent> {
    let mut route = vec![
        route_start_battle(ReducerRouteOwnerGroup::ActionEconomy),
        route_discover_battle_acts(
            ReducerRouteSubjectFamily::CreatureAttack,
            vec![BattleHoleKind::TargetChoice],
            ReducerRouteOwnerGroup::TargetSelection,
        ),
    ];
    for (fill, holes, owner) in [
        (
            ReducerRouteFillKind::TargetChoice,
            vec![BattleHoleKind::AttackRoll],
            ReducerRouteOwnerGroup::TargetSelection,
        ),
        (
            ReducerRouteFillKind::AttackRoll,
            vec![BattleHoleKind::RolledDice],
            ReducerRouteOwnerGroup::AttackRoll,
        ),
        (
            ReducerRouteFillKind::RolledDice,
            Vec::new(),
            ReducerRouteOwnerGroup::HitPoint,
        ),
    ] {
        route.push(
            super::battle_runtime_reducer_route::route_resolve_battle_subject(
                ReducerRouteSubjectFamily::CreatureAttack,
                fill,
                holes,
                owner,
            ),
        );
    }
    route
}

pub fn projection_payload(state: &CreatureAttackState) -> String {
    [
        format!("qCreatureAHp={}", state.creature_a_hit_points),
        format!("qCreatureBHp={}", state.creature_b_hit_points),
    ]
    .join("\n")
}

pub fn replay_sampled_inputs() -> CreatureAttackFills {
    CreatureAttackFills {
        damage: REPLAY_DAMAGE_SAMPLE,
        hit: REPLAY_HIT_SAMPLE,
    }
}

fn replay_attack(attacker: CreatureAttackActor) -> CreatureAttackState {
    resolve_creature_attack(
        creature_attack_initial_state(),
        attacker,
        replay_sampled_inputs(),
    )
}

fn replay_attack_through_battle(attacker: CreatureAttackActor) -> CreatureAttackState {
    let (state, subject, _) = creature_attack_target_choice_route(attacker);
    let (result, _) = resolve_creature_attack_route(
        state,
        subject,
        BattleWeaponAttackFill::AttackRoll(hit_attack_roll()),
        ReducerRouteFillKind::AttackRoll,
        ReducerRouteOwnerGroup::AttackRoll,
        Vec::new(),
    );
    let (state, subject) = battle_resolution_continuation(result, "creature attack roll hit");
    let (result, _) = resolve_creature_attack_route(
        state,
        subject,
        BattleWeaponAttackFill::DamageRoll(REPLAY_DAMAGE_SAMPLE),
        ReducerRouteFillKind::RolledDice,
        ReducerRouteOwnerGroup::HitPoint,
        Vec::new(),
    );
    creature_attack_projection_from_battle(result.state())
}

fn creature_attack_route_for_hit(attacker: CreatureAttackActor) -> Vec<ReducerRouteEvent> {
    let (state, subject, route) = creature_attack_target_choice_route(attacker);
    let (result, route) = resolve_creature_attack_route(
        state,
        subject,
        BattleWeaponAttackFill::AttackRoll(hit_attack_roll()),
        ReducerRouteFillKind::AttackRoll,
        ReducerRouteOwnerGroup::AttackRoll,
        route,
    );
    let (state, subject) = battle_resolution_continuation(result, "creature attack roll hit");
    let (_result, route) = resolve_creature_attack_route(
        state,
        subject,
        BattleWeaponAttackFill::DamageRoll(REPLAY_DAMAGE_SAMPLE),
        ReducerRouteFillKind::RolledDice,
        ReducerRouteOwnerGroup::HitPoint,
        route,
    );
    route
}

fn creature_attack_target_choice_route(
    attacker: CreatureAttackActor,
) -> (BattleState, BattleSubject, Vec<ReducerRouteEvent>) {
    let state = creature_attack_battle(attacker);
    let act = discovered_weapon_attack(&state);
    let mut route = vec![route_start_battle(ReducerRouteOwnerGroup::ActionEconomy)];
    route.push(route_discover_battle_acts(
        ReducerRouteSubjectFamily::CreatureAttack,
        act.holes,
        ReducerRouteOwnerGroup::TargetSelection,
    ));
    let (result, route) = resolve_creature_attack_route(
        state,
        act.subject,
        BattleWeaponAttackFill::TargetChoice(creature_attack_target(attacker)),
        ReducerRouteFillKind::TargetChoice,
        ReducerRouteOwnerGroup::TargetSelection,
        route,
    );
    let (state, subject) = battle_resolution_continuation(result, "creature attack target choice");
    (state, subject, route)
}

fn resolve_creature_attack_route(
    state: BattleState,
    subject: BattleSubject,
    fill: BattleWeaponAttackFill,
    route_fill: ReducerRouteFillKind,
    owner: ReducerRouteOwnerGroup,
    mut route: Vec<ReducerRouteEvent>,
) -> (
    crate::rules::battle_reducer_spine::BattleResolutionResult,
    Vec<ReducerRouteEvent>,
) {
    let request = BattleResolutionRequest::weapon_attack(subject, fill)
        .expect("adapter-selected creature attack subject must accept weapon attack fill");
    let result = resolve_battle_subject(state, request);
    route.push(route_resolve_battle_subject_from_result(
        ReducerRouteResolveConnector {
            subject: ReducerRouteSubjectFamily::CreatureAttack,
            fill: ReducerRouteResolveFill::Fill(route_fill),
            owner,
        },
        &result,
    ));
    (result, route)
}

fn discovered_weapon_attack(
    state: &BattleState,
) -> crate::rules::battle_reducer_spine::AvailableBattleAct {
    discover_battle_acts(state)
        .into_available_acts()
        .into_iter()
        .find(|act| act.subject.kind == BattleSubjectKind::WeaponAttack)
        .expect("creature attack battle must expose a weapon attack subject")
}

fn creature_attack_battle(attacker: CreatureAttackActor) -> BattleState {
    let mut setup = BattleSetup::standard();
    setup.initiative = Initiative {
        round: 1,
        already_acted: Vec::new(),
        still_to_act: InitiativeStillToAct {
            actor: creature_attack_actor(attacker),
            waiting: vec![creature_attack_target(attacker)],
        },
    };
    setup.fighter = creature_attack_combatant();
    setup.goblin = creature_attack_combatant();
    start_battle(setup).state
}

fn creature_attack_combatant() -> Combatant {
    Combatant {
        hp: 20,
        max_hp: 20,
        temporary_hp: 0,
        armor_class: 10,
        speed_feet: 30,
        shield_armor_class_bonus_active: false,
        unconscious: false,
        incapacitated: false,
        prone: false,
        creature_size: CreatureSize::Medium,
        lifecycle: CombatantLifecycle::DiesAtZeroHitPoints,
        reaction_available: true,
        movement_spent_feet: 0,
        weapon_attack_supported: true,
        weapon_damage_modifier: 0,
        multiattack_profile: None,
        sneak_attack_supported: false,
        sneak_attack_used_this_turn: false,
        recharge_available: false,
        spell_slots: crate::rules::battle_reducer_spine::BattleSpellSlotLedger::none(),
    }
}

fn creature_attack_actor(attacker: CreatureAttackActor) -> Actor {
    match attacker {
        CreatureAttackActor::CreatureA => Actor::Fighter,
        CreatureAttackActor::CreatureB => Actor::Goblin,
    }
}

fn creature_attack_target(attacker: CreatureAttackActor) -> Actor {
    match attacker {
        CreatureAttackActor::CreatureA => Actor::Goblin,
        CreatureAttackActor::CreatureB => Actor::Fighter,
    }
}

fn creature_attack_projection_from_battle(state: &BattleState) -> CreatureAttackState {
    CreatureAttackState {
        creature_a_hit_points: state.fighter.hp,
        creature_b_hit_points: state.goblin.hp,
    }
}

fn hit_attack_roll() -> AttackRollFacts {
    AttackRollFacts {
        total: 15,
        natural_d20: 10,
    }
}
