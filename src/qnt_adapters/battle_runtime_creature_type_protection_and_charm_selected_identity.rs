use crate::rules::battle_reducer_spine::{
    discover_battle_acts_observed, protection_charm_ward_from_battle,
    resolve_battle_subject_observed, start_protection_charm_ward_battle_observed,
    BattleEntrypointTrace, BattleHoleKind, BattleProtectionCharmWardFill,
    BattleProtectionCharmWardProjection, BattleProtectionCharmWardSubject, BattleResolutionRequest,
    BattleResolutionResult, BattleSubjectKind,
};
use crate::rules::creature_type_protection::{
    CreatureTypeProtectionProtocol, CreatureTypeProtectionState, ProtectionScenarioOutcome,
};

use super::battle_runtime_reducer_route::{
    observed_reducer_route, route_discover_battle_acts, route_resolve_battle_subject,
    route_start_battle, ReducerRouteEvent, ReducerRouteFillKind, ReducerRouteOwnerGroup,
    ReducerRouteSubjectFamily,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CreatureTypeProtectionAndCharmWitness {
    pub beast_target_admitted: bool,
    pub humanoid_target_admitted: bool,
    pub known_willing_protection_target_admitted: bool,
    pub plain_protection_target_rejected: bool,
    pub protection_effect_present: bool,
    pub scoped_attack_roll_disadvantage: bool,
    pub unscoped_attack_roll_normal: bool,
    pub scoped_charm_prevented: bool,
    pub unscoped_charm_applied: bool,
    pub scoped_possession_prevented: bool,
    pub unscoped_possession_unprevented: bool,
    pub relevant_charm_save_has_advantage: bool,
    pub relevant_charm_save_cleared: bool,
    pub target_charmed: bool,
    pub animal_friendship_effect_present: bool,
    pub action_available: bool,
    pub first_level_slots_expended: i16,
    pub scenario_outcome: &'static str,
    pub protocol_result: &'static str,
    pub protocol_holes: Vec<&'static str>,
}

pub const BRANCH_ACTIONS: [&str; 7] = [
    "doDiscoverAnimalFriendshipBeastTargetAdmission",
    "doResolveAnimalFriendshipFailedSaveCharmed",
    "doResolveAnimalFriendshipCasterDamageBreak",
    "doResolveProtectionFromEvilAndGoodKnownWillingTargetProtection",
    "doProjectProtectionFromEvilAndGoodScopedAttackDisadvantage",
    "doPreventProtectionFromEvilAndGoodScopedCharmAndPossession",
    "doResolveProtectionFromEvilAndGoodRelevantCharmSaveAdvantage",
];

pub fn replay_observed_action(
    observed_action_taken: &str,
) -> CreatureTypeProtectionAndCharmWitness {
    witness_from_state(replay_observed_state_and_route(observed_action_taken).0)
}

pub fn expected_witness(observed_action_taken: &str) -> CreatureTypeProtectionAndCharmWitness {
    replay_observed_action(observed_action_taken)
}

pub fn replay_observed_route(observed_action_taken: &str) -> Vec<ReducerRouteEvent> {
    replay_observed_state_and_route(observed_action_taken).1
}

pub fn expected_route(observed_action_taken: &str) -> Vec<ReducerRouteEvent> {
    let (subject, holes, owner) = expected_route_witness(observed_action_taken);
    vec![
        route_start_battle(ReducerRouteOwnerGroup::ActionEconomy),
        route_discover_battle_acts(subject, holes, owner),
        route_resolve_battle_subject(
            subject,
            ReducerRouteFillKind::UnitFeatureDecision,
            Vec::new(),
            owner,
        ),
    ]
}

pub fn projection_payload(witness: &CreatureTypeProtectionAndCharmWitness) -> String {
    [
        format!("beastTargetAdmitted={}", witness.beast_target_admitted),
        format!(
            "humanoidTargetAdmitted={}",
            witness.humanoid_target_admitted
        ),
        format!(
            "knownWillingProtectionTargetAdmitted={}",
            witness.known_willing_protection_target_admitted
        ),
        format!(
            "plainProtectionTargetRejected={}",
            witness.plain_protection_target_rejected
        ),
        format!(
            "protectionEffectPresent={}",
            witness.protection_effect_present
        ),
        format!(
            "scopedAttackRollDisadvantage={}",
            witness.scoped_attack_roll_disadvantage
        ),
        format!(
            "unscopedAttackRollNormal={}",
            witness.unscoped_attack_roll_normal
        ),
        format!("scopedCharmPrevented={}", witness.scoped_charm_prevented),
        format!("unscopedCharmApplied={}", witness.unscoped_charm_applied),
        format!(
            "scopedPossessionPrevented={}",
            witness.scoped_possession_prevented
        ),
        format!(
            "unscopedPossessionUnprevented={}",
            witness.unscoped_possession_unprevented
        ),
        format!(
            "relevantCharmSaveHasAdvantage={}",
            witness.relevant_charm_save_has_advantage
        ),
        format!(
            "relevantCharmSaveCleared={}",
            witness.relevant_charm_save_cleared
        ),
        format!("targetCharmed={}", witness.target_charmed),
        format!(
            "animalFriendshipEffectPresent={}",
            witness.animal_friendship_effect_present
        ),
        format!("actionAvailable={}", witness.action_available),
        format!(
            "firstLevelSlotsExpended={}",
            witness.first_level_slots_expended
        ),
        format!("scenarioOutcome={}", witness.scenario_outcome),
        format!("protocolResult={}", witness.protocol_result),
        format!("protocolHoles={}", joined_or_none(&witness.protocol_holes)),
    ]
    .join("\n")
}

fn replay_observed_state_and_route(
    observed_action_taken: &str,
) -> (CreatureTypeProtectionState, Vec<ReducerRouteEvent>) {
    let fill = fill_for_action(observed_action_taken);
    let mut trace = BattleEntrypointTrace::default();
    let state = start_protection_charm_ward_battle_observed(fill, &mut trace);
    let discovery = discover_battle_acts_observed(&state, &mut trace);
    let subject = discovery
        .available_acts()
        .iter()
        .map(|act| act.subject)
        .find(|subject| subject.kind == route_subject_kind(fill.subject()))
        .expect("protection/charm route subject should be discoverable");
    let request = BattleResolutionRequest::protection_charm_ward(subject, fill)
        .expect("protection/charm fill should match route subject");
    let result = resolve_battle_subject_observed(state, request, &mut trace);
    let BattleResolutionResult::Resolved { state } = result else {
        panic!("{observed_action_taken} should resolve through protection/charm route")
    };
    let projection = protection_charm_ward_from_battle(&state, fill);
    let BattleProtectionCharmWardProjection::CreatureTypeProtection(protection) = projection else {
        panic!("{observed_action_taken} should project creature type protection/charm facts")
    };
    (
        protection,
        observed_reducer_route(&trace, &[route_subject_family(fill.subject())]),
    )
}

fn fill_for_action(observed_action_taken: &str) -> BattleProtectionCharmWardFill {
    match observed_action_taken {
        "doDiscoverAnimalFriendshipBeastTargetAdmission" => {
            BattleProtectionCharmWardFill::CreatureTypeConditionTargetAdmission
        }
        "doResolveAnimalFriendshipFailedSaveCharmed" => {
            BattleProtectionCharmWardFill::CreatureTypeConditionFailedSave
        }
        "doResolveAnimalFriendshipCasterDamageBreak" => {
            BattleProtectionCharmWardFill::CreatureTypeConditionDamageBreak
        }
        "doResolveProtectionFromEvilAndGoodKnownWillingTargetProtection" => {
            BattleProtectionCharmWardFill::CreatureTypeProtectionTargetAdmission
        }
        "doProjectProtectionFromEvilAndGoodScopedAttackDisadvantage" => {
            BattleProtectionCharmWardFill::CreatureTypeProtectionAttackRollMode
        }
        "doPreventProtectionFromEvilAndGoodScopedCharmAndPossession" => {
            BattleProtectionCharmWardFill::CreatureTypeProtectionConditionInterdiction
        }
        "doResolveProtectionFromEvilAndGoodRelevantCharmSaveAdvantage" => {
            BattleProtectionCharmWardFill::CreatureTypeProtectionRelevantSaveRollMode
        }
        action => panic!("unsupported mbt::actionTaken {action}"),
    }
}

fn route_subject_family(subject: BattleProtectionCharmWardSubject) -> ReducerRouteSubjectFamily {
    match subject {
        BattleProtectionCharmWardSubject::TargetAdmission => {
            ReducerRouteSubjectFamily::TargetAdmission
        }
        BattleProtectionCharmWardSubject::ActiveEffect => ReducerRouteSubjectFamily::ActiveEffect,
        BattleProtectionCharmWardSubject::ConditionLifecycle => {
            ReducerRouteSubjectFamily::ConditionLifecycle
        }
        BattleProtectionCharmWardSubject::SavingThrowRollMode => {
            ReducerRouteSubjectFamily::SavingThrowRollMode
        }
        BattleProtectionCharmWardSubject::TargetInterdiction => {
            ReducerRouteSubjectFamily::TargetInterdiction
        }
    }
}

fn expected_route_witness(
    observed_action_taken: &str,
) -> (
    ReducerRouteSubjectFamily,
    Vec<BattleHoleKind>,
    ReducerRouteOwnerGroup,
) {
    match observed_action_taken {
        "doDiscoverAnimalFriendshipBeastTargetAdmission" => (
            ReducerRouteSubjectFamily::TargetAdmission,
            vec![BattleHoleKind::TargetChoice],
            ReducerRouteOwnerGroup::TargetAdmission,
        ),
        "doResolveAnimalFriendshipFailedSaveCharmed"
        | "doResolveAnimalFriendshipCasterDamageBreak"
        | "doPreventProtectionFromEvilAndGoodScopedCharmAndPossession" => (
            ReducerRouteSubjectFamily::ConditionLifecycle,
            Vec::new(),
            ReducerRouteOwnerGroup::ConditionLifecycle,
        ),
        "doResolveProtectionFromEvilAndGoodKnownWillingTargetProtection"
        | "doProjectProtectionFromEvilAndGoodScopedAttackDisadvantage" => (
            ReducerRouteSubjectFamily::ActiveEffect,
            Vec::new(),
            ReducerRouteOwnerGroup::ActiveEffect,
        ),
        "doResolveProtectionFromEvilAndGoodRelevantCharmSaveAdvantage" => (
            ReducerRouteSubjectFamily::SavingThrowRollMode,
            Vec::new(),
            ReducerRouteOwnerGroup::SavingThrowRollMode,
        ),
        action => panic!("unsupported expected route action {action}"),
    }
}

fn route_subject_kind(subject: BattleProtectionCharmWardSubject) -> BattleSubjectKind {
    match subject {
        BattleProtectionCharmWardSubject::TargetAdmission => BattleSubjectKind::TargetAdmission,
        BattleProtectionCharmWardSubject::ActiveEffect => BattleSubjectKind::ActiveEffect,
        BattleProtectionCharmWardSubject::ConditionLifecycle => {
            BattleSubjectKind::ConditionLifecycle
        }
        BattleProtectionCharmWardSubject::SavingThrowRollMode => {
            BattleSubjectKind::SavingThrowRollMode
        }
        BattleProtectionCharmWardSubject::TargetInterdiction => {
            BattleSubjectKind::TargetInterdiction
        }
    }
}

fn witness_from_state(state: CreatureTypeProtectionState) -> CreatureTypeProtectionAndCharmWitness {
    CreatureTypeProtectionAndCharmWitness {
        beast_target_admitted: state.beast_target_admitted,
        humanoid_target_admitted: state.humanoid_target_admitted,
        known_willing_protection_target_admitted: state.known_willing_protection_target_admitted,
        plain_protection_target_rejected: state.plain_protection_target_rejected,
        protection_effect_present: state.protection_effect_present,
        scoped_attack_roll_disadvantage: state.scoped_attack_roll_disadvantage,
        unscoped_attack_roll_normal: state.unscoped_attack_roll_normal,
        scoped_charm_prevented: state.scoped_charm_prevented,
        unscoped_charm_applied: state.unscoped_charm_applied,
        scoped_possession_prevented: state.scoped_possession_prevented,
        unscoped_possession_unprevented: state.unscoped_possession_unprevented,
        relevant_charm_save_has_advantage: state.relevant_charm_save_has_advantage,
        relevant_charm_save_cleared: state.relevant_charm_save_cleared,
        target_charmed: state.target_charmed,
        animal_friendship_effect_present: state.animal_friendship_effect_present,
        action_available: state.action_available,
        first_level_slots_expended: state.first_level_slots_expended,
        scenario_outcome: scenario_outcome_ref(state.scenario_outcome),
        protocol_result: protocol_result_ref(state.protocol),
        protocol_holes: protocol_holes(state.protocol),
    }
}

fn scenario_outcome_ref(outcome: ProtectionScenarioOutcome) -> &'static str {
    match outcome {
        ProtectionScenarioOutcome::Init => "Init",
        ProtectionScenarioOutcome::Discovered => "Discovered",
        ProtectionScenarioOutcome::Resolved => "Resolved",
        ProtectionScenarioOutcome::DamageBreakResolved => "DamageBreakResolved",
        ProtectionScenarioOutcome::ProtectionResolved => "ProtectionResolved",
        ProtectionScenarioOutcome::ProtectionAttackProjected => "ProtectionAttackProjected",
        ProtectionScenarioOutcome::ProtectionCharmPrevented => "ProtectionCharmPrevented",
        ProtectionScenarioOutcome::ProtectionRelevantSaveResolved => {
            "ProtectionRelevantSaveResolved"
        }
    }
}

fn protocol_result_ref(protocol: CreatureTypeProtectionProtocol) -> &'static str {
    match protocol {
        CreatureTypeProtectionProtocol::Init => "init",
        CreatureTypeProtectionProtocol::Resolved => "resolved",
    }
}

fn protocol_holes(protocol: CreatureTypeProtectionProtocol) -> Vec<&'static str> {
    match protocol {
        CreatureTypeProtectionProtocol::Init => vec!["CreatureTypeProtectionCharm"],
        CreatureTypeProtectionProtocol::Resolved => vec![],
    }
}

fn joined_or_none(values: &[&'static str]) -> String {
    if values.is_empty() {
        "none".to_string()
    } else {
        values.join(",")
    }
}
