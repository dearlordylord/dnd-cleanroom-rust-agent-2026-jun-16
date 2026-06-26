use crate::rules::battle_reducer_spine::{
    discover_rolled_stat_block_attack_control, discover_static_stat_block_attack_control,
    primary_stat_block_multiattack_profile, resolve_stat_block_action_subject,
    spend_recharge_gated_rolled_stat_block_attack, start_stat_block_actor_battle,
    start_stat_block_multiattack_control, stat_block_action_projection_from_result, Actor,
    AttackRollFacts, BattleState, StatBlockActionFill, StatBlockActionResolutionResult,
    StatBlockActionSubject,
};
use crate::rules::stat_block_action_ordering::{
    self, StatBlockActionFillOrderingError, StatBlockActionFrontierStage, StatBlockActionHoleKind,
    StatBlockActionInvalidReason, StatBlockActionOrderingProtocol, StatBlockActionOrderingState,
};

use super::battle_runtime_reducer_route::{
    route_discover_battle_acts_from_route_holes, route_resolve_stat_block_action_from_result,
    route_start_battle, ReducerRouteEvent, ReducerRouteFillKind, ReducerRouteHoleKind,
    ReducerRouteOwnerGroup, ReducerRouteResolveConnector, ReducerRouteResolveFill,
    ReducerRouteSubjectFamily,
};

pub const BRANCH_ACTIONS: [&str; 12] = [
    "doStartMultiattackControl",
    "doDiscoverRolledActionAttackControl",
    "doDiscoverStaticActionAttackControl",
    "doRejectAttackRollBeforeTargetChoice",
    "doFillTargetChoice",
    "doRejectDamageBeforeAttackRoll",
    "doFillAttackRollMiss",
    "doFillRolledAttackRollHit",
    "doFillStaticAttackRollHit",
    "doFillDamageDice",
    "doSpendRechargeGatedRolledAttack",
    "doFillRechargeRoll",
];

pub fn replay_observed_action(observed_action_taken: &str) -> StatBlockActionOrderingState {
    match observed_action_taken {
        "doStartMultiattackControl" => project(start_stat_block_multiattack_control(
            start_stat_block_actor_battle(Actor::Goblin),
            Actor::Goblin,
            primary_stat_block_multiattack_profile(3),
        )),
        "doDiscoverRolledActionAttackControl" => {
            project(discover_rolled_stat_block_attack_control(
                start_stat_block_actor_battle(Actor::Goblin),
                Actor::Goblin,
            ))
        }
        "doDiscoverStaticActionAttackControl" => {
            project(discover_static_stat_block_attack_control(
                start_stat_block_actor_battle(Actor::Goblin),
                Actor::Goblin,
                3,
            ))
        }
        "doRejectAttackRollBeforeTargetChoice" => {
            let (state, subject) = rolled_action_subject();
            project(resolve_stat_block_action_subject(
                state,
                subject,
                StatBlockActionFill::AttackRoll(hit_roll()),
            ))
        }
        "doFillTargetChoice" => {
            let (state, subject) = rolled_action_subject();
            project(resolve_stat_block_action_subject(
                state,
                subject,
                StatBlockActionFill::TargetChoice(Actor::Fighter),
            ))
        }
        "doRejectDamageBeforeAttackRoll" => {
            let (state, subject) = rolled_action_target_chosen_subject();
            project(resolve_stat_block_action_subject(
                state,
                subject,
                StatBlockActionFill::DamageDice(4),
            ))
        }
        "doFillAttackRollMiss" => {
            let (state, subject) = rolled_action_target_chosen_subject();
            project(resolve_stat_block_action_subject(
                state,
                subject,
                StatBlockActionFill::AttackRoll(miss_roll()),
            ))
        }
        "doFillRolledAttackRollHit" => {
            let (state, subject) = rolled_action_target_chosen_subject();
            project(resolve_stat_block_action_subject(
                state,
                subject,
                StatBlockActionFill::AttackRoll(hit_roll()),
            ))
        }
        "doFillStaticAttackRollHit" => {
            let (state, subject) = static_action_target_chosen_subject();
            project(resolve_stat_block_action_subject(
                state,
                subject,
                StatBlockActionFill::AttackRoll(hit_roll()),
            ))
        }
        "doFillDamageDice" => {
            let (state, subject) = rolled_action_attack_hit_subject();
            project(resolve_stat_block_action_subject(
                state,
                subject,
                StatBlockActionFill::DamageDice(4),
            ))
        }
        "doSpendRechargeGatedRolledAttack" => {
            project(spend_recharge_gated_rolled_stat_block_attack(
                start_stat_block_actor_battle(Actor::Goblin),
                Actor::Goblin,
            ))
        }
        "doFillRechargeRoll" => {
            let (state, subject) = recharge_roll_subject();
            project(resolve_stat_block_action_subject(
                state,
                subject,
                StatBlockActionFill::RechargeRoll(5),
            ))
        }
        action => panic!("unsupported mbt::actionTaken {action}"),
    }
}

