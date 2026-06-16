use crate::rules::weapon_mastery_selected_identity::{
    resolve_cleave_mastery_property_second_target_hit, resolve_sap_mastery_property_hit,
    resolve_topple_mastery_property_failed_saving_throw, WeaponMasteryRuntimeHole,
    WeaponMasteryRuntimeOutcome, WeaponMasteryRuntimeProtocol, WeaponMasterySelectedIdentityState,
};

pub const BRANCH_ACTIONS: [&str; 3] = [
    "doResolveSapMasteryPropertyHit",
    "doResolveToppleMasteryPropertyFailedSavingThrow",
    "doResolveCleaveMasteryPropertySecondTargetHit",
];

pub fn replay_observed_action(observed_action_taken: &str) -> WeaponMasterySelectedIdentityState {
    match observed_action_taken {
        "doResolveSapMasteryPropertyHit" => resolve_sap_mastery_property_hit(),
        "doResolveToppleMasteryPropertyFailedSavingThrow" => {
            resolve_topple_mastery_property_failed_saving_throw()
        }
        "doResolveCleaveMasteryPropertySecondTargetHit" => {
            resolve_cleave_mastery_property_second_target_hit()
        }
        action => panic!("unsupported mbt::actionTaken {action}"),
    }
}

pub fn expected_witness(observed_action_taken: &str) -> WeaponMasterySelectedIdentityState {
    replay_observed_action(observed_action_taken)
}

pub fn projection_payload(state: &WeaponMasterySelectedIdentityState) -> String {
    let protocol_holes = protocol_holes(&state.protocol);
    [
        format!("qPrimaryTargetHp={}", state.primary_target_hit_points),
        format!("qSecondTargetHp={}", state.second_target_hit_points),
        format!("qActionAvailable={}", state.action_available),
        format!(
            "qPrimaryTargetHasSapEffect={}",
            state.primary_target_has_sap_effect
        ),
        format!("qPrimaryTargetProne={}", state.primary_target_prone),
        format!("qCleaveUsed={}", state.cleave_used),
        format!("qLastResult={}", outcome_ref(state.outcome)),
        format!("qHoles={}", joined_or_none(&protocol_holes)),
        format!("protocolResult={}", protocol_result_ref(&state.protocol)),
        format!("protocolHoles={}", joined_or_none(&protocol_holes)),
    ]
    .join("\n")
}

fn outcome_ref(outcome: WeaponMasteryRuntimeOutcome) -> &'static str {
    match outcome {
        WeaponMasteryRuntimeOutcome::Init => "Init",
        WeaponMasteryRuntimeOutcome::NeedsHoles => "NeedsHoles",
        WeaponMasteryRuntimeOutcome::Resolved => "Resolved",
    }
}

fn protocol_result_ref(protocol: &WeaponMasteryRuntimeProtocol) -> &'static str {
    match protocol {
        WeaponMasteryRuntimeProtocol::Init(_) => "init",
        WeaponMasteryRuntimeProtocol::NeedsHoles(_) => "needsHoles",
        WeaponMasteryRuntimeProtocol::Resolved => "resolved",
    }
}

fn protocol_holes(protocol: &WeaponMasteryRuntimeProtocol) -> Vec<&'static str> {
    match protocol {
        WeaponMasteryRuntimeProtocol::Init(holes)
        | WeaponMasteryRuntimeProtocol::NeedsHoles(holes) => holes.iter().map(hole_ref).collect(),
        WeaponMasteryRuntimeProtocol::Resolved => Vec::new(),
    }
}

fn hole_ref(hole: &WeaponMasteryRuntimeHole) -> &'static str {
    match hole {
        WeaponMasteryRuntimeHole::WitnessProtocol => "WitnessProtocolHole",
    }
}

fn joined_or_none(values: &[&str]) -> String {
    if values.is_empty() {
        "none".to_string()
    } else {
        values.join(",")
    }
}
