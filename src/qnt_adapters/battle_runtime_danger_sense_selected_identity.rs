use crate::rules::battle_features::{
    danger_sense_initial_state, project_danger_sense_dexterity_advantage,
    suppress_danger_sense_while_incapacitated, DangerSenseProtocol, DangerSenseScenarioOutcome,
    DangerSenseState,
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
    replay_observed_action(observed_action_taken)
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
