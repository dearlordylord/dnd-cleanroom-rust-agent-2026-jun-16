use crate::rules::find_familiar::{
    create_find_familiar_companion, deliver_find_familiar_touch_spell,
    dismiss_and_reappear_find_familiar_companion, find_familiar_companion_initial_state,
    recast_find_familiar_companion_replacement, FamiliarCreatureTypeOverride, FamiliarForm,
    FamiliarFormFacts, FamiliarSlot, FamiliarStatus, FindFamiliarCompanionProtocol,
    FindFamiliarCompanionScenarioOutcome, FindFamiliarCompanionState,
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
    replay_observed_action(observed_action_taken)
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
