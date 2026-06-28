use crate::rules::battle_features::{
    BreathWeaponAreaShape, DragonbornBreathWeaponFacts, DragonbornBreathWeaponInvalidReason,
    DragonbornBreathWeaponProtocol, DragonbornBreathWeaponScenarioOutcome,
};
use crate::rules::battle_reducer_spine::{
    discover_battle_acts, discover_battle_acts_observed, dragonborn_breath_weapon_from_battle,
    resolve_battle_subject, resolve_battle_subject_observed, start_dragonborn_breath_weapon_battle,
    start_dragonborn_breath_weapon_battle_observed, with_dragonborn_breath_weapon_uses_remaining,
    BattleAttackActionAreaSaveDamageReplacementFill, BattleEntrypointTrace,
    BattleResolutionRequest, BattleState, BattleSubject, BattleSubjectKind,
};

use super::battle_runtime_reducer_route::{
    observed_reducer_route, ReducerRouteEvent, ReducerRouteFillKind, ReducerRouteHoleKind,
    ReducerRouteOwnerGroup, ReducerRouteResolutionOutcome, ReducerRouteSubjectFamily,
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
        "doResolveBreathWeapon" => witness_from_battle(resolved_state()),
        "doOpenExtraAttackSlot" => witness_from_battle(open_extra_attack_slot_state()),
        "doRejectMissingResource" => witness_from_battle(reject_missing_resource_state()),
        "doRejectMismatchedArea" => witness_from_battle(reject_mismatched_area_state()),
        "doRejectInvalidDamageRoll" => witness_from_battle(reject_invalid_damage_roll_state()),
        action => panic!("unsupported mbt::actionTaken {action}"),
    }
}

pub fn expected_witness(observed_action_taken: &str) -> DragonbornBreathWeaponWitness {
    match observed_action_taken {
        "doResolveBreathWeapon" => expected_witness_from_parts(ExpectedBreathWeaponWitness {
            target_hp: 10,
            second_target_hp: 15,
            breath_weapon_uses_remaining: 2,
            action_resources_remaining: 0,
            scenario_outcome: "Resolved",
            protocol_result: "resolved",
            protocol_invalid_reason: "",
        }),
        "doOpenExtraAttackSlot" => expected_witness_from_parts(ExpectedBreathWeaponWitness {
            target_hp: 10,
            second_target_hp: 20,
            breath_weapon_uses_remaining: 2,
            action_resources_remaining: 1,
            scenario_outcome: "OpenedExtraAttack",
            protocol_result: "resolved",
            protocol_invalid_reason: "",
        }),
        "doRejectMissingResource" => expected_witness_from_parts(ExpectedBreathWeaponWitness {
            target_hp: 20,
            second_target_hp: 20,
            breath_weapon_uses_remaining: 0,
            action_resources_remaining: 1,
            scenario_outcome: "RejectMissingResource",
            protocol_result: "invalid",
            protocol_invalid_reason: "WInvalidFill",
        }),
        "doRejectMismatchedArea" => expected_witness_from_parts(ExpectedBreathWeaponWitness {
            target_hp: 20,
            second_target_hp: 20,
            breath_weapon_uses_remaining: 3,
            action_resources_remaining: 1,
            scenario_outcome: "RejectMismatchedArea",
            protocol_result: "invalid",
            protocol_invalid_reason: "WInvalidFill",
        }),
        "doRejectInvalidDamageRoll" => expected_witness_from_parts(ExpectedBreathWeaponWitness {
            target_hp: 20,
            second_target_hp: 20,
            breath_weapon_uses_remaining: 3,
            action_resources_remaining: 1,
            scenario_outcome: "RejectInvalidDamageRoll",
            protocol_result: "invalid",
            protocol_invalid_reason: "WInvalidFill",
        }),
        action => panic!("unsupported mbt::actionTaken {action}"),
    }
}

