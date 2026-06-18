//! Experimental reducer-spine assembly from copied QNT support modules.
//!
//! This module is not accepted battle-runtime parity evidence by itself. It is a
//! tracked experiment proving that one durable `BattleState` path can be built
//! from QNT before an MBT adapter is wired through the same entrypoints.

use crate::rules::attack_damage_disposition::{
    apply_resolved_damage_to_positive_hit_points, CreatureKind, CreatureVitals,
};
use crate::rules::creature_size::CreatureSize;
use crate::rules::interrupt_stack_resume::{
    decline_reaction_window, interrupt_stack_resume_projection, offer_reaction_window,
    reaction_window_depth, recorded_attack_replay_from_root_equivalent, take_matching_reaction,
    InterruptPendingTrigger, InterruptResumeHole, InterruptStackResumeProjectionFacts,
    InterruptStackResumeScenarioOutcome, InterruptStackResumeState, ReactionWindowOffer,
    ReactionWindowRole,
};
use crate::rules::rule_core_stat_block_controls::{
    resolve_stat_block_control_subject, start_stat_block_multiattack_from,
    stat_block_control_initial_state, stat_block_multiattack_continuation_open,
    StatBlockAttackSlot, StatBlockControlState, StatBlockDispatchSubject,
    StatBlockMultiattackProfile,
};
use crate::rules::spell_attack_ordering::{
    spell_attack_fill_order_error, spell_attack_fill_order_result,
    spell_attack_fill_order_runtime_result, spell_attack_ordering_initial_state,
    spell_attack_ordering_projection, SpellAttackFillKind, SpellAttackFillOrderResult,
    SpellAttackFillOrderingError, SpellAttackFrontierStage, SpellAttackOrderingProjectionFacts,
    SpellAttackOrderingState, SpellAttackRuntimeResult,
};
use crate::rules::stat_block_action_ordering::{
    stat_block_action_fill_order_result, stat_block_action_hole_frontier,
    stat_block_action_ordering_projection, StatBlockActionFillKind, StatBlockActionFillOrderResult,
    StatBlockActionFillOrderingError, StatBlockActionFrontierStage, StatBlockActionHoleKind,
    StatBlockActionOrderingProjectionFacts, StatBlockActionOrderingState,
    StatBlockActionRuntimeResult,
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
    Rogue,
    Skeleton,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CombatantLifecycle {
    UsesDeathSavingThrows,
    DiesAtZeroHitPoints,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BattleSpellSlotLevel {
    First,
    Second,
    Third,
    Fourth,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BattleSpellSlotLedger {
    pub first_level_expended: i16,
    pub second_level_expended: i16,
    pub third_level_expended: i16,
    pub fourth_level_expended: i16,
}

impl BattleSpellSlotLedger {
    #[must_use]
    pub fn none() -> Self {
        Self {
            first_level_expended: 0,
            second_level_expended: 0,
            third_level_expended: 0,
            fourth_level_expended: 0,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Combatant {
    pub hp: i16,
    pub max_hp: i16,
    pub temporary_hp: i16,
    pub armor_class: i16,
    pub shield_armor_class_bonus_active: bool,
    pub unconscious: bool,
    pub incapacitated: bool,
    pub prone: bool,
    pub creature_size: CreatureSize,
    pub lifecycle: CombatantLifecycle,
    pub reaction_available: bool,
    pub movement_spent_feet: i16,
    pub sneak_attack_supported: bool,
    pub sneak_attack_used_this_turn: bool,
    pub recharge_available: bool,
    pub spell_slots: BattleSpellSlotLedger,
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
    pub rogue: Combatant,
    pub skeleton: Combatant,
    pub stat_block_control: StatBlockControlState,
    pub turn_boundary_effects: TurnBoundaryEffects,
    pub interrupt_resume: BattleInterruptResumeState,
    pub reaction_casting_time: BattleReactionCastingTimeState,
    pub spell_attack_procedure: BattleSpellAttackProcedure,
    pub spell_slot_uses_this_turn: Vec<BattleTurnSpellSlotUse>,
    pub level_one_plus_spell_casters_this_turn: Vec<Actor>,
    pub quickened_level_one_plus_spell_casters_this_turn: Vec<Actor>,
    pub action_available: bool,
    pub bonus_action_available: bool,
    pub attack_roll_made_this_turn: bool,
    pub dash_movement_bonus_feet: i16,
    pub disengaged: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BattleTurnSpellSlotUse {
    Pending(Actor),
    Committed(Actor),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BattleReactionCastingTrigger {
    None,
    AfterDamage,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BattleReactionCastingContinuation {
    None,
    AfterDamageResolved,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BattleReactionCastingOutcome {
    Init,
    AfterDamageReactionResolved,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BattleReactionSpellSelectedIdentityOutcome {
    Init,
    ShieldReactionSpellHitResolved,
    HellishRebukeFailedSaveResolved,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BattleReactionCastingTimeState {
    pub trigger: BattleReactionCastingTrigger,
    pub continuation: BattleReactionCastingContinuation,
    pub reaction_window_open: bool,
    pub outcome: BattleReactionCastingOutcome,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BattleReactionSpellSelectedIdentityProjection {
    pub reactor_hp: i16,
    pub trigger_creature_hp: i16,
    pub reactor_armor_class: i16,
    pub reactor_reaction_available: bool,
    pub trigger_creature_first_level_slots_expended: i16,
    pub first_level_slots_expended: i16,
    pub second_level_slots_expended: i16,
    pub third_level_slots_expended: i16,
    pub outcome: BattleReactionSpellSelectedIdentityOutcome,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BattleSpellAttackProcedure {
    Inactive,
    Active(BattleSpellAttackSubject),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BattleSpellAttackSubject {
    pub actor: Actor,
    pub target: Option<Actor>,
    pub stage: SpellAttackFrontierStage,
    pub requires_spell_slot: bool,
    pub last_ordering_error: Option<SpellAttackFillOrderingError>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BattleSpellAttackFill {
    TargetChoice(Actor),
    DamageTypeChoice,
    AttackRoll(AttackRollFacts),
    DamageRoll(i16),
}

impl BattleReactionCastingTimeState {
    #[must_use]
    pub fn none() -> Self {
        Self {
            trigger: BattleReactionCastingTrigger::None,
            continuation: BattleReactionCastingContinuation::None,
            reaction_window_open: false,
            outcome: BattleReactionCastingOutcome::Init,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BattleInterruptResumeState {
    pub reaction_window: ReactionWindowOffer,
    pub max_stack_depth_observed: u8,
    pub pending_trigger: InterruptPendingTrigger,
    pub resumed_hole: InterruptResumeHole,
    pub active_effect_mutation_seen_on_resume: bool,
    pub replay_from_root_equivalent: bool,
    pub scenario_outcome: InterruptStackResumeScenarioOutcome,
}

impl BattleInterruptResumeState {
    #[must_use]
    pub fn none() -> Self {
        Self {
            reaction_window: ReactionWindowOffer::Closed,
            max_stack_depth_observed: 0,
            pending_trigger: InterruptPendingTrigger::NoPendingTrigger,
            resumed_hole: InterruptResumeHole::NoResumedHole,
            active_effect_mutation_seen_on_resume: false,
            replay_from_root_equivalent: false,
            scenario_outcome: InterruptStackResumeScenarioOutcome::Init,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TurnBoundaryEffects {
    pub turn_start_damage_active: bool,
    pub turn_end_damage_active: bool,
    pub until_next_turn_active: bool,
    pub start_turn_ongoing_feature_active: bool,
    pub end_turn_ongoing_feature_active: bool,
}

impl TurnBoundaryEffects {
    #[must_use]
    pub fn none() -> Self {
        Self {
            turn_start_damage_active: false,
            turn_end_damage_active: false,
            until_next_turn_active: false,
            start_turn_ongoing_feature_active: false,
            end_turn_ongoing_feature_active: false,
        }
    }

    #[must_use]
    pub fn lifecycle_witness_fixture() -> Self {
        // QNT: battle-runtime-turn-boundary-effect-lifecycle.mbt.qnt `init`.
        Self {
            turn_start_damage_active: true,
            turn_end_damage_active: true,
            until_next_turn_active: true,
            start_turn_ongoing_feature_active: true,
            end_turn_ongoing_feature_active: true,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BattleSubjectKind {
    WeaponAttack,
    Multiattack,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BattleSubject {
    pub kind: BattleSubjectKind,
    pub actor: Actor,
    pub target: Option<Actor>,
    pub stage: WeaponAttackFrontierStage,
    pub damage_modifier: i16,
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
    SneakAttackDamageRoll(i16),
    ResolveMultiattack,
    SpendMultiattackDispatch,
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
pub struct StatBlockActionSubject {
    pub actor: Actor,
    pub target: Option<Actor>,
    pub stage: StatBlockActionFrontierStage,
    pub damage_mode: StatBlockActionDamageMode,
    pub prone_on_hit_rider: StatBlockProneOnHitRider,
    pub recharge_action_available: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StatBlockActionDamageMode {
    Rolled,
    Static { damage: i16 },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StatBlockProneOnHitRider {
    NoProneRider,
    MediumOrSmaller { target_prone_condition_immune: bool },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StatBlockActionFill {
    TargetChoice(Actor),
    AttackRoll(AttackRollFacts),
    DamageDice(i16),
    RechargeRoll(i16),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StatBlockActionResolutionInvalidReason {
    InvalidFill,
    StaleSubject,
    WrongActor,
    WrongTarget,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum StatBlockActionResolutionResult {
    NeedsHoles {
        state: BattleState,
        subject: StatBlockActionSubject,
        holes: Vec<StatBlockActionHoleKind>,
    },
    Resolved {
        state: BattleState,
        subject: StatBlockActionSubject,
    },
    Invalid {
        state: BattleState,
        subject: StatBlockActionSubject,
        reason: StatBlockActionResolutionInvalidReason,
        holes: Vec<StatBlockActionHoleKind>,
        last_ordering_error: Option<StatBlockActionFillOrderingError>,
    },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct AttackRollOutcome {
    hits: bool,
    critical: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum StatBlockOrderTransition {
    Accepted(StatBlockActionFrontierStage),
    Rejected(StatBlockActionFillOrderingError),
    NotOrdering(StatBlockActionFrontierStage),
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
        fighter: fighter_combatant(),
        goblin: goblin_combatant(),
        rogue: rogue_combatant(),
        skeleton: skeleton_combatant(),
        stat_block_control: stat_block_control_initial_state(),
        turn_boundary_effects: TurnBoundaryEffects::none(),
        interrupt_resume: BattleInterruptResumeState::none(),
        reaction_casting_time: BattleReactionCastingTimeState::none(),
        spell_attack_procedure: BattleSpellAttackProcedure::Inactive,
        spell_slot_uses_this_turn: Vec::new(),
        level_one_plus_spell_casters_this_turn: Vec::new(),
        quickened_level_one_plus_spell_casters_this_turn: Vec::new(),
        action_available: true,
        bonus_action_available: true,
        attack_roll_made_this_turn: false,
        dash_movement_bonus_feet: 0,
        disengaged: false,
    }
}

#[must_use]
pub fn start_skeleton_battle() -> BattleState {
    // QNT: battle-runtime-weapon-attack-skeleton.mbt.qnt literal initial
    // projection for the Rogue attacking a Skeleton.
    BattleState {
        initiative: Initiative {
            round: 1,
            already_acted: Vec::new(),
            still_to_act: InitiativeStillToAct {
                actor: Actor::Rogue,
                waiting: vec![Actor::Skeleton],
            },
        },
        fighter: fighter_combatant(),
        goblin: goblin_combatant(),
        rogue: rogue_combatant(),
        skeleton: skeleton_combatant(),
        stat_block_control: stat_block_control_initial_state(),
        turn_boundary_effects: TurnBoundaryEffects::none(),
        interrupt_resume: BattleInterruptResumeState::none(),
        reaction_casting_time: BattleReactionCastingTimeState::none(),
        spell_attack_procedure: BattleSpellAttackProcedure::Inactive,
        spell_slot_uses_this_turn: Vec::new(),
        level_one_plus_spell_casters_this_turn: Vec::new(),
        quickened_level_one_plus_spell_casters_this_turn: Vec::new(),
        action_available: true,
        bonus_action_available: true,
        attack_roll_made_this_turn: false,
        dash_movement_bonus_feet: 0,
        disengaged: false,
    }
}

#[must_use]
pub fn start_skeleton_actor_turn() -> BattleState {
    BattleState {
        initiative: Initiative {
            round: 1,
            already_acted: vec![Actor::Rogue],
            still_to_act: InitiativeStillToAct {
                actor: Actor::Skeleton,
                waiting: Vec::new(),
            },
        },
        ..start_skeleton_battle()
    }
}

#[must_use]
pub fn start_goblin_stat_block_battle() -> BattleState {
    BattleState {
        initiative: Initiative {
            round: 1,
            already_acted: vec![Actor::Fighter],
            still_to_act: InitiativeStillToAct {
                actor: Actor::Goblin,
                waiting: Vec::new(),
            },
        },
        ..start_battle()
    }
}

#[must_use]
pub fn start_goblin_prone_rider_battle(target_size: CreatureSize) -> BattleState {
    // QNT: battle-runtime-stat-block-size-gated-condition-rider.mbt.qnt
    // uses a target with 20 HP and a size/condition-immunity gate for a Prone
    // on-hit rider.
    let mut state = start_goblin_stat_block_battle();
    state.fighter = Combatant {
        hp: 20,
        max_hp: 20,
        prone: false,
        creature_size: target_size,
        ..state.fighter
    };
    state
}

#[must_use]
pub fn start_turn_boundary_effect_lifecycle_battle() -> BattleState {
    // QNT: battle-runtime-turn-boundary-effect-lifecycle.mbt.qnt `init`;
    // battle-runtime-turn-advancement.qnt `endTurn`.
    BattleState {
        turn_boundary_effects: TurnBoundaryEffects::lifecycle_witness_fixture(),
        ..start_battle()
    }
}

#[must_use]
pub fn start_interrupt_stack_resume_battle() -> BattleState {
    // QNT: battle-runtime-interrupt-stack-resume.mbt.qnt `init`. The
    // interrupted target is the Fighter because the witness projects target HP
    // from 12 through the nested/replay cases.
    start_goblin_stat_block_battle()
}

#[must_use]
pub fn start_reaction_casting_time_battle() -> BattleState {
    // QNT: battle-runtime-reaction-casting-time.mbt.qnt `initialHp`.
    let mut state = start_goblin_stat_block_battle();
    state.fighter = Combatant {
        hp: 30,
        max_hp: 30,
        ..state.fighter
    };
    state.goblin = Combatant {
        hp: 30,
        max_hp: 30,
        ..state.goblin
    };
    state
}

#[must_use]
pub fn start_reaction_spell_selected_identity_battle() -> BattleState {
    // QNT: battle-runtime-reaction-spell-selected-identity.mbt.qnt `init`.
    let mut state = start_battle();
    state.goblin = Combatant {
        hp: 12,
        max_hp: 12,
        ..state.goblin
    };
    state
}

#[must_use]
pub fn start_spell_attack_ordering_battle() -> BattleState {
    start_battle()
}

#[must_use]
pub fn discover_single_target_spell_attack_battle(state: BattleState) -> BattleState {
    // RAW: cleanroom-input/raw/srd-5.2.1/Playing-the-Game.md
    // "Making an Attack"; QNT: battle-runtime-spell-attack-ordering.qnt
    // `SpellAttackTargetChoiceStage`.
    start_spell_attack_subject(state, false, SpellAttackFrontierStage::TargetChoice)
}

#[must_use]
pub fn discover_typed_spell_attack_battle(state: BattleState) -> BattleState {
    // RAW: cleanroom-input/raw/srd-5.2.1/Spells/Gaining-and-Casting.md
    // "Spell Slots"; support QNT: battle-runtime-spell-invocation.qnt
    // `claimPendingActorSpellSlotUse`; ordering QNT:
    // battle-runtime-spell-attack-ordering.qnt
    // `SpellAttackDamageTypeAndTargetChoiceStage`.
    let actor = current_actor(&state);
    if !state.action_available || !can_actor_expend_spell_slot_this_turn(&state, actor) {
        return state;
    }
    let state = claim_pending_actor_spell_slot_use(state, actor);
    start_spell_attack_subject(
        state,
        true,
        SpellAttackFrontierStage::DamageTypeAndTargetChoice,
    )
}

#[must_use]
pub fn resolve_spell_attack_subject(
    state: BattleState,
    fill: BattleSpellAttackFill,
) -> BattleState {
    let BattleSpellAttackProcedure::Active(subject) = state.spell_attack_procedure else {
        return state;
    };
    if subject.actor != current_actor(&state) {
        return state;
    }
    let fill_kind = spell_attack_fill_kind(fill);
    let attack_roll_hits = spell_attack_fill_attack_hits(&state, &subject, fill);
    let result = spell_attack_fill_order_result(subject.stage, fill_kind, attack_roll_hits);
    let next_stage = spell_attack_result_stage(result);
    let next_subject = BattleSpellAttackSubject {
        target: spell_attack_target_after_fill(subject.target, fill, result),
        stage: next_stage,
        last_ordering_error: spell_attack_fill_order_error(result),
        ..subject
    };
    let next_state = BattleState {
        spell_attack_procedure: BattleSpellAttackProcedure::Active(next_subject),
        ..state
    };
    if spell_attack_fill_order_runtime_result(result) == SpellAttackRuntimeResult::Resolved {
        finalize_spell_attack_subject(next_state, next_subject)
    } else {
        next_state
    }
}

#[must_use]
pub fn spell_attack_ordering_projection_from_battle(
    state: &BattleState,
) -> SpellAttackOrderingState {
    match state.spell_attack_procedure {
        BattleSpellAttackProcedure::Inactive => spell_attack_ordering_initial_state(),
        BattleSpellAttackProcedure::Active(subject) => {
            spell_attack_ordering_projection(SpellAttackOrderingProjectionFacts {
                stage: subject.stage,
                runtime_result: if subject.stage == SpellAttackFrontierStage::Resolved {
                    SpellAttackRuntimeResult::Resolved
                } else {
                    SpellAttackRuntimeResult::NeedsHoles
                },
                last_ordering_error: subject.last_ordering_error,
            })
        }
    }
}

#[must_use]
pub fn resolve_shield_reaction_spell_hit_battle(state: BattleState) -> BattleState {
    // QNT: battle-runtime-reaction-window.qnt `castShieldReactionSpell`;
    // battle-runtime-reaction-resolution.qnt `resolveReactionOffer` AttackHit
    // Shield branch.
    if !can_actor_expend_spell_slot_this_turn(&state, Actor::Fighter)
        || !state.fighter.reaction_available
    {
        return state;
    }
    let state = claim_pending_actor_spell_slot_use(state, Actor::Fighter);
    let state = spend_reaction(state, Actor::Fighter);
    let state = apply_shield_armor_class_bonus(state, Actor::Fighter);
    let state = expend_combatant_spell_slot(state, Actor::Fighter, BattleSpellSlotLevel::First);
    commit_actor_spell_slot_use(state, Actor::Fighter)
}

#[must_use]
pub fn resolve_hellish_rebuke_after_damage_battle(state: BattleState) -> BattleState {
    // QNT: battle-runtime-reaction-casting-time.mbt.qnt in-scope after-damage
    // branch; battle-runtime-reaction-window.qnt
    // `openHellishRebukeAfterDamageReactionWindow`; battle-runtime-
    // reaction-resolution.qnt `resolveHellishRebukeAfterDamage`.
    let state = BattleState {
        reaction_casting_time: BattleReactionCastingTimeState {
            trigger: BattleReactionCastingTrigger::AfterDamage,
            continuation: BattleReactionCastingContinuation::AfterDamageResolved,
            reaction_window_open: true,
            outcome: BattleReactionCastingOutcome::Init,
        },
        ..with_damaged_target(state, Actor::Fighter, 1)
    };
    if !can_actor_expend_spell_slot_this_turn(&state, Actor::Fighter)
        || !state.fighter.reaction_available
    {
        return state;
    }
    let state = claim_pending_actor_spell_slot_use(state, Actor::Fighter);
    let state = spend_reaction(state, Actor::Fighter);
    let state = expend_combatant_spell_slot(state, Actor::Fighter, BattleSpellSlotLevel::Second);
    let state = commit_actor_spell_slot_use(state, Actor::Fighter);
    BattleState {
        reaction_casting_time: BattleReactionCastingTimeState {
            trigger: BattleReactionCastingTrigger::AfterDamage,
            continuation: BattleReactionCastingContinuation::AfterDamageResolved,
            reaction_window_open: false,
            outcome: BattleReactionCastingOutcome::AfterDamageReactionResolved,
        },
        ..with_damaged_target(state, Actor::Goblin, 3)
    }
}

#[must_use]
pub fn resolve_hellish_rebuke_failed_save_reaction_spell_battle(state: BattleState) -> BattleState {
    // QNT: battle-runtime-reaction-spell-selected-identity.mbt.qnt
    // failed-save selected-identity branch; support QNT:
    // battle-runtime-reaction-resolution.qnt `resolveHellishRebukeAfterDamage`.
    let state = with_damaged_target(state, Actor::Fighter, 1);
    if !can_actor_expend_spell_slot_this_turn(&state, Actor::Fighter)
        || !state.fighter.reaction_available
    {
        return state;
    }
    let state = claim_pending_actor_spell_slot_use(state, Actor::Fighter);
    let state = spend_reaction(state, Actor::Fighter);
    let state = expend_combatant_spell_slot(state, Actor::Fighter, BattleSpellSlotLevel::Second);
    let state = commit_actor_spell_slot_use(state, Actor::Fighter);
    with_damaged_target(state, Actor::Goblin, 3)
}

#[must_use]
pub fn reaction_spell_selected_identity_projection_from_battle(
    state: &BattleState,
    outcome: BattleReactionSpellSelectedIdentityOutcome,
) -> BattleReactionSpellSelectedIdentityProjection {
    BattleReactionSpellSelectedIdentityProjection {
        reactor_hp: state.fighter.hp,
        trigger_creature_hp: state.goblin.hp,
        reactor_armor_class: armor_class_for(state, Actor::Fighter),
        reactor_reaction_available: state.fighter.reaction_available,
        trigger_creature_first_level_slots_expended: state.goblin.spell_slots.first_level_expended,
        first_level_slots_expended: state.fighter.spell_slots.first_level_expended,
        second_level_slots_expended: state.fighter.spell_slots.second_level_expended,
        third_level_slots_expended: state.fighter.spell_slots.third_level_expended,
        outcome,
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BattleReactionCastingTimeProjection {
    pub trigger: BattleReactionCastingTrigger,
    pub continuation: BattleReactionCastingContinuation,
    pub reactor_hp: i16,
    pub trigger_creature_hp: i16,
    pub reactor_reaction_available: bool,
    pub trigger_creature_first_level_slots_expended: i16,
    pub trigger_creature_fourth_level_slots_expended: i16,
    pub reactor_second_level_slots_expended: i16,
    pub reactor_third_level_slots_expended: i16,
    pub reaction_window_cleared: bool,
    pub outcome: BattleReactionCastingOutcome,
}

#[must_use]
pub fn reaction_casting_time_projection_from_battle(
    state: &BattleState,
) -> BattleReactionCastingTimeProjection {
    BattleReactionCastingTimeProjection {
        trigger: state.reaction_casting_time.trigger,
        continuation: state.reaction_casting_time.continuation,
        reactor_hp: state.fighter.hp,
        trigger_creature_hp: state.goblin.hp,
        reactor_reaction_available: state.fighter.reaction_available,
        trigger_creature_first_level_slots_expended: state.goblin.spell_slots.first_level_expended,
        trigger_creature_fourth_level_slots_expended: state
            .goblin
            .spell_slots
            .fourth_level_expended,
        reactor_second_level_slots_expended: state.fighter.spell_slots.second_level_expended,
        reactor_third_level_slots_expended: state.fighter.spell_slots.third_level_expended,
        reaction_window_cleared: !state.reaction_casting_time.reaction_window_open,
        outcome: state.reaction_casting_time.outcome,
    }
}

#[must_use]
pub fn resolve_nested_interrupt_decline_battle(state: BattleState) -> BattleState {
    // QNT: battle-runtime-interrupt-stack-resume.mbt.qnt
    // nested decline branch; rule-core: reactions-continuations-concentration.qnt
    // `offerReactionWindow` and `declineReactionWindow`; assumption A45 bounds
    // active+suspended depth.
    let outer = offer_reaction_window(
        ReactionWindowOffer::Closed,
        ReactionWindowRole::AttackHitInterruption,
    );
    let inner = offer_reaction_window(outer, ReactionWindowRole::AttackHitInterruption);
    let resumed = decline_reaction_window(inner);
    let state = with_damaged_target(state, Actor::Fighter, 2);
    BattleState {
        interrupt_resume: BattleInterruptResumeState {
            reaction_window: resumed,
            max_stack_depth_observed: reaction_window_depth(inner),
            pending_trigger: InterruptPendingTrigger::AttackHit,
            resumed_hole: InterruptResumeHole::RolledDice,
            active_effect_mutation_seen_on_resume: false,
            replay_from_root_equivalent: false,
            scenario_outcome: InterruptStackResumeScenarioOutcome::NestedDeclineResumedOuter,
        },
        ..state
    }
}

#[must_use]
pub fn resolve_interrupted_attack_after_reaction_mutation_battle(
    state: BattleState,
) -> BattleState {
    // QNT: battle-runtime-interrupt-stack-resume.mbt.qnt
    // active-effect mutation branch; RAW: Rules-Glossary.md "Reaction";
    // assumption A45.
    let offered = offer_reaction_window(
        ReactionWindowOffer::Closed,
        ReactionWindowRole::AttackHitInterruption,
    );
    let (resumed, responder_reaction_available) =
        take_matching_reaction(offered, state.fighter.reaction_available);
    let mut state = state;
    state.fighter.reaction_available = responder_reaction_available;
    BattleState {
        interrupt_resume: BattleInterruptResumeState {
            reaction_window: resumed,
            max_stack_depth_observed: reaction_window_depth(offered),
            pending_trigger: InterruptPendingTrigger::NoPendingTrigger,
            resumed_hole: InterruptResumeHole::NoResumedHole,
            active_effect_mutation_seen_on_resume: true,
            replay_from_root_equivalent: false,
            scenario_outcome: InterruptStackResumeScenarioOutcome::ActiveEffectMutationResumed,
        },
        ..state
    }
}

#[must_use]
pub fn resolve_recorded_attack_replay_from_root_battle(state: BattleState) -> BattleState {
    // QNT: battle-runtime-interrupt-stack-resume.mbt.qnt
    // recorded attack replay branch; replay vocabulary:
    // battle-runtime-replay-equivalence.qnt.
    let state = with_damaged_target(state, Actor::Fighter, 9);
    BattleState {
        interrupt_resume: BattleInterruptResumeState {
            reaction_window: ReactionWindowOffer::Closed,
            max_stack_depth_observed: 0,
            pending_trigger: InterruptPendingTrigger::NoPendingTrigger,
            resumed_hole: InterruptResumeHole::NoResumedHole,
            active_effect_mutation_seen_on_resume: false,
            replay_from_root_equivalent: recorded_attack_replay_from_root_equivalent(12, 3),
            scenario_outcome: InterruptStackResumeScenarioOutcome::ReplayFromRootResolved,
        },
        ..state
    }
}

#[must_use]
pub fn interrupt_stack_resume_projection_from_battle(
    state: &BattleState,
) -> InterruptStackResumeState {
    interrupt_stack_resume_projection(InterruptStackResumeProjectionFacts {
        max_stack_depth_observed: state.interrupt_resume.max_stack_depth_observed,
        final_stack_depth: reaction_window_depth(state.interrupt_resume.reaction_window),
        pending_trigger: state.interrupt_resume.pending_trigger,
        resumed_hole: state.interrupt_resume.resumed_hole,
        active_effect_mutation_seen_on_resume: state
            .interrupt_resume
            .active_effect_mutation_seen_on_resume,
        replay_from_root_equivalent: state.interrupt_resume.replay_from_root_equivalent,
        responder_reaction_available: state.fighter.reaction_available,
        target_hit_points: state.fighter.hp,
        scenario_outcome: state.interrupt_resume.scenario_outcome,
    })
}

fn fighter_combatant() -> Combatant {
    Combatant {
        hp: 12,
        max_hp: 12,
        temporary_hp: 0,
        armor_class: 10,
        shield_armor_class_bonus_active: false,
        unconscious: false,
        incapacitated: false,
        prone: false,
        creature_size: CreatureSize::Medium,
        lifecycle: CombatantLifecycle::UsesDeathSavingThrows,
        reaction_available: true,
        movement_spent_feet: 0,
        sneak_attack_supported: false,
        sneak_attack_used_this_turn: false,
        recharge_available: false,
        spell_slots: BattleSpellSlotLedger::none(),
    }
}

fn goblin_combatant() -> Combatant {
    Combatant {
        hp: 10,
        max_hp: 10,
        temporary_hp: 0,
        armor_class: 15,
        shield_armor_class_bonus_active: false,
        unconscious: false,
        incapacitated: false,
        prone: false,
        creature_size: CreatureSize::Small,
        lifecycle: CombatantLifecycle::DiesAtZeroHitPoints,
        reaction_available: true,
        movement_spent_feet: 0,
        sneak_attack_supported: false,
        sneak_attack_used_this_turn: false,
        recharge_available: true,
        spell_slots: BattleSpellSlotLedger::none(),
    }
}

fn rogue_combatant() -> Combatant {
    Combatant {
        hp: 11,
        max_hp: 11,
        temporary_hp: 0,
        armor_class: 14,
        shield_armor_class_bonus_active: false,
        unconscious: false,
        incapacitated: false,
        prone: false,
        creature_size: CreatureSize::Medium,
        lifecycle: CombatantLifecycle::UsesDeathSavingThrows,
        reaction_available: true,
        movement_spent_feet: 0,
        sneak_attack_supported: true,
        sneak_attack_used_this_turn: false,
        recharge_available: false,
        spell_slots: BattleSpellSlotLedger::none(),
    }
}

fn skeleton_combatant() -> Combatant {
    Combatant {
        hp: 13,
        max_hp: 13,
        temporary_hp: 0,
        armor_class: 13,
        shield_armor_class_bonus_active: false,
        unconscious: false,
        incapacitated: false,
        prone: false,
        creature_size: CreatureSize::Medium,
        lifecycle: CombatantLifecycle::DiesAtZeroHitPoints,
        reaction_available: true,
        movement_spent_feet: 0,
        sneak_attack_supported: false,
        sneak_attack_used_this_turn: false,
        recharge_available: false,
        spell_slots: BattleSpellSlotLedger::none(),
    }
}

#[must_use]
pub fn current_actor(state: &BattleState) -> Actor {
    // QNT: battle-runtime-turn-order.qnt `currentActor`.
    state.initiative.still_to_act.actor
}

#[must_use]
pub fn stat_block_multiattack_dispatches_available(state: &BattleState) -> i16 {
    state.stat_block_control.pending_primary_dispatches
        + state.stat_block_control.pending_secondary_dispatches
}

#[must_use]
pub fn end_turn(state: BattleState) -> BattleState {
    // RAW: cleanroom-input/raw/srd-5.2.1/Playing-the-Game.md
    // "The Order of Combat"; QNT: battle-runtime-turn-advancement.qnt
    // `endTurn`, narrowed to the current reducer-spine combatants and the
    // T062 lifecycle fixture effects.
    let previous_round = state.initiative.round;
    let current = current_actor(&state);
    let state = apply_turn_end_boundary_effects(state, current);
    let initiative = next_initiative(state.initiative.clone());
    let next_actor = initiative.still_to_act.actor;
    let state = BattleState {
        initiative,
        action_available: true,
        bonus_action_available: true,
        spell_slot_uses_this_turn: Vec::new(),
        level_one_plus_spell_casters_this_turn: Vec::new(),
        quickened_level_one_plus_spell_casters_this_turn: Vec::new(),
        attack_roll_made_this_turn: false,
        dash_movement_bonus_feet: 0,
        disengaged: false,
        ..state
    };
    let state = reset_start_turn_combatant(state, next_actor);
    let state = apply_start_turn_boundary_effects(state, next_actor);

    if state.initiative.round > previous_round {
        tick_round_duration_boundary_effects(state)
    } else {
        state
    }
}

fn next_initiative(initiative: Initiative) -> Initiative {
    // QNT: battle-runtime-turn-order.qnt `nextInitiative`.
    let current = initiative.still_to_act.actor;
    let acted = {
        let mut already_acted = initiative.already_acted.clone();
        already_acted.push(current);
        already_acted
    };

    if let Some((actor, waiting)) = initiative.still_to_act.waiting.split_first() {
        Initiative {
            round: initiative.round,
            already_acted: acted,
            still_to_act: InitiativeStillToAct {
                actor: *actor,
                waiting: waiting.to_vec(),
            },
        }
    } else {
        let mut next_round_order = initiative.already_acted;
        next_round_order.push(current);
        let Some((actor, waiting)) = next_round_order.split_first() else {
            return Initiative {
                round: initiative.round + 1,
                already_acted: Vec::new(),
                still_to_act: InitiativeStillToAct {
                    actor: current,
                    waiting: Vec::new(),
                },
            };
        };
        Initiative {
            round: initiative.round + 1,
            already_acted: Vec::new(),
            still_to_act: InitiativeStillToAct {
                actor: *actor,
                waiting: waiting.to_vec(),
            },
        }
    }
}

fn reset_start_turn_combatant(mut state: BattleState, actor: Actor) -> BattleState {
    // QNT: battle-runtime-turn-advancement.qnt `battleRuntimeStartTurn`
    // projection fields used by `endTurn`.
    let combatant = combatant_for_mut(&mut state, actor);
    combatant.reaction_available = true;
    combatant.movement_spent_feet = 0;
    combatant.sneak_attack_used_this_turn = false;
    state
}

fn apply_turn_end_boundary_effects(state: BattleState, actor: Actor) -> BattleState {
    if actor != Actor::Goblin {
        return state;
    }

    let mut state = if state.turn_boundary_effects.turn_end_damage_active {
        with_damaged_target(state, Actor::Goblin, 3)
    } else {
        state
    };

    state.turn_boundary_effects.turn_end_damage_active = false;
    state.turn_boundary_effects.end_turn_ongoing_feature_active = false;
    state
}

fn apply_start_turn_boundary_effects(state: BattleState, actor: Actor) -> BattleState {
    match actor {
        Actor::Goblin if state.turn_boundary_effects.turn_start_damage_active => {
            with_damaged_target(state, Actor::Goblin, 2)
        }
        Actor::Fighter => expire_source_next_turn_boundary_effects(state),
        Actor::Goblin | Actor::Rogue | Actor::Skeleton => state,
    }
}

fn expire_source_next_turn_boundary_effects(mut state: BattleState) -> BattleState {
    state.turn_boundary_effects.until_next_turn_active = false;
    state
        .turn_boundary_effects
        .start_turn_ongoing_feature_active = false;
    state.turn_boundary_effects.turn_start_damage_active = false;
    state
}

fn tick_round_duration_boundary_effects(state: BattleState) -> BattleState {
    // QNT: battle-runtime-turn-advancement.qnt ticks round-duration spell
    // effects after the initiative round advances. The T062 fixture has no
    // separate stored round-duration counter after source start-turn expiry.
    state
}

#[must_use]
pub fn discover_battle_acts(state: &BattleState) -> Vec<AvailableBattleAct> {
    // QNT: battle-runtime-combat-holes.qnt `combatOpenHoles`, narrowed to the
    // current weapon-attack and Skeleton multiattack paths for this
    // reducer-spine experiment.
    match current_actor(state) {
        Actor::Fighter | Actor::Rogue if current_actor_can_attack(state) => {
            vec![AvailableBattleAct {
                subject: BattleSubject {
                    kind: BattleSubjectKind::WeaponAttack,
                    actor: current_actor(state),
                    target: None,
                    stage: WeaponAttackFrontierStage::TargetChoice,
                    damage_modifier: weapon_damage_modifier_for(current_actor(state)),
                },
                holes: weapon_attack_hole_frontier(WeaponAttackFrontierStage::TargetChoice),
            }]
        }
        Actor::Skeleton if state.action_available && state.skeleton.hp > 0 => {
            vec![AvailableBattleAct {
                subject: BattleSubject {
                    kind: BattleSubjectKind::Multiattack,
                    actor: Actor::Skeleton,
                    target: None,
                    stage: WeaponAttackFrontierStage::Resolved,
                    damage_modifier: 0,
                },
                holes: Vec::new(),
            }]
        }
        Actor::Fighter | Actor::Goblin | Actor::Rogue | Actor::Skeleton => Vec::new(),
    }
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

    match (subject.kind, fill) {
        (BattleSubjectKind::WeaponAttack, fill) => {
            if !state.action_available {
                return invalid(
                    state,
                    BattleResolutionInvalidReason::StaleSubject,
                    WeaponAttackFrontierStage::Resolved,
                );
            }
            if subject.stage == WeaponAttackFrontierStage::Resolved {
                return invalid(
                    state,
                    BattleResolutionInvalidReason::StaleSubject,
                    WeaponAttackFrontierStage::Resolved,
                );
            }
            match fill {
                BattleFill::TargetChoice(target) => resolve_target_choice(state, subject, target),
                BattleFill::AttackRoll(roll) => resolve_attack_roll_fill(state, subject, roll),
                BattleFill::DamageRoll(rolled_damage) => {
                    resolve_damage_roll(state, subject, rolled_damage, false)
                }
                BattleFill::SneakAttackDamageRoll(rolled_damage) => {
                    resolve_damage_roll(state, subject, rolled_damage, true)
                }
                BattleFill::ResolveMultiattack | BattleFill::SpendMultiattackDispatch => invalid(
                    state,
                    BattleResolutionInvalidReason::InvalidFill,
                    subject.stage,
                ),
            }
        }
        (BattleSubjectKind::Multiattack, BattleFill::ResolveMultiattack) => {
            resolve_multiattack(state, subject)
        }
        (BattleSubjectKind::Multiattack, BattleFill::SpendMultiattackDispatch) => {
            spend_multiattack_dispatch(state, subject)
        }
        (BattleSubjectKind::Multiattack, _) => invalid(
            state,
            BattleResolutionInvalidReason::InvalidFill,
            subject.stage,
        ),
    }
}

#[must_use]
pub fn start_goblin_multiattack_control(state: BattleState) -> StatBlockActionResolutionResult {
    if current_actor(&state) != Actor::Goblin || !state.action_available || state.goblin.hp <= 0 {
        return invalid_stat_block_action(
            state,
            initial_stat_block_action_subject(
                Actor::Goblin,
                StatBlockActionFrontierStage::ActSelection,
                StatBlockActionDamageMode::Rolled,
                false,
            ),
            StatBlockActionResolutionInvalidReason::StaleSubject,
            None,
        );
    }

    let stat_block_control =
        start_stat_block_multiattack_from(state.stat_block_control.clone(), goblin_profile());
    if stat_block_control == state.stat_block_control {
        return invalid_stat_block_action(
            state,
            initial_stat_block_action_subject(
                Actor::Goblin,
                StatBlockActionFrontierStage::ActSelection,
                StatBlockActionDamageMode::Rolled,
                false,
            ),
            StatBlockActionResolutionInvalidReason::StaleSubject,
            None,
        );
    }

    let state = BattleState {
        action_available: stat_block_control.attack_action_available,
        stat_block_control,
        ..state
    };
    let subject = initial_stat_block_action_subject(
        Actor::Goblin,
        StatBlockActionFrontierStage::AttackTargetChoice,
        StatBlockActionDamageMode::Rolled,
        false,
    );
    StatBlockActionResolutionResult::NeedsHoles {
        state,
        subject,
        holes: stat_block_action_hole_frontier(subject.stage),
    }
}

#[must_use]
pub fn discover_goblin_rolled_action_attack_control(
    state: BattleState,
) -> StatBlockActionResolutionResult {
    discover_goblin_stat_block_attack_control(state, StatBlockActionDamageMode::Rolled, true)
}

#[must_use]
pub fn discover_goblin_static_action_attack_control(
    state: BattleState,
) -> StatBlockActionResolutionResult {
    // QNT: battle-runtime-stat-block-multi-damage.mbt.qnt
    // `targetInitialHp` 12 -> `targetHpAfterStaticPiercingOnly` 9.
    discover_goblin_stat_block_attack_control(
        state,
        StatBlockActionDamageMode::Static { damage: 3 },
        false,
    )
}

#[must_use]
pub fn discover_goblin_prone_rider_attack_control(
    state: BattleState,
    target_prone_condition_immune: bool,
) -> StatBlockActionResolutionResult {
    if current_actor(&state) != Actor::Goblin || !state.action_available || state.goblin.hp <= 0 {
        return invalid_stat_block_action(
            state,
            initial_stat_block_action_subject_with_prone_rider(
                Actor::Goblin,
                StatBlockActionFrontierStage::ActSelection,
                StatBlockActionDamageMode::Rolled,
                false,
                StatBlockProneOnHitRider::MediumOrSmaller {
                    target_prone_condition_immune,
                },
            ),
            StatBlockActionResolutionInvalidReason::StaleSubject,
            None,
        );
    }

    let subject = initial_stat_block_action_subject_with_prone_rider(
        Actor::Goblin,
        StatBlockActionFrontierStage::AttackTargetChoice,
        StatBlockActionDamageMode::Rolled,
        false,
        StatBlockProneOnHitRider::MediumOrSmaller {
            target_prone_condition_immune,
        },
    );
    StatBlockActionResolutionResult::NeedsHoles {
        state,
        subject,
        holes: stat_block_action_hole_frontier(subject.stage),
    }
}

#[must_use]
pub fn spend_goblin_recharge_gated_rolled_attack(
    state: BattleState,
) -> StatBlockActionResolutionResult {
    if current_actor(&state) != Actor::Goblin
        || !state.action_available
        || !state.goblin.recharge_available
        || state.goblin.hp <= 0
    {
        return invalid_stat_block_action(
            state,
            initial_stat_block_action_subject(
                Actor::Goblin,
                StatBlockActionFrontierStage::ActSelection,
                StatBlockActionDamageMode::Rolled,
                false,
            ),
            StatBlockActionResolutionInvalidReason::StaleSubject,
            None,
        );
    }

    let mut goblin = state.goblin;
    goblin.recharge_available = false;
    let state = BattleState {
        goblin,
        action_available: false,
        ..state
    };
    let subject = initial_stat_block_action_subject(
        Actor::Goblin,
        StatBlockActionFrontierStage::RechargeRoll,
        StatBlockActionDamageMode::Rolled,
        false,
    );
    StatBlockActionResolutionResult::NeedsHoles {
        state,
        subject,
        holes: stat_block_action_hole_frontier(subject.stage),
    }
}

#[must_use]
pub fn resolve_stat_block_action_subject(
    state: BattleState,
    subject: StatBlockActionSubject,
    fill: StatBlockActionFill,
) -> StatBlockActionResolutionResult {
    if subject.actor != current_actor(&state) {
        return invalid_stat_block_action(
            state,
            subject,
            StatBlockActionResolutionInvalidReason::WrongActor,
            None,
        );
    }

    match fill {
        StatBlockActionFill::TargetChoice(target) => {
            resolve_stat_block_target_choice(state, subject, target)
        }
        StatBlockActionFill::AttackRoll(roll) => {
            resolve_stat_block_attack_roll_fill(state, subject, roll)
        }
        StatBlockActionFill::DamageDice(rolled_damage) => {
            resolve_stat_block_damage_dice(state, subject, rolled_damage)
        }
        StatBlockActionFill::RechargeRoll(roll) => {
            resolve_stat_block_recharge_roll(state, subject, roll)
        }
    }
}

#[must_use]
pub fn stat_block_action_projection_from_result(
    result: &StatBlockActionResolutionResult,
) -> StatBlockActionOrderingState {
    let (state, subject, runtime_result, last_ordering_error) = match result {
        StatBlockActionResolutionResult::NeedsHoles { state, subject, .. } => (
            state,
            subject,
            StatBlockActionRuntimeResult::NeedsHoles,
            None,
        ),
        StatBlockActionResolutionResult::Resolved { state, subject } => {
            (state, subject, StatBlockActionRuntimeResult::Resolved, None)
        }
        StatBlockActionResolutionResult::Invalid {
            state,
            subject,
            last_ordering_error,
            ..
        } => (
            state,
            subject,
            StatBlockActionRuntimeResult::Invalid,
            *last_ordering_error,
        ),
    };

    stat_block_action_ordering_projection(StatBlockActionOrderingProjectionFacts {
        stage: subject.stage,
        runtime_result,
        last_ordering_error,
        multiattack_dispatches_available: stat_block_multiattack_dispatches_available(state),
        recharge_action_available: subject.recharge_action_available,
        uses_rolled_damage: stat_block_subject_uses_rolled_damage(subject),
    })
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
        Actor::Rogue => !state.rogue.unconscious && !state.rogue.incapacitated,
        Actor::Skeleton => state.skeleton.hp > 0,
    }
}

fn weapon_damage_modifier_for(actor: Actor) -> i16 {
    match actor {
        Actor::Rogue => 3,
        Actor::Fighter | Actor::Goblin | Actor::Skeleton => 0,
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
    use_sneak_attack: bool,
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

    let damage = rolled_damage + subject.damage_modifier;
    let state = if use_sneak_attack {
        with_sneak_attack_used(state, subject.actor)
    } else {
        state
    };
    let state = with_damaged_target(state, target, damage);
    BattleResolutionResult::Resolved {
        state: BattleState {
            action_available: false,
            ..state
        },
    }
}

fn resolve_multiattack(state: BattleState, subject: BattleSubject) -> BattleResolutionResult {
    if !state.action_available
        || subject.actor != Actor::Skeleton
        || current_actor(&state) != Actor::Skeleton
        || stat_block_multiattack_continuation_open(&state.stat_block_control)
    {
        return invalid(
            state,
            BattleResolutionInvalidReason::StaleSubject,
            subject.stage,
        );
    }

    let stat_block_control =
        start_stat_block_multiattack_from(state.stat_block_control.clone(), skeleton_profile());
    if stat_block_control == state.stat_block_control {
        return invalid(
            state,
            BattleResolutionInvalidReason::StaleSubject,
            subject.stage,
        );
    }

    BattleResolutionResult::Resolved {
        state: BattleState {
            action_available: stat_block_control.attack_action_available,
            stat_block_control,
            ..state
        },
    }
}

fn spend_multiattack_dispatch(
    state: BattleState,
    subject: BattleSubject,
) -> BattleResolutionResult {
    if subject.actor != Actor::Skeleton
        || current_actor(&state) != Actor::Skeleton
        || !stat_block_multiattack_continuation_open(&state.stat_block_control)
    {
        return invalid(
            state,
            BattleResolutionInvalidReason::StaleSubject,
            subject.stage,
        );
    }

    let stat_block_control = resolve_stat_block_control_subject(
        state.stat_block_control.clone(),
        StatBlockDispatchSubject::PrimaryAttackSlot,
    );
    if stat_block_control == state.stat_block_control {
        return invalid(
            state,
            BattleResolutionInvalidReason::StaleSubject,
            subject.stage,
        );
    }

    BattleResolutionResult::Resolved {
        state: BattleState {
            stat_block_control,
            ..state
        },
    }
}

fn discover_goblin_stat_block_attack_control(
    state: BattleState,
    damage_mode: StatBlockActionDamageMode,
    recharge_action_available: bool,
) -> StatBlockActionResolutionResult {
    if current_actor(&state) != Actor::Goblin || !state.action_available || state.goblin.hp <= 0 {
        return invalid_stat_block_action(
            state,
            initial_stat_block_action_subject(
                Actor::Goblin,
                StatBlockActionFrontierStage::ActSelection,
                damage_mode,
                recharge_action_available,
            ),
            StatBlockActionResolutionInvalidReason::StaleSubject,
            None,
        );
    }

    let subject = initial_stat_block_action_subject(
        Actor::Goblin,
        StatBlockActionFrontierStage::AttackTargetChoice,
        damage_mode,
        recharge_action_available,
    );
    StatBlockActionResolutionResult::NeedsHoles {
        state,
        subject,
        holes: stat_block_action_hole_frontier(subject.stage),
    }
}

fn resolve_stat_block_target_choice(
    state: BattleState,
    subject: StatBlockActionSubject,
    target: Actor,
) -> StatBlockActionResolutionResult {
    let order = stat_block_action_fill_order_result(
        subject.stage,
        StatBlockActionFillKind::TargetChoice,
        false,
        stat_block_subject_uses_rolled_damage(&subject),
    );
    let next_stage = match stat_block_order_transition(order) {
        StatBlockOrderTransition::Accepted(stage) => stage,
        StatBlockOrderTransition::Rejected(error) => {
            return invalid_stat_block_action(
                state,
                subject,
                StatBlockActionResolutionInvalidReason::InvalidFill,
                Some(error),
            );
        }
        StatBlockOrderTransition::NotOrdering(stage) => {
            return stat_block_action_needs_holes(
                state,
                StatBlockActionSubject { stage, ..subject },
            );
        }
    };

    if target == subject.actor {
        return invalid_stat_block_action(
            state,
            subject,
            StatBlockActionResolutionInvalidReason::WrongTarget,
            None,
        );
    }

    let subject = StatBlockActionSubject {
        target: Some(target),
        stage: next_stage,
        ..subject
    };
    stat_block_action_needs_holes(state, subject)
}

fn resolve_stat_block_attack_roll_fill(
    state: BattleState,
    subject: StatBlockActionSubject,
    roll: AttackRollFacts,
) -> StatBlockActionResolutionResult {
    let attack_roll_hits = subject
        .target
        .map(|target| resolve_attack_roll(roll, armor_class_for(&state, target), 20).hits)
        .unwrap_or(true);
    let order = stat_block_action_fill_order_result(
        subject.stage,
        StatBlockActionFillKind::AttackRoll,
        attack_roll_hits,
        stat_block_subject_uses_rolled_damage(&subject),
    );
    let next_stage = match stat_block_order_transition(order) {
        StatBlockOrderTransition::Accepted(stage) => stage,
        StatBlockOrderTransition::Rejected(error) => {
            return invalid_stat_block_action(
                state,
                subject,
                StatBlockActionResolutionInvalidReason::InvalidFill,
                Some(error),
            );
        }
        StatBlockOrderTransition::NotOrdering(stage) => {
            return stat_block_action_needs_holes(
                state,
                StatBlockActionSubject { stage, ..subject },
            );
        }
    };

    let Some(target) = subject.target else {
        return invalid_stat_block_action(
            state,
            subject,
            StatBlockActionResolutionInvalidReason::InvalidFill,
            None,
        );
    };

    let state = BattleState {
        attack_roll_made_this_turn: true,
        ..state
    };
    let state = if attack_roll_hits {
        with_stat_block_prone_on_hit_rider(state, target, subject.prone_on_hit_rider)
    } else {
        state
    };
    let subject = StatBlockActionSubject {
        stage: next_stage,
        recharge_action_available: if next_stage == StatBlockActionFrontierStage::Resolved {
            false
        } else {
            subject.recharge_action_available
        },
        ..subject
    };

    if next_stage == StatBlockActionFrontierStage::Resolved {
        let state = match subject.damage_mode {
            StatBlockActionDamageMode::Rolled => state,
            StatBlockActionDamageMode::Static { damage } if attack_roll_hits => {
                with_damaged_target(state, target, damage)
            }
            StatBlockActionDamageMode::Static { .. } => state,
        };
        return StatBlockActionResolutionResult::Resolved {
            state: BattleState {
                action_available: false,
                ..state
            },
            subject,
        };
    }

    stat_block_action_needs_holes(state, subject)
}

fn resolve_stat_block_damage_dice(
    state: BattleState,
    subject: StatBlockActionSubject,
    rolled_damage: i16,
) -> StatBlockActionResolutionResult {
    let order = stat_block_action_fill_order_result(
        subject.stage,
        StatBlockActionFillKind::RolledDice,
        true,
        stat_block_subject_uses_rolled_damage(&subject),
    );
    let next_stage = match stat_block_order_transition(order) {
        StatBlockOrderTransition::Accepted(stage) => stage,
        StatBlockOrderTransition::Rejected(error) => {
            return invalid_stat_block_action(
                state,
                subject,
                StatBlockActionResolutionInvalidReason::InvalidFill,
                Some(error),
            );
        }
        StatBlockOrderTransition::NotOrdering(stage) => {
            return stat_block_action_needs_holes(
                state,
                StatBlockActionSubject { stage, ..subject },
            );
        }
    };

    let Some(target) = subject.target else {
        return invalid_stat_block_action(
            state,
            subject,
            StatBlockActionResolutionInvalidReason::InvalidFill,
            None,
        );
    };

    let state = with_damaged_target(state, target, rolled_damage);
    StatBlockActionResolutionResult::Resolved {
        state: BattleState {
            action_available: false,
            ..state
        },
        subject: StatBlockActionSubject {
            stage: next_stage,
            recharge_action_available: false,
            ..subject
        },
    }
}

fn resolve_stat_block_recharge_roll(
    state: BattleState,
    subject: StatBlockActionSubject,
    roll: i16,
) -> StatBlockActionResolutionResult {
    let order = stat_block_action_fill_order_result(
        subject.stage,
        StatBlockActionFillKind::StatBlockRechargeRoll,
        false,
        false,
    );
    let next_stage = match stat_block_order_transition(order) {
        StatBlockOrderTransition::Accepted(stage) => stage,
        StatBlockOrderTransition::Rejected(error) => {
            return invalid_stat_block_action(
                state,
                subject,
                StatBlockActionResolutionInvalidReason::InvalidFill,
                Some(error),
            );
        }
        StatBlockOrderTransition::NotOrdering(stage) => {
            return stat_block_action_needs_holes(
                state,
                StatBlockActionSubject { stage, ..subject },
            );
        }
    };

    let recharged = roll >= 5;
    let mut goblin = state.goblin;
    goblin.recharge_available = recharged;
    StatBlockActionResolutionResult::Resolved {
        state: BattleState { goblin, ..state },
        subject: StatBlockActionSubject {
            stage: next_stage,
            recharge_action_available: recharged,
            ..subject
        },
    }
}

fn skeleton_profile() -> StatBlockMultiattackProfile {
    StatBlockMultiattackProfile {
        first_attack_slot: StatBlockAttackSlot::Primary,
        primary_attack_count: 2,
        secondary_attack_count: 0,
    }
}

fn goblin_profile() -> StatBlockMultiattackProfile {
    StatBlockMultiattackProfile {
        first_attack_slot: StatBlockAttackSlot::Primary,
        primary_attack_count: 3,
        secondary_attack_count: 0,
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

fn stat_block_order_transition(result: StatBlockActionFillOrderResult) -> StatBlockOrderTransition {
    match result {
        StatBlockActionFillOrderResult::Accepted(stage) => {
            StatBlockOrderTransition::Accepted(stage)
        }
        StatBlockActionFillOrderResult::Rejected(error) => {
            StatBlockOrderTransition::Rejected(error)
        }
        StatBlockActionFillOrderResult::NotOrderingError(stage) => {
            StatBlockOrderTransition::NotOrdering(stage)
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

fn initial_stat_block_action_subject(
    actor: Actor,
    stage: StatBlockActionFrontierStage,
    damage_mode: StatBlockActionDamageMode,
    recharge_action_available: bool,
) -> StatBlockActionSubject {
    initial_stat_block_action_subject_with_prone_rider(
        actor,
        stage,
        damage_mode,
        recharge_action_available,
        StatBlockProneOnHitRider::NoProneRider,
    )
}

fn initial_stat_block_action_subject_with_prone_rider(
    actor: Actor,
    stage: StatBlockActionFrontierStage,
    damage_mode: StatBlockActionDamageMode,
    recharge_action_available: bool,
    prone_on_hit_rider: StatBlockProneOnHitRider,
) -> StatBlockActionSubject {
    StatBlockActionSubject {
        actor,
        target: None,
        stage,
        damage_mode,
        prone_on_hit_rider,
        recharge_action_available,
    }
}

fn stat_block_subject_uses_rolled_damage(subject: &StatBlockActionSubject) -> bool {
    subject.damage_mode == StatBlockActionDamageMode::Rolled
}

fn stat_block_action_needs_holes(
    state: BattleState,
    subject: StatBlockActionSubject,
) -> StatBlockActionResolutionResult {
    StatBlockActionResolutionResult::NeedsHoles {
        state,
        subject,
        holes: stat_block_action_hole_frontier(subject.stage),
    }
}

fn invalid_stat_block_action(
    state: BattleState,
    subject: StatBlockActionSubject,
    reason: StatBlockActionResolutionInvalidReason,
    last_ordering_error: Option<StatBlockActionFillOrderingError>,
) -> StatBlockActionResolutionResult {
    StatBlockActionResolutionResult::Invalid {
        state,
        subject,
        reason,
        holes: stat_block_action_hole_frontier(subject.stage),
        last_ordering_error,
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

fn start_spell_attack_subject(
    mut state: BattleState,
    requires_spell_slot: bool,
    stage: SpellAttackFrontierStage,
) -> BattleState {
    let actor = current_actor(&state);
    if !state.action_available {
        return state;
    }
    state.action_available = false;
    state.spell_attack_procedure = BattleSpellAttackProcedure::Active(BattleSpellAttackSubject {
        actor,
        target: None,
        stage,
        requires_spell_slot,
        last_ordering_error: None,
    });
    state
}

fn spell_attack_fill_kind(fill: BattleSpellAttackFill) -> SpellAttackFillKind {
    match fill {
        BattleSpellAttackFill::TargetChoice(_) => SpellAttackFillKind::TargetChoice,
        BattleSpellAttackFill::DamageTypeChoice => SpellAttackFillKind::DamageTypeChoice,
        BattleSpellAttackFill::AttackRoll(_) => SpellAttackFillKind::AttackRoll,
        BattleSpellAttackFill::DamageRoll(_) => SpellAttackFillKind::RolledDice,
    }
}

fn spell_attack_fill_attack_hits(
    state: &BattleState,
    subject: &BattleSpellAttackSubject,
    fill: BattleSpellAttackFill,
) -> bool {
    let BattleSpellAttackFill::AttackRoll(facts) = fill else {
        return true;
    };
    subject
        .target
        .map(|target| resolve_attack_roll(facts, armor_class_for(state, target), 20).hits)
        .unwrap_or(true)
}

fn spell_attack_result_stage(result: SpellAttackFillOrderResult) -> SpellAttackFrontierStage {
    match result {
        SpellAttackFillOrderResult::Accepted(stage)
        | SpellAttackFillOrderResult::RequestedEarlier { stage, .. }
        | SpellAttackFillOrderResult::NotOrderingError(stage) => stage,
    }
}

fn spell_attack_target_after_fill(
    target: Option<Actor>,
    fill: BattleSpellAttackFill,
    result: SpellAttackFillOrderResult,
) -> Option<Actor> {
    match (fill, result) {
        (
            BattleSpellAttackFill::TargetChoice(selected),
            SpellAttackFillOrderResult::Accepted(_),
        ) => Some(selected),
        _ => target,
    }
}

fn finalize_spell_attack_subject(
    state: BattleState,
    subject: BattleSpellAttackSubject,
) -> BattleState {
    if !subject.requires_spell_slot {
        return state;
    }
    let state = expend_combatant_spell_slot(state, subject.actor, BattleSpellSlotLevel::First);
    commit_actor_spell_slot_use(state, subject.actor)
}

fn armor_class_for(state: &BattleState, actor: Actor) -> i16 {
    let combatant = match actor {
        Actor::Fighter => state.fighter.armor_class,
        Actor::Goblin => state.goblin.armor_class,
        Actor::Rogue => state.rogue.armor_class,
        Actor::Skeleton => state.skeleton.armor_class,
    };
    let shield_bonus = if combatant_for(state, actor).shield_armor_class_bonus_active {
        5
    } else {
        0
    };
    combatant + shield_bonus
}

fn apply_shield_armor_class_bonus(mut state: BattleState, actor: Actor) -> BattleState {
    // QNT: battle-runtime-reaction-window.qnt `applyShieldReactionSpell`.
    match actor {
        Actor::Fighter => state.fighter.shield_armor_class_bonus_active = true,
        Actor::Goblin => state.goblin.shield_armor_class_bonus_active = true,
        Actor::Rogue => state.rogue.shield_armor_class_bonus_active = true,
        Actor::Skeleton => state.skeleton.shield_armor_class_bonus_active = true,
    }
    state
}

fn with_damaged_target(mut state: BattleState, target: Actor, damage: i16) -> BattleState {
    let target_combatant = combatant_for(&state, target);
    let result =
        apply_resolved_damage_to_positive_hit_points(combatant_vitals(target_combatant), damage);
    let damaged = with_vitals(target_combatant, result.vitals);
    match target {
        Actor::Fighter => state.fighter = damaged,
        Actor::Goblin => state.goblin = damaged,
        Actor::Rogue => state.rogue = damaged,
        Actor::Skeleton => state.skeleton = damaged,
    }
    state
}

fn spend_reaction(mut state: BattleState, actor: Actor) -> BattleState {
    combatant_for_mut(&mut state, actor).reaction_available = false;
    state
}

fn can_actor_expend_spell_slot_this_turn(state: &BattleState, actor: Actor) -> bool {
    !state
        .spell_slot_uses_this_turn
        .iter()
        .any(|slot_use| battle_turn_spell_slot_use_actor(*slot_use) == actor)
}

fn battle_turn_spell_slot_use_actor(slot_use: BattleTurnSpellSlotUse) -> Actor {
    match slot_use {
        BattleTurnSpellSlotUse::Pending(actor) | BattleTurnSpellSlotUse::Committed(actor) => actor,
    }
}

fn claim_pending_actor_spell_slot_use(mut state: BattleState, actor: Actor) -> BattleState {
    // QNT: battle-runtime-spell-invocation.qnt
    // `claimPendingActorSpellSlotUse`.
    if can_actor_expend_spell_slot_this_turn(&state, actor) {
        state
            .spell_slot_uses_this_turn
            .push(BattleTurnSpellSlotUse::Pending(actor));
    }
    state
}

fn release_pending_actor_spell_slot_use(mut state: BattleState, actor: Actor) -> BattleState {
    // QNT: battle-runtime-spell-invocation.qnt
    // `releasePendingActorSpellSlotUse`.
    state
        .spell_slot_uses_this_turn
        .retain(|slot_use| *slot_use != BattleTurnSpellSlotUse::Pending(actor));
    state
}

fn commit_actor_spell_slot_use(state: BattleState, actor: Actor) -> BattleState {
    // QNT: battle-runtime-spell-invocation.qnt `commitActorSpellSlotUse`.
    if state
        .spell_slot_uses_this_turn
        .contains(&BattleTurnSpellSlotUse::Committed(actor))
    {
        return state;
    }
    let mut state = release_pending_actor_spell_slot_use(state, actor);
    state
        .spell_slot_uses_this_turn
        .push(BattleTurnSpellSlotUse::Committed(actor));
    if !state
        .level_one_plus_spell_casters_this_turn
        .contains(&actor)
    {
        state.level_one_plus_spell_casters_this_turn.push(actor);
    }
    state
}

fn expend_combatant_spell_slot(
    mut state: BattleState,
    actor: Actor,
    slot_level: BattleSpellSlotLevel,
) -> BattleState {
    let ledger = &mut combatant_for_mut(&mut state, actor).spell_slots;
    match slot_level {
        BattleSpellSlotLevel::First => ledger.first_level_expended += 1,
        BattleSpellSlotLevel::Second => ledger.second_level_expended += 1,
        BattleSpellSlotLevel::Third => ledger.third_level_expended += 1,
        BattleSpellSlotLevel::Fourth => ledger.fourth_level_expended += 1,
    }
    state
}

fn with_stat_block_prone_on_hit_rider(
    state: BattleState,
    target: Actor,
    rider: StatBlockProneOnHitRider,
) -> BattleState {
    match rider {
        StatBlockProneOnHitRider::NoProneRider => state,
        StatBlockProneOnHitRider::MediumOrSmaller {
            target_prone_condition_immune: true,
        } => state,
        StatBlockProneOnHitRider::MediumOrSmaller {
            target_prone_condition_immune: false,
        } if combatant_for(&state, target)
            .creature_size
            .is_medium_or_smaller() =>
        {
            with_prone_target(state, target)
        }
        StatBlockProneOnHitRider::MediumOrSmaller { .. } => state,
    }
}

fn with_prone_target(mut state: BattleState, target: Actor) -> BattleState {
    match target {
        Actor::Fighter => state.fighter.prone = true,
        Actor::Goblin => state.goblin.prone = true,
        Actor::Rogue => state.rogue.prone = true,
        Actor::Skeleton => state.skeleton.prone = true,
    }
    state
}

fn with_sneak_attack_used(mut state: BattleState, actor: Actor) -> BattleState {
    match actor {
        Actor::Fighter => state.fighter.sneak_attack_used_this_turn = true,
        Actor::Goblin => state.goblin.sneak_attack_used_this_turn = true,
        Actor::Rogue => state.rogue.sneak_attack_used_this_turn = true,
        Actor::Skeleton => state.skeleton.sneak_attack_used_this_turn = true,
    }
    state
}

fn combatant_for(state: &BattleState, actor: Actor) -> Combatant {
    match actor {
        Actor::Fighter => state.fighter,
        Actor::Goblin => state.goblin,
        Actor::Rogue => state.rogue,
        Actor::Skeleton => state.skeleton,
    }
}

fn combatant_for_mut(state: &mut BattleState, actor: Actor) -> &mut Combatant {
    match actor {
        Actor::Fighter => &mut state.fighter,
        Actor::Goblin => &mut state.goblin,
        Actor::Rogue => &mut state.rogue,
        Actor::Skeleton => &mut state.skeleton,
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
