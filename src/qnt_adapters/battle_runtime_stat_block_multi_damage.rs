use crate::rules::battle_reducer_spine::{
    discover_rolled_stat_block_attack_control, discover_static_stat_block_attack_control,
    resolve_stat_block_action_subject, start_stat_block_actor_battle, Actor, AttackRollFacts,
    BattleState, StatBlockActionDamageMode, StatBlockActionFill, StatBlockActionResolutionResult,
    StatBlockActionSubject,
};
use crate::rules::stat_block_action_ordering::StatBlockActionFrontierStage;

use super::battle_runtime_reducer_route::{
    route_discover_battle_acts_from_route_holes, route_resolve_stat_block_action_from_result,
    route_start_battle, ReducerRouteEvent, ReducerRouteFillKind, ReducerRouteHoleKind,
    ReducerRouteOwnerGroup, ReducerRouteResolveConnector, ReducerRouteResolveFill,
    ReducerRouteSubjectFamily,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StatBlockMultiDamageMode {
    Rolled,
    Static,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StatBlockMultiDamageProtocolResult {
    NeedsHoles,
    Resolved,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StatBlockMultiDamageState {
    pub target_hp: i16,
    pub damage_mode: StatBlockMultiDamageMode,
    pub holes: Vec<StatBlockMultiDamageHole>,
    pub protocol_result: StatBlockMultiDamageProtocolResult,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StatBlockMultiDamageHole {
    TargetChoice,
    AttackRoll,
    DamageRoll,
}

pub const BRANCH_ACTIONS: [&str; 3] = [
    "doFillTargetChoice",
    "doFillHitAttackRoll",
    "doResolveRolledDamage",
];

pub fn replay_observed_action(observed_action_taken: &str) -> StatBlockMultiDamageState {
    match observed_action_taken {
        "doFillTargetChoice" => project(rolled_target_choice()),
        "doFillHitAttackRoll" => project(rolled_hit_attack_roll()),
        "doResolveRolledDamage" => project(resolve_rolled_damage()),
        action => panic!("unsupported mbt::actionTaken {action}"),
    }
}

pub fn replay_observed_static_hit_attack_roll() -> StatBlockMultiDamageState {
    project(static_hit_attack_roll())
}

pub fn expected_witness(observed_action_taken: &str) -> StatBlockMultiDamageState {
    match observed_action_taken {
        "doFillTargetChoice" => StatBlockMultiDamageState {
            target_hp: 12,
            damage_mode: StatBlockMultiDamageMode::Rolled,
            holes: vec![StatBlockMultiDamageHole::AttackRoll],
            protocol_result: StatBlockMultiDamageProtocolResult::NeedsHoles,
        },
        "doFillHitAttackRoll" => StatBlockMultiDamageState {
            target_hp: 12,
            damage_mode: StatBlockMultiDamageMode::Rolled,
            holes: vec![StatBlockMultiDamageHole::DamageRoll],
            protocol_result: StatBlockMultiDamageProtocolResult::NeedsHoles,
        },
        "doResolveRolledDamage" => StatBlockMultiDamageState {
            target_hp: 10,
            damage_mode: StatBlockMultiDamageMode::Rolled,
            holes: Vec::new(),
            protocol_result: StatBlockMultiDamageProtocolResult::Resolved,
        },
        action => panic!("unsupported mbt::actionTaken {action}"),
    }
}

pub fn expected_static_hit_attack_roll() -> StatBlockMultiDamageState {
    StatBlockMultiDamageState {
        target_hp: 9,
        damage_mode: StatBlockMultiDamageMode::Static,
        holes: Vec::new(),
        protocol_result: StatBlockMultiDamageProtocolResult::Resolved,
    }
}

pub fn replay_observed_route(observed_action_taken: &str) -> Vec<ReducerRouteEvent> {
    match observed_action_taken {
        "doFillTargetChoice" => rolled_target_choice_route().2,
        "doFillHitAttackRoll" => rolled_hit_attack_roll_route().2,
        "doResolveRolledDamage" => {
            let (state, subject, mut route) = rolled_hit_attack_roll_route();
            let result = resolve_stat_block_action_subject(
                state,
                subject,
                StatBlockActionFill::DamageDice(2),
            );
            route.push(stat_block_route_event(
                ReducerRouteFillKind::RolledDice,
                ReducerRouteOwnerGroup::HitPoint,
                &result,
            ));
            route
        }
        action => panic!("unsupported mbt::actionTaken {action}"),
    }
}

pub fn expected_route(observed_action_taken: &str) -> Vec<ReducerRouteEvent> {
    replay_observed_route(observed_action_taken)
}

pub fn projection_payload(state: &StatBlockMultiDamageState) -> String {
    [
        format!("qTargetHp={}", state.target_hp),
        format!("qDamageMode={}", damage_mode_ref(state.damage_mode)),
        format!("qHoles={}", joined_or_none(&hole_refs(&state.holes))),
        format!(
            "protocolResult={}",
            protocol_result_ref(state.protocol_result)
        ),
        format!("protocolHoles={}", joined_or_none(&hole_refs(&state.holes))),
    ]
    .join("\n")
}

fn rolled_subject() -> (BattleState, StatBlockActionSubject) {
    expect_needs_holes(discover_rolled_stat_block_attack_control(
        start_stat_block_actor_battle(Actor::Goblin),
        Actor::Goblin,
    ))
}

fn rolled_subject_route() -> (BattleState, StatBlockActionSubject, Vec<ReducerRouteEvent>) {
    let result = discover_rolled_stat_block_attack_control(
        start_stat_block_actor_battle(Actor::Goblin),
        Actor::Goblin,
    );
    let route = initial_stat_block_route();
    let (state, subject) = expect_needs_holes(result);
    (state, subject, route)
}

fn static_subject() -> (BattleState, StatBlockActionSubject) {
    expect_needs_holes(discover_static_stat_block_attack_control(
        start_stat_block_actor_battle(Actor::Goblin),
        Actor::Goblin,
        3,
    ))
}

fn rolled_target_choice() -> StatBlockActionResolutionResult {
    let (state, subject) = rolled_subject();
    resolve_stat_block_action_subject(
        state,
        subject,
        StatBlockActionFill::TargetChoice(Actor::Fighter),
    )
}

fn rolled_target_choice_route() -> (BattleState, StatBlockActionSubject, Vec<ReducerRouteEvent>) {
    let (state, subject, mut route) = rolled_subject_route();
    let result = resolve_stat_block_action_subject(
        state,
        subject,
        StatBlockActionFill::TargetChoice(Actor::Fighter),
    );
    route.push(stat_block_route_event(
        ReducerRouteFillKind::TargetChoice,
        ReducerRouteOwnerGroup::TargetSelection,
        &result,
    ));
    let (state, subject) = expect_needs_holes(result);
    (state, subject, route)
}

fn rolled_target_chosen_subject() -> (BattleState, StatBlockActionSubject) {
    expect_needs_holes(rolled_target_choice())
}

fn static_target_chosen_subject() -> (BattleState, StatBlockActionSubject) {
    let (state, subject) = static_subject();
    expect_needs_holes(resolve_stat_block_action_subject(
        state,
        subject,
        StatBlockActionFill::TargetChoice(Actor::Fighter),
    ))
}

fn rolled_hit_attack_roll() -> StatBlockActionResolutionResult {
    let (state, subject) = rolled_target_chosen_subject();
    resolve_stat_block_action_subject(state, subject, StatBlockActionFill::AttackRoll(hit_roll()))
}

fn rolled_hit_attack_roll_route() -> (BattleState, StatBlockActionSubject, Vec<ReducerRouteEvent>) {
    let (state, subject, mut route) = rolled_target_choice_route();
    let result = resolve_stat_block_action_subject(
        state,
        subject,
        StatBlockActionFill::AttackRoll(hit_roll()),
    );
    route.push(stat_block_route_event(
        ReducerRouteFillKind::AttackRoll,
        ReducerRouteOwnerGroup::AttackRoll,
        &result,
    ));
    let (state, subject) = expect_needs_holes(result);
    (state, subject, route)
}

fn static_hit_attack_roll() -> StatBlockActionResolutionResult {
    let (state, subject) = static_target_chosen_subject();
    resolve_stat_block_action_subject(state, subject, StatBlockActionFill::AttackRoll(hit_roll()))
}

fn resolve_rolled_damage() -> StatBlockActionResolutionResult {
    let (state, subject) = expect_needs_holes(rolled_hit_attack_roll());
    resolve_stat_block_action_subject(state, subject, StatBlockActionFill::DamageDice(2))
}

fn expect_needs_holes(
    result: StatBlockActionResolutionResult,
) -> (BattleState, StatBlockActionSubject) {
    match result {
        StatBlockActionResolutionResult::NeedsHoles { state, subject, .. } => (state, subject),
        other => panic!("expected stat-block action subject with holes, got {other:?}"),
    }
}

fn initial_stat_block_route() -> Vec<ReducerRouteEvent> {
    vec![
        route_start_battle(ReducerRouteOwnerGroup::ActionEconomy),
        route_discover_battle_acts_from_route_holes(
            ReducerRouteSubjectFamily::StatBlockAction,
            vec![ReducerRouteHoleKind::TargetChoice],
            ReducerRouteOwnerGroup::StatBlockAction,
        ),
    ]
}

fn stat_block_route_event(
    fill: ReducerRouteFillKind,
    owner: ReducerRouteOwnerGroup,
    result: &StatBlockActionResolutionResult,
) -> ReducerRouteEvent {
    route_resolve_stat_block_action_from_result(
        ReducerRouteResolveConnector {
            subject: ReducerRouteSubjectFamily::StatBlockAction,
            fill: ReducerRouteResolveFill::Fill(fill),
            owner,
        },
        result,
    )
}

fn project(result: StatBlockActionResolutionResult) -> StatBlockMultiDamageState {
    let (state, subject, protocol_result) = match result {
        StatBlockActionResolutionResult::NeedsHoles { state, subject, .. } => (
            state,
            subject,
            StatBlockMultiDamageProtocolResult::NeedsHoles,
        ),
        StatBlockActionResolutionResult::Resolved { state, subject } => {
            (state, subject, StatBlockMultiDamageProtocolResult::Resolved)
        }
        StatBlockActionResolutionResult::Invalid { state, subject, .. } => (
            state,
            subject,
            StatBlockMultiDamageProtocolResult::NeedsHoles,
        ),
    };

    StatBlockMultiDamageState {
        target_hp: state.fighter.hp,
        damage_mode: match subject.damage_mode {
            StatBlockActionDamageMode::Rolled => StatBlockMultiDamageMode::Rolled,
            StatBlockActionDamageMode::Static { .. } => StatBlockMultiDamageMode::Static,
        },
        holes: stat_block_multi_damage_holes(subject.stage, protocol_result),
        protocol_result,
    }
}

fn stat_block_multi_damage_holes(
    stage: StatBlockActionFrontierStage,
    protocol_result: StatBlockMultiDamageProtocolResult,
) -> Vec<StatBlockMultiDamageHole> {
    if protocol_result != StatBlockMultiDamageProtocolResult::NeedsHoles {
        return Vec::new();
    }
    match stage {
        StatBlockActionFrontierStage::AttackTargetChoice => {
            vec![StatBlockMultiDamageHole::TargetChoice]
        }
        StatBlockActionFrontierStage::AttackRoll => vec![StatBlockMultiDamageHole::AttackRoll],
        StatBlockActionFrontierStage::DamageDice => vec![StatBlockMultiDamageHole::DamageRoll],
        StatBlockActionFrontierStage::ActSelection
        | StatBlockActionFrontierStage::RechargeRoll
        | StatBlockActionFrontierStage::Resolved => Vec::new(),
    }
}

fn hit_roll() -> AttackRollFacts {
    AttackRollFacts {
        total: 16,
        natural_d20: 12,
    }
}

fn damage_mode_ref(mode: StatBlockMultiDamageMode) -> &'static str {
    match mode {
        StatBlockMultiDamageMode::Rolled => "RolledDamageMode",
        StatBlockMultiDamageMode::Static => "StaticDamageMode",
    }
}

fn protocol_result_ref(result: StatBlockMultiDamageProtocolResult) -> &'static str {
    match result {
        StatBlockMultiDamageProtocolResult::NeedsHoles => "needsHoles",
        StatBlockMultiDamageProtocolResult::Resolved => "resolved",
    }
}

fn hole_refs(holes: &[StatBlockMultiDamageHole]) -> Vec<&'static str> {
    holes
        .iter()
        .map(|hole| match hole {
            StatBlockMultiDamageHole::TargetChoice => "TargetChoice",
            StatBlockMultiDamageHole::AttackRoll => "AttackRoll",
            StatBlockMultiDamageHole::DamageRoll => "DamageRoll",
        })
        .collect()
}

fn joined_or_none(values: &[&str]) -> String {
    if values.is_empty() {
        "none".to_string()
    } else {
        values.join(",")
    }
}
