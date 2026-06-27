use crate::rules::species_passive_traits::{
    creature_space_traversal_allowed, CreatureSpaceTraversalFacts, CreatureSpaceTraversalPermission,
};

use super::battle_runtime_reducer_route::ReducerRouteEvent;
use super::battle_runtime_species_passive_trait_selected_identity::{
    observed_passive_route_after_accepted_creature_space_movement,
    observed_passive_route_after_rejected_creature_space_movement,
    passive_route_after_accepted_creature_space_movement,
    passive_route_after_rejected_creature_space_movement,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HalflingNimblenessWitness {
    pub traversal_accepted: bool,
    pub accepted_movement_spent_feet: i16,
    pub occupied_stop_rejected: bool,
    pub missing_profile_rejected: bool,
    pub same_size_rejected: bool,
    pub scenario_outcome: &'static str,
    pub protocol_result: &'static str,
    pub protocol_holes: Vec<&'static str>,
}

pub const BRANCH_ACTIONS: [&str; 4] = [
    "doMoveThroughLargerCreatureSpace",
    "doRejectOccupiedStop",
    "doRejectMissingProfile",
    "doRejectSameSizeTraversal",
];

pub fn replay_observed_action(observed_action_taken: &str) -> HalflingNimblenessWitness {
    match observed_action_taken {
        "doMoveThroughLargerCreatureSpace" => movement_witness(
            "MoveThroughLargerCreatureSpace",
            CreatureSpaceTraversalFacts {
                permission: Some(CreatureSpaceTraversalPermission::LargerCreatureSpaceNoStop),
                mover_size_rank: 1,
                occupied_creature_size_rank: 2,
                stops_in_occupied_space: false,
            },
        ),
        "doRejectOccupiedStop" => movement_witness(
            "RejectOccupiedStop",
            CreatureSpaceTraversalFacts {
                permission: Some(CreatureSpaceTraversalPermission::LargerCreatureSpaceNoStop),
                mover_size_rank: 1,
                occupied_creature_size_rank: 2,
                stops_in_occupied_space: true,
            },
        ),
        "doRejectMissingProfile" => movement_witness(
            "RejectMissingProfile",
            CreatureSpaceTraversalFacts {
                permission: None,
                mover_size_rank: 1,
                occupied_creature_size_rank: 2,
                stops_in_occupied_space: false,
            },
        ),
        "doRejectSameSizeTraversal" => movement_witness(
            "RejectSameSizeTraversal",
            CreatureSpaceTraversalFacts {
                permission: Some(CreatureSpaceTraversalPermission::LargerCreatureSpaceNoStop),
                mover_size_rank: 1,
                occupied_creature_size_rank: 1,
                stops_in_occupied_space: false,
            },
        ),
        action => panic!("unsupported mbt::actionTaken {action}"),
    }
}

pub fn expected_witness(observed_action_taken: &str) -> HalflingNimblenessWitness {
    match observed_action_taken {
        "doMoveThroughLargerCreatureSpace" => HalflingNimblenessWitness {
            traversal_accepted: true,
            accepted_movement_spent_feet: 10,
            occupied_stop_rejected: false,
            missing_profile_rejected: false,
            same_size_rejected: false,
            scenario_outcome: "MoveThroughLargerCreatureSpace",
            protocol_result: "resolved",
            protocol_holes: Vec::new(),
        },
        "doRejectOccupiedStop" => HalflingNimblenessWitness {
            traversal_accepted: false,
            accepted_movement_spent_feet: 0,
            occupied_stop_rejected: true,
            missing_profile_rejected: false,
            same_size_rejected: false,
            scenario_outcome: "RejectOccupiedStop",
            protocol_result: "resolved",
            protocol_holes: Vec::new(),
        },
        "doRejectMissingProfile" => HalflingNimblenessWitness {
            traversal_accepted: false,
            accepted_movement_spent_feet: 0,
            occupied_stop_rejected: false,
            missing_profile_rejected: true,
            same_size_rejected: false,
            scenario_outcome: "RejectMissingProfile",
            protocol_result: "resolved",
            protocol_holes: Vec::new(),
        },
        "doRejectSameSizeTraversal" => HalflingNimblenessWitness {
            traversal_accepted: false,
            accepted_movement_spent_feet: 0,
            occupied_stop_rejected: false,
            missing_profile_rejected: false,
            same_size_rejected: true,
            scenario_outcome: "RejectSameSizeTraversal",
            protocol_result: "resolved",
            protocol_holes: Vec::new(),
        },
        action => panic!("unsupported mbt::actionTaken {action}"),
    }
}

pub fn replay_observed_route(observed_action_taken: &str) -> Vec<ReducerRouteEvent> {
    match replay_observed_action(observed_action_taken).scenario_outcome {
        "MoveThroughLargerCreatureSpace" => {
            observed_passive_route_after_accepted_creature_space_movement()
        }
        "RejectOccupiedStop" => observed_passive_route_after_rejected_creature_space_movement(1),
        "RejectMissingProfile" => observed_passive_route_after_rejected_creature_space_movement(2),
        "RejectSameSizeTraversal" => {
            observed_passive_route_after_rejected_creature_space_movement(3)
        }
        action => panic!("unsupported halfling nimbleness replay outcome {action}"),
    }
}

pub fn expected_route(observed_action_taken: &str) -> Vec<ReducerRouteEvent> {
    match observed_action_taken {
        "doMoveThroughLargerCreatureSpace" => {
            passive_route_after_accepted_creature_space_movement()
        }
        "doRejectOccupiedStop" => passive_route_after_rejected_creature_space_movement(1),
        "doRejectMissingProfile" => passive_route_after_rejected_creature_space_movement(2),
        "doRejectSameSizeTraversal" => passive_route_after_rejected_creature_space_movement(3),
        action => panic!("unsupported mbt::actionTaken {action}"),
    }
}

pub fn projection_payload(witness: &HalflingNimblenessWitness) -> String {
    [
        format!("traversalAccepted={}", witness.traversal_accepted),
        format!(
            "acceptedMovementSpentFeet={}",
            witness.accepted_movement_spent_feet
        ),
        format!("occupiedStopRejected={}", witness.occupied_stop_rejected),
        format!(
            "missingProfileRejected={}",
            witness.missing_profile_rejected
        ),
        format!("sameSizeRejected={}", witness.same_size_rejected),
        format!("scenarioOutcome={}", witness.scenario_outcome),
        format!("protocolResult={}", witness.protocol_result),
        format!("protocolHoles={}", joined_or_none(&witness.protocol_holes)),
    ]
    .join("\n")
}

fn movement_witness(
    scenario_outcome: &'static str,
    facts: CreatureSpaceTraversalFacts,
) -> HalflingNimblenessWitness {
    let accepted = creature_space_traversal_allowed(facts);
    HalflingNimblenessWitness {
        traversal_accepted: accepted,
        accepted_movement_spent_feet: if accepted { 10 } else { 0 },
        occupied_stop_rejected: !accepted && facts.stops_in_occupied_space,
        missing_profile_rejected: !accepted && facts.permission.is_none(),
        same_size_rejected: !accepted
            && facts.permission.is_some()
            && facts.occupied_creature_size_rank <= facts.mover_size_rank
            && !facts.stops_in_occupied_space,
        scenario_outcome,
        protocol_result: "resolved",
        protocol_holes: Vec::new(),
    }
}

fn joined_or_none(values: &[&'static str]) -> String {
    if values.is_empty() {
        "none".to_string()
    } else {
        values.join(",")
    }
}
