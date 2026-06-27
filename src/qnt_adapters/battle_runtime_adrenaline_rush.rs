use crate::rules::battle_features::AdrenalineRushResult;
use crate::rules::battle_reducer_spine::{
    bonus_action_dash_temporary_hit_points_feature_turn_state,
    resolve_bonus_action_dash_temporary_hit_points_feature_battle, start_standard_battle,
    trace_adrenaline_rush_bonus_action_dash,
    trace_adrenaline_rush_bonus_action_dash_then_reject_second_dash, Actor,
    BattleResolutionInvalidReason, BattleResolutionResult,
};

use super::battle_runtime_reducer_route::{
    reducer_route_events_from_battle_trace, route_discover_battle_acts_from_route_holes,
    route_resolve_battle_subject_without_fill_from_route_result, route_start_battle,
    ReducerRouteEvent, ReducerRouteHoleKind, ReducerRouteOwnerGroup, ReducerRouteResolutionOutcome,
    ReducerRouteSubjectFamily,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AdrenalineRushWitness {
    pub actor_temporary_hit_points: i16,
    pub bonus_action_available: bool,
    pub dash_bonus_feet: i16,
    pub feature_uses_remaining: i16,
    pub protocol_result: &'static str,
    pub invalid_reason: &'static str,
    pub protocol_hole_count: usize,
}

pub const BRANCH_ACTIONS: [&str; 2] = ["doAdrenalineRushDash", "doRejectSecondDash"];

pub fn replay_observed_action(observed_action_taken: &str) -> AdrenalineRushWitness {
    match observed_action_taken {
        "doAdrenalineRushDash" => adrenaline_rush_dash(),
        "doRejectSecondDash" => reject_second_dash(),
        action => panic!("unsupported mbt::actionTaken {action}"),
    }
}

pub fn expected_witness(observed_action_taken: &str) -> AdrenalineRushWitness {
    match observed_action_taken {
        "doAdrenalineRushDash" => AdrenalineRushWitness {
            actor_temporary_hit_points: 3,
            bonus_action_available: false,
            dash_bonus_feet: 30,
            feature_uses_remaining: 2,
            protocol_result: "resolved",
            invalid_reason: "none",
            protocol_hole_count: 0,
        },
        "doRejectSecondDash" => AdrenalineRushWitness {
            actor_temporary_hit_points: 3,
            bonus_action_available: false,
            dash_bonus_feet: 30,
            feature_uses_remaining: 2,
            protocol_result: "invalid",
            invalid_reason: "staleSubject",
            protocol_hole_count: 0,
        },
        action => panic!("unsupported mbt::actionTaken {action}"),
    }
}

pub fn replay_observed_route(observed_action_taken: &str) -> Vec<ReducerRouteEvent> {
    match observed_action_taken {
        "doAdrenalineRushDash" => {
            let (_result, trace) = trace_adrenaline_rush_bonus_action_dash(
                adrenaline_rush_fixture_battle(),
                Actor::Fighter,
                3,
            );
            reducer_route_events_from_battle_trace(&trace)
        }
        "doRejectSecondDash" => {
            let (_result, trace) = trace_adrenaline_rush_bonus_action_dash_then_reject_second_dash(
                adrenaline_rush_fixture_battle(),
                Actor::Fighter,
                3,
            );
            reducer_route_events_from_battle_trace(&trace)
        }
        action => panic!("unsupported mbt::actionTaken {action}"),
    }
}

pub fn expected_route(observed_action_taken: &str) -> Vec<ReducerRouteEvent> {
    match observed_action_taken {
        "doAdrenalineRushDash" => {
            let mut route = expected_initial_feature_dash_route();
            route.extend([
                expected_feature_dash_resolution(ReducerRouteOwnerGroup::ActionEconomy),
                expected_feature_dash_resolution(ReducerRouteOwnerGroup::FeatureResource),
                expected_feature_dash_resolution(ReducerRouteOwnerGroup::MovementResource),
                expected_feature_dash_resolution(ReducerRouteOwnerGroup::TemporaryHitPoint),
            ]);
            route
        }
        "doRejectSecondDash" => {
            let mut route = expected_route("doAdrenalineRushDash");
            route.push(route_resolve_battle_subject_without_fill_from_route_result(
                ReducerRouteSubjectFamily::UnitFeatureBonusAction,
                ReducerRouteResolutionOutcome::Invalid(BattleResolutionInvalidReason::StaleSubject),
                Vec::new(),
                ReducerRouteOwnerGroup::ActionEconomy,
            ));
            route
        }
        action => panic!("unsupported mbt::actionTaken {action}"),
    }
}

pub fn projection_payload(witness: &AdrenalineRushWitness) -> String {
    [
        format!("actorTempHp={}", witness.actor_temporary_hit_points),
        format!("bonusActionAvailable={}", witness.bonus_action_available),
        format!("dashBonusFeet={}", witness.dash_bonus_feet),
        format!("featureUsesRemaining={}", witness.feature_uses_remaining),
        format!("protocolResult={}", witness.protocol_result),
        format!("invalidReason={}", witness.invalid_reason),
        format!("protocolHoleCount={}", witness.protocol_hole_count),
    ]
    .join("\n")
}

fn expected_initial_feature_dash_route() -> Vec<ReducerRouteEvent> {
    vec![
        route_start_battle(ReducerRouteOwnerGroup::ActionEconomy),
        route_discover_battle_acts_from_route_holes(
            ReducerRouteSubjectFamily::UnitFeatureBonusAction,
            Vec::<ReducerRouteHoleKind>::new(),
            ReducerRouteOwnerGroup::FeatureResource,
        ),
    ]
}

fn expected_feature_dash_resolution(owner: ReducerRouteOwnerGroup) -> ReducerRouteEvent {
    route_resolve_battle_subject_without_fill_from_route_result(
        ReducerRouteSubjectFamily::UnitFeatureBonusAction,
        ReducerRouteResolutionOutcome::Resolved,
        Vec::new(),
        owner,
    )
}

fn adrenaline_rush_dash() -> AdrenalineRushWitness {
    let state = adrenaline_rush_fixture_battle();
    let result =
        resolve_bonus_action_dash_temporary_hit_points_feature_battle(state, Actor::Fighter, 3);
    witness_from_battle_result(result, AdrenalineRushResult::Resolved)
}

fn reject_second_dash() -> AdrenalineRushWitness {
    let state = adrenaline_rush_fixture_battle();
    let first_dash =
        resolve_bonus_action_dash_temporary_hit_points_feature_battle(state, Actor::Fighter, 3)
            .into_resolved_state()
            .expect("first fixture dash resolves");
    let second_dash = resolve_bonus_action_dash_temporary_hit_points_feature_battle(
        first_dash,
        Actor::Fighter,
        3,
    );

    witness_from_battle_result(
        second_dash,
        AdrenalineRushResult::Invalid(
            crate::rules::battle_features::AdrenalineRushRejection::StaleSubject,
        ),
    )
}

fn adrenaline_rush_fixture_battle() -> crate::rules::battle_reducer_spine::BattleState {
    let mut state = start_standard_battle();
    state.fighter.temporary_hp = 1;
    state
}

fn witness_from_battle_result(
    result: BattleResolutionResult,
    feature_result: AdrenalineRushResult,
) -> AdrenalineRushWitness {
    let (protocol_result, invalid_reason) = match result.outcome() {
        crate::rules::battle_reducer_spine::BattleResolutionOutcome::Resolved => {
            protocol_projection(feature_result)
        }
        crate::rules::battle_reducer_spine::BattleResolutionOutcome::Invalid(reason) => {
            ("invalid", battle_invalid_reason_ref(reason))
        }
        crate::rules::battle_reducer_spine::BattleResolutionOutcome::NeedsHoles => {
            ("needsHoles", "none")
        }
    };
    let turn =
        bonus_action_dash_temporary_hit_points_feature_turn_state(result.state(), Actor::Fighter);

    AdrenalineRushWitness {
        actor_temporary_hit_points: turn.actor_temporary_hit_points,
        bonus_action_available: turn.bonus_action_available,
        dash_bonus_feet: turn.dash_bonus_feet,
        feature_uses_remaining: turn.feature_uses_remaining,
        protocol_result,
        invalid_reason,
        protocol_hole_count: 0,
    }
}

fn protocol_projection(result: AdrenalineRushResult) -> (&'static str, &'static str) {
    match result {
        AdrenalineRushResult::Resolved => ("resolved", "none"),
        AdrenalineRushResult::Invalid(reason) => ("invalid", invalid_reason_ref(reason)),
    }
}

fn invalid_reason_ref(
    reason: crate::rules::battle_features::AdrenalineRushRejection,
) -> &'static str {
    match reason {
        crate::rules::battle_features::AdrenalineRushRejection::WrongActor => "wrongActor",
        crate::rules::battle_features::AdrenalineRushRejection::StaleSubject => "staleSubject",
        crate::rules::battle_features::AdrenalineRushRejection::NoUsesRemaining => {
            "noUsesRemaining"
        }
        crate::rules::battle_features::AdrenalineRushRejection::InvalidFill => "invalidFill",
        crate::rules::battle_features::AdrenalineRushRejection::UnableToAct => "unableToAct",
    }
}

fn battle_invalid_reason_ref(reason: BattleResolutionInvalidReason) -> &'static str {
    match reason {
        BattleResolutionInvalidReason::InvalidFill => "invalidFill",
        BattleResolutionInvalidReason::MetamagicOptionEffectMismatch => {
            "metamagicOptionEffectMismatch"
        }
        BattleResolutionInvalidReason::StaleSubject => "staleSubject",
        BattleResolutionInvalidReason::WrongActor => "wrongActor",
        BattleResolutionInvalidReason::WrongTarget => "wrongTarget",
    }
}
