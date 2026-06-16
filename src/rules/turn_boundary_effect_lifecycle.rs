#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TurnBoundaryLifecycleScenario {
    Init,
    TargetStartTurnResolved,
    SourceNextTurnResolved,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TurnBoundaryActor {
    SourceTurn,
    TargetTurn,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TurnBoundaryHoleOrder {
    NoBoundaryHoles,
    TurnStartDamageThenSave,
    TurnEndDamageOnly,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TurnBoundaryLifecycleProtocol {
    Init,
    Resolved,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TurnBoundaryEffectLifecycleState {
    pub scenario: TurnBoundaryLifecycleScenario,
    pub actor: TurnBoundaryActor,
    pub round: i16,
    pub target_hp: i16,
    pub turn_start_damage_active: bool,
    pub turn_end_damage_active: bool,
    pub until_next_turn_active: bool,
    pub start_turn_ongoing_feature_active: bool,
    pub end_turn_ongoing_feature_active: bool,
    pub turn_start_damage_applied_before_end_damage: bool,
    pub turn_end_damage_applied_before_expiry: bool,
    pub end_turn_ongoing_expired_at_target_end: bool,
    pub until_next_turn_expired_at_source_start: bool,
    pub start_turn_ongoing_expired_at_source_start: bool,
    pub turn_start_duration_expired_after_round_tick: bool,
    pub last_hole_order: TurnBoundaryHoleOrder,
    pub protocol: TurnBoundaryLifecycleProtocol,
}

#[must_use]
pub fn turn_boundary_effect_lifecycle_initial_state() -> TurnBoundaryEffectLifecycleState {
    TurnBoundaryEffectLifecycleState {
        scenario: TurnBoundaryLifecycleScenario::Init,
        actor: TurnBoundaryActor::SourceTurn,
        round: 1,
        target_hp: 10,
        turn_start_damage_active: true,
        turn_end_damage_active: true,
        until_next_turn_active: true,
        start_turn_ongoing_feature_active: true,
        end_turn_ongoing_feature_active: true,
        turn_start_damage_applied_before_end_damage: false,
        turn_end_damage_applied_before_expiry: false,
        end_turn_ongoing_expired_at_target_end: false,
        until_next_turn_expired_at_source_start: false,
        start_turn_ongoing_expired_at_source_start: false,
        turn_start_duration_expired_after_round_tick: false,
        last_hole_order: TurnBoundaryHoleOrder::NoBoundaryHoles,
        protocol: TurnBoundaryLifecycleProtocol::Init,
    }
}

#[must_use]
pub fn resolve_target_start_turn() -> TurnBoundaryEffectLifecycleState {
    // RAW: cleanroom-input/raw/srd-5.2.1/Playing-the-Game.md
    // "The Order of Combat"; RAW:
    // cleanroom-input/raw/srd-5.2.1/Rules-Glossary.md "Burning";
    // QNT: battle-runtime-turn-boundary-effect-lifecycle.mbt.qnt.
    TurnBoundaryEffectLifecycleState {
        scenario: TurnBoundaryLifecycleScenario::TargetStartTurnResolved,
        actor: TurnBoundaryActor::TargetTurn,
        round: 1,
        target_hp: 8,
        turn_start_damage_active: true,
        turn_end_damage_active: true,
        until_next_turn_active: true,
        start_turn_ongoing_feature_active: true,
        end_turn_ongoing_feature_active: true,
        turn_start_damage_applied_before_end_damage: true,
        turn_end_damage_applied_before_expiry: false,
        end_turn_ongoing_expired_at_target_end: false,
        until_next_turn_expired_at_source_start: false,
        start_turn_ongoing_expired_at_source_start: false,
        turn_start_duration_expired_after_round_tick: false,
        last_hole_order: TurnBoundaryHoleOrder::TurnStartDamageThenSave,
        protocol: TurnBoundaryLifecycleProtocol::Resolved,
    }
}

#[must_use]
pub fn resolve_source_next_turn() -> TurnBoundaryEffectLifecycleState {
    resolve_source_next_turn_after_target_start(resolve_target_start_turn())
}

#[must_use]
pub fn resolve_source_next_turn_after_target_start(
    target_start: TurnBoundaryEffectLifecycleState,
) -> TurnBoundaryEffectLifecycleState {
    // RAW: cleanroom-input/raw/srd-5.2.1/Playing-the-Game.md
    // "The Order of Combat"; RAW:
    // cleanroom-input/raw/srd-5.2.1/Rules-Glossary.md
    // "Simultaneous Effects", "Reaction", and "Ready"; QNT:
    // battle-runtime-turn-boundary-effect-lifecycle.mbt.qnt.
    let target_hp_after_target_end_damage =
        if target_start.scenario == TurnBoundaryLifecycleScenario::TargetStartTurnResolved {
            5
        } else {
            target_start.target_hp
        };

    TurnBoundaryEffectLifecycleState {
        scenario: TurnBoundaryLifecycleScenario::SourceNextTurnResolved,
        actor: TurnBoundaryActor::SourceTurn,
        round: 2,
        target_hp: target_hp_after_target_end_damage,
        turn_start_damage_active: false,
        turn_end_damage_active: false,
        until_next_turn_active: false,
        start_turn_ongoing_feature_active: false,
        end_turn_ongoing_feature_active: false,
        turn_start_damage_applied_before_end_damage: target_start
            .turn_start_damage_applied_before_end_damage,
        turn_end_damage_applied_before_expiry: true,
        end_turn_ongoing_expired_at_target_end: true,
        until_next_turn_expired_at_source_start: true,
        start_turn_ongoing_expired_at_source_start: true,
        turn_start_duration_expired_after_round_tick: true,
        last_hole_order: TurnBoundaryHoleOrder::TurnEndDamageOnly,
        protocol: TurnBoundaryLifecycleProtocol::Resolved,
    }
}
