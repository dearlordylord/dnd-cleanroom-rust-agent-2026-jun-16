use crate::rules::wild_shape::{
    assume_riding_horse_wild_shape, begin_wild_shape_next_turn, dismiss_wild_shape_form,
    reuse_wild_shape_as_cat, revert_wild_shape_due_to_death,
    revert_wild_shape_due_to_incapacitated, stutter_wild_shape_state, wild_shape_initial_state,
    WildShapeCreatureSize, WildShapeDruidStatus, WildShapeForm, WildShapeProtocol,
    WildShapeScenarioOutcome, WildShapeState,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DruidWildShapeFormLifecycleWitness {
    pub active_form: &'static str,
    pub bonus_action_available: bool,
    pub uses_remaining: i16,
    pub temporary_hit_points: i16,
    pub armor_class: i16,
    pub creature_size: &'static str,
    pub speed_feet: i16,
    pub shove_dc: i16,
    pub spell_available: bool,
    pub active_form_effect_count: i16,
    pub merged_equipment_count: i16,
    pub druid_status: &'static str,
    pub scenario_outcome: &'static str,
    pub protocol_result: &'static str,
    pub protocol_invalid_reason: &'static str,
    pub protocol_holes: Vec<&'static str>,
}

pub const BRANCH_ACTIONS: [&str; 7] = [
    "doAssumeRidingHorse",
    "doBeginNextTurn",
    "doDeathReversion",
    "doDismissForm",
    "doIncapacitatedReversion",
    "doReuseAsCat",
    "doStutter",
];

pub fn replay_observed_action(observed_action_taken: &str) -> DruidWildShapeFormLifecycleWitness {
    match observed_action_taken {
        "doAssumeRidingHorse" => witness_from_state(assumed_riding_horse()),
        "doBeginNextTurn" => witness_from_state(next_turn()),
        "doDeathReversion" => witness_from_state(death_reversion()),
        "doDismissForm" => witness_from_state(dismissed()),
        "doIncapacitatedReversion" => witness_from_state(incapacitated_reversion()),
        "doReuseAsCat" => witness_from_state(reused_cat()),
        "doStutter" => witness_from_state(stuttered_after_death()),
        action => panic!("unsupported mbt::actionTaken {action}"),
    }
}

pub fn expected_witness(observed_action_taken: &str) -> DruidWildShapeFormLifecycleWitness {
    replay_observed_action(observed_action_taken)
}

pub fn projection_payload(witness: &DruidWildShapeFormLifecycleWitness) -> String {
    [
        format!("qActiveForm={}", witness.active_form),
        format!("qBonusActionAvailable={}", witness.bonus_action_available),
        format!("qUsesRemaining={}", witness.uses_remaining),
        format!("qTempHp={}", witness.temporary_hit_points),
        format!("qArmorClass={}", witness.armor_class),
        format!("qCreatureSize={}", witness.creature_size),
        format!("qSpeedFeet={}", witness.speed_feet),
        format!("qShoveDc={}", witness.shove_dc),
        format!("qSpellAvailable={}", witness.spell_available),
        format!(
            "qActiveFormEffectCount={}",
            witness.active_form_effect_count
        ),
        format!("qMergedEquipmentCount={}", witness.merged_equipment_count),
        format!("qDruidStatus={}", witness.druid_status),
        format!("qScenarioOutcome={}", witness.scenario_outcome),
        format!("protocolResult={}", witness.protocol_result),
        format!("protocolInvalidReason={}", witness.protocol_invalid_reason),
        format!("protocolHoles={}", joined_or_none(&witness.protocol_holes)),
    ]
    .join("\n")
}

fn assumed_riding_horse() -> WildShapeState {
    assume_riding_horse_wild_shape(wild_shape_initial_state())
}

fn next_turn() -> WildShapeState {
    begin_wild_shape_next_turn(assumed_riding_horse())
}

fn reused_cat() -> WildShapeState {
    reuse_wild_shape_as_cat(next_turn())
}

fn dismissed() -> WildShapeState {
    dismiss_wild_shape_form(next_turn())
}

fn incapacitated_reversion() -> WildShapeState {
    revert_wild_shape_due_to_incapacitated(assumed_riding_horse())
}

fn death_reversion() -> WildShapeState {
    revert_wild_shape_due_to_death(assumed_riding_horse())
}

fn stuttered_after_death() -> WildShapeState {
    stutter_wild_shape_state(death_reversion())
}

fn witness_from_state(state: WildShapeState) -> DruidWildShapeFormLifecycleWitness {
    DruidWildShapeFormLifecycleWitness {
        active_form: active_form_ref(state.active_form),
        bonus_action_available: state.bonus_action_available,
        uses_remaining: state.uses_remaining,
        temporary_hit_points: state.temporary_hit_points,
        armor_class: state.armor_class,
        creature_size: creature_size_ref(state.creature_size),
        speed_feet: state.speed_feet,
        shove_dc: state.shove_dc,
        spell_available: state.spell_available,
        active_form_effect_count: state.active_form_effect_count,
        merged_equipment_count: state.merged_equipment_count,
        druid_status: druid_status_ref(state.druid_status),
        scenario_outcome: scenario_outcome_ref(state.scenario_outcome),
        protocol_result: protocol_result_ref(state.protocol),
        protocol_invalid_reason: "",
        protocol_holes: protocol_holes(state.protocol),
    }
}

fn active_form_ref(form: WildShapeForm) -> &'static str {
    match form {
        WildShapeForm::TrueForm => "trueForm",
        WildShapeForm::RidingHorse => "ridingHorse",
        WildShapeForm::Cat => "cat",
    }
}

fn creature_size_ref(size: WildShapeCreatureSize) -> &'static str {
    match size {
        WildShapeCreatureSize::Tiny => "tiny",
        WildShapeCreatureSize::Small => "small",
        WildShapeCreatureSize::Medium => "medium",
        WildShapeCreatureSize::Large => "large",
        WildShapeCreatureSize::Huge => "huge",
        WildShapeCreatureSize::Gargantuan => "gargantuan",
    }
}

fn druid_status_ref(status: WildShapeDruidStatus) -> &'static str {
    match status {
        WildShapeDruidStatus::Able => "able",
        WildShapeDruidStatus::Incapacitated => "incapacitated",
        WildShapeDruidStatus::Dead => "dead",
    }
}

fn scenario_outcome_ref(outcome: WildShapeScenarioOutcome) -> &'static str {
    match outcome {
        WildShapeScenarioOutcome::Init => "Init",
        WildShapeScenarioOutcome::AssumedRidingHorse => "AssumedRidingHorse",
        WildShapeScenarioOutcome::NextTurn => "NextTurn",
        WildShapeScenarioOutcome::ReusedCat => "ReusedCat",
        WildShapeScenarioOutcome::Dismissed => "Dismissed",
        WildShapeScenarioOutcome::FormIncapacitated => "FormIncapacitated",
        WildShapeScenarioOutcome::FormDead => "FormDead",
    }
}

fn protocol_result_ref(protocol: WildShapeProtocol) -> &'static str {
    match protocol {
        WildShapeProtocol::Init => "init",
        WildShapeProtocol::Resolved => "resolved",
    }
}

fn protocol_holes(protocol: WildShapeProtocol) -> Vec<&'static str> {
    match protocol {
        WildShapeProtocol::Init => vec!["WitnessProtocolHole"],
        WildShapeProtocol::Resolved => vec![],
    }
}

fn joined_or_none(values: &[&'static str]) -> String {
    if values.is_empty() {
        "none".to_string()
    } else {
        values.join(",")
    }
}
