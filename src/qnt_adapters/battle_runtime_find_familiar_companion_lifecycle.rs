use crate::rules::find_familiar::{
    create_find_familiar_companion, deliver_find_familiar_touch_spell,
    find_familiar_companion_initial_state, replace_find_familiar_companion_form,
    resolve_pact_find_familiar_reaction_attack, share_find_familiar_senses,
    FamiliarCreatureTypeOverride, FamiliarForm, FamiliarFormFacts, FamiliarSlot, FamiliarStatus,
    FindFamiliarCompanionProtocol, FindFamiliarCompanionScenarioOutcome,
    FindFamiliarCompanionState,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FindFamiliarCompanionLifecycleWitness {
    pub familiar_status: &'static str,
    pub familiar_id: &'static str,
    pub familiar_form: &'static str,
    pub creature_type_override: &'static str,
    pub companion_count: i16,
    pub telepathy_available: bool,
    pub shared_senses_active: bool,
    pub bonus_action_available: bool,
    pub owner_attack_available: bool,
    pub familiar_reaction_available: bool,
    pub touch_delivery_reaction_spent: bool,
    pub pact_reaction_attack_resolved: bool,
    pub spell_slot_committed: bool,
    pub target_hp: i16,
    pub scenario_outcome: &'static str,
    pub protocol_result: &'static str,
    pub protocol_holes: Vec<&'static str>,
}

pub const BRANCH_ACTIONS: [&str; 5] = [
    "doCreateCatFamiliar",
    "doDeliverTouchSpell",
    "doPactFamiliarAttack",
    "doReplaceWithRatFamiliar",
    "doShareSenses",
];

pub fn replay_observed_action(
    observed_action_taken: &str,
) -> FindFamiliarCompanionLifecycleWitness {
    match observed_action_taken {
        "doCreateCatFamiliar" => witness_from_state(created_cat()),
        "doDeliverTouchSpell" => witness_from_state(touch_spell_delivered()),
        "doPactFamiliarAttack" => witness_from_state(pact_familiar_attack()),
        "doReplaceWithRatFamiliar" => witness_from_state(replaced_rat()),
        "doShareSenses" => witness_from_state(shared_senses()),
        action => panic!("unsupported mbt::actionTaken {action}"),
    }
}

pub fn expected_witness(observed_action_taken: &str) -> FindFamiliarCompanionLifecycleWitness {
    replay_observed_action(observed_action_taken)
}

pub fn projection_payload(witness: &FindFamiliarCompanionLifecycleWitness) -> String {
    [
        format!("qFamiliarStatus={}", witness.familiar_status),
        format!("qFamiliarId={}", witness.familiar_id),
        format!("qFamiliarForm={}", witness.familiar_form),
        format!("qCreatureTypeOverride={}", witness.creature_type_override),
        format!("qCompanionCount={}", witness.companion_count),
        format!("qTelepathyAvailable={}", witness.telepathy_available),
        format!("qSharedSensesActive={}", witness.shared_senses_active),
        format!("qBonusActionAvailable={}", witness.bonus_action_available),
        format!("qOwnerAttackAvailable={}", witness.owner_attack_available),
        format!(
            "qFamiliarReactionAvailable={}",
            witness.familiar_reaction_available
        ),
        format!(
            "qTouchDeliveryReactionSpent={}",
            witness.touch_delivery_reaction_spent
        ),
        format!(
            "qPactReactionAttackResolved={}",
            witness.pact_reaction_attack_resolved
        ),
        format!("qSpellSlotCommitted={}", witness.spell_slot_committed),
        format!("qTargetHp={}", witness.target_hp),
        format!("qScenarioOutcome={}", witness.scenario_outcome),
        format!("protocolResult={}", witness.protocol_result),
        format!("protocolHoles={}", joined_or_none(&witness.protocol_holes)),
    ]
    .join("\n")
}

fn created_cat() -> FindFamiliarCompanionState {
    create_find_familiar_companion(
        find_familiar_companion_initial_state(),
        FamiliarFormFacts {
            form: FamiliarForm::Cat,
            creature_type_override: FamiliarCreatureTypeOverride::Fey,
        },
    )
}

fn replaced_rat() -> FindFamiliarCompanionState {
    replace_find_familiar_companion_form(created_cat(), FamiliarForm::Rat)
}

fn shared_senses() -> FindFamiliarCompanionState {
    share_find_familiar_senses(created_cat())
}

fn touch_spell_delivered() -> FindFamiliarCompanionState {
    deliver_find_familiar_touch_spell(created_cat())
}

fn pact_familiar_attack() -> FindFamiliarCompanionState {
    resolve_pact_find_familiar_reaction_attack(created_cat(), 1)
}

fn witness_from_state(state: FindFamiliarCompanionState) -> FindFamiliarCompanionLifecycleWitness {
    FindFamiliarCompanionLifecycleWitness {
        familiar_status: status_ref(state.status),
        familiar_id: slot_ref(state.slot),
        familiar_form: form_ref(state.form),
        creature_type_override: creature_type_ref(state.creature_type_override),
        companion_count: state.companion_count,
        telepathy_available: state.telepathy_available,
        shared_senses_active: state.shared_senses_active,
        bonus_action_available: state.bonus_action_available,
        owner_attack_available: state.owner_attack_available,
        familiar_reaction_available: state.familiar_reaction_available,
        touch_delivery_reaction_spent: state.touch_delivery_reaction_spent,
        pact_reaction_attack_resolved: state.pact_reaction_attack_resolved,
        spell_slot_committed: state.spell_slot_committed,
        target_hp: state.target_hit_points,
        scenario_outcome: scenario_outcome_ref(state.scenario_outcome),
        protocol_result: protocol_result_ref(state.protocol),
        protocol_holes: protocol_holes(state.protocol),
    }
}

fn status_ref(status: FamiliarStatus) -> &'static str {
    match status {
        FamiliarStatus::None => "none",
        FamiliarStatus::Present => "present",
    }
}

fn slot_ref(slot: FamiliarSlot) -> &'static str {
    match slot {
        FamiliarSlot::None => "none",
        FamiliarSlot::Primary => "primary",
    }
}

fn form_ref(form: FamiliarForm) -> &'static str {
    match form {
        FamiliarForm::None => "none",
        FamiliarForm::Cat => "cat",
        FamiliarForm::Rat => "rat",
    }
}

fn creature_type_ref(creature_type: FamiliarCreatureTypeOverride) -> &'static str {
    match creature_type {
        FamiliarCreatureTypeOverride::None => "none",
        FamiliarCreatureTypeOverride::Fey => "fey",
    }
}

fn scenario_outcome_ref(outcome: FindFamiliarCompanionScenarioOutcome) -> &'static str {
    match outcome {
        FindFamiliarCompanionScenarioOutcome::Init => "Init",
        FindFamiliarCompanionScenarioOutcome::Created => "CreatedCat",
        FindFamiliarCompanionScenarioOutcome::Replaced => "ReplacedRat",
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
