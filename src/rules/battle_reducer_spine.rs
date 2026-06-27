//! Experimental reducer-spine assembly from copied QNT support modules.
//!
//! This module is not accepted battle-runtime parity evidence by itself. It is a
//! tracked experiment proving that one durable `BattleState` path can be built
//! from QNT before an MBT adapter is wired through the same entrypoints.

use crate::rules::armor_class::{
    armor_class_projection, ArmorClassAbility, ArmorClassFacts, ArmorClassFormula, ArmorClassOption,
};
use crate::rules::attack_damage_disposition::{
    apply_resolved_damage_to_positive_hit_points, CreatureKind, CreatureVitals,
};
use crate::rules::battle_features::{
    dragonborn_breath_weapon_initial_state, innate_sorcery_initial_state,
    resolve_adrenaline_rush_bonus_action_dash, resolve_dragonborn_breath_weapon,
    sorcerer_spell_save_dc, AdrenalineRushFacts, AdrenalineRushRejection, AdrenalineRushResult,
    BattleTurnFeatureState, BreathWeaponAreaShape, DragonbornBreathWeaponFacts,
    DragonbornBreathWeaponProtocol, DragonbornBreathWeaponScenarioOutcome,
    DragonbornBreathWeaponState, InnateSorceryOccurrence, InnateSorceryProtocol,
    InnateSorceryScenarioOutcome, InnateSorcerySpellAttackRollMode,
    InnateSorcerySpellBenefitEligibility, InnateSorcerySpellFacts, InnateSorceryState,
};
use crate::rules::command_options::{
    command_fill_order_accepted_stage, command_fill_order_error, command_fill_order_result,
    command_fill_order_runtime_result, command_hole_frontier, command_next_turn_initial_state,
    command_ordering_initial_state, command_ordering_projection, CommandFillKind,
    CommandFillOrderingError, CommandFrontierStage, CommandHoleKind, CommandNextTurnInvalidReason,
    CommandNextTurnOption, CommandNextTurnProtocol, CommandNextTurnScenario,
    CommandOrderingFillFacts, CommandOrderingProjectionFacts, CommandOrderingRuntimeResult,
    CommandOrderingState, CommandTurnActor,
};
use crate::rules::concentration::{
    cast_concentration_spell, cast_replacement_concentration_spell, concentration_initial_state,
    fail_concentration_saving_throw, request_concentration_save_after_damage,
    voluntarily_end_concentration, ConcentrationDamageFacts, ConcentrationProtocol,
    ConcentrationSavingThrowFacts, ConcentrationState,
};
use crate::rules::creature_size::CreatureSize;
use crate::rules::feature_resources::{
    resource_pool_remaining, spend_resource_pool, ResourcePoolFacts,
};
use crate::rules::hit_point_restoration_ordering::{
    hit_point_restoration_fill_order_accepted_stage, hit_point_restoration_fill_order_error,
    hit_point_restoration_fill_order_result, hit_point_restoration_fill_order_runtime_result,
    hit_point_restoration_hole_frontier, hit_point_restoration_projection,
    HitPointRestorationFillKind, HitPointRestorationFillOrderResult,
    HitPointRestorationFillOrderingError, HitPointRestorationFrontierStage,
    HitPointRestorationHoleKind, HitPointRestorationProjectionFacts,
    HitPointRestorationRuntimeResult, HitPointRestorationState,
};
use crate::rules::hit_points::{
    death_saving_throw_initial_state, discover_death_saving_throw, fill_death_saving_throw,
    DeathSavingThrowFacts, DeathSavingThrowInvalidReason, DeathSavingThrowProtocol,
    DeathSavingThrowState, DeathSavingThrowTurnRole,
};
use crate::rules::interrupt_stack_resume::{
    decline_reaction_window, interrupt_stack_resume_projection, offer_reaction_window,
    reaction_window_depth, recorded_attack_replay_from_root_equivalent, take_matching_reaction,
    InterruptPendingTrigger, InterruptResumeHole, InterruptStackResumeProjectionFacts,
    InterruptStackResumeScenarioOutcome, InterruptStackResumeState, ReactionWindowOffer,
    ReactionWindowRole,
};
use crate::rules::magic_missile::magic_missile_force_damage;
use crate::rules::quickened_spell_governor::{
    QuickenedSpellGovernorState, QuickenedSpellInvalidKind, QuickenedSpellProtocol,
    QuickenedSpellScenarioOutcome, QUICKENED_INITIAL_TARGET_HIT_POINTS,
    QUICKENED_SORCERY_POINT_COST,
};
use crate::rules::roll_modifier_buff_selected_identity::{
    project_bane_failed_save_penalty, project_bless_attack_and_save_modifier,
    project_guidance_skill_ability_check_modifier, project_resistance_matching_damage_reduction,
    project_shield_of_faith_armor_class_bonus, RollModifierBuffSelectedIdentityState,
};
use crate::rules::rule_core_stat_block_controls::{
    resolve_stat_block_control_subject, start_stat_block_multiattack_from,
    stat_block_control_initial_state, stat_block_multiattack_continuation_open,
    StatBlockAttackSlot, StatBlockControlState, StatBlockDispatchSubject,
    StatBlockMultiattackProfile,
};
use crate::rules::save_gated_spell_ordering::{
    save_gated_spell_fill_order_accepted_stage, save_gated_spell_fill_order_error,
    save_gated_spell_fill_order_result, save_gated_spell_fill_order_runtime_result,
    save_gated_spell_hole_frontier, save_gated_spell_ordering_projection, SaveGatedSpellFillKind,
    SaveGatedSpellFillOrderingError, SaveGatedSpellFrontierStage, SaveGatedSpellHoleKind,
    SaveGatedSpellOrderingProjectionFacts, SaveGatedSpellOrderingState,
    SaveGatedSpellRuntimeResult,
};
use crate::rules::scalar_buff::{
    ScalarBuffTargetHole, ScalarBuffTargetInvalidReason, ScalarBuffTargetProtocol,
    ScalarBuffTargetState,
};
use crate::rules::sorcerer_metamagic::{
    CarefulSpellProtocol, CarefulSpellScenarioResult, CarefulSpellState, DistantSpellProtocol,
    DistantSpellScenarioResult, DistantSpellState, EmpoweredSpellProtocol,
    EmpoweredSpellScenarioResult, EmpoweredSpellState, ExtendedSpellConcentrationSaveMode,
    ExtendedSpellProtocol, ExtendedSpellScenarioResult, ExtendedSpellState,
    HeightenedSpellProtocol, HeightenedSpellScenarioResult, HeightenedSpellState,
    QuickenedMetamagicProtocol, QuickenedMetamagicScenarioResult, QuickenedMetamagicState,
    SeekingSpellProtocol, SeekingSpellScenarioResult, SeekingSpellState, SubtleSpellProtocol,
    SubtleSpellScenarioResult, SubtleSpellState, TransmutedSpellProtocol,
    TransmutedSpellScenarioResult, TransmutedSpellState, TwinnedSpellProtocol,
    TwinnedSpellScenarioResult, TwinnedSpellState,
};
use crate::rules::species_passive_traits::{
    creature_space_traversal_allowed, project_dragonborn_damage_resistance,
    project_dwarven_resilience, project_goliath_powerful_build, project_halfling_brave,
    CreatureSpaceTraversalFacts, SpeciesPassiveTraitState,
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
use crate::rules::weapon_mastery_selected_identity::{
    damage_after_weapon_mastery_hit, WeaponMasteryProperty, WeaponMasteryRuntimeHole,
    WeaponMasteryRuntimeOutcome, WeaponMasteryRuntimeProtocol, WeaponMasterySelectedIdentityState,
    WEAPON_MASTERY_TARGET_INITIAL_HIT_POINTS,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Actor {
    Fighter,
    Goblin,
    Rogue,
    Skeleton,
}

const BATTLE_ACTORS: [Actor; 4] = [Actor::Fighter, Actor::Goblin, Actor::Rogue, Actor::Skeleton];

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DeathSavingThrowCount {
    Zero,
    One,
    Two,
}

impl DeathSavingThrowCount {
    #[must_use]
    pub const fn from_unresolved_count(count: i16) -> Self {
        match count {
            i16::MIN..=0 => Self::Zero,
            1 => Self::One,
            _ => Self::Two,
        }
    }

    #[must_use]
    pub const fn as_i16(self) -> i16 {
        match self {
            Self::Zero => 0,
            Self::One => 1,
            Self::Two => 2,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DeathSavingThrowLifecycle {
    MakingDeathSavingThrows {
        successes: DeathSavingThrowCount,
        failures: DeathSavingThrowCount,
    },
    Stable,
    Dead,
}

impl DeathSavingThrowLifecycle {
    #[must_use]
    pub const fn fresh() -> Self {
        Self::MakingDeathSavingThrows {
            successes: DeathSavingThrowCount::Zero,
            failures: DeathSavingThrowCount::Zero,
        }
    }

    #[must_use]
    pub const fn from_state(state: DeathSavingThrowState) -> Self {
        if state.target_dead {
            return Self::Dead;
        }
        if state.target_stable {
            return Self::Stable;
        }
        Self::MakingDeathSavingThrows {
            successes: DeathSavingThrowCount::from_unresolved_count(state.target_death_successes),
            failures: DeathSavingThrowCount::from_unresolved_count(state.target_death_failures),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CombatantLifecycle {
    UsesDeathSavingThrows(DeathSavingThrowLifecycle),
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
pub struct BattleFeatureSubstrates {
    pub weapon_mastery: BattleWeaponMasterySubstrate,
    pub dragonborn_breath_weapon: BattleDragonbornBreathWeaponSubstrate,
    pub innate_sorcery: BattleInnateSorcerySubstrate,
    pub quickened_spell: BattleQuickenedSpellSubstrate,
    pub metamagic_spell: BattleMetamagicSpellSubstrate,
    pub metamagic_option_spell: BattleMetamagicOptionSpellSubstrate,
}

impl BattleFeatureSubstrates {
    #[must_use]
    pub fn standard() -> Self {
        Self {
            weapon_mastery: BattleWeaponMasterySubstrate::initial(),
            dragonborn_breath_weapon: BattleDragonbornBreathWeaponSubstrate::initial(),
            innate_sorcery: BattleInnateSorcerySubstrate::initial(),
            quickened_spell: BattleQuickenedSpellSubstrate::initial(),
            metamagic_spell: BattleMetamagicSpellSubstrate::initial(),
            metamagic_option_spell: BattleMetamagicOptionSpellSubstrate::initial(),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BattleWeaponMasterySubstrate {
    pub sap_target: Option<Actor>,
    pub cleave_used_this_turn: bool,
    pub outcome: WeaponMasteryRuntimeOutcome,
}

impl BattleWeaponMasterySubstrate {
    #[must_use]
    pub const fn initial() -> Self {
        Self {
            sap_target: None,
            cleave_used_this_turn: false,
            outcome: WeaponMasteryRuntimeOutcome::Init,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BattleDragonbornBreathWeaponSubstrate {
    pub uses_remaining: i16,
    pub attack_action_attacks_remaining: i16,
    pub expected_area_shape: BreathWeaponAreaShape,
    pub scenario_outcome: DragonbornBreathWeaponScenarioOutcome,
    pub protocol: DragonbornBreathWeaponProtocol,
}

impl BattleDragonbornBreathWeaponSubstrate {
    #[must_use]
    pub fn initial() -> Self {
        let initial = dragonborn_breath_weapon_initial_state();
        Self {
            uses_remaining: initial.breath_weapon_uses_remaining,
            attack_action_attacks_remaining: initial.attack_action_attacks_remaining,
            expected_area_shape: BreathWeaponAreaShape::Cone15Feet,
            scenario_outcome: initial.scenario_outcome,
            protocol: initial.protocol,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BattleInnateSorcerySubstrate {
    pub base_spell_save_dc: i16,
    pub uses_remaining: i16,
    pub occurrence: InnateSorceryOccurrence,
    pub spell_save_dc_bonus: i16,
    pub spell_attack_roll_mode: InnateSorcerySpellAttackRollMode,
    pub scenario_outcome: InnateSorceryScenarioOutcome,
    pub protocol: InnateSorceryProtocol,
}

impl BattleInnateSorcerySubstrate {
    #[must_use]
    pub fn initial() -> Self {
        let initial = innate_sorcery_initial_state(sorcerer_spell_save_dc(3, 2));
        Self {
            base_spell_save_dc: initial.spell_save_dc,
            uses_remaining: initial.feature_uses_remaining,
            occurrence: initial.occurrence,
            spell_save_dc_bonus: 0,
            spell_attack_roll_mode: initial.spell_attack_roll_mode,
            scenario_outcome: initial.scenario_outcome,
            protocol: initial.protocol,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BattleQuickenedSpellSubstrate {
    pub offered: bool,
    pub color_spray_blinded: bool,
    pub calm_emotions_immunity: bool,
    pub invisibility_active: bool,
    pub bless_active: bool,
    pub spell_slot_committed: bool,
    pub target_active_effect_count: i16,
    pub invalid_kind: QuickenedSpellInvalidKind,
    pub governor_outcome: QuickenedSpellScenarioOutcome,
    pub metamagic_outcome: QuickenedMetamagicScenarioResult,
    pub governor_protocol: QuickenedSpellProtocol,
    pub metamagic_protocol: QuickenedMetamagicProtocol,
}

impl BattleQuickenedSpellSubstrate {
    #[must_use]
    pub const fn initial() -> Self {
        Self {
            offered: false,
            color_spray_blinded: false,
            calm_emotions_immunity: false,
            invisibility_active: false,
            bless_active: false,
            spell_slot_committed: false,
            target_active_effect_count: 0,
            invalid_kind: QuickenedSpellInvalidKind::None,
            governor_outcome: QuickenedSpellScenarioOutcome::Init,
            metamagic_outcome: QuickenedMetamagicScenarioResult::Init,
            governor_protocol: QuickenedSpellProtocol::Init,
            metamagic_protocol: QuickenedMetamagicProtocol::Init,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BattleMetamagicSpellOutcome {
    Init,
    ProtectedSaveGatedDamage,
    ProtectedSaveGatedNoEffect,
    FirstTargetDisadvantageSaveGatedDamage,
    FirstTargetDisadvantageCondition,
    FirstTargetDisadvantageEntrySave,
    FirstTargetDisadvantageEndTurnSave,
    FirstTargetDisadvantageConditionEndTurnSave,
    ObjectRangeLight,
    AdditionalSingleTarget,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BattleMetamagicSpellProtocol {
    Init,
    Resolved,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BattleMetamagicSpellSubstrate {
    pub offered: bool,
    pub protected_save_targets: i16,
    pub first_target_save_disadvantage: bool,
    pub light_emitter_count: i16,
    pub bright_radius_feet: i16,
    pub dim_additional_feet: i16,
    pub target_active_effect_count: i16,
    pub outcome: BattleMetamagicSpellOutcome,
    pub protocol: BattleMetamagicSpellProtocol,
}

impl BattleMetamagicSpellSubstrate {
    #[must_use]
    pub const fn initial() -> Self {
        Self {
            offered: false,
            protected_save_targets: 0,
            first_target_save_disadvantage: false,
            light_emitter_count: 0,
            bright_radius_feet: 0,
            dim_additional_feet: 0,
            target_active_effect_count: 0,
            outcome: BattleMetamagicSpellOutcome::Init,
            protocol: BattleMetamagicSpellProtocol::Init,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BattleMetamagicOptionSpellSubstrate {
    pub offered: bool,
    pub projection: BattleMetamagicOptionSpellProjection,
}

impl BattleMetamagicOptionSpellSubstrate {
    #[must_use]
    pub const fn initial() -> Self {
        Self {
            offered: false,
            projection: BattleMetamagicOptionSpellProjection::Init,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BattleMetamagicOptionSpellProjection {
    Init,
    EmpoweredDamageReroll {
        target_active_effect_count: i16,
    },
    SeekingAttackReroll {
        target_active_effect_count: i16,
    },
    SubtleComponents {
        verbal_suppressed: bool,
        somatic_suppressed: bool,
        material_suppressed: bool,
        material_preserved: bool,
    },
    SubtleUnaffordable,
    TransmutedSaveGatedDamage,
    TransmutedSpellAttack {
        target_active_effect_count: i16,
    },
    ExtendedDuration {
        duration_ticks: i16,
        concentration_saving_throw_mode: ExtendedSpellConcentrationSaveMode,
    },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Combatant {
    pub hp: i16,
    pub max_hp: i16,
    pub temporary_hp: i16,
    pub armor_class: i16,
    pub speed_feet: i16,
    pub shield_armor_class_bonus_active: bool,
    pub unconscious: bool,
    pub incapacitated: bool,
    pub prone: bool,
    pub creature_size: CreatureSize,
    pub lifecycle: CombatantLifecycle,
    pub reaction_available: bool,
    pub movement_spent_feet: i16,
    pub weapon_attack_supported: bool,
    pub weapon_damage_modifier: i16,
    pub multiattack_profile: Option<StatBlockMultiattackProfile>,
    pub sneak_attack_supported: bool,
    pub sneak_attack_used_this_turn: bool,
    pub recharge_available: bool,
    pub spell_slots: BattleSpellSlotLedger,
    pub spell_active_effects: BattleSpellActiveEffects,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BattleArmorClassBaseEffect {
    None,
    Active {
        base_armor_class: i8,
        dexterity_modifier: i8,
        duration_ticks: u16,
    },
}

impl BattleArmorClassBaseEffect {
    #[must_use]
    pub const fn is_active(self) -> bool {
        matches!(self, Self::Active { .. })
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BattleSpellActiveEffectKind {
    None,
    HitPointRegainPrevented,
    NextAttackRollAgainstSelfAdvantage,
    OpportunityAttackDenied,
    Poisoned,
    NextAttackRollDisadvantage,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BattleSpellActiveEffects {
    pub armor_class_base_effect: BattleArmorClassBaseEffect,
    pub hit_point_regain_prevented: bool,
    pub next_attack_roll_against_self_advantage: bool,
    pub opportunity_attack_denied: bool,
    pub poisoned: bool,
    pub next_attack_roll_disadvantage: bool,
}

impl BattleSpellActiveEffects {
    #[must_use]
    pub const fn none() -> Self {
        Self {
            armor_class_base_effect: BattleArmorClassBaseEffect::None,
            hit_point_regain_prevented: false,
            next_attack_roll_against_self_advantage: false,
            opportunity_attack_denied: false,
            poisoned: false,
            next_attack_roll_disadvantage: false,
        }
    }

    #[must_use]
    pub const fn with_effect(effect: BattleSpellActiveEffectKind) -> Self {
        match effect {
            BattleSpellActiveEffectKind::None => Self::none(),
            BattleSpellActiveEffectKind::HitPointRegainPrevented => Self {
                hit_point_regain_prevented: true,
                ..Self::none()
            },
            BattleSpellActiveEffectKind::NextAttackRollAgainstSelfAdvantage => Self {
                next_attack_roll_against_self_advantage: true,
                ..Self::none()
            },
            BattleSpellActiveEffectKind::OpportunityAttackDenied => Self {
                opportunity_attack_denied: true,
                ..Self::none()
            },
            BattleSpellActiveEffectKind::Poisoned => Self {
                poisoned: true,
                ..Self::none()
            },
            BattleSpellActiveEffectKind::NextAttackRollDisadvantage => Self {
                next_attack_roll_disadvantage: true,
                ..Self::none()
            },
        }
    }

    #[must_use]
    pub const fn count(self) -> usize {
        self.armor_class_base_effect.is_active() as usize
            + self.hit_point_regain_prevented as usize
            + self.next_attack_roll_against_self_advantage as usize
            + self.opportunity_attack_denied as usize
            + self.poisoned as usize
            + self.next_attack_roll_disadvantage as usize
    }
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
    pub concentration: ConcentrationState,
    pub slot_spell_procedure: BattleSlotSpellProcedure,
    pub save_gated_spell_procedure: BattleSaveGatedSpellProcedure,
    pub hit_point_restoration_procedure: BattleHitPointRestorationProcedure,
    pub spell_attack_procedure: BattleSpellAttackProcedure,
    pub command_effect_procedure: BattleCommandEffectProcedure,
    pub spatial_route_subjects: Vec<BattleSpatialRouteSubject>,
    pub feature_substrates: BattleFeatureSubstrates,
    pub feature_resources: BattleFeatureResources,
    pub spell_slot_uses_this_turn: Vec<BattleTurnSpellSlotUse>,
    pub level_one_plus_spell_casters_this_turn: Vec<Actor>,
    pub quickened_level_one_plus_spell_casters_this_turn: Vec<Actor>,
    pub action_available: bool,
    pub bonus_action_available: bool,
    pub attack_roll_made_this_turn: bool,
    pub dash_movement_bonus_feet: i16,
    pub disengaged: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BattleSetup {
    pub initiative: Initiative,
    pub fighter: Combatant,
    pub goblin: Combatant,
    pub rogue: Combatant,
    pub skeleton: Combatant,
    pub stat_block_control: StatBlockControlState,
    pub turn_boundary_effects: TurnBoundaryEffects,
    pub interrupt_resume: BattleInterruptResumeState,
    pub reaction_casting_time: BattleReactionCastingTimeState,
    pub concentration: ConcentrationState,
    pub slot_spell_procedure: BattleSlotSpellProcedure,
    pub save_gated_spell_procedure: BattleSaveGatedSpellProcedure,
    pub hit_point_restoration_procedure: BattleHitPointRestorationProcedure,
    pub spell_attack_procedure: BattleSpellAttackProcedure,
    pub command_effect_procedure: BattleCommandEffectProcedure,
    pub spatial_route_subjects: Vec<BattleSpatialRouteSubject>,
    pub feature_substrates: BattleFeatureSubstrates,
    pub feature_resources: BattleFeatureResources,
    pub spell_slot_uses_this_turn: Vec<BattleTurnSpellSlotUse>,
    pub level_one_plus_spell_casters_this_turn: Vec<Actor>,
    pub quickened_level_one_plus_spell_casters_this_turn: Vec<Actor>,
    pub action_available: bool,
    pub bonus_action_available: bool,
    pub attack_roll_made_this_turn: bool,
    pub dash_movement_bonus_feet: i16,
    pub disengaged: bool,
}

impl BattleSetup {
    #[must_use]
    pub fn standard() -> Self {
        // QNT: cleanroom-input/qnt/battle-runtime/battle-runtime-model.qnt
        // `initialState`; setup facts are data inputs to the public battle
        // start entrypoint, not reducer dispatch rules.
        Self {
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
            concentration: concentration_initial_state(),
            slot_spell_procedure: BattleSlotSpellProcedure::Inactive,
            save_gated_spell_procedure: BattleSaveGatedSpellProcedure::Inactive,
            hit_point_restoration_procedure: BattleHitPointRestorationProcedure::Inactive,
            spell_attack_procedure: BattleSpellAttackProcedure::Inactive,
            command_effect_procedure: BattleCommandEffectProcedure::Inactive,
            spatial_route_subjects: Vec::new(),
            feature_substrates: BattleFeatureSubstrates::standard(),
            feature_resources: BattleFeatureResources::standard(),
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
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BattleStartResult {
    pub state: BattleState,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BattleFeatureResources {
    pub bonus_action_dash_temporary_hit_points: ResourcePoolFacts,
    pub sorcery_points: ResourcePoolFacts,
}

impl BattleFeatureResources {
    #[must_use]
    pub const fn standard() -> Self {
        Self {
            bonus_action_dash_temporary_hit_points: ResourcePoolFacts {
                capacity: 3,
                expended: 0,
            },
            sorcery_points: ResourcePoolFacts {
                capacity: 4,
                expended: 0,
            },
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BattleTurnAdvanceResult {
    pub state: BattleState,
    pub previous_actor: Actor,
    pub next_actor: Actor,
    pub round: i16,
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
pub enum BattleSlotSpellProcedure {
    Inactive,
    Active(BattleSlotSpellSubject),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BattleSlotSpellSubject {
    pub actor: Actor,
    pub target: Option<Actor>,
    pub stage: BattleSlotSpellStage,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BattleSlotSpellStage {
    TargetAllocation,
    DamageRoll,
    Resolved,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BattleSlotSpellHole {
    SpellTargetAllocation,
    RolledDice,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BattleSlotSpellFill {
    TargetAllocation(Actor),
    DamageRoll(i16),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BattleSaveGatedSpellProcedure {
    Inactive,
    Active(BattleSaveGatedSpellSubject),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BattleSaveGatedSpellSubject {
    pub actor: Actor,
    pub stage: SaveGatedSpellFrontierStage,
    pub damage_dice_required: bool,
    pub last_ordering_error: Option<SaveGatedSpellFillOrderingError>,
    pub damage_application: BattleSaveGatedDamageApplication,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BattleSaveGatedDamageApplication {
    pub primary_target: Actor,
    pub primary_damage: i16,
    pub primary_effect: BattleSpellActiveEffectKind,
    pub secondary_target: Option<Actor>,
    pub secondary_damage: i16,
    pub spend_first_level_slot: bool,
}

impl BattleSaveGatedDamageApplication {
    #[must_use]
    pub const fn none(actor: Actor) -> Self {
        Self {
            primary_target: actor,
            primary_damage: 0,
            primary_effect: BattleSpellActiveEffectKind::None,
            secondary_target: None,
            secondary_damage: 0,
            spend_first_level_slot: false,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BattleSaveGatedSpellFill {
    SpellTargetList,
    ConditionChoice,
    SavingThrowOutcome,
    DamageRoll,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BattleHitPointRestorationProcedure {
    Inactive,
    Active(BattleHitPointRestorationSubject),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BattleHitPointRestorationSubject {
    pub actor: Actor,
    pub stage: HitPointRestorationFrontierStage,
    pub targeting: BattleHitPointRestorationTargeting,
    pub last_ordering_error: Option<HitPointRestorationFillOrderingError>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BattleHitPointRestorationTargeting {
    SingleTargetSpell { target: Option<Actor> },
    TargetListSpell { target: Option<Actor> },
    FeatureHealingPool { target: Option<Actor> },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BattleHitPointRestorationFill {
    TargetChoice(Actor),
    SpellTargetList(Actor),
    HealingRoll(i16),
    HitPointHealingDistribution { target: Actor, amount: i16 },
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
    pub spend_first_level_slot: bool,
    pub last_ordering_error: Option<SpellAttackFillOrderingError>,
    pub target_effect: BattleSpellActiveEffectKind,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BattleSpellAttackFill {
    TargetChoice(Actor),
    DamageTypeChoice,
    AttackRoll(AttackRollFacts),
    DamageRoll(i16),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BattleCommandEffectProcedure {
    Inactive,
    Active(BattleCommandEffectSubject),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BattleCommandEffectSubject {
    pub actor: Actor,
    pub target: Option<Actor>,
    pub stage: CommandFrontierStage,
    pub pending_option: Option<CommandNextTurnOption>,
    pub last_ordering_error: Option<CommandFillOrderingError>,
    pub scenario: CommandNextTurnScenario,
    pub dropped_object_count: i16,
    pub halt_suppressed: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BattleCommandEffectFill {
    SpellTargetList(Actor),
    CommandOptionChoice(CommandNextTurnOption),
    SavingThrowOutcome {
        option: CommandNextTurnOption,
        failed: bool,
        movement_available: bool,
        held_object_facts_required: bool,
    },
    DropHeldObjectFacts {
        dropped_object_count: i16,
    },
    Movement {
        option: CommandNextTurnOption,
        movement_available: bool,
        moved_within_five_feet_of_caster: bool,
        opened_opportunity_attack: bool,
        movement_spent_feet: i16,
    },
    FollowPendingOption(CommandPendingOptionFollow),
    CleanupPendingOption(CommandPendingOptionCleanup),
    Complete,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CommandPendingOptionFollow {
    Grovel,
    Drop { dropped_object_count: i16 },
    Halt { movement_spent_feet: i16 },
    Approach { movement_spent_feet: i16 },
    Flee { movement_spent_feet: i16 },
    FleeOpportunityAttackWindow,
    FleeOpportunityAttackDeclined { movement_spent_feet: i16 },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CommandPendingOptionCleanup {
    Halt,
    ApproachNoMovement,
    FleeNoMovement,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BattleScalarBuffFill {
    TargetChoice(Actor),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BattleArmorClassSpellEffectFill {
    BaseArmorClassProjection(BattleArmorClassBaseProjectionFacts),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BattleArmorClassBaseProjectionFacts {
    pub target: Actor,
    pub base_armor_class: i8,
    pub dexterity_modifier: i8,
    pub duration_ticks: u16,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BattleReactionSpellFill {
    ArmorClassInterruption(BattleReactionArmorClassInterruptionFacts),
    FailedSaveDamage(BattleReactionFailedSaveDamageFacts),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BattleReactionArmorClassInterruptionFacts {
    pub reactor: Actor,
    pub armor_class_bonus: i16,
    pub slot_level: BattleSpellSlotLevel,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BattleReactionFailedSaveDamageFacts {
    pub reactor: Actor,
    pub trigger_creature: Actor,
    pub reactor_damage_taken: i16,
    pub damage: i16,
    pub slot_level: BattleSpellSlotLevel,
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
    EndTurn,
    WeaponAttack,
    Multiattack,
    SingleTargetSpellAttack,
    TypedSpellAttack,
    SlotSpell,
    SaveGatedAreaDamage,
    SaveGatedTargetListConditionChoice,
    HitPointRestorationSingleTargetSpell,
    HitPointRestorationTargetListSpell,
    HitPointRestorationFeatureHealingPool,
    DeathSavingThrow,
    ConcentrationTeardown,
    StatBlockAction,
    CommandSpell,
    ArmorClassSpellEffect,
    ReactionSpell,
    ScalarBuffTargetSpell,
    WeaponMasteryProperty,
    AttackActionAreaSaveDamageReplacement,
    UnitFeatureBonusAction,
    ActiveFeatureSpellSaveDc,
    ActiveFeatureSpellAttackRollMode,
    MetamagicOptionSpell,
    Spatial(BattleSpatialRouteSubject),
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
    pub holes: Vec<BattleHoleKind>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BattleActDiscoveryResult {
    available_acts: Vec<AvailableBattleAct>,
    current_actor: Actor,
    action_available: bool,
}

impl BattleActDiscoveryResult {
    #[must_use]
    pub fn available_acts(&self) -> &[AvailableBattleAct] {
        &self.available_acts
    }

    #[must_use]
    pub fn into_available_acts(self) -> Vec<AvailableBattleAct> {
        self.available_acts
    }

    #[must_use]
    pub fn current_actor(&self) -> Actor {
        self.current_actor
    }

    #[must_use]
    pub fn action_available(&self) -> bool {
        self.action_available
    }

    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.available_acts.is_empty()
    }

    #[must_use]
    pub fn len(&self) -> usize {
        self.available_acts.len()
    }

    fn from_state(state: &BattleState, available_acts: Vec<AvailableBattleAct>) -> Self {
        Self {
            available_acts,
            current_actor: current_actor(state),
            action_available: state.action_available,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct AttackRollFacts {
    pub total: i16,
    pub natural_d20: i16,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BattleFill {
    NoFill,
    TargetChoice(Actor),
    AttackRoll(AttackRollFacts),
    DamageRoll(i16),
    SneakAttackDamageRoll(i16),
    ResolveMultiattack,
    SpendMultiattackDispatch,
    SpellAttack(BattleSpellAttackFill),
    SlotSpell(BattleSlotSpellFill),
    SaveGatedSpell(BattleSaveGatedSpellFill),
    HitPointRestoration(BattleHitPointRestorationFill),
    DeathSavingThrow(DeathSavingThrowFacts),
    Concentration(BattleConcentrationFill),
    CommandEffect(BattleCommandEffectFill),
    ArmorClassSpellEffect(BattleArmorClassSpellEffectFill),
    ReactionSpell(BattleReactionSpellFill),
    ScalarBuff(BattleScalarBuffFill),
    WeaponMasteryProperty(BattleWeaponMasteryPropertyFill),
    AttackActionAreaSaveDamageReplacement(BattleAttackActionAreaSaveDamageReplacementFill),
    UnitFeatureBonusAction(BattleUnitFeatureBonusActionFill),
    ActiveFeatureSpellSaveDc(BattleActiveFeatureSpellBenefitFill),
    ActiveFeatureSpellAttackRollMode(BattleActiveFeatureSpellBenefitFill),
    MetamagicOptionSpell(BattleMetamagicOptionSpellFill),
    Spatial(BattleSpatialRouteFill),
    StatBlockAction {
        subject: StatBlockActionSubject,
        fill: StatBlockActionFill,
    },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BattleWeaponAttackFill {
    TargetChoice(Actor),
    AttackRoll(AttackRollFacts),
    DamageRoll(i16),
    SneakAttackDamageRoll(i16),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BattleMultiattackFill {
    ResolveMultiattack,
    SpendMultiattackDispatch,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BattleWeaponMasteryPropertyFill {
    pub property: WeaponMasteryProperty,
    pub primary_target: Actor,
    pub second_target: Option<Actor>,
    pub damage: i16,
    pub saving_throw_failed: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BattleAttackActionAreaSaveDamageReplacementFill {
    DragonbornBreathWeapon(DragonbornBreathWeaponFacts),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BattleUnitFeatureBonusActionFill {
    InnateSorcery { current_round: i16 },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BattleActiveFeatureSpellBenefitFill {
    InnateSorcery {
        current_round: i16,
        spell_facts: InnateSorcerySpellFacts,
    },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BattleMetamagicOptionFacts {
    pub selected_option_admitted: bool,
    pub sorcery_point_cost: i16,
    pub modification: BattleMetamagicSpellModification,
    pub changes_action_casting_time_to_bonus_action: bool,
    pub permits_multiple_options_for_spell: bool,
}

impl BattleMetamagicOptionFacts {
    #[must_use]
    pub const fn quickened_action_to_bonus_action() -> Self {
        Self {
            selected_option_admitted: true,
            sorcery_point_cost: QUICKENED_SORCERY_POINT_COST,
            modification: BattleMetamagicSpellModification::ActionCastingTimeToBonusAction,
            changes_action_casting_time_to_bonus_action: true,
            permits_multiple_options_for_spell: false,
        }
    }

    #[must_use]
    pub const fn spell_damage_dice_reroll() -> Self {
        Self {
            selected_option_admitted: true,
            sorcery_point_cost: 1,
            modification: BattleMetamagicSpellModification::SpellDamageDiceReroll,
            changes_action_casting_time_to_bonus_action: false,
            permits_multiple_options_for_spell: true,
        }
    }

    #[must_use]
    pub const fn missed_spell_attack_d20_reroll() -> Self {
        Self {
            selected_option_admitted: true,
            sorcery_point_cost: 1,
            modification: BattleMetamagicSpellModification::MissedSpellAttackD20Reroll,
            changes_action_casting_time_to_bonus_action: false,
            permits_multiple_options_for_spell: true,
        }
    }

    #[must_use]
    pub const fn spell_component_suppression() -> Self {
        Self {
            selected_option_admitted: true,
            sorcery_point_cost: 1,
            modification: BattleMetamagicSpellModification::SpellComponentSuppression,
            changes_action_casting_time_to_bonus_action: false,
            permits_multiple_options_for_spell: false,
        }
    }

    #[must_use]
    pub const fn spell_damage_type_substitution() -> Self {
        Self {
            selected_option_admitted: true,
            sorcery_point_cost: 1,
            modification: BattleMetamagicSpellModification::SpellDamageTypeSubstitution,
            changes_action_casting_time_to_bonus_action: false,
            permits_multiple_options_for_spell: false,
        }
    }

    #[must_use]
    pub const fn spell_duration_extension() -> Self {
        Self {
            selected_option_admitted: true,
            sorcery_point_cost: 1,
            modification: BattleMetamagicSpellModification::SpellDurationExtension,
            changes_action_casting_time_to_bonus_action: false,
            permits_multiple_options_for_spell: false,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BattleMetamagicSpellModification {
    ActionCastingTimeToBonusAction,
    ProtectedSavingThrow,
    FirstTargetSavingThrowDisadvantage,
    SpellRangeExtension,
    AdditionalSingleTarget,
    SpellDamageDiceReroll,
    MissedSpellAttackD20Reroll,
    SpellComponentSuppression,
    SpellDamageTypeSubstitution,
    SpellDurationExtension,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BattleMetamagicOptionSpellEffect {
    HitPointRestoration {
        target_hit_points_after: i16,
    },
    SaveGatedCondition,
    SaveGatedConditionImmunity,
    DirectCondition,
    RollModifier,
    SaveGatedDamage {
        target_hit_points_after: i16,
    },
    SpellAttack {
        target_hit_points_after: i16,
        target_active_effect_count: i16,
    },
    SpellAttackSequence {
        target_hit_points_after: i16,
        target_active_effect_count: i16,
    },
    ProtectedSaveGatedDamage {
        target_hit_points_after: i16,
        protected_save_targets: i16,
    },
    ProtectedSaveGatedNoEffect {
        protected_save_targets: i16,
    },
    FirstTargetDisadvantageSaveGatedDamage {
        target_hit_points_after: i16,
    },
    FirstTargetDisadvantageCondition {
        target_active_effect_count: i16,
    },
    FirstTargetDisadvantageEntrySave,
    FirstTargetDisadvantageEndTurnSave,
    FirstTargetDisadvantageConditionEndTurnSave,
    ObjectRangeLight {
        bright_radius_feet: i16,
        dim_additional_feet: i16,
    },
    AdditionalSingleTarget {
        target_active_effect_count: i16,
    },
    DamageReroll {
        target_hit_points_after: i16,
        target_active_effect_count: i16,
    },
    SpellAttackReroll {
        target_hit_points_after: i16,
        target_active_effect_count: i16,
    },
    ComponentSuppressedHitPointBuff {
        temporary_hit_points: i16,
        verbal_suppressed: bool,
        somatic_suppressed: bool,
        material_suppressed: bool,
        material_preserved: bool,
    },
    DamageTypeSubstitutionSaveGatedDamage {
        target_hit_points_after: i16,
    },
    DamageTypeSubstitutionSpellAttack {
        target_hit_points_after: i16,
        target_active_effect_count: i16,
    },
    DurationExtension {
        duration_ticks: i16,
        concentration_saving_throw_mode: ExtendedSpellConcentrationSaveMode,
    },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BattleMetamagicOptionSpellFill {
    pub option_facts: BattleMetamagicOptionFacts,
    pub effect: BattleMetamagicOptionSpellEffect,
    pub options_already_applied_to_spell: u8,
    pub selected_second_option_supported: bool,
    pub spell_uses_level_one_plus_slot: bool,
    pub spell_consumes_magic_action: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BattleResolutionRequestError {
    SubjectKindMismatch,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BattleResolutionRequest {
    subject: BattleSubject,
    fill: BattleFill,
}

impl BattleResolutionRequest {
    #[must_use]
    pub fn subject(&self) -> BattleSubject {
        self.subject
    }

    pub fn end_turn(subject: BattleSubject) -> Result<Self, BattleResolutionRequestError> {
        if subject.kind != BattleSubjectKind::EndTurn {
            return Err(BattleResolutionRequestError::SubjectKindMismatch);
        }
        Ok(Self {
            subject,
            fill: BattleFill::NoFill,
        })
    }

    pub fn weapon_attack(
        subject: BattleSubject,
        fill: BattleWeaponAttackFill,
    ) -> Result<Self, BattleResolutionRequestError> {
        if subject.kind != BattleSubjectKind::WeaponAttack {
            return Err(BattleResolutionRequestError::SubjectKindMismatch);
        }
        Ok(Self {
            subject,
            fill: match fill {
                BattleWeaponAttackFill::TargetChoice(target) => BattleFill::TargetChoice(target),
                BattleWeaponAttackFill::AttackRoll(roll) => BattleFill::AttackRoll(roll),
                BattleWeaponAttackFill::DamageRoll(damage) => BattleFill::DamageRoll(damage),
                BattleWeaponAttackFill::SneakAttackDamageRoll(damage) => {
                    BattleFill::SneakAttackDamageRoll(damage)
                }
            },
        })
    }

    pub fn multiattack(
        subject: BattleSubject,
        fill: BattleMultiattackFill,
    ) -> Result<Self, BattleResolutionRequestError> {
        if subject.kind != BattleSubjectKind::Multiattack {
            return Err(BattleResolutionRequestError::SubjectKindMismatch);
        }
        Ok(Self {
            subject,
            fill: match fill {
                BattleMultiattackFill::ResolveMultiattack => BattleFill::ResolveMultiattack,
                BattleMultiattackFill::SpendMultiattackDispatch => {
                    BattleFill::SpendMultiattackDispatch
                }
            },
        })
    }

    pub fn slot_spell(
        subject: BattleSubject,
        fill: BattleSlotSpellFill,
    ) -> Result<Self, BattleResolutionRequestError> {
        if subject.kind != BattleSubjectKind::SlotSpell {
            return Err(BattleResolutionRequestError::SubjectKindMismatch);
        }
        Ok(Self {
            subject,
            fill: BattleFill::SlotSpell(fill),
        })
    }

    pub fn spell_attack(
        subject: BattleSubject,
        fill: BattleSpellAttackFill,
    ) -> Result<Self, BattleResolutionRequestError> {
        if !matches!(
            subject.kind,
            BattleSubjectKind::SingleTargetSpellAttack | BattleSubjectKind::TypedSpellAttack
        ) {
            return Err(BattleResolutionRequestError::SubjectKindMismatch);
        }
        Ok(Self {
            subject,
            fill: BattleFill::SpellAttack(fill),
        })
    }

    pub fn save_gated_spell(
        subject: BattleSubject,
        fill: BattleSaveGatedSpellFill,
    ) -> Result<Self, BattleResolutionRequestError> {
        if !matches!(
            subject.kind,
            BattleSubjectKind::SaveGatedAreaDamage
                | BattleSubjectKind::SaveGatedTargetListConditionChoice
        ) {
            return Err(BattleResolutionRequestError::SubjectKindMismatch);
        }
        Ok(Self {
            subject,
            fill: BattleFill::SaveGatedSpell(fill),
        })
    }

    pub fn hit_point_restoration(
        subject: BattleSubject,
        fill: BattleHitPointRestorationFill,
    ) -> Result<Self, BattleResolutionRequestError> {
        if !matches!(
            subject.kind,
            BattleSubjectKind::HitPointRestorationSingleTargetSpell
                | BattleSubjectKind::HitPointRestorationTargetListSpell
                | BattleSubjectKind::HitPointRestorationFeatureHealingPool
        ) {
            return Err(BattleResolutionRequestError::SubjectKindMismatch);
        }
        Ok(Self {
            subject,
            fill: BattleFill::HitPointRestoration(fill),
        })
    }

    pub fn death_saving_throw(
        subject: BattleSubject,
        facts: DeathSavingThrowFacts,
    ) -> Result<Self, BattleResolutionRequestError> {
        if subject.kind != BattleSubjectKind::DeathSavingThrow {
            return Err(BattleResolutionRequestError::SubjectKindMismatch);
        }
        Ok(Self {
            subject,
            fill: BattleFill::DeathSavingThrow(facts),
        })
    }

    pub fn concentration(
        subject: BattleSubject,
        fill: BattleConcentrationFill,
    ) -> Result<Self, BattleResolutionRequestError> {
        if subject.kind != BattleSubjectKind::ConcentrationTeardown {
            return Err(BattleResolutionRequestError::SubjectKindMismatch);
        }
        Ok(Self {
            subject,
            fill: BattleFill::Concentration(fill),
        })
    }

    pub fn command_effect(
        subject: BattleSubject,
        fill: BattleCommandEffectFill,
    ) -> Result<Self, BattleResolutionRequestError> {
        if subject.kind != BattleSubjectKind::CommandSpell {
            return Err(BattleResolutionRequestError::SubjectKindMismatch);
        }
        Ok(Self {
            subject,
            fill: BattleFill::CommandEffect(fill),
        })
    }

    pub fn armor_class_spell_effect(
        subject: BattleSubject,
        fill: BattleArmorClassSpellEffectFill,
    ) -> Result<Self, BattleResolutionRequestError> {
        if subject.kind != BattleSubjectKind::ArmorClassSpellEffect {
            return Err(BattleResolutionRequestError::SubjectKindMismatch);
        }
        Ok(Self {
            subject,
            fill: BattleFill::ArmorClassSpellEffect(fill),
        })
    }

    pub fn reaction_spell(
        subject: BattleSubject,
        fill: BattleReactionSpellFill,
    ) -> Result<Self, BattleResolutionRequestError> {
        if subject.kind != BattleSubjectKind::ReactionSpell {
            return Err(BattleResolutionRequestError::SubjectKindMismatch);
        }
        Ok(Self {
            subject,
            fill: BattleFill::ReactionSpell(fill),
        })
    }

    pub fn scalar_buff(
        subject: BattleSubject,
        fill: BattleScalarBuffFill,
    ) -> Result<Self, BattleResolutionRequestError> {
        if subject.kind != BattleSubjectKind::ScalarBuffTargetSpell {
            return Err(BattleResolutionRequestError::SubjectKindMismatch);
        }
        Ok(Self {
            subject,
            fill: BattleFill::ScalarBuff(fill),
        })
    }

    pub fn weapon_mastery_property(
        subject: BattleSubject,
        fill: BattleWeaponMasteryPropertyFill,
    ) -> Result<Self, BattleResolutionRequestError> {
        if subject.kind != BattleSubjectKind::WeaponMasteryProperty {
            return Err(BattleResolutionRequestError::SubjectKindMismatch);
        }
        Ok(Self {
            subject,
            fill: BattleFill::WeaponMasteryProperty(fill),
        })
    }

    pub fn attack_action_area_save_damage_replacement(
        subject: BattleSubject,
        fill: BattleAttackActionAreaSaveDamageReplacementFill,
    ) -> Result<Self, BattleResolutionRequestError> {
        if subject.kind != BattleSubjectKind::AttackActionAreaSaveDamageReplacement {
            return Err(BattleResolutionRequestError::SubjectKindMismatch);
        }
        Ok(Self {
            subject,
            fill: BattleFill::AttackActionAreaSaveDamageReplacement(fill),
        })
    }

    pub fn unit_feature_bonus_action(
        subject: BattleSubject,
        fill: BattleUnitFeatureBonusActionFill,
    ) -> Result<Self, BattleResolutionRequestError> {
        if subject.kind != BattleSubjectKind::UnitFeatureBonusAction {
            return Err(BattleResolutionRequestError::SubjectKindMismatch);
        }
        Ok(Self {
            subject,
            fill: BattleFill::UnitFeatureBonusAction(fill),
        })
    }

    pub fn active_feature_spell_save_dc(
        subject: BattleSubject,
        fill: BattleActiveFeatureSpellBenefitFill,
    ) -> Result<Self, BattleResolutionRequestError> {
        if subject.kind != BattleSubjectKind::ActiveFeatureSpellSaveDc {
            return Err(BattleResolutionRequestError::SubjectKindMismatch);
        }
        Ok(Self {
            subject,
            fill: BattleFill::ActiveFeatureSpellSaveDc(fill),
        })
    }

    pub fn active_feature_spell_attack_roll_mode(
        subject: BattleSubject,
        fill: BattleActiveFeatureSpellBenefitFill,
    ) -> Result<Self, BattleResolutionRequestError> {
        if subject.kind != BattleSubjectKind::ActiveFeatureSpellAttackRollMode {
            return Err(BattleResolutionRequestError::SubjectKindMismatch);
        }
        Ok(Self {
            subject,
            fill: BattleFill::ActiveFeatureSpellAttackRollMode(fill),
        })
    }

    pub fn metamagic_option_spell(
        subject: BattleSubject,
        fill: BattleMetamagicOptionSpellFill,
    ) -> Result<Self, BattleResolutionRequestError> {
        if subject.kind != BattleSubjectKind::MetamagicOptionSpell {
            return Err(BattleResolutionRequestError::SubjectKindMismatch);
        }
        Ok(Self {
            subject,
            fill: BattleFill::MetamagicOptionSpell(fill),
        })
    }

    pub fn spatial_route(
        subject: BattleSubject,
        fill: BattleSpatialRouteFill,
    ) -> Result<Self, BattleResolutionRequestError> {
        if !matches!(subject.kind, BattleSubjectKind::Spatial(_)) {
            return Err(BattleResolutionRequestError::SubjectKindMismatch);
        }
        Ok(Self {
            subject,
            fill: BattleFill::Spatial(fill),
        })
    }

    pub fn stat_block_action(subject: StatBlockActionSubject, fill: StatBlockActionFill) -> Self {
        Self {
            subject: battle_subject_from_stat_block_action_subject(subject),
            fill: BattleFill::StatBlockAction { subject, fill },
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BattleHoleKind {
    TargetChoice,
    SpellTargetAllocation,
    AttackRoll,
    RolledDice,
    DamageTypeChoice,
    SpellTargetList,
    ConditionChoice,
    SavingThrowOutcome,
    HitPointHealingDistribution,
    DeathSavingThrow,
    ConcentrationSavingThrow,
    StatBlockRechargeRoll,
    CommandOptionChoice,
    InterruptDecision,
    Movement,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BattleSpatialRouteFill {
    TargetChoice,
    SavingThrowOutcome,
    UnitFeatureDecision,
    Movement,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BattleConcentrationFill {
    CastSpell,
    DamageTaken(ConcentrationDamageFacts),
    SavingThrow(ConcentrationSavingThrowFacts),
    VoluntaryEnd,
    CastReplacementSpell,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BattleResolutionInvalidReason {
    InvalidFill,
    MetamagicOptionEffectMismatch,
    StaleSubject,
    WrongActor,
    WrongTarget,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BattleResolutionResult {
    NeedsHoles {
        state: BattleState,
        subject: BattleSubject,
        holes: Vec<BattleHoleKind>,
    },
    Resolved {
        state: BattleState,
    },
    TurnAdvanced {
        turn_advance: BattleTurnAdvanceResult,
    },
    Invalid {
        state: BattleState,
        reason: BattleResolutionInvalidReason,
        holes: Vec<BattleHoleKind>,
    },
    StatBlockNeedsHoles {
        state: BattleState,
        subject: StatBlockActionSubject,
        holes: Vec<BattleHoleKind>,
    },
    StatBlockResolved {
        state: BattleState,
        subject: StatBlockActionSubject,
    },
    StatBlockInvalid {
        state: BattleState,
        subject: StatBlockActionSubject,
        reason: BattleResolutionInvalidReason,
        holes: Vec<BattleHoleKind>,
        last_ordering_error: Option<StatBlockActionFillOrderingError>,
    },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BattleResolutionNeedsHoles<'a> {
    state: &'a BattleState,
    subject: BattleSubject,
    holes: &'a [BattleHoleKind],
}

impl<'a> BattleResolutionNeedsHoles<'a> {
    #[must_use]
    pub const fn state(&self) -> &'a BattleState {
        self.state
    }

    #[must_use]
    pub const fn subject(&self) -> BattleSubject {
        self.subject
    }

    #[must_use]
    pub const fn holes(&self) -> &'a [BattleHoleKind] {
        self.holes
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BattleResolutionNeedsHolesParts {
    pub state: BattleState,
    pub subject: BattleSubject,
    pub holes: Vec<BattleHoleKind>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BattleResolutionInvalid<'a> {
    state: &'a BattleState,
    reason: BattleResolutionInvalidReason,
    holes: &'a [BattleHoleKind],
}

impl<'a> BattleResolutionInvalid<'a> {
    #[must_use]
    pub const fn state(&self) -> &'a BattleState {
        self.state
    }

    #[must_use]
    pub const fn reason(&self) -> BattleResolutionInvalidReason {
        self.reason
    }

    #[must_use]
    pub const fn holes(&self) -> &'a [BattleHoleKind] {
        self.holes
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BattleResolutionInvalidParts {
    pub state: BattleState,
    pub reason: BattleResolutionInvalidReason,
    pub holes: Vec<BattleHoleKind>,
}

impl BattleResolutionResult {
    #[must_use]
    pub const fn outcome(&self) -> BattleResolutionOutcome {
        match self {
            Self::NeedsHoles { .. } | Self::StatBlockNeedsHoles { .. } => {
                BattleResolutionOutcome::NeedsHoles
            }
            Self::Resolved { .. } | Self::TurnAdvanced { .. } | Self::StatBlockResolved { .. } => {
                BattleResolutionOutcome::Resolved
            }
            Self::Invalid { reason, .. } | Self::StatBlockInvalid { reason, .. } => {
                BattleResolutionOutcome::Invalid(*reason)
            }
        }
    }

    #[must_use]
    pub const fn state(&self) -> &BattleState {
        match self {
            Self::NeedsHoles { state, .. }
            | Self::Resolved { state }
            | Self::TurnAdvanced {
                turn_advance: BattleTurnAdvanceResult { state, .. },
            }
            | Self::Invalid { state, .. }
            | Self::StatBlockNeedsHoles { state, .. }
            | Self::StatBlockResolved { state, .. }
            | Self::StatBlockInvalid { state, .. } => state,
        }
    }

    #[must_use]
    pub fn into_state(self) -> BattleState {
        match self {
            Self::NeedsHoles { state, .. }
            | Self::Resolved { state }
            | Self::TurnAdvanced {
                turn_advance: BattleTurnAdvanceResult { state, .. },
            }
            | Self::Invalid { state, .. }
            | Self::StatBlockNeedsHoles { state, .. }
            | Self::StatBlockResolved { state, .. }
            | Self::StatBlockInvalid { state, .. } => state,
        }
    }

    #[must_use]
    pub const fn resolved_state(&self) -> Option<&BattleState> {
        match self {
            Self::Resolved { state }
            | Self::TurnAdvanced {
                turn_advance: BattleTurnAdvanceResult { state, .. },
            }
            | Self::StatBlockResolved { state, .. } => Some(state),
            Self::NeedsHoles { .. }
            | Self::Invalid { .. }
            | Self::StatBlockNeedsHoles { .. }
            | Self::StatBlockInvalid { .. } => None,
        }
    }

    #[must_use]
    pub fn into_resolved_state(self) -> Option<BattleState> {
        match self {
            Self::Resolved { state }
            | Self::TurnAdvanced {
                turn_advance: BattleTurnAdvanceResult { state, .. },
            }
            | Self::StatBlockResolved { state, .. } => Some(state),
            Self::NeedsHoles { .. }
            | Self::Invalid { .. }
            | Self::StatBlockNeedsHoles { .. }
            | Self::StatBlockInvalid { .. } => None,
        }
    }

    #[must_use]
    pub const fn turn_advance(&self) -> Option<&BattleTurnAdvanceResult> {
        match self {
            Self::TurnAdvanced { turn_advance } => Some(turn_advance),
            Self::NeedsHoles { .. }
            | Self::Resolved { .. }
            | Self::Invalid { .. }
            | Self::StatBlockNeedsHoles { .. }
            | Self::StatBlockResolved { .. }
            | Self::StatBlockInvalid { .. } => None,
        }
    }

    #[must_use]
    pub fn into_turn_advance(self) -> Option<BattleTurnAdvanceResult> {
        match self {
            Self::TurnAdvanced { turn_advance } => Some(turn_advance),
            Self::NeedsHoles { .. }
            | Self::Resolved { .. }
            | Self::Invalid { .. }
            | Self::StatBlockNeedsHoles { .. }
            | Self::StatBlockResolved { .. }
            | Self::StatBlockInvalid { .. } => None,
        }
    }

    #[must_use]
    pub fn needs_holes(&self) -> Option<BattleResolutionNeedsHoles<'_>> {
        match self {
            Self::NeedsHoles {
                state,
                subject,
                holes,
            } => Some(BattleResolutionNeedsHoles {
                state,
                subject: *subject,
                holes,
            }),
            Self::StatBlockNeedsHoles {
                state,
                subject,
                holes,
            } => Some(BattleResolutionNeedsHoles {
                state,
                subject: battle_subject_from_stat_block_action_subject(*subject),
                holes,
            }),
            Self::Resolved { .. }
            | Self::TurnAdvanced { .. }
            | Self::Invalid { .. }
            | Self::StatBlockResolved { .. }
            | Self::StatBlockInvalid { .. } => None,
        }
    }

    #[must_use]
    pub fn into_needs_holes(self) -> Option<BattleResolutionNeedsHolesParts> {
        match self {
            Self::NeedsHoles {
                state,
                subject,
                holes,
            } => Some(BattleResolutionNeedsHolesParts {
                state,
                subject,
                holes,
            }),
            Self::StatBlockNeedsHoles {
                state,
                subject,
                holes,
            } => Some(BattleResolutionNeedsHolesParts {
                state,
                subject: battle_subject_from_stat_block_action_subject(subject),
                holes,
            }),
            Self::Resolved { .. }
            | Self::TurnAdvanced { .. }
            | Self::Invalid { .. }
            | Self::StatBlockResolved { .. }
            | Self::StatBlockInvalid { .. } => None,
        }
    }

    #[must_use]
    pub const fn continuing_subject(&self) -> Option<BattleSubject> {
        match self {
            Self::NeedsHoles { subject, .. } => Some(*subject),
            Self::StatBlockNeedsHoles { subject, .. } => {
                Some(battle_subject_from_stat_block_action_subject(*subject))
            }
            Self::Resolved { .. }
            | Self::TurnAdvanced { .. }
            | Self::Invalid { .. }
            | Self::StatBlockResolved { .. }
            | Self::StatBlockInvalid { .. } => None,
        }
    }

    #[must_use]
    pub fn requested_holes(&self) -> Option<&[BattleHoleKind]> {
        match self {
            Self::NeedsHoles { holes, .. }
            | Self::Invalid { holes, .. }
            | Self::StatBlockNeedsHoles { holes, .. }
            | Self::StatBlockInvalid { holes, .. } => Some(holes),
            Self::Resolved { .. } | Self::TurnAdvanced { .. } | Self::StatBlockResolved { .. } => {
                None
            }
        }
    }

    #[must_use]
    pub fn invalid(&self) -> Option<BattleResolutionInvalid<'_>> {
        match self {
            Self::Invalid {
                state,
                reason,
                holes,
            } => Some(BattleResolutionInvalid {
                state,
                reason: *reason,
                holes,
            }),
            Self::StatBlockInvalid {
                state,
                reason,
                holes,
                ..
            } => Some(BattleResolutionInvalid {
                state,
                reason: *reason,
                holes,
            }),
            Self::NeedsHoles { .. }
            | Self::Resolved { .. }
            | Self::TurnAdvanced { .. }
            | Self::StatBlockNeedsHoles { .. }
            | Self::StatBlockResolved { .. } => None,
        }
    }

    #[must_use]
    pub fn into_invalid(self) -> Option<BattleResolutionInvalidParts> {
        match self {
            Self::Invalid {
                state,
                reason,
                holes,
            } => Some(BattleResolutionInvalidParts {
                state,
                reason,
                holes,
            }),
            Self::StatBlockInvalid {
                state,
                reason,
                holes,
                ..
            } => Some(BattleResolutionInvalidParts {
                state,
                reason,
                holes,
            }),
            Self::NeedsHoles { .. }
            | Self::Resolved { .. }
            | Self::TurnAdvanced { .. }
            | Self::StatBlockNeedsHoles { .. }
            | Self::StatBlockResolved { .. } => None,
        }
    }

    #[must_use]
    pub const fn invalid_reason(&self) -> Option<BattleResolutionInvalidReason> {
        match self {
            Self::Invalid { reason, .. } | Self::StatBlockInvalid { reason, .. } => Some(*reason),
            Self::NeedsHoles { .. }
            | Self::Resolved { .. }
            | Self::TurnAdvanced { .. }
            | Self::StatBlockNeedsHoles { .. }
            | Self::StatBlockResolved { .. } => None,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BattleEntrypointKind {
    StartBattle,
    DiscoverBattleActs,
    ResolveBattleSubject,
    AdvanceTurn,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BattleResolutionOutcome {
    NeedsHoles,
    Resolved,
    Invalid(BattleResolutionInvalidReason),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BattleEntrypointEvent {
    StartBattle,
    DiscoverBattleActs {
        available_subjects: Vec<BattleSubjectKind>,
    },
    ResolveBattleSubject {
        subject: BattleSubjectKind,
        outcome: BattleResolutionOutcome,
    },
    AdvanceTurn {
        previous_actor: Actor,
        next_actor: Actor,
        round: i16,
    },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BattleReducerRouteFillKind {
    AttackRoll,
    GrappleOutcome,
    ConcentrationSavingThrow,
    ConditionChoice,
    DamageTypeChoice,
    DeathSavingThrow,
    HitPointHealingDistribution,
    CommandOptionChoice,
    Movement,
    RolledDice,
    SavingThrowOutcome,
    SkillChoice,
    SpellTargetAllocation,
    SpellTargetList,
    StatBlockRechargeRoll,
    TargetChoice,
    UnitFeatureDecision,
    WildShapeEquipmentDisposition,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum BattleReducerRouteHoleKind {
    AttackRoll,
    GrappleOutcome,
    ConcentrationSavingThrow,
    ConditionChoice,
    DamageTypeChoice,
    DeathSavingThrow,
    HitPointHealingDistribution,
    InterruptDecision,
    CommandOptionChoice,
    Movement,
    RolledDice,
    SavingThrowOutcome,
    SkillChoice,
    SpellTargetAllocation,
    SpellTargetList,
    StatBlockRechargeRoll,
    TargetChoice,
    WildShapeEquipmentDisposition,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BattleReducerRouteSubjectFamily {
    ActiveFormLifecycle,
    AreaHazard,
    AreaObscurement,
    ConcentrationTeardown,
    CommandSpell,
    CreatureAttack,
    BattleAction,
    DeathSavingThrow,
    FallingMitigation,
    ForcedMovement,
    HitPointRestoration,
    LightProjection,
    MovementResource,
    ObjectBoundaryEffect,
    OutlineEffect,
    SaveGatedSpell,
    SlotSpell,
    SpellAttack,
    ScalarBuff,
    StatBlockAction,
    WeaponAttack,
    ActiveFeatureSpellAttackRollMode,
    ActiveFeatureSpellSaveDc,
    AttackActionAreaSaveDamageReplacement,
    MetamagicOptionSpell,
    UnitFeatureBonusAction,
    WeaponMasteryProperty,
    ZeroHitPointStabilization,
    PassiveAbilityCheckRollMode,
    PassiveDamageAdjustment,
    PassiveSavingThrowRollMode,
    CreatureSpaceMovementPermission,
    CreatureStatProjection,
    RollModifierEffect,
    ScalarBuffEffect,
    ArmorClassSpellEffect,
    ReactionSpell,
    SpellDamageReduction,
    CompanionLifecycle,
    CompanionSharedSenses,
    CompanionTouchDelivery,
    CompanionReactionAttack,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BattleSpatialRouteSubject {
    AreaHazard,
    AreaObscurement,
    FallingMitigation,
    ForcedMovement,
    LightProjection,
    MovementReplacement,
    ObjectBoundary,
    OutlineEffect,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BattleReducerRouteOwnerGroup {
    ActionEconomy,
    ActiveEffect,
    AbilityCheckRollMode,
    AreaShape,
    AttackRoll,
    AttackActionProcedure,
    Component,
    Concentration,
    ConditionLifecycle,
    Companion,
    CreatureSpaceMovement,
    CreatureState,
    DamageAdjustment,
    DamageRoll,
    DamageType,
    FeatureResource,
    HitPointAndZeroHpLifecycle,
    HitPoint,
    HoleFrontier,
    InterruptStack,
    MovementResource,
    ObjectBoundary,
    Reaction,
    SavingThrowOutcome,
    SavingThrowRollMode,
    SpellAttackProcedure,
    SpellSlotAndActionEconomy,
    StatBlockAction,
    TargetSelection,
    TemporaryHitPoint,
    TurnBoundary,
}

pub type BattleReducerRouteResolutionOutcome = BattleResolutionOutcome;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BattleReducerRouteEvent {
    StartBattle {
        owner: BattleReducerRouteOwnerGroup,
    },
    DiscoverBattleActs {
        subject: BattleReducerRouteSubjectFamily,
        holes: Vec<BattleReducerRouteHoleKind>,
        owner: BattleReducerRouteOwnerGroup,
    },
    ResolveBattleSubject {
        subject: BattleReducerRouteSubjectFamily,
        fill: BattleReducerRouteFillKind,
        outcome: BattleReducerRouteResolutionOutcome,
        holes: Vec<BattleReducerRouteHoleKind>,
        owner: BattleReducerRouteOwnerGroup,
    },
    ResolveBattleSubjectWithoutFill {
        subject: BattleReducerRouteSubjectFamily,
        outcome: BattleReducerRouteResolutionOutcome,
        holes: Vec<BattleReducerRouteHoleKind>,
        owner: BattleReducerRouteOwnerGroup,
    },
}

pub trait BattleReducerRouteObserver {
    fn observe_battle_reducer_route(&mut self, event: BattleReducerRouteEvent);
}

#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct BattleReducerRouteTrace {
    events: Vec<BattleReducerRouteEvent>,
}

impl BattleReducerRouteTrace {
    #[must_use]
    pub fn events(&self) -> &[BattleReducerRouteEvent] {
        &self.events
    }
}

impl BattleReducerRouteObserver for BattleReducerRouteTrace {
    fn observe_battle_reducer_route(&mut self, event: BattleReducerRouteEvent) {
        self.events.push(event);
    }
}

impl BattleEntrypointEvent {
    #[must_use]
    pub const fn kind(&self) -> BattleEntrypointKind {
        match self {
            Self::StartBattle => BattleEntrypointKind::StartBattle,
            Self::DiscoverBattleActs { .. } => BattleEntrypointKind::DiscoverBattleActs,
            Self::ResolveBattleSubject { .. } => BattleEntrypointKind::ResolveBattleSubject,
            Self::AdvanceTurn { .. } => BattleEntrypointKind::AdvanceTurn,
        }
    }
}

pub trait BattleEntrypointObserver {
    fn observe_battle_entrypoint(&mut self, event: BattleEntrypointEvent);

    fn observe_battle_reducer_route(&mut self, _event: BattleReducerRouteEvent) {}
}

#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct BattleEntrypointTrace {
    events: Vec<BattleEntrypointEvent>,
    route_events: Vec<BattleReducerRouteEvent>,
}

impl BattleEntrypointTrace {
    #[must_use]
    pub fn events(&self) -> &[BattleEntrypointEvent] {
        &self.events
    }

    #[must_use]
    pub fn route_events(&self) -> &[BattleReducerRouteEvent] {
        &self.route_events
    }
}

impl BattleEntrypointObserver for BattleEntrypointTrace {
    fn observe_battle_entrypoint(&mut self, event: BattleEntrypointEvent) {
        self.events.push(event);
    }

    fn observe_battle_reducer_route(&mut self, event: BattleReducerRouteEvent) {
        self.route_events.push(event);
    }
}

impl BattleReducerRouteObserver for BattleEntrypointTrace {
    fn observe_battle_reducer_route(&mut self, event: BattleReducerRouteEvent) {
        self.route_events.push(event);
    }
}

pub fn observe_battle_route_start(
    owner: BattleReducerRouteOwnerGroup,
    observer: &mut impl BattleReducerRouteObserver,
) {
    observer.observe_battle_reducer_route(BattleReducerRouteEvent::StartBattle { owner });
}

pub fn observe_battle_route_discovery(
    subject: BattleReducerRouteSubjectFamily,
    holes: Vec<BattleReducerRouteHoleKind>,
    owner: BattleReducerRouteOwnerGroup,
    observer: &mut impl BattleReducerRouteObserver,
) {
    observer.observe_battle_reducer_route(BattleReducerRouteEvent::DiscoverBattleActs {
        subject,
        holes: sorted_battle_reducer_route_holes(holes),
        owner,
    });
}

pub fn observe_battle_route_resolution(
    subject: BattleReducerRouteSubjectFamily,
    fill: BattleReducerRouteFillKind,
    outcome: BattleReducerRouteResolutionOutcome,
    holes: Vec<BattleReducerRouteHoleKind>,
    owner: BattleReducerRouteOwnerGroup,
    observer: &mut impl BattleReducerRouteObserver,
) {
    observer.observe_battle_reducer_route(BattleReducerRouteEvent::ResolveBattleSubject {
        subject,
        fill,
        outcome,
        holes: sorted_battle_reducer_route_holes(holes),
        owner,
    });
}

pub fn observe_battle_route_resolution_without_fill(
    subject: BattleReducerRouteSubjectFamily,
    outcome: BattleReducerRouteResolutionOutcome,
    holes: Vec<BattleReducerRouteHoleKind>,
    owner: BattleReducerRouteOwnerGroup,
    observer: &mut impl BattleReducerRouteObserver,
) {
    observer.observe_battle_reducer_route(
        BattleReducerRouteEvent::ResolveBattleSubjectWithoutFill {
            subject,
            outcome,
            holes: sorted_battle_reducer_route_holes(holes),
            owner,
        },
    );
}

pub fn observe_spatial_route_subject(
    subject: BattleSpatialRouteSubject,
    observer: &mut impl BattleReducerRouteObserver,
) {
    observe_battle_route_start(spatial_route_start_owner(subject), observer);
    observe_battle_route_discovery(
        spatial_route_subject_family(subject),
        spatial_route_holes(subject)
            .into_iter()
            .map(battle_reducer_route_hole)
            .collect(),
        spatial_route_owner(subject),
        observer,
    );
    observe_battle_route_resolution(
        spatial_route_subject_family(subject),
        spatial_route_fill_kind(spatial_route_fill(subject)),
        BattleReducerRouteResolutionOutcome::Resolved,
        Vec::new(),
        spatial_route_resolution_owner(subject),
        observer,
    );
}

const fn spatial_route_start_owner(
    subject: BattleSpatialRouteSubject,
) -> BattleReducerRouteOwnerGroup {
    match subject {
        BattleSpatialRouteSubject::FallingMitigation => {
            BattleReducerRouteOwnerGroup::InterruptStack
        }
        BattleSpatialRouteSubject::ForcedMovement
        | BattleSpatialRouteSubject::MovementReplacement => {
            BattleReducerRouteOwnerGroup::MovementResource
        }
        BattleSpatialRouteSubject::ObjectBoundary => BattleReducerRouteOwnerGroup::ObjectBoundary,
        BattleSpatialRouteSubject::AreaHazard
        | BattleSpatialRouteSubject::AreaObscurement
        | BattleSpatialRouteSubject::LightProjection
        | BattleSpatialRouteSubject::OutlineEffect => {
            BattleReducerRouteOwnerGroup::SpellSlotAndActionEconomy
        }
    }
}

const fn spatial_route_subject_family(
    subject: BattleSpatialRouteSubject,
) -> BattleReducerRouteSubjectFamily {
    match subject {
        BattleSpatialRouteSubject::AreaHazard => BattleReducerRouteSubjectFamily::AreaHazard,
        BattleSpatialRouteSubject::AreaObscurement => {
            BattleReducerRouteSubjectFamily::AreaObscurement
        }
        BattleSpatialRouteSubject::FallingMitigation => {
            BattleReducerRouteSubjectFamily::FallingMitigation
        }
        BattleSpatialRouteSubject::ForcedMovement => {
            BattleReducerRouteSubjectFamily::ForcedMovement
        }
        BattleSpatialRouteSubject::LightProjection => {
            BattleReducerRouteSubjectFamily::LightProjection
        }
        BattleSpatialRouteSubject::MovementReplacement => {
            BattleReducerRouteSubjectFamily::MovementResource
        }
        BattleSpatialRouteSubject::ObjectBoundary => {
            BattleReducerRouteSubjectFamily::ObjectBoundaryEffect
        }
        BattleSpatialRouteSubject::OutlineEffect => BattleReducerRouteSubjectFamily::OutlineEffect,
    }
}

fn spatial_route_holes(subject: BattleSpatialRouteSubject) -> Vec<BattleHoleKind> {
    match subject {
        BattleSpatialRouteSubject::AreaHazard => vec![BattleHoleKind::SavingThrowOutcome],
        BattleSpatialRouteSubject::AreaObscurement
        | BattleSpatialRouteSubject::LightProjection
        | BattleSpatialRouteSubject::ObjectBoundary
        | BattleSpatialRouteSubject::OutlineEffect => vec![BattleHoleKind::TargetChoice],
        BattleSpatialRouteSubject::FallingMitigation => vec![BattleHoleKind::InterruptDecision],
        BattleSpatialRouteSubject::ForcedMovement
        | BattleSpatialRouteSubject::MovementReplacement => {
            vec![BattleHoleKind::Movement]
        }
    }
}

pub const fn spatial_route_fill(subject: BattleSpatialRouteSubject) -> BattleSpatialRouteFill {
    match subject {
        BattleSpatialRouteSubject::AreaHazard => BattleSpatialRouteFill::SavingThrowOutcome,
        BattleSpatialRouteSubject::AreaObscurement
        | BattleSpatialRouteSubject::LightProjection
        | BattleSpatialRouteSubject::ObjectBoundary
        | BattleSpatialRouteSubject::OutlineEffect => BattleSpatialRouteFill::TargetChoice,
        BattleSpatialRouteSubject::FallingMitigation => BattleSpatialRouteFill::UnitFeatureDecision,
        BattleSpatialRouteSubject::ForcedMovement
        | BattleSpatialRouteSubject::MovementReplacement => BattleSpatialRouteFill::Movement,
    }
}

const fn spatial_route_fill_kind(fill: BattleSpatialRouteFill) -> BattleReducerRouteFillKind {
    match fill {
        BattleSpatialRouteFill::TargetChoice => BattleReducerRouteFillKind::TargetChoice,
        BattleSpatialRouteFill::SavingThrowOutcome => {
            BattleReducerRouteFillKind::SavingThrowOutcome
        }
        BattleSpatialRouteFill::UnitFeatureDecision => {
            BattleReducerRouteFillKind::UnitFeatureDecision
        }
        BattleSpatialRouteFill::Movement => BattleReducerRouteFillKind::Movement,
    }
}

const fn spatial_route_owner(subject: BattleSpatialRouteSubject) -> BattleReducerRouteOwnerGroup {
    match subject {
        BattleSpatialRouteSubject::AreaHazard | BattleSpatialRouteSubject::AreaObscurement => {
            BattleReducerRouteOwnerGroup::AreaShape
        }
        BattleSpatialRouteSubject::FallingMitigation => {
            BattleReducerRouteOwnerGroup::InterruptStack
        }
        BattleSpatialRouteSubject::ForcedMovement
        | BattleSpatialRouteSubject::MovementReplacement => {
            BattleReducerRouteOwnerGroup::MovementResource
        }
        BattleSpatialRouteSubject::LightProjection | BattleSpatialRouteSubject::OutlineEffect => {
            BattleReducerRouteOwnerGroup::ActiveEffect
        }
        BattleSpatialRouteSubject::ObjectBoundary => BattleReducerRouteOwnerGroup::ObjectBoundary,
    }
}

const fn spatial_route_resolution_owner(
    subject: BattleSpatialRouteSubject,
) -> BattleReducerRouteOwnerGroup {
    match subject {
        BattleSpatialRouteSubject::AreaHazard => BattleReducerRouteOwnerGroup::ConditionLifecycle,
        BattleSpatialRouteSubject::AreaObscurement
        | BattleSpatialRouteSubject::FallingMitigation
        | BattleSpatialRouteSubject::LightProjection => BattleReducerRouteOwnerGroup::ActiveEffect,
        BattleSpatialRouteSubject::ForcedMovement
        | BattleSpatialRouteSubject::MovementReplacement => {
            BattleReducerRouteOwnerGroup::MovementResource
        }
        BattleSpatialRouteSubject::ObjectBoundary => BattleReducerRouteOwnerGroup::ObjectBoundary,
        BattleSpatialRouteSubject::OutlineEffect => BattleReducerRouteOwnerGroup::AttackRoll,
    }
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
pub fn start_battle(setup: BattleSetup) -> BattleStartResult {
    BattleStartResult {
        state: BattleState {
            initiative: setup.initiative,
            fighter: setup.fighter,
            goblin: setup.goblin,
            rogue: setup.rogue,
            skeleton: setup.skeleton,
            stat_block_control: setup.stat_block_control,
            turn_boundary_effects: setup.turn_boundary_effects,
            interrupt_resume: setup.interrupt_resume,
            reaction_casting_time: setup.reaction_casting_time,
            concentration: setup.concentration,
            slot_spell_procedure: setup.slot_spell_procedure,
            save_gated_spell_procedure: setup.save_gated_spell_procedure,
            hit_point_restoration_procedure: setup.hit_point_restoration_procedure,
            spell_attack_procedure: setup.spell_attack_procedure,
            command_effect_procedure: setup.command_effect_procedure,
            spatial_route_subjects: setup.spatial_route_subjects,
            feature_substrates: setup.feature_substrates,
            feature_resources: setup.feature_resources,
            spell_slot_uses_this_turn: setup.spell_slot_uses_this_turn,
            level_one_plus_spell_casters_this_turn: setup.level_one_plus_spell_casters_this_turn,
            quickened_level_one_plus_spell_casters_this_turn: setup
                .quickened_level_one_plus_spell_casters_this_turn,
            action_available: setup.action_available,
            bonus_action_available: setup.bonus_action_available,
            attack_roll_made_this_turn: setup.attack_roll_made_this_turn,
            dash_movement_bonus_feet: setup.dash_movement_bonus_feet,
            disengaged: setup.disengaged,
        },
    }
}

#[must_use]
pub fn start_battle_observed(
    setup: BattleSetup,
    observer: &mut impl BattleEntrypointObserver,
) -> BattleStartResult {
    observer.observe_battle_entrypoint(BattleEntrypointEvent::StartBattle);
    observer.observe_battle_reducer_route(BattleReducerRouteEvent::StartBattle {
        owner: BattleReducerRouteOwnerGroup::ActionEconomy,
    });
    start_battle(setup)
}

#[must_use]
pub fn start_standard_battle() -> BattleState {
    start_battle(BattleSetup::standard()).state
}

#[must_use]
pub fn resolve_bonus_action_dash_temporary_hit_points_feature_battle(
    mut state: BattleState,
    actor: Actor,
    proficiency_bonus: i16,
) -> BattleResolutionResult {
    let combatant = combatant_for(&state, actor);
    let projection = resolve_adrenaline_rush_bonus_action_dash(AdrenalineRushFacts {
        turn: BattleTurnFeatureState {
            actor_temporary_hit_points: combatant.temporary_hp,
            bonus_action_available: state.bonus_action_available,
            dash_bonus_feet: state.dash_movement_bonus_feet,
            feature_uses_remaining: resource_pool_remaining(
                state
                    .feature_resources
                    .bonus_action_dash_temporary_hit_points,
            ),
        },
        speed_feet: combatant.speed_feet,
        proficiency_bonus,
        actor_owns_turn: actor == current_actor(&state),
        actor_unconscious: combatant.unconscious,
        actor_incapacitated: combatant.incapacitated,
    });

    match projection.result {
        AdrenalineRushResult::Resolved => {
            let resource_pool = state
                .feature_resources
                .bonus_action_dash_temporary_hit_points;
            let Ok(spent_pool) = spend_resource_pool(resource_pool, 1) else {
                return invalid_with_holes(
                    state,
                    BattleResolutionInvalidReason::InvalidFill,
                    Vec::new(),
                );
            };
            state
                .feature_resources
                .bonus_action_dash_temporary_hit_points = spent_pool;
            let actor_state = combatant_for_mut(&mut state, actor);
            actor_state.temporary_hp = projection.turn.actor_temporary_hit_points;
            state.bonus_action_available = projection.turn.bonus_action_available;
            state.dash_movement_bonus_feet = projection.turn.dash_bonus_feet;
            BattleResolutionResult::Resolved { state }
        }
        AdrenalineRushResult::Invalid(reason) => BattleResolutionResult::Invalid {
            state,
            reason: adrenaline_rush_invalid_reason(reason),
            holes: Vec::new(),
        },
    }
}

#[must_use]
pub fn bonus_action_dash_temporary_hit_points_feature_turn_state(
    state: &BattleState,
    actor: Actor,
) -> BattleTurnFeatureState {
    let combatant = combatant_for(state, actor);
    BattleTurnFeatureState {
        actor_temporary_hit_points: combatant.temporary_hp,
        bonus_action_available: state.bonus_action_available,
        dash_bonus_feet: state.dash_movement_bonus_feet,
        feature_uses_remaining: resource_pool_remaining(
            state
                .feature_resources
                .bonus_action_dash_temporary_hit_points,
        ),
    }
}

const fn adrenaline_rush_invalid_reason(
    reason: AdrenalineRushRejection,
) -> BattleResolutionInvalidReason {
    match reason {
        AdrenalineRushRejection::WrongActor => BattleResolutionInvalidReason::WrongActor,
        AdrenalineRushRejection::StaleSubject
        | AdrenalineRushRejection::NoUsesRemaining
        | AdrenalineRushRejection::UnableToAct => BattleResolutionInvalidReason::StaleSubject,
        AdrenalineRushRejection::InvalidFill => BattleResolutionInvalidReason::InvalidFill,
    }
}

#[must_use]
pub fn trace_adrenaline_rush_bonus_action_dash(
    state: BattleState,
    actor: Actor,
    proficiency_bonus: i16,
) -> (BattleResolutionResult, BattleReducerRouteTrace) {
    let mut trace = BattleReducerRouteTrace::default();
    observe_feature_dash_start(&mut trace);
    let result = resolve_feature_dash_observed(state, actor, proficiency_bonus, &mut trace);
    (result, trace)
}

#[must_use]
pub fn trace_adrenaline_rush_bonus_action_dash_then_reject_second_dash(
    state: BattleState,
    actor: Actor,
    proficiency_bonus: i16,
) -> (BattleResolutionResult, BattleReducerRouteTrace) {
    let mut trace = BattleReducerRouteTrace::default();
    observe_feature_dash_start(&mut trace);
    let first_result = resolve_feature_dash_observed(state, actor, proficiency_bonus, &mut trace);
    let second_result = resolve_feature_dash_observed(
        first_result.state().clone(),
        actor,
        proficiency_bonus,
        &mut trace,
    );
    (second_result, trace)
}

#[must_use]
pub fn project_dragonborn_damage_resistance_observed(
) -> (SpeciesPassiveTraitState, BattleReducerRouteTrace) {
    let mut trace = BattleReducerRouteTrace::default();
    observe_passive_route_after_creature_stat_projection(&mut trace);
    let state = project_dragonborn_damage_resistance();
    observe_damage_adjustment(&mut trace);
    (state, trace)
}

#[must_use]
pub fn project_dwarven_resilience_observed() -> (SpeciesPassiveTraitState, BattleReducerRouteTrace)
{
    let mut trace = BattleReducerRouteTrace::default();
    observe_passive_route_after_creature_stat_projection(&mut trace);
    observe_damage_adjustment(&mut trace);
    let state = project_dwarven_resilience();
    observe_saving_throw_roll_mode(&mut trace);
    (state, trace)
}

#[must_use]
pub fn project_halfling_brave_observed() -> (SpeciesPassiveTraitState, BattleReducerRouteTrace) {
    let mut trace = BattleReducerRouteTrace::default();
    observe_passive_route_after_creature_stat_projection(&mut trace);
    observe_damage_adjustment(&mut trace);
    let state = project_halfling_brave();
    observe_saving_throw_roll_mode(&mut trace);
    (state, trace)
}

#[must_use]
pub fn project_goliath_powerful_build_observed(
) -> (SpeciesPassiveTraitState, BattleReducerRouteTrace) {
    let mut trace = BattleReducerRouteTrace::default();
    observe_passive_route_after_ability_check_roll_mode(&mut trace);
    let state = project_goliath_powerful_build();
    (state, trace)
}

#[must_use]
pub fn trace_creature_space_traversal_sequence(
    facts: &[CreatureSpaceTraversalFacts],
) -> (Vec<bool>, BattleReducerRouteTrace) {
    let mut trace = BattleReducerRouteTrace::default();
    observe_passive_route_after_ability_check_roll_mode(&mut trace);
    let mut results = Vec::with_capacity(facts.len());

    for facts in facts {
        let accepted = creature_space_traversal_allowed(*facts);
        observe_creature_space_movement(accepted, &mut trace);
        results.push(accepted);
    }

    (results, trace)
}

#[must_use]
pub fn project_bless_attack_and_save_modifier_observed() -> (
    RollModifierBuffSelectedIdentityState,
    BattleReducerRouteTrace,
) {
    let mut trace = BattleReducerRouteTrace::default();
    observe_roll_modifier_start(&mut trace);
    observe_roll_modifier_discovery(Vec::new(), &mut trace);
    let state = project_bless_attack_and_save_modifier();
    observe_roll_modifier_resolution_without_fill(
        BattleReducerRouteOwnerGroup::ActiveEffect,
        &mut trace,
    );
    observe_roll_modifier_resolution_without_fill(
        BattleReducerRouteOwnerGroup::Concentration,
        &mut trace,
    );
    (state, trace)
}

#[must_use]
pub fn project_bane_failed_save_penalty_observed() -> (
    RollModifierBuffSelectedIdentityState,
    BattleReducerRouteTrace,
) {
    let mut trace = BattleReducerRouteTrace::default();
    observe_roll_modifier_start(&mut trace);
    observe_roll_modifier_discovery(
        vec![BattleReducerRouteHoleKind::SavingThrowOutcome],
        &mut trace,
    );
    let state = project_bane_failed_save_penalty();
    observe_battle_route_resolution(
        BattleReducerRouteSubjectFamily::RollModifierEffect,
        BattleReducerRouteFillKind::SavingThrowOutcome,
        BattleReducerRouteResolutionOutcome::Resolved,
        Vec::new(),
        BattleReducerRouteOwnerGroup::ActiveEffect,
        &mut trace,
    );
    observe_roll_modifier_resolution_without_fill(
        BattleReducerRouteOwnerGroup::Concentration,
        &mut trace,
    );
    (state, trace)
}

#[must_use]
pub fn project_guidance_skill_ability_check_modifier_observed() -> (
    RollModifierBuffSelectedIdentityState,
    BattleReducerRouteTrace,
) {
    let mut trace = BattleReducerRouteTrace::default();
    observe_roll_modifier_start(&mut trace);
    observe_roll_modifier_discovery(vec![BattleReducerRouteHoleKind::SkillChoice], &mut trace);
    let state = project_guidance_skill_ability_check_modifier();
    observe_battle_route_resolution(
        BattleReducerRouteSubjectFamily::RollModifierEffect,
        BattleReducerRouteFillKind::SkillChoice,
        BattleReducerRouteResolutionOutcome::Resolved,
        Vec::new(),
        BattleReducerRouteOwnerGroup::ActiveEffect,
        &mut trace,
    );
    observe_roll_modifier_resolution_without_fill(
        BattleReducerRouteOwnerGroup::Concentration,
        &mut trace,
    );
    (state, trace)
}

#[must_use]
pub fn project_resistance_matching_damage_reduction_observed() -> (
    RollModifierBuffSelectedIdentityState,
    BattleReducerRouteTrace,
) {
    let mut trace = BattleReducerRouteTrace::default();
    observe_battle_route_start(BattleReducerRouteOwnerGroup::ActionEconomy, &mut trace);
    observe_battle_route_discovery(
        BattleReducerRouteSubjectFamily::SpellDamageReduction,
        vec![BattleReducerRouteHoleKind::TargetChoice],
        BattleReducerRouteOwnerGroup::SpellSlotAndActionEconomy,
        &mut trace,
    );
    observe_battle_route_resolution(
        BattleReducerRouteSubjectFamily::SpellDamageReduction,
        BattleReducerRouteFillKind::TargetChoice,
        BattleReducerRouteResolutionOutcome::NeedsHoles,
        vec![BattleReducerRouteHoleKind::DamageTypeChoice],
        BattleReducerRouteOwnerGroup::TargetSelection,
        &mut trace,
    );
    let state = project_resistance_matching_damage_reduction();
    observe_battle_route_resolution(
        BattleReducerRouteSubjectFamily::SpellDamageReduction,
        BattleReducerRouteFillKind::DamageTypeChoice,
        BattleReducerRouteResolutionOutcome::Resolved,
        Vec::new(),
        BattleReducerRouteOwnerGroup::ActiveEffect,
        &mut trace,
    );
    observe_battle_route_resolution_without_fill(
        BattleReducerRouteSubjectFamily::SpellDamageReduction,
        BattleReducerRouteResolutionOutcome::Resolved,
        Vec::new(),
        BattleReducerRouteOwnerGroup::Concentration,
        &mut trace,
    );
    observe_battle_route_discovery(
        BattleReducerRouteSubjectFamily::SpellDamageReduction,
        vec![BattleReducerRouteHoleKind::RolledDice],
        BattleReducerRouteOwnerGroup::DamageAdjustment,
        &mut trace,
    );
    observe_battle_route_resolution(
        BattleReducerRouteSubjectFamily::SpellDamageReduction,
        BattleReducerRouteFillKind::RolledDice,
        BattleReducerRouteResolutionOutcome::Resolved,
        Vec::new(),
        BattleReducerRouteOwnerGroup::DamageAdjustment,
        &mut trace,
    );
    observe_battle_route_resolution_without_fill(
        BattleReducerRouteSubjectFamily::SpellDamageReduction,
        BattleReducerRouteResolutionOutcome::Resolved,
        Vec::new(),
        BattleReducerRouteOwnerGroup::ActiveEffect,
        &mut trace,
    );
    (state, trace)
}

#[must_use]
pub fn project_shield_of_faith_armor_class_bonus_observed() -> (
    RollModifierBuffSelectedIdentityState,
    BattleReducerRouteTrace,
) {
    let mut trace = BattleReducerRouteTrace::default();
    observe_battle_route_start(BattleReducerRouteOwnerGroup::ActionEconomy, &mut trace);
    observe_battle_route_discovery(
        BattleReducerRouteSubjectFamily::ScalarBuffEffect,
        Vec::new(),
        BattleReducerRouteOwnerGroup::SpellSlotAndActionEconomy,
        &mut trace,
    );
    let state = project_shield_of_faith_armor_class_bonus();
    observe_battle_route_resolution_without_fill(
        BattleReducerRouteSubjectFamily::ScalarBuffEffect,
        BattleReducerRouteResolutionOutcome::Resolved,
        Vec::new(),
        BattleReducerRouteOwnerGroup::ActiveEffect,
        &mut trace,
    );
    observe_battle_route_resolution_without_fill(
        BattleReducerRouteSubjectFamily::ScalarBuffEffect,
        BattleReducerRouteResolutionOutcome::Resolved,
        Vec::new(),
        BattleReducerRouteOwnerGroup::Concentration,
        &mut trace,
    );
    (state, trace)
}

fn observe_feature_dash_start(observer: &mut impl BattleReducerRouteObserver) {
    observe_battle_route_start(BattleReducerRouteOwnerGroup::ActionEconomy, observer);
    observe_battle_route_discovery(
        BattleReducerRouteSubjectFamily::UnitFeatureBonusAction,
        Vec::new(),
        BattleReducerRouteOwnerGroup::FeatureResource,
        observer,
    );
}

fn resolve_feature_dash_observed(
    state: BattleState,
    actor: Actor,
    proficiency_bonus: i16,
    observer: &mut impl BattleReducerRouteObserver,
) -> BattleResolutionResult {
    let result = resolve_bonus_action_dash_temporary_hit_points_feature_battle(
        state,
        actor,
        proficiency_bonus,
    );
    observe_feature_dash_result(&result, observer);
    result
}

fn observe_feature_dash_result(
    result: &BattleResolutionResult,
    observer: &mut impl BattleReducerRouteObserver,
) {
    observe_feature_dash_resolution(
        result,
        BattleReducerRouteOwnerGroup::ActionEconomy,
        observer,
    );

    if matches!(
        result.outcome(),
        BattleReducerRouteResolutionOutcome::Resolved
    ) {
        observe_feature_dash_resolution(
            result,
            BattleReducerRouteOwnerGroup::FeatureResource,
            observer,
        );
        observe_feature_dash_resolution(
            result,
            BattleReducerRouteOwnerGroup::MovementResource,
            observer,
        );
        observe_feature_dash_resolution(
            result,
            BattleReducerRouteOwnerGroup::TemporaryHitPoint,
            observer,
        );
    }
}

fn observe_feature_dash_resolution(
    result: &BattleResolutionResult,
    owner: BattleReducerRouteOwnerGroup,
    observer: &mut impl BattleReducerRouteObserver,
) {
    observe_battle_route_resolution_without_fill(
        BattleReducerRouteSubjectFamily::UnitFeatureBonusAction,
        result.outcome(),
        battle_reducer_route_holes(result),
        owner,
        observer,
    );
}

fn observe_passive_route_after_ability_check_roll_mode(
    observer: &mut impl BattleReducerRouteObserver,
) {
    observe_passive_route_after_creature_stat_projection(observer);
    observe_damage_adjustment(observer);
    observe_saving_throw_roll_mode(observer);
    observe_ability_check_roll_mode(observer);
}

fn observe_passive_route_after_creature_stat_projection(
    observer: &mut impl BattleReducerRouteObserver,
) {
    observe_battle_route_start(BattleReducerRouteOwnerGroup::CreatureState, observer);
    observe_battle_route_discovery(
        BattleReducerRouteSubjectFamily::CreatureStatProjection,
        Vec::new(),
        BattleReducerRouteOwnerGroup::CreatureState,
        observer,
    );
    observe_battle_route_resolution_without_fill(
        BattleReducerRouteSubjectFamily::CreatureStatProjection,
        BattleReducerRouteResolutionOutcome::Resolved,
        Vec::new(),
        BattleReducerRouteOwnerGroup::CreatureState,
        observer,
    );
    observe_battle_route_resolution_without_fill(
        BattleReducerRouteSubjectFamily::CreatureStatProjection,
        BattleReducerRouteResolutionOutcome::Resolved,
        Vec::new(),
        BattleReducerRouteOwnerGroup::MovementResource,
        observer,
    );
}

fn observe_damage_adjustment(observer: &mut impl BattleReducerRouteObserver) {
    observe_battle_route_discovery(
        BattleReducerRouteSubjectFamily::PassiveDamageAdjustment,
        Vec::new(),
        BattleReducerRouteOwnerGroup::DamageAdjustment,
        observer,
    );
    observe_battle_route_resolution_without_fill(
        BattleReducerRouteSubjectFamily::PassiveDamageAdjustment,
        BattleReducerRouteResolutionOutcome::Resolved,
        Vec::new(),
        BattleReducerRouteOwnerGroup::DamageAdjustment,
        observer,
    );
}

fn observe_saving_throw_roll_mode(observer: &mut impl BattleReducerRouteObserver) {
    observe_battle_route_discovery(
        BattleReducerRouteSubjectFamily::PassiveSavingThrowRollMode,
        vec![BattleReducerRouteHoleKind::SavingThrowOutcome],
        BattleReducerRouteOwnerGroup::SavingThrowRollMode,
        observer,
    );
    observe_battle_route_resolution(
        BattleReducerRouteSubjectFamily::PassiveSavingThrowRollMode,
        BattleReducerRouteFillKind::SavingThrowOutcome,
        BattleReducerRouteResolutionOutcome::Resolved,
        Vec::new(),
        BattleReducerRouteOwnerGroup::SavingThrowRollMode,
        observer,
    );
}

fn observe_ability_check_roll_mode(observer: &mut impl BattleReducerRouteObserver) {
    observe_battle_route_discovery(
        BattleReducerRouteSubjectFamily::PassiveAbilityCheckRollMode,
        vec![BattleReducerRouteHoleKind::GrappleOutcome],
        BattleReducerRouteOwnerGroup::AbilityCheckRollMode,
        observer,
    );
    observe_battle_route_resolution(
        BattleReducerRouteSubjectFamily::PassiveAbilityCheckRollMode,
        BattleReducerRouteFillKind::GrappleOutcome,
        BattleReducerRouteResolutionOutcome::Resolved,
        Vec::new(),
        BattleReducerRouteOwnerGroup::AbilityCheckRollMode,
        observer,
    );
}

fn observe_creature_space_movement(accepted: bool, observer: &mut impl BattleReducerRouteObserver) {
    observe_battle_route_discovery(
        BattleReducerRouteSubjectFamily::CreatureSpaceMovementPermission,
        vec![BattleReducerRouteHoleKind::Movement],
        BattleReducerRouteOwnerGroup::CreatureSpaceMovement,
        observer,
    );
    observe_battle_route_resolution(
        BattleReducerRouteSubjectFamily::CreatureSpaceMovementPermission,
        BattleReducerRouteFillKind::Movement,
        if accepted {
            BattleReducerRouteResolutionOutcome::Resolved
        } else {
            BattleReducerRouteResolutionOutcome::NeedsHoles
        },
        if accepted {
            Vec::new()
        } else {
            vec![BattleReducerRouteHoleKind::Movement]
        },
        BattleReducerRouteOwnerGroup::CreatureSpaceMovement,
        observer,
    );
    if accepted {
        observe_battle_route_resolution_without_fill(
            BattleReducerRouteSubjectFamily::CreatureSpaceMovementPermission,
            BattleReducerRouteResolutionOutcome::Resolved,
            Vec::new(),
            BattleReducerRouteOwnerGroup::MovementResource,
            observer,
        );
    }
}

fn observe_roll_modifier_start(observer: &mut impl BattleReducerRouteObserver) {
    observe_battle_route_start(BattleReducerRouteOwnerGroup::ActionEconomy, observer);
}

fn observe_roll_modifier_discovery(
    holes: Vec<BattleReducerRouteHoleKind>,
    observer: &mut impl BattleReducerRouteObserver,
) {
    observe_battle_route_discovery(
        BattleReducerRouteSubjectFamily::RollModifierEffect,
        holes,
        BattleReducerRouteOwnerGroup::SpellSlotAndActionEconomy,
        observer,
    );
}

fn observe_roll_modifier_resolution_without_fill(
    owner: BattleReducerRouteOwnerGroup,
    observer: &mut impl BattleReducerRouteObserver,
) {
    observe_battle_route_resolution_without_fill(
        BattleReducerRouteSubjectFamily::RollModifierEffect,
        BattleReducerRouteResolutionOutcome::Resolved,
        Vec::new(),
        owner,
        observer,
    );
}

#[must_use]
pub fn battle_reducer_route_holes(
    result: &BattleResolutionResult,
) -> Vec<BattleReducerRouteHoleKind> {
    let Some(holes) = result.requested_holes() else {
        return Vec::new();
    };
    sorted_battle_reducer_route_holes(
        holes
            .iter()
            .copied()
            .map(battle_reducer_route_hole)
            .collect(),
    )
}

fn sorted_battle_reducer_route_holes(
    mut holes: Vec<BattleReducerRouteHoleKind>,
) -> Vec<BattleReducerRouteHoleKind> {
    holes.sort();
    holes
}

fn battle_reducer_route_hole(hole: BattleHoleKind) -> BattleReducerRouteHoleKind {
    match hole {
        BattleHoleKind::AttackRoll => BattleReducerRouteHoleKind::AttackRoll,
        BattleHoleKind::ConcentrationSavingThrow => {
            BattleReducerRouteHoleKind::ConcentrationSavingThrow
        }
        BattleHoleKind::ConditionChoice => BattleReducerRouteHoleKind::ConditionChoice,
        BattleHoleKind::DamageTypeChoice => BattleReducerRouteHoleKind::DamageTypeChoice,
        BattleHoleKind::DeathSavingThrow => BattleReducerRouteHoleKind::DeathSavingThrow,
        BattleHoleKind::HitPointHealingDistribution => {
            BattleReducerRouteHoleKind::HitPointHealingDistribution
        }
        BattleHoleKind::InterruptDecision => BattleReducerRouteHoleKind::InterruptDecision,
        BattleHoleKind::CommandOptionChoice => BattleReducerRouteHoleKind::CommandOptionChoice,
        BattleHoleKind::Movement => BattleReducerRouteHoleKind::Movement,
        BattleHoleKind::RolledDice => BattleReducerRouteHoleKind::RolledDice,
        BattleHoleKind::SavingThrowOutcome => BattleReducerRouteHoleKind::SavingThrowOutcome,
        BattleHoleKind::SpellTargetAllocation => BattleReducerRouteHoleKind::SpellTargetAllocation,
        BattleHoleKind::SpellTargetList => BattleReducerRouteHoleKind::SpellTargetList,
        BattleHoleKind::StatBlockRechargeRoll => BattleReducerRouteHoleKind::StatBlockRechargeRoll,
        BattleHoleKind::TargetChoice => BattleReducerRouteHoleKind::TargetChoice,
    }
}

const fn battle_reducer_route_subject_family(
    subject: BattleSubjectKind,
) -> BattleReducerRouteSubjectFamily {
    match subject {
        BattleSubjectKind::EndTurn | BattleSubjectKind::Multiattack => {
            BattleReducerRouteSubjectFamily::BattleAction
        }
        BattleSubjectKind::WeaponAttack => BattleReducerRouteSubjectFamily::WeaponAttack,
        BattleSubjectKind::SingleTargetSpellAttack | BattleSubjectKind::TypedSpellAttack => {
            BattleReducerRouteSubjectFamily::SpellAttack
        }
        BattleSubjectKind::SlotSpell => BattleReducerRouteSubjectFamily::SlotSpell,
        BattleSubjectKind::SaveGatedAreaDamage
        | BattleSubjectKind::SaveGatedTargetListConditionChoice => {
            BattleReducerRouteSubjectFamily::SaveGatedSpell
        }
        BattleSubjectKind::HitPointRestorationSingleTargetSpell
        | BattleSubjectKind::HitPointRestorationTargetListSpell
        | BattleSubjectKind::HitPointRestorationFeatureHealingPool => {
            BattleReducerRouteSubjectFamily::HitPointRestoration
        }
        BattleSubjectKind::DeathSavingThrow => BattleReducerRouteSubjectFamily::DeathSavingThrow,
        BattleSubjectKind::ConcentrationTeardown => {
            BattleReducerRouteSubjectFamily::ConcentrationTeardown
        }
        BattleSubjectKind::StatBlockAction => BattleReducerRouteSubjectFamily::StatBlockAction,
        BattleSubjectKind::CommandSpell => BattleReducerRouteSubjectFamily::CommandSpell,
        BattleSubjectKind::ArmorClassSpellEffect => {
            BattleReducerRouteSubjectFamily::ArmorClassSpellEffect
        }
        BattleSubjectKind::ReactionSpell => BattleReducerRouteSubjectFamily::ReactionSpell,
        BattleSubjectKind::ScalarBuffTargetSpell => BattleReducerRouteSubjectFamily::ScalarBuff,
        BattleSubjectKind::WeaponMasteryProperty => {
            BattleReducerRouteSubjectFamily::WeaponMasteryProperty
        }
        BattleSubjectKind::AttackActionAreaSaveDamageReplacement => {
            BattleReducerRouteSubjectFamily::AttackActionAreaSaveDamageReplacement
        }
        BattleSubjectKind::UnitFeatureBonusAction => {
            BattleReducerRouteSubjectFamily::UnitFeatureBonusAction
        }
        BattleSubjectKind::ActiveFeatureSpellSaveDc => {
            BattleReducerRouteSubjectFamily::ActiveFeatureSpellSaveDc
        }
        BattleSubjectKind::ActiveFeatureSpellAttackRollMode => {
            BattleReducerRouteSubjectFamily::ActiveFeatureSpellAttackRollMode
        }
        BattleSubjectKind::MetamagicOptionSpell => {
            BattleReducerRouteSubjectFamily::MetamagicOptionSpell
        }
        BattleSubjectKind::Spatial(subject) => spatial_route_subject_family(subject),
    }
}

#[must_use]
pub fn start_skeleton_battle() -> BattleState {
    // QNT: battle-runtime-weapon-attack-skeleton.mbt.qnt literal initial
    // projection for the Rogue attacking a Skeleton.
    start_battle(BattleSetup {
        initiative: Initiative {
            round: 1,
            already_acted: Vec::new(),
            still_to_act: InitiativeStillToAct {
                actor: Actor::Rogue,
                waiting: vec![Actor::Skeleton],
            },
        },
        ..BattleSetup::standard()
    })
    .state
}

#[must_use]
pub fn start_skeleton_actor_turn() -> BattleState {
    start_battle(BattleSetup {
        initiative: Initiative {
            round: 1,
            already_acted: vec![Actor::Rogue],
            still_to_act: InitiativeStillToAct {
                actor: Actor::Skeleton,
                waiting: Vec::new(),
            },
        },
        ..BattleSetup::standard()
    })
    .state
}

#[must_use]
pub fn start_stat_block_actor_battle(actor: Actor) -> BattleState {
    start_battle(BattleSetup {
        initiative: Initiative {
            round: 1,
            already_acted: vec![Actor::Fighter],
            still_to_act: InitiativeStillToAct {
                actor,
                waiting: Vec::new(),
            },
        },
        ..BattleSetup::standard()
    })
    .state
}

#[must_use]
pub fn start_prone_rider_stat_block_battle(actor: Actor, target_size: CreatureSize) -> BattleState {
    // QNT: battle-runtime-stat-block-size-gated-condition-rider.mbt.qnt
    // uses a target with 20 HP and a size/condition-immunity gate for a Prone
    // on-hit rider.
    let mut state = start_stat_block_actor_battle(actor);
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
    start_battle(BattleSetup {
        turn_boundary_effects: TurnBoundaryEffects::lifecycle_witness_fixture(),
        ..BattleSetup::standard()
    })
    .state
}

#[must_use]
pub fn start_interrupt_stack_resume_battle() -> BattleState {
    // QNT: battle-runtime-interrupt-stack-resume.mbt.qnt `init`. The
    // interrupted target is the Fighter because the witness projects target HP
    // from 12 through the nested/replay cases.
    start_stat_block_actor_battle(Actor::Goblin)
}

#[must_use]
pub fn start_reaction_casting_time_battle() -> BattleState {
    // QNT: battle-runtime-reaction-casting-time.mbt.qnt `initialHp`.
    let mut state = start_stat_block_actor_battle(Actor::Goblin);
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
    let mut setup = BattleSetup::standard();
    setup.goblin = Combatant {
        hp: 12,
        max_hp: 12,
        ..setup.goblin
    };
    start_battle(setup).state
}

#[must_use]
pub fn start_spell_attack_ordering_battle() -> BattleState {
    start_standard_battle()
}

#[must_use]
pub fn start_fighter_skeleton_battle() -> BattleState {
    // QNT: cleanroom-input/qnt/battle-runtime/
    // battle-runtime-reducer-spine-contract.mbt.qnt start-battle projection.
    start_battle(fighter_skeleton_battle_setup()).state
}

fn fighter_skeleton_battle_setup() -> BattleSetup {
    BattleSetup {
        initiative: Initiative {
            round: 1,
            already_acted: Vec::new(),
            still_to_act: InitiativeStillToAct {
                actor: Actor::Fighter,
                waiting: vec![Actor::Skeleton],
            },
        },
        ..BattleSetup::standard()
    }
}

#[must_use]
pub fn start_weapon_mastery_property_battle() -> BattleState {
    // QNT: battle-runtime-weapon-mastery-selected-identity.mbt.qnt `init`.
    start_battle(weapon_mastery_property_battle_setup()).state
}

#[must_use]
pub fn start_weapon_mastery_property_battle_observed(
    observer: &mut impl BattleEntrypointObserver,
) -> BattleState {
    // QNT: battle-runtime-weapon-mastery-selected-identity.mbt.qnt `init`.
    start_battle_observed(weapon_mastery_property_battle_setup(), observer).state
}

fn weapon_mastery_property_battle_setup() -> BattleSetup {
    let mut setup = BattleSetup::standard();
    setup.initiative = Initiative {
        round: 1,
        already_acted: Vec::new(),
        still_to_act: InitiativeStillToAct {
            actor: Actor::Fighter,
            waiting: vec![Actor::Skeleton, Actor::Goblin],
        },
    };
    setup.fighter.weapon_damage_modifier = 0;
    setup.skeleton = Combatant {
        hp: WEAPON_MASTERY_TARGET_INITIAL_HIT_POINTS,
        max_hp: WEAPON_MASTERY_TARGET_INITIAL_HIT_POINTS,
        ..setup.skeleton
    };
    setup.goblin = Combatant {
        hp: WEAPON_MASTERY_TARGET_INITIAL_HIT_POINTS,
        max_hp: WEAPON_MASTERY_TARGET_INITIAL_HIT_POINTS,
        ..setup.goblin
    };
    setup
}

#[must_use]
pub fn start_dragonborn_breath_weapon_battle() -> BattleState {
    // QNT: battle-runtime-dragonborn-breath-weapon.mbt.qnt `init`.
    start_battle(dragonborn_breath_weapon_battle_setup()).state
}

#[must_use]
pub fn start_dragonborn_breath_weapon_battle_observed(
    observer: &mut impl BattleEntrypointObserver,
) -> BattleState {
    // QNT: battle-runtime-dragonborn-breath-weapon.mbt.qnt `init`.
    start_battle_observed(dragonborn_breath_weapon_battle_setup(), observer).state
}

fn dragonborn_breath_weapon_battle_setup() -> BattleSetup {
    let mut setup = BattleSetup::standard();
    setup.initiative = Initiative {
        round: 1,
        already_acted: Vec::new(),
        still_to_act: InitiativeStillToAct {
            actor: Actor::Fighter,
            waiting: vec![Actor::Skeleton, Actor::Goblin],
        },
    };
    setup.skeleton = Combatant {
        hp: 20,
        max_hp: 20,
        ..setup.skeleton
    };
    setup.goblin = Combatant {
        hp: 20,
        max_hp: 20,
        ..setup.goblin
    };
    setup.feature_substrates.dragonborn_breath_weapon =
        BattleDragonbornBreathWeaponSubstrate::initial();
    setup
}

#[must_use]
pub fn with_dragonborn_breath_weapon_uses_remaining(
    mut state: BattleState,
    uses_remaining: i16,
) -> BattleState {
    state
        .feature_substrates
        .dragonborn_breath_weapon
        .uses_remaining = uses_remaining;
    state
}

#[must_use]
pub fn start_innate_sorcery_feature_battle() -> BattleState {
    // QNT: battle-runtime-feature-selected-identity.mbt.qnt `init`.
    start_fighter_skeleton_battle()
}

#[must_use]
pub fn start_innate_sorcery_feature_battle_observed(
    observer: &mut impl BattleEntrypointObserver,
) -> BattleState {
    // QNT: battle-runtime-feature-selected-identity.mbt.qnt `init`.
    start_battle_observed(fighter_skeleton_battle_setup(), observer).state
}

#[must_use]
pub fn start_quickened_spell_governor_battle() -> BattleState {
    // RAW: cleanroom-input/raw/srd-5.2.1/Classes/Sorcerer.md
    // "Level 2: Metamagic" and "Quickened Spell"; QNT:
    // battle-runtime-quickened-spell-governor.mbt.qnt `init`.
    let mut setup = fighter_skeleton_battle_setup();
    setup.skeleton = Combatant {
        hp: QUICKENED_INITIAL_TARGET_HIT_POINTS,
        max_hp: 20,
        ..setup.skeleton
    };
    setup.feature_substrates.quickened_spell = BattleQuickenedSpellSubstrate {
        offered: true,
        ..BattleQuickenedSpellSubstrate::initial()
    };
    setup.feature_resources.sorcery_points = ResourcePoolFacts {
        capacity: 4,
        expended: 0,
    };
    start_battle(setup).state
}

#[must_use]
pub fn start_quickened_metamagic_battle() -> BattleState {
    // RAW: cleanroom-input/raw/srd-5.2.1/Classes/Sorcerer.md
    // "Level 2: Metamagic" and "Quickened Spell"; QNT:
    // battle-runtime-sorcerer-metamagic-*-selected-identity.mbt.qnt `init`.
    let mut setup = fighter_skeleton_battle_setup();
    setup.skeleton = Combatant {
        hp: 10,
        max_hp: 20,
        ..setup.skeleton
    };
    setup.feature_substrates.quickened_spell = BattleQuickenedSpellSubstrate {
        offered: true,
        ..BattleQuickenedSpellSubstrate::initial()
    };
    setup.feature_resources.sorcery_points = ResourcePoolFacts {
        capacity: 4,
        expended: 0,
    };
    start_battle(setup).state
}

#[must_use]
pub fn start_metamagic_option_spell_battle(sorcery_points_remaining: i16) -> BattleState {
    // RAW: cleanroom-input/raw/srd-5.2.1/Classes/Sorcerer.md
    // "Level 2: Metamagic"; QNT:
    // battle-runtime-sorcerer-metamagic-*-selected-identity.mbt.qnt `init`.
    let mut setup = fighter_skeleton_battle_setup();
    setup.skeleton = Combatant {
        hp: 10,
        max_hp: 20,
        ..setup.skeleton
    };
    setup.feature_substrates.metamagic_spell = BattleMetamagicSpellSubstrate {
        offered: true,
        ..BattleMetamagicSpellSubstrate::initial()
    };
    setup.feature_substrates.metamagic_option_spell = BattleMetamagicOptionSpellSubstrate {
        offered: true,
        ..BattleMetamagicOptionSpellSubstrate::initial()
    };
    setup.feature_resources.sorcery_points = ResourcePoolFacts {
        capacity: sorcery_points_remaining.max(0),
        expended: 0,
    };
    start_battle(setup).state
}

#[must_use]
pub fn with_battle_sorcery_points(mut state: BattleState, remaining: i16) -> BattleState {
    state.feature_resources.sorcery_points = ResourcePoolFacts {
        capacity: remaining.max(0),
        expended: 0,
    };
    state
}

#[must_use]
pub fn start_death_saving_throw_battle() -> BattleState {
    // RAW: cleanroom-input/raw/srd-5.2.1/Playing-the-Game.md
    // "Death Saving Throws"; QNT:
    // battle-runtime-death-saving-throw.route.mbt.qnt `init`.
    let mut state = start_fighter_skeleton_battle();
    let death_save = death_saving_throw_initial_state();
    state.fighter = Combatant {
        hp: death_save.target_hp,
        unconscious: death_save.target_unconscious,
        incapacitated: death_save.target_unconscious,
        prone: death_save.target_unconscious,
        lifecycle: CombatantLifecycle::UsesDeathSavingThrows(
            DeathSavingThrowLifecycle::from_state(death_save),
        ),
        ..state.fighter
    };
    state
}

#[must_use]
pub fn with_zero_hit_point_lifecycle(mut state: BattleState, target: Actor) -> Option<BattleState> {
    // RAW: cleanroom-input/raw/srd-5.2.1/Playing-the-Game.md
    // "Dropping to 0 Hit Points"; Ubiquitous Language "Hit Points and Death".
    let combatant = combatant_for(&state, target);
    if !matches!(
        combatant.lifecycle,
        CombatantLifecycle::UsesDeathSavingThrows(_)
    ) {
        return None;
    }
    let zero_hit_point_combatant = Combatant {
        hp: 0,
        unconscious: true,
        incapacitated: true,
        prone: true,
        lifecycle: CombatantLifecycle::UsesDeathSavingThrows(DeathSavingThrowLifecycle::fresh()),
        ..combatant
    };
    *combatant_for_mut(&mut state, target) = zero_hit_point_combatant;
    Some(state)
}

#[must_use]
pub fn discover_slot_spell_battle(mut state: BattleState) -> BattleState {
    // RAW: cleanroom-input/raw/srd-5.2.1/Spells/Descriptions-M-P.md
    // "Magic Missile"; support QNT: battle-runtime-magic-missile.mbt.qnt;
    // composition witness:
    // battle-runtime-reducer-spine-contract.mbt.qnt slot-spell discovery projection.
    let actor = current_actor(&state);
    if !state.action_available || !can_actor_expend_spell_slot_this_turn(&state, actor) {
        return state;
    }

    state.slot_spell_procedure = BattleSlotSpellProcedure::Active(BattleSlotSpellSubject {
        actor,
        target: None,
        stage: BattleSlotSpellStage::TargetAllocation,
    });
    state
}

#[must_use]
pub fn resolve_slot_spell_subject(state: BattleState, fill: BattleSlotSpellFill) -> BattleState {
    let BattleSlotSpellProcedure::Active(subject) = state.slot_spell_procedure else {
        return state;
    };
    if subject.actor != current_actor(&state) {
        return state;
    }

    match (subject.stage, fill) {
        (BattleSlotSpellStage::TargetAllocation, BattleSlotSpellFill::TargetAllocation(target)) => {
            BattleState {
                slot_spell_procedure: BattleSlotSpellProcedure::Active(BattleSlotSpellSubject {
                    target: Some(target),
                    stage: BattleSlotSpellStage::DamageRoll,
                    ..subject
                }),
                ..state
            }
        }
        (BattleSlotSpellStage::DamageRoll, BattleSlotSpellFill::DamageRoll(dart_pips_sum)) => {
            finalize_slot_spell_damage(state, subject, dart_pips_sum)
        }
        _ => state,
    }
}

#[must_use]
pub fn slot_spell_holes_from_battle(state: &BattleState) -> Vec<BattleSlotSpellHole> {
    match state.slot_spell_procedure {
        BattleSlotSpellProcedure::Active(BattleSlotSpellSubject {
            stage: BattleSlotSpellStage::TargetAllocation,
            ..
        }) => vec![BattleSlotSpellHole::SpellTargetAllocation],
        BattleSlotSpellProcedure::Active(BattleSlotSpellSubject {
            stage: BattleSlotSpellStage::DamageRoll,
            ..
        }) => vec![BattleSlotSpellHole::RolledDice],
        BattleSlotSpellProcedure::Inactive
        | BattleSlotSpellProcedure::Active(BattleSlotSpellSubject {
            stage: BattleSlotSpellStage::Resolved,
            ..
        }) => Vec::new(),
    }
}

#[must_use]
pub fn discover_single_target_spell_attack_battle(state: BattleState) -> BattleState {
    // RAW: cleanroom-input/raw/srd-5.2.1/Playing-the-Game.md
    // "Making an Attack"; QNT: battle-runtime-spell-attack-ordering.qnt
    // `SpellAttackTargetChoiceStage`.
    start_spell_attack_subject(
        state,
        false,
        false,
        SpellAttackFrontierStage::TargetChoice,
        BattleSpellActiveEffectKind::None,
    )
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
        true,
        SpellAttackFrontierStage::DamageTypeAndTargetChoice,
        BattleSpellActiveEffectKind::None,
    )
}

#[must_use]
pub fn discover_spell_attack_damage_subject_observed(
    state: BattleState,
    requires_spell_slot: bool,
    target_effect: BattleSpellActiveEffectKind,
    observer: &mut impl BattleEntrypointObserver,
) -> (BattleState, BattleSubject) {
    let actor = current_actor(&state);
    let subject = diagnostic_subject(BattleSubjectKind::SingleTargetSpellAttack, actor, None);
    observer.observe_battle_reducer_route(BattleReducerRouteEvent::DiscoverBattleActs {
        subject: BattleReducerRouteSubjectFamily::SpellAttack,
        holes: sorted_battle_reducer_route_holes(vec![BattleReducerRouteHoleKind::TargetChoice]),
        owner: BattleReducerRouteOwnerGroup::SpellSlotAndActionEconomy,
    });
    let state = start_spell_attack_subject(
        state,
        false,
        requires_spell_slot,
        SpellAttackFrontierStage::TargetChoice,
        target_effect,
    );
    (state, subject)
}

#[must_use]
pub fn discover_save_gated_damage_subject_observed(
    state: BattleState,
    damage_application: BattleSaveGatedDamageApplication,
    observer: &mut impl BattleEntrypointObserver,
) -> (BattleState, BattleSubject) {
    let actor = current_actor(&state);
    let subject = diagnostic_subject(BattleSubjectKind::SaveGatedAreaDamage, actor, None);
    observer.observe_battle_reducer_route(BattleReducerRouteEvent::DiscoverBattleActs {
        subject: BattleReducerRouteSubjectFamily::SaveGatedSpell,
        holes: sorted_battle_reducer_route_holes(vec![
            BattleReducerRouteHoleKind::SavingThrowOutcome,
        ]),
        owner: BattleReducerRouteOwnerGroup::SpellSlotAndActionEconomy,
    });
    let mut state = state;
    if !state.action_available {
        return (state, subject);
    }
    if damage_application.spend_first_level_slot {
        if !can_actor_expend_spell_slot_this_turn(&state, actor) {
            return (state, subject);
        }
        state = claim_pending_actor_spell_slot_use(state, actor);
    }
    state.action_available = false;
    state.save_gated_spell_procedure =
        BattleSaveGatedSpellProcedure::Active(BattleSaveGatedSpellSubject {
            actor,
            stage: SaveGatedSpellFrontierStage::DamageSavingThrowOutcome,
            damage_dice_required: true,
            last_ordering_error: None,
            damage_application,
        });
    (state, subject)
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
        let rolled_damage = spell_attack_damage_from_fill(fill);
        finalize_spell_attack_subject(next_state, next_subject, rolled_damage)
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
pub fn discover_command_effect_battle(mut state: BattleState) -> BattleState {
    if !state.action_available || first_waiting_actor(&state).is_none() {
        return state;
    }
    let actor = current_actor(&state);
    state.command_effect_procedure =
        BattleCommandEffectProcedure::Active(BattleCommandEffectSubject {
            actor,
            target: None,
            stage: CommandFrontierStage::TargetListAndOptionChoice,
            pending_option: None,
            last_ordering_error: None,
            scenario: CommandNextTurnScenario::Init,
            dropped_object_count: 0,
            halt_suppressed: false,
        });
    state
}

#[must_use]
pub fn start_command_effect_battle_at_actor(actor: CommandTurnActor) -> BattleState {
    start_battle(BattleSetup {
        initiative: Initiative {
            round: 1,
            already_acted: match actor {
                CommandTurnActor::Caster => Vec::new(),
                CommandTurnActor::Target => vec![Actor::Fighter],
            },
            still_to_act: InitiativeStillToAct {
                actor: battle_actor_for_command_actor(actor),
                waiting: match actor {
                    CommandTurnActor::Caster => vec![Actor::Goblin],
                    CommandTurnActor::Target => Vec::new(),
                },
            },
        },
        ..BattleSetup::standard()
    })
    .state
}

#[must_use]
pub fn command_ordering_projection_from_battle(state: &BattleState) -> CommandOrderingState {
    match state.command_effect_procedure {
        BattleCommandEffectProcedure::Inactive => command_ordering_initial_state(),
        BattleCommandEffectProcedure::Active(subject) => {
            let target = command_target_combatant(state, subject);
            let current_actor = command_current_turn_actor_from_subject(state, subject);
            command_ordering_projection(CommandOrderingProjectionFacts {
                stage: subject.stage,
                runtime_result: command_runtime_result_from_subject(subject),
                last_ordering_error: subject.last_ordering_error,
                pending_option: subject.pending_option,
                target_prone: target.is_some_and(|combatant| combatant.prone),
                dropped_object_count: subject.dropped_object_count,
                halt_suppressed: subject.halt_suppressed,
                movement_spent_feet: target.map_or(0, |combatant| combatant.movement_spent_feet),
                current_actor,
                reaction_window_open: command_reaction_window_open(state),
            })
        }
    }
}

#[must_use]
pub fn command_next_turn_projection_from_battle(
    state: &BattleState,
) -> crate::rules::command_options::CommandNextTurnState {
    let Some(subject) = active_command_subject(state) else {
        return command_next_turn_initial_state();
    };
    let target = command_target_combatant(state, subject);
    let current_actor = command_current_turn_actor_from_subject(state, subject);
    crate::rules::command_options::CommandNextTurnState {
        scenario: subject.scenario,
        protocol: command_next_turn_protocol_from_battle(state, subject, current_actor),
        target_prone: target.is_some_and(|combatant| combatant.prone),
        target_effect_count: if subject.pending_option.is_some() {
            1
        } else {
            0
        },
        action_available: state.action_available,
        bonus_action_available: state.bonus_action_available,
        movement_spent_feet: target.map_or(0, |combatant| combatant.movement_spent_feet),
        current_actor,
        pending_option: subject.pending_option,
        dropped_object_count: subject.dropped_object_count,
        reaction_window_open: command_reaction_window_open(state),
        halt_suppressed: subject.halt_suppressed,
    }
}

fn resolve_command_effect_battle_subject(
    state: BattleState,
    subject: BattleSubject,
    fill: BattleCommandEffectFill,
) -> BattleResolutionResult {
    let state = if matches!(
        state.command_effect_procedure,
        BattleCommandEffectProcedure::Inactive
    ) {
        discover_command_effect_battle(state)
    } else {
        state
    };
    let BattleCommandEffectProcedure::Active(active) = state.command_effect_procedure else {
        return invalid_with_holes(
            state,
            BattleResolutionInvalidReason::StaleSubject,
            Vec::new(),
        );
    };
    if active.actor != subject.actor {
        return invalid_with_holes(
            state,
            BattleResolutionInvalidReason::StaleSubject,
            Vec::new(),
        );
    }

    match fill {
        BattleCommandEffectFill::SpellTargetList(target) => resolve_command_ordered_fill(
            state,
            active,
            CommandOrderingFillFacts {
                fill_kind: CommandFillKind::SpellTargetList,
                option: active
                    .pending_option
                    .unwrap_or(CommandNextTurnOption::Grovel),
                failed_saving_throw: true,
                movement_available: false,
                held_object_facts_required: false,
                moved_within_five_feet_of_caster: false,
                opened_opportunity_attack: false,
            },
            |next| BattleCommandEffectSubject {
                target: Some(target),
                ..next
            },
        ),
        BattleCommandEffectFill::CommandOptionChoice(option) => resolve_command_ordered_fill(
            state,
            active,
            CommandOrderingFillFacts {
                fill_kind: CommandFillKind::CommandOptionChoice,
                option,
                failed_saving_throw: true,
                movement_available: false,
                held_object_facts_required: false,
                moved_within_five_feet_of_caster: false,
                opened_opportunity_attack: false,
            },
            |next| BattleCommandEffectSubject { ..next },
        ),
        BattleCommandEffectFill::SavingThrowOutcome {
            option,
            failed,
            movement_available,
            held_object_facts_required,
        } => resolve_command_ordered_fill(
            state,
            active,
            CommandOrderingFillFacts {
                fill_kind: CommandFillKind::SavingThrowOutcome,
                option,
                failed_saving_throw: failed,
                movement_available,
                held_object_facts_required,
                moved_within_five_feet_of_caster: false,
                opened_opportunity_attack: false,
            },
            |next| {
                if failed {
                    BattleCommandEffectSubject {
                        pending_option: Some(option),
                        scenario: CommandNextTurnScenario::FailedSaveRecordsPending,
                        ..next
                    }
                } else {
                    BattleCommandEffectSubject {
                        pending_option: None,
                        ..next
                    }
                }
            },
        ),
        BattleCommandEffectFill::DropHeldObjectFacts {
            dropped_object_count,
        } => resolve_command_effect_result(
            command_effect_state(
                state,
                BattleCommandEffectSubject {
                    stage: CommandFrontierStage::Resolved,
                    pending_option: None,
                    dropped_object_count: dropped_object_count.max(0),
                    scenario: CommandNextTurnScenario::FollowDrop,
                    ..active
                },
            ),
            BattleCommandEffectSubject {
                stage: CommandFrontierStage::Resolved,
                pending_option: None,
                dropped_object_count: dropped_object_count.max(0),
                scenario: CommandNextTurnScenario::FollowDrop,
                ..active
            },
        ),
        BattleCommandEffectFill::Movement {
            option,
            movement_available,
            moved_within_five_feet_of_caster,
            opened_opportunity_attack,
            movement_spent_feet,
        } => resolve_command_movement_fill(
            state,
            active,
            option,
            movement_available,
            moved_within_five_feet_of_caster,
            opened_opportunity_attack,
            movement_spent_feet,
        ),
        BattleCommandEffectFill::FollowPendingOption(follow) => {
            resolve_command_follow(state, active, follow)
        }
        BattleCommandEffectFill::CleanupPendingOption(cleanup) => {
            resolve_command_cleanup(state, active, cleanup)
        }
        BattleCommandEffectFill::Complete => resolve_command_effect_result(state, active),
    }
}

fn resolve_command_ordered_fill(
    state: BattleState,
    active: BattleCommandEffectSubject,
    facts: CommandOrderingFillFacts,
    update_subject: impl FnOnce(BattleCommandEffectSubject) -> BattleCommandEffectSubject,
) -> BattleResolutionResult {
    let result = command_fill_order_result(active.stage, facts);
    let next_stage = command_fill_order_accepted_stage(result, active.stage);
    let next_subject = update_subject(BattleCommandEffectSubject {
        stage: next_stage,
        last_ordering_error: command_fill_order_error(result),
        ..active
    });
    let next_state = command_effect_state(state, next_subject);
    match command_fill_order_runtime_result(result) {
        CommandOrderingRuntimeResult::Invalid => BattleResolutionResult::Invalid {
            state: next_state,
            reason: BattleResolutionInvalidReason::InvalidFill,
            holes: command_hole_kinds(next_subject.stage),
        },
        CommandOrderingRuntimeResult::Resolved => {
            BattleResolutionResult::Resolved { state: next_state }
        }
        CommandOrderingRuntimeResult::Init | CommandOrderingRuntimeResult::NeedsHoles => {
            BattleResolutionResult::NeedsHoles {
                state: next_state,
                subject: diagnostic_subject(
                    BattleSubjectKind::CommandSpell,
                    next_subject.actor,
                    next_subject.target,
                ),
                holes: command_hole_kinds(next_subject.stage),
            }
        }
    }
}

fn resolve_command_movement_fill(
    state: BattleState,
    active: BattleCommandEffectSubject,
    option: CommandNextTurnOption,
    movement_available: bool,
    moved_within_five_feet_of_caster: bool,
    opened_opportunity_attack: bool,
    movement_spent_feet: i16,
) -> BattleResolutionResult {
    let result = command_fill_order_result(
        active.stage,
        CommandOrderingFillFacts {
            fill_kind: CommandFillKind::Movement,
            option,
            failed_saving_throw: true,
            movement_available,
            held_object_facts_required: false,
            moved_within_five_feet_of_caster,
            opened_opportunity_attack,
        },
    );
    if command_fill_order_runtime_result(result) == CommandOrderingRuntimeResult::Invalid {
        let next_subject = BattleCommandEffectSubject {
            last_ordering_error: command_fill_order_error(result),
            pending_option: Some(option),
            scenario: match option {
                CommandNextTurnOption::Approach => {
                    CommandNextTurnScenario::ApproachMovementRejected
                }
                CommandNextTurnOption::Flee => CommandNextTurnScenario::FleePartialMovementRejected,
                CommandNextTurnOption::Drop
                | CommandNextTurnOption::Grovel
                | CommandNextTurnOption::Halt => active.scenario,
            },
            ..active
        };
        return BattleResolutionResult::Invalid {
            state: command_effect_state(state, next_subject),
            reason: BattleResolutionInvalidReason::InvalidFill,
            holes: command_hole_kinds(next_subject.stage),
        };
    }
    let Some(command_actor) = current_actor_for_command(active, option) else {
        return invalid_with_holes(
            state,
            BattleResolutionInvalidReason::InvalidFill,
            command_hole_kinds(active.stage),
        );
    };
    let mut next_state = state;
    combatant_for_mut(&mut next_state, command_actor).movement_spent_feet =
        movement_spent_feet.max(0);
    if opened_opportunity_attack {
        next_state.interrupt_resume.reaction_window = offer_reaction_window(
            next_state.interrupt_resume.reaction_window,
            ReactionWindowRole::AttackHitInterruption,
        );
    }
    let next_subject = BattleCommandEffectSubject {
        stage: command_fill_order_accepted_stage(result, active.stage),
        last_ordering_error: command_fill_order_error(result),
        pending_option: if opened_opportunity_attack {
            Some(option)
        } else {
            None
        },
        scenario: match option {
            CommandNextTurnOption::Approach if moved_within_five_feet_of_caster => {
                CommandNextTurnScenario::ApproachWithinFiveEndsTurn
            }
            CommandNextTurnOption::Approach => CommandNextTurnScenario::ApproachContinues,
            CommandNextTurnOption::Flee if opened_opportunity_attack => {
                CommandNextTurnScenario::FleeOpportunityAttackWindow
            }
            CommandNextTurnOption::Flee => CommandNextTurnScenario::FleeFullMovementEndsTurn,
            CommandNextTurnOption::Drop
            | CommandNextTurnOption::Grovel
            | CommandNextTurnOption::Halt => active.scenario,
        },
        ..active
    };
    resolve_command_effect_result(command_effect_state(next_state, next_subject), next_subject)
}

fn resolve_command_follow(
    state: BattleState,
    active: BattleCommandEffectSubject,
    follow: CommandPendingOptionFollow,
) -> BattleResolutionResult {
    let Some(target) = command_target_actor(active) else {
        return invalid_with_holes(
            state,
            BattleResolutionInvalidReason::InvalidFill,
            command_hole_kinds(active.stage),
        );
    };
    let mut next_state = state;
    let mut next_subject = BattleCommandEffectSubject {
        stage: CommandFrontierStage::Resolved,
        pending_option: None,
        last_ordering_error: None,
        ..active
    };
    match follow {
        CommandPendingOptionFollow::Grovel => {
            combatant_for_mut(&mut next_state, target).prone = true;
            next_subject.scenario = CommandNextTurnScenario::FollowGrovel;
        }
        CommandPendingOptionFollow::Drop {
            dropped_object_count,
        } => {
            next_subject.dropped_object_count = dropped_object_count.max(0);
            next_subject.scenario = CommandNextTurnScenario::FollowDrop;
        }
        CommandPendingOptionFollow::Halt {
            movement_spent_feet,
        } => {
            next_state.action_available = false;
            next_state.bonus_action_available = false;
            combatant_for_mut(&mut next_state, target).movement_spent_feet =
                movement_spent_feet.max(0);
            next_subject.pending_option = Some(CommandNextTurnOption::Halt);
            next_subject.halt_suppressed = true;
            next_subject.scenario = CommandNextTurnScenario::HaltSuppresses;
        }
        CommandPendingOptionFollow::Approach {
            movement_spent_feet,
        } => {
            combatant_for_mut(&mut next_state, target).movement_spent_feet =
                movement_spent_feet.max(0);
            next_subject.scenario = CommandNextTurnScenario::ApproachContinues;
        }
        CommandPendingOptionFollow::Flee {
            movement_spent_feet,
        } => {
            combatant_for_mut(&mut next_state, target).movement_spent_feet =
                movement_spent_feet.max(0);
            next_subject.scenario = CommandNextTurnScenario::FleeFullMovementEndsTurn;
        }
        CommandPendingOptionFollow::FleeOpportunityAttackWindow => {
            next_state.interrupt_resume.reaction_window = offer_reaction_window(
                next_state.interrupt_resume.reaction_window,
                ReactionWindowRole::AttackHitInterruption,
            );
            next_subject.pending_option = Some(CommandNextTurnOption::Flee);
            next_subject.scenario = CommandNextTurnScenario::FleeOpportunityAttackWindow;
        }
        CommandPendingOptionFollow::FleeOpportunityAttackDeclined {
            movement_spent_feet,
        } => {
            next_state.interrupt_resume.reaction_window =
                decline_reaction_window(next_state.interrupt_resume.reaction_window);
            combatant_for_mut(&mut next_state, target).movement_spent_feet =
                movement_spent_feet.max(0);
            next_subject.scenario =
                CommandNextTurnScenario::FleeOpportunityAttackDeclinedContinuation;
        }
    }
    resolve_command_effect_result(command_effect_state(next_state, next_subject), next_subject)
}

fn resolve_command_cleanup(
    state: BattleState,
    active: BattleCommandEffectSubject,
    cleanup: CommandPendingOptionCleanup,
) -> BattleResolutionResult {
    let mut next_state = state;
    let next_subject = match cleanup {
        CommandPendingOptionCleanup::Halt => {
            next_state.action_available = true;
            next_state.bonus_action_available = true;
            BattleCommandEffectSubject {
                stage: CommandFrontierStage::Resolved,
                pending_option: None,
                halt_suppressed: false,
                scenario: CommandNextTurnScenario::HaltEndTurnCleanup,
                ..active
            }
        }
        CommandPendingOptionCleanup::ApproachNoMovement => BattleCommandEffectSubject {
            stage: CommandFrontierStage::Resolved,
            pending_option: None,
            scenario: CommandNextTurnScenario::ApproachNoMovementCleanup,
            ..active
        },
        CommandPendingOptionCleanup::FleeNoMovement => BattleCommandEffectSubject {
            stage: CommandFrontierStage::Resolved,
            pending_option: None,
            scenario: CommandNextTurnScenario::FleeNoMovementCleanup,
            ..active
        },
    };
    resolve_command_effect_result(command_effect_state(next_state, next_subject), next_subject)
}

fn resolve_command_effect_result(
    state: BattleState,
    subject: BattleCommandEffectSubject,
) -> BattleResolutionResult {
    if subject.stage == CommandFrontierStage::Resolved {
        BattleResolutionResult::Resolved { state }
    } else {
        BattleResolutionResult::NeedsHoles {
            state,
            subject: diagnostic_subject(
                BattleSubjectKind::CommandSpell,
                subject.actor,
                subject.target,
            ),
            holes: command_hole_kinds(subject.stage),
        }
    }
}

fn command_effect_state(state: BattleState, subject: BattleCommandEffectSubject) -> BattleState {
    BattleState {
        command_effect_procedure: BattleCommandEffectProcedure::Active(subject),
        ..state
    }
}

fn active_command_subject(state: &BattleState) -> Option<BattleCommandEffectSubject> {
    match state.command_effect_procedure {
        BattleCommandEffectProcedure::Inactive => None,
        BattleCommandEffectProcedure::Active(subject) => Some(subject),
    }
}

#[must_use]
pub fn active_command_battle_subject(state: &BattleState) -> Option<BattleSubject> {
    active_command_subject(state).map(|subject| {
        diagnostic_subject(
            BattleSubjectKind::CommandSpell,
            subject.actor,
            subject.target,
        )
    })
}

fn command_runtime_result_from_subject(
    subject: BattleCommandEffectSubject,
) -> CommandOrderingRuntimeResult {
    if subject.last_ordering_error.is_some() {
        return CommandOrderingRuntimeResult::Invalid;
    }
    if subject.stage == CommandFrontierStage::Resolved {
        CommandOrderingRuntimeResult::Resolved
    } else {
        CommandOrderingRuntimeResult::NeedsHoles
    }
}

fn command_next_turn_protocol_from_battle(
    state: &BattleState,
    subject: BattleCommandEffectSubject,
    current_actor: Option<CommandTurnActor>,
) -> CommandNextTurnProtocol {
    if subject.last_ordering_error.is_some() || current_actor.is_none() {
        return CommandNextTurnProtocol::Invalid(CommandNextTurnInvalidReason::InvalidFill);
    }
    if command_reaction_window_open(state) {
        CommandNextTurnProtocol::NeedsInterruptDecision
    } else if subject.scenario == CommandNextTurnScenario::Init {
        CommandNextTurnProtocol::Init
    } else {
        CommandNextTurnProtocol::Resolved
    }
}

fn command_reaction_window_open(state: &BattleState) -> bool {
    state.interrupt_resume.reaction_window != ReactionWindowOffer::Closed
}

fn command_target_combatant(
    state: &BattleState,
    active: BattleCommandEffectSubject,
) -> Option<Combatant> {
    active.target.map(|target| combatant_for(state, target))
}

const fn command_target_actor(active: BattleCommandEffectSubject) -> Option<Actor> {
    active.target
}

const fn battle_actor_for_command_actor(actor: CommandTurnActor) -> Actor {
    match actor {
        CommandTurnActor::Caster => Actor::Fighter,
        CommandTurnActor::Target => Actor::Goblin,
    }
}

fn command_current_turn_actor_from_subject(
    state: &BattleState,
    active: BattleCommandEffectSubject,
) -> Option<CommandTurnActor> {
    let actor = current_actor(state);
    if actor == active.actor {
        Some(CommandTurnActor::Caster)
    } else if Some(actor) == active.target {
        Some(CommandTurnActor::Target)
    } else {
        None
    }
}

const fn current_actor_for_command(
    active: BattleCommandEffectSubject,
    option: CommandNextTurnOption,
) -> Option<Actor> {
    match option {
        CommandNextTurnOption::Approach | CommandNextTurnOption::Flee => {
            command_target_actor(active)
        }
        CommandNextTurnOption::Drop
        | CommandNextTurnOption::Grovel
        | CommandNextTurnOption::Halt => Some(active.actor),
    }
}

#[must_use]
pub fn scalar_buff_target_projection_from_battle_result(
    result: &BattleResolutionResult,
) -> ScalarBuffTargetState {
    let state = result.state();
    ScalarBuffTargetState {
        fighter_speed_feet: state.fighter.speed_feet,
        goblin_speed_feet: state.goblin.speed_feet,
        action_available: state.action_available,
        protocol: scalar_buff_protocol_from_result(result),
        protocol_holes: scalar_buff_holes_from_result(result),
    }
}

fn resolve_scalar_buff_battle_subject(
    state: BattleState,
    fill: BattleScalarBuffFill,
) -> BattleResolutionResult {
    match fill {
        BattleScalarBuffFill::TargetChoice(target) => {
            let mut next_state = state;
            combatant_for_mut(&mut next_state, target).speed_feet += 10;
            next_state.action_available = false;
            BattleResolutionResult::Resolved { state: next_state }
        }
    }
}

fn resolve_armor_class_spell_effect_subject(
    mut state: BattleState,
    fill: BattleArmorClassSpellEffectFill,
) -> BattleResolutionResult {
    match fill {
        BattleArmorClassSpellEffectFill::BaseArmorClassProjection(facts) => {
            if !can_actor_expend_spell_slot_this_turn(&state, current_actor(&state)) {
                return invalid_with_holes(
                    state,
                    BattleResolutionInvalidReason::StaleSubject,
                    Vec::new(),
                );
            }
            let actor = current_actor(&state);
            state = claim_pending_actor_spell_slot_use(state, actor);
            combatant_for_mut(&mut state, facts.target)
                .spell_active_effects
                .armor_class_base_effect = BattleArmorClassBaseEffect::Active {
                base_armor_class: facts.base_armor_class,
                dexterity_modifier: facts.dexterity_modifier,
                duration_ticks: facts.duration_ticks,
            };
            state = expend_combatant_spell_slot(state, actor, BattleSpellSlotLevel::First);
            state = commit_actor_spell_slot_use(state, actor);
            state.action_available = false;
            BattleResolutionResult::Resolved { state }
        }
    }
}

fn resolve_reaction_spell_subject(
    state: BattleState,
    fill: BattleReactionSpellFill,
) -> BattleResolutionResult {
    match fill {
        BattleReactionSpellFill::ArmorClassInterruption(facts) => {
            resolve_reaction_armor_class_interruption(state, facts)
        }
        BattleReactionSpellFill::FailedSaveDamage(facts) => {
            resolve_reaction_failed_save_damage(state, facts)
        }
    }
}

fn resolve_reaction_armor_class_interruption(
    state: BattleState,
    facts: BattleReactionArmorClassInterruptionFacts,
) -> BattleResolutionResult {
    if !can_actor_expend_spell_slot_this_turn(&state, facts.reactor)
        || !combatant_for(&state, facts.reactor).reaction_available
    {
        return invalid_with_holes(
            state,
            BattleResolutionInvalidReason::StaleSubject,
            Vec::new(),
        );
    }
    let state = claim_pending_actor_spell_slot_use(state, facts.reactor);
    let state = spend_reaction(state, facts.reactor);
    let state = apply_reaction_armor_class_bonus(state, facts.reactor, facts.armor_class_bonus);
    let state = expend_combatant_spell_slot(state, facts.reactor, facts.slot_level);
    BattleResolutionResult::Resolved {
        state: commit_actor_spell_slot_use(state, facts.reactor),
    }
}

fn resolve_reaction_failed_save_damage(
    state: BattleState,
    facts: BattleReactionFailedSaveDamageFacts,
) -> BattleResolutionResult {
    if !can_actor_expend_spell_slot_this_turn(&state, facts.reactor)
        || !combatant_for(&state, facts.reactor).reaction_available
    {
        return invalid_with_holes(
            state,
            BattleResolutionInvalidReason::StaleSubject,
            Vec::new(),
        );
    }
    let state = claim_pending_actor_spell_slot_use(state, facts.reactor);
    let state = spend_reaction(state, facts.reactor);
    let state = expend_combatant_spell_slot(state, facts.reactor, facts.slot_level);
    let state = commit_actor_spell_slot_use(state, facts.reactor);
    let state = with_damaged_target(state, facts.reactor, facts.reactor_damage_taken);
    BattleResolutionResult::Resolved {
        state: with_damaged_target(state, facts.trigger_creature, facts.damage),
    }
}

fn resolve_weapon_mastery_property_subject(
    mut state: BattleState,
    fill: BattleWeaponMasteryPropertyFill,
) -> BattleResolutionResult {
    match fill.property {
        WeaponMasteryProperty::Sap => {
            if fill.primary_target != Actor::Skeleton {
                return invalid_with_holes(
                    state,
                    BattleResolutionInvalidReason::WrongTarget,
                    Vec::new(),
                );
            }
            combatant_for_mut(&mut state, fill.primary_target).hp = damage_after_weapon_mastery_hit(
                combatant_for(&state, fill.primary_target).hp,
                fill.damage,
            );
            state.action_available = false;
            state.feature_substrates.weapon_mastery = BattleWeaponMasterySubstrate {
                sap_target: Some(fill.primary_target),
                cleave_used_this_turn: false,
                outcome: WeaponMasteryRuntimeOutcome::Resolved,
            };
        }
        WeaponMasteryProperty::Topple => {
            if fill.primary_target != Actor::Skeleton || !fill.saving_throw_failed {
                return invalid_with_holes(
                    state,
                    BattleResolutionInvalidReason::InvalidFill,
                    Vec::new(),
                );
            }
            combatant_for_mut(&mut state, fill.primary_target).prone = true;
            state.feature_substrates.weapon_mastery = BattleWeaponMasterySubstrate {
                sap_target: None,
                cleave_used_this_turn: false,
                outcome: WeaponMasteryRuntimeOutcome::NeedsHoles,
            };
            return BattleResolutionResult::NeedsHoles {
                state,
                subject: diagnostic_subject(
                    BattleSubjectKind::WeaponMasteryProperty,
                    Actor::Fighter,
                    None,
                ),
                holes: vec![BattleHoleKind::RolledDice],
            };
        }
        WeaponMasteryProperty::Cleave => {
            let Some(second_target) = fill.second_target else {
                return invalid_with_holes(
                    state,
                    BattleResolutionInvalidReason::WrongTarget,
                    Vec::new(),
                );
            };
            if fill.primary_target != Actor::Skeleton || second_target != Actor::Goblin {
                return invalid_with_holes(
                    state,
                    BattleResolutionInvalidReason::WrongTarget,
                    Vec::new(),
                );
            }
            combatant_for_mut(&mut state, fill.primary_target).hp = damage_after_weapon_mastery_hit(
                combatant_for(&state, fill.primary_target).hp,
                fill.damage,
            );
            combatant_for_mut(&mut state, second_target).hp = damage_after_weapon_mastery_hit(
                combatant_for(&state, second_target).hp,
                fill.damage,
            );
            state.action_available = false;
            state.feature_substrates.weapon_mastery = BattleWeaponMasterySubstrate {
                sap_target: None,
                cleave_used_this_turn: true,
                outcome: WeaponMasteryRuntimeOutcome::Resolved,
            };
        }
    }

    BattleResolutionResult::Resolved { state }
}

fn resolve_attack_action_area_save_damage_replacement_subject(
    state: BattleState,
    fill: BattleAttackActionAreaSaveDamageReplacementFill,
) -> BattleResolutionResult {
    match fill {
        BattleAttackActionAreaSaveDamageReplacementFill::DragonbornBreathWeapon(facts) => {
            let prior = dragonborn_breath_weapon_from_battle(&state);
            let state_from_feature = resolve_dragonborn_breath_weapon(prior, facts);
            let mut next_state = state;
            next_state.skeleton.hp = state_from_feature.target_hit_points;
            next_state.goblin.hp = state_from_feature.second_target_hit_points;
            next_state.feature_substrates.dragonborn_breath_weapon =
                BattleDragonbornBreathWeaponSubstrate {
                    uses_remaining: state_from_feature.breath_weapon_uses_remaining,
                    attack_action_attacks_remaining: state_from_feature
                        .attack_action_attacks_remaining,
                    expected_area_shape: facts.expected_area_shape,
                    scenario_outcome: state_from_feature.scenario_outcome,
                    protocol: state_from_feature.protocol,
                };
            next_state.action_available = state_from_feature.attack_action_attacks_remaining > 0;

            if matches!(
                state_from_feature.protocol,
                DragonbornBreathWeaponProtocol::Invalid(_)
            ) {
                BattleResolutionResult::Invalid {
                    state: next_state,
                    reason: BattleResolutionInvalidReason::InvalidFill,
                    holes: breath_weapon_invalid_holes(state_from_feature.scenario_outcome),
                }
            } else {
                BattleResolutionResult::Resolved { state: next_state }
            }
        }
    }
}

fn resolve_unit_feature_bonus_action_subject(
    state: BattleState,
    fill: BattleUnitFeatureBonusActionFill,
) -> BattleResolutionResult {
    match fill {
        BattleUnitFeatureBonusActionFill::InnateSorcery { current_round } => {
            BattleResolutionResult::Resolved {
                state: apply_innate_sorcery_spell_benefit(
                    state,
                    current_round,
                    InnateSorceryScenarioOutcome::Activated,
                    1,
                    InnateSorcerySpellAttackRollMode::Normal,
                ),
            }
        }
    }
}

fn resolve_active_feature_spell_save_dc_subject(
    state: BattleState,
    fill: BattleActiveFeatureSpellBenefitFill,
) -> BattleResolutionResult {
    match fill {
        BattleActiveFeatureSpellBenefitFill::InnateSorcery {
            current_round,
            spell_facts,
        } => {
            let (outcome, save_dc_bonus) = innate_sorcery_spell_benefit_projection(spell_facts);
            BattleResolutionResult::Resolved {
                state: apply_innate_sorcery_spell_benefit(
                    state,
                    current_round,
                    outcome,
                    save_dc_bonus,
                    InnateSorcerySpellAttackRollMode::Normal,
                ),
            }
        }
    }
}

fn resolve_active_feature_spell_attack_roll_mode_subject(
    state: BattleState,
    fill: BattleActiveFeatureSpellBenefitFill,
) -> BattleResolutionResult {
    match fill {
        BattleActiveFeatureSpellBenefitFill::InnateSorcery {
            current_round,
            spell_facts,
        } => {
            let (outcome, save_dc_bonus) = innate_sorcery_spell_benefit_projection(spell_facts);
            let roll_mode = if spell_facts.benefit_eligibility
                == InnateSorcerySpellBenefitEligibility::Eligible
            {
                InnateSorcerySpellAttackRollMode::Advantage
            } else {
                InnateSorcerySpellAttackRollMode::Normal
            };
            BattleResolutionResult::Resolved {
                state: apply_innate_sorcery_spell_benefit(
                    state,
                    current_round,
                    outcome,
                    save_dc_bonus,
                    roll_mode,
                ),
            }
        }
    }
}

fn resolve_metamagic_option_spell_subject(
    state: BattleState,
    subject: BattleSubject,
    fill: BattleMetamagicOptionSpellFill,
) -> BattleResolutionResult {
    if !metamagic_option_effect_matches_modification(fill.option_facts.modification, fill.effect) {
        return BattleResolutionResult::Invalid {
            state,
            reason: BattleResolutionInvalidReason::MetamagicOptionEffectMismatch,
            holes: Vec::new(),
        };
    }
    if !fill.option_facts.selected_option_admitted {
        return rejected_metamagic_option_spell_result(
            state,
            fill.effect,
            QuickenedSpellInvalidKind::UnknownOption,
            QuickenedSpellScenarioOutcome::RejectedUnknownOption,
        );
    }
    if fill.option_facts.sorcery_point_cost
        > resource_pool_remaining(state.feature_resources.sorcery_points)
    {
        return rejected_metamagic_option_spell_result(
            state,
            fill.effect,
            QuickenedSpellInvalidKind::Unaffordable,
            QuickenedSpellScenarioOutcome::RejectedUnaffordable,
        );
    }
    if fill.options_already_applied_to_spell > 0 && !fill.selected_second_option_supported {
        return rejected_metamagic_option_spell_result(
            state,
            fill.effect,
            QuickenedSpellInvalidKind::UnsupportedSecondOption,
            QuickenedSpellScenarioOutcome::RejectedUnsupportedSecondOption,
        );
    }
    if fill.options_already_applied_to_spell > 0
        && !fill.option_facts.permits_multiple_options_for_spell
    {
        return rejected_metamagic_option_spell_result(
            state,
            fill.effect,
            QuickenedSpellInvalidKind::OnePerSpell,
            QuickenedSpellScenarioOutcome::RejectedOnePerSpell,
        );
    }
    if fill.option_facts.modification
        == BattleMetamagicSpellModification::ActionCastingTimeToBonusAction
        && fill.spell_uses_level_one_plus_slot
        && state
            .level_one_plus_spell_casters_this_turn
            .contains(&subject.actor)
    {
        return rejected_metamagic_option_spell_result(
            state,
            fill.effect,
            QuickenedSpellInvalidKind::SameTurnLevelOnePlus,
            QuickenedSpellScenarioOutcome::RejectedPriorLevelOnePlusSpell,
        );
    }

    let Ok(sorcery_points) = spend_resource_pool(
        state.feature_resources.sorcery_points,
        fill.option_facts.sorcery_point_cost,
    ) else {
        return rejected_metamagic_option_spell_result(
            state,
            fill.effect,
            QuickenedSpellInvalidKind::Unaffordable,
            QuickenedSpellScenarioOutcome::RejectedUnaffordable,
        );
    };

    let magic_action_available_before_resolution = state.action_available;
    let mut state = state;
    state.feature_resources.sorcery_points = sorcery_points;
    if fill.option_facts.modification
        == BattleMetamagicSpellModification::ActionCastingTimeToBonusAction
    {
        state.bonus_action_available = false;
    } else if fill.spell_consumes_magic_action {
        state.action_available = false;
    }
    if fill.spell_uses_level_one_plus_slot
        && fill.option_facts.modification
            == BattleMetamagicSpellModification::ActionCastingTimeToBonusAction
    {
        state = claim_pending_actor_spell_slot_use(state, subject.actor);
        state = expend_combatant_spell_slot(state, subject.actor, BattleSpellSlotLevel::First);
        state = commit_actor_spell_slot_use(state, subject.actor);
        if !state
            .quickened_level_one_plus_spell_casters_this_turn
            .contains(&subject.actor)
        {
            state
                .quickened_level_one_plus_spell_casters_this_turn
                .push(subject.actor);
        }
    }

    state = apply_metamagic_spell_effect(state, fill.effect);
    match fill.option_facts.modification {
        BattleMetamagicSpellModification::ActionCastingTimeToBonusAction => {
            state.feature_substrates.quickened_spell =
                quickened_success_substrate(fill.effect, magic_action_available_before_resolution);
        }
        BattleMetamagicSpellModification::ProtectedSavingThrow
        | BattleMetamagicSpellModification::FirstTargetSavingThrowDisadvantage
        | BattleMetamagicSpellModification::SpellRangeExtension
        | BattleMetamagicSpellModification::AdditionalSingleTarget => {
            state.feature_substrates.metamagic_spell =
                metamagic_spell_success_substrate(fill.effect);
        }
        BattleMetamagicSpellModification::SpellDamageDiceReroll
        | BattleMetamagicSpellModification::MissedSpellAttackD20Reroll
        | BattleMetamagicSpellModification::SpellComponentSuppression
        | BattleMetamagicSpellModification::SpellDamageTypeSubstitution
        | BattleMetamagicSpellModification::SpellDurationExtension => {
            state.feature_substrates.metamagic_option_spell =
                metamagic_success_substrate(fill.effect);
        }
    }

    BattleResolutionResult::Resolved { state }
}

const fn metamagic_option_effect_matches_modification(
    modification: BattleMetamagicSpellModification,
    effect: BattleMetamagicOptionSpellEffect,
) -> bool {
    match modification {
        BattleMetamagicSpellModification::ActionCastingTimeToBonusAction => matches!(
            effect,
            BattleMetamagicOptionSpellEffect::HitPointRestoration { .. }
                | BattleMetamagicOptionSpellEffect::SaveGatedCondition
                | BattleMetamagicOptionSpellEffect::SaveGatedConditionImmunity
                | BattleMetamagicOptionSpellEffect::DirectCondition
                | BattleMetamagicOptionSpellEffect::RollModifier
                | BattleMetamagicOptionSpellEffect::SaveGatedDamage { .. }
                | BattleMetamagicOptionSpellEffect::SpellAttack { .. }
                | BattleMetamagicOptionSpellEffect::SpellAttackSequence { .. }
        ),
        BattleMetamagicSpellModification::ProtectedSavingThrow => matches!(
            effect,
            BattleMetamagicOptionSpellEffect::ProtectedSaveGatedDamage { .. }
                | BattleMetamagicOptionSpellEffect::ProtectedSaveGatedNoEffect { .. }
        ),
        BattleMetamagicSpellModification::FirstTargetSavingThrowDisadvantage => matches!(
            effect,
            BattleMetamagicOptionSpellEffect::FirstTargetDisadvantageSaveGatedDamage { .. }
                | BattleMetamagicOptionSpellEffect::FirstTargetDisadvantageCondition { .. }
                | BattleMetamagicOptionSpellEffect::FirstTargetDisadvantageEntrySave
                | BattleMetamagicOptionSpellEffect::FirstTargetDisadvantageEndTurnSave
                | BattleMetamagicOptionSpellEffect::FirstTargetDisadvantageConditionEndTurnSave
        ),
        BattleMetamagicSpellModification::SpellRangeExtension => matches!(
            effect,
            BattleMetamagicOptionSpellEffect::ObjectRangeLight { .. }
        ),
        BattleMetamagicSpellModification::AdditionalSingleTarget => matches!(
            effect,
            BattleMetamagicOptionSpellEffect::AdditionalSingleTarget { .. }
        ),
        BattleMetamagicSpellModification::SpellDamageDiceReroll => matches!(
            effect,
            BattleMetamagicOptionSpellEffect::DamageReroll { .. }
        ),
        BattleMetamagicSpellModification::MissedSpellAttackD20Reroll => matches!(
            effect,
            BattleMetamagicOptionSpellEffect::SpellAttackReroll { .. }
        ),
        BattleMetamagicSpellModification::SpellComponentSuppression => matches!(
            effect,
            BattleMetamagicOptionSpellEffect::ComponentSuppressedHitPointBuff { .. }
        ),
        BattleMetamagicSpellModification::SpellDamageTypeSubstitution => matches!(
            effect,
            BattleMetamagicOptionSpellEffect::DamageTypeSubstitutionSaveGatedDamage { .. }
                | BattleMetamagicOptionSpellEffect::DamageTypeSubstitutionSpellAttack { .. }
        ),
        BattleMetamagicSpellModification::SpellDurationExtension => matches!(
            effect,
            BattleMetamagicOptionSpellEffect::DurationExtension { .. }
        ),
    }
}

fn rejected_metamagic_option_spell_result(
    mut state: BattleState,
    effect: BattleMetamagicOptionSpellEffect,
    invalid_kind: QuickenedSpellInvalidKind,
    outcome: QuickenedSpellScenarioOutcome,
) -> BattleResolutionResult {
    if matches!(
        effect,
        BattleMetamagicOptionSpellEffect::ComponentSuppressedHitPointBuff { .. }
    ) {
        state.feature_substrates.metamagic_option_spell = BattleMetamagicOptionSpellSubstrate {
            offered: false,
            projection: BattleMetamagicOptionSpellProjection::SubtleUnaffordable,
        };
    } else {
        state.feature_substrates.quickened_spell = BattleQuickenedSpellSubstrate {
            offered: invalid_kind != QuickenedSpellInvalidKind::Unaffordable
                && invalid_kind != QuickenedSpellInvalidKind::SameTurnLevelOnePlus,
            invalid_kind,
            governor_outcome: outcome,
            governor_protocol: QuickenedSpellProtocol::Resolved,
            metamagic_protocol: QuickenedMetamagicProtocol::Resolved,
            ..state.feature_substrates.quickened_spell
        };
    }
    BattleResolutionResult::Invalid {
        state,
        reason: BattleResolutionInvalidReason::InvalidFill,
        holes: Vec::new(),
    }
}

fn apply_metamagic_spell_effect(
    mut state: BattleState,
    effect: BattleMetamagicOptionSpellEffect,
) -> BattleState {
    let target_hit_points_after = match effect {
        BattleMetamagicOptionSpellEffect::HitPointRestoration {
            target_hit_points_after,
        }
        | BattleMetamagicOptionSpellEffect::SaveGatedDamage {
            target_hit_points_after,
        }
        | BattleMetamagicOptionSpellEffect::SpellAttack {
            target_hit_points_after,
            ..
        }
        | BattleMetamagicOptionSpellEffect::SpellAttackSequence {
            target_hit_points_after,
            ..
        }
        | BattleMetamagicOptionSpellEffect::ProtectedSaveGatedDamage {
            target_hit_points_after,
            ..
        }
        | BattleMetamagicOptionSpellEffect::FirstTargetDisadvantageSaveGatedDamage {
            target_hit_points_after,
        }
        | BattleMetamagicOptionSpellEffect::DamageReroll {
            target_hit_points_after,
            ..
        }
        | BattleMetamagicOptionSpellEffect::SpellAttackReroll {
            target_hit_points_after,
            ..
        }
        | BattleMetamagicOptionSpellEffect::DamageTypeSubstitutionSaveGatedDamage {
            target_hit_points_after,
        }
        | BattleMetamagicOptionSpellEffect::DamageTypeSubstitutionSpellAttack {
            target_hit_points_after,
            ..
        } => Some(target_hit_points_after),
        BattleMetamagicOptionSpellEffect::SaveGatedCondition
        | BattleMetamagicOptionSpellEffect::SaveGatedConditionImmunity
        | BattleMetamagicOptionSpellEffect::DirectCondition
        | BattleMetamagicOptionSpellEffect::RollModifier
        | BattleMetamagicOptionSpellEffect::ProtectedSaveGatedNoEffect { .. }
        | BattleMetamagicOptionSpellEffect::FirstTargetDisadvantageCondition { .. }
        | BattleMetamagicOptionSpellEffect::FirstTargetDisadvantageEntrySave
        | BattleMetamagicOptionSpellEffect::FirstTargetDisadvantageEndTurnSave
        | BattleMetamagicOptionSpellEffect::FirstTargetDisadvantageConditionEndTurnSave
        | BattleMetamagicOptionSpellEffect::ObjectRangeLight { .. }
        | BattleMetamagicOptionSpellEffect::AdditionalSingleTarget { .. }
        | BattleMetamagicOptionSpellEffect::ComponentSuppressedHitPointBuff { .. }
        | BattleMetamagicOptionSpellEffect::DurationExtension { .. } => None,
    };
    if let Some(target_hit_points_after) = target_hit_points_after {
        state.skeleton.hp = target_hit_points_after;
    }
    if let BattleMetamagicOptionSpellEffect::ComponentSuppressedHitPointBuff {
        temporary_hit_points,
        ..
    } = effect
    {
        state.fighter.temporary_hp = temporary_hit_points;
    }
    state
}

const fn metamagic_spell_success_substrate(
    effect: BattleMetamagicOptionSpellEffect,
) -> BattleMetamagicSpellSubstrate {
    match effect {
        BattleMetamagicOptionSpellEffect::ProtectedSaveGatedDamage {
            protected_save_targets,
            ..
        } => BattleMetamagicSpellSubstrate {
            offered: false,
            protected_save_targets,
            outcome: BattleMetamagicSpellOutcome::ProtectedSaveGatedDamage,
            protocol: BattleMetamagicSpellProtocol::Resolved,
            ..BattleMetamagicSpellSubstrate::initial()
        },
        BattleMetamagicOptionSpellEffect::ProtectedSaveGatedNoEffect {
            protected_save_targets,
        } => BattleMetamagicSpellSubstrate {
            offered: false,
            protected_save_targets,
            outcome: BattleMetamagicSpellOutcome::ProtectedSaveGatedNoEffect,
            protocol: BattleMetamagicSpellProtocol::Resolved,
            ..BattleMetamagicSpellSubstrate::initial()
        },
        BattleMetamagicOptionSpellEffect::FirstTargetDisadvantageSaveGatedDamage { .. } => {
            first_target_disadvantage_substrate(
                0,
                BattleMetamagicSpellOutcome::FirstTargetDisadvantageSaveGatedDamage,
            )
        }
        BattleMetamagicOptionSpellEffect::FirstTargetDisadvantageCondition {
            target_active_effect_count,
        } => first_target_disadvantage_substrate(
            target_active_effect_count,
            BattleMetamagicSpellOutcome::FirstTargetDisadvantageCondition,
        ),
        BattleMetamagicOptionSpellEffect::FirstTargetDisadvantageEntrySave => {
            first_target_disadvantage_substrate(
                0,
                BattleMetamagicSpellOutcome::FirstTargetDisadvantageEntrySave,
            )
        }
        BattleMetamagicOptionSpellEffect::FirstTargetDisadvantageEndTurnSave => {
            first_target_disadvantage_substrate(
                0,
                BattleMetamagicSpellOutcome::FirstTargetDisadvantageEndTurnSave,
            )
        }
        BattleMetamagicOptionSpellEffect::FirstTargetDisadvantageConditionEndTurnSave => {
            first_target_disadvantage_substrate(
                0,
                BattleMetamagicSpellOutcome::FirstTargetDisadvantageConditionEndTurnSave,
            )
        }
        BattleMetamagicOptionSpellEffect::ObjectRangeLight {
            bright_radius_feet,
            dim_additional_feet,
        } => BattleMetamagicSpellSubstrate {
            offered: false,
            light_emitter_count: 1,
            bright_radius_feet,
            dim_additional_feet,
            outcome: BattleMetamagicSpellOutcome::ObjectRangeLight,
            protocol: BattleMetamagicSpellProtocol::Resolved,
            ..BattleMetamagicSpellSubstrate::initial()
        },
        BattleMetamagicOptionSpellEffect::AdditionalSingleTarget {
            target_active_effect_count,
        } => BattleMetamagicSpellSubstrate {
            offered: false,
            target_active_effect_count,
            outcome: BattleMetamagicSpellOutcome::AdditionalSingleTarget,
            protocol: BattleMetamagicSpellProtocol::Resolved,
            ..BattleMetamagicSpellSubstrate::initial()
        },
        BattleMetamagicOptionSpellEffect::HitPointRestoration { .. }
        | BattleMetamagicOptionSpellEffect::SaveGatedCondition
        | BattleMetamagicOptionSpellEffect::SaveGatedConditionImmunity
        | BattleMetamagicOptionSpellEffect::DirectCondition
        | BattleMetamagicOptionSpellEffect::RollModifier
        | BattleMetamagicOptionSpellEffect::SaveGatedDamage { .. }
        | BattleMetamagicOptionSpellEffect::SpellAttack { .. }
        | BattleMetamagicOptionSpellEffect::SpellAttackSequence { .. }
        | BattleMetamagicOptionSpellEffect::DamageReroll { .. }
        | BattleMetamagicOptionSpellEffect::SpellAttackReroll { .. }
        | BattleMetamagicOptionSpellEffect::ComponentSuppressedHitPointBuff { .. }
        | BattleMetamagicOptionSpellEffect::DamageTypeSubstitutionSaveGatedDamage { .. }
        | BattleMetamagicOptionSpellEffect::DamageTypeSubstitutionSpellAttack { .. }
        | BattleMetamagicOptionSpellEffect::DurationExtension { .. } => {
            BattleMetamagicSpellSubstrate::initial()
        }
    }
}

const fn first_target_disadvantage_substrate(
    target_active_effect_count: i16,
    outcome: BattleMetamagicSpellOutcome,
) -> BattleMetamagicSpellSubstrate {
    BattleMetamagicSpellSubstrate {
        offered: false,
        first_target_save_disadvantage: true,
        target_active_effect_count,
        outcome,
        protocol: BattleMetamagicSpellProtocol::Resolved,
        ..BattleMetamagicSpellSubstrate::initial()
    }
}

const fn metamagic_success_substrate(
    effect: BattleMetamagicOptionSpellEffect,
) -> BattleMetamagicOptionSpellSubstrate {
    BattleMetamagicOptionSpellSubstrate {
        offered: false,
        projection: match effect {
            BattleMetamagicOptionSpellEffect::DamageReroll {
                target_active_effect_count,
                ..
            } => BattleMetamagicOptionSpellProjection::EmpoweredDamageReroll {
                target_active_effect_count,
            },
            BattleMetamagicOptionSpellEffect::SpellAttackReroll {
                target_active_effect_count,
                ..
            } => BattleMetamagicOptionSpellProjection::SeekingAttackReroll {
                target_active_effect_count,
            },
            BattleMetamagicOptionSpellEffect::ComponentSuppressedHitPointBuff {
                verbal_suppressed,
                somatic_suppressed,
                material_suppressed,
                material_preserved,
                ..
            } => BattleMetamagicOptionSpellProjection::SubtleComponents {
                verbal_suppressed,
                somatic_suppressed,
                material_suppressed,
                material_preserved,
            },
            BattleMetamagicOptionSpellEffect::DamageTypeSubstitutionSaveGatedDamage { .. } => {
                BattleMetamagicOptionSpellProjection::TransmutedSaveGatedDamage
            }
            BattleMetamagicOptionSpellEffect::DamageTypeSubstitutionSpellAttack {
                target_active_effect_count,
                ..
            } => BattleMetamagicOptionSpellProjection::TransmutedSpellAttack {
                target_active_effect_count,
            },
            BattleMetamagicOptionSpellEffect::DurationExtension {
                duration_ticks,
                concentration_saving_throw_mode,
            } => BattleMetamagicOptionSpellProjection::ExtendedDuration {
                duration_ticks,
                concentration_saving_throw_mode,
            },
            BattleMetamagicOptionSpellEffect::HitPointRestoration { .. }
            | BattleMetamagicOptionSpellEffect::SaveGatedCondition
            | BattleMetamagicOptionSpellEffect::SaveGatedConditionImmunity
            | BattleMetamagicOptionSpellEffect::DirectCondition
            | BattleMetamagicOptionSpellEffect::RollModifier
            | BattleMetamagicOptionSpellEffect::SaveGatedDamage { .. }
            | BattleMetamagicOptionSpellEffect::SpellAttack { .. }
            | BattleMetamagicOptionSpellEffect::SpellAttackSequence { .. }
            | BattleMetamagicOptionSpellEffect::ProtectedSaveGatedDamage { .. }
            | BattleMetamagicOptionSpellEffect::ProtectedSaveGatedNoEffect { .. }
            | BattleMetamagicOptionSpellEffect::FirstTargetDisadvantageSaveGatedDamage { .. }
            | BattleMetamagicOptionSpellEffect::FirstTargetDisadvantageCondition { .. }
            | BattleMetamagicOptionSpellEffect::FirstTargetDisadvantageEntrySave
            | BattleMetamagicOptionSpellEffect::FirstTargetDisadvantageEndTurnSave
            | BattleMetamagicOptionSpellEffect::FirstTargetDisadvantageConditionEndTurnSave
            | BattleMetamagicOptionSpellEffect::ObjectRangeLight { .. }
            | BattleMetamagicOptionSpellEffect::AdditionalSingleTarget { .. } => {
                BattleMetamagicOptionSpellProjection::Init
            }
        },
    }
}

const fn quickened_success_substrate(
    effect: BattleMetamagicOptionSpellEffect,
    magic_action_available_before_resolution: bool,
) -> BattleQuickenedSpellSubstrate {
    match effect {
        BattleMetamagicOptionSpellEffect::HitPointRestoration { .. } => {
            quickened_resolved_substrate(
                false,
                false,
                false,
                false,
                0,
                if magic_action_available_before_resolution {
                    QuickenedSpellScenarioOutcome::ResolvedQuickenedRestoration
                } else {
                    QuickenedSpellScenarioOutcome::ResolvedAfterMagicActionSpent
                },
                QuickenedMetamagicScenarioResult::Init,
            )
        }
        BattleMetamagicOptionSpellEffect::SaveGatedCondition => quickened_resolved_substrate(
            true,
            false,
            false,
            false,
            0,
            QuickenedSpellScenarioOutcome::ResolvedQuickenedSaveGatedCondition,
            QuickenedMetamagicScenarioResult::Init,
        ),
        BattleMetamagicOptionSpellEffect::SaveGatedConditionImmunity => {
            quickened_resolved_substrate(
                false,
                true,
                false,
                false,
                0,
                QuickenedSpellScenarioOutcome::ResolvedQuickenedSaveGatedConditionImmunity,
                QuickenedMetamagicScenarioResult::Init,
            )
        }
        BattleMetamagicOptionSpellEffect::DirectCondition => quickened_resolved_substrate(
            false,
            false,
            true,
            false,
            0,
            QuickenedSpellScenarioOutcome::ResolvedQuickenedDirectCondition,
            QuickenedMetamagicScenarioResult::Init,
        ),
        BattleMetamagicOptionSpellEffect::RollModifier => quickened_resolved_substrate(
            false,
            false,
            false,
            true,
            0,
            QuickenedSpellScenarioOutcome::ResolvedQuickenedRollModifier,
            QuickenedMetamagicScenarioResult::Init,
        ),
        BattleMetamagicOptionSpellEffect::SaveGatedDamage { .. } => quickened_resolved_substrate(
            false,
            false,
            false,
            false,
            0,
            QuickenedSpellScenarioOutcome::Init,
            QuickenedMetamagicScenarioResult::QuickenedSaveGatedDamage,
        ),
        BattleMetamagicOptionSpellEffect::SpellAttack {
            target_active_effect_count,
            ..
        } => quickened_resolved_substrate(
            false,
            false,
            false,
            false,
            target_active_effect_count,
            QuickenedSpellScenarioOutcome::Init,
            QuickenedMetamagicScenarioResult::QuickenedSpellAttack,
        ),
        BattleMetamagicOptionSpellEffect::SpellAttackSequence {
            target_active_effect_count,
            ..
        } => quickened_resolved_substrate(
            false,
            false,
            false,
            false,
            target_active_effect_count,
            QuickenedSpellScenarioOutcome::Init,
            QuickenedMetamagicScenarioResult::QuickenedSpellAttackSequence,
        ),
        BattleMetamagicOptionSpellEffect::DamageReroll { .. }
        | BattleMetamagicOptionSpellEffect::SpellAttackReroll { .. }
        | BattleMetamagicOptionSpellEffect::ComponentSuppressedHitPointBuff { .. }
        | BattleMetamagicOptionSpellEffect::DamageTypeSubstitutionSaveGatedDamage { .. }
        | BattleMetamagicOptionSpellEffect::DamageTypeSubstitutionSpellAttack { .. }
        | BattleMetamagicOptionSpellEffect::DurationExtension { .. }
        | BattleMetamagicOptionSpellEffect::ProtectedSaveGatedDamage { .. }
        | BattleMetamagicOptionSpellEffect::ProtectedSaveGatedNoEffect { .. }
        | BattleMetamagicOptionSpellEffect::FirstTargetDisadvantageSaveGatedDamage { .. }
        | BattleMetamagicOptionSpellEffect::FirstTargetDisadvantageCondition { .. }
        | BattleMetamagicOptionSpellEffect::FirstTargetDisadvantageEntrySave
        | BattleMetamagicOptionSpellEffect::FirstTargetDisadvantageEndTurnSave
        | BattleMetamagicOptionSpellEffect::FirstTargetDisadvantageConditionEndTurnSave
        | BattleMetamagicOptionSpellEffect::ObjectRangeLight { .. }
        | BattleMetamagicOptionSpellEffect::AdditionalSingleTarget { .. } => {
            BattleQuickenedSpellSubstrate::initial()
        }
    }
}

const fn quickened_resolved_substrate(
    color_spray_blinded: bool,
    calm_emotions_immunity: bool,
    invisibility_active: bool,
    bless_active: bool,
    target_active_effect_count: i16,
    governor_outcome: QuickenedSpellScenarioOutcome,
    metamagic_outcome: QuickenedMetamagicScenarioResult,
) -> BattleQuickenedSpellSubstrate {
    BattleQuickenedSpellSubstrate {
        offered: false,
        color_spray_blinded,
        calm_emotions_immunity,
        invisibility_active,
        bless_active,
        spell_slot_committed: true,
        target_active_effect_count,
        invalid_kind: QuickenedSpellInvalidKind::None,
        governor_outcome,
        metamagic_outcome,
        governor_protocol: QuickenedSpellProtocol::Resolved,
        metamagic_protocol: QuickenedMetamagicProtocol::Resolved,
    }
}

fn breath_weapon_invalid_holes(
    outcome: DragonbornBreathWeaponScenarioOutcome,
) -> Vec<BattleHoleKind> {
    match outcome {
        DragonbornBreathWeaponScenarioOutcome::RejectMismatchedArea => {
            vec![BattleHoleKind::SavingThrowOutcome]
        }
        DragonbornBreathWeaponScenarioOutcome::RejectInvalidDamageRoll => {
            vec![BattleHoleKind::RolledDice]
        }
        DragonbornBreathWeaponScenarioOutcome::RejectMissingResource
        | DragonbornBreathWeaponScenarioOutcome::Init
        | DragonbornBreathWeaponScenarioOutcome::Resolved
        | DragonbornBreathWeaponScenarioOutcome::OpenedExtraAttack => Vec::new(),
    }
}

fn innate_sorcery_spell_benefit_projection(
    spell_facts: InnateSorcerySpellFacts,
) -> (InnateSorceryScenarioOutcome, i16) {
    match spell_facts.benefit_eligibility {
        InnateSorcerySpellBenefitEligibility::Eligible => {
            (InnateSorceryScenarioOutcome::SpellBenefitsProjected, 1)
        }
        InnateSorcerySpellBenefitEligibility::Ineligible => {
            (InnateSorceryScenarioOutcome::NonSorcererExcluded, 0)
        }
    }
}

fn apply_innate_sorcery_spell_benefit(
    mut state: BattleState,
    current_round: i16,
    scenario_outcome: InnateSorceryScenarioOutcome,
    spell_save_dc_bonus: i16,
    spell_attack_roll_mode: InnateSorcerySpellAttackRollMode,
) -> BattleState {
    state.bonus_action_available = false;
    state.feature_substrates.innate_sorcery = BattleInnateSorcerySubstrate {
        uses_remaining: state
            .feature_substrates
            .innate_sorcery
            .uses_remaining
            .saturating_sub(1),
        occurrence: InnateSorceryOccurrence::ActiveUntilEndOfRound(current_round + 10),
        spell_save_dc_bonus,
        spell_attack_roll_mode,
        scenario_outcome,
        protocol: InnateSorceryProtocol::Resolved,
        ..state.feature_substrates.innate_sorcery
    };
    state
}

fn scalar_buff_protocol_from_result(result: &BattleResolutionResult) -> ScalarBuffTargetProtocol {
    match result.outcome() {
        BattleResolutionOutcome::NeedsHoles => ScalarBuffTargetProtocol::Init,
        BattleResolutionOutcome::Resolved => ScalarBuffTargetProtocol::Resolved,
        BattleResolutionOutcome::Invalid(BattleResolutionInvalidReason::StaleSubject) => {
            ScalarBuffTargetProtocol::Invalid(ScalarBuffTargetInvalidReason::StaleSubject)
        }
        BattleResolutionOutcome::Invalid(
            BattleResolutionInvalidReason::InvalidFill
            | BattleResolutionInvalidReason::MetamagicOptionEffectMismatch
            | BattleResolutionInvalidReason::WrongActor
            | BattleResolutionInvalidReason::WrongTarget,
        ) => ScalarBuffTargetProtocol::Invalid(ScalarBuffTargetInvalidReason::StaleSubject),
    }
}

fn scalar_buff_holes_from_result(result: &BattleResolutionResult) -> Vec<ScalarBuffTargetHole> {
    match result.requested_holes() {
        Some(holes) if holes.contains(&BattleHoleKind::TargetChoice) => {
            vec![ScalarBuffTargetHole::TargetChoice]
        }
        Some(_) | None => Vec::new(),
    }
}

#[must_use]
pub fn resolve_shield_reaction_spell_hit_battle(state: BattleState) -> BattleState {
    // QNT: battle-runtime-reaction-window.qnt `castShieldReactionSpell`;
    // battle-runtime-reaction-resolution.qnt `resolveReactionOffer` AttackHit
    // Shield branch.
    resolve_reaction_spell_subject(
        state,
        BattleReactionSpellFill::ArmorClassInterruption(
            BattleReactionArmorClassInterruptionFacts {
                reactor: Actor::Fighter,
                armor_class_bonus: 5,
                slot_level: BattleSpellSlotLevel::First,
            },
        ),
    )
    .state()
    .clone()
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
        ..state
    };
    if !can_actor_expend_spell_slot_this_turn(&state, Actor::Fighter)
        || !state.fighter.reaction_available
    {
        return state;
    }
    let state = resolve_reaction_spell_subject(
        state,
        BattleReactionSpellFill::FailedSaveDamage(BattleReactionFailedSaveDamageFacts {
            reactor: Actor::Fighter,
            trigger_creature: Actor::Goblin,
            reactor_damage_taken: 1,
            damage: 3,
            slot_level: BattleSpellSlotLevel::Second,
        }),
    )
    .state()
    .clone();
    BattleState {
        reaction_casting_time: BattleReactionCastingTimeState {
            trigger: BattleReactionCastingTrigger::AfterDamage,
            continuation: BattleReactionCastingContinuation::AfterDamageResolved,
            reaction_window_open: false,
            outcome: BattleReactionCastingOutcome::AfterDamageReactionResolved,
        },
        ..state
    }
}

#[must_use]
pub fn resolve_hellish_rebuke_failed_save_reaction_spell_battle(state: BattleState) -> BattleState {
    // QNT: battle-runtime-reaction-spell-selected-identity.mbt.qnt
    // failed-save selected-identity branch; support QNT:
    // battle-runtime-reaction-resolution.qnt `resolveHellishRebukeAfterDamage`.
    if !can_actor_expend_spell_slot_this_turn(&state, Actor::Fighter)
        || !state.fighter.reaction_available
    {
        return state;
    }
    resolve_reaction_spell_subject(
        state,
        BattleReactionSpellFill::FailedSaveDamage(BattleReactionFailedSaveDamageFacts {
            reactor: Actor::Fighter,
            trigger_creature: Actor::Goblin,
            reactor_damage_taken: 1,
            damage: 3,
            slot_level: BattleSpellSlotLevel::Second,
        }),
    )
    .state()
    .clone()
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

#[must_use]
pub fn weapon_mastery_selected_identity_from_battle(
    state: &BattleState,
) -> WeaponMasterySelectedIdentityState {
    let mastery = state.feature_substrates.weapon_mastery;
    let protocol = weapon_mastery_protocol_for_outcome(mastery.outcome);
    WeaponMasterySelectedIdentityState {
        primary_target_hit_points: state.skeleton.hp,
        second_target_hit_points: state.goblin.hp,
        action_available: state.action_available,
        primary_target_has_sap_effect: mastery.sap_target == Some(Actor::Skeleton),
        primary_target_prone: state.skeleton.prone,
        cleave_used: mastery.cleave_used_this_turn,
        outcome: mastery.outcome,
        protocol,
    }
}

#[must_use]
pub fn dragonborn_breath_weapon_from_battle(state: &BattleState) -> DragonbornBreathWeaponState {
    let breath = state.feature_substrates.dragonborn_breath_weapon;
    DragonbornBreathWeaponState {
        target_hit_points: state.skeleton.hp,
        second_target_hit_points: state.goblin.hp,
        breath_weapon_uses_remaining: breath.uses_remaining,
        attack_action_attacks_remaining: breath.attack_action_attacks_remaining,
        scenario_outcome: breath.scenario_outcome,
        protocol: breath.protocol,
    }
}

#[must_use]
pub fn innate_sorcery_from_battle(state: &BattleState) -> InnateSorceryState {
    let innate = state.feature_substrates.innate_sorcery;
    InnateSorceryState {
        bonus_action_available: state.bonus_action_available,
        feature_uses_remaining: innate.uses_remaining,
        occurrence: innate.occurrence,
        spell_save_dc: innate.base_spell_save_dc + innate.spell_save_dc_bonus,
        spell_attack_roll_mode: innate.spell_attack_roll_mode,
        scenario_outcome: innate.scenario_outcome,
        protocol: innate.protocol,
    }
}

#[must_use]
pub fn quickened_spell_governor_from_battle(state: &BattleState) -> QuickenedSpellGovernorState {
    let quickened = state.feature_substrates.quickened_spell;
    let actor = current_actor(state);
    QuickenedSpellGovernorState {
        quickened_cure_wounds_offered: quickened.offered,
        color_spray_blinded: quickened.color_spray_blinded,
        calm_emotions_immunity: quickened.calm_emotions_immunity,
        invisibility_active: quickened.invisibility_active,
        bless_active: quickened.bless_active,
        magic_action_available: state.action_available,
        bonus_action_available: state.bonus_action_available,
        sorcery_points_remaining: resource_pool_remaining(state.feature_resources.sorcery_points),
        target_hit_points: state.skeleton.hp,
        spell_slot_committed: quickened.spell_slot_committed,
        level_one_plus_cast_this_turn: state
            .level_one_plus_spell_casters_this_turn
            .contains(&actor),
        quickened_level_one_plus_cast_this_turn: state
            .quickened_level_one_plus_spell_casters_this_turn
            .contains(&actor),
        spell_slot_acts_available: can_actor_expend_spell_slot_this_turn(state, actor),
        invalid_kind: quickened.invalid_kind,
        scenario_outcome: quickened.governor_outcome,
        protocol: quickened.governor_protocol,
    }
}

#[must_use]
pub fn quickened_metamagic_from_battle(state: &BattleState) -> QuickenedMetamagicState {
    let quickened = state.feature_substrates.quickened_spell;
    QuickenedMetamagicState {
        magic_action_available: state.action_available,
        bonus_action_available: state.bonus_action_available,
        sorcery_points_remaining: resource_pool_remaining(state.feature_resources.sorcery_points),
        target_hit_points: state.skeleton.hp,
        target_active_effect_count: quickened.target_active_effect_count,
        scenario_result: quickened.metamagic_outcome,
        protocol: quickened.metamagic_protocol,
    }
}

#[must_use]
pub fn careful_spell_from_battle(state: &BattleState) -> CarefulSpellState {
    let metamagic = state.feature_substrates.metamagic_spell;
    CarefulSpellState {
        magic_action_available: state.action_available,
        bonus_action_available: state.bonus_action_available,
        sorcery_points_remaining: resource_pool_remaining(state.feature_resources.sorcery_points),
        target_hit_points: state.skeleton.hp,
        target_active_effect_count: metamagic.target_active_effect_count,
        scenario_result: match (metamagic.outcome, metamagic.protected_save_targets > 0) {
            (BattleMetamagicSpellOutcome::ProtectedSaveGatedDamage, true) => {
                CarefulSpellScenarioResult::CarefulSaveGatedDamage
            }
            (BattleMetamagicSpellOutcome::ProtectedSaveGatedNoEffect, true) => {
                CarefulSpellScenarioResult::CarefulSaveGatedNoEffect
            }
            (
                BattleMetamagicSpellOutcome::Init
                | BattleMetamagicSpellOutcome::ProtectedSaveGatedDamage
                | BattleMetamagicSpellOutcome::ProtectedSaveGatedNoEffect
                | BattleMetamagicSpellOutcome::FirstTargetDisadvantageSaveGatedDamage
                | BattleMetamagicSpellOutcome::FirstTargetDisadvantageCondition
                | BattleMetamagicSpellOutcome::FirstTargetDisadvantageEntrySave
                | BattleMetamagicSpellOutcome::FirstTargetDisadvantageEndTurnSave
                | BattleMetamagicSpellOutcome::FirstTargetDisadvantageConditionEndTurnSave
                | BattleMetamagicSpellOutcome::ObjectRangeLight
                | BattleMetamagicSpellOutcome::AdditionalSingleTarget,
                _,
            ) => CarefulSpellScenarioResult::Init,
        },
        protocol: match metamagic.protocol {
            BattleMetamagicSpellProtocol::Init => CarefulSpellProtocol::Init,
            BattleMetamagicSpellProtocol::Resolved => CarefulSpellProtocol::Resolved,
        },
    }
}

#[must_use]
pub fn heightened_spell_from_battle(state: &BattleState) -> HeightenedSpellState {
    let metamagic = state.feature_substrates.metamagic_spell;
    HeightenedSpellState {
        magic_action_available: state.action_available,
        bonus_action_available: state.bonus_action_available,
        sorcery_points_remaining: resource_pool_remaining(state.feature_resources.sorcery_points),
        target_hit_points: state.skeleton.hp,
        target_active_effect_count: metamagic.target_active_effect_count,
        scenario_result: match (metamagic.outcome, metamagic.first_target_save_disadvantage) {
            (BattleMetamagicSpellOutcome::FirstTargetDisadvantageSaveGatedDamage, true) => {
                HeightenedSpellScenarioResult::HeightenedSaveGatedDamage
            }
            (BattleMetamagicSpellOutcome::FirstTargetDisadvantageCondition, true) => {
                HeightenedSpellScenarioResult::HeightenedHideousLaughter
            }
            (BattleMetamagicSpellOutcome::FirstTargetDisadvantageEntrySave, true) => {
                HeightenedSpellScenarioResult::HeightenedGreaseEntrySave
            }
            (BattleMetamagicSpellOutcome::FirstTargetDisadvantageEndTurnSave, true) => {
                HeightenedSpellScenarioResult::HeightenedGustOfWindEndTurnSave
            }
            (BattleMetamagicSpellOutcome::FirstTargetDisadvantageConditionEndTurnSave, true) => {
                HeightenedSpellScenarioResult::HeightenedSaveGatedConditionEndTurnSave
            }
            (
                BattleMetamagicSpellOutcome::Init
                | BattleMetamagicSpellOutcome::ProtectedSaveGatedDamage
                | BattleMetamagicSpellOutcome::ProtectedSaveGatedNoEffect
                | BattleMetamagicSpellOutcome::FirstTargetDisadvantageSaveGatedDamage
                | BattleMetamagicSpellOutcome::FirstTargetDisadvantageCondition
                | BattleMetamagicSpellOutcome::FirstTargetDisadvantageEntrySave
                | BattleMetamagicSpellOutcome::FirstTargetDisadvantageEndTurnSave
                | BattleMetamagicSpellOutcome::FirstTargetDisadvantageConditionEndTurnSave
                | BattleMetamagicSpellOutcome::ObjectRangeLight
                | BattleMetamagicSpellOutcome::AdditionalSingleTarget,
                _,
            ) => HeightenedSpellScenarioResult::Init,
        },
        protocol: match metamagic.protocol {
            BattleMetamagicSpellProtocol::Init => HeightenedSpellProtocol::Init,
            BattleMetamagicSpellProtocol::Resolved => HeightenedSpellProtocol::Resolved,
        },
    }
}

#[must_use]
pub fn distant_spell_from_battle(state: &BattleState) -> DistantSpellState {
    let metamagic = state.feature_substrates.metamagic_spell;
    DistantSpellState {
        sorcery_points_remaining: resource_pool_remaining(state.feature_resources.sorcery_points),
        light_emitter_count: metamagic.light_emitter_count,
        bright_radius_feet: metamagic.bright_radius_feet,
        dim_additional_feet: metamagic.dim_additional_feet,
        scenario_result: match metamagic.outcome {
            BattleMetamagicSpellOutcome::ObjectRangeLight => {
                DistantSpellScenarioResult::DistantObjectLight
            }
            BattleMetamagicSpellOutcome::Init
            | BattleMetamagicSpellOutcome::ProtectedSaveGatedDamage
            | BattleMetamagicSpellOutcome::ProtectedSaveGatedNoEffect
            | BattleMetamagicSpellOutcome::FirstTargetDisadvantageSaveGatedDamage
            | BattleMetamagicSpellOutcome::FirstTargetDisadvantageCondition
            | BattleMetamagicSpellOutcome::FirstTargetDisadvantageEntrySave
            | BattleMetamagicSpellOutcome::FirstTargetDisadvantageEndTurnSave
            | BattleMetamagicSpellOutcome::FirstTargetDisadvantageConditionEndTurnSave
            | BattleMetamagicSpellOutcome::AdditionalSingleTarget => {
                DistantSpellScenarioResult::Init
            }
        },
        protocol: match metamagic.protocol {
            BattleMetamagicSpellProtocol::Init => DistantSpellProtocol::Init,
            BattleMetamagicSpellProtocol::Resolved => DistantSpellProtocol::Resolved,
        },
    }
}

#[must_use]
pub fn twinned_spell_from_battle(state: &BattleState) -> TwinnedSpellState {
    let metamagic = state.feature_substrates.metamagic_spell;
    TwinnedSpellState {
        magic_action_available: state.action_available,
        bonus_action_available: state.bonus_action_available,
        sorcery_points_remaining: resource_pool_remaining(state.feature_resources.sorcery_points),
        target_hit_points: state.skeleton.hp,
        target_active_effect_count: metamagic.target_active_effect_count,
        scenario_result: match metamagic.outcome {
            BattleMetamagicSpellOutcome::AdditionalSingleTarget => {
                TwinnedSpellScenarioResult::TwinnedTargetCount
            }
            BattleMetamagicSpellOutcome::Init
            | BattleMetamagicSpellOutcome::ProtectedSaveGatedDamage
            | BattleMetamagicSpellOutcome::ProtectedSaveGatedNoEffect
            | BattleMetamagicSpellOutcome::FirstTargetDisadvantageSaveGatedDamage
            | BattleMetamagicSpellOutcome::FirstTargetDisadvantageCondition
            | BattleMetamagicSpellOutcome::FirstTargetDisadvantageEntrySave
            | BattleMetamagicSpellOutcome::FirstTargetDisadvantageEndTurnSave
            | BattleMetamagicSpellOutcome::FirstTargetDisadvantageConditionEndTurnSave
            | BattleMetamagicSpellOutcome::ObjectRangeLight => TwinnedSpellScenarioResult::Init,
        },
        protocol: match metamagic.protocol {
            BattleMetamagicSpellProtocol::Init => TwinnedSpellProtocol::Init,
            BattleMetamagicSpellProtocol::Resolved => TwinnedSpellProtocol::Resolved,
        },
    }
}

#[must_use]
pub fn empowered_spell_from_battle(state: &BattleState) -> EmpoweredSpellState {
    let projection = state.feature_substrates.metamagic_option_spell.projection;
    let (target_active_effect_count, scenario_result, protocol) = match projection {
        BattleMetamagicOptionSpellProjection::EmpoweredDamageReroll {
            target_active_effect_count,
        } => (
            target_active_effect_count,
            EmpoweredSpellScenarioResult::EmpoweredSpellDamageReroll,
            EmpoweredSpellProtocol::Resolved,
        ),
        _ => (
            0,
            EmpoweredSpellScenarioResult::Init,
            EmpoweredSpellProtocol::Init,
        ),
    };
    EmpoweredSpellState {
        magic_action_available: state.action_available,
        bonus_action_available: state.bonus_action_available,
        sorcery_points_remaining: resource_pool_remaining(state.feature_resources.sorcery_points),
        target_hit_points: state.skeleton.hp,
        target_active_effect_count,
        scenario_result,
        protocol,
    }
}

#[must_use]
pub fn seeking_spell_from_battle(state: &BattleState) -> SeekingSpellState {
    let projection = state.feature_substrates.metamagic_option_spell.projection;
    let (target_active_effect_count, scenario_result, protocol) = match projection {
        BattleMetamagicOptionSpellProjection::SeekingAttackReroll {
            target_active_effect_count,
        } => (
            target_active_effect_count,
            SeekingSpellScenarioResult::SeekingSpellAttackReroll,
            SeekingSpellProtocol::Resolved,
        ),
        _ => (
            0,
            SeekingSpellScenarioResult::Init,
            SeekingSpellProtocol::Init,
        ),
    };
    SeekingSpellState {
        magic_action_available: state.action_available,
        bonus_action_available: state.bonus_action_available,
        sorcery_points_remaining: resource_pool_remaining(state.feature_resources.sorcery_points),
        target_hit_points: state.skeleton.hp,
        target_active_effect_count,
        scenario_result,
        protocol,
    }
}

#[must_use]
pub fn subtle_spell_from_battle(state: &BattleState) -> SubtleSpellState {
    let projection = state.feature_substrates.metamagic_option_spell.projection;
    let (
        verbal_suppressed,
        somatic_suppressed,
        material_suppressed,
        material_preserved,
        scenario_result,
        protocol,
    ) = match projection {
        BattleMetamagicOptionSpellProjection::SubtleComponents {
            verbal_suppressed,
            somatic_suppressed,
            material_suppressed,
            material_preserved,
        } => (
            verbal_suppressed,
            somatic_suppressed,
            material_suppressed,
            material_preserved,
            SubtleSpellScenarioResult::SubtleFalseLife,
            SubtleSpellProtocol::Resolved,
        ),
        BattleMetamagicOptionSpellProjection::SubtleUnaffordable => (
            false,
            false,
            false,
            false,
            SubtleSpellScenarioResult::UnaffordableSubtleFalseLife,
            SubtleSpellProtocol::InvalidUnsupportedActOption,
        ),
        _ => (
            false,
            false,
            false,
            false,
            SubtleSpellScenarioResult::Init,
            SubtleSpellProtocol::Init,
        ),
    };
    SubtleSpellState {
        verbal_suppressed,
        somatic_suppressed,
        material_suppressed,
        material_preserved,
        sorcery_points_remaining: resource_pool_remaining(state.feature_resources.sorcery_points),
        temporary_hit_points: state.fighter.temporary_hp,
        scenario_result,
        protocol,
    }
}

#[must_use]
pub fn transmuted_spell_from_battle(state: &BattleState) -> TransmutedSpellState {
    let projection = state.feature_substrates.metamagic_option_spell.projection;
    let (target_active_effect_count, scenario_result, protocol) = match projection {
        BattleMetamagicOptionSpellProjection::TransmutedSaveGatedDamage => (
            0,
            TransmutedSpellScenarioResult::TransmutedSaveGatedDamage,
            TransmutedSpellProtocol::Resolved,
        ),
        BattleMetamagicOptionSpellProjection::TransmutedSpellAttack {
            target_active_effect_count,
        } => (
            target_active_effect_count,
            TransmutedSpellScenarioResult::TransmutedSpellAttack,
            TransmutedSpellProtocol::Resolved,
        ),
        _ => (
            0,
            TransmutedSpellScenarioResult::Init,
            TransmutedSpellProtocol::Init,
        ),
    };
    TransmutedSpellState {
        magic_action_available: state.action_available,
        bonus_action_available: state.bonus_action_available,
        sorcery_points_remaining: resource_pool_remaining(state.feature_resources.sorcery_points),
        target_hit_points: state.skeleton.hp,
        target_active_effect_count,
        scenario_result,
        protocol,
    }
}

#[must_use]
pub fn extended_spell_from_battle(state: &BattleState) -> ExtendedSpellState {
    let projection = state.feature_substrates.metamagic_option_spell.projection;
    let (duration_ticks, concentration_saving_throw_mode, scenario_result, protocol) =
        match projection {
            BattleMetamagicOptionSpellProjection::ExtendedDuration {
                duration_ticks,
                concentration_saving_throw_mode,
            } => (
                duration_ticks,
                concentration_saving_throw_mode,
                ExtendedSpellScenarioResult::ExtendedCreatureSizeIncrease,
                ExtendedSpellProtocol::Resolved,
            ),
            _ => (
                0,
                ExtendedSpellConcentrationSaveMode::Normal,
                ExtendedSpellScenarioResult::Init,
                ExtendedSpellProtocol::Init,
            ),
        };
    ExtendedSpellState {
        sorcery_points_remaining: resource_pool_remaining(state.feature_resources.sorcery_points),
        duration_ticks,
        concentration_saving_throw_mode,
        scenario_result,
        protocol,
    }
}

fn weapon_mastery_protocol_for_outcome(
    outcome: WeaponMasteryRuntimeOutcome,
) -> WeaponMasteryRuntimeProtocol {
    match outcome {
        WeaponMasteryRuntimeOutcome::Init => {
            WeaponMasteryRuntimeProtocol::Init(vec![WeaponMasteryRuntimeHole::WitnessProtocol])
        }
        WeaponMasteryRuntimeOutcome::NeedsHoles => WeaponMasteryRuntimeProtocol::NeedsHoles(vec![
            WeaponMasteryRuntimeHole::WitnessProtocol,
        ]),
        WeaponMasteryRuntimeOutcome::Resolved => WeaponMasteryRuntimeProtocol::Resolved,
    }
}

fn fighter_combatant() -> Combatant {
    Combatant {
        hp: 12,
        max_hp: 12,
        temporary_hp: 0,
        armor_class: 10,
        speed_feet: 30,
        shield_armor_class_bonus_active: false,
        unconscious: false,
        incapacitated: false,
        prone: false,
        creature_size: CreatureSize::Medium,
        lifecycle: CombatantLifecycle::UsesDeathSavingThrows(DeathSavingThrowLifecycle::fresh()),
        reaction_available: true,
        movement_spent_feet: 0,
        weapon_attack_supported: true,
        weapon_damage_modifier: 0,
        multiattack_profile: None,
        sneak_attack_supported: false,
        sneak_attack_used_this_turn: false,
        recharge_available: false,
        spell_slots: BattleSpellSlotLedger::none(),
        spell_active_effects: BattleSpellActiveEffects::none(),
    }
}

fn goblin_combatant() -> Combatant {
    Combatant {
        hp: 10,
        max_hp: 10,
        temporary_hp: 0,
        armor_class: 15,
        speed_feet: 30,
        shield_armor_class_bonus_active: false,
        unconscious: false,
        incapacitated: false,
        prone: false,
        creature_size: CreatureSize::Small,
        lifecycle: CombatantLifecycle::DiesAtZeroHitPoints,
        reaction_available: true,
        movement_spent_feet: 0,
        weapon_attack_supported: false,
        weapon_damage_modifier: 0,
        multiattack_profile: None,
        sneak_attack_supported: false,
        sneak_attack_used_this_turn: false,
        recharge_available: true,
        spell_slots: BattleSpellSlotLedger::none(),
        spell_active_effects: BattleSpellActiveEffects::none(),
    }
}

fn rogue_combatant() -> Combatant {
    Combatant {
        hp: 11,
        max_hp: 11,
        temporary_hp: 0,
        armor_class: 14,
        speed_feet: 30,
        shield_armor_class_bonus_active: false,
        unconscious: false,
        incapacitated: false,
        prone: false,
        creature_size: CreatureSize::Medium,
        lifecycle: CombatantLifecycle::UsesDeathSavingThrows(DeathSavingThrowLifecycle::fresh()),
        reaction_available: true,
        movement_spent_feet: 0,
        weapon_attack_supported: true,
        weapon_damage_modifier: 3,
        multiattack_profile: None,
        sneak_attack_supported: true,
        sneak_attack_used_this_turn: false,
        recharge_available: false,
        spell_slots: BattleSpellSlotLedger::none(),
        spell_active_effects: BattleSpellActiveEffects::none(),
    }
}

fn skeleton_combatant() -> Combatant {
    Combatant {
        hp: 13,
        max_hp: 13,
        temporary_hp: 0,
        armor_class: 13,
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
        weapon_damage_modifier: 3,
        multiattack_profile: Some(primary_stat_block_multiattack_profile(2)),
        sneak_attack_supported: false,
        sneak_attack_used_this_turn: false,
        recharge_available: false,
        spell_slots: BattleSpellSlotLedger::none(),
        spell_active_effects: BattleSpellActiveEffects::none(),
    }
}

#[must_use]
pub fn current_actor(state: &BattleState) -> Actor {
    // QNT: battle-runtime-turn-order.qnt `currentActor`.
    state.initiative.still_to_act.actor
}

#[must_use]
pub fn first_waiting_actor(state: &BattleState) -> Option<Actor> {
    state.initiative.still_to_act.waiting.first().copied()
}

fn ordinary_zero_hit_point_restoration_target(combatant: Combatant) -> bool {
    // RAW: cleanroom-input/raw/srd-5.2.1/Playing-the-Game.md
    // "Healing" and "Dropping to 0 Hit Points". Ordinary Hit Point
    // restoration can wake a creature that is making Death Saving Throws or is
    // Stable at 0 HP. It does not model the GM exception that lets a monster be
    // treated like a character after dropping to 0 HP.
    combatant.hp == 0
        && matches!(
            combatant.lifecycle,
            CombatantLifecycle::UsesDeathSavingThrows(
                DeathSavingThrowLifecycle::MakingDeathSavingThrows { .. }
                    | DeathSavingThrowLifecycle::Stable
            )
        )
}

fn first_zero_hit_point_restoration_target_except(
    state: &BattleState,
    actor: Actor,
) -> Option<Actor> {
    BATTLE_ACTORS.into_iter().find(|candidate| {
        *candidate != actor
            && ordinary_zero_hit_point_restoration_target(combatant_for(state, *candidate))
    })
}

#[must_use]
pub fn stat_block_multiattack_dispatches_available(state: &BattleState) -> i16 {
    state.stat_block_control.pending_primary_dispatches
        + state.stat_block_control.pending_secondary_dispatches
}

#[must_use]
pub fn advance_turn(state: BattleState) -> BattleTurnAdvanceResult {
    // RAW: cleanroom-input/raw/srd-5.2.1/Playing-the-Game.md
    // "The Order of Combat"; QNT: battle-runtime-turn-advancement.qnt
    // `endTurn`, narrowed to the current reducer-spine combatants and the
    // T062 lifecycle fixture effects.
    let previous_round = state.initiative.round;
    let previous_actor = current_actor(&state);
    let state = apply_turn_end_boundary_effects(state, previous_actor);
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

    let state = if state.initiative.round > previous_round {
        tick_round_duration_boundary_effects(state)
    } else {
        state
    };

    BattleTurnAdvanceResult {
        round: state.initiative.round,
        state,
        previous_actor,
        next_actor,
    }
}

#[must_use]
pub fn end_turn(state: BattleState) -> BattleTurnAdvanceResult {
    advance_turn(state)
}

#[must_use]
pub fn end_turn_subject(state: &BattleState) -> BattleSubject {
    // QNT: cleanroom-input/qnt/battle-runtime/
    // battle-runtime-reducer-spine-contract.mbt.qnt End Turn subject branch.
    // End Turn is a runtime command subject with no table-supplied fill.
    diagnostic_subject(BattleSubjectKind::EndTurn, current_actor(state), None)
}

#[must_use]
pub fn advance_turn_state(state: BattleState) -> BattleState {
    advance_turn(state).state
}

#[must_use]
pub fn advance_turn_observed(
    state: BattleState,
    observer: &mut impl BattleEntrypointObserver,
) -> BattleTurnAdvanceResult {
    let result = advance_turn(state);
    observer.observe_battle_entrypoint(BattleEntrypointEvent::AdvanceTurn {
        previous_actor: result.previous_actor,
        next_actor: result.next_actor,
        round: result.round,
    });
    result
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
pub fn discover_battle_acts(state: &BattleState) -> BattleActDiscoveryResult {
    // QNT: battle-runtime-combat-holes.qnt `combatOpenHoles`, plus the copied
    // reducer-spine diagnostic guidance for slot-spell, save-gated spell, and
    // Hit Point restoration subject families.
    let actor = current_actor(state);
    let mut acts = Vec::new();
    push_reaction_spell_acts(state, actor, &mut acts);
    if !state.action_available {
        push_metamagic_option_spell_acts(state, actor, &mut acts);
        return BattleActDiscoveryResult::from_state(state, acts);
    }
    push_reducer_spine_diagnostic_acts(state, actor, &mut acts);
    push_metamagic_option_spell_acts(state, actor, &mut acts);
    push_spatial_route_acts(state, actor, &mut acts);
    push_capability_weapon_acts(state, actor, &mut acts);
    BattleActDiscoveryResult::from_state(state, acts)
}

fn push_reaction_spell_acts(state: &BattleState, actor: Actor, acts: &mut Vec<AvailableBattleAct>) {
    if !reaction_spell_opportunity_open(state)
        || !combatant_for(state, actor).reaction_available
        || !can_actor_expend_spell_slot_this_turn(state, actor)
    {
        return;
    }

    acts.push(AvailableBattleAct {
        subject: diagnostic_subject(BattleSubjectKind::ReactionSpell, actor, None),
        holes: Vec::new(),
    });
}

fn reaction_spell_opportunity_open(state: &BattleState) -> bool {
    attack_hit_reaction_opportunity_open(state) || after_damage_reaction_opportunity_open(state)
}

fn attack_hit_reaction_opportunity_open(state: &BattleState) -> bool {
    matches!(
        state.interrupt_resume.reaction_window,
        ReactionWindowOffer::Offered {
            active: ReactionWindowRole::AttackHitInterruption,
            ..
        }
    )
}

fn after_damage_reaction_opportunity_open(state: &BattleState) -> bool {
    state.reaction_casting_time.reaction_window_open
        && state.reaction_casting_time.trigger == BattleReactionCastingTrigger::AfterDamage
        && state.reaction_casting_time.continuation
            == BattleReactionCastingContinuation::AfterDamageResolved
}

fn reaction_spell_fill_matches_opportunity(
    state: &BattleState,
    fill: BattleReactionSpellFill,
) -> bool {
    match fill {
        BattleReactionSpellFill::ArmorClassInterruption(_) => {
            attack_hit_reaction_opportunity_open(state)
        }
        BattleReactionSpellFill::FailedSaveDamage(_) => {
            after_damage_reaction_opportunity_open(state)
        }
    }
}

const fn reaction_spell_fill_reactor(fill: BattleReactionSpellFill) -> Actor {
    match fill {
        BattleReactionSpellFill::ArmorClassInterruption(facts) => facts.reactor,
        BattleReactionSpellFill::FailedSaveDamage(facts) => facts.reactor,
    }
}

#[must_use]
pub fn discover_battle_acts_observed(
    state: &BattleState,
    observer: &mut impl BattleEntrypointObserver,
) -> BattleActDiscoveryResult {
    let discovery = discover_battle_acts(state);
    for act in discovery.available_acts() {
        observer.observe_battle_reducer_route(BattleReducerRouteEvent::DiscoverBattleActs {
            subject: battle_reducer_route_subject_family(act.subject.kind),
            holes: sorted_battle_reducer_route_holes(
                act.holes
                    .iter()
                    .copied()
                    .map(battle_reducer_route_hole)
                    .collect(),
            ),
            owner: battle_discovery_route_owner(act.subject.kind),
        });
    }
    observer.observe_battle_entrypoint(BattleEntrypointEvent::DiscoverBattleActs {
        available_subjects: discovery
            .available_acts()
            .iter()
            .map(|act| act.subject.kind)
            .collect(),
    });
    discovery
}

const fn battle_discovery_route_owner(kind: BattleSubjectKind) -> BattleReducerRouteOwnerGroup {
    match kind {
        BattleSubjectKind::WeaponAttack => BattleReducerRouteOwnerGroup::TargetSelection,
        BattleSubjectKind::Multiattack => BattleReducerRouteOwnerGroup::AttackActionProcedure,
        BattleSubjectKind::SingleTargetSpellAttack | BattleSubjectKind::TypedSpellAttack => {
            BattleReducerRouteOwnerGroup::SpellSlotAndActionEconomy
        }
        BattleSubjectKind::SlotSpell => BattleReducerRouteOwnerGroup::SpellSlotAndActionEconomy,
        BattleSubjectKind::SaveGatedAreaDamage
        | BattleSubjectKind::SaveGatedTargetListConditionChoice => {
            BattleReducerRouteOwnerGroup::SpellSlotAndActionEconomy
        }
        BattleSubjectKind::HitPointRestorationSingleTargetSpell
        | BattleSubjectKind::HitPointRestorationTargetListSpell
        | BattleSubjectKind::HitPointRestorationFeatureHealingPool => {
            BattleReducerRouteOwnerGroup::HitPoint
        }
        BattleSubjectKind::DeathSavingThrow => {
            BattleReducerRouteOwnerGroup::HitPointAndZeroHpLifecycle
        }
        BattleSubjectKind::ConcentrationTeardown => BattleReducerRouteOwnerGroup::Concentration,
        BattleSubjectKind::StatBlockAction => BattleReducerRouteOwnerGroup::StatBlockAction,
        BattleSubjectKind::CommandSpell => BattleReducerRouteOwnerGroup::ActiveEffect,
        BattleSubjectKind::ArmorClassSpellEffect => {
            BattleReducerRouteOwnerGroup::SpellSlotAndActionEconomy
        }
        BattleSubjectKind::ReactionSpell => BattleReducerRouteOwnerGroup::Reaction,
        BattleSubjectKind::ScalarBuffTargetSpell => BattleReducerRouteOwnerGroup::ActiveEffect,
        BattleSubjectKind::WeaponMasteryProperty
        | BattleSubjectKind::AttackActionAreaSaveDamageReplacement
        | BattleSubjectKind::UnitFeatureBonusAction => {
            BattleReducerRouteOwnerGroup::FeatureResource
        }
        BattleSubjectKind::ActiveFeatureSpellSaveDc
        | BattleSubjectKind::ActiveFeatureSpellAttackRollMode => {
            BattleReducerRouteOwnerGroup::ActiveEffect
        }
        BattleSubjectKind::MetamagicOptionSpell => BattleReducerRouteOwnerGroup::FeatureResource,
        BattleSubjectKind::Spatial(subject) => spatial_route_owner(subject),
        BattleSubjectKind::EndTurn => BattleReducerRouteOwnerGroup::ActionEconomy,
    }
}

fn push_reducer_spine_diagnostic_acts(
    state: &BattleState,
    actor: Actor,
    acts: &mut Vec<AvailableBattleAct>,
) {
    if death_saving_throw_available(combatant_for(state, actor)) {
        acts.push(AvailableBattleAct {
            subject: diagnostic_subject(BattleSubjectKind::DeathSavingThrow, actor, Some(actor)),
            holes: vec![BattleHoleKind::DeathSavingThrow],
        });
    }

    if let BattleCommandEffectProcedure::Active(active) = state.command_effect_procedure {
        if active.actor == actor && active.stage != CommandFrontierStage::Resolved {
            acts.push(AvailableBattleAct {
                subject: diagnostic_subject(BattleSubjectKind::CommandSpell, actor, active.target),
                holes: command_hole_kinds(active.stage),
            });
        }
    }

    if can_actor_expend_spell_slot_this_turn(state, actor) && first_waiting_actor(state).is_some() {
        acts.push(AvailableBattleAct {
            subject: diagnostic_subject(BattleSubjectKind::SingleTargetSpellAttack, actor, None),
            holes: spell_attack_hole_kinds(SpellAttackFrontierStage::TargetChoice),
        });
        acts.push(AvailableBattleAct {
            subject: diagnostic_subject(BattleSubjectKind::TypedSpellAttack, actor, None),
            holes: spell_attack_hole_kinds(SpellAttackFrontierStage::DamageTypeAndTargetChoice),
        });
        acts.push(AvailableBattleAct {
            subject: diagnostic_subject(BattleSubjectKind::SlotSpell, actor, None),
            holes: vec![BattleHoleKind::SpellTargetAllocation],
        });
        acts.push(AvailableBattleAct {
            subject: diagnostic_subject(BattleSubjectKind::CommandSpell, actor, None),
            holes: command_hole_kinds(CommandFrontierStage::TargetListAndOptionChoice),
        });
        acts.push(AvailableBattleAct {
            subject: diagnostic_subject(BattleSubjectKind::ScalarBuffTargetSpell, actor, None),
            holes: vec![BattleHoleKind::TargetChoice],
        });
        acts.push(AvailableBattleAct {
            subject: diagnostic_subject(BattleSubjectKind::ArmorClassSpellEffect, actor, None),
            holes: Vec::new(),
        });
        acts.push(AvailableBattleAct {
            subject: diagnostic_subject(BattleSubjectKind::ConcentrationTeardown, actor, None),
            holes: Vec::new(),
        });
        acts.push(AvailableBattleAct {
            subject: diagnostic_subject(BattleSubjectKind::SaveGatedAreaDamage, actor, None),
            holes: save_gated_spell_hole_kinds(
                SaveGatedSpellFrontierStage::DamageSavingThrowOutcome,
            ),
        });
        acts.push(AvailableBattleAct {
            subject: diagnostic_subject(
                BattleSubjectKind::SaveGatedTargetListConditionChoice,
                actor,
                None,
            ),
            holes: save_gated_spell_hole_kinds(
                SaveGatedSpellFrontierStage::TargetListAndConditionChoice,
            ),
        });
    }

    if first_waiting_actor(state).is_some() {
        acts.push(AvailableBattleAct {
            subject: diagnostic_subject(BattleSubjectKind::WeaponMasteryProperty, actor, None),
            holes: Vec::new(),
        });
        acts.push(AvailableBattleAct {
            subject: diagnostic_subject(
                BattleSubjectKind::AttackActionAreaSaveDamageReplacement,
                actor,
                None,
            ),
            holes: vec![BattleHoleKind::SavingThrowOutcome],
        });
        acts.push(AvailableBattleAct {
            subject: diagnostic_subject(BattleSubjectKind::ActiveFeatureSpellSaveDc, actor, None),
            holes: Vec::new(),
        });
        acts.push(AvailableBattleAct {
            subject: diagnostic_subject(
                BattleSubjectKind::ActiveFeatureSpellAttackRollMode,
                actor,
                None,
            ),
            holes: vec![BattleHoleKind::TargetChoice],
        });
    }

    if state.bonus_action_available && first_waiting_actor(state).is_some() {
        acts.push(AvailableBattleAct {
            subject: diagnostic_subject(BattleSubjectKind::UnitFeatureBonusAction, actor, None),
            holes: Vec::new(),
        });
    }

    if let Some(target) = first_zero_hit_point_restoration_target_except(state, actor) {
        acts.push(AvailableBattleAct {
            subject: diagnostic_subject(
                BattleSubjectKind::HitPointRestorationSingleTargetSpell,
                actor,
                Some(target),
            ),
            holes: hit_point_restoration_hole_kinds(
                HitPointRestorationFrontierStage::SpellHealingTargetChoice,
            ),
        });
        acts.push(AvailableBattleAct {
            subject: diagnostic_subject(
                BattleSubjectKind::HitPointRestorationTargetListSpell,
                actor,
                Some(target),
            ),
            holes: hit_point_restoration_hole_kinds(
                HitPointRestorationFrontierStage::SpellHealingTargetList,
            ),
        });
        acts.push(AvailableBattleAct {
            subject: diagnostic_subject(
                BattleSubjectKind::HitPointRestorationFeatureHealingPool,
                actor,
                Some(target),
            ),
            holes: hit_point_restoration_hole_kinds(
                HitPointRestorationFrontierStage::FeatureHealingPoolDistribution,
            ),
        });
    }
}

fn push_metamagic_option_spell_acts(
    state: &BattleState,
    actor: Actor,
    acts: &mut Vec<AvailableBattleAct>,
) {
    if !metamagic_option_spell_available(state) || first_waiting_actor(state).is_none() {
        return;
    }

    acts.push(AvailableBattleAct {
        subject: diagnostic_subject(BattleSubjectKind::MetamagicOptionSpell, actor, None),
        holes: Vec::new(),
    });
}

fn push_spatial_route_acts(state: &BattleState, actor: Actor, acts: &mut Vec<AvailableBattleAct>) {
    for subject in state.spatial_route_subjects.iter().copied() {
        acts.push(AvailableBattleAct {
            subject: diagnostic_subject(BattleSubjectKind::Spatial(subject), actor, None),
            holes: spatial_route_holes(subject),
        });
    }
}

fn diagnostic_subject(
    kind: BattleSubjectKind,
    actor: Actor,
    target: Option<Actor>,
) -> BattleSubject {
    BattleSubject {
        kind,
        actor,
        target,
        stage: WeaponAttackFrontierStage::Resolved,
        damage_modifier: 0,
    }
}

fn diagnostic_subject_shape_matches(
    subject: BattleSubject,
    kind: BattleSubjectKind,
    target: Option<Actor>,
) -> bool {
    subject.kind == kind
        && subject.target == target
        && subject.stage == WeaponAttackFrontierStage::Resolved
        && subject.damage_modifier == 0
}

fn route_subject_discoverable_now(state: &BattleState, subject: BattleSubject) -> bool {
    discover_battle_acts(state)
        .available_acts()
        .iter()
        .any(|act| act.subject == subject)
}

fn slot_spell_route_subject_is_live(state: &BattleState, subject: BattleSubject) -> bool {
    if !state.action_available || subject.kind != BattleSubjectKind::SlotSpell {
        return false;
    }

    match state.slot_spell_procedure {
        BattleSlotSpellProcedure::Inactive => {
            diagnostic_subject_shape_matches(subject, BattleSubjectKind::SlotSpell, None)
                && route_subject_discoverable_now(state, subject)
        }
        BattleSlotSpellProcedure::Active(active) => {
            active.actor == subject.actor
                && active.stage != BattleSlotSpellStage::Resolved
                && diagnostic_subject_shape_matches(
                    subject,
                    BattleSubjectKind::SlotSpell,
                    active.target,
                )
        }
    }
}

fn save_gated_spell_route_subject_is_live(state: &BattleState, subject: BattleSubject) -> bool {
    if !matches!(
        subject.kind,
        BattleSubjectKind::SaveGatedAreaDamage
            | BattleSubjectKind::SaveGatedTargetListConditionChoice
    ) || !diagnostic_subject_shape_matches(subject, subject.kind, None)
    {
        return false;
    }

    match state.save_gated_spell_procedure {
        BattleSaveGatedSpellProcedure::Inactive => {
            state.action_available && route_subject_discoverable_now(state, subject)
        }
        BattleSaveGatedSpellProcedure::Active(active) => {
            active.actor == subject.actor
                && active.stage != SaveGatedSpellFrontierStage::Resolved
                && save_gated_spell_subject_kind_matches(subject.kind, active)
        }
    }
}

fn save_gated_spell_subject_kind_matches(
    kind: BattleSubjectKind,
    subject: BattleSaveGatedSpellSubject,
) -> bool {
    match kind {
        BattleSubjectKind::SaveGatedAreaDamage => subject.damage_dice_required,
        BattleSubjectKind::SaveGatedTargetListConditionChoice => !subject.damage_dice_required,
        BattleSubjectKind::MetamagicOptionSpell
        | BattleSubjectKind::WeaponAttack
        | BattleSubjectKind::Multiattack
        | BattleSubjectKind::SingleTargetSpellAttack
        | BattleSubjectKind::TypedSpellAttack
        | BattleSubjectKind::SlotSpell
        | BattleSubjectKind::HitPointRestorationSingleTargetSpell
        | BattleSubjectKind::HitPointRestorationTargetListSpell
        | BattleSubjectKind::HitPointRestorationFeatureHealingPool
        | BattleSubjectKind::DeathSavingThrow
        | BattleSubjectKind::ConcentrationTeardown
        | BattleSubjectKind::StatBlockAction
        | BattleSubjectKind::CommandSpell
        | BattleSubjectKind::ArmorClassSpellEffect
        | BattleSubjectKind::ReactionSpell
        | BattleSubjectKind::ScalarBuffTargetSpell
        | BattleSubjectKind::WeaponMasteryProperty
        | BattleSubjectKind::AttackActionAreaSaveDamageReplacement
        | BattleSubjectKind::UnitFeatureBonusAction
        | BattleSubjectKind::ActiveFeatureSpellSaveDc
        | BattleSubjectKind::ActiveFeatureSpellAttackRollMode
        | BattleSubjectKind::Spatial(_)
        | BattleSubjectKind::EndTurn => false,
    }
}

fn concentration_teardown_route_subject_is_live(
    state: &BattleState,
    subject: BattleSubject,
) -> bool {
    state.action_available
        && diagnostic_subject_shape_matches(subject, BattleSubjectKind::ConcentrationTeardown, None)
        && route_subject_discoverable_now(state, subject)
}

fn command_effect_route_subject_is_live(state: &BattleState, subject: BattleSubject) -> bool {
    if subject.kind != BattleSubjectKind::CommandSpell {
        return false;
    }

    match state.command_effect_procedure {
        BattleCommandEffectProcedure::Inactive => {
            state.action_available
                && diagnostic_subject_shape_matches(subject, BattleSubjectKind::CommandSpell, None)
                && route_subject_discoverable_now(state, subject)
        }
        BattleCommandEffectProcedure::Active(active) => {
            active.actor == subject.actor
                && (active.stage != CommandFrontierStage::Resolved
                    || active.pending_option.is_some()
                    || active.scenario == CommandNextTurnScenario::FleeOpportunityAttackWindow)
                && diagnostic_subject_shape_matches(
                    subject,
                    BattleSubjectKind::CommandSpell,
                    active.target,
                )
        }
    }
}

fn scalar_buff_route_subject_is_live(state: &BattleState, subject: BattleSubject) -> bool {
    state.action_available
        && diagnostic_subject_shape_matches(subject, BattleSubjectKind::ScalarBuffTargetSpell, None)
        && route_subject_discoverable_now(state, subject)
}

fn armor_class_spell_effect_route_subject_is_live(
    state: &BattleState,
    subject: BattleSubject,
) -> bool {
    state.action_available
        && diagnostic_subject_shape_matches(subject, BattleSubjectKind::ArmorClassSpellEffect, None)
        && route_subject_discoverable_now(state, subject)
}

fn reaction_spell_route_subject_is_live(state: &BattleState, subject: BattleSubject) -> bool {
    subject.kind == BattleSubjectKind::ReactionSpell
        && diagnostic_subject_shape_matches(subject, BattleSubjectKind::ReactionSpell, None)
        && route_subject_discoverable_now(state, subject)
        && combatant_for(state, subject.actor).reaction_available
        && can_actor_expend_spell_slot_this_turn(state, subject.actor)
}

fn feature_substrate_route_subject_is_live(state: &BattleState, subject: BattleSubject) -> bool {
    if !diagnostic_subject_shape_matches(subject, subject.kind, None) {
        return false;
    }

    match subject.kind {
        BattleSubjectKind::WeaponMasteryProperty
        | BattleSubjectKind::AttackActionAreaSaveDamageReplacement
        | BattleSubjectKind::ActiveFeatureSpellSaveDc
        | BattleSubjectKind::ActiveFeatureSpellAttackRollMode => {
            state.action_available && route_subject_discoverable_now(state, subject)
        }
        BattleSubjectKind::UnitFeatureBonusAction => {
            state.bonus_action_available && route_subject_discoverable_now(state, subject)
        }
        BattleSubjectKind::MetamagicOptionSpell => {
            metamagic_option_spell_route_subject_is_live(state, subject)
        }
        BattleSubjectKind::EndTurn
        | BattleSubjectKind::WeaponAttack
        | BattleSubjectKind::Multiattack
        | BattleSubjectKind::SingleTargetSpellAttack
        | BattleSubjectKind::TypedSpellAttack
        | BattleSubjectKind::SlotSpell
        | BattleSubjectKind::SaveGatedAreaDamage
        | BattleSubjectKind::SaveGatedTargetListConditionChoice
        | BattleSubjectKind::HitPointRestorationSingleTargetSpell
        | BattleSubjectKind::HitPointRestorationTargetListSpell
        | BattleSubjectKind::HitPointRestorationFeatureHealingPool
        | BattleSubjectKind::DeathSavingThrow
        | BattleSubjectKind::ConcentrationTeardown
        | BattleSubjectKind::StatBlockAction
        | BattleSubjectKind::CommandSpell
        | BattleSubjectKind::Spatial(_)
        | BattleSubjectKind::ArmorClassSpellEffect
        | BattleSubjectKind::ReactionSpell
        | BattleSubjectKind::ScalarBuffTargetSpell => false,
    }
}

fn metamagic_option_spell_route_subject_is_live(
    state: &BattleState,
    subject: BattleSubject,
) -> bool {
    metamagic_option_spell_available(state)
        && diagnostic_subject_shape_matches(subject, BattleSubjectKind::MetamagicOptionSpell, None)
        && route_subject_discoverable_now(state, subject)
}

fn metamagic_option_spell_available(state: &BattleState) -> bool {
    (state.feature_substrates.quickened_spell.offered && state.bonus_action_available)
        || (state.feature_substrates.metamagic_spell.offered && state.action_available)
        || (state.feature_substrates.metamagic_option_spell.offered && state.action_available)
}

fn spatial_route_subject_is_live(state: &BattleState, subject: BattleSubject) -> bool {
    let BattleSubjectKind::Spatial(spatial_subject) = subject.kind else {
        return false;
    };
    diagnostic_subject_shape_matches(subject, subject.kind, None)
        && state.spatial_route_subjects.contains(&spatial_subject)
        && route_subject_discoverable_now(state, subject)
}

fn spell_attack_route_subject_is_live(state: &BattleState, subject: BattleSubject) -> bool {
    if !matches!(
        subject.kind,
        BattleSubjectKind::SingleTargetSpellAttack | BattleSubjectKind::TypedSpellAttack
    ) {
        return false;
    }

    match state.spell_attack_procedure {
        BattleSpellAttackProcedure::Inactive => {
            state.action_available
                && diagnostic_subject_shape_matches(subject, subject.kind, None)
                && route_subject_discoverable_now(state, subject)
        }
        BattleSpellAttackProcedure::Active(active) => {
            active.actor == subject.actor
                && active.stage != SpellAttackFrontierStage::Resolved
                && spell_attack_subject_kind_matches(subject.kind, active)
        }
    }
}

fn spell_attack_subject_kind_matches(
    kind: BattleSubjectKind,
    subject: BattleSpellAttackSubject,
) -> bool {
    matches!(
        (kind, subject.requires_spell_slot),
        (BattleSubjectKind::SingleTargetSpellAttack, false)
            | (BattleSubjectKind::TypedSpellAttack, true)
    )
}

fn hit_point_restoration_route_subject_is_live(
    state: &BattleState,
    subject: BattleSubject,
) -> bool {
    let Some(target) = subject.target else {
        return false;
    };
    if !state.action_available
        || !matches!(
            subject.kind,
            BattleSubjectKind::HitPointRestorationSingleTargetSpell
                | BattleSubjectKind::HitPointRestorationTargetListSpell
                | BattleSubjectKind::HitPointRestorationFeatureHealingPool
        )
        || !diagnostic_subject_shape_matches(subject, subject.kind, Some(target))
        || !ordinary_zero_hit_point_restoration_target(combatant_for(state, target))
    {
        return false;
    }

    match state.hit_point_restoration_procedure {
        BattleHitPointRestorationProcedure::Inactive => {
            route_subject_discoverable_now(state, subject)
        }
        BattleHitPointRestorationProcedure::Active(active) => {
            active.actor == subject.actor
                && active.stage != HitPointRestorationFrontierStage::Resolved
                && hit_point_restoration_subject_kind_matches(subject.kind, active.targeting)
        }
    }
}

fn hit_point_restoration_subject_kind_matches(
    kind: BattleSubjectKind,
    targeting: BattleHitPointRestorationTargeting,
) -> bool {
    matches!(
        (kind, targeting),
        (
            BattleSubjectKind::HitPointRestorationSingleTargetSpell,
            BattleHitPointRestorationTargeting::SingleTargetSpell { .. }
        ) | (
            BattleSubjectKind::HitPointRestorationTargetListSpell,
            BattleHitPointRestorationTargeting::TargetListSpell { .. }
        ) | (
            BattleSubjectKind::HitPointRestorationFeatureHealingPool,
            BattleHitPointRestorationTargeting::FeatureHealingPool { .. }
        )
    )
}

fn push_capability_weapon_acts(
    state: &BattleState,
    actor: Actor,
    acts: &mut Vec<AvailableBattleAct>,
) {
    let combatant = combatant_for(state, actor);
    if !current_actor_can_attack(state) {
        return;
    }

    if combatant.multiattack_profile.is_some() {
        acts.push(AvailableBattleAct {
            subject: BattleSubject {
                kind: BattleSubjectKind::Multiattack,
                actor,
                target: None,
                stage: WeaponAttackFrontierStage::Resolved,
                damage_modifier: 0,
            },
            holes: Vec::new(),
        });
    }

    acts.push(AvailableBattleAct {
        subject: BattleSubject {
            kind: BattleSubjectKind::WeaponAttack,
            actor,
            target: None,
            stage: WeaponAttackFrontierStage::TargetChoice,
            damage_modifier: combatant.weapon_damage_modifier,
        },
        holes: weapon_hole_kinds(WeaponAttackFrontierStage::TargetChoice),
    });
}

#[must_use]
pub fn resolve_battle_subject(
    state: BattleState,
    request: BattleResolutionRequest,
) -> BattleResolutionResult {
    resolve_battle_subject_unchecked(state, request.subject, request.fill)
}

#[cfg(test)]
#[must_use]
pub fn resolve_battle_subject_test_fill(
    state: BattleState,
    subject: BattleSubject,
    fill: BattleFill,
) -> BattleResolutionResult {
    resolve_battle_subject_unchecked(state, subject, fill)
}

#[must_use]
pub fn resolve_battle_subject_observed(
    state: BattleState,
    request: BattleResolutionRequest,
    observer: &mut impl BattleEntrypointObserver,
) -> BattleResolutionResult {
    let subject = request.subject().kind;
    let fill = request.fill;
    let result = resolve_battle_subject(state, request);
    observer.observe_battle_reducer_route(battle_resolution_route_event(subject, fill, &result));
    observer.observe_battle_entrypoint(BattleEntrypointEvent::ResolveBattleSubject {
        subject,
        outcome: result.outcome(),
    });
    result
}

fn battle_resolution_route_event(
    subject: BattleSubjectKind,
    fill: BattleFill,
    result: &BattleResolutionResult,
) -> BattleReducerRouteEvent {
    let holes = result
        .requested_holes()
        .map_or_else(Vec::new, <[_]>::to_vec);
    let route_holes = sorted_battle_reducer_route_holes(
        holes
            .iter()
            .copied()
            .map(battle_reducer_route_hole)
            .collect(),
    );
    let outcome = result.outcome();
    let owner = battle_resolution_route_owner(subject, fill, outcome, &holes);
    let subject = battle_reducer_route_subject_family(subject);
    match battle_reducer_route_fill_kind(fill) {
        Some(fill) => BattleReducerRouteEvent::ResolveBattleSubject {
            subject,
            fill,
            outcome,
            holes: route_holes,
            owner,
        },
        None => BattleReducerRouteEvent::ResolveBattleSubjectWithoutFill {
            subject,
            outcome,
            holes: route_holes,
            owner,
        },
    }
}

fn battle_reducer_route_fill_kind(fill: BattleFill) -> Option<BattleReducerRouteFillKind> {
    match fill {
        BattleFill::NoFill => None,
        BattleFill::TargetChoice(_) => Some(BattleReducerRouteFillKind::TargetChoice),
        BattleFill::AttackRoll(_) => Some(BattleReducerRouteFillKind::AttackRoll),
        BattleFill::DamageRoll(_) | BattleFill::SneakAttackDamageRoll(_) => {
            Some(BattleReducerRouteFillKind::RolledDice)
        }
        BattleFill::ResolveMultiattack | BattleFill::SpendMultiattackDispatch => {
            Some(BattleReducerRouteFillKind::UnitFeatureDecision)
        }
        BattleFill::SpellAttack(fill) => Some(match fill {
            BattleSpellAttackFill::TargetChoice(_) => BattleReducerRouteFillKind::TargetChoice,
            BattleSpellAttackFill::DamageTypeChoice => BattleReducerRouteFillKind::DamageTypeChoice,
            BattleSpellAttackFill::AttackRoll(_) => BattleReducerRouteFillKind::AttackRoll,
            BattleSpellAttackFill::DamageRoll(_) => BattleReducerRouteFillKind::RolledDice,
        }),
        BattleFill::SlotSpell(fill) => Some(match fill {
            BattleSlotSpellFill::TargetAllocation(_) => {
                BattleReducerRouteFillKind::SpellTargetAllocation
            }
            BattleSlotSpellFill::DamageRoll(_) => BattleReducerRouteFillKind::RolledDice,
        }),
        BattleFill::SaveGatedSpell(fill) => Some(match fill {
            BattleSaveGatedSpellFill::SpellTargetList => {
                BattleReducerRouteFillKind::SpellTargetList
            }
            BattleSaveGatedSpellFill::ConditionChoice => {
                BattleReducerRouteFillKind::ConditionChoice
            }
            BattleSaveGatedSpellFill::SavingThrowOutcome => {
                BattleReducerRouteFillKind::SavingThrowOutcome
            }
            BattleSaveGatedSpellFill::DamageRoll => BattleReducerRouteFillKind::RolledDice,
        }),
        BattleFill::HitPointRestoration(fill) => Some(match fill {
            BattleHitPointRestorationFill::TargetChoice(_) => {
                BattleReducerRouteFillKind::TargetChoice
            }
            BattleHitPointRestorationFill::SpellTargetList(_) => {
                BattleReducerRouteFillKind::SpellTargetList
            }
            BattleHitPointRestorationFill::HealingRoll(_) => BattleReducerRouteFillKind::RolledDice,
            BattleHitPointRestorationFill::HitPointHealingDistribution { .. } => {
                BattleReducerRouteFillKind::HitPointHealingDistribution
            }
        }),
        BattleFill::DeathSavingThrow(_) => Some(BattleReducerRouteFillKind::DeathSavingThrow),
        BattleFill::Concentration(fill) => Some(match fill {
            BattleConcentrationFill::SavingThrow(_) => {
                BattleReducerRouteFillKind::ConcentrationSavingThrow
            }
            BattleConcentrationFill::CastSpell
            | BattleConcentrationFill::DamageTaken(_)
            | BattleConcentrationFill::VoluntaryEnd
            | BattleConcentrationFill::CastReplacementSpell => {
                BattleReducerRouteFillKind::UnitFeatureDecision
            }
        }),
        BattleFill::CommandEffect(fill) => Some(match fill {
            BattleCommandEffectFill::SpellTargetList(_) => {
                BattleReducerRouteFillKind::SpellTargetList
            }
            BattleCommandEffectFill::CommandOptionChoice(_) => {
                BattleReducerRouteFillKind::CommandOptionChoice
            }
            BattleCommandEffectFill::SavingThrowOutcome { .. }
            | BattleCommandEffectFill::DropHeldObjectFacts { .. }
            | BattleCommandEffectFill::FollowPendingOption(_)
            | BattleCommandEffectFill::CleanupPendingOption(_)
            | BattleCommandEffectFill::Complete => BattleReducerRouteFillKind::UnitFeatureDecision,
            BattleCommandEffectFill::Movement { .. } => BattleReducerRouteFillKind::Movement,
        }),
        BattleFill::ArmorClassSpellEffect(_) | BattleFill::ReactionSpell(_) => {
            Some(BattleReducerRouteFillKind::UnitFeatureDecision)
        }
        BattleFill::ScalarBuff(_) => Some(BattleReducerRouteFillKind::TargetChoice),
        BattleFill::WeaponMasteryProperty(_)
        | BattleFill::UnitFeatureBonusAction(_)
        | BattleFill::ActiveFeatureSpellSaveDc(_)
        | BattleFill::ActiveFeatureSpellAttackRollMode(_)
        | BattleFill::MetamagicOptionSpell(_) => {
            Some(BattleReducerRouteFillKind::UnitFeatureDecision)
        }
        BattleFill::Spatial(fill) => Some(spatial_route_fill_kind(fill)),
        BattleFill::AttackActionAreaSaveDamageReplacement(_) => {
            Some(BattleReducerRouteFillKind::SavingThrowOutcome)
        }
        BattleFill::StatBlockAction { fill, .. } => Some(match fill {
            StatBlockActionFill::TargetChoice(_) => BattleReducerRouteFillKind::TargetChoice,
            StatBlockActionFill::AttackRoll(_) => BattleReducerRouteFillKind::AttackRoll,
            StatBlockActionFill::DamageDice(_) => BattleReducerRouteFillKind::RolledDice,
            StatBlockActionFill::RechargeRoll(_) => {
                BattleReducerRouteFillKind::StatBlockRechargeRoll
            }
        }),
    }
}

fn battle_resolution_route_owner(
    subject: BattleSubjectKind,
    fill: BattleFill,
    outcome: BattleResolutionOutcome,
    holes: &[BattleHoleKind],
) -> BattleReducerRouteOwnerGroup {
    match subject {
        BattleSubjectKind::WeaponAttack => match battle_reducer_route_fill_kind(fill) {
            Some(BattleReducerRouteFillKind::TargetChoice) => {
                BattleReducerRouteOwnerGroup::TargetSelection
            }
            Some(BattleReducerRouteFillKind::AttackRoll) => {
                BattleReducerRouteOwnerGroup::AttackRoll
            }
            Some(BattleReducerRouteFillKind::RolledDice) => BattleReducerRouteOwnerGroup::HitPoint,
            _ => BattleReducerRouteOwnerGroup::AttackActionProcedure,
        },
        BattleSubjectKind::SingleTargetSpellAttack | BattleSubjectKind::TypedSpellAttack => {
            match battle_reducer_route_fill_kind(fill) {
                Some(BattleReducerRouteFillKind::TargetChoice) => {
                    BattleReducerRouteOwnerGroup::TargetSelection
                }
                Some(BattleReducerRouteFillKind::DamageTypeChoice) => {
                    BattleReducerRouteOwnerGroup::DamageType
                }
                Some(BattleReducerRouteFillKind::AttackRoll) => {
                    BattleReducerRouteOwnerGroup::AttackRoll
                }
                Some(BattleReducerRouteFillKind::RolledDice) => {
                    BattleReducerRouteOwnerGroup::HitPoint
                }
                _ => BattleReducerRouteOwnerGroup::SpellAttackProcedure,
            }
        }
        BattleSubjectKind::SaveGatedAreaDamage
        | BattleSubjectKind::SaveGatedTargetListConditionChoice => {
            match battle_reducer_route_fill_kind(fill) {
                Some(BattleReducerRouteFillKind::SavingThrowOutcome) => {
                    BattleReducerRouteOwnerGroup::HoleFrontier
                }
                Some(BattleReducerRouteFillKind::RolledDice) => {
                    BattleReducerRouteOwnerGroup::HitPoint
                }
                Some(
                    BattleReducerRouteFillKind::SpellTargetList
                    | BattleReducerRouteFillKind::ConditionChoice,
                ) => BattleReducerRouteOwnerGroup::HoleFrontier,
                _ => BattleReducerRouteOwnerGroup::SpellSlotAndActionEconomy,
            }
        }
        BattleSubjectKind::WeaponMasteryProperty => match (fill, outcome, holes) {
            (
                BattleFill::WeaponMasteryProperty(BattleWeaponMasteryPropertyFill {
                    property: WeaponMasteryProperty::Topple,
                    ..
                }),
                BattleResolutionOutcome::NeedsHoles,
                _,
            ) => BattleReducerRouteOwnerGroup::ConditionLifecycle,
            (
                BattleFill::WeaponMasteryProperty(BattleWeaponMasteryPropertyFill {
                    property: WeaponMasteryProperty::Sap,
                    ..
                }),
                BattleResolutionOutcome::Resolved,
                _,
            ) => BattleReducerRouteOwnerGroup::ActiveEffect,
            _ => BattleReducerRouteOwnerGroup::FeatureResource,
        },
        BattleSubjectKind::AttackActionAreaSaveDamageReplacement => match outcome {
            BattleResolutionOutcome::Invalid(_) if holes.contains(&BattleHoleKind::RolledDice) => {
                BattleReducerRouteOwnerGroup::DamageRoll
            }
            BattleResolutionOutcome::Invalid(_)
                if holes.contains(&BattleHoleKind::SavingThrowOutcome) =>
            {
                BattleReducerRouteOwnerGroup::AreaShape
            }
            BattleResolutionOutcome::Invalid(_) => BattleReducerRouteOwnerGroup::FeatureResource,
            BattleResolutionOutcome::Resolved | BattleResolutionOutcome::NeedsHoles => {
                BattleReducerRouteOwnerGroup::FeatureResource
            }
        },
        BattleSubjectKind::UnitFeatureBonusAction => BattleReducerRouteOwnerGroup::FeatureResource,
        BattleSubjectKind::ActiveFeatureSpellSaveDc
        | BattleSubjectKind::ActiveFeatureSpellAttackRollMode => {
            BattleReducerRouteOwnerGroup::ActiveEffect
        }
        BattleSubjectKind::ArmorClassSpellEffect => BattleReducerRouteOwnerGroup::ActiveEffect,
        BattleSubjectKind::ReactionSpell => match fill {
            BattleFill::ReactionSpell(BattleReactionSpellFill::ArmorClassInterruption(_)) => {
                BattleReducerRouteOwnerGroup::ActiveEffect
            }
            BattleFill::ReactionSpell(BattleReactionSpellFill::FailedSaveDamage(_)) => {
                BattleReducerRouteOwnerGroup::HitPoint
            }
            _ => BattleReducerRouteOwnerGroup::Reaction,
        },
        BattleSubjectKind::MetamagicOptionSpell => match outcome {
            BattleResolutionOutcome::Invalid(BattleResolutionInvalidReason::StaleSubject) => {
                BattleReducerRouteOwnerGroup::SpellSlotAndActionEconomy
            }
            BattleResolutionOutcome::Invalid(BattleResolutionInvalidReason::InvalidFill) => {
                BattleReducerRouteOwnerGroup::FeatureResource
            }
            BattleResolutionOutcome::Invalid(
                BattleResolutionInvalidReason::MetamagicOptionEffectMismatch,
            ) => BattleReducerRouteOwnerGroup::FeatureResource,
            BattleResolutionOutcome::Invalid(
                BattleResolutionInvalidReason::WrongActor
                | BattleResolutionInvalidReason::WrongTarget,
            ) => BattleReducerRouteOwnerGroup::ActionEconomy,
            BattleResolutionOutcome::Resolved | BattleResolutionOutcome::NeedsHoles => match fill {
                BattleFill::MetamagicOptionSpell(BattleMetamagicOptionSpellFill {
                    effect: BattleMetamagicOptionSpellEffect::ProtectedSaveGatedDamage { .. },
                    ..
                }) => BattleReducerRouteOwnerGroup::DamageAdjustment,
                BattleFill::MetamagicOptionSpell(BattleMetamagicOptionSpellFill {
                    effect: BattleMetamagicOptionSpellEffect::ProtectedSaveGatedNoEffect { .. },
                    ..
                }) => BattleReducerRouteOwnerGroup::SavingThrowOutcome,
                BattleFill::MetamagicOptionSpell(BattleMetamagicOptionSpellFill {
                    effect:
                        BattleMetamagicOptionSpellEffect::FirstTargetDisadvantageCondition { .. },
                    ..
                }) => BattleReducerRouteOwnerGroup::ConditionLifecycle,
                BattleFill::MetamagicOptionSpell(BattleMetamagicOptionSpellFill {
                    effect:
                        BattleMetamagicOptionSpellEffect::FirstTargetDisadvantageSaveGatedDamage {
                            ..
                        }
                        | BattleMetamagicOptionSpellEffect::FirstTargetDisadvantageEntrySave
                        | BattleMetamagicOptionSpellEffect::FirstTargetDisadvantageEndTurnSave
                        | BattleMetamagicOptionSpellEffect::FirstTargetDisadvantageConditionEndTurnSave,
                    ..
                }) => BattleReducerRouteOwnerGroup::SavingThrowRollMode,
                BattleFill::MetamagicOptionSpell(BattleMetamagicOptionSpellFill {
                    effect: BattleMetamagicOptionSpellEffect::ObjectRangeLight { .. },
                    ..
                }) => BattleReducerRouteOwnerGroup::ObjectBoundary,
                BattleFill::MetamagicOptionSpell(BattleMetamagicOptionSpellFill {
                    effect: BattleMetamagicOptionSpellEffect::AdditionalSingleTarget { .. },
                    ..
                }) => BattleReducerRouteOwnerGroup::TargetSelection,
                BattleFill::MetamagicOptionSpell(BattleMetamagicOptionSpellFill {
                    effect: BattleMetamagicOptionSpellEffect::DamageReroll { .. },
                    ..
                }) => BattleReducerRouteOwnerGroup::DamageRoll,
                BattleFill::MetamagicOptionSpell(BattleMetamagicOptionSpellFill {
                    effect: BattleMetamagicOptionSpellEffect::SpellAttackReroll { .. },
                    ..
                }) => BattleReducerRouteOwnerGroup::AttackRoll,
                BattleFill::MetamagicOptionSpell(BattleMetamagicOptionSpellFill {
                    effect: BattleMetamagicOptionSpellEffect::ComponentSuppressedHitPointBuff { .. },
                    ..
                }) => BattleReducerRouteOwnerGroup::Component,
                BattleFill::MetamagicOptionSpell(BattleMetamagicOptionSpellFill {
                    effect:
                        BattleMetamagicOptionSpellEffect::DamageTypeSubstitutionSaveGatedDamage {
                            ..
                        }
                        | BattleMetamagicOptionSpellEffect::DamageTypeSubstitutionSpellAttack { .. },
                    ..
                }) => BattleReducerRouteOwnerGroup::DamageType,
                BattleFill::MetamagicOptionSpell(BattleMetamagicOptionSpellFill {
                    effect: BattleMetamagicOptionSpellEffect::DurationExtension { .. },
                    ..
                }) => BattleReducerRouteOwnerGroup::Concentration,
                _ => BattleReducerRouteOwnerGroup::SpellSlotAndActionEconomy,
            },
        },
        BattleSubjectKind::Spatial(subject) => match outcome {
            BattleResolutionOutcome::Resolved | BattleResolutionOutcome::NeedsHoles => {
                spatial_route_resolution_owner(subject)
            }
            BattleResolutionOutcome::Invalid(BattleResolutionInvalidReason::StaleSubject) => {
                spatial_route_owner(subject)
            }
            BattleResolutionOutcome::Invalid(_) => {
                battle_discovery_route_owner(BattleSubjectKind::Spatial(subject))
            }
        },
        _ => battle_discovery_route_owner(subject),
    }
}

fn resolve_battle_subject_unchecked(
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
        (BattleSubjectKind::SlotSpell, BattleFill::SlotSpell(fill)) => {
            if !slot_spell_route_subject_is_live(&state, subject) {
                return invalid(
                    state,
                    BattleResolutionInvalidReason::StaleSubject,
                    subject.stage,
                );
            }
            resolve_slot_spell_battle_subject_result(state, subject, fill)
        }
        (
            BattleSubjectKind::SingleTargetSpellAttack | BattleSubjectKind::TypedSpellAttack,
            BattleFill::SpellAttack(fill),
        ) => {
            if !spell_attack_route_subject_is_live(&state, subject) {
                return invalid_with_holes(
                    state,
                    BattleResolutionInvalidReason::StaleSubject,
                    Vec::new(),
                );
            }
            resolve_spell_attack_battle_subject_result(state, subject, fill)
        }
        (
            BattleSubjectKind::SaveGatedAreaDamage
            | BattleSubjectKind::SaveGatedTargetListConditionChoice,
            BattleFill::SaveGatedSpell(fill),
        ) => {
            if !save_gated_spell_route_subject_is_live(&state, subject) {
                return invalid(
                    state,
                    BattleResolutionInvalidReason::StaleSubject,
                    subject.stage,
                );
            }
            resolve_save_gated_spell_battle_subject(state, subject, fill)
        }
        (
            BattleSubjectKind::HitPointRestorationSingleTargetSpell
            | BattleSubjectKind::HitPointRestorationTargetListSpell
            | BattleSubjectKind::HitPointRestorationFeatureHealingPool,
            BattleFill::HitPointRestoration(fill),
        ) => {
            if !hit_point_restoration_route_subject_is_live(&state, subject) {
                return invalid(
                    state,
                    BattleResolutionInvalidReason::StaleSubject,
                    subject.stage,
                );
            }
            resolve_hit_point_restoration_battle_subject(state, subject, fill)
        }
        (BattleSubjectKind::DeathSavingThrow, BattleFill::DeathSavingThrow(facts)) => {
            resolve_death_saving_throw_battle_subject(state, subject, facts)
        }
        (BattleSubjectKind::ConcentrationTeardown, BattleFill::Concentration(fill)) => {
            if !concentration_teardown_route_subject_is_live(&state, subject) {
                return invalid(
                    state,
                    BattleResolutionInvalidReason::StaleSubject,
                    subject.stage,
                );
            }
            resolve_concentration_teardown_battle_subject(state, subject, fill)
        }
        (BattleSubjectKind::CommandSpell, BattleFill::CommandEffect(fill)) => {
            if !command_effect_route_subject_is_live(&state, subject) {
                return invalid_with_holes(
                    state,
                    BattleResolutionInvalidReason::StaleSubject,
                    Vec::new(),
                );
            }
            resolve_command_effect_battle_subject(state, subject, fill)
        }
        (BattleSubjectKind::ArmorClassSpellEffect, BattleFill::ArmorClassSpellEffect(fill)) => {
            if !armor_class_spell_effect_route_subject_is_live(&state, subject) {
                return invalid_with_holes(
                    state,
                    BattleResolutionInvalidReason::StaleSubject,
                    Vec::new(),
                );
            }
            resolve_armor_class_spell_effect_subject(state, fill)
        }
        (BattleSubjectKind::ReactionSpell, BattleFill::ReactionSpell(fill)) => {
            if !reaction_spell_route_subject_is_live(&state, subject)
                || subject.actor != reaction_spell_fill_reactor(fill)
                || !reaction_spell_fill_matches_opportunity(&state, fill)
            {
                return invalid_with_holes(
                    state,
                    BattleResolutionInvalidReason::StaleSubject,
                    Vec::new(),
                );
            }
            resolve_reaction_spell_subject(state, fill)
        }
        (BattleSubjectKind::ScalarBuffTargetSpell, BattleFill::ScalarBuff(fill)) => {
            if !scalar_buff_route_subject_is_live(&state, subject) {
                return invalid_with_holes(
                    state,
                    BattleResolutionInvalidReason::StaleSubject,
                    Vec::new(),
                );
            }
            resolve_scalar_buff_battle_subject(state, fill)
        }
        (BattleSubjectKind::WeaponMasteryProperty, BattleFill::WeaponMasteryProperty(fill)) => {
            if !feature_substrate_route_subject_is_live(&state, subject) {
                return invalid_with_holes(
                    state,
                    BattleResolutionInvalidReason::StaleSubject,
                    Vec::new(),
                );
            }
            resolve_weapon_mastery_property_subject(state, fill)
        }
        (
            BattleSubjectKind::AttackActionAreaSaveDamageReplacement,
            BattleFill::AttackActionAreaSaveDamageReplacement(fill),
        ) => {
            if !feature_substrate_route_subject_is_live(&state, subject) {
                return invalid_with_holes(
                    state,
                    BattleResolutionInvalidReason::StaleSubject,
                    Vec::new(),
                );
            }
            resolve_attack_action_area_save_damage_replacement_subject(state, fill)
        }
        (BattleSubjectKind::UnitFeatureBonusAction, BattleFill::UnitFeatureBonusAction(fill)) => {
            if !feature_substrate_route_subject_is_live(&state, subject) {
                return invalid_with_holes(
                    state,
                    BattleResolutionInvalidReason::StaleSubject,
                    Vec::new(),
                );
            }
            resolve_unit_feature_bonus_action_subject(state, fill)
        }
        (
            BattleSubjectKind::ActiveFeatureSpellSaveDc,
            BattleFill::ActiveFeatureSpellSaveDc(fill),
        ) => {
            if !feature_substrate_route_subject_is_live(&state, subject) {
                return invalid_with_holes(
                    state,
                    BattleResolutionInvalidReason::StaleSubject,
                    Vec::new(),
                );
            }
            resolve_active_feature_spell_save_dc_subject(state, fill)
        }
        (
            BattleSubjectKind::ActiveFeatureSpellAttackRollMode,
            BattleFill::ActiveFeatureSpellAttackRollMode(fill),
        ) => {
            if !feature_substrate_route_subject_is_live(&state, subject) {
                return invalid_with_holes(
                    state,
                    BattleResolutionInvalidReason::StaleSubject,
                    Vec::new(),
                );
            }
            resolve_active_feature_spell_attack_roll_mode_subject(state, fill)
        }
        (BattleSubjectKind::MetamagicOptionSpell, BattleFill::MetamagicOptionSpell(fill)) => {
            if !metamagic_option_spell_route_subject_is_live(&state, subject) {
                return invalid_with_holes(
                    state,
                    BattleResolutionInvalidReason::StaleSubject,
                    Vec::new(),
                );
            }
            resolve_metamagic_option_spell_subject(state, subject, fill)
        }
        (BattleSubjectKind::Spatial(spatial_subject), BattleFill::Spatial(fill)) => {
            if !spatial_route_subject_is_live(&state, subject) {
                return invalid_with_holes(
                    state,
                    BattleResolutionInvalidReason::StaleSubject,
                    Vec::new(),
                );
            }
            if fill != spatial_route_fill(spatial_subject) {
                return invalid_with_holes(
                    state,
                    BattleResolutionInvalidReason::InvalidFill,
                    Vec::new(),
                );
            }
            BattleResolutionResult::Resolved { state }
        }
        (
            BattleSubjectKind::StatBlockAction,
            BattleFill::StatBlockAction {
                subject: stat_block_subject,
                fill,
            },
        ) => resolve_stat_block_action_result(state, stat_block_subject, fill),
        (BattleSubjectKind::EndTurn, BattleFill::NoFill) => {
            if !diagnostic_subject_shape_matches(subject, BattleSubjectKind::EndTurn, None) {
                return invalid(
                    state,
                    BattleResolutionInvalidReason::StaleSubject,
                    subject.stage,
                );
            }
            BattleResolutionResult::TurnAdvanced {
                turn_advance: advance_turn(state),
            }
        }
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
                BattleFill::NoFill
                | BattleFill::ResolveMultiattack
                | BattleFill::SpendMultiattackDispatch
                | BattleFill::SpellAttack(_)
                | BattleFill::SlotSpell(_)
                | BattleFill::SaveGatedSpell(_)
                | BattleFill::HitPointRestoration(_)
                | BattleFill::DeathSavingThrow(_)
                | BattleFill::Concentration(_)
                | BattleFill::CommandEffect(_)
                | BattleFill::ArmorClassSpellEffect(_)
                | BattleFill::ReactionSpell(_)
                | BattleFill::ScalarBuff(_)
                | BattleFill::WeaponMasteryProperty(_)
                | BattleFill::AttackActionAreaSaveDamageReplacement(_)
                | BattleFill::UnitFeatureBonusAction(_)
                | BattleFill::ActiveFeatureSpellSaveDc(_)
                | BattleFill::ActiveFeatureSpellAttackRollMode(_)
                | BattleFill::MetamagicOptionSpell(_)
                | BattleFill::Spatial(_)
                | BattleFill::StatBlockAction { .. } => invalid(
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
        (
            BattleSubjectKind::SlotSpell
            | BattleSubjectKind::SingleTargetSpellAttack
            | BattleSubjectKind::TypedSpellAttack
            | BattleSubjectKind::SaveGatedAreaDamage
            | BattleSubjectKind::SaveGatedTargetListConditionChoice
            | BattleSubjectKind::HitPointRestorationSingleTargetSpell
            | BattleSubjectKind::HitPointRestorationTargetListSpell
            | BattleSubjectKind::HitPointRestorationFeatureHealingPool
            | BattleSubjectKind::DeathSavingThrow
            | BattleSubjectKind::ConcentrationTeardown
            | BattleSubjectKind::CommandSpell
            | BattleSubjectKind::ArmorClassSpellEffect
            | BattleSubjectKind::ReactionSpell
            | BattleSubjectKind::ScalarBuffTargetSpell
            | BattleSubjectKind::WeaponMasteryProperty
            | BattleSubjectKind::AttackActionAreaSaveDamageReplacement
            | BattleSubjectKind::UnitFeatureBonusAction
            | BattleSubjectKind::ActiveFeatureSpellSaveDc
            | BattleSubjectKind::ActiveFeatureSpellAttackRollMode
            | BattleSubjectKind::MetamagicOptionSpell
            | BattleSubjectKind::Spatial(_)
            | BattleSubjectKind::StatBlockAction
            | BattleSubjectKind::EndTurn,
            _,
        ) => invalid(
            state,
            BattleResolutionInvalidReason::InvalidFill,
            subject.stage,
        ),
    }
}

#[must_use]
pub fn save_gated_spell_ordering_from_battle(state: &BattleState) -> SaveGatedSpellOrderingState {
    match state.save_gated_spell_procedure {
        BattleSaveGatedSpellProcedure::Inactive => {
            save_gated_spell_ordering_projection(SaveGatedSpellOrderingProjectionFacts {
                stage: SaveGatedSpellFrontierStage::ActSelection,
                runtime_result: SaveGatedSpellRuntimeResult::Init,
                last_ordering_error: None,
            })
        }
        BattleSaveGatedSpellProcedure::Active(subject) => {
            save_gated_spell_ordering_projection(SaveGatedSpellOrderingProjectionFacts {
                stage: subject.stage,
                runtime_result: if subject.stage == SaveGatedSpellFrontierStage::Resolved {
                    SaveGatedSpellRuntimeResult::Resolved
                } else {
                    SaveGatedSpellRuntimeResult::NeedsHoles
                },
                last_ordering_error: subject.last_ordering_error,
            })
        }
    }
}

#[must_use]
pub fn hit_point_restoration_from_battle(state: &BattleState) -> HitPointRestorationState {
    match state.hit_point_restoration_procedure {
        BattleHitPointRestorationProcedure::Inactive => {
            hit_point_restoration_projection(HitPointRestorationProjectionFacts {
                stage: HitPointRestorationFrontierStage::ActSelection,
                runtime_result: HitPointRestorationRuntimeResult::Init,
                last_ordering_error: None,
                spell_target_hit_points: 0,
                spell_target_zero_hit_point_lifecycle_cleared: false,
                feature_target_hit_points: 0,
                feature_target_zero_hit_point_lifecycle_cleared: false,
            })
        }
        BattleHitPointRestorationProcedure::Active(subject) => {
            let spell_target = hit_point_restoration_spell_target(subject);
            let feature_target = hit_point_restoration_feature_target(subject);
            let spell_target_hit_points = spell_target
                .map(|actor| combatant_for(state, actor).hp)
                .unwrap_or(0);
            let feature_target_hit_points = feature_target
                .map(|actor| combatant_for(state, actor).hp)
                .unwrap_or(0);
            hit_point_restoration_projection(HitPointRestorationProjectionFacts {
                stage: subject.stage,
                runtime_result: if subject.stage == HitPointRestorationFrontierStage::Resolved {
                    HitPointRestorationRuntimeResult::Resolved
                } else {
                    HitPointRestorationRuntimeResult::NeedsHoles
                },
                last_ordering_error: subject.last_ordering_error,
                spell_target_hit_points,
                spell_target_zero_hit_point_lifecycle_cleared: spell_target
                    .map(|actor| zero_hit_point_lifecycle_cleared(combatant_for(state, actor)))
                    .unwrap_or(false),
                feature_target_hit_points,
                feature_target_zero_hit_point_lifecycle_cleared: feature_target
                    .map(|actor| zero_hit_point_lifecycle_cleared(combatant_for(state, actor)))
                    .unwrap_or(false),
            })
        }
    }
}

fn resolve_slot_spell_battle_subject_result(
    state: BattleState,
    subject: BattleSubject,
    fill: BattleSlotSpellFill,
) -> BattleResolutionResult {
    let state = if matches!(
        state.slot_spell_procedure,
        BattleSlotSpellProcedure::Inactive
    ) {
        discover_slot_spell_battle(state)
    } else {
        state
    };
    let state = resolve_slot_spell_subject(state, fill);
    let holes = slot_spell_hole_kinds(&state);
    if holes.is_empty() {
        BattleResolutionResult::Resolved { state }
    } else {
        BattleResolutionResult::NeedsHoles {
            state,
            subject,
            holes,
        }
    }
}

fn resolve_spell_attack_battle_subject_result(
    state: BattleState,
    subject: BattleSubject,
    fill: BattleSpellAttackFill,
) -> BattleResolutionResult {
    let BattleSpellAttackProcedure::Active(active) = state.spell_attack_procedure else {
        return invalid_with_holes(
            state,
            BattleResolutionInvalidReason::StaleSubject,
            Vec::new(),
        );
    };
    let fill_kind = spell_attack_fill_kind(fill);
    let attack_roll_hits = spell_attack_fill_attack_hits(&state, &active, fill);
    let result = spell_attack_fill_order_result(active.stage, fill_kind, attack_roll_hits);
    let next_stage = spell_attack_result_stage(result);
    let next_active = BattleSpellAttackSubject {
        target: spell_attack_target_after_fill(active.target, fill, result),
        stage: next_stage,
        last_ordering_error: spell_attack_fill_order_error(result),
        ..active
    };
    let next_subject = BattleSubject {
        target: next_active.target,
        ..subject
    };
    let next_state = BattleState {
        spell_attack_procedure: BattleSpellAttackProcedure::Active(next_active),
        ..state
    };
    if spell_attack_fill_order_runtime_result(result) == SpellAttackRuntimeResult::Resolved {
        BattleResolutionResult::Resolved {
            state: finalize_spell_attack_subject(
                next_state,
                next_active,
                spell_attack_damage_from_fill(fill),
            ),
        }
    } else {
        BattleResolutionResult::NeedsHoles {
            state: next_state,
            subject: next_subject,
            holes: spell_attack_hole_kinds(next_stage),
        }
    }
}

fn resolve_save_gated_spell_battle_subject(
    state: BattleState,
    subject: BattleSubject,
    fill: BattleSaveGatedSpellFill,
) -> BattleResolutionResult {
    let current_subject = active_or_initial_save_gated_subject(&state, subject);
    let fill_kind = save_gated_spell_fill_kind(fill);
    let result = save_gated_spell_fill_order_result(
        current_subject.stage,
        fill_kind,
        current_subject.damage_dice_required,
    );
    let next_stage = save_gated_spell_fill_order_accepted_stage(result, current_subject.stage);
    let next_subject = BattleSaveGatedSpellSubject {
        stage: next_stage,
        last_ordering_error: save_gated_spell_fill_order_error(result),
        ..current_subject
    };
    let state = BattleState {
        save_gated_spell_procedure: BattleSaveGatedSpellProcedure::Active(next_subject),
        ..state
    };
    if save_gated_spell_fill_order_runtime_result(result) == SaveGatedSpellRuntimeResult::Resolved {
        BattleResolutionResult::Resolved {
            state: finalize_save_gated_spell_subject(state, next_subject),
        }
    } else {
        BattleResolutionResult::NeedsHoles {
            state,
            subject,
            holes: save_gated_spell_hole_kinds(next_stage),
        }
    }
}

fn resolve_hit_point_restoration_battle_subject(
    state: BattleState,
    subject: BattleSubject,
    fill: BattleHitPointRestorationFill,
) -> BattleResolutionResult {
    let current_subject = active_or_initial_hit_point_restoration_subject(&state, subject);
    let fill_kind = hit_point_restoration_fill_kind(fill);
    let result = hit_point_restoration_fill_order_result(current_subject.stage, fill_kind);
    if matches!(result, HitPointRestorationFillOrderResult::Accepted(_)) {
        if let Some(target) = hit_point_restoration_fill_target(current_subject, fill) {
            if !ordinary_zero_hit_point_restoration_target(combatant_for(&state, target)) {
                return invalid(
                    state,
                    BattleResolutionInvalidReason::WrongTarget,
                    subject.stage,
                );
            }
        } else {
            return invalid(
                state,
                BattleResolutionInvalidReason::InvalidFill,
                subject.stage,
            );
        }
    }
    let next_stage = hit_point_restoration_fill_order_accepted_stage(result);
    let mut next_subject = BattleHitPointRestorationSubject {
        stage: next_stage,
        last_ordering_error: hit_point_restoration_fill_order_error(result),
        ..current_subject
    };
    next_subject = hit_point_restoration_target_after_fill(next_subject, fill);
    let mut state = BattleState {
        hit_point_restoration_procedure: BattleHitPointRestorationProcedure::Active(next_subject),
        ..state
    };
    if hit_point_restoration_fill_order_runtime_result(result)
        == HitPointRestorationRuntimeResult::Resolved
    {
        state = apply_hit_point_restoration_fill(state, next_subject, fill);
        BattleResolutionResult::Resolved { state }
    } else {
        BattleResolutionResult::NeedsHoles {
            state,
            subject,
            holes: hit_point_restoration_hole_kinds(next_stage),
        }
    }
}

fn resolve_death_saving_throw_battle_subject(
    state: BattleState,
    subject: BattleSubject,
    facts: DeathSavingThrowFacts,
) -> BattleResolutionResult {
    let target = subject.target.unwrap_or(subject.actor);
    let Some(death_save_state) =
        death_saving_throw_state_from_combatant(combatant_for(&state, target))
    else {
        return BattleResolutionResult::Invalid {
            state,
            reason: BattleResolutionInvalidReason::WrongTarget,
            holes: Vec::new(),
        };
    };
    let discovered = if death_save_state.protocol == DeathSavingThrowProtocol::NeedsSavingThrow {
        death_save_state
    } else {
        discover_death_saving_throw(death_save_state)
    };
    let filled = fill_death_saving_throw(discovered, facts);
    if let DeathSavingThrowProtocol::Invalid(DeathSavingThrowInvalidReason::InvalidFill) =
        filled.protocol
    {
        return BattleResolutionResult::Invalid {
            state,
            reason: BattleResolutionInvalidReason::InvalidFill,
            holes: vec![BattleHoleKind::DeathSavingThrow],
        };
    }

    let mut state = state;
    let Some(combatant) =
        combatant_from_death_saving_throw_state(combatant_for(&state, target), filled)
    else {
        return BattleResolutionResult::Invalid {
            state,
            reason: BattleResolutionInvalidReason::WrongTarget,
            holes: Vec::new(),
        };
    };
    *combatant_for_mut(&mut state, target) = combatant;
    BattleResolutionResult::Resolved {
        state: advance_turn_state(state),
    }
}

fn resolve_concentration_teardown_battle_subject(
    state: BattleState,
    _subject: BattleSubject,
    fill: BattleConcentrationFill,
) -> BattleResolutionResult {
    let concentration = match fill {
        BattleConcentrationFill::CastSpell => cast_concentration_spell(state.concentration.clone()),
        BattleConcentrationFill::DamageTaken(facts) => {
            request_concentration_save_after_damage(state.concentration.clone(), facts)
        }
        BattleConcentrationFill::SavingThrow(facts) => {
            match fail_concentration_saving_throw(state.concentration.clone(), facts) {
                Ok(next) => next,
                Err(_error) => {
                    return BattleResolutionResult::Invalid {
                        state,
                        reason: BattleResolutionInvalidReason::InvalidFill,
                        holes: vec![BattleHoleKind::ConcentrationSavingThrow],
                    };
                }
            }
        }
        BattleConcentrationFill::VoluntaryEnd => {
            voluntarily_end_concentration(state.concentration.clone())
        }
        BattleConcentrationFill::CastReplacementSpell => {
            cast_replacement_concentration_spell(state.concentration.clone())
        }
    };
    let holes = concentration_holes(&concentration);
    let state = BattleState {
        concentration,
        ..state
    };
    if holes.is_empty() {
        BattleResolutionResult::Resolved { state }
    } else {
        BattleResolutionResult::NeedsHoles {
            state,
            subject: _subject,
            holes,
        }
    }
}

fn active_or_initial_save_gated_subject(
    state: &BattleState,
    subject: BattleSubject,
) -> BattleSaveGatedSpellSubject {
    match state.save_gated_spell_procedure {
        BattleSaveGatedSpellProcedure::Active(active) => active,
        BattleSaveGatedSpellProcedure::Inactive => match subject.kind {
            BattleSubjectKind::SaveGatedAreaDamage => BattleSaveGatedSpellSubject {
                actor: subject.actor,
                stage: SaveGatedSpellFrontierStage::DamageSavingThrowOutcome,
                damage_dice_required: true,
                last_ordering_error: None,
                damage_application: BattleSaveGatedDamageApplication::none(subject.actor),
            },
            BattleSubjectKind::SaveGatedTargetListConditionChoice => BattleSaveGatedSpellSubject {
                actor: subject.actor,
                stage: SaveGatedSpellFrontierStage::TargetListAndConditionChoice,
                damage_dice_required: false,
                last_ordering_error: None,
                damage_application: BattleSaveGatedDamageApplication::none(subject.actor),
            },
            _ => BattleSaveGatedSpellSubject {
                actor: subject.actor,
                stage: SaveGatedSpellFrontierStage::ActSelection,
                damage_dice_required: false,
                last_ordering_error: None,
                damage_application: BattleSaveGatedDamageApplication::none(subject.actor),
            },
        },
    }
}

fn active_or_initial_hit_point_restoration_subject(
    state: &BattleState,
    subject: BattleSubject,
) -> BattleHitPointRestorationSubject {
    match state.hit_point_restoration_procedure {
        BattleHitPointRestorationProcedure::Active(active) => active,
        BattleHitPointRestorationProcedure::Inactive => match subject.kind {
            BattleSubjectKind::HitPointRestorationSingleTargetSpell => {
                BattleHitPointRestorationSubject {
                    actor: subject.actor,
                    stage: HitPointRestorationFrontierStage::SpellHealingTargetChoice,
                    targeting: BattleHitPointRestorationTargeting::SingleTargetSpell {
                        target: None,
                    },
                    last_ordering_error: None,
                }
            }
            BattleSubjectKind::HitPointRestorationTargetListSpell => {
                BattleHitPointRestorationSubject {
                    actor: subject.actor,
                    stage: HitPointRestorationFrontierStage::SpellHealingTargetList,
                    targeting: BattleHitPointRestorationTargeting::TargetListSpell { target: None },
                    last_ordering_error: None,
                }
            }
            BattleSubjectKind::HitPointRestorationFeatureHealingPool => {
                BattleHitPointRestorationSubject {
                    actor: subject.actor,
                    stage: HitPointRestorationFrontierStage::FeatureHealingPoolDistribution,
                    targeting: BattleHitPointRestorationTargeting::FeatureHealingPool {
                        target: None,
                    },
                    last_ordering_error: None,
                }
            }
            _ => BattleHitPointRestorationSubject {
                actor: subject.actor,
                stage: HitPointRestorationFrontierStage::ActSelection,
                targeting: BattleHitPointRestorationTargeting::SingleTargetSpell { target: None },
                last_ordering_error: None,
            },
        },
    }
}

fn save_gated_spell_fill_kind(fill: BattleSaveGatedSpellFill) -> SaveGatedSpellFillKind {
    match fill {
        BattleSaveGatedSpellFill::SpellTargetList => SaveGatedSpellFillKind::SpellTargetList,
        BattleSaveGatedSpellFill::ConditionChoice => SaveGatedSpellFillKind::ConditionChoice,
        BattleSaveGatedSpellFill::SavingThrowOutcome => SaveGatedSpellFillKind::SavingThrowOutcome,
        BattleSaveGatedSpellFill::DamageRoll => SaveGatedSpellFillKind::RolledDice,
    }
}

fn hit_point_restoration_fill_kind(
    fill: BattleHitPointRestorationFill,
) -> HitPointRestorationFillKind {
    match fill {
        BattleHitPointRestorationFill::TargetChoice(_) => HitPointRestorationFillKind::TargetChoice,
        BattleHitPointRestorationFill::SpellTargetList(_) => {
            HitPointRestorationFillKind::SpellTargetList
        }
        BattleHitPointRestorationFill::HealingRoll(_) => HitPointRestorationFillKind::RolledDice,
        BattleHitPointRestorationFill::HitPointHealingDistribution { .. } => {
            HitPointRestorationFillKind::HitPointHealingDistribution
        }
    }
}

fn hit_point_restoration_target_after_fill(
    subject: BattleHitPointRestorationSubject,
    fill: BattleHitPointRestorationFill,
) -> BattleHitPointRestorationSubject {
    let targeting = match (subject.targeting, fill) {
        (
            BattleHitPointRestorationTargeting::SingleTargetSpell { .. },
            BattleHitPointRestorationFill::TargetChoice(target),
        ) => BattleHitPointRestorationTargeting::SingleTargetSpell {
            target: Some(target),
        },
        (
            BattleHitPointRestorationTargeting::TargetListSpell { .. },
            BattleHitPointRestorationFill::SpellTargetList(target),
        ) => BattleHitPointRestorationTargeting::TargetListSpell {
            target: Some(target),
        },
        (
            BattleHitPointRestorationTargeting::FeatureHealingPool { .. },
            BattleHitPointRestorationFill::HitPointHealingDistribution { target, .. },
        ) => BattleHitPointRestorationTargeting::FeatureHealingPool {
            target: Some(target),
        },
        (targeting, _) => targeting,
    };
    BattleHitPointRestorationSubject {
        targeting,
        ..subject
    }
}

fn hit_point_restoration_spell_target(subject: BattleHitPointRestorationSubject) -> Option<Actor> {
    match subject.targeting {
        BattleHitPointRestorationTargeting::SingleTargetSpell { target }
        | BattleHitPointRestorationTargeting::TargetListSpell { target } => target,
        BattleHitPointRestorationTargeting::FeatureHealingPool { .. } => None,
    }
}

fn hit_point_restoration_feature_target(
    subject: BattleHitPointRestorationSubject,
) -> Option<Actor> {
    match subject.targeting {
        BattleHitPointRestorationTargeting::FeatureHealingPool { target } => target,
        BattleHitPointRestorationTargeting::SingleTargetSpell { .. }
        | BattleHitPointRestorationTargeting::TargetListSpell { .. } => None,
    }
}

fn hit_point_restoration_fill_target(
    subject: BattleHitPointRestorationSubject,
    fill: BattleHitPointRestorationFill,
) -> Option<Actor> {
    match fill {
        BattleHitPointRestorationFill::TargetChoice(target)
        | BattleHitPointRestorationFill::SpellTargetList(target)
        | BattleHitPointRestorationFill::HitPointHealingDistribution { target, .. } => Some(target),
        BattleHitPointRestorationFill::HealingRoll(_) => {
            hit_point_restoration_spell_target(subject)
        }
    }
}

fn apply_hit_point_restoration_fill(
    state: BattleState,
    subject: BattleHitPointRestorationSubject,
    fill: BattleHitPointRestorationFill,
) -> BattleState {
    match fill {
        BattleHitPointRestorationFill::HealingRoll(amount) => {
            if let Some(target) = hit_point_restoration_spell_target(subject) {
                with_healed_target(state, target, amount)
            } else {
                state
            }
        }
        BattleHitPointRestorationFill::HitPointHealingDistribution { target, amount } => {
            with_healed_target(state, target, amount)
        }
        BattleHitPointRestorationFill::TargetChoice(_)
        | BattleHitPointRestorationFill::SpellTargetList(_) => state,
    }
}

#[must_use]
pub fn start_stat_block_multiattack_control(
    state: BattleState,
    actor: Actor,
    profile: StatBlockMultiattackProfile,
) -> BattleResolutionResult {
    if current_actor(&state) != actor
        || !state.action_available
        || combatant_for(&state, actor).hp <= 0
    {
        return invalid_stat_block_action(
            state,
            initial_stat_block_action_subject(
                actor,
                StatBlockActionFrontierStage::ActSelection,
                StatBlockActionDamageMode::Rolled,
                false,
            ),
            BattleResolutionInvalidReason::StaleSubject,
            None,
        );
    }

    let stat_block_control =
        start_stat_block_multiattack_from(state.stat_block_control.clone(), profile);
    if stat_block_control == state.stat_block_control {
        return invalid_stat_block_action(
            state,
            initial_stat_block_action_subject(
                actor,
                StatBlockActionFrontierStage::ActSelection,
                StatBlockActionDamageMode::Rolled,
                false,
            ),
            BattleResolutionInvalidReason::StaleSubject,
            None,
        );
    }

    let state = BattleState {
        action_available: stat_block_control.attack_action_available,
        stat_block_control,
        ..state
    };
    let subject = initial_stat_block_action_subject(
        actor,
        StatBlockActionFrontierStage::AttackTargetChoice,
        StatBlockActionDamageMode::Rolled,
        false,
    );
    BattleResolutionResult::StatBlockNeedsHoles {
        state,
        subject,
        holes: stat_block_action_hole_kinds(subject.stage),
    }
}

#[must_use]
pub fn discover_rolled_stat_block_attack_control(
    state: BattleState,
    actor: Actor,
) -> BattleResolutionResult {
    discover_stat_block_attack_control(state, actor, StatBlockActionDamageMode::Rolled, true)
}

#[must_use]
pub fn discover_static_stat_block_attack_control(
    state: BattleState,
    actor: Actor,
    damage: i16,
) -> BattleResolutionResult {
    // QNT: battle-runtime-stat-block-multi-damage.mbt.qnt
    // `targetInitialHp` 12 -> `targetHpAfterStaticPiercingOnly` 9.
    discover_stat_block_attack_control(
        state,
        actor,
        StatBlockActionDamageMode::Static { damage },
        false,
    )
}

#[must_use]
pub fn discover_prone_rider_stat_block_attack_control(
    state: BattleState,
    actor: Actor,
    target_prone_condition_immune: bool,
) -> BattleResolutionResult {
    if current_actor(&state) != actor
        || !state.action_available
        || combatant_for(&state, actor).hp <= 0
    {
        return invalid_stat_block_action(
            state,
            initial_stat_block_action_subject_with_prone_rider(
                actor,
                StatBlockActionFrontierStage::ActSelection,
                StatBlockActionDamageMode::Rolled,
                false,
                StatBlockProneOnHitRider::MediumOrSmaller {
                    target_prone_condition_immune,
                },
            ),
            BattleResolutionInvalidReason::StaleSubject,
            None,
        );
    }

    let subject = initial_stat_block_action_subject_with_prone_rider(
        actor,
        StatBlockActionFrontierStage::AttackTargetChoice,
        StatBlockActionDamageMode::Rolled,
        false,
        StatBlockProneOnHitRider::MediumOrSmaller {
            target_prone_condition_immune,
        },
    );
    BattleResolutionResult::StatBlockNeedsHoles {
        state,
        subject,
        holes: stat_block_action_hole_kinds(subject.stage),
    }
}

#[must_use]
pub fn spend_recharge_gated_rolled_stat_block_attack(
    state: BattleState,
    actor: Actor,
) -> BattleResolutionResult {
    if current_actor(&state) != actor
        || !state.action_available
        || !combatant_for(&state, actor).recharge_available
        || combatant_for(&state, actor).hp <= 0
    {
        return invalid_stat_block_action(
            state,
            initial_stat_block_action_subject(
                actor,
                StatBlockActionFrontierStage::ActSelection,
                StatBlockActionDamageMode::Rolled,
                false,
            ),
            BattleResolutionInvalidReason::StaleSubject,
            None,
        );
    }

    let mut state = state;
    combatant_for_mut(&mut state, actor).recharge_available = false;
    let state = BattleState {
        action_available: false,
        ..state
    };
    let subject = initial_stat_block_action_subject(
        actor,
        StatBlockActionFrontierStage::RechargeRoll,
        StatBlockActionDamageMode::Rolled,
        false,
    );
    BattleResolutionResult::StatBlockNeedsHoles {
        state,
        subject,
        holes: stat_block_action_hole_kinds(subject.stage),
    }
}

fn resolve_stat_block_action_result(
    state: BattleState,
    subject: StatBlockActionSubject,
    fill: StatBlockActionFill,
) -> BattleResolutionResult {
    if subject.actor != current_actor(&state) {
        return invalid_stat_block_action(
            state,
            subject,
            BattleResolutionInvalidReason::WrongActor,
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
    result: &BattleResolutionResult,
) -> StatBlockActionOrderingState {
    let (state, subject, runtime_result, last_ordering_error) = match result {
        BattleResolutionResult::StatBlockNeedsHoles { state, subject, .. } => (
            state,
            subject,
            StatBlockActionRuntimeResult::NeedsHoles,
            None,
        ),
        BattleResolutionResult::StatBlockResolved { state, subject } => {
            (state, subject, StatBlockActionRuntimeResult::Resolved, None)
        }
        BattleResolutionResult::StatBlockInvalid {
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
        other => panic!("expected stat-block action result, got {other:?}"),
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
        CombatantLifecycle::UsesDeathSavingThrows(DeathSavingThrowLifecycle::Dead) => true,
        CombatantLifecycle::UsesDeathSavingThrows(
            DeathSavingThrowLifecycle::MakingDeathSavingThrows { .. }
            | DeathSavingThrowLifecycle::Stable,
        ) => false,
    }
}

fn current_actor_can_attack(state: &BattleState) -> bool {
    if !state.action_available {
        return false;
    }

    combatant_can_attack(combatant_for(state, current_actor(state)))
}

fn combatant_can_attack(combatant: Combatant) -> bool {
    combatant.weapon_attack_supported
        && combatant.hp > 0
        && !combatant.unconscious
        && !combatant.incapacitated
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
        holes: weapon_hole_kinds(next_stage),
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
        holes: weapon_hole_kinds(next_stage),
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
        || current_actor(&state) != subject.actor
        || stat_block_multiattack_continuation_open(&state.stat_block_control)
    {
        return invalid(
            state,
            BattleResolutionInvalidReason::StaleSubject,
            subject.stage,
        );
    }

    let Some(profile) = combatant_for(&state, subject.actor).multiattack_profile else {
        return invalid(
            state,
            BattleResolutionInvalidReason::StaleSubject,
            subject.stage,
        );
    };

    let stat_block_control =
        start_stat_block_multiattack_from(state.stat_block_control.clone(), profile);
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
    if current_actor(&state) != subject.actor
        || !stat_block_multiattack_continuation_open(&state.stat_block_control)
        || combatant_for(&state, subject.actor)
            .multiattack_profile
            .is_none()
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

fn discover_stat_block_attack_control(
    state: BattleState,
    actor: Actor,
    damage_mode: StatBlockActionDamageMode,
    recharge_action_available: bool,
) -> BattleResolutionResult {
    if current_actor(&state) != actor
        || !state.action_available
        || combatant_for(&state, actor).hp <= 0
    {
        return invalid_stat_block_action(
            state,
            initial_stat_block_action_subject(
                actor,
                StatBlockActionFrontierStage::ActSelection,
                damage_mode,
                recharge_action_available,
            ),
            BattleResolutionInvalidReason::StaleSubject,
            None,
        );
    }

    let subject = initial_stat_block_action_subject(
        actor,
        StatBlockActionFrontierStage::AttackTargetChoice,
        damage_mode,
        recharge_action_available,
    );
    BattleResolutionResult::StatBlockNeedsHoles {
        state,
        subject,
        holes: stat_block_action_hole_kinds(subject.stage),
    }
}

fn resolve_stat_block_target_choice(
    state: BattleState,
    subject: StatBlockActionSubject,
    target: Actor,
) -> BattleResolutionResult {
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
                BattleResolutionInvalidReason::InvalidFill,
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
            BattleResolutionInvalidReason::WrongTarget,
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
) -> BattleResolutionResult {
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
                BattleResolutionInvalidReason::InvalidFill,
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
            BattleResolutionInvalidReason::InvalidFill,
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
        return BattleResolutionResult::StatBlockResolved {
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
) -> BattleResolutionResult {
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
                BattleResolutionInvalidReason::InvalidFill,
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
            BattleResolutionInvalidReason::InvalidFill,
            None,
        );
    };

    let state = with_damaged_target(state, target, rolled_damage);
    BattleResolutionResult::StatBlockResolved {
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
) -> BattleResolutionResult {
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
                BattleResolutionInvalidReason::InvalidFill,
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
    let mut state = state;
    combatant_for_mut(&mut state, subject.actor).recharge_available = recharged;
    BattleResolutionResult::StatBlockResolved {
        state,
        subject: StatBlockActionSubject {
            stage: next_stage,
            recharge_action_available: recharged,
            ..subject
        },
    }
}

#[must_use]
pub fn primary_stat_block_multiattack_profile(
    primary_attack_count: i16,
) -> StatBlockMultiattackProfile {
    StatBlockMultiattackProfile {
        first_attack_slot: StatBlockAttackSlot::Primary,
        primary_attack_count,
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
    invalid_with_holes(state, reason, weapon_hole_kinds(stage))
}

fn invalid_with_holes(
    state: BattleState,
    reason: BattleResolutionInvalidReason,
    holes: Vec<BattleHoleKind>,
) -> BattleResolutionResult {
    BattleResolutionResult::Invalid {
        state,
        reason,
        holes,
    }
}

fn weapon_hole_kinds(stage: WeaponAttackFrontierStage) -> Vec<BattleHoleKind> {
    weapon_attack_hole_frontier(stage)
        .into_iter()
        .map(|hole| match hole {
            WeaponAttackHoleKind::TargetChoice => BattleHoleKind::TargetChoice,
            WeaponAttackHoleKind::AttackRoll => BattleHoleKind::AttackRoll,
            WeaponAttackHoleKind::RolledDice => BattleHoleKind::RolledDice,
        })
        .collect()
}

fn spell_attack_hole_kinds(stage: SpellAttackFrontierStage) -> Vec<BattleHoleKind> {
    match stage {
        SpellAttackFrontierStage::ActSelection | SpellAttackFrontierStage::Resolved => Vec::new(),
        SpellAttackFrontierStage::TargetChoice | SpellAttackFrontierStage::TypedTargetChoice => {
            vec![BattleHoleKind::TargetChoice]
        }
        SpellAttackFrontierStage::TargetList => vec![BattleHoleKind::SpellTargetList],
        SpellAttackFrontierStage::TargetAllocation => vec![BattleHoleKind::SpellTargetAllocation],
        SpellAttackFrontierStage::DamageTypeAndTargetChoice => {
            vec![
                BattleHoleKind::DamageTypeChoice,
                BattleHoleKind::TargetChoice,
            ]
        }
        SpellAttackFrontierStage::DamageTypeChoice => vec![BattleHoleKind::DamageTypeChoice],
        SpellAttackFrontierStage::AttackRoll => vec![BattleHoleKind::AttackRoll],
        SpellAttackFrontierStage::DamageDice => vec![BattleHoleKind::RolledDice],
    }
}

fn slot_spell_hole_kinds(state: &BattleState) -> Vec<BattleHoleKind> {
    slot_spell_holes_from_battle(state)
        .into_iter()
        .map(|hole| match hole {
            BattleSlotSpellHole::SpellTargetAllocation => BattleHoleKind::SpellTargetAllocation,
            BattleSlotSpellHole::RolledDice => BattleHoleKind::RolledDice,
        })
        .collect()
}

fn save_gated_spell_hole_kinds(stage: SaveGatedSpellFrontierStage) -> Vec<BattleHoleKind> {
    save_gated_spell_hole_frontier(stage)
        .into_iter()
        .map(|hole| match hole {
            SaveGatedSpellHoleKind::SpellTargetList => BattleHoleKind::SpellTargetList,
            SaveGatedSpellHoleKind::ConditionChoice => BattleHoleKind::ConditionChoice,
            SaveGatedSpellHoleKind::SavingThrowOutcome => BattleHoleKind::SavingThrowOutcome,
            SaveGatedSpellHoleKind::RolledDice => BattleHoleKind::RolledDice,
        })
        .collect()
}

fn hit_point_restoration_hole_kinds(
    stage: HitPointRestorationFrontierStage,
) -> Vec<BattleHoleKind> {
    hit_point_restoration_hole_frontier(stage)
        .into_iter()
        .map(|hole| match hole {
            HitPointRestorationHoleKind::TargetChoice => BattleHoleKind::TargetChoice,
            HitPointRestorationHoleKind::SpellTargetList => BattleHoleKind::SpellTargetList,
            HitPointRestorationHoleKind::RolledDice => BattleHoleKind::RolledDice,
            HitPointRestorationHoleKind::HitPointHealingDistribution => {
                BattleHoleKind::HitPointHealingDistribution
            }
        })
        .collect()
}

fn command_hole_kinds(stage: CommandFrontierStage) -> Vec<BattleHoleKind> {
    command_hole_frontier(stage)
        .into_iter()
        .map(|hole| match hole {
            CommandHoleKind::TargetChoice => BattleHoleKind::TargetChoice,
            CommandHoleKind::SpellTargetList => BattleHoleKind::SpellTargetList,
            CommandHoleKind::CommandOptionChoice => BattleHoleKind::CommandOptionChoice,
            CommandHoleKind::SavingThrowOutcome => BattleHoleKind::SavingThrowOutcome,
            CommandHoleKind::Movement => BattleHoleKind::Movement,
        })
        .collect()
}

fn stat_block_action_hole_kinds(stage: StatBlockActionFrontierStage) -> Vec<BattleHoleKind> {
    stat_block_action_hole_frontier(stage)
        .into_iter()
        .map(|hole| match hole {
            StatBlockActionHoleKind::TargetChoice => BattleHoleKind::TargetChoice,
            StatBlockActionHoleKind::AttackRoll => BattleHoleKind::AttackRoll,
            StatBlockActionHoleKind::RolledDice => BattleHoleKind::RolledDice,
            StatBlockActionHoleKind::StatBlockRechargeRoll => BattleHoleKind::StatBlockRechargeRoll,
        })
        .collect()
}

const fn battle_subject_from_stat_block_action_subject(
    subject: StatBlockActionSubject,
) -> BattleSubject {
    BattleSubject {
        kind: BattleSubjectKind::StatBlockAction,
        actor: subject.actor,
        target: subject.target,
        stage: WeaponAttackFrontierStage::ActSelection,
        damage_modifier: 0,
    }
}

fn concentration_holes(state: &ConcentrationState) -> Vec<BattleHoleKind> {
    match state.protocol {
        ConcentrationProtocol::NeedsSavingThrow => {
            vec![BattleHoleKind::ConcentrationSavingThrow]
        }
        ConcentrationProtocol::Init | ConcentrationProtocol::Resolved => Vec::new(),
    }
}

fn death_saving_throw_available(combatant: Combatant) -> bool {
    let CombatantLifecycle::UsesDeathSavingThrows(death_save) = combatant.lifecycle else {
        return false;
    };
    combatant.hp == 0
        && combatant.unconscious
        && matches!(
            death_save,
            DeathSavingThrowLifecycle::MakingDeathSavingThrows { .. }
        )
}

#[must_use]
pub fn death_saving_throw_state_from_combatant(
    combatant: Combatant,
) -> Option<DeathSavingThrowState> {
    let CombatantLifecycle::UsesDeathSavingThrows(death_save) = combatant.lifecycle else {
        return None;
    };
    Some(DeathSavingThrowState {
        current_turn_role: DeathSavingThrowTurnRole::Actor,
        target_hp: combatant.hp,
        target_unconscious: combatant.unconscious,
        target_stable: matches!(death_save, DeathSavingThrowLifecycle::Stable),
        target_dead: matches!(death_save, DeathSavingThrowLifecycle::Dead),
        target_death_successes: death_saving_throw_successes(death_save),
        target_death_failures: death_saving_throw_failures(death_save),
        protocol: DeathSavingThrowProtocol::Init,
    })
}

fn combatant_from_death_saving_throw_state(
    combatant: Combatant,
    death_save: DeathSavingThrowState,
) -> Option<Combatant> {
    let CombatantLifecycle::UsesDeathSavingThrows(_) = combatant.lifecycle else {
        return None;
    };
    Some(Combatant {
        hp: death_save.target_hp,
        unconscious: death_save.target_unconscious,
        incapacitated: death_save.target_unconscious,
        prone: death_save.target_unconscious,
        lifecycle: CombatantLifecycle::UsesDeathSavingThrows(
            DeathSavingThrowLifecycle::from_state(death_save),
        ),
        ..combatant
    })
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
) -> BattleResolutionResult {
    BattleResolutionResult::StatBlockNeedsHoles {
        state,
        subject,
        holes: stat_block_action_hole_kinds(subject.stage),
    }
}

fn invalid_stat_block_action(
    state: BattleState,
    subject: StatBlockActionSubject,
    reason: BattleResolutionInvalidReason,
    last_ordering_error: Option<StatBlockActionFillOrderingError>,
) -> BattleResolutionResult {
    BattleResolutionResult::StatBlockInvalid {
        state,
        subject,
        reason,
        holes: stat_block_action_hole_kinds(subject.stage),
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
    spend_first_level_slot: bool,
    stage: SpellAttackFrontierStage,
    target_effect: BattleSpellActiveEffectKind,
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
        spend_first_level_slot,
        last_ordering_error: None,
        target_effect,
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

fn spell_attack_damage_from_fill(fill: BattleSpellAttackFill) -> i16 {
    match fill {
        BattleSpellAttackFill::DamageRoll(damage) => damage,
        BattleSpellAttackFill::TargetChoice(_)
        | BattleSpellAttackFill::DamageTypeChoice
        | BattleSpellAttackFill::AttackRoll(_) => 0,
    }
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
    rolled_damage: i16,
) -> BattleState {
    let state = if let Some(target) = subject.target {
        apply_spell_damage_to_target(state, target, rolled_damage, subject.target_effect)
    } else {
        state
    };
    if subject.spend_first_level_slot {
        let state = expend_combatant_spell_slot(state, subject.actor, BattleSpellSlotLevel::First);
        commit_actor_spell_slot_use(state, subject.actor)
    } else {
        state
    }
}

fn finalize_save_gated_spell_subject(
    state: BattleState,
    subject: BattleSaveGatedSpellSubject,
) -> BattleState {
    let application = subject.damage_application;
    let state = apply_spell_damage_to_target(
        state,
        application.primary_target,
        application.primary_damage,
        application.primary_effect,
    );
    let state = if let Some(target) = application.secondary_target {
        apply_spell_damage_to_target(
            state,
            target,
            application.secondary_damage,
            BattleSpellActiveEffectKind::None,
        )
    } else {
        state
    };
    if application.spend_first_level_slot {
        let state = expend_combatant_spell_slot(state, subject.actor, BattleSpellSlotLevel::First);
        commit_actor_spell_slot_use(state, subject.actor)
    } else {
        state
    }
}

fn apply_spell_damage_to_target(
    state: BattleState,
    target: Actor,
    damage: i16,
    effect: BattleSpellActiveEffectKind,
) -> BattleState {
    let mut state = with_damaged_target(state, target, damage);
    if effect != BattleSpellActiveEffectKind::None {
        combatant_for_mut(&mut state, target).spell_active_effects =
            BattleSpellActiveEffects::with_effect(effect);
    }
    state
}

fn finalize_slot_spell_damage(
    state: BattleState,
    subject: BattleSlotSpellSubject,
    dart_pips_sum: i16,
) -> BattleState {
    let Some(target) = subject.target else {
        return state;
    };

    let state = with_damaged_target(state, target, magic_missile_force_damage(dart_pips_sum));
    let state = expend_combatant_spell_slot(state, subject.actor, BattleSpellSlotLevel::First);
    let state = commit_actor_spell_slot_use(state, subject.actor);
    BattleState {
        action_available: false,
        slot_spell_procedure: BattleSlotSpellProcedure::Active(BattleSlotSpellSubject {
            stage: BattleSlotSpellStage::Resolved,
            ..subject
        }),
        ..state
    }
}

fn armor_class_for(state: &BattleState, actor: Actor) -> i16 {
    let combatant = combatant_for(state, actor);
    let base_armor_class = match combatant.spell_active_effects.armor_class_base_effect {
        BattleArmorClassBaseEffect::None => combatant.armor_class,
        BattleArmorClassBaseEffect::Active {
            base_armor_class,
            dexterity_modifier,
            ..
        } => {
            let projected = armor_class_projection(
                ArmorClassOption::DefaultUnarmored,
                ArmorClassFacts {
                    dexterity_modifier,
                    constitution_modifier: 0,
                    wisdom_modifier: 0,
                    charisma_modifier: 0,
                    formula: ArmorClassFormula::AbilitySum {
                        base: base_armor_class,
                        abilities: vec![ArmorClassAbility::Dexterity],
                    },
                    shield_bonus: None,
                },
            );
            i16::from(projected.armor_class)
        }
    };
    let shield_bonus = if combatant.shield_armor_class_bonus_active {
        5
    } else {
        0
    };
    base_armor_class + shield_bonus
}

fn apply_reaction_armor_class_bonus(
    mut state: BattleState,
    actor: Actor,
    armor_class_bonus: i16,
) -> BattleState {
    if armor_class_bonus <= 0 {
        return state;
    }
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

fn with_healed_target(mut state: BattleState, target: Actor, healing: i16) -> BattleState {
    let target_combatant = combatant_for(&state, target);
    if !ordinary_zero_hit_point_restoration_target(target_combatant) {
        return state;
    }
    let new_hp = (target_combatant.hp + healing.max(0)).min(target_combatant.max_hp);
    let healed = if new_hp > 0 {
        Combatant {
            hp: new_hp,
            unconscious: false,
            incapacitated: false,
            prone: false,
            lifecycle: healed_lifecycle(target_combatant.lifecycle),
            ..target_combatant
        }
    } else {
        Combatant {
            hp: new_hp,
            ..target_combatant
        }
    };
    match target {
        Actor::Fighter => state.fighter = healed,
        Actor::Goblin => state.goblin = healed,
        Actor::Rogue => state.rogue = healed,
        Actor::Skeleton => state.skeleton = healed,
    }
    state
}

fn zero_hit_point_lifecycle_cleared(combatant: Combatant) -> bool {
    combatant.hp > 0 && !combatant.unconscious && !combatant.incapacitated && !combatant.prone
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
            CombatantLifecycle::UsesDeathSavingThrows(_) => CreatureKind::PlayerCharacter,
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
        lifecycle: with_lifecycle_dead(combatant.lifecycle, vitals.dead),
        ..combatant
    }
}

fn healed_lifecycle(lifecycle: CombatantLifecycle) -> CombatantLifecycle {
    match lifecycle {
        CombatantLifecycle::UsesDeathSavingThrows(_) => {
            CombatantLifecycle::UsesDeathSavingThrows(DeathSavingThrowLifecycle::fresh())
        }
        CombatantLifecycle::DiesAtZeroHitPoints => CombatantLifecycle::DiesAtZeroHitPoints,
    }
}

fn with_lifecycle_dead(lifecycle: CombatantLifecycle, dead: bool) -> CombatantLifecycle {
    match lifecycle {
        CombatantLifecycle::UsesDeathSavingThrows(_) if dead => {
            CombatantLifecycle::UsesDeathSavingThrows(DeathSavingThrowLifecycle::Dead)
        }
        CombatantLifecycle::UsesDeathSavingThrows(death_save) => {
            CombatantLifecycle::UsesDeathSavingThrows(death_save)
        }
        CombatantLifecycle::DiesAtZeroHitPoints => CombatantLifecycle::DiesAtZeroHitPoints,
    }
}

fn death_saving_throw_successes(lifecycle: DeathSavingThrowLifecycle) -> i16 {
    match lifecycle {
        DeathSavingThrowLifecycle::MakingDeathSavingThrows { successes, .. } => successes.as_i16(),
        DeathSavingThrowLifecycle::Stable | DeathSavingThrowLifecycle::Dead => 0,
    }
}

fn death_saving_throw_failures(lifecycle: DeathSavingThrowLifecycle) -> i16 {
    match lifecycle {
        DeathSavingThrowLifecycle::MakingDeathSavingThrows { failures, .. } => failures.as_i16(),
        DeathSavingThrowLifecycle::Stable => 0,
        DeathSavingThrowLifecycle::Dead => 3,
    }
}
