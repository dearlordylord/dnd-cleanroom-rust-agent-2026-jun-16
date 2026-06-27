use crate::rules::find_familiar::{
    create_find_familiar_companion, deliver_find_familiar_touch_spell,
    dismiss_and_reappear_find_familiar_companion, find_familiar_companion_initial_state,
    recast_find_familiar_companion_replacement, FamiliarCreatureTypeOverride, FamiliarForm,
    FamiliarFormFacts, FamiliarSlot, FamiliarStatus, FindFamiliarCompanionProtocol,
    FindFamiliarCompanionScenarioOutcome, FindFamiliarCompanionState,
};

use super::battle_runtime_reducer_route::{
    route_discover_battle_acts_from_route_holes, route_resolve_battle_subject_from_route_result,
    route_resolve_battle_subject_without_fill_from_route_result, route_start_battle,
    ReducerRouteEvent, ReducerRouteFillKind, ReducerRouteHoleKind, ReducerRouteOwnerGroup,
    ReducerRouteResolutionOutcome, ReducerRouteSubjectFamily,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FindFamiliarSelectedIdentityWitness {
    pub familiar_status: &'static str,
    pub form_id: &'static str,
    pub familiar_combatant_present: bool,
    pub replacement_combatant_present: bool,
    pub familiar_reaction_available: bool,
    pub owner_action_available: bool,
    pub owner_spell_slot_committed: bool,
    pub target_hp: i16,
    pub scenario_outcome: &'static str,
    pub protocol_result: &'static str,
    pub protocol_holes: Vec<&'static str>,
}

pub const BRANCH_ACTIONS: [&str; 4] = [
    "doCastFindFamiliar",
    "doDeliverTouchSpellThroughFindFamiliar",
    "doDismissAndReappearFindFamiliar",
    "doRecastFindFamiliarReplacement",
];

pub fn replay_observed_action(observed_action_taken: &str) -> FindFamiliarSelectedIdentityWitness {
    match observed_action_taken {
        "doCastFindFamiliar" => witness_from_state(cast_find_familiar()),
        "doDeliverTouchSpellThroughFindFamiliar" => witness_from_state(touch_spell_delivered()),
        "doDismissAndReappearFindFamiliar" => witness_from_state(dismissed_and_reappeared()),
        "doRecastFindFamiliarReplacement" => witness_from_state(recast_replacement()),
        action => panic!("unsupported mbt::actionTaken {action}"),
    }
}

pub fn expected_witness(observed_action_taken: &str) -> FindFamiliarSelectedIdentityWitness {
    match observed_action_taken {
        "doCastFindFamiliar" => expected_witness_from_parts(FindFamiliarExpectedParts {
            familiar_status: "present",
            form_id: "cat",
            familiar_combatant_present: true,
            replacement_combatant_present: false,
            familiar_reaction_available: true,
            owner_action_available: true,
            owner_spell_slot_committed: false,
            target_hp: 1,
            scenario_outcome: "Cast",
        }),
        "doDeliverTouchSpellThroughFindFamiliar" => {
            expected_witness_from_parts(FindFamiliarExpectedParts {
                familiar_status: "present",
                form_id: "cat",
                familiar_combatant_present: true,
                replacement_combatant_present: false,
                familiar_reaction_available: false,
                owner_action_available: false,
                owner_spell_slot_committed: true,
                target_hp: 12,
                scenario_outcome: "TouchDelivered",
            })
        }
        "doDismissAndReappearFindFamiliar" => {
            expected_witness_from_parts(FindFamiliarExpectedParts {
                familiar_status: "present",
                form_id: "cat",
                familiar_combatant_present: true,
                replacement_combatant_present: false,
                familiar_reaction_available: true,
                owner_action_available: false,
                owner_spell_slot_committed: false,
                target_hp: 1,
                scenario_outcome: "DismissedAndReappeared",
            })
        }
        "doRecastFindFamiliarReplacement" => {
            expected_witness_from_parts(FindFamiliarExpectedParts {
                familiar_status: "present",
                form_id: "rat",
                familiar_combatant_present: true,
                replacement_combatant_present: false,
                familiar_reaction_available: true,
                owner_action_available: true,
                owner_spell_slot_committed: false,
                target_hp: 1,
                scenario_outcome: "Recast",
            })
        }
        action => panic!("unsupported mbt::actionTaken {action}"),
    }
}

pub fn replay_observed_route(observed_action_taken: &str) -> Vec<ReducerRouteEvent> {
    let state = match observed_action_taken {
        "doCastFindFamiliar" => cast_find_familiar(),
        "doDeliverTouchSpellThroughFindFamiliar" => touch_spell_delivered(),
        "doDismissAndReappearFindFamiliar" => dismissed_and_reappeared(),
        "doRecastFindFamiliarReplacement" => recast_replacement(),
        action => panic!("unsupported mbt::actionTaken {action}"),
    };
    route_from_companion_substrate(state.scenario_outcome)
}

pub fn expected_route(observed_action_taken: &str) -> Vec<ReducerRouteEvent> {
    match observed_action_taken {
        "doCastFindFamiliar"
        | "doDismissAndReappearFindFamiliar"
        | "doRecastFindFamiliarReplacement" => companion_lifecycle_route(),
        "doDeliverTouchSpellThroughFindFamiliar" => companion_touch_delivery_route(),
        action => panic!("unsupported mbt::actionTaken {action}"),
    }
}

pub fn projection_payload(witness: &FindFamiliarSelectedIdentityWitness) -> String {
    [
        format!("qFamiliarStatus={}", witness.familiar_status),
        format!("qFormId={}", witness.form_id),
        format!(
            "qFamiliarCombatantPresent={}",
            witness.familiar_combatant_present
        ),
        format!(
            "qReplacementCombatantPresent={}",
            witness.replacement_combatant_present
        ),
        format!(
            "qFamiliarReactionAvailable={}",
            witness.familiar_reaction_available
        ),
        format!("qOwnerActionAvailable={}", witness.owner_action_available),
        format!(
            "qOwnerSpellSlotCommitted={}",
            witness.owner_spell_slot_committed
        ),
        format!("qTargetHp={}", witness.target_hp),
        format!("qScenarioOutcome={}", witness.scenario_outcome),
        format!("protocolResult={}", witness.protocol_result),
        format!("protocolHoles={}", joined_or_none(&witness.protocol_holes)),
    ]
    .join("\n")
}

fn cast_find_familiar() -> FindFamiliarCompanionState {
    create_find_familiar_companion(target_state(1), cat_familiar_facts())
}

fn recast_replacement() -> FindFamiliarCompanionState {
    recast_find_familiar_companion_replacement(cast_find_familiar(), FamiliarForm::Rat)
}

fn dismissed_and_reappeared() -> FindFamiliarCompanionState {
    dismiss_and_reappear_find_familiar_companion(cast_find_familiar())
}

fn touch_spell_delivered() -> FindFamiliarCompanionState {
    deliver_find_familiar_touch_spell(create_find_familiar_companion(
        target_state(12),
        cat_familiar_facts(),
    ))
}

fn target_state(target_hit_points: i16) -> FindFamiliarCompanionState {
    FindFamiliarCompanionState {
        target_hit_points,
        ..find_familiar_companion_initial_state()
    }
}

fn cat_familiar_facts() -> FamiliarFormFacts {
    FamiliarFormFacts {
        form: FamiliarForm::Cat,
        creature_type_override: FamiliarCreatureTypeOverride::Fey,
    }
}

fn witness_from_state(state: FindFamiliarCompanionState) -> FindFamiliarSelectedIdentityWitness {
    FindFamiliarSelectedIdentityWitness {
        familiar_status: status_ref(state.status),
        form_id: form_ref(state.form),
        familiar_combatant_present: familiar_combatant_present(state),
        replacement_combatant_present: replacement_combatant_present(state),
        familiar_reaction_available: state.familiar_reaction_available,
        owner_action_available: state.owner_action_available,
        owner_spell_slot_committed: state.spell_slot_committed,
        target_hp: state.target_hit_points,
        scenario_outcome: scenario_outcome_ref(state.scenario_outcome),
        protocol_result: protocol_result_ref(state.protocol),
        protocol_holes: protocol_holes(state.protocol),
    }
}

struct FindFamiliarExpectedParts {
    familiar_status: &'static str,
    form_id: &'static str,
    familiar_combatant_present: bool,
    replacement_combatant_present: bool,
    familiar_reaction_available: bool,
    owner_action_available: bool,
    owner_spell_slot_committed: bool,
    target_hp: i16,
    scenario_outcome: &'static str,
}

fn expected_witness_from_parts(
    parts: FindFamiliarExpectedParts,
) -> FindFamiliarSelectedIdentityWitness {
    FindFamiliarSelectedIdentityWitness {
        familiar_status: parts.familiar_status,
        form_id: parts.form_id,
        familiar_combatant_present: parts.familiar_combatant_present,
        replacement_combatant_present: parts.replacement_combatant_present,
        familiar_reaction_available: parts.familiar_reaction_available,
        owner_action_available: parts.owner_action_available,
        owner_spell_slot_committed: parts.owner_spell_slot_committed,
        target_hp: parts.target_hp,
        scenario_outcome: parts.scenario_outcome,
        protocol_result: "resolved",
        protocol_holes: Vec::new(),
    }
}

fn familiar_combatant_present(state: FindFamiliarCompanionState) -> bool {
    state.status == FamiliarStatus::Present
        && state.slot == FamiliarSlot::Primary
        && state.companion_count == 1
}

fn replacement_combatant_present(state: FindFamiliarCompanionState) -> bool {
    state.companion_count > 1
}

fn status_ref(status: FamiliarStatus) -> &'static str {
    match status {
        FamiliarStatus::None => "none",
        FamiliarStatus::Present => "present",
    }
}

