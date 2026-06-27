use crate::rules::attack_damage_disposition::{
    reject_ranged_knock_out_disposition, resolve_melee_knock_out_disposition,
    AttackDamageDispositionOutcome, AttackDamageDispositionState,
};

pub const BRANCH_ACTIONS: [&str; 2] = ["doMeleeKnockOut", "doRejectRangedKnockOut"];

pub fn replay_observed_action(observed_action_taken: &str) -> AttackDamageDispositionState {
    match observed_action_taken {
        "doMeleeKnockOut" => resolve_melee_knock_out_disposition(),
        "doRejectRangedKnockOut" => reject_ranged_knock_out_disposition(),
        action => panic!("unsupported mbt::actionTaken {action}"),
    }
}

pub fn expected_witness(observed_action_taken: &str) -> AttackDamageDispositionState {
    replay_observed_action(observed_action_taken)
}

pub fn projection_payload(state: &AttackDamageDispositionState) -> String {
    [
        format!("qLastScenario={}", scenario_ref(state.outcome)),
        format!("qAccepted={}", state.accepted),
        format!("qTargetHp={}", state.target_hit_points),
        format!("qTargetUnconscious={}", state.target_unconscious),
        format!("qTargetDead={}", state.target_dead),
        format!(
            "qKnockOutRecovery={}",
            state.knock_out_recovery_ends_when_hit_points_regained
        ),
        format!("qReplayIndex={}", state.replay_index),
        format!("qComponentRoute={}", component_route_payload()),
    ]
    .join("\n")
}

pub fn component_route_payload() -> String {
    [
        "RuleCoreComponentParseInput(RuleCoreAttackDamageDispositionOwner)",
        "RuleCoreComponentAdmitInput(RuleCoreAttackDamageDispositionOwner)",
        "RuleCoreComponentCall(RuleCoreAttackDamageDispositionOwner)",
        "RuleCoreComponentProjectResult(RuleCoreAttackDamageDispositionOwner)",
    ]
    .join(",")
}

fn scenario_ref(outcome: AttackDamageDispositionOutcome) -> &'static str {
    match outcome {
        AttackDamageDispositionOutcome::Initial => "init",
        AttackDamageDispositionOutcome::MeleeKnockOutAccepted => "melee-knock-out",
        AttackDamageDispositionOutcome::RangedKnockOutRejected => "ranged-knock-out-rejected",
    }
}
