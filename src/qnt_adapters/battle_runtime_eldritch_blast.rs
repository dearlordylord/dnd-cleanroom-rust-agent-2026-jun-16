use crate::rules::spell_shapes::{
    eldritch_blast_initial_state, fill_eldritch_blast_attack, fill_eldritch_blast_damage,
    fill_eldritch_blast_targets, reject_eldritch_blast_stale_after_resolved,
    EldritchBlastAttackFacts, EldritchBlastDamageFacts, EldritchBlastInvalidReason,
    EldritchBlastProtocol, EldritchBlastState,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EldritchBlastWitness {
    pub action_available: bool,
    pub target_hp: i16,
    pub resolved_beams: i16,
    pub protocol_result: &'static str,
    pub protocol_invalid_reason: &'static str,
    pub protocol_holes: Vec<&'static str>,
}

pub const BRANCH_ACTIONS: [&str; 8] = [
    "doFillFirstAttackHit",
    "doFillFirstAttackMiss",
    "doFillFirstDamageLow",
    "doFillSecondAttackHit",
    "doFillSecondAttackMiss",
    "doFillSecondDamageLow",
    "doFillTwoCreatureTargets",
    "doRejectStaleAfterResolved",
];

pub fn replay_observed_action(observed_action_taken: &str) -> EldritchBlastWitness {
    match observed_action_taken {
        "doFillFirstAttackHit" => witness_from_state(first_attack_hit()),
        "doFillFirstAttackMiss" => witness_from_state(first_attack_miss()),
        "doFillFirstDamageLow" => witness_from_state(first_damage_low()),
        "doFillSecondAttackHit" => witness_from_state(second_attack_hit()),
        "doFillSecondAttackMiss" => witness_from_state(second_attack_miss()),
        "doFillSecondDamageLow" => witness_from_state(second_damage_low()),
        "doFillTwoCreatureTargets" => witness_from_state(two_creature_targets()),
        "doRejectStaleAfterResolved" => witness_from_state(reject_stale_after_resolved()),
        action => panic!("unsupported mbt::actionTaken {action}"),
    }
}

pub fn expected_witness(observed_action_taken: &str) -> EldritchBlastWitness {
    replay_observed_action(observed_action_taken)
}

pub fn projection_payload(witness: &EldritchBlastWitness) -> String {
    [
        format!("qActionAvailable={}", witness.action_available),
        format!("qTargetHp={}", witness.target_hp),
        format!("qResolvedBeams={}", witness.resolved_beams),
        format!("protocolResult={}", witness.protocol_result),
        format!("protocolInvalidReason={}", witness.protocol_invalid_reason),
        format!("protocolHoles={}", joined_or_none(&witness.protocol_holes)),
    ]
    .join("\n")
}

fn two_creature_targets() -> EldritchBlastState {
    fill_eldritch_blast_targets(eldritch_blast_initial_state())
}

fn first_attack_miss() -> EldritchBlastState {
    fill_eldritch_blast_attack(
        two_creature_targets(),
        EldritchBlastAttackFacts { hit: false },
    )
}

fn first_attack_hit() -> EldritchBlastState {
    fill_eldritch_blast_attack(
        two_creature_targets(),
        EldritchBlastAttackFacts { hit: true },
    )
}

fn first_damage_low() -> EldritchBlastState {
    fill_eldritch_blast_damage(
        first_attack_hit(),
        EldritchBlastDamageFacts { damage_roll: 4 },
    )
}

fn second_attack_miss() -> EldritchBlastState {
    fill_eldritch_blast_attack(first_damage_low(), EldritchBlastAttackFacts { hit: false })
}

fn second_attack_hit() -> EldritchBlastState {
    fill_eldritch_blast_attack(first_damage_low(), EldritchBlastAttackFacts { hit: true })
}

fn second_damage_low() -> EldritchBlastState {
    fill_eldritch_blast_damage(
        second_attack_hit(),
        EldritchBlastDamageFacts { damage_roll: 4 },
    )
}

fn reject_stale_after_resolved() -> EldritchBlastState {
    reject_eldritch_blast_stale_after_resolved(second_damage_low())
}

fn witness_from_state(state: EldritchBlastState) -> EldritchBlastWitness {
    EldritchBlastWitness {
        action_available: state.action_available,
        target_hp: state.target_hit_points,
        resolved_beams: state.resolved_beams,
        protocol_result: protocol_result_ref(state.protocol),
        protocol_invalid_reason: protocol_invalid_reason_ref(state.protocol),
        protocol_holes: protocol_holes(state.protocol),
    }
}

fn protocol_result_ref(protocol: EldritchBlastProtocol) -> &'static str {
    match protocol {
        EldritchBlastProtocol::NeedsTargets | EldritchBlastProtocol::NeedsAttackRoll => {
            "needsHoles"
        }
        EldritchBlastProtocol::NeedsDamageRoll => "needsHoles",
        EldritchBlastProtocol::Resolved => "resolved",
        EldritchBlastProtocol::Invalid(_) => "invalid",
    }
}

fn protocol_invalid_reason_ref(protocol: EldritchBlastProtocol) -> &'static str {
    match protocol {
        EldritchBlastProtocol::Invalid(EldritchBlastInvalidReason::StaleSubject) => "WStaleSubject",
        EldritchBlastProtocol::Invalid(EldritchBlastInvalidReason::InvalidFill) => "WInvalidFill",
        EldritchBlastProtocol::NeedsTargets
        | EldritchBlastProtocol::NeedsAttackRoll
        | EldritchBlastProtocol::NeedsDamageRoll
        | EldritchBlastProtocol::Resolved => "",
    }
}

fn protocol_holes(protocol: EldritchBlastProtocol) -> Vec<&'static str> {
    match protocol {
        EldritchBlastProtocol::NeedsTargets => vec!["ObjectTargetChoice", "TargetChoice"],
        EldritchBlastProtocol::NeedsAttackRoll => vec!["AttackRoll"],
        EldritchBlastProtocol::NeedsDamageRoll => vec!["SpellDamageRoll"],
        EldritchBlastProtocol::Resolved | EldritchBlastProtocol::Invalid(_) => vec![],
    }
}

fn joined_or_none(values: &[&'static str]) -> String {
    if values.is_empty() {
        "none".to_string()
    } else {
        values.join(",")
    }
}
