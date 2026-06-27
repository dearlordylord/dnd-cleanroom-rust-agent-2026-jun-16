use crate::rules::battle_reducer_spine::{
    BattleEntrypointTrace, BattleHoleKind, BattleReducerRouteEvent, BattleReducerRouteFillKind,
    BattleReducerRouteHoleKind, BattleReducerRouteOwnerGroup, BattleReducerRouteSubjectFamily,
    BattleReducerRouteTrace, BattleResolutionInvalidReason, BattleResolutionOutcome,
    BattleResolutionResult, BattleSetup, BattleState, BattleSubject,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum ReducerRouteHoleKind {
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
pub enum ReducerRouteFillKind {
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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ReducerRouteResolveFill {
    Fill(ReducerRouteFillKind),
    WithoutFill,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ReducerRouteSubjectFamily {
    ActiveFormLifecycle,
    ConcentrationTeardown,
    CommandSpell,
    CreatureAttack,
    BattleAction,
    DeathSavingThrow,
    ForcedMovement,
    HitPointRestoration,
    MovementResource,
    RepeatSaveConditionEffect,
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
    SpellDamageReduction,
    CompanionLifecycle,
    CompanionSharedSenses,
    CompanionTouchDelivery,
    CompanionReactionAttack,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ReducerRouteOwnerGroup {
    ActionEconomy,
    ActiveEffect,
    AbilityCheckRollMode,
    AreaShape,
    AttackRoll,
    AttackActionProcedure,
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
    SavingThrowOutcome,
    SavingThrowRollMode,
    SpellAttackProcedure,
    SpellSlotAndActionEconomy,
    StatBlockAction,
    TargetSelection,
    TemporaryHitPoint,
    TurnBoundary,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ReducerRouteResolutionOutcome {
    NeedsHoles,
    Resolved,
    Invalid(BattleResolutionInvalidReason),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ReducerRouteResolveConnector {
    pub subject: ReducerRouteSubjectFamily,
    pub fill: ReducerRouteResolveFill,
    pub owner: ReducerRouteOwnerGroup,
}

#[derive(Debug, Clone, Eq)]
pub enum ReducerRouteEvent {
    StartBattle {
        owner: ReducerRouteOwnerGroup,
    },
    DiscoverBattleActs {
        subject: ReducerRouteSubjectFamily,
        holes: Vec<ReducerRouteHoleKind>,
        owner: ReducerRouteOwnerGroup,
    },
    ResolveBattleSubject {
        subject: ReducerRouteSubjectFamily,
        fill: ReducerRouteFillKind,
        outcome: ReducerRouteResolutionOutcome,
        holes: Vec<ReducerRouteHoleKind>,
        owner: ReducerRouteOwnerGroup,
    },
    ResolveBattleSubjectWithoutFill {
        subject: ReducerRouteSubjectFamily,
        outcome: ReducerRouteResolutionOutcome,
        holes: Vec<ReducerRouteHoleKind>,
        owner: ReducerRouteOwnerGroup,
    },
}

impl PartialEq for ReducerRouteEvent {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::StartBattle { owner: left_owner }, Self::StartBattle { owner: right_owner }) => {
                left_owner == right_owner
            }
            (
                Self::DiscoverBattleActs {
                    subject: left_subject,
                    holes: left_holes,
                    owner: left_owner,
                },
                Self::DiscoverBattleActs {
                    subject: right_subject,
                    holes: right_holes,
                    owner: right_owner,
                },
            ) => {
                left_subject == right_subject
                    && left_holes == right_holes
                    && left_owner == right_owner
            }
            (
                Self::ResolveBattleSubject {
                    subject: left_subject,
                    fill: left_fill,
                    outcome: left_outcome,
                    holes: left_holes,
                    owner: left_owner,
                },
                Self::ResolveBattleSubject {
                    subject: right_subject,
                    fill: right_fill,
                    outcome: right_outcome,
                    holes: right_holes,
                    owner: right_owner,
                },
            ) => {
                left_subject == right_subject
                    && left_fill == right_fill
                    && left_outcome == right_outcome
                    && left_holes == right_holes
                    && left_owner == right_owner
            }
            (
                Self::ResolveBattleSubjectWithoutFill {
                    subject: left_subject,
                    outcome: left_outcome,
                    holes: left_holes,
                    owner: left_owner,
                },
                Self::ResolveBattleSubjectWithoutFill {
                    subject: right_subject,
                    outcome: right_outcome,
                    holes: right_holes,
                    owner: right_owner,
                },
            ) => {
                left_subject == right_subject
                    && left_outcome == right_outcome
                    && left_holes == right_holes
                    && left_owner == right_owner
            }
            _ => false,
        }
    }
}

#[must_use]
pub fn route_start_battle(owner: ReducerRouteOwnerGroup) -> ReducerRouteEvent {
    ReducerRouteEvent::StartBattle { owner }
}

#[must_use]
pub fn route_discover_battle_acts(
    subject: ReducerRouteSubjectFamily,
    holes: Vec<BattleHoleKind>,
    owner: ReducerRouteOwnerGroup,
) -> ReducerRouteEvent {
    route_discover_battle_acts_from_route_holes(subject, reducer_route_holes(holes), owner)
}

#[must_use]
pub fn route_discover_battle_acts_from_route_holes(
    subject: ReducerRouteSubjectFamily,
    holes: Vec<ReducerRouteHoleKind>,
    owner: ReducerRouteOwnerGroup,
) -> ReducerRouteEvent {
    ReducerRouteEvent::DiscoverBattleActs {
        subject,
        holes: sorted_route_holes(holes),
        owner,
    }
}

#[must_use]
pub fn route_discover_battle_acts_from_result(
    subject: ReducerRouteSubjectFamily,
    result: &BattleResolutionResult,
    owner: ReducerRouteOwnerGroup,
) -> ReducerRouteEvent {
    route_discover_battle_acts_from_route_holes(subject, reducer_route_result_holes(result), owner)
}

#[must_use]
pub fn route_resolve_battle_subject(
    subject: ReducerRouteSubjectFamily,
    fill: ReducerRouteFillKind,
    holes: Vec<BattleHoleKind>,
    owner: ReducerRouteOwnerGroup,
) -> ReducerRouteEvent {
    route_resolve_battle_subject_from_route_holes(subject, fill, reducer_route_holes(holes), owner)
}

#[must_use]
pub fn route_resolve_battle_subject_from_route_holes(
    subject: ReducerRouteSubjectFamily,
    fill: ReducerRouteFillKind,
    holes: Vec<ReducerRouteHoleKind>,
    owner: ReducerRouteOwnerGroup,
) -> ReducerRouteEvent {
    route_resolve_battle_subject_from_route_result(
        subject,
        fill,
        route_outcome_for_holes(&holes),
        holes,
        owner,
    )
}

#[must_use]
pub fn route_resolve_battle_subject_from_route_result(
    subject: ReducerRouteSubjectFamily,
    fill: ReducerRouteFillKind,
    outcome: ReducerRouteResolutionOutcome,
    holes: Vec<ReducerRouteHoleKind>,
    owner: ReducerRouteOwnerGroup,
) -> ReducerRouteEvent {
    ReducerRouteEvent::ResolveBattleSubject {
        subject,
        fill,
        outcome,
        holes: sorted_route_holes(holes),
        owner,
    }
}

#[must_use]
pub fn route_resolve_battle_subject_without_fill(
    subject: ReducerRouteSubjectFamily,
    holes: Vec<BattleHoleKind>,
    owner: ReducerRouteOwnerGroup,
) -> ReducerRouteEvent {
    route_resolve_battle_subject_without_fill_from_route_holes(
        subject,
        reducer_route_holes(holes),
        owner,
    )
}

#[must_use]
pub fn route_resolve_battle_subject_without_fill_from_route_holes(
    subject: ReducerRouteSubjectFamily,
    holes: Vec<ReducerRouteHoleKind>,
    owner: ReducerRouteOwnerGroup,
) -> ReducerRouteEvent {
    route_resolve_battle_subject_without_fill_from_route_result(
        subject,
        route_outcome_for_holes(&holes),
        holes,
        owner,
    )
}

#[must_use]
pub fn route_resolve_battle_subject_without_fill_from_route_result(
    subject: ReducerRouteSubjectFamily,
    outcome: ReducerRouteResolutionOutcome,
    holes: Vec<ReducerRouteHoleKind>,
    owner: ReducerRouteOwnerGroup,
) -> ReducerRouteEvent {
    ReducerRouteEvent::ResolveBattleSubjectWithoutFill {
        subject,
        outcome,
        holes: sorted_route_holes(holes),
        owner,
    }
}

#[must_use]
pub fn route_resolve_battle_subject_from_result(
    connector: ReducerRouteResolveConnector,
    result: &BattleResolutionResult,
) -> ReducerRouteEvent {
    let holes = reducer_route_result_holes(result);
    route_resolve_battle_subject_from_result_holes(connector, reducer_route_outcome(result), holes)
}

fn route_resolve_battle_subject_from_result_holes(
    connector: ReducerRouteResolveConnector,
    outcome: ReducerRouteResolutionOutcome,
    holes: Vec<ReducerRouteHoleKind>,
) -> ReducerRouteEvent {
    match connector.fill {
        ReducerRouteResolveFill::Fill(fill) => ReducerRouteEvent::ResolveBattleSubject {
            subject: connector.subject,
            fill,
            outcome,
            holes,
            owner: connector.owner,
        },
        ReducerRouteResolveFill::WithoutFill => {
            ReducerRouteEvent::ResolveBattleSubjectWithoutFill {
                subject: connector.subject,
                outcome,
                holes,
                owner: connector.owner,
            }
        }
    }
}

#[must_use]
pub fn battle_resolution_continuation(
    result: BattleResolutionResult,
    context: &str,
) -> (BattleState, BattleSubject) {
    let outcome = result.outcome();
    let continuing_subject = result
        .continuing_subject()
        .unwrap_or_else(|| panic!("{context} should continue from NeedsHoles, got {outcome:?}"));
    let needs_holes = result
        .into_needs_holes()
        .unwrap_or_else(|| panic!("{context} should continue from NeedsHoles, got {outcome:?}"));
    assert_eq!(
        needs_holes.subject, continuing_subject,
        "{context} continuing_subject() and into_needs_holes() disagreed"
    );
    (needs_holes.state, needs_holes.subject)
}

#[must_use]
pub fn reducer_route_payload(route: &[ReducerRouteEvent]) -> String {
    route
        .iter()
        .map(route_event_ref)
        .collect::<Vec<_>>()
        .join("\n")
}

#[must_use]
pub fn reducer_route_events_from_battle_trace(
    trace: &BattleReducerRouteTrace,
) -> Vec<ReducerRouteEvent> {
    trace
        .events()
        .iter()
        .map(reducer_route_event_from_observed)
        .collect()
}

#[must_use]
pub fn reducer_route_event_from_observed(event: &BattleReducerRouteEvent) -> ReducerRouteEvent {
    match event {
        BattleReducerRouteEvent::StartBattle { owner } => ReducerRouteEvent::StartBattle {
            owner: reducer_route_owner(*owner),
        },
        BattleReducerRouteEvent::DiscoverBattleActs {
            subject,
            holes,
            owner,
        } => ReducerRouteEvent::DiscoverBattleActs {
            subject: reducer_route_subject(*subject),
            holes: reducer_route_holes_from_route_slice(holes),
            owner: reducer_route_owner(*owner),
        },
        BattleReducerRouteEvent::ResolveBattleSubject {
            subject,
            fill,
            outcome,
            holes,
            owner,
        } => ReducerRouteEvent::ResolveBattleSubject {
            subject: reducer_route_subject(*subject),
            fill: reducer_route_fill(*fill),
            outcome: reducer_route_outcome_from_battle(*outcome),
            holes: reducer_route_holes_from_route_slice(holes),
            owner: reducer_route_owner(*owner),
        },
        BattleReducerRouteEvent::ResolveBattleSubjectWithoutFill {
            subject,
            outcome,
            holes,
            owner,
        } => ReducerRouteEvent::ResolveBattleSubjectWithoutFill {
            subject: reducer_route_subject(*subject),
            outcome: reducer_route_outcome_from_battle(*outcome),
            holes: reducer_route_holes_from_route_slice(holes),
            owner: reducer_route_owner(*owner),
        },
    }
}

#[must_use]
pub fn observed_reducer_route(
    trace: &BattleEntrypointTrace,
    subjects: &[ReducerRouteSubjectFamily],
) -> Vec<ReducerRouteEvent> {
    trace
        .route_events()
        .iter()
        .map(reducer_route_event_from_observed)
        .filter(|event| route_event_matches_subjects(event, subjects))
        .collect()
}

#[must_use]
pub fn setup_from_battle_state(state: BattleState) -> BattleSetup {
    BattleSetup {
        initiative: state.initiative,
        fighter: state.fighter,
        goblin: state.goblin,
        rogue: state.rogue,
        skeleton: state.skeleton,
        stat_block_control: state.stat_block_control,
        turn_boundary_effects: state.turn_boundary_effects,
        interrupt_resume: state.interrupt_resume,
        reaction_casting_time: state.reaction_casting_time,
        concentration: state.concentration,
        slot_spell_procedure: state.slot_spell_procedure,
        save_gated_spell_procedure: state.save_gated_spell_procedure,
        hit_point_restoration_procedure: state.hit_point_restoration_procedure,
        spell_attack_procedure: state.spell_attack_procedure,
        command_effect_procedure: state.command_effect_procedure,
        feature_substrates: state.feature_substrates,
        feature_resources: state.feature_resources,
        spell_slot_uses_this_turn: state.spell_slot_uses_this_turn,
        level_one_plus_spell_casters_this_turn: state.level_one_plus_spell_casters_this_turn,
        quickened_level_one_plus_spell_casters_this_turn: state
            .quickened_level_one_plus_spell_casters_this_turn,
        action_available: state.action_available,
        bonus_action_available: state.bonus_action_available,
        attack_roll_made_this_turn: state.attack_roll_made_this_turn,
        dash_movement_bonus_feet: state.dash_movement_bonus_feet,
        disengaged: state.disengaged,
    }
}

fn route_event_matches_subjects(
    event: &ReducerRouteEvent,
    subjects: &[ReducerRouteSubjectFamily],
) -> bool {
    match event {
        ReducerRouteEvent::StartBattle { .. } => true,
        ReducerRouteEvent::DiscoverBattleActs { subject, .. }
        | ReducerRouteEvent::ResolveBattleSubject { subject, .. }
        | ReducerRouteEvent::ResolveBattleSubjectWithoutFill { subject, .. } => {
            subjects.contains(subject)
        }
    }
}

fn reducer_route_result_holes(result: &BattleResolutionResult) -> Vec<ReducerRouteHoleKind> {
    let holes = match result.outcome() {
        BattleResolutionOutcome::NeedsHoles | BattleResolutionOutcome::Invalid(_) => result
            .requested_holes()
            .expect("non-resolved battle resolution result should carry requested holes"),
        BattleResolutionOutcome::Resolved => &[],
    };
    reducer_route_holes_from_slice(holes)
}

const fn reducer_route_outcome_from_battle(
    outcome: BattleResolutionOutcome,
) -> ReducerRouteResolutionOutcome {
    match outcome {
        BattleResolutionOutcome::NeedsHoles => ReducerRouteResolutionOutcome::NeedsHoles,
        BattleResolutionOutcome::Resolved => ReducerRouteResolutionOutcome::Resolved,
        BattleResolutionOutcome::Invalid(reason) => ReducerRouteResolutionOutcome::Invalid(reason),
    }
}

const fn reducer_route_subject(
    subject: BattleReducerRouteSubjectFamily,
) -> ReducerRouteSubjectFamily {
    match subject {
        BattleReducerRouteSubjectFamily::ActiveFormLifecycle => {
            ReducerRouteSubjectFamily::ActiveFormLifecycle
        }
        BattleReducerRouteSubjectFamily::ConcentrationTeardown => {
            ReducerRouteSubjectFamily::ConcentrationTeardown
        }
        BattleReducerRouteSubjectFamily::CommandSpell => ReducerRouteSubjectFamily::CommandSpell,
        BattleReducerRouteSubjectFamily::CreatureAttack => {
            ReducerRouteSubjectFamily::CreatureAttack
        }
        BattleReducerRouteSubjectFamily::BattleAction => ReducerRouteSubjectFamily::BattleAction,
        BattleReducerRouteSubjectFamily::DeathSavingThrow => {
            ReducerRouteSubjectFamily::DeathSavingThrow
        }
        BattleReducerRouteSubjectFamily::ForcedMovement => {
            ReducerRouteSubjectFamily::ForcedMovement
        }
        BattleReducerRouteSubjectFamily::HitPointRestoration => {
            ReducerRouteSubjectFamily::HitPointRestoration
        }
        BattleReducerRouteSubjectFamily::MovementResource => {
            ReducerRouteSubjectFamily::MovementResource
        }
        BattleReducerRouteSubjectFamily::SaveGatedSpell => {
            ReducerRouteSubjectFamily::SaveGatedSpell
        }
        BattleReducerRouteSubjectFamily::SlotSpell => ReducerRouteSubjectFamily::SlotSpell,
        BattleReducerRouteSubjectFamily::SpellAttack => ReducerRouteSubjectFamily::SpellAttack,
        BattleReducerRouteSubjectFamily::ScalarBuff => ReducerRouteSubjectFamily::ScalarBuff,
        BattleReducerRouteSubjectFamily::StatBlockAction => {
            ReducerRouteSubjectFamily::StatBlockAction
        }
        BattleReducerRouteSubjectFamily::WeaponAttack => ReducerRouteSubjectFamily::WeaponAttack,
        BattleReducerRouteSubjectFamily::ActiveFeatureSpellAttackRollMode => {
            ReducerRouteSubjectFamily::ActiveFeatureSpellAttackRollMode
        }
        BattleReducerRouteSubjectFamily::ActiveFeatureSpellSaveDc => {
            ReducerRouteSubjectFamily::ActiveFeatureSpellSaveDc
        }
        BattleReducerRouteSubjectFamily::AttackActionAreaSaveDamageReplacement => {
            ReducerRouteSubjectFamily::AttackActionAreaSaveDamageReplacement
        }
        BattleReducerRouteSubjectFamily::MetamagicOptionSpell => {
            ReducerRouteSubjectFamily::MetamagicOptionSpell
        }
        BattleReducerRouteSubjectFamily::UnitFeatureBonusAction => {
            ReducerRouteSubjectFamily::UnitFeatureBonusAction
        }
        BattleReducerRouteSubjectFamily::WeaponMasteryProperty => {
            ReducerRouteSubjectFamily::WeaponMasteryProperty
        }
        BattleReducerRouteSubjectFamily::ZeroHitPointStabilization => {
            ReducerRouteSubjectFamily::ZeroHitPointStabilization
        }
        BattleReducerRouteSubjectFamily::PassiveAbilityCheckRollMode => {
            ReducerRouteSubjectFamily::PassiveAbilityCheckRollMode
        }
        BattleReducerRouteSubjectFamily::PassiveDamageAdjustment => {
            ReducerRouteSubjectFamily::PassiveDamageAdjustment
        }
        BattleReducerRouteSubjectFamily::PassiveSavingThrowRollMode => {
            ReducerRouteSubjectFamily::PassiveSavingThrowRollMode
        }
        BattleReducerRouteSubjectFamily::CreatureSpaceMovementPermission => {
            ReducerRouteSubjectFamily::CreatureSpaceMovementPermission
        }
        BattleReducerRouteSubjectFamily::CreatureStatProjection => {
            ReducerRouteSubjectFamily::CreatureStatProjection
        }
        BattleReducerRouteSubjectFamily::RollModifierEffect => {
            ReducerRouteSubjectFamily::RollModifierEffect
        }
        BattleReducerRouteSubjectFamily::ScalarBuffEffect => {
            ReducerRouteSubjectFamily::ScalarBuffEffect
        }
        BattleReducerRouteSubjectFamily::SpellDamageReduction => {
            ReducerRouteSubjectFamily::SpellDamageReduction
        }
        BattleReducerRouteSubjectFamily::CompanionLifecycle => {
            ReducerRouteSubjectFamily::CompanionLifecycle
        }
        BattleReducerRouteSubjectFamily::CompanionSharedSenses => {
            ReducerRouteSubjectFamily::CompanionSharedSenses
        }
        BattleReducerRouteSubjectFamily::CompanionTouchDelivery => {
            ReducerRouteSubjectFamily::CompanionTouchDelivery
        }
        BattleReducerRouteSubjectFamily::CompanionReactionAttack => {
            ReducerRouteSubjectFamily::CompanionReactionAttack
        }
    }
}

const fn reducer_route_fill(fill: BattleReducerRouteFillKind) -> ReducerRouteFillKind {
    match fill {
        BattleReducerRouteFillKind::AttackRoll => ReducerRouteFillKind::AttackRoll,
        BattleReducerRouteFillKind::GrappleOutcome => ReducerRouteFillKind::GrappleOutcome,
        BattleReducerRouteFillKind::ConcentrationSavingThrow => {
            ReducerRouteFillKind::ConcentrationSavingThrow
        }
        BattleReducerRouteFillKind::ConditionChoice => ReducerRouteFillKind::ConditionChoice,
        BattleReducerRouteFillKind::DamageTypeChoice => ReducerRouteFillKind::DamageTypeChoice,
        BattleReducerRouteFillKind::DeathSavingThrow => ReducerRouteFillKind::DeathSavingThrow,
        BattleReducerRouteFillKind::HitPointHealingDistribution => {
            ReducerRouteFillKind::HitPointHealingDistribution
        }
        BattleReducerRouteFillKind::CommandOptionChoice => {
            ReducerRouteFillKind::CommandOptionChoice
        }
        BattleReducerRouteFillKind::Movement => ReducerRouteFillKind::Movement,
        BattleReducerRouteFillKind::RolledDice => ReducerRouteFillKind::RolledDice,
        BattleReducerRouteFillKind::SavingThrowOutcome => ReducerRouteFillKind::SavingThrowOutcome,
        BattleReducerRouteFillKind::SkillChoice => ReducerRouteFillKind::SkillChoice,
        BattleReducerRouteFillKind::SpellTargetAllocation => {
            ReducerRouteFillKind::SpellTargetAllocation
        }
        BattleReducerRouteFillKind::SpellTargetList => ReducerRouteFillKind::SpellTargetList,
        BattleReducerRouteFillKind::StatBlockRechargeRoll => {
            ReducerRouteFillKind::StatBlockRechargeRoll
        }
        BattleReducerRouteFillKind::TargetChoice => ReducerRouteFillKind::TargetChoice,
        BattleReducerRouteFillKind::UnitFeatureDecision => {
            ReducerRouteFillKind::UnitFeatureDecision
        }
        BattleReducerRouteFillKind::WildShapeEquipmentDisposition => {
            ReducerRouteFillKind::WildShapeEquipmentDisposition
        }
    }
}

const fn reducer_route_owner(owner: BattleReducerRouteOwnerGroup) -> ReducerRouteOwnerGroup {
    match owner {
        BattleReducerRouteOwnerGroup::ActionEconomy => ReducerRouteOwnerGroup::ActionEconomy,
        BattleReducerRouteOwnerGroup::ActiveEffect => ReducerRouteOwnerGroup::ActiveEffect,
        BattleReducerRouteOwnerGroup::AbilityCheckRollMode => {
            ReducerRouteOwnerGroup::AbilityCheckRollMode
        }
        BattleReducerRouteOwnerGroup::AreaShape => ReducerRouteOwnerGroup::AreaShape,
        BattleReducerRouteOwnerGroup::AttackRoll => ReducerRouteOwnerGroup::AttackRoll,
        BattleReducerRouteOwnerGroup::AttackActionProcedure => {
            ReducerRouteOwnerGroup::AttackActionProcedure
        }
        BattleReducerRouteOwnerGroup::Concentration => ReducerRouteOwnerGroup::Concentration,
        BattleReducerRouteOwnerGroup::ConditionLifecycle => {
            ReducerRouteOwnerGroup::ConditionLifecycle
        }
        BattleReducerRouteOwnerGroup::Companion => ReducerRouteOwnerGroup::Companion,
        BattleReducerRouteOwnerGroup::CreatureSpaceMovement => {
            ReducerRouteOwnerGroup::CreatureSpaceMovement
        }
        BattleReducerRouteOwnerGroup::CreatureState => ReducerRouteOwnerGroup::CreatureState,
        BattleReducerRouteOwnerGroup::DamageRoll => ReducerRouteOwnerGroup::DamageRoll,
        BattleReducerRouteOwnerGroup::DamageAdjustment => ReducerRouteOwnerGroup::DamageAdjustment,
        BattleReducerRouteOwnerGroup::DamageType => ReducerRouteOwnerGroup::DamageType,
        BattleReducerRouteOwnerGroup::FeatureResource => ReducerRouteOwnerGroup::FeatureResource,
        BattleReducerRouteOwnerGroup::HitPointAndZeroHpLifecycle => {
            ReducerRouteOwnerGroup::HitPointAndZeroHpLifecycle
        }
        BattleReducerRouteOwnerGroup::HitPoint => ReducerRouteOwnerGroup::HitPoint,
        BattleReducerRouteOwnerGroup::HoleFrontier => ReducerRouteOwnerGroup::HoleFrontier,
        BattleReducerRouteOwnerGroup::InterruptStack => ReducerRouteOwnerGroup::InterruptStack,
        BattleReducerRouteOwnerGroup::MovementResource => ReducerRouteOwnerGroup::MovementResource,
        BattleReducerRouteOwnerGroup::ObjectBoundary => ReducerRouteOwnerGroup::ObjectBoundary,
        BattleReducerRouteOwnerGroup::SavingThrowOutcome => {
            ReducerRouteOwnerGroup::SavingThrowOutcome
        }
        BattleReducerRouteOwnerGroup::SavingThrowRollMode => {
            ReducerRouteOwnerGroup::SavingThrowRollMode
        }
        BattleReducerRouteOwnerGroup::SpellAttackProcedure => {
            ReducerRouteOwnerGroup::SpellAttackProcedure
        }
        BattleReducerRouteOwnerGroup::SpellSlotAndActionEconomy => {
            ReducerRouteOwnerGroup::SpellSlotAndActionEconomy
        }
        BattleReducerRouteOwnerGroup::StatBlockAction => ReducerRouteOwnerGroup::StatBlockAction,
        BattleReducerRouteOwnerGroup::TargetSelection => ReducerRouteOwnerGroup::TargetSelection,
        BattleReducerRouteOwnerGroup::TemporaryHitPoint => {
            ReducerRouteOwnerGroup::TemporaryHitPoint
        }
        BattleReducerRouteOwnerGroup::TurnBoundary => ReducerRouteOwnerGroup::TurnBoundary,
    }
}

const fn reducer_route_outcome(result: &BattleResolutionResult) -> ReducerRouteResolutionOutcome {
    match result.outcome() {
        BattleResolutionOutcome::NeedsHoles => ReducerRouteResolutionOutcome::NeedsHoles,
        BattleResolutionOutcome::Resolved => ReducerRouteResolutionOutcome::Resolved,
        BattleResolutionOutcome::Invalid(reason) => ReducerRouteResolutionOutcome::Invalid(reason),
    }
}

fn route_outcome_for_holes(holes: &[ReducerRouteHoleKind]) -> ReducerRouteResolutionOutcome {
    if holes.is_empty() {
        ReducerRouteResolutionOutcome::Resolved
    } else {
        ReducerRouteResolutionOutcome::NeedsHoles
    }
}

fn reducer_route_holes(holes: Vec<BattleHoleKind>) -> Vec<ReducerRouteHoleKind> {
    reducer_route_holes_from_slice(&holes)
}

fn reducer_route_holes_from_slice(holes: &[BattleHoleKind]) -> Vec<ReducerRouteHoleKind> {
    sorted_route_holes(
        holes
            .iter()
            .copied()
            .map(reducer_route_hole)
            .collect::<Vec<_>>(),
    )
}

fn reducer_route_holes_from_route_slice(
    holes: &[BattleReducerRouteHoleKind],
) -> Vec<ReducerRouteHoleKind> {
    sorted_route_holes(
        holes
            .iter()
            .copied()
            .map(reducer_route_route_hole)
            .collect::<Vec<_>>(),
    )
}

fn sorted_route_holes(mut route_holes: Vec<ReducerRouteHoleKind>) -> Vec<ReducerRouteHoleKind> {
    route_holes.sort();
    route_holes
}

fn reducer_route_hole(hole: BattleHoleKind) -> ReducerRouteHoleKind {
    match hole {
        BattleHoleKind::ConcentrationSavingThrow => ReducerRouteHoleKind::ConcentrationSavingThrow,
        BattleHoleKind::ConditionChoice => ReducerRouteHoleKind::ConditionChoice,
        BattleHoleKind::DeathSavingThrow => ReducerRouteHoleKind::DeathSavingThrow,
        BattleHoleKind::DamageTypeChoice => ReducerRouteHoleKind::DamageTypeChoice,
        BattleHoleKind::HitPointHealingDistribution => {
            ReducerRouteHoleKind::HitPointHealingDistribution
        }
        BattleHoleKind::InterruptDecision => ReducerRouteHoleKind::InterruptDecision,
        BattleHoleKind::CommandOptionChoice => ReducerRouteHoleKind::CommandOptionChoice,
        BattleHoleKind::Movement => ReducerRouteHoleKind::Movement,
        BattleHoleKind::RolledDice => ReducerRouteHoleKind::RolledDice,
        BattleHoleKind::SavingThrowOutcome => ReducerRouteHoleKind::SavingThrowOutcome,
        BattleHoleKind::SpellTargetAllocation => ReducerRouteHoleKind::SpellTargetAllocation,
        BattleHoleKind::SpellTargetList => ReducerRouteHoleKind::SpellTargetList,
        BattleHoleKind::StatBlockRechargeRoll => ReducerRouteHoleKind::StatBlockRechargeRoll,
        BattleHoleKind::TargetChoice => ReducerRouteHoleKind::TargetChoice,
        BattleHoleKind::AttackRoll => ReducerRouteHoleKind::AttackRoll,
    }
}

const fn reducer_route_route_hole(hole: BattleReducerRouteHoleKind) -> ReducerRouteHoleKind {
    match hole {
        BattleReducerRouteHoleKind::AttackRoll => ReducerRouteHoleKind::AttackRoll,
        BattleReducerRouteHoleKind::GrappleOutcome => ReducerRouteHoleKind::GrappleOutcome,
        BattleReducerRouteHoleKind::ConcentrationSavingThrow => {
            ReducerRouteHoleKind::ConcentrationSavingThrow
        }
        BattleReducerRouteHoleKind::ConditionChoice => ReducerRouteHoleKind::ConditionChoice,
        BattleReducerRouteHoleKind::DamageTypeChoice => ReducerRouteHoleKind::DamageTypeChoice,
        BattleReducerRouteHoleKind::DeathSavingThrow => ReducerRouteHoleKind::DeathSavingThrow,
        BattleReducerRouteHoleKind::HitPointHealingDistribution => {
            ReducerRouteHoleKind::HitPointHealingDistribution
        }
        BattleReducerRouteHoleKind::InterruptDecision => ReducerRouteHoleKind::InterruptDecision,
        BattleReducerRouteHoleKind::CommandOptionChoice => {
            ReducerRouteHoleKind::CommandOptionChoice
        }
        BattleReducerRouteHoleKind::Movement => ReducerRouteHoleKind::Movement,
        BattleReducerRouteHoleKind::RolledDice => ReducerRouteHoleKind::RolledDice,
        BattleReducerRouteHoleKind::SavingThrowOutcome => ReducerRouteHoleKind::SavingThrowOutcome,
        BattleReducerRouteHoleKind::SkillChoice => ReducerRouteHoleKind::SkillChoice,
        BattleReducerRouteHoleKind::SpellTargetAllocation => {
            ReducerRouteHoleKind::SpellTargetAllocation
        }
        BattleReducerRouteHoleKind::SpellTargetList => ReducerRouteHoleKind::SpellTargetList,
        BattleReducerRouteHoleKind::StatBlockRechargeRoll => {
            ReducerRouteHoleKind::StatBlockRechargeRoll
        }
        BattleReducerRouteHoleKind::TargetChoice => ReducerRouteHoleKind::TargetChoice,
        BattleReducerRouteHoleKind::WildShapeEquipmentDisposition => {
            ReducerRouteHoleKind::WildShapeEquipmentDisposition
        }
    }
}

fn route_event_ref(event: &ReducerRouteEvent) -> String {
    match event {
        ReducerRouteEvent::StartBattle { owner } => {
            format!("start_battle owner={}", owner_ref(*owner))
        }
        ReducerRouteEvent::DiscoverBattleActs {
            subject,
            holes,
            owner,
        } => format!(
            "discover_battle_acts subject={} holes={} owner={}",
            subject_ref(*subject),
            joined_holes(holes),
            owner_ref(*owner)
        ),
        ReducerRouteEvent::ResolveBattleSubject {
            subject,
            fill,
            outcome,
            holes,
            owner,
        } => format!(
            "resolve_battle_subject subject={} fill={} outcome={} holes={} owner={}",
            subject_ref(*subject),
            fill_ref(*fill),
            outcome_ref(*outcome),
            joined_holes(holes),
            owner_ref(*owner)
        ),
        ReducerRouteEvent::ResolveBattleSubjectWithoutFill {
            subject,
            outcome,
            holes,
            owner,
        } => format!(
            "resolve_battle_subject_without_fill subject={} outcome={} holes={} owner={}",
            subject_ref(*subject),
            outcome_ref(*outcome),
            joined_holes(holes),
            owner_ref(*owner)
        ),
    }
}

fn joined_holes(holes: &[ReducerRouteHoleKind]) -> String {
    if holes.is_empty() {
        return "none".to_string();
    }
    holes
        .iter()
        .map(|hole| hole_ref(*hole))
        .collect::<Vec<_>>()
        .join(",")
}

fn hole_ref(hole: ReducerRouteHoleKind) -> &'static str {
    match hole {
        ReducerRouteHoleKind::AttackRoll => "AttackRollHoleKind",
        ReducerRouteHoleKind::GrappleOutcome => "GrappleOutcomeHoleKind",
        ReducerRouteHoleKind::ConcentrationSavingThrow => "ConcentrationSavingThrowHoleKind",
        ReducerRouteHoleKind::ConditionChoice => "ConditionChoiceHoleKind",
        ReducerRouteHoleKind::DamageTypeChoice => "DamageTypeChoiceHoleKind",
        ReducerRouteHoleKind::DeathSavingThrow => "DeathSavingThrowHoleKind",
        ReducerRouteHoleKind::HitPointHealingDistribution => "HitPointHealingDistributionHoleKind",
        ReducerRouteHoleKind::InterruptDecision => "InterruptDecisionHoleKind",
        ReducerRouteHoleKind::CommandOptionChoice => "CommandOptionChoiceHoleKind",
        ReducerRouteHoleKind::Movement => "MovementHoleKind",
        ReducerRouteHoleKind::RolledDice => "RolledDiceHoleKind",
        ReducerRouteHoleKind::SavingThrowOutcome => "SavingThrowOutcomeHoleKind",
        ReducerRouteHoleKind::SkillChoice => "SkillChoiceHoleKind",
        ReducerRouteHoleKind::SpellTargetAllocation => "SpellTargetAllocationHoleKind",
        ReducerRouteHoleKind::SpellTargetList => "SpellTargetListHoleKind",
        ReducerRouteHoleKind::StatBlockRechargeRoll => "StatBlockRechargeRollHoleKind",
        ReducerRouteHoleKind::TargetChoice => "TargetChoiceHoleKind",
        ReducerRouteHoleKind::WildShapeEquipmentDisposition => {
            "WildShapeEquipmentDispositionHoleKind"
        }
    }
}

fn fill_ref(fill: ReducerRouteFillKind) -> &'static str {
    match fill {
        ReducerRouteFillKind::AttackRoll => "AttackRollFillKind",
        ReducerRouteFillKind::GrappleOutcome => "GrappleOutcomeFillKind",
        ReducerRouteFillKind::ConcentrationSavingThrow => "ConcentrationSavingThrowFillKind",
        ReducerRouteFillKind::ConditionChoice => "ConditionChoiceFillKind",
        ReducerRouteFillKind::DamageTypeChoice => "DamageTypeChoiceFillKind",
        ReducerRouteFillKind::DeathSavingThrow => "DeathSavingThrowFillKind",
        ReducerRouteFillKind::HitPointHealingDistribution => "HitPointHealingDistributionFillKind",
        ReducerRouteFillKind::CommandOptionChoice => "CommandOptionChoiceFillKind",
        ReducerRouteFillKind::Movement => "MovementFillKind",
        ReducerRouteFillKind::RolledDice => "RolledDiceFillKind",
        ReducerRouteFillKind::SavingThrowOutcome => "SavingThrowOutcomeFillKind",
        ReducerRouteFillKind::SkillChoice => "SkillChoiceFillKind",
        ReducerRouteFillKind::SpellTargetAllocation => "SpellTargetAllocationFillKind",
        ReducerRouteFillKind::SpellTargetList => "SpellTargetListFillKind",
        ReducerRouteFillKind::StatBlockRechargeRoll => "StatBlockRechargeRollFillKind",
        ReducerRouteFillKind::TargetChoice => "TargetChoiceFillKind",
        ReducerRouteFillKind::UnitFeatureDecision => "UnitFeatureDecisionFillKind",
        ReducerRouteFillKind::WildShapeEquipmentDisposition => {
            "WildShapeEquipmentDispositionFillKind"
        }
    }
}

fn owner_ref(owner: ReducerRouteOwnerGroup) -> &'static str {
    match owner {
        ReducerRouteOwnerGroup::ActionEconomy => "BattleActionEconomyOwner",
        ReducerRouteOwnerGroup::ActiveEffect => "BattleActiveEffectOwner",
        ReducerRouteOwnerGroup::AbilityCheckRollMode => "BattleAbilityCheckRollModeOwner",
        ReducerRouteOwnerGroup::AreaShape => "BattleAreaShapeOwner",
        ReducerRouteOwnerGroup::AttackRoll => "BattleAttackRollOwner",
        ReducerRouteOwnerGroup::AttackActionProcedure => "BattleAttackActionProcedureOwner",
        ReducerRouteOwnerGroup::Concentration => "BattleConcentrationOwner",
        ReducerRouteOwnerGroup::ConditionLifecycle => "BattleConditionLifecycleOwner",
        ReducerRouteOwnerGroup::Companion => "BattleCompanionOwner",
        ReducerRouteOwnerGroup::CreatureSpaceMovement => "BattleCreatureSpaceMovementOwner",
        ReducerRouteOwnerGroup::CreatureState => "BattleCreatureStateOwner",
        ReducerRouteOwnerGroup::DamageAdjustment => "BattleDamageAdjustmentOwner",
        ReducerRouteOwnerGroup::DamageRoll => "BattleDamageRollOwner",
        ReducerRouteOwnerGroup::DamageType => "BattleDamageTypeOwner",
        ReducerRouteOwnerGroup::FeatureResource => "BattleFeatureResourceOwner",
        ReducerRouteOwnerGroup::HitPointAndZeroHpLifecycle => {
            "BattleHitPointAndZeroHpLifecycleOwner"
        }
        ReducerRouteOwnerGroup::HitPoint => "BattleHitPointOwner",
        ReducerRouteOwnerGroup::HoleFrontier => "BattleHoleFrontierOwner",
        ReducerRouteOwnerGroup::InterruptStack => "BattleInterruptStackOwner",
        ReducerRouteOwnerGroup::MovementResource => "BattleMovementResourceOwner",
        ReducerRouteOwnerGroup::ObjectBoundary => "BattleObjectBoundaryOwner",
        ReducerRouteOwnerGroup::SavingThrowOutcome => "BattleSavingThrowOutcomeOwner",
        ReducerRouteOwnerGroup::SavingThrowRollMode => "BattleSavingThrowRollModeOwner",
        ReducerRouteOwnerGroup::SpellAttackProcedure => "BattleSpellAttackProcedureOwner",
        ReducerRouteOwnerGroup::SpellSlotAndActionEconomy => "BattleSpellSlotAndActionEconomyOwner",
        ReducerRouteOwnerGroup::StatBlockAction => "BattleStatBlockActionOwner",
        ReducerRouteOwnerGroup::TargetSelection => "BattleTargetSelectionOwner",
        ReducerRouteOwnerGroup::TemporaryHitPoint => "BattleTemporaryHitPointOwner",
        ReducerRouteOwnerGroup::TurnBoundary => "BattleTurnBoundaryOwner",
    }
}

fn outcome_ref(outcome: ReducerRouteResolutionOutcome) -> &'static str {
    match outcome {
        ReducerRouteResolutionOutcome::NeedsHoles => "needsHoles",
        ReducerRouteResolutionOutcome::Resolved => "resolved",
        ReducerRouteResolutionOutcome::Invalid(BattleResolutionInvalidReason::InvalidFill) => {
            "invalid:InvalidFill"
        }
        ReducerRouteResolutionOutcome::Invalid(BattleResolutionInvalidReason::StaleSubject) => {
            "invalid:StaleSubject"
        }
        ReducerRouteResolutionOutcome::Invalid(BattleResolutionInvalidReason::WrongActor) => {
            "invalid:WrongActor"
        }
        ReducerRouteResolutionOutcome::Invalid(BattleResolutionInvalidReason::WrongTarget) => {
            "invalid:WrongTarget"
        }
    }
}