pub fn replay_observed_route(observed_action_taken: &str) -> Vec<ReducerRouteEvent> {
    match observed_action_taken {
        "doResolveBreathWeapon" => observed_breath_weapon_route(
            start_dragonborn_breath_weapon_battle_observed,
            breath_weapon_facts(false, true),
        ),
        "doOpenExtraAttackSlot" => observed_breath_weapon_route(
            start_dragonborn_breath_weapon_battle_observed,
            breath_weapon_facts(true, false),
        ),
        "doRejectMissingResource" => observed_breath_weapon_route(
            |trace| {
                with_dragonborn_breath_weapon_uses_remaining(
                    start_dragonborn_breath_weapon_battle_observed(trace),
                    0,
                )
            },
            breath_weapon_facts(false, true),
        ),
        "doRejectMismatchedArea" => observed_breath_weapon_route(
            start_dragonborn_breath_weapon_battle_observed,
            DragonbornBreathWeaponFacts {
                area_shape: BreathWeaponAreaShape::Line30By5Feet,
                ..breath_weapon_facts(false, true)
            },
        ),
        "doRejectInvalidDamageRoll" => observed_breath_weapon_route(
            start_dragonborn_breath_weapon_battle_observed,
            DragonbornBreathWeaponFacts {
                damage_roll: 11,
                ..breath_weapon_facts(false, true)
            },
        ),
        action => panic!("unsupported mbt::actionTaken {action}"),
    }
}

