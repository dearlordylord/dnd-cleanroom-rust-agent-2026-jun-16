use crate::rules::hit_points::{
    death_saving_throw_initial_state, discover_death_saving_throw, fill_death_saving_throw,
    reject_wrong_actor_after_resolved, DeathSavingThrowFacts, DeathSavingThrowInvalidReason,
    DeathSavingThrowProtocol, DeathSavingThrowState, DeathSavingThrowTurnRole,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DeathSavingThrowWitness {
    pub current_turn_role: &'static str,
    pub target_hp: i16,
    pub target_unconscious: bool,
    pub target_stable: bool,
    pub target_dead: bool,
    pub target_death_successes: i16,
    pub target_death_failures: i16,
    pub protocol_result: &'static str,
    pub protocol_invalid_reason: &'static str,
    pub protocol_holes: Vec<&'static str>,
}

pub const BRANCH_ACTIONS: [&str; 3] = [
    "doDiscoverEndTurnDeathSavingThrow",
    "doFillDeathSavingThrow",
    "doRejectWrongActorEndTurnAfterResolved",
];

pub const FILL_SAMPLE_NATURAL_D20S: [i16; 4] = [1, 5, 10, 20];

pub fn replay_observed_action(observed_action_taken: &str) -> DeathSavingThrowWitness {
    match observed_action_taken {
        "doDiscoverEndTurnDeathSavingThrow" => witness_from_state(discovered()),
        "doFillDeathSavingThrow" => replay_fill_death_saving_throw(10),
        "doRejectWrongActorEndTurnAfterResolved" => witness_from_state(reject_wrong_actor()),
        action => panic!("unsupported mbt::actionTaken {action}"),
    }
}

pub fn replay_fill_death_saving_throw(natural_d20: i16) -> DeathSavingThrowWitness {
    witness_from_state(filled(natural_d20))
}

pub fn expected_witness(observed_action_taken: &str) -> DeathSavingThrowWitness {
    replay_observed_action(observed_action_taken)
}

pub fn projection_payload(witness: &DeathSavingThrowWitness) -> String {
    [
        format!("currentTurnRole={}", witness.current_turn_role),
        format!("targetHp={}", witness.target_hp),
        format!("targetUnconscious={}", witness.target_unconscious),
        format!("targetStable={}", witness.target_stable),
        format!("targetDead={}", witness.target_dead),
        format!("targetDeathSuccesses={}", witness.target_death_successes),
        format!("targetDeathFailures={}", witness.target_death_failures),
        format!("protocolResult={}", witness.protocol_result),
        format!("protocolInvalidReason={}", witness.protocol_invalid_reason),
        format!("protocolHoles={}", joined_or_none(&witness.protocol_holes)),
    ]
    .join("\n")
}

fn discovered() -> DeathSavingThrowState {
    discover_death_saving_throw(death_saving_throw_initial_state())
}

fn filled(natural_d20: i16) -> DeathSavingThrowState {
    fill_death_saving_throw(discovered(), DeathSavingThrowFacts { natural_d20 })
}

fn reject_wrong_actor() -> DeathSavingThrowState {
    reject_wrong_actor_after_resolved(filled(20))
}

fn witness_from_state(state: DeathSavingThrowState) -> DeathSavingThrowWitness {
    DeathSavingThrowWitness {
        current_turn_role: turn_role_ref(state.current_turn_role),
        target_hp: state.target_hp,
        target_unconscious: state.target_unconscious,
        target_stable: state.target_stable,
        target_dead: state.target_dead,
        target_death_successes: state.target_death_successes,
        target_death_failures: state.target_death_failures,
        protocol_result: protocol_result_ref(state.protocol),
        protocol_invalid_reason: protocol_invalid_reason_ref(state.protocol),
        protocol_holes: protocol_holes(state.protocol),
    }
}

fn turn_role_ref(role: DeathSavingThrowTurnRole) -> &'static str {
    match role {
        DeathSavingThrowTurnRole::Actor => "actor",
        DeathSavingThrowTurnRole::Target => "target",
    }
}

fn protocol_result_ref(protocol: DeathSavingThrowProtocol) -> &'static str {
    match protocol {
        DeathSavingThrowProtocol::Init => "init",
        DeathSavingThrowProtocol::NeedsSavingThrow => "needsHoles",
        DeathSavingThrowProtocol::Resolved => "resolved",
        DeathSavingThrowProtocol::Invalid(_) => "invalid",
    }
}

fn protocol_invalid_reason_ref(protocol: DeathSavingThrowProtocol) -> &'static str {
    match protocol {
        DeathSavingThrowProtocol::Invalid(DeathSavingThrowInvalidReason::WrongActor) => {
            "WWrongActor"
        }
        DeathSavingThrowProtocol::Invalid(DeathSavingThrowInvalidReason::InvalidFill) => {
            "WInvalidFill"
        }
        DeathSavingThrowProtocol::Init
        | DeathSavingThrowProtocol::NeedsSavingThrow
        | DeathSavingThrowProtocol::Resolved => "",
    }
}

fn protocol_holes(protocol: DeathSavingThrowProtocol) -> Vec<&'static str> {
    match protocol {
        DeathSavingThrowProtocol::NeedsSavingThrow => vec!["DeathSavingThrow"],
        DeathSavingThrowProtocol::Init
        | DeathSavingThrowProtocol::Resolved
        | DeathSavingThrowProtocol::Invalid(_) => vec![],
    }
}

fn joined_or_none(values: &[&'static str]) -> String {
    if values.is_empty() {
        "none".to_string()
    } else {
        values.join(",")
    }
}