pub fn expected_witness(observed_action_taken: &str) -> StatBlockActionOrderingState {
    match observed_action_taken {
        "doStartMultiattackControl" => stat_block_action_ordering::start_multiattack_control(),
        "doDiscoverRolledActionAttackControl" => {
            stat_block_action_ordering::discover_rolled_action_attack_control()
        }
        "doDiscoverStaticActionAttackControl" => {
            stat_block_action_ordering::discover_static_action_attack_control()
        }
        "doRejectAttackRollBeforeTargetChoice" => {
            stat_block_action_ordering::reject_attack_roll_before_target_choice()
        }
        "doFillTargetChoice" => stat_block_action_ordering::fill_stat_block_target_choice(),
        "doRejectDamageBeforeAttackRoll" => {
            stat_block_action_ordering::reject_damage_before_attack_roll()
        }
        "doFillAttackRollMiss" => stat_block_action_ordering::fill_stat_block_attack_roll_miss(),
        "doFillRolledAttackRollHit" => stat_block_action_ordering::fill_rolled_attack_roll_hit(),
        "doFillStaticAttackRollHit" => stat_block_action_ordering::fill_static_attack_roll_hit(),
        "doFillDamageDice" => stat_block_action_ordering::fill_stat_block_damage_dice(),
        "doSpendRechargeGatedRolledAttack" => {
            stat_block_action_ordering::spend_recharge_gated_rolled_attack()
        }
        "doFillRechargeRoll" => stat_block_action_ordering::fill_stat_block_recharge_roll(),
        action => panic!("unsupported mbt::actionTaken {action}"),
    }
}