pub fn expected_route(observed_action_taken: &str) -> Vec<ReducerRouteEvent> {
    match observed_action_taken {
        "doResolveBreathWeapon" => expected_resolve_breath_weapon_route(false),
        "doOpenExtraAttackSlot" => expected_resolve_breath_weapon_route(true),
        "doRejectMissingResource" => expected_reject_missing_resource_route(),
        "doRejectMismatchedArea" => expected_reject_mismatched_area_route(),
        "doRejectInvalidDamageRoll" => expected_reject_invalid_damage_roll_route(),
        action => panic!("unsupported mbt::actionTaken {action}"),
    }
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

fn resolved_state() -> BattleState {
    resolve_breath_weapon(
        start_dragonborn_breath_weapon_battle(),
        breath_weapon_facts(false, true),
    )
}

fn open_extra_attack_slot_state() -> BattleState {
    resolve_breath_weapon(
        start_dragonborn_breath_weapon_battle(),
        breath_weapon_facts(true, false),
    )
}

fn reject_missing_resource_state() -> BattleState {
    resolve_breath_weapon(
        with_dragonborn_breath_weapon_uses_remaining(start_dragonborn_breath_weapon_battle(), 0),
        breath_weapon_facts(false, true),
    )
}

fn reject_mismatched_area_state() -> BattleState {
    resolve_breath_weapon(
        start_dragonborn_breath_weapon_battle(),
        DragonbornBreathWeaponFacts {
            area_shape: BreathWeaponAreaShape::Line30By5Feet,
            ..breath_weapon_facts(false, true)
        },
    )
}

fn reject_invalid_damage_roll_state() -> BattleState {
    resolve_breath_weapon(
        start_dragonborn_breath_weapon_battle(),
        DragonbornBreathWeaponFacts {
            damage_roll: 11,
            ..breath_weapon_facts(false, true)
        },
    )
}

fn breath_weapon_facts(
    opens_extra_attack_slot: bool,
    second_target_in_area: bool,
) -> DragonbornBreathWeaponFacts {
    DragonbornBreathWeaponFacts {
        character_level: 1,
        damage_roll: 10,
        target_saving_throw_succeeded: false,
        second_target_in_area,
        second_target_saving_throw_succeeded: true,
        area_shape: BreathWeaponAreaShape::Cone15Feet,
        expected_area_shape: BreathWeaponAreaShape::Cone15Feet,
        opens_extra_attack_slot,
    }
}

fn resolve_breath_weapon(state: BattleState, facts: DragonbornBreathWeaponFacts) -> BattleState {
    let subject = breath_weapon_subject(&state);
    resolve_battle_subject(
        state,
        BattleResolutionRequest::attack_action_area_save_damage_replacement(
            subject,
            BattleAttackActionAreaSaveDamageReplacementFill::DragonbornBreathWeapon(facts),
        )
        .expect("breath weapon subject should match request"),
    )
    .into_state()
}

fn breath_weapon_subject(state: &BattleState) -> BattleSubject {
    discover_battle_acts(state)
        .available_acts()
        .iter()
        .map(|act| act.subject)
        .find(|subject| subject.kind == BattleSubjectKind::AttackActionAreaSaveDamageReplacement)
        .expect("breath weapon subject should be discoverable")
}

fn witness_from_battle(state: BattleState) -> DragonbornBreathWeaponWitness {
    let state = dragonborn_breath_weapon_from_battle(&state);
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

struct ExpectedBreathWeaponWitness {
    target_hp: i16,
    second_target_hp: i16,
    breath_weapon_uses_remaining: i16,
    action_resources_remaining: i16,
    scenario_outcome: &'static str,
    protocol_result: &'static str,
    protocol_invalid_reason: &'static str,
}

fn expected_witness_from_parts(
    parts: ExpectedBreathWeaponWitness,
) -> DragonbornBreathWeaponWitness {
    DragonbornBreathWeaponWitness {
        target_hp: parts.target_hp,
        second_target_hp: parts.second_target_hp,
        breath_weapon_uses_remaining: parts.breath_weapon_uses_remaining,
        action_resources_remaining: parts.action_resources_remaining,
        scenario_outcome: parts.scenario_outcome,
        protocol_result: parts.protocol_result,
        protocol_invalid_reason: parts.protocol_invalid_reason,
        protocol_holes: Vec::new(),
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

fn observed_breath_weapon_route(
    start: impl FnOnce(&mut BattleEntrypointTrace) -> BattleState,
    facts: DragonbornBreathWeaponFacts,
) -> Vec<ReducerRouteEvent> {
    let mut trace = BattleEntrypointTrace::default();
    let state = start(&mut trace);
    let discovery = discover_battle_acts_observed(&state, &mut trace);
    let subject = discovery
        .available_acts()
        .iter()
        .map(|act| act.subject)
        .find(|subject| subject.kind == BattleSubjectKind::AttackActionAreaSaveDamageReplacement)
        .expect("breath weapon subject should be discoverable");
    let request = BattleResolutionRequest::attack_action_area_save_damage_replacement(
        subject,
        BattleAttackActionAreaSaveDamageReplacementFill::DragonbornBreathWeapon(facts),
    )
    .expect("breath weapon subject should match request");
    let _result = resolve_battle_subject_observed(state, request, &mut trace);
    observed_reducer_route(
        &trace,
        &[ReducerRouteSubjectFamily::AttackActionAreaSaveDamageReplacement],
    )
}

fn expected_resolve_breath_weapon_route(opens_extra_attack_slot: bool) -> Vec<ReducerRouteEvent> {
    let _observed_action_resources_remaining = opens_extra_attack_slot;
    vec![
        ReducerRouteEvent::StartBattle {
            owner: ReducerRouteOwnerGroup::ActionEconomy,
        },
        ReducerRouteEvent::DiscoverBattleActs {
            subject: ReducerRouteSubjectFamily::AttackActionAreaSaveDamageReplacement,
            holes: vec![ReducerRouteHoleKind::SavingThrowOutcome],
            owner: ReducerRouteOwnerGroup::FeatureResource,
        },
        ReducerRouteEvent::ResolveBattleSubject {
            subject: ReducerRouteSubjectFamily::AttackActionAreaSaveDamageReplacement,
            fill: super::battle_runtime_reducer_route::ReducerRouteFillEvidence::FillKind(
                ReducerRouteFillKind::SavingThrowOutcome,
            ),
            outcome: ReducerRouteResolutionOutcome::Resolved,
            holes: Vec::new(),
            owner: ReducerRouteOwnerGroup::FeatureResource,
        },
    ]
}

fn expected_reject_missing_resource_route() -> Vec<ReducerRouteEvent> {
    vec![
        ReducerRouteEvent::StartBattle {
            owner: ReducerRouteOwnerGroup::ActionEconomy,
        },
        ReducerRouteEvent::DiscoverBattleActs {
            subject: ReducerRouteSubjectFamily::AttackActionAreaSaveDamageReplacement,
            holes: vec![ReducerRouteHoleKind::SavingThrowOutcome],
            owner: ReducerRouteOwnerGroup::FeatureResource,
        },
        ReducerRouteEvent::ResolveBattleSubject {
            subject: ReducerRouteSubjectFamily::AttackActionAreaSaveDamageReplacement,
            fill: super::battle_runtime_reducer_route::ReducerRouteFillEvidence::FillKind(
                ReducerRouteFillKind::SavingThrowOutcome,
            ),
            outcome: ReducerRouteResolutionOutcome::Invalid(
                crate::rules::battle_reducer_spine::BattleResolutionInvalidReason::InvalidFill,
            ),
            holes: Vec::new(),
            owner: ReducerRouteOwnerGroup::FeatureResource,
        },
    ]
}

fn expected_reject_mismatched_area_route() -> Vec<ReducerRouteEvent> {
    vec![
        ReducerRouteEvent::StartBattle {
            owner: ReducerRouteOwnerGroup::ActionEconomy,
        },
        ReducerRouteEvent::DiscoverBattleActs {
            subject: ReducerRouteSubjectFamily::AttackActionAreaSaveDamageReplacement,
            holes: vec![ReducerRouteHoleKind::SavingThrowOutcome],
            owner: ReducerRouteOwnerGroup::FeatureResource,
        },
        ReducerRouteEvent::ResolveBattleSubject {
            subject: ReducerRouteSubjectFamily::AttackActionAreaSaveDamageReplacement,
            fill: super::battle_runtime_reducer_route::ReducerRouteFillEvidence::FillKind(
                ReducerRouteFillKind::SavingThrowOutcome,
            ),
            outcome: ReducerRouteResolutionOutcome::Invalid(
                crate::rules::battle_reducer_spine::BattleResolutionInvalidReason::InvalidFill,
            ),
            holes: vec![ReducerRouteHoleKind::SavingThrowOutcome],
            owner: ReducerRouteOwnerGroup::AreaShape,
        },
    ]
}

fn expected_reject_invalid_damage_roll_route() -> Vec<ReducerRouteEvent> {
    vec![
        ReducerRouteEvent::StartBattle {
            owner: ReducerRouteOwnerGroup::ActionEconomy,
        },
        ReducerRouteEvent::DiscoverBattleActs {
            subject: ReducerRouteSubjectFamily::AttackActionAreaSaveDamageReplacement,
            holes: vec![ReducerRouteHoleKind::SavingThrowOutcome],
            owner: ReducerRouteOwnerGroup::FeatureResource,
        },
        ReducerRouteEvent::ResolveBattleSubject {
            subject: ReducerRouteSubjectFamily::AttackActionAreaSaveDamageReplacement,
            fill: super::battle_runtime_reducer_route::ReducerRouteFillEvidence::FillKind(
                ReducerRouteFillKind::SavingThrowOutcome,
            ),
            outcome: ReducerRouteResolutionOutcome::Invalid(
                crate::rules::battle_reducer_spine::BattleResolutionInvalidReason::InvalidFill,
            ),
            holes: vec![ReducerRouteHoleKind::RolledDice],
            owner: ReducerRouteOwnerGroup::DamageRoll,
        },
    ]
}
