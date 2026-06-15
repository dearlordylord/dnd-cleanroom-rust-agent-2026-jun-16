use crate::rules::battle_features::{
    dragonborn_breath_weapon_initial_state, reject_dragonborn_breath_weapon_invalid_damage_roll,
    reject_dragonborn_breath_weapon_mismatched_area,
    reject_dragonborn_breath_weapon_missing_resource, resolve_dragonborn_breath_weapon,
    BreathWeaponAreaShape, DragonbornBreathWeaponFacts, DragonbornBreathWeaponInvalidReason,
    DragonbornBreathWeaponProtocol, DragonbornBreathWeaponScenarioOutcome,
    DragonbornBreathWeaponState,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DragonbornBreathWeaponWitness {
    pub target_hp: i16,
    pub second_target_hp: i16,
    pub breath_weapon_uses_remaining: i16,
    pub action_resources_remaining: i16,
    pub scenario_outcome: &'static str,
    pub protocol_result: &'static str,
    pub protocol_invalid_reason: &'static str,
    pub protocol_holes: Vec<&'static str>,
}

pub const BRANCH_ACTIONS: [&str; 5] = [
    "doResolveBreathWeapon",
    "doOpenExtraAttackSlot",
    "doRejectMissingResource",
    "doRejectMismatchedArea",
    "doRejectInvalidDamageRoll",
];

pub fn replay_observed_action(observed_action_taken: &str) -> DragonbornBreathWeaponWitness {
    match observed_action_taken {
        "doResolveBreathWeapon" => witness_from_state(resolved()),
        "doOpenExtraAttackSlot" => witness_from_state(open_extra_attack_slot()),
        "doRejectMissingResource" => witness_from_state(reject_missing_resource()),
        "doRejectMismatchedArea" => witness_from_state(reject_mismatched_area()),
        "doRejectInvalidDamageRoll" => witness_from_state(reject_invalid_damage_roll()),
        action => panic!("unsupported mbt::actionTaken {action}"),
    }
}

pub fn expected_witness(observed_action_taken: &str) -> DragonbornBreathWeaponWitness {
    replay_observed_action(observed_action_taken)
}

pub fn projection_payload(witness: &DragonbornBreathWeaponWitness) -> String {
    [
        format!("qTargetHp={}", witness.target_hp),
        format!("qSecondTargetHp={}", witness.second_target_hp),
        format!(
            "qBreathWeaponUsesRemaining={}",
            witness.breath_weapon_uses_remaining
        ),
        format!(
            "qActionResourcesRemaining={}",
            witness.action_resources_remaining
        ),
        format!("qScenarioOutcome={}", witness.scenario_outcome),
        format!("protocolResult={}", witness.protocol_result),
        format!("protocolInvalidReason={}", witness.protocol_invalid_reason),
        format!("protocolHoles={}", joined_or_none(&witness.protocol_holes)),
    ]
    .join("\n")
}

fn resolved() -> DragonbornBreathWeaponState {
    resolve_dragonborn_breath_weapon(
        dragonborn_breath_weapon_initial_state(),
        DragonbornBreathWeaponFacts {
            character_level: 1,
            damage_roll: 10,
            target_saving_throw_succeeded: false,
            second_target_in_area: true,
            second_target_saving_throw_succeeded: true,
            area_shape: BreathWeaponAreaShape::Cone15Feet,
            expected_area_shape: BreathWeaponAreaShape::Cone15Feet,
            opens_extra_attack_slot: false,
        },
    )
}

fn open_extra_attack_slot() -> DragonbornBreathWeaponState {
    resolve_dragonborn_breath_weapon(
        dragonborn_breath_weapon_initial_state(),
        DragonbornBreathWeaponFacts {
            character_level: 1,
            damage_roll: 10,
            target_saving_throw_succeeded: false,
            second_target_in_area: false,
            second_target_saving_throw_succeeded: true,
            area_shape: BreathWeaponAreaShape::Cone15Feet,
            expected_area_shape: BreathWeaponAreaShape::Cone15Feet,
            opens_extra_attack_slot: true,
        },
    )
}

fn reject_missing_resource() -> DragonbornBreathWeaponState {
    reject_dragonborn_breath_weapon_missing_resource(dragonborn_breath_weapon_initial_state())
}

fn reject_mismatched_area() -> DragonbornBreathWeaponState {
    reject_dragonborn_breath_weapon_mismatched_area(dragonborn_breath_weapon_initial_state())
}

fn reject_invalid_damage_roll() -> DragonbornBreathWeaponState {
    reject_dragonborn_breath_weapon_invalid_damage_roll(dragonborn_breath_weapon_initial_state())
}

fn witness_from_state(state: DragonbornBreathWeaponState) -> DragonbornBreathWeaponWitness {
    DragonbornBreathWeaponWitness {
        target_hp: state.target_hit_points,
        second_target_hp: state.second_target_hit_points,
        breath_weapon_uses_remaining: state.breath_weapon_uses_remaining,
        action_resources_remaining: state.attack_action_attacks_remaining,
        scenario_outcome: scenario_outcome_ref(state.scenario_outcome),
        protocol_result: protocol_result_ref(state.protocol),
        protocol_invalid_reason: protocol_invalid_reason_ref(state.protocol),
        protocol_holes: protocol_holes(state.protocol),
    }
}

fn scenario_outcome_ref(outcome: DragonbornBreathWeaponScenarioOutcome) -> &'static str {
    match outcome {
        DragonbornBreathWeaponScenarioOutcome::Init => "Init",
        DragonbornBreathWeaponScenarioOutcome::Resolved => "Resolved",
        DragonbornBreathWeaponScenarioOutcome::OpenedExtraAttack => "OpenedExtraAttack",
        DragonbornBreathWeaponScenarioOutcome::RejectMissingResource => "RejectMissingResource",
        DragonbornBreathWeaponScenarioOutcome::RejectMismatchedArea => "RejectMismatchedArea",
        DragonbornBreathWeaponScenarioOutcome::RejectInvalidDamageRoll => "RejectInvalidDamageRoll",
    }
}

fn protocol_result_ref(protocol: DragonbornBreathWeaponProtocol) -> &'static str {
    match protocol {
        DragonbornBreathWeaponProtocol::Init => "init",
        DragonbornBreathWeaponProtocol::Resolved => "resolved",
        DragonbornBreathWeaponProtocol::Invalid(_) => "invalid",
    }
}

fn protocol_invalid_reason_ref(protocol: DragonbornBreathWeaponProtocol) -> &'static str {
    match protocol {
        DragonbornBreathWeaponProtocol::Invalid(
            DragonbornBreathWeaponInvalidReason::InvalidFill,
        ) => "WInvalidFill",
        DragonbornBreathWeaponProtocol::Init | DragonbornBreathWeaponProtocol::Resolved => "",
    }
}

fn protocol_holes(protocol: DragonbornBreathWeaponProtocol) -> Vec<&'static str> {
    match protocol {
        DragonbornBreathWeaponProtocol::Init => vec!["DragonbornBreathWeapon"],
        DragonbornBreathWeaponProtocol::Resolved | DragonbornBreathWeaponProtocol::Invalid(_) => {
            vec![]
        }
    }
}

fn joined_or_none(values: &[&'static str]) -> String {
    if values.is_empty() {
        "none".to_string()
    } else {
        values.join(",")
    }
}
