//! Experimental reducer-spine assembly from copied QNT support modules.
//!
//! This module is not accepted battle-runtime parity evidence by itself. It is a
//! tracked experiment proving that one durable `BattleState` path can be built
//! from QNT before an MBT adapter is wired through the same entrypoints.

use crate::rules::attack_damage_disposition::{
    apply_resolved_damage_to_positive_hit_points, CreatureKind, CreatureVitals,
};
use crate::rules::weapon_attack_ordering::{
    weapon_attack_fill_order_result, weapon_attack_hole_frontier, WeaponAttackFillKind,
    WeaponAttackFillOrderResult, WeaponAttackFillOrderingError, WeaponAttackFrontierStage,
    WeaponAttackHoleKind,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Actor {
    Fighter,
    Goblin,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CombatantLifecycle {
    UsesDeathSavingThrows,
    DiesAtZeroHitPoints,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Combatant {
    pub hp: i16,
    pub max_hp: i16,
    pub temporary_hp: i16,
    pub armor_class: i16,
    pub unconscious: bool,
    pub incapacitated: bool,
    pub lifecycle: CombatantLifecycle,
    pub reaction_available: bool,
    pub movement_spent_feet: i16,
    pub sneak_attack_supported: bool,
    pub sneak_attack_used_this_turn: bool,
    pub multiattack_dispatches_remaining: i16,
    pub recharge_available: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct InitiativeStillToAct {
    pub actor: Actor,
    pub waiting: Vec<Actor>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Initiative {
    pub round: i16,
    pub already_acted: Vec<Actor>,
    pub still_to_act: InitiativeStillToAct,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BattleState {
    pub initiative: Initiative,
    pub fighter: Combatant,
    pub goblin: Combatant,
    pub action_available: bool,
    pub bonus_action_available: bool,
    pub attack_roll_made_this_turn: bool,
    pub dash_movement_bonus_feet: i16,
    pub disengaged: bool,
    pub interrupt_stack_depth: usize,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BattleSubjectKind {
    WeaponAttack,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BattleSubject {
    pub kind: BattleSubjectKind,
    pub actor: Actor,
    pub target: Option<Actor>,
    pub stage: WeaponAttackFrontierStage,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AvailableBattleAct {
    pub subject: BattleSubject,
    pub holes: Vec<WeaponAttackHoleKind>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct AttackRollFacts {
    pub total: i16,
    pub natural_d20: i16,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BattleFill {
    TargetChoice(Actor),
    AttackRoll(AttackRollFacts),
    DamageRoll(i16),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BattleResolutionInvalidReason {
    InvalidFill,
    StaleSubject,
    WrongActor,
    WrongTarget,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BattleResolutionResult {
    NeedsHoles {
        state: BattleState,
        subject: BattleSubject,
        holes: Vec<WeaponAttackHoleKind>,
    },
    Resolved {
        state: BattleState,
    },
    Invalid {
        state: BattleState,
        reason: BattleResolutionInvalidReason,
        holes: Vec<WeaponAttackHoleKind>,
    },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct AttackRollOutcome {
    hits: bool,
    critical: bool,
}

#[must_use]
pub fn start_battle() -> BattleState {
    // QNT: cleanroom-input/qnt/battle-runtime/battle-runtime-model.qnt
    // `initialState`.
    BattleState {
        initiative: Initiative {
            round: 1,
            already_acted: Vec::new(),
            still_to_act: InitiativeStillToAct {
                actor: Actor::Fighter,
                waiting: vec![Actor::Goblin],
            },
        },
        fighter: Combatant {
            hp: 12,
            max_hp: 12,
            temporary_hp: 0,
            armor_class: 10,
            unconscious: false,
            incapacitated: false,
            lifecycle: CombatantLifecycle::UsesDeathSavingThrows,
            reaction_available: true,
            movement_spent_feet: 0,
            sneak_attack_supported: false,
            sneak_attack_used_this_turn: false,
            multiattack_dispatches_remaining: 0,
            recharge_available: false,
        },
        goblin: Combatant {
            hp: 10,
            max_hp: 10,
            temporary_hp: 0,
            armor_class: 15,
            unconscious: false,
            incapacitated: false,
            lifecycle: CombatantLifecycle::DiesAtZeroHitPoints,
            reaction_available: true,
            movement_spent_feet: 0,
            sneak_attack_supported: false,
            sneak_attack_used_this_turn: false,
            multiattack_dispatches_remaining: 0,
            recharge_available: true,
        },
        action_available: true,
        bonus_action_available: true,
        attack_roll_made_this_turn: false,
        dash_movement_bonus_feet: 0,
        disengaged: false,
        interrupt_stack_depth: 0,
    }
}

#[must_use]
pub fn current_actor(state: &BattleState) -> Actor {
    // QNT: battle-runtime-turn-order.qnt `currentActor`.
    state.initiative.still_to_act.actor
}

#[must_use]
pub fn discover_battle_acts(state: &BattleState) -> Vec<AvailableBattleAct> {
    // QNT: battle-runtime-combat-holes.qnt `combatOpenHoles`, narrowed to the
    // Fighter weapon-attack path for this reducer-spine experiment.
    if !current_actor_can_attack(state) {
        return Vec::new();
    }

    vec![AvailableBattleAct {
        subject: BattleSubject {
            kind: BattleSubjectKind::WeaponAttack,
            actor: current_actor(state),
            target: None,
            stage: WeaponAttackFrontierStage::TargetChoice,
        },
        holes: weapon_attack_hole_frontier(WeaponAttackFrontierStage::TargetChoice),
    }]
}

#[must_use]
pub fn resolve_battle_subject(
    state: BattleState,
    subject: BattleSubject,
    fill: BattleFill,
) -> BattleResolutionResult {
    if subject.actor != current_actor(&state) {
        return invalid(
            state,
            BattleResolutionInvalidReason::WrongActor,
            subject.stage,
        );
    }

    if !state.action_available || subject.stage == WeaponAttackFrontierStage::Resolved {
        return invalid(
            state,
            BattleResolutionInvalidReason::StaleSubject,
            subject.stage,
        );
    }

    match fill {
        BattleFill::TargetChoice(target) => resolve_target_choice(state, subject, target),
        BattleFill::AttackRoll(roll) => resolve_attack_roll_fill(state, subject, roll),
        BattleFill::DamageRoll(rolled_damage) => resolve_damage_roll(state, subject, rolled_damage),
    }
}

#[must_use]
pub fn combatant_is_dead(combatant: Combatant) -> bool {
    match combatant.lifecycle {
        CombatantLifecycle::DiesAtZeroHitPoints => combatant.hp == 0,
        CombatantLifecycle::UsesDeathSavingThrows => false,
    }
}

fn current_actor_can_attack(state: &BattleState) -> bool {
    if !state.action_available {
        return false;
    }

    match current_actor(state) {
        Actor::Fighter => !state.fighter.unconscious && !state.fighter.incapacitated,
        Actor::Goblin => state.goblin.hp > 0,
    }
}

fn resolve_target_choice(
    state: BattleState,
    subject: BattleSubject,
    target: Actor,
) -> BattleResolutionResult {
    let order =
        weapon_attack_fill_order_result(subject.stage, WeaponAttackFillKind::TargetChoice, false);
    let next_stage = match accepted_stage(order) {
        Ok(stage) => stage,
        Err(reason) => return invalid(state, reason, subject.stage),
    };

    if target == subject.actor {
        return invalid(
            state,
            BattleResolutionInvalidReason::WrongTarget,
            subject.stage,
        );
    }

    let subject = BattleSubject {
        target: Some(target),
        stage: next_stage,
        ..subject
    };

    BattleResolutionResult::NeedsHoles {
        state,
        subject,
        holes: weapon_attack_hole_frontier(next_stage),
    }
}

fn resolve_attack_roll_fill(
    state: BattleState,
    subject: BattleSubject,
    roll: AttackRollFacts,
) -> BattleResolutionResult {
    let Some(target) = subject.target else {
        return invalid(
            state,
            BattleResolutionInvalidReason::InvalidFill,
            subject.stage,
        );
    };
    let outcome = resolve_attack_roll(roll, armor_class_for(&state, target), 20);
    let order = weapon_attack_fill_order_result(
        subject.stage,
        WeaponAttackFillKind::AttackRoll,
        outcome.hits,
    );
    let next_stage = match accepted_stage(order) {
        Ok(stage) => stage,
        Err(reason) => return invalid(state, reason, subject.stage),
    };

    let state = BattleState {
        attack_roll_made_this_turn: true,
        ..state
    };

    if !outcome.hits {
        return BattleResolutionResult::Resolved {
            state: BattleState {
                action_available: false,
                ..state
            },
        };
    }

    let subject = BattleSubject {
        stage: next_stage,
        ..subject
    };

    BattleResolutionResult::NeedsHoles {
        state,
        subject,
        holes: weapon_attack_hole_frontier(next_stage),
    }
}

fn resolve_damage_roll(
    state: BattleState,
    subject: BattleSubject,
    rolled_damage: i16,
) -> BattleResolutionResult {
    let Some(target) = subject.target else {
        return invalid(
            state,
            BattleResolutionInvalidReason::InvalidFill,
            subject.stage,
        );
    };
    let order =
        weapon_attack_fill_order_result(subject.stage, WeaponAttackFillKind::RolledDice, true);
    if let Err(reason) = accepted_stage(order) {
        return invalid(state, reason, subject.stage);
    }

    let state = with_damaged_target(state, target, rolled_damage);
    BattleResolutionResult::Resolved {
        state: BattleState {
            action_available: false,
            ..state
        },
    }
}

fn accepted_stage(
    result: WeaponAttackFillOrderResult,
) -> Result<WeaponAttackFrontierStage, BattleResolutionInvalidReason> {
    match result {
        WeaponAttackFillOrderResult::Accepted(stage) => Ok(stage),
        WeaponAttackFillOrderResult::Rejected(error) => Err(match error {
            WeaponAttackFillOrderingError::TargetChoiceRequired
            | WeaponAttackFillOrderingError::AttackRollRequired => {
                BattleResolutionInvalidReason::InvalidFill
            }
        }),
        WeaponAttackFillOrderResult::NotOrderingError(_) => {
            Err(BattleResolutionInvalidReason::InvalidFill)
        }
    }
}

fn invalid(
    state: BattleState,
    reason: BattleResolutionInvalidReason,
    stage: WeaponAttackFrontierStage,
) -> BattleResolutionResult {
    BattleResolutionResult::Invalid {
        state,
        reason,
        holes: weapon_attack_hole_frontier(stage),
    }
}

fn resolve_attack_roll(
    facts: AttackRollFacts,
    armor_class: i16,
    critical_threshold: i16,
) -> AttackRollOutcome {
    // QNT: shared-algebras/proofs/rule-core/
    // attack-damage-composition.qnt `resolveAttackRoll`.
    let critical = facts.natural_d20 >= critical_threshold;
    AttackRollOutcome {
        hits: facts.natural_d20 != 1 && (critical || facts.total >= armor_class),
        critical,
    }
}

fn armor_class_for(state: &BattleState, actor: Actor) -> i16 {
    match actor {
        Actor::Fighter => state.fighter.armor_class,
        Actor::Goblin => state.goblin.armor_class,
    }
}

fn with_damaged_target(mut state: BattleState, target: Actor, damage: i16) -> BattleState {
    let target_combatant = combatant_for(&state, target);
    let result =
        apply_resolved_damage_to_positive_hit_points(combatant_vitals(target_combatant), damage);
    let damaged = with_vitals(target_combatant, result.vitals);
    match target {
        Actor::Fighter => state.fighter = damaged,
        Actor::Goblin => state.goblin = damaged,
    }
    state
}

fn combatant_for(state: &BattleState, actor: Actor) -> Combatant {
    match actor {
        Actor::Fighter => state.fighter,
        Actor::Goblin => state.goblin,
    }
}

fn combatant_vitals(combatant: Combatant) -> CreatureVitals {
    CreatureVitals {
        kind: match combatant.lifecycle {
            CombatantLifecycle::UsesDeathSavingThrows => CreatureKind::PlayerCharacter,
            CombatantLifecycle::DiesAtZeroHitPoints => CreatureKind::Monster,
        },
        hit_points: combatant.hp,
        hit_point_maximum: combatant.max_hp,
        temporary_hit_points: combatant.temporary_hp,
        dead: combatant_is_dead(combatant),
        unconscious: combatant.unconscious,
    }
}

fn with_vitals(combatant: Combatant, vitals: CreatureVitals) -> Combatant {
    Combatant {
        hp: vitals.hit_points,
        temporary_hp: vitals.temporary_hit_points,
        unconscious: vitals.unconscious,
        ..combatant
    }
}