pub fn replay_observed_route(observed_action_taken: &str) -> Vec<ReducerRouteEvent> {
    match observed_action_taken {
        "doStartMultiattackControl" => {
            let result = start_stat_block_multiattack_control(
                start_stat_block_actor_battle(Actor::Goblin),
                Actor::Goblin,
                primary_stat_block_multiattack_profile(3),
            );
            stat_block_discovery_route_from_result(
                &result,
                vec![ReducerRouteHoleKind::TargetChoice],
            )
        }
        "doDiscoverRolledActionAttackControl" => {
            let result = discover_rolled_stat_block_attack_control(
                start_stat_block_actor_battle(Actor::Goblin),
                Actor::Goblin,
            );
            stat_block_discovery_route_from_result(
                &result,
                vec![ReducerRouteHoleKind::TargetChoice],
            )
        }
        "doDiscoverStaticActionAttackControl" => {
            let result = discover_static_stat_block_attack_control(
                start_stat_block_actor_battle(Actor::Goblin),
                Actor::Goblin,
                3,
            );
            stat_block_discovery_route_from_result(
                &result,
                vec![ReducerRouteHoleKind::TargetChoice],
            )
        }
        "doRejectAttackRollBeforeTargetChoice" => {
            let (state, subject, mut route) = rolled_action_subject_route();
            let result = resolve_stat_block_action_subject(
                state,
                subject,
                StatBlockActionFill::AttackRoll(hit_roll()),
            );
            route.push(stat_block_route_event(
                ReducerRouteFillKind::AttackRoll,
                ReducerRouteOwnerGroup::HoleFrontier,
                &result,
            ));
            route
        }
        "doFillTargetChoice" => rolled_action_target_chosen_route().2,
        "doRejectDamageBeforeAttackRoll" => {
            let (state, subject, mut route) = rolled_action_target_chosen_route();
            let result = resolve_stat_block_action_subject(
                state,
                subject,
                StatBlockActionFill::DamageDice(4),
            );
            route.push(stat_block_route_event(
                ReducerRouteFillKind::RolledDice,
                ReducerRouteOwnerGroup::HoleFrontier,
                &result,
            ));
            route
        }
        "doFillAttackRollMiss" => {
            let (state, subject, mut route) = rolled_action_target_chosen_route();
            let result = resolve_stat_block_action_subject(
                state,
                subject,
                StatBlockActionFill::AttackRoll(miss_roll()),
            );
            route.push(stat_block_route_event(
                ReducerRouteFillKind::AttackRoll,
                ReducerRouteOwnerGroup::AttackRoll,
                &result,
            ));
            route
        }
        "doFillRolledAttackRollHit" => rolled_action_attack_hit_route().2,
        "doFillStaticAttackRollHit" => {
            let (state, subject, mut route) = static_action_target_chosen_route();
            let result = resolve_stat_block_action_subject(
                state,
                subject,
                StatBlockActionFill::AttackRoll(hit_roll()),
            );
            route.push(stat_block_route_event(
                ReducerRouteFillKind::AttackRoll,
                ReducerRouteOwnerGroup::HitPoint,
                &result,
            ));
            route
        }
        "doFillDamageDice" => {
            let (state, subject, mut route) = rolled_action_attack_hit_route();
            let result = resolve_stat_block_action_subject(
                state,
                subject,
                StatBlockActionFill::DamageDice(4),
            );
            route.push(stat_block_route_event(
                ReducerRouteFillKind::RolledDice,
                ReducerRouteOwnerGroup::HitPoint,
                &result,
            ));
            route
        }
        "doSpendRechargeGatedRolledAttack" => {
            let result = spend_recharge_gated_rolled_stat_block_attack(
                start_stat_block_actor_battle(Actor::Goblin),
                Actor::Goblin,
            );
            stat_block_discovery_route_from_result(
                &result,
                vec![ReducerRouteHoleKind::StatBlockRechargeRoll],
            )
        }
        "doFillRechargeRoll" => {
            let (state, subject, mut route) = recharge_roll_subject_route();
            let result = resolve_stat_block_action_subject(
                state,
                subject,
                StatBlockActionFill::RechargeRoll(5),
            );
            route.push(stat_block_route_event(
                ReducerRouteFillKind::StatBlockRechargeRoll,
                ReducerRouteOwnerGroup::StatBlockAction,
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

fn project(result: StatBlockActionResolutionResult) -> StatBlockActionOrderingState {
    stat_block_action_projection_from_result(&result)
}

fn rolled_action_subject() -> (BattleState, StatBlockActionSubject) {
    expect_needs_holes(discover_rolled_stat_block_attack_control(
        start_stat_block_actor_battle(Actor::Goblin),
        Actor::Goblin,
    ))
}

fn rolled_action_subject_route() -> (BattleState, StatBlockActionSubject, Vec<ReducerRouteEvent>) {
    let result = discover_rolled_stat_block_attack_control(
        start_stat_block_actor_battle(Actor::Goblin),
        Actor::Goblin,
    );
    let route =
        stat_block_discovery_route_from_result(&result, vec![ReducerRouteHoleKind::TargetChoice]);
    let (state, subject) = expect_needs_holes(result);
    (state, subject, route)
}

fn static_action_subject() -> (BattleState, StatBlockActionSubject) {
    expect_needs_holes(discover_static_stat_block_attack_control(
        start_stat_block_actor_battle(Actor::Goblin),
        Actor::Goblin,
        3,
    ))
}

fn static_action_subject_route() -> (BattleState, StatBlockActionSubject, Vec<ReducerRouteEvent>) {
    let result = discover_static_stat_block_attack_control(
        start_stat_block_actor_battle(Actor::Goblin),
        Actor::Goblin,
        3,
    );
    let route =
        stat_block_discovery_route_from_result(&result, vec![ReducerRouteHoleKind::TargetChoice]);
    let (state, subject) = expect_needs_holes(result);
    (state, subject, route)
}

fn rolled_action_target_chosen_subject() -> (BattleState, StatBlockActionSubject) {
    let (state, subject) = rolled_action_subject();
    expect_needs_holes(resolve_stat_block_action_subject(
        state,
        subject,
        StatBlockActionFill::TargetChoice(Actor::Fighter),
    ))
}

fn rolled_action_target_chosen_route(
) -> (BattleState, StatBlockActionSubject, Vec<ReducerRouteEvent>) {
    let (state, subject, mut route) = rolled_action_subject_route();
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

fn static_action_target_chosen_subject() -> (BattleState, StatBlockActionSubject) {
    let (state, subject) = static_action_subject();
    expect_needs_holes(resolve_stat_block_action_subject(
        state,
        subject,
        StatBlockActionFill::TargetChoice(Actor::Fighter),
    ))
}

fn static_action_target_chosen_route(
) -> (BattleState, StatBlockActionSubject, Vec<ReducerRouteEvent>) {
    let (state, subject, mut route) = static_action_subject_route();
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

fn rolled_action_attack_hit_subject() -> (BattleState, StatBlockActionSubject) {
    let (state, subject) = rolled_action_target_chosen_subject();
    expect_needs_holes(resolve_stat_block_action_subject(
        state,
        subject,
        StatBlockActionFill::AttackRoll(hit_roll()),
    ))
}

fn rolled_action_attack_hit_route() -> (BattleState, StatBlockActionSubject, Vec<ReducerRouteEvent>)
{
    let (state, subject, mut route) = rolled_action_target_chosen_route();
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

fn recharge_roll_subject() -> (BattleState, StatBlockActionSubject) {
    expect_needs_holes(spend_recharge_gated_rolled_stat_block_attack(
        start_stat_block_actor_battle(Actor::Goblin),
        Actor::Goblin,
    ))
}

fn recharge_roll_subject_route() -> (BattleState, StatBlockActionSubject, Vec<ReducerRouteEvent>) {
    let result = spend_recharge_gated_rolled_stat_block_attack(
        start_stat_block_actor_battle(Actor::Goblin),
        Actor::Goblin,
    );
    let route = stat_block_discovery_route_from_result(
        &result,
        vec![ReducerRouteHoleKind::StatBlockRechargeRoll],
    );
    let (state, subject) = expect_needs_holes(result);
    (state, subject, route)
}

fn stat_block_discovery_route_from_result(
    result: &StatBlockActionResolutionResult,
    holes: Vec<ReducerRouteHoleKind>,
) -> Vec<ReducerRouteEvent> {
    assert!(
        matches!(result, StatBlockActionResolutionResult::NeedsHoles { .. }),
        "stat-block discovery should need holes, got {result:?}"
    );
    vec![
        route_start_battle(ReducerRouteOwnerGroup::ActionEconomy),
        route_discover_battle_acts_from_route_holes(
            ReducerRouteSubjectFamily::StatBlockAction,
            holes,
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

fn expect_needs_holes(
    result: StatBlockActionResolutionResult,
) -> (BattleState, StatBlockActionSubject) {
    match result {
        StatBlockActionResolutionResult::NeedsHoles { state, subject, .. } => (state, subject),
        other => panic!("expected stat-block action subject with holes, got {other:?}"),
    }
}

fn hit_roll() -> AttackRollFacts {
    AttackRollFacts {
        total: 16,
        natural_d20: 12,
    }
}

fn miss_roll() -> AttackRollFacts {
    AttackRollFacts {
        total: 2,
        natural_d20: 1,
    }
}

pub fn projection_payload(state: &StatBlockActionOrderingState) -> String {
    let protocol_holes = protocol_holes(&state.protocol);
    [
        format!("qStage={}", stage_ref(state.stage)),
        format!("qHoles={}", joined_or_none(&protocol_holes)),
        format!(
            "qLastOrderingError={}",
            ordering_error_ref(state.last_ordering_error)
        ),
        format!(
            "qMultiattackDispatchesAvailable={}",
            state.multiattack_dispatches_available
        ),
        format!(
            "qRechargeActionAvailable={}",
            state.recharge_action_available
        ),
        format!("qUsesRolledDamage={}", state.uses_rolled_damage),
        format!("protocolResult={}", protocol_result_ref(&state.protocol)),
        format!(
            "protocolInvalidReason={}",
            protocol_invalid_reason_ref(&state.protocol)
        ),
        format!("protocolHoles={}", joined_or_none(&protocol_holes)),
    ]
    .join("\n")
}

fn stage_ref(stage: StatBlockActionFrontierStage) -> &'static str {
    match stage {
        StatBlockActionFrontierStage::ActSelection => "StatBlockActSelectionStage",
        StatBlockActionFrontierStage::AttackTargetChoice => "StatBlockAttackTargetChoiceStage",
        StatBlockActionFrontierStage::AttackRoll => "StatBlockAttackRollStage",
        StatBlockActionFrontierStage::DamageDice => "StatBlockDamageDiceStage",
        StatBlockActionFrontierStage::RechargeRoll => "StatBlockRechargeRollStage",
        StatBlockActionFrontierStage::Resolved => "StatBlockResolvedStage",
    }
}

fn ordering_error_ref(error: Option<StatBlockActionFillOrderingError>) -> &'static str {
    match error {
        Some(StatBlockActionFillOrderingError::TargetChoiceRequired) => {
            "statBlockTargetChoiceRequired"
        }
        Some(StatBlockActionFillOrderingError::AttackRollRequired) => "statBlockAttackRollRequired",
        Some(StatBlockActionFillOrderingError::RechargeRollRequired) => {
            "statBlockRechargeRollRequired"
        }
        None => "",
    }
}

fn protocol_result_ref(protocol: &StatBlockActionOrderingProtocol) -> &'static str {
    match protocol {
        StatBlockActionOrderingProtocol::Init => "init",
        StatBlockActionOrderingProtocol::NeedsHoles(_) => "needsHoles",
        StatBlockActionOrderingProtocol::Resolved => "resolved",
        StatBlockActionOrderingProtocol::Invalid { .. } => "invalid",
    }
}

fn protocol_invalid_reason_ref(protocol: &StatBlockActionOrderingProtocol) -> &'static str {
    match protocol {
        StatBlockActionOrderingProtocol::Invalid {
            reason: StatBlockActionInvalidReason::InvalidFill,
            ..
        } => "WInvalidFill",
        StatBlockActionOrderingProtocol::Init
        | StatBlockActionOrderingProtocol::NeedsHoles(_)
        | StatBlockActionOrderingProtocol::Resolved => "",
    }
}

fn protocol_holes(protocol: &StatBlockActionOrderingProtocol) -> Vec<&'static str> {
    match protocol {
        StatBlockActionOrderingProtocol::NeedsHoles(holes) => holes.iter().map(hole_ref).collect(),
        StatBlockActionOrderingProtocol::Invalid { holes, .. } => {
            holes.iter().map(hole_ref).collect()
        }
        StatBlockActionOrderingProtocol::Init | StatBlockActionOrderingProtocol::Resolved => {
            Vec::new()
        }
    }
}

fn hole_ref(hole: &StatBlockActionHoleKind) -> &'static str {
    match hole {
        StatBlockActionHoleKind::TargetChoice => "TargetChoiceHoleKind",
        StatBlockActionHoleKind::AttackRoll => "AttackRollHoleKind",
        StatBlockActionHoleKind::RolledDice => "RolledDiceHoleKind",
        StatBlockActionHoleKind::StatBlockRechargeRoll => "StatBlockRechargeRollHoleKind",
    }
}

fn joined_or_none(values: &[&str]) -> String {
    if values.is_empty() {
        "none".to_string()
    } else {
        values.join(",")
    }
}