fn form_ref(form: FamiliarForm) -> &'static str {
    match form {
        FamiliarForm::None => "none",
        FamiliarForm::Cat => "cat",
        FamiliarForm::Rat => "rat",
    }
}

fn scenario_outcome_ref(outcome: FindFamiliarCompanionScenarioOutcome) -> &'static str {
    match outcome {
        FindFamiliarCompanionScenarioOutcome::Init => "Init",
        FindFamiliarCompanionScenarioOutcome::Created => "Cast",
        FindFamiliarCompanionScenarioOutcome::Replaced => "Recast",
        FindFamiliarCompanionScenarioOutcome::DismissedAndReappeared => "DismissedAndReappeared",
        FindFamiliarCompanionScenarioOutcome::SharedSenses => "SharedSenses",
        FindFamiliarCompanionScenarioOutcome::TouchDelivered => "TouchDelivered",
        FindFamiliarCompanionScenarioOutcome::PactAttack => "PactAttack",
    }
}

fn protocol_result_ref(protocol: FindFamiliarCompanionProtocol) -> &'static str {
    match protocol {
        FindFamiliarCompanionProtocol::Init => "init",
        FindFamiliarCompanionProtocol::Resolved => "resolved",
    }
}

fn protocol_holes(protocol: FindFamiliarCompanionProtocol) -> Vec<&'static str> {
    match protocol {
        FindFamiliarCompanionProtocol::Init => vec!["WitnessProtocolHole"],
        FindFamiliarCompanionProtocol::Resolved => vec![],
    }
}