fn subject_ref(subject: ReducerRouteSubjectFamily) -> &'static str {
    match subject {
        ReducerRouteSubjectFamily::ActiveFormLifecycle => "ActiveFormLifecycleRouteSubject",
        ReducerRouteSubjectFamily::BattleAction => "BattleActionRouteSubject",
        ReducerRouteSubjectFamily::ConcentrationTeardown => "ConcentrationTeardownRouteSubject",
        ReducerRouteSubjectFamily::CommandSpell => "CommandSpellRouteSubject",
        ReducerRouteSubjectFamily::CompanionLifecycle => "CompanionLifecycleRouteSubject",
        ReducerRouteSubjectFamily::CompanionSharedSenses => "CompanionSharedSensesRouteSubject",
        ReducerRouteSubjectFamily::CompanionTouchDelivery => "CompanionTouchDeliveryRouteSubject",
        ReducerRouteSubjectFamily::CompanionReactionAttack => "CompanionReactionAttackRouteSubject",
        ReducerRouteSubjectFamily::CreatureAttack => "CreatureAttackRouteSubject",
        ReducerRouteSubjectFamily::CreatureSpaceMovementPermission => {
            "CreatureSpaceMovementPermissionRouteSubject"
        }
        ReducerRouteSubjectFamily::CreatureStatProjection => "CreatureStatProjectionRouteSubject",
        ReducerRouteSubjectFamily::DeathSavingThrow => "DeathSavingThrowRouteSubject",
        ReducerRouteSubjectFamily::ForcedMovement => "ForcedMovementRouteSubject",
        ReducerRouteSubjectFamily::HitPointRestoration => "HitPointRestorationRouteSubject",
        ReducerRouteSubjectFamily::MovementResource => "MovementResourceRouteSubject",
        ReducerRouteSubjectFamily::PassiveAbilityCheckRollMode => {
            "PassiveAbilityCheckRollModeRouteSubject"
        }
        ReducerRouteSubjectFamily::PassiveDamageAdjustment => "PassiveDamageAdjustmentRouteSubject",
        ReducerRouteSubjectFamily::PassiveSavingThrowRollMode => {
            "PassiveSavingThrowRollModeRouteSubject"
        }
        ReducerRouteSubjectFamily::RepeatSaveConditionEffect => {
            "RepeatSaveConditionEffectRouteSubject"
        }
        ReducerRouteSubjectFamily::RollModifierEffect => "RollModifierEffectRouteSubject",
        ReducerRouteSubjectFamily::SaveGatedSpell => "SaveGatedSpellRouteSubject",
        ReducerRouteSubjectFamily::SlotSpell => "SlotSpellRouteSubject",
        ReducerRouteSubjectFamily::SpellAttack => "SpellAttackRouteSubject",
        ReducerRouteSubjectFamily::ScalarBuff => "ScalarBuffRouteSubject",
        ReducerRouteSubjectFamily::ScalarBuffEffect => "ScalarBuffEffectRouteSubject",
        ReducerRouteSubjectFamily::SpellDamageReduction => "SpellDamageReductionRouteSubject",
        ReducerRouteSubjectFamily::StatBlockAction => "StatBlockActionRouteSubject",
        ReducerRouteSubjectFamily::WeaponAttack => "WeaponAttackRouteSubject",
        ReducerRouteSubjectFamily::WeaponMasteryProperty => "WeaponMasteryPropertyRouteSubject",
        ReducerRouteSubjectFamily::AttackActionAreaSaveDamageReplacement => {
            "AttackActionAreaSaveDamageReplacementRouteSubject"
        }
        ReducerRouteSubjectFamily::MetamagicOptionSpell => "MetamagicOptionSpellRouteSubject",
        ReducerRouteSubjectFamily::UnitFeatureBonusAction => "UnitFeatureBonusActionRouteSubject",
        ReducerRouteSubjectFamily::ActiveFeatureSpellSaveDc => {
            "ActiveFeatureSpellSaveDcRouteSubject"
        }
        ReducerRouteSubjectFamily::ActiveFeatureSpellAttackRollMode => {
            "ActiveFeatureSpellAttackRollModeRouteSubject"
        }
        ReducerRouteSubjectFamily::ZeroHitPointStabilization => {
            "ZeroHitPointStabilizationRouteSubject"
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{
        battle_resolution_continuation, route_discover_battle_acts_from_result,
        route_resolve_battle_subject_from_result, ReducerRouteEvent, ReducerRouteFillKind,
        ReducerRouteHoleKind, ReducerRouteOwnerGroup, ReducerRouteResolutionOutcome,
        ReducerRouteResolveConnector, ReducerRouteResolveFill, ReducerRouteSubjectFamily,
    };
    use crate::rules::battle_reducer_spine::{
        discover_rolled_stat_block_attack_control, start_fighter_skeleton_battle,
        start_stat_block_actor_battle, Actor, BattleHoleKind, BattleResolutionInvalidReason,
        BattleResolutionResult, BattleSubject, BattleSubjectKind,
    };
    use crate::rules::weapon_attack_ordering::WeaponAttackFrontierStage;

    #[test]
    fn resolve_route_event_reads_holes_from_resolution_result() {
        let result = BattleResolutionResult::NeedsHoles {
            state: start_fighter_skeleton_battle(),
            subject: route_test_subject(BattleSubjectKind::SlotSpell),
            holes: vec![BattleHoleKind::RolledDice],
        };

        let event = route_resolve_battle_subject_from_result(
            ReducerRouteResolveConnector {
                subject: ReducerRouteSubjectFamily::SlotSpell,
                fill: ReducerRouteResolveFill::Fill(ReducerRouteFillKind::SpellTargetAllocation),
                owner: ReducerRouteOwnerGroup::HoleFrontier,
            },
            &result,
        );

        assert_eq!(
            event,
            ReducerRouteEvent::ResolveBattleSubject {
                subject: ReducerRouteSubjectFamily::SlotSpell,
                fill: ReducerRouteFillKind::SpellTargetAllocation,
                outcome: ReducerRouteResolutionOutcome::NeedsHoles,
                holes: vec![ReducerRouteHoleKind::RolledDice],
                owner: ReducerRouteOwnerGroup::HoleFrontier,
            }
        );
    }

    #[test]
    fn resolved_route_event_uses_empty_holes_from_result_outcome() {
        let result = BattleResolutionResult::Resolved {
            state: start_fighter_skeleton_battle(),
        };

        let event = route_resolve_battle_subject_from_result(
            ReducerRouteResolveConnector {
                subject: ReducerRouteSubjectFamily::SaveGatedSpell,
                fill: ReducerRouteResolveFill::Fill(ReducerRouteFillKind::RolledDice),
                owner: ReducerRouteOwnerGroup::HitPoint,
            },
            &result,
        );

        assert_eq!(
            event,
            ReducerRouteEvent::ResolveBattleSubject {
                subject: ReducerRouteSubjectFamily::SaveGatedSpell,
                fill: ReducerRouteFillKind::RolledDice,
                outcome: ReducerRouteResolutionOutcome::Resolved,
                holes: Vec::new(),
                owner: ReducerRouteOwnerGroup::HitPoint,
            }
        );
    }

    #[test]
    fn no_fill_route_event_reads_invalid_result_holes() {
        let result = BattleResolutionResult::Invalid {
            state: start_fighter_skeleton_battle(),
            reason: BattleResolutionInvalidReason::WrongActor,
            holes: vec![BattleHoleKind::ConcentrationSavingThrow],
        };

        let event = route_resolve_battle_subject_from_result(
            ReducerRouteResolveConnector {
                subject: ReducerRouteSubjectFamily::ConcentrationTeardown,
                fill: ReducerRouteResolveFill::WithoutFill,
                owner: ReducerRouteOwnerGroup::Concentration,
            },
            &result,
        );

        assert_eq!(
            event,
            ReducerRouteEvent::ResolveBattleSubjectWithoutFill {
                subject: ReducerRouteSubjectFamily::ConcentrationTeardown,
                outcome: ReducerRouteResolutionOutcome::Invalid(
                    BattleResolutionInvalidReason::WrongActor
                ),
                holes: vec![ReducerRouteHoleKind::ConcentrationSavingThrow],
                owner: ReducerRouteOwnerGroup::Concentration,
            }
        );
    }

    #[test]
    fn discovery_route_event_reads_stat_block_holes_from_resolution_result() {
        let result = discover_rolled_stat_block_attack_control(
            start_stat_block_actor_battle(Actor::Goblin),
            Actor::Goblin,
        );

        let event = route_discover_battle_acts_from_result(
            ReducerRouteSubjectFamily::StatBlockAction,
            &result,
            ReducerRouteOwnerGroup::StatBlockAction,
        );

        assert_eq!(
            event,
            ReducerRouteEvent::DiscoverBattleActs {
                subject: ReducerRouteSubjectFamily::StatBlockAction,
                holes: vec![ReducerRouteHoleKind::TargetChoice],
                owner: ReducerRouteOwnerGroup::StatBlockAction,
            }
        );
    }

    #[test]
    fn continuation_helper_consumes_stat_block_needs_holes_result() {
        let result = discover_rolled_stat_block_attack_control(
            start_stat_block_actor_battle(Actor::Goblin),
            Actor::Goblin,
        );
        let borrowed = result
            .needs_holes()
            .expect("stat-block needs-holes result should expose generic holes");
        let continuing_subject = result
            .continuing_subject()
            .expect("stat-block needs-holes result should expose a continuing subject");

        assert_eq!(borrowed.holes(), &[BattleHoleKind::TargetChoice]);
        assert_eq!(borrowed.subject(), continuing_subject);

        let (_, subject) = battle_resolution_continuation(result, "stat-block test continuation");
        assert_eq!(subject, continuing_subject);
    }

    fn route_test_subject(kind: BattleSubjectKind) -> BattleSubject {
        BattleSubject {
            kind,
            actor: Actor::Fighter,
            target: Some(Actor::Skeleton),
            stage: WeaponAttackFrontierStage::ActSelection,
            damage_modifier: 0,
        }
    }
}
