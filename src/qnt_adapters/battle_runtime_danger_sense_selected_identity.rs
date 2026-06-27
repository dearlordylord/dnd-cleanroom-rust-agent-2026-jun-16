use crate::rules::battle_features::{
    danger_sense_initial_state, project_danger_sense_dexterity_advantage,
    project_danger_sense_dexterity_advantage_observed, suppress_danger_sense_while_incapacitated,
    suppress_danger_sense_while_incapacitated_observed, DangerSenseProtocol,
    DangerSenseScenarioOutcome, DangerSenseState,
};

use super::battle_runtime_reducer_route::{
    reducer_route_events_from_battle_trace, route_discover_battle_acts_from_route_holes,
    route_resolve_battle_subject_from_route_result,
    route_resolve_battle_subject_without_fill_from_route_result, route_start_battle,
    ReducerRouteEvent, ReducerRouteFillKind, ReducerRouteHoleKind, ReducerRouteOwnerGroup,
    ReducerRouteResolutionOutcome, ReducerRouteSubjectFamily,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DangerSenseWitness {
    pub scenario_outcome: &'static str,
    pub source_unit_id: &'static str,
    pub dexterity_roll_mode_count: i16,
    pub constitution_roll_mode_count: i16,
    pub suppressed: bool,
    pub accepted: bool,
    pub protocol_result: &'static str,
    pub protocol_holes: Vec<&'static str>,
}

pub const BRANCH_ACTIONS: [&str; 2] = [
    "doProjectDangerSenseDexterityAdvantage",
    "doSuppressDangerSenseWhileIncapacitated",
];

pub fn replay_observed_action(observed_action_taken: &str) -> DangerSenseWitness {
    match observed_action_taken {
        "doProjectDangerSenseDexterityAdvantage" => witness_from_state(
            project_danger_sense_dexterity_advantage(danger_sense_initial_state()),
        ),
        "doSuppressDangerSenseWhileIncapacitated" => witness_from_state(
            suppress_danger_sense_while_incapacitated(danger_sense_initial_state()),
        ),
        action => panic!("unsupported mbt::actionTaken {action}"),
    }
}

pub fn expected_witness(observed_action_taken: &str) -> DangerSenseWitness {
    match observed_action_taken {
        "doProjectDangerSenseDexterityAdvantage" => DangerSenseWitness {
            scenario_outcome: "DangerSenseDexterityAdvantage",
            source_unit_id: "barbarian_danger_sense",
            dexterity_roll_mode_count: 1,
            constitution_roll_mode_count: 0,
            suppressed: false,
            accepted: true,
            protocol_result: "resolved",
            protocol_holes: Vec::new(),
        },
        "doSuppressDangerSenseWhileIncapacitated" => DangerSenseWitness {
            scenario_outcome: "DangerSenseIncapacitatedSuppressed",
            source_unit_id: "barbarian_danger_sense",
            dexterity_roll_mode_count: 0,
            constitution_roll_mode_count: 0,
            suppressed: true,
            accepted: true,
            protocol_result: "resolved",
            protocol_holes: Vec::new(),
        },
        action => panic!("unsupported mbt::actionTaken {action}"),
    }
}

pub fn replay_observed_route(observed_action_taken: &str) -> Vec<ReducerRouteEvent> {
    let (_state, trace) = match observed_action_taken {
        "doProjectDangerSenseDexterityAdvantage" => {
            project_danger_sense_dexterity_advantage_observed()
        }
        "doSuppressDangerSenseWhileIncapacitated" => {
            suppress_danger_sense_while_incapacitated_observed()
        }
        action => panic!("unsupported mbt::actionTaken {action}"),
    };
    reducer_route_events_from_battle_trace(&trace)
}

pub fn expected_route(observed_action_taken: &str) -> Vec<ReducerRouteEvent> {
    match observed_action_taken {
        "doProjectDangerSenseDexterityAdvantage" => vec![
            route_start_battle(ReducerRouteOwnerGroup::SavingThrowRollMode),
            route_discover_battle_acts_from_route_holes(
                ReducerRouteSubjectFamily::PassiveSavingThrowRollMode,
                vec![ReducerRouteHoleKind::SavingThrowOutcome],
                ReducerRouteOwnerGroup::SavingThrowRollMode,
            ),
            route_resolve_battle_subject_from_route_result(
                ReducerRouteSubjectFamily::PassiveSavingThrowRollMode,
                ReducerRouteFillKind::SavingThrowOutcome,
                ReducerRouteResolutionOutcome::Resolved,
                Vec::new(),
                ReducerRouteOwnerGroup::SavingThrowRollMode,
            ),
        ],
        "doSuppressDangerSenseWhileIncapacitated" => vec![
            route_start_battle(ReducerRouteOwnerGroup::SavingThrowRollMode),
            route_discover_battle_acts_from_route_holes(
                ReducerRouteSubjectFamily::PassiveSavingThrowRollMode,
                Vec::new(),
                ReducerRouteOwnerGroup::SavingThrowRollMode,
            ),
            route_resolve_battle_subject_without_fill_from_route_result(
                ReducerRouteSubjectFamily::PassiveSavingThrowRollMode,
                ReducerRouteResolutionOutcome::Resolved,
                Vec::new(),
                ReducerRouteOwnerGroup::ConditionLifecycle,
            ),
        ],
        action => panic!("unsupported mbt::actionTaken {action}"),
    }
}

pub fn projection_payload(witness: &DangerSenseWitness) -> String {
    [
        format!("scenarioOutcome={}", witness.scenario_outcome),
        format!("sourceUnitId={}", witness.source_unit_id),
        format!(
            "dexterityRollModeCount={}",
            witness.dexterity_roll_mode_count
        ),
        format!(
            "constitutionRollModeCount={}",
            witness.constitution_roll_mode_count
        ),
        format!("suppressed={}", witness.suppressed),
        format!("accepted={}", witness.accepted),
        format!("protocolResult={}", witness.protocol_result),
        format!("protocolHoles={}", joined_or_none(&witness.protocol_holes)),
    ]
    .join("\n")
}

fn witness_from_state(state: DangerSenseState) -> DangerSenseWitness {
    DangerSenseWitness {
        scenario_outcome: scenario_outcome_ref(state.scenario_outcome),
        source_unit_id: if state.feature_present {
            "barbarian_danger_sense"
        } else {
            "none"
        },
        dexterity_roll_mode_count: state.dexterity_roll_mode_count,
        constitution_roll_mode_count: state.constitution_roll_mode_count,
        suppressed: state.suppressed,
        accepted: state.accepted,
        protocol_result: protocol_result_ref(state.protocol),
        protocol_holes: protocol_holes(state.protocol),
    }
}

fn scenario_outcome_ref(outcome: DangerSenseScenarioOutcome) -> &'static str {
    match outcome {
        DangerSenseScenarioOutcome::Init => "Init",
        DangerSenseScenarioOutcome::DexterityAdvantage => "DangerSenseDexterityAdvantage",
        DangerSenseScenarioOutcome::IncapacitatedSuppressed => "DangerSenseIncapacitatedSuppressed",
    }
}

fn protocol_result_ref(protocol: DangerSenseProtocol) -> &'static str {
    match protocol {
        DangerSenseProtocol::Init => "init",
        DangerSenseProtocol::Resolved => "resolved",
    }
}

fn protocol_holes(protocol: DangerSenseProtocol) -> Vec<&'static str> {
    match protocol {
        DangerSenseProtocol::Init => vec!["DangerSense"],
        DangerSenseProtocol::Resolved => vec![],
    }
}

fn joined_or_none(values: &[&'static str]) -> String {
    if values.is_empty() {
        "none".to_string()
    } else {
        values.join(",")
    }
}
