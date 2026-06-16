use crate::rules::zero_hit_point_mid_resolution::{
    resolve_eldritch_blast_zero_hit_point_mid_resolution, ZeroHitPointMidResolutionHole,
    ZeroHitPointMidResolutionProtocol, ZeroHitPointMidResolutionScenario,
    ZeroHitPointMidResolutionState,
};

pub const BRANCH_ACTIONS: [&str; 1] = ["doResolveEldritchBlast"];

pub fn replay_observed_action(observed_action_taken: &str) -> ZeroHitPointMidResolutionState {
    match observed_action_taken {
        "doResolveEldritchBlast" => resolve_eldritch_blast_zero_hit_point_mid_resolution(),
        action => panic!("unsupported mbt::actionTaken {action}"),
    }
}

pub fn expected_witness(observed_action_taken: &str) -> ZeroHitPointMidResolutionState {
    replay_observed_action(observed_action_taken)
}

pub fn projection_payload(state: &ZeroHitPointMidResolutionState) -> String {
    let protocol_holes = protocol_holes(&state.protocol);
    [
        format!("qScenario={}", scenario_ref(state.scenario)),
        format!("qSourceHp={}", state.source_hit_points),
        format!("qSourceUnconscious={}", state.source_unconscious),
        format!("qSourceConcentrating={}", state.source_concentrating),
        format!("qShieldOfFaithPresent={}", state.shield_of_faith_present),
        format!("qProtectedTargetHp={}", state.protected_target_hit_points),
        format!("qSourceDamage={}", state.source_damage),
        format!("qProtectedTargetDamage={}", state.protected_target_damage),
        format!(
            "qProtectedTargetDamageIfShieldOfFaithRemained={}",
            state.protected_target_damage_if_shield_of_faith_remained
        ),
        format!(
            "qZeroHpAppliedBeforeSecondBeam={}",
            state.zero_hit_points_applied_before_second_beam
        ),
        format!(
            "qTeardownBeforeSecondBeam={}",
            state.teardown_before_second_beam
        ),
        format!(
            "qRemainderUsedPostTeardownState={}",
            state.remainder_used_post_teardown_state
        ),
        format!("qHoles={}", joined_or_none(&protocol_holes)),
        format!("protocolResult={}", protocol_result_ref(&state.protocol)),
        format!("protocolHoles={}", joined_or_none(&protocol_holes)),
    ]
    .join("\n")
}

fn scenario_ref(scenario: ZeroHitPointMidResolutionScenario) -> &'static str {
    match scenario {
        ZeroHitPointMidResolutionScenario::Init => "Init",
        ZeroHitPointMidResolutionScenario::SpellAttackSequenceResolved => {
            "SpellAttackSequenceResolved"
        }
    }
}

fn protocol_result_ref(protocol: &ZeroHitPointMidResolutionProtocol) -> &'static str {
    match protocol {
        ZeroHitPointMidResolutionProtocol::Init(_) => "init",
        ZeroHitPointMidResolutionProtocol::Resolved => "resolved",
    }
}

fn protocol_holes(protocol: &ZeroHitPointMidResolutionProtocol) -> Vec<&'static str> {
    match protocol {
        ZeroHitPointMidResolutionProtocol::Init(holes) => holes.iter().map(hole_ref).collect(),
        ZeroHitPointMidResolutionProtocol::Resolved => Vec::new(),
    }
}

fn hole_ref(hole: &ZeroHitPointMidResolutionHole) -> &'static str {
    match hole {
        ZeroHitPointMidResolutionHole::ZeroHitPointMidResolution => "ZeroHitPointMidResolution",
    }
}

fn joined_or_none(values: &[&str]) -> String {
    if values.is_empty() {
        "none".to_string()
    } else {
        values.join(",")
    }
}
