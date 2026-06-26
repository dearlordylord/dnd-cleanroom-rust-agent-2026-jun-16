use crate::rules::battle_reducer_spine::{
    discover_prone_rider_stat_block_attack_control, resolve_battle_subject,
    start_prone_rider_stat_block_battle, Actor, AttackRollFacts, BattleHoleKind,
    BattleResolutionRequest, BattleResolutionResult, BattleState, StatBlockActionFill,
    StatBlockActionSubject, StatBlockProneOnHitRider,
};
use crate::rules::creature_size::CreatureSize;
use crate::rules::stat_block_action_ordering::StatBlockActionFrontierStage;

use super::battle_runtime_reducer_route::{
    route_discover_battle_acts, route_discover_battle_acts_from_route_holes,
    route_resolve_battle_subject_from_result, route_start_battle, ReducerRouteEvent,
    ReducerRouteFillKind, ReducerRouteHoleKind, ReducerRouteOwnerGroup,
    ReducerRouteResolveConnector, ReducerRouteResolveFill, ReducerRouteSubjectFamily,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StatBlockSizeGatedTarget {
    MediumOrSmaller,
    Larger,
    MediumOrSmallerProneImmune,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StatBlockSizeGatedProtocolResult {
    NeedsHoles,
    Resolved,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StatBlockSizeGatedConditionRiderState {
    pub target_hp: i16,
    pub target_prone: bool,
    pub target_size_gate: StatBlockSizeGatedTarget,
    pub holes: Vec<StatBlockSizeGatedConditionRiderHole>,
    pub protocol_result: StatBlockSizeGatedProtocolResult,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StatBlockSizeGatedConditionRiderHole {
    TargetChoice,
    AttackRoll,
    DamageRoll,
}

pub const BRANCH_ACTIONS: [&str; 3] = [
    "doFillTargetChoice",
    "doFillHitAttackRoll",
    "doResolveDamage",
];

pub fn replay_observed_action(
    observed_action_taken: &str,
) -> StatBlockSizeGatedConditionRiderState {
    match observed_action_taken {
        "doFillTargetChoice" => project(medium_target_choice()),
        "doFillHitAttackRoll" => project(medium_hit_attack_roll()),
        "doResolveDamage" => project(resolve_medium_damage()),
        action => panic!("unsupported mbt::actionTaken {action}"),
    }
}

pub fn replay_observed_larger_hit_attack_roll() -> StatBlockSizeGatedConditionRiderState {
    project(larger_hit_attack_roll())
}

pub fn replay_observed_prone_immune_hit_attack_roll() -> StatBlockSizeGatedConditionRiderState {
    project(prone_immune_hit_attack_roll())
}

pub fn expected_witness(observed_action_taken: &str) -> StatBlockSizeGatedConditionRiderState {
    match observed_action_taken {
        "doFillTargetChoice" => StatBlockSizeGatedConditionRiderState {
            target_hp: 20,
            target_prone: false,
            target_size_gate: StatBlockSizeGatedTarget::MediumOrSmaller,
            holes: vec![StatBlockSizeGatedConditionRiderHole::AttackRoll],
            protocol_result: StatBlockSizeGatedProtocolResult::NeedsHoles,
        },
        "doFillHitAttackRoll" => StatBlockSizeGatedConditionRiderState {
            target_hp: 20,
            target_prone: true,
            target_size_gate: StatBlockSizeGatedTarget::MediumOrSmaller,
            holes: vec![StatBlockSizeGatedConditionRiderHole::DamageRoll],
            protocol_result: StatBlockSizeGatedProtocolResult::NeedsHoles,
        },
        "doResolveDamage" => StatBlockSizeGatedConditionRiderState {
            target_hp: 17,
            target_prone: true,
            target_size_gate: StatBlockSizeGatedTarget::MediumOrSmaller,
            holes: Vec::new(),
            protocol_result: StatBlockSizeGatedProtocolResult::Resolved,
        },
        action => panic!("unsupported mbt::actionTaken {action}"),
    }
}

pub fn expected_larger_hit_attack_roll() -> StatBlockSizeGatedConditionRiderState {
    StatBlockSizeGatedConditionRiderState {
        target_hp: 20,
        target_prone: false,
        target_size_gate: StatBlockSizeGatedTarget::Larger,
        holes: vec![StatBlockSizeGatedConditionRiderHole::DamageRoll],
        protocol_result: StatBlockSizeGatedProtocolResult::NeedsHoles,
    }
}

pub fn expected_prone_immune_hit_attack_roll() -> StatBlockSizeGatedConditionRiderState {
    StatBlockSizeGatedConditionRiderState {
        target_hp: 20,
        target_prone: false,
        target_size_gate: StatBlockSizeGatedTarget::MediumOrSmallerProneImmune,
        holes: vec![StatBlockSizeGatedConditionRiderHole::DamageRoll],
        protocol_result: StatBlockSizeGatedProtocolResult::NeedsHoles,
    }
}

pub fn replay_observed_route(observed_action_taken: &str) -> Vec<ReducerRouteEvent> {
    match observed_action_taken {
        "doFillTargetChoice" => medium_target_choice_route().2,
        "doFillHitAttackRoll" => medium_hit_attack_roll_route().2,
        "doResolveDamage" => {
            let (state, subject, mut route) = medium_hit_attack_roll_route();
            let result =
                resolve_stat_block_action(state, subject, StatBlockActionFill::DamageDice(3));
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
    match observed_action_taken {
        "doFillTargetChoice" => expected_stat_block_size_gated_route(&[(
            ReducerRouteFillKind::TargetChoice,
            vec![BattleHoleKind::AttackRoll],
            ReducerRouteOwnerGroup::TargetSelection,
        )]),
        "doFillHitAttackRoll" => expected_stat_block_size_gated_route(&[
            (
                ReducerRouteFillKind::TargetChoice,
                vec![BattleHoleKind::AttackRoll],
                ReducerRouteOwnerGroup::TargetSelection,
            ),
            (
                ReducerRouteFillKind::AttackRoll,
                vec![BattleHoleKind::RolledDice],
                ReducerRouteOwnerGroup::ConditionLifecycle,
            ),
        ]),
        "doResolveDamage" => expected_stat_block_size_gated_route(&[
            (
                ReducerRouteFillKind::TargetChoice,
                vec![BattleHoleKind::AttackRoll],
                ReducerRouteOwnerGroup::TargetSelection,
            ),
            (
                ReducerRouteFillKind::AttackRoll,
                vec![BattleHoleKind::RolledDice],
                ReducerRouteOwnerGroup::ConditionLifecycle,
            ),
            (
                ReducerRouteFillKind::RolledDice,
                Vec::new(),
                ReducerRouteOwnerGroup::HitPoint,
            ),
        ]),
        action => panic!("unsupported mbt::actionTaken {action}"),
    }
}

fn expected_stat_block_size_gated_route(
    steps: &[(
        ReducerRouteFillKind,
        Vec<BattleHoleKind>,
        ReducerRouteOwnerGroup,
    )],
) -> Vec<ReducerRouteEvent> {
    let mut route = vec![
        route_start_battle(ReducerRouteOwnerGroup::ActionEconomy),
        route_discover_battle_acts(
            ReducerRouteSubjectFamily::StatBlockAction,
            vec![BattleHoleKind::TargetChoice],
            ReducerRouteOwnerGroup::StatBlockAction,
        ),
    ];
    for (fill, holes, owner) in steps {
        route.push(
            super::battle_runtime_reducer_route::route_resolve_battle_subject(
                ReducerRouteSubjectFamily::StatBlockAction,
                *fill,
                holes.clone(),
                *owner,
            ),
        );
    }
    route
}

pub fn projection_payload(state: &StatBlockSizeGatedConditionRiderState) -> String {
    [
        format!("qTargetHp={}", state.target_hp),
        format!("qTargetProne={}", state.target_prone),
        format!(
            "qTargetSizeGate={}",
            target_size_gate_ref(state.target_size_gate)
        ),
        format!("qHoles={}", joined_or_none(&hole_refs(&state.holes))),
        format!(
            "protocolResult={}",
            protocol_result_ref(state.protocol_result)
        ),
        format!("protocolHoles={}", joined_or_none(&hole_refs(&state.holes))),
    ]
    .join("\n")
}

fn medium_subject() -> (BattleState, StatBlockActionSubject) {
    rider_subject(CreatureSize::Medium, false)
}

fn medium_subject_route() -> (BattleState, StatBlockActionSubject, Vec<ReducerRouteEvent>) {
    let result = discover_prone_rider_stat_block_attack_control(
        start_prone_rider_stat_block_battle(Actor::Goblin, CreatureSize::Medium),
        Actor::Goblin,
        false,
    );
    let route = initial_stat_block_route();
    let (state, subject) = expect_needs_holes(result);
    (state, subject, route)
}

fn larger_subject() -> (BattleState, StatBlockActionSubject) {
    rider_subject(CreatureSize::Large, false)
}

fn prone_immune_subject() -> (BattleState, StatBlockActionSubject) {
    rider_subject(CreatureSize::Medium, true)
}

fn rider_subject(
    target_size: CreatureSize,
    target_prone_condition_immune: bool,
) -> (BattleState, StatBlockActionSubject) {
    expect_needs_holes(discover_prone_rider_stat_block_attack_control(
        start_prone_rider_stat_block_battle(Actor::Goblin, target_size),
        Actor::Goblin,
        target_prone_condition_immune,
    ))
}

fn medium_target_choice() -> BattleResolutionResult {
    let (state, subject) = medium_subject();
    resolve_stat_block_action(
        state,
        subject,
        StatBlockActionFill::TargetChoice(Actor::Fighter),
    )
}

fn medium_target_choice_route() -> (BattleState, StatBlockActionSubject, Vec<ReducerRouteEvent>) {
    let (state, subject, mut route) = medium_subject_route();
    let result = resolve_stat_block_action(
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

fn medium_target_chosen_subject() -> (BattleState, StatBlockActionSubject) {
    expect_needs_holes(medium_target_choice())
}

fn larger_target_chosen_subject() -> (BattleState, StatBlockActionSubject) {
    let (state, subject) = larger_subject();
    expect_needs_holes(resolve_stat_block_action(
        state,
        subject,
        StatBlockActionFill::TargetChoice(Actor::Fighter),
    ))
}

fn prone_immune_target_chosen_subject() -> (BattleState, StatBlockActionSubject) {
    let (state, subject) = prone_immune_subject();
    expect_needs_holes(resolve_stat_block_action(
        state,
        subject,
        StatBlockActionFill::TargetChoice(Actor::Fighter),
    ))
}

fn medium_hit_attack_roll() -> BattleResolutionResult {
    let (state, subject) = medium_target_chosen_subject();
    resolve_stat_block_action(state, subject, StatBlockActionFill::AttackRoll(hit_roll()))
}

fn medium_hit_attack_roll_route() -> (BattleState, StatBlockActionSubject, Vec<ReducerRouteEvent>) {
    let (state, subject, mut route) = medium_target_choice_route();
    let result =
        resolve_stat_block_action(state, subject, StatBlockActionFill::AttackRoll(hit_roll()));
    route.push(stat_block_route_event(
        ReducerRouteFillKind::AttackRoll,
        ReducerRouteOwnerGroup::ConditionLifecycle,
        &result,
    ));
    let (state, subject) = expect_needs_holes(result);
    (state, subject, route)
}

fn larger_hit_attack_roll() -> BattleResolutionResult {
    let (state, subject) = larger_target_chosen_subject();
    resolve_stat_block_action(state, subject, StatBlockActionFill::AttackRoll(hit_roll()))
}

fn prone_immune_hit_attack_roll() -> BattleResolutionResult {
    let (state, subject) = prone_immune_target_chosen_subject();
    resolve_stat_block_action(state, subject, StatBlockActionFill::AttackRoll(hit_roll()))
}

fn resolve_medium_damage() -> BattleResolutionResult {
    let (state, subject) = expect_needs_holes(medium_hit_attack_roll());
    resolve_stat_block_action(state, subject, StatBlockActionFill::DamageDice(3))
}

fn resolve_stat_block_action(
    state: BattleState,
    subject: StatBlockActionSubject,
    fill: StatBlockActionFill,
) -> BattleResolutionResult {
    resolve_battle_subject(
        state,
        BattleResolutionRequest::stat_block_action(subject, fill),
    )
}

fn expect_needs_holes(result: BattleResolutionResult) -> (BattleState, StatBlockActionSubject) {
    match result {
        BattleResolutionResult::StatBlockNeedsHoles { state, subject, .. } => (state, subject),
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
    result: &BattleResolutionResult,
) -> ReducerRouteEvent {
    route_resolve_battle_subject_from_result(
        ReducerRouteResolveConnector {
            subject: ReducerRouteSubjectFamily::StatBlockAction,
            fill: ReducerRouteResolveFill::Fill(fill),
            owner,
        },
        result,
    )
}

fn project(result: BattleResolutionResult) -> StatBlockSizeGatedConditionRiderState {
    let (state, subject, protocol_result) = match result {
        BattleResolutionResult::StatBlockNeedsHoles { state, subject, .. } => {
            (state, subject, StatBlockSizeGatedProtocolResult::NeedsHoles)
        }
        BattleResolutionResult::StatBlockResolved { state, subject } => {
            (state, subject, StatBlockSizeGatedProtocolResult::Resolved)
        }
        BattleResolutionResult::StatBlockInvalid { state, subject, .. } => {
            (state, subject, StatBlockSizeGatedProtocolResult::NeedsHoles)
        }
        other => panic!("expected stat-block size-gated result, got {other:?}"),
    };

    StatBlockSizeGatedConditionRiderState {
        target_hp: state.fighter.hp,
        target_prone: state.fighter.prone,
        target_size_gate: target_size_gate(&state, subject.prone_on_hit_rider),
        holes: stat_block_size_gated_holes(subject.stage, protocol_result),
        protocol_result,
    }
}

fn target_size_gate(
    state: &BattleState,
    rider: StatBlockProneOnHitRider,
) -> StatBlockSizeGatedTarget {
    match rider {
        StatBlockProneOnHitRider::MediumOrSmaller {
            target_prone_condition_immune: true,
        } => StatBlockSizeGatedTarget::MediumOrSmallerProneImmune,
        StatBlockProneOnHitRider::MediumOrSmaller {
            target_prone_condition_immune: false,
        } if state.fighter.creature_size.is_medium_or_smaller() => {
            StatBlockSizeGatedTarget::MediumOrSmaller
        }
        StatBlockProneOnHitRider::MediumOrSmaller {
            target_prone_condition_immune: false,
        } => StatBlockSizeGatedTarget::Larger,
        StatBlockProneOnHitRider::NoProneRider => StatBlockSizeGatedTarget::Larger,
    }
}

fn stat_block_size_gated_holes(
    stage: StatBlockActionFrontierStage,
    protocol_result: StatBlockSizeGatedProtocolResult,
) -> Vec<StatBlockSizeGatedConditionRiderHole> {
    if protocol_result != StatBlockSizeGatedProtocolResult::NeedsHoles {
        return Vec::new();
    }
    match stage {
        StatBlockActionFrontierStage::AttackTargetChoice => {
            vec![StatBlockSizeGatedConditionRiderHole::TargetChoice]
        }
        StatBlockActionFrontierStage::AttackRoll => {
            vec![StatBlockSizeGatedConditionRiderHole::AttackRoll]
        }
        StatBlockActionFrontierStage::DamageDice => {
            vec![StatBlockSizeGatedConditionRiderHole::DamageRoll]
        }
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

fn target_size_gate_ref(gate: StatBlockSizeGatedTarget) -> &'static str {
    match gate {
        StatBlockSizeGatedTarget::MediumOrSmaller => "MediumOrSmallerTarget",
        StatBlockSizeGatedTarget::Larger => "LargerTarget",
        StatBlockSizeGatedTarget::MediumOrSmallerProneImmune => "MediumOrSmallerProneImmuneTarget",
    }
}

fn protocol_result_ref(result: StatBlockSizeGatedProtocolResult) -> &'static str {
    match result {
        StatBlockSizeGatedProtocolResult::NeedsHoles => "needsHoles",
        StatBlockSizeGatedProtocolResult::Resolved => "resolved",
    }
}

fn hole_refs(holes: &[StatBlockSizeGatedConditionRiderHole]) -> Vec<&'static str> {
    holes
        .iter()
        .map(|hole| match hole {
            StatBlockSizeGatedConditionRiderHole::TargetChoice => "TargetChoice",
            StatBlockSizeGatedConditionRiderHole::AttackRoll => "AttackRoll",
            StatBlockSizeGatedConditionRiderHole::DamageRoll => "DamageRoll",
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
