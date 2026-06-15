use crate::rules::battle_features::{
    resolve_adrenaline_rush_bonus_action_dash, AdrenalineRushFacts, AdrenalineRushProjection,
    AdrenalineRushRejection, AdrenalineRushResult, BattleTurnFeatureState,
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
    replay_observed_action(observed_action_taken)
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

fn adrenaline_rush_dash() -> AdrenalineRushWitness {
    let projection = resolve_adrenaline_rush_bonus_action_dash(initial_facts());
    witness_from_projection(projection)
}

fn reject_second_dash() -> AdrenalineRushWitness {
    let first_dash = resolve_adrenaline_rush_bonus_action_dash(initial_facts());
    let second_dash = resolve_adrenaline_rush_bonus_action_dash(AdrenalineRushFacts {
        turn: first_dash.turn,
        ..initial_facts()
    });

    witness_from_projection(second_dash)
}

fn initial_facts() -> AdrenalineRushFacts {
    AdrenalineRushFacts {
        turn: BattleTurnFeatureState {
            actor_temporary_hit_points: 1,
            bonus_action_available: true,
            dash_bonus_feet: 0,
            feature_uses_remaining: 3,
        },
        speed_feet: 30,
        proficiency_bonus: 3,
        actor_owns_turn: true,
        actor_unconscious: false,
        actor_incapacitated: false,
    }
}

fn witness_from_projection(projection: AdrenalineRushProjection) -> AdrenalineRushWitness {
    let (protocol_result, invalid_reason) = protocol_projection(projection.result);

    AdrenalineRushWitness {
        actor_temporary_hit_points: projection.turn.actor_temporary_hit_points,
        bonus_action_available: projection.turn.bonus_action_available,
        dash_bonus_feet: projection.turn.dash_bonus_feet,
        feature_uses_remaining: projection.turn.feature_uses_remaining,
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

fn invalid_reason_ref(reason: AdrenalineRushRejection) -> &'static str {
    match reason {
        AdrenalineRushRejection::WrongActor => "wrongActor",
        AdrenalineRushRejection::StaleSubject => "staleSubject",
        AdrenalineRushRejection::NoUsesRemaining => "noUsesRemaining",
        AdrenalineRushRejection::InvalidFill => "invalidFill",
        AdrenalineRushRejection::UnableToAct => "unableToAct",
    }
}