fn joined_or_none(values: &[&'static str]) -> String {
    if values.is_empty() {
        "none".to_string()
    } else {
        values.join(",")
    }
}

fn route_from_companion_substrate(
    outcome: FindFamiliarCompanionScenarioOutcome,
) -> Vec<ReducerRouteEvent> {
    match outcome {
        FindFamiliarCompanionScenarioOutcome::Created
        | FindFamiliarCompanionScenarioOutcome::Replaced
        | FindFamiliarCompanionScenarioOutcome::DismissedAndReappeared => {
            companion_lifecycle_route()
        }
        FindFamiliarCompanionScenarioOutcome::TouchDelivered => companion_touch_delivery_route(),
        FindFamiliarCompanionScenarioOutcome::SharedSenses => companion_shared_senses_route(),
        FindFamiliarCompanionScenarioOutcome::PactAttack => companion_reaction_attack_route(),
        FindFamiliarCompanionScenarioOutcome::Init => vec![route_start()],
    }
}

fn companion_lifecycle_route() -> Vec<ReducerRouteEvent> {
    vec![
        route_start(),
        route_discover_battle_acts_from_route_holes(
            ReducerRouteSubjectFamily::CompanionLifecycle,
            Vec::new(),
            ReducerRouteOwnerGroup::Companion,
        ),
        route_resolve_battle_subject_without_fill_from_route_result(
            ReducerRouteSubjectFamily::CompanionLifecycle,
            ReducerRouteResolutionOutcome::Resolved,
            Vec::new(),
            ReducerRouteOwnerGroup::Companion,
        ),
    ]
}

fn companion_shared_senses_route() -> Vec<ReducerRouteEvent> {
    vec![
        route_start(),
        route_discover_battle_acts_from_route_holes(
            ReducerRouteSubjectFamily::CompanionSharedSenses,
            Vec::new(),
            ReducerRouteOwnerGroup::Companion,
        ),
        route_resolve_battle_subject_without_fill_from_route_result(
            ReducerRouteSubjectFamily::CompanionSharedSenses,
            ReducerRouteResolutionOutcome::Resolved,
            Vec::new(),
            ReducerRouteOwnerGroup::ActionEconomy,
        ),
        route_resolve_battle_subject_without_fill_from_route_result(
            ReducerRouteSubjectFamily::CompanionSharedSenses,
            ReducerRouteResolutionOutcome::Resolved,
            Vec::new(),
            ReducerRouteOwnerGroup::ActiveEffect,
        ),
    ]
}

fn companion_touch_delivery_route() -> Vec<ReducerRouteEvent> {
    vec![
        route_start(),
        route_discover_battle_acts_from_route_holes(
            ReducerRouteSubjectFamily::CompanionTouchDelivery,
            vec![ReducerRouteHoleKind::TargetChoice],
            ReducerRouteOwnerGroup::SpellSlotAndActionEconomy,
        ),
        route_resolve_battle_subject_from_route_result(
            ReducerRouteSubjectFamily::CompanionTouchDelivery,
            ReducerRouteFillKind::TargetChoice,
            ReducerRouteResolutionOutcome::NeedsHoles,
            vec![ReducerRouteHoleKind::RolledDice],
            ReducerRouteOwnerGroup::Companion,
        ),
        route_resolve_battle_subject_from_route_result(
            ReducerRouteSubjectFamily::CompanionTouchDelivery,
            ReducerRouteFillKind::RolledDice,
            ReducerRouteResolutionOutcome::Resolved,
            Vec::new(),
            ReducerRouteOwnerGroup::SpellSlotAndActionEconomy,
        ),
        route_resolve_battle_subject_without_fill_from_route_result(
            ReducerRouteSubjectFamily::CompanionTouchDelivery,
            ReducerRouteResolutionOutcome::Resolved,
            Vec::new(),
            ReducerRouteOwnerGroup::ActionEconomy,
        ),
    ]
}

fn companion_reaction_attack_route() -> Vec<ReducerRouteEvent> {
    vec![
        route_start(),
        route_discover_battle_acts_from_route_holes(
            ReducerRouteSubjectFamily::CompanionReactionAttack,
            vec![ReducerRouteHoleKind::TargetChoice],
            ReducerRouteOwnerGroup::Companion,
        ),
        route_resolve_battle_subject_without_fill_from_route_result(
            ReducerRouteSubjectFamily::CompanionReactionAttack,
            ReducerRouteResolutionOutcome::NeedsHoles,
            vec![ReducerRouteHoleKind::TargetChoice],
            ReducerRouteOwnerGroup::StatBlockAction,
        ),
        route_resolve_battle_subject_from_route_result(
            ReducerRouteSubjectFamily::CompanionReactionAttack,
            ReducerRouteFillKind::TargetChoice,
            ReducerRouteResolutionOutcome::NeedsHoles,
            vec![ReducerRouteHoleKind::AttackRoll],
            ReducerRouteOwnerGroup::TargetSelection,
        ),
        route_resolve_battle_subject_from_route_result(
            ReducerRouteSubjectFamily::CompanionReactionAttack,
            ReducerRouteFillKind::AttackRoll,
            ReducerRouteResolutionOutcome::NeedsHoles,
            vec![ReducerRouteHoleKind::RolledDice],
            ReducerRouteOwnerGroup::AttackRoll,
        ),
        route_resolve_battle_subject_from_route_result(
            ReducerRouteSubjectFamily::CompanionReactionAttack,
            ReducerRouteFillKind::RolledDice,
            ReducerRouteResolutionOutcome::Resolved,
            Vec::new(),
            ReducerRouteOwnerGroup::HitPoint,
        ),
        route_resolve_battle_subject_without_fill_from_route_result(
            ReducerRouteSubjectFamily::CompanionReactionAttack,
            ReducerRouteResolutionOutcome::Resolved,
            Vec::new(),
            ReducerRouteOwnerGroup::ActionEconomy,
        ),
    ]
}

fn route_start() -> ReducerRouteEvent {
    route_start_battle(ReducerRouteOwnerGroup::ActionEconomy)
}
