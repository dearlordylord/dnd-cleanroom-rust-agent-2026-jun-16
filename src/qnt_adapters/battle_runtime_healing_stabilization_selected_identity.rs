use crate::rules::hit_points::{
    healing_stabilization_initial_state, resolve_spare_the_dying_stabilization,
    HealingStabilizationProtocol, HealingStabilizationScenarioOutcome, HealingStabilizationState,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HealingStabilizationSelectedIdentityWitness {
    pub target_hp: i16,
    pub target_stable: bool,
    pub target_unconscious: bool,
    pub target_death_successes: i16,
    pub target_death_failures: i16,
    pub action_available: bool,
    pub scenario_outcome: &'static str,
    pub protocol_result: &'static str,
    pub protocol_holes: Vec<&'static str>,
}

pub const BRANCH_ACTIONS: [&str; 1] = ["doResolveSpareTheDyingStable"];

pub fn replay_observed_action(
    observed_action_taken: &str,
) -> HealingStabilizationSelectedIdentityWitness {
    match observed_action_taken {
        "doResolveSpareTheDyingStable" => witness_from_state(resolved_spare_the_dying()),
        action => panic!("unsupported mbt::actionTaken {action}"),
    }
}

pub fn expected_witness(
    observed_action_taken: &str,
) -> HealingStabilizationSelectedIdentityWitness {
    replay_observed_action(observed_action_taken)
}

pub fn projection_payload(witness: &HealingStabilizationSelectedIdentityWitness) -> String {
    [
        format!("qTargetHp={}", witness.target_hp),
        format!("qTargetStable={}", witness.target_stable),
        format!("qTargetUnconscious={}", witness.target_unconscious),
        format!("qTargetDeathSuccesses={}", witness.target_death_successes),
        format!("qTargetDeathFailures={}", witness.target_death_failures),
        format!("qActionAvailable={}", witness.action_available),
        format!("qScenarioOutcome={}", witness.scenario_outcome),
        format!("protocolResult={}", witness.protocol_result),
        format!("protocolHoles={}", joined_or_none(&witness.protocol_holes)),
    ]
    .join("\n")
}

fn resolved_spare_the_dying() -> HealingStabilizationState {
    resolve_spare_the_dying_stabilization(healing_stabilization_initial_state())
}

fn witness_from_state(
    state: HealingStabilizationState,
) -> HealingStabilizationSelectedIdentityWitness {
    HealingStabilizationSelectedIdentityWitness {
        target_hp: state.death_saving_throw.target_hp,
        target_stable: state.death_saving_throw.target_stable,
        target_unconscious: state.death_saving_throw.target_unconscious,
        target_death_successes: state.death_saving_throw.target_death_successes,
        target_death_failures: state.death_saving_throw.target_death_failures,
        action_available: state.action_available,
        scenario_outcome: scenario_outcome_ref(state.scenario_outcome),
        protocol_result: protocol_result_ref(state.protocol),
        protocol_holes: protocol_holes(state.protocol),
    }
}

fn scenario_outcome_ref(outcome: HealingStabilizationScenarioOutcome) -> &'static str {
    match outcome {
        HealingStabilizationScenarioOutcome::Init => "Init",
        HealingStabilizationScenarioOutcome::Resolved => "Resolved",
    }
}

fn protocol_result_ref(protocol: HealingStabilizationProtocol) -> &'static str {
    match protocol {
        HealingStabilizationProtocol::Init => "init",
        HealingStabilizationProtocol::Resolved => "resolved",
    }
}

fn protocol_holes(protocol: HealingStabilizationProtocol) -> Vec<&'static str> {
    match protocol {
        HealingStabilizationProtocol::Init => vec!["WitnessProtocolHole"],
        HealingStabilizationProtocol::Resolved => vec![],
    }
}

fn joined_or_none(values: &[&'static str]) -> String {
    if values.is_empty() {
        "none".to_string()
    } else {
        values.join(",")
    }
}
